#![allow(unused_macros)]
#![allow(dead_code)]

use cairo_lang_compiler::{
    compile_prepared_db, db::RootDatabase, project::setup_project, CompilerConfig,
};
use cairo_lang_filesystem::db::init_dev_corelib;
use cairo_lang_runner::{
    Arg, RunResult, RunResultValue, RunnerError, SierraCasmRunner, StarknetState,
};
use cairo_lang_sierra::{
    extensions::core::{CoreLibfunc, CoreType, CoreTypeConcrete},
    ids::FunctionId,
    program::Program,
    program_registry::ProgramRegistry,
};
use cairo_lang_sierra_generator::replace_ids::DebugReplacer;
use cairo_lang_starknet::contract::get_contracts_info;
use cairo_native::{
    metadata::{runtime_bindings::RuntimeBindingsMeta, MetadataStorage},
    types::felt252::PRIME,
    utils::register_runtime_symbols,
};
use melior::{
    dialect::DialectRegistry,
    ir::{Location, Module},
    pass::{self, PassManager},
    utility::{register_all_dialects, register_all_passes},
    Context, ExecutionEngine,
};
use num_bigint::{BigInt, BigUint, Sign};
use std::{env::var, fs, ops::Neg, path::Path, str::FromStr, sync::Arc};

macro_rules! load_cairo {
    ( $( $program:tt )+ ) => {
        $crate::common::load_cairo_str(stringify!($($program)+))
    };
}

#[allow(unused_imports)]
pub(crate) use load_cairo;

pub(crate) const GAS: usize = usize::MAX;

// Parse numeric string into felt, wrapping negatives around the prime modulo.
pub fn felt(value: &str) -> [u32; 8] {
    let value = value.parse::<BigInt>().unwrap();
    let value = match value.sign() {
        Sign::Minus => &*PRIME - value.neg().to_biguint().unwrap(),
        _ => value.to_biguint().unwrap(),
    };

    let mut u32_digits = value.to_u32_digits();
    u32_digits.resize(8, 0);
    u32_digits.try_into().unwrap()
}

/// Converts a casm variant to sierra.
pub const fn casm_variant_to_sierra(idx: i64, num_variants: i64) -> i64 {
    num_variants - 1 - (idx >> 1)
}

pub fn get_result_success(r: &RunResultValue) -> Vec<String> {
    match r {
        RunResultValue::Success(x) => x.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
        RunResultValue::Panic(_) => panic!(),
    }
}

pub fn load_cairo_str(program_str: &str) -> (String, Program, SierraCasmRunner) {
    let mut program_file = tempfile::Builder::new()
        .prefix("test_")
        .suffix(".cairo")
        .tempfile()
        .unwrap();
    fs::write(&mut program_file, program_str).unwrap();

    let mut db = RootDatabase::default();
    init_dev_corelib(
        &mut db,
        Path::new(&var("CARGO_MANIFEST_DIR").unwrap()).join("corelib/src"),
    );
    let main_crate_ids = setup_project(&mut db, program_file.path()).unwrap();
    let program = Arc::try_unwrap(
        compile_prepared_db(
            &mut db,
            main_crate_ids.clone(),
            CompilerConfig {
                replace_ids: true,
                ..Default::default()
            },
        )
        .unwrap(),
    )
    .unwrap();

    let module_name = program_file.path().with_extension("");
    let module_name = module_name.file_name().unwrap().to_str().unwrap();

    let replacer = DebugReplacer { db: &db };
    let contracts_info = get_contracts_info(&db, main_crate_ids, &replacer).unwrap();

    let runner =
        SierraCasmRunner::new(program.clone(), Some(Default::default()), contracts_info).unwrap();

    (module_name.to_string(), program, runner)
}

pub fn run_native_program(
    program: &(&str, &Program),
    entry_point: &str,
    args: serde_json::Value,
) -> serde_json::Value {
    let entry_point = format!("{0}::{0}::{1}", program.0, entry_point);
    let program = &program.1;

    let registry = ProgramRegistry::<CoreType, CoreLibfunc>::new(program)
        .expect("Could not create the test program registry.");

    let context = Context::new();
    context.append_dialect_registry(&{
        let registry = DialectRegistry::new();
        register_all_dialects(&registry);
        registry
    });
    context.load_all_available_dialects();
    register_all_passes();

    let mut module = Module::new(Location::unknown(&context));

    let mut metadata = MetadataStorage::new();
    // Make the runtime library available.
    metadata.insert(RuntimeBindingsMeta::default()).unwrap();

    cairo_native::compile::<CoreType, CoreLibfunc>(
        &context,
        &module,
        program,
        &registry,
        &mut metadata,
        None,
    )
    .expect("Could not compile test program to MLIR.");

    assert!(
        module.as_operation().verify(),
        "Test program generated invalid MLIR:\n{}",
        module.as_operation()
    );

    let pass_manager = PassManager::new(&context);
    pass_manager.enable_verifier(true);
    pass_manager.add_pass(pass::transform::create_canonicalizer());

    pass_manager.add_pass(pass::conversion::create_scf_to_control_flow());

    pass_manager.add_pass(pass::conversion::create_arith_to_llvm());
    pass_manager.add_pass(pass::conversion::create_control_flow_to_llvm());
    pass_manager.add_pass(pass::conversion::create_func_to_llvm());
    pass_manager.add_pass(pass::conversion::create_index_to_llvm_pass());
    pass_manager.add_pass(pass::conversion::create_mem_ref_to_llvm());
    pass_manager.add_pass(pass::conversion::create_reconcile_unrealized_casts());

    pass_manager
        .run(&mut module)
        .expect("Could not apply passes to the compiled test program.");

    let engine = ExecutionEngine::new(&module, 0, &[], false);

    #[cfg(feature = "with-runtime")]
    register_runtime_symbols(&engine);

    cairo_native::execute::<CoreType, CoreLibfunc, _, _>(
        &engine,
        &registry,
        &program
            .funcs
            .iter()
            .find(|x| x.id.debug_name.as_deref() == Some(&entry_point))
            .expect("Test program entry point not found.")
            .id,
        args,
        serde_json::value::Serializer,
    )
    .expect("Test program execution failed.")
}

pub fn run_vm_program(
    program: &(&str, &Program, &SierraCasmRunner),
    entry_point: &str,
    args: &[Arg],
    gas: Option<usize>,
) -> Result<RunResult, RunnerError> {
    let runner = program.2;
    runner.run_function(
        runner.find_function(entry_point).unwrap(),
        args,
        gas,
        StarknetState::default(),
    )
}

// Panics if results don't match.
pub fn compare_outputs(
    program: &Program,
    entry_point: &FunctionId,
    vm_result: &RunResult,
    native_result: &serde_json::Value,
    ignore_gas: bool,
    has_panic: bool, // whether the function can panic, and thus the outer result enum exists in sierra but not on the vm.
) {
    let reg: ProgramRegistry<CoreType, CoreLibfunc> = ProgramRegistry::new(program).unwrap();

    let func = reg.get_function(entry_point).unwrap();

    let ret_types = &func.signature.ret_types;

    let mut native_rets = native_result.as_array().expect("should be an array").iter();
    let success_val = get_result_success(&vm_result.value);
    dbg!(&success_val);
    dbg!(&native_result);
    let mut vm_rets = success_val.iter();
    let vm_gas: u64 = vm_result
        .gas_counter
        .as_ref()
        .map(|x| x.to_biguint().try_into().unwrap())
        .unwrap_or(0);

    let mut panic_handled = !has_panic;

    fn check_next_type(
        ty: &CoreTypeConcrete,
        ignore_gas: bool,
        native_rets: &mut std::slice::Iter<'_, serde_json::Value>,
        vm_rets: &mut std::slice::Iter<'_, String>,
        vm_gas: u64,
        reg: &ProgramRegistry<CoreType, CoreLibfunc>,
        panic_handled: &mut bool,
    ) {
        match ty {
            CoreTypeConcrete::Array(_) => todo!(),
            CoreTypeConcrete::Bitwise(_) => todo!(),
            CoreTypeConcrete::Box(_) => todo!(),
            CoreTypeConcrete::EcOp(_) => todo!(),
            CoreTypeConcrete::EcPoint(_) => todo!(),
            CoreTypeConcrete::EcState(_) => todo!(),
            CoreTypeConcrete::Felt252(_) => {
                let vm_value = vm_rets.next().unwrap();

                if let serde_json::Value::Number(n) = native_rets.next().unwrap() {
                    let mut native_value =
                        BigUint::from_str(&n.to_string()).unwrap().to_u32_digits();
                    native_value.resize(8, 0);
                    assert_eq!(felt(vm_value).as_slice(), native_value.as_slice());
                } else {
                    panic!("invalid value")
                }
            }
            CoreTypeConcrete::GasBuiltin(_) => {
                // runner: ignore
                // native: compare to gas
                let gas_val = native_rets.next().unwrap().as_u64().expect("should be u64");

                if !ignore_gas {
                    assert_eq!(vm_gas, gas_val, "gas mismatch");
                }
            }
            CoreTypeConcrete::BuiltinCosts(_) => todo!(),
            CoreTypeConcrete::Uint8(_) => {
                let vm_value: u8 = vm_rets.next().unwrap().parse().unwrap();
                let native_value: u8 = native_rets
                    .next()
                    .unwrap()
                    .as_u64()
                    .unwrap()
                    .try_into()
                    .unwrap();
                assert_eq!(vm_value, native_value)
            }
            CoreTypeConcrete::Uint16(_) => {
                let vm_value: u16 = vm_rets.next().unwrap().parse().unwrap();
                let native_value: u16 = native_rets
                    .next()
                    .unwrap()
                    .as_u64()
                    .unwrap()
                    .try_into()
                    .unwrap();
                assert_eq!(vm_value, native_value)
            }
            CoreTypeConcrete::Uint32(_) => {
                let vm_value: u32 = vm_rets.next().unwrap().parse().unwrap();
                let native_value: u32 = native_rets
                    .next()
                    .unwrap()
                    .as_u64()
                    .unwrap()
                    .try_into()
                    .unwrap();
                assert_eq!(vm_value, native_value)
            }
            CoreTypeConcrete::Uint64(_) => {
                let vm_value: u64 = vm_rets.next().unwrap().parse().unwrap();
                let native_value: u64 = native_rets.next().unwrap().as_u64().unwrap();
                assert_eq!(vm_value, native_value)
            }
            CoreTypeConcrete::Uint128(_) => {
                let vm_value: u128 = vm_rets.next().unwrap().parse().unwrap();
                let native_value: u128 = native_rets.next().unwrap().as_u64().unwrap().into();
                assert_eq!(vm_value, native_value)
            }
            CoreTypeConcrete::Uint128MulGuarantee(_) => todo!(),
            CoreTypeConcrete::NonZero(_) => todo!(),
            CoreTypeConcrete::Nullable(_) => todo!(),
            CoreTypeConcrete::RangeCheck(_) => {
                // runner: ignore
                // native: null
                native_rets
                    .next()
                    .unwrap()
                    .as_null()
                    .expect("should be null");
            }
            CoreTypeConcrete::Uninitialized(_) => todo!(),
            CoreTypeConcrete::Enum(info) => {
                let enum_container = native_rets.next().unwrap().as_array().unwrap();
                assert_eq!(enum_container.len(), 2);
                let native_tag = enum_container[0].as_u64().unwrap();

                if *panic_handled {
                    let vm_tag = vm_rets.next().unwrap();
                    let vm_tag = casm_variant_to_sierra(
                        vm_tag.parse::<i64>().unwrap(),
                        info.variants.len() as i64,
                    ) as u64;
                    assert_eq!(vm_tag, native_tag, "enum tag mismatch");

                    let payload = enum_container[1].as_array().unwrap();
                    check_next_type(
                        reg.get_type(&info.variants[native_tag as usize]).unwrap(),
                        ignore_gas,
                        &mut payload.iter(),
                        vm_rets,
                        vm_gas,
                        reg,
                        panic_handled,
                    );
                } else {
                    *panic_handled = true;

                    if native_tag == 1 {
                        // todo: compare errors
                        todo!()
                    } else {
                        let payload = enum_container[1].as_array().unwrap();
                        check_next_type(
                            reg.get_type(&info.variants[native_tag as usize]).unwrap(),
                            ignore_gas,
                            &mut payload.iter(),
                            vm_rets,
                            vm_gas,
                            reg,
                            panic_handled,
                        );
                    }
                }
            }
            CoreTypeConcrete::Struct(info) => {
                let struct_container = native_rets.next().unwrap().as_array().unwrap();
                let mut iter = struct_container.iter();
                for field in &info.members {
                    check_next_type(
                        reg.get_type(field).unwrap(),
                        ignore_gas,
                        &mut iter,
                        vm_rets,
                        vm_gas,
                        reg,
                        panic_handled,
                    );
                }
            }
            CoreTypeConcrete::Felt252Dict(_) => todo!(),
            CoreTypeConcrete::Felt252DictEntry(_) => todo!(),
            CoreTypeConcrete::SquashedFelt252Dict(_) => todo!(),
            CoreTypeConcrete::Pedersen(_) => todo!(),
            CoreTypeConcrete::Poseidon(_) => todo!(),
            CoreTypeConcrete::Span(_) => todo!(),
            CoreTypeConcrete::StarkNet(_) => todo!(),
            CoreTypeConcrete::SegmentArena(_) => todo!(),
            CoreTypeConcrete::Snapshot(_) => todo!(),
        }
    }

    for ty in ret_types {
        let ty = reg.get_type(ty).unwrap();
        check_next_type(
            ty,
            ignore_gas,
            &mut native_rets,
            &mut vm_rets,
            vm_gas,
            &reg,
            &mut panic_handled,
        );
    }
}
