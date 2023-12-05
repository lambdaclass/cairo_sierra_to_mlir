use crate::common::{load_cairo, run_native_or_vm_program};
use cairo_lang_runner::{Arg, SierraCasmRunner};
use cairo_lang_sierra::program::Program;
use cairo_native::values::JITValue;
use common::compare_outputs;
use lazy_static::lazy_static;
use proptest::prelude::*;
use std::borrow::Borrow;

mod common;

const GAS: usize = usize::MAX;

lazy_static! {
    static ref U8_OVERFLOWING_ADD: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u8, rhs: u8) -> u8 {
            lhs + rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> u8 {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U8_OVERFLOWING_SUB: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u8, rhs: u8) -> u8 {
            lhs - rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> u8 {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U8_SAFE_DIVMOD: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u8, rhs: u8) -> (u8, u8) {
            let q = lhs / rhs;
            let r = lhs % rhs;

            (q, r)
        }

        fn run_test(lhs: felt252, rhs: felt252) -> (u8, u8) {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U8_EQUAL: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u8, rhs: u8) -> bool {
            lhs == rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> bool {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U8_IS_ZERO: (String, Program, SierraCasmRunner) = load_cairo! {
        use zeroable::IsZeroResult;

        extern fn u8_is_zero(a: u8) -> IsZeroResult<u8> implicits() nopanic;

        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(value: u8) -> bool {
            match u8_is_zero(value) {
                IsZeroResult::Zero(_) => true,
                IsZeroResult::NonZero(_) => false,
            }
        }

        fn run_test(value: felt252) -> bool {
            program(value.try_into().unwrap())
        }
    };
    static ref U8_SQRT: (String, Program, SierraCasmRunner) = load_cairo! {
        use core::integer::u8_sqrt;
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(value: u8) -> u8 {
            u8_sqrt(value)
        }

        fn run_test(value: felt252) -> u8 {
            program(value.try_into().unwrap())
        }
    };

    // U16

    static ref U16_OVERFLOWING_ADD: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u16, rhs: u16) -> u16 {
            lhs + rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> u16 {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U16_OVERFLOWING_SUB: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u16, rhs: u16) -> u16 {
            lhs - rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> u16 {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U16_SAFE_DIVMOD: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u16, rhs: u16) -> (u16, u16) {
            let q = lhs / rhs;
            let r = lhs % rhs;

            (q, r)
        }

        fn run_test(lhs: felt252, rhs: felt252) -> (u16, u16) {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U16_EQUAL: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u16, rhs: u16) -> bool {
            lhs == rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> bool {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U16_IS_ZERO: (String, Program, SierraCasmRunner) = load_cairo! {
        use zeroable::IsZeroResult;

        extern fn u16_is_zero(a: u16) -> IsZeroResult<u16> implicits() nopanic;

        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(value: u16) -> bool {
            match u16_is_zero(value) {
                IsZeroResult::Zero(_) => true,
                IsZeroResult::NonZero(_) => false,
            }
        }

        fn run_test(value: felt252) -> bool {
            program(value.try_into().unwrap())
        }
    };
    static ref U16_SQRT: (String, Program, SierraCasmRunner)= load_cairo! {
        use core::integer::u16_sqrt;
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(value: u16) -> u16 {
            u16_sqrt(value)
        }

        fn run_test(value: felt252) -> u16 {
            program(value.try_into().unwrap())
        }
    };

    // U32

    static ref U32_OVERFLOWING_ADD: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u32, rhs: u32) -> u32 {
            lhs + rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> u32 {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U32_OVERFLOWING_SUB: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u32, rhs: u32) -> u32 {
            lhs - rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> u32 {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U32_SAFE_DIVMOD: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u32, rhs: u32) -> (u32, u32) {
            let q = lhs / rhs;
            let r = lhs % rhs;

            (q, r)
        }

        fn run_test(lhs: felt252, rhs: felt252) -> (u32, u32) {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U32_EQUAL: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u32, rhs: u32) -> bool {
            lhs == rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> bool {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U32_IS_ZERO: (String, Program, SierraCasmRunner) = load_cairo! {
        use zeroable::IsZeroResult;

        extern fn u32_is_zero(a: u32) -> IsZeroResult<u32> implicits() nopanic;

        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(value: u32) -> bool {
            match u32_is_zero(value) {
                IsZeroResult::Zero(_) => true,
                IsZeroResult::NonZero(_) => false,
            }
        }

        fn run_test(value: felt252) -> bool {
            program(value.try_into().unwrap())
        }
    };
    static ref U32_SQRT: (String, Program, SierraCasmRunner) = load_cairo! {
        use core::integer::u32_sqrt;
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(value: u32) -> u32 {
            u32_sqrt(value)
        }

        fn run_test(value: felt252) -> u32 {
            program(value.try_into().unwrap())
        }
    };

    // U64

    static ref U64_OVERFLOWING_ADD: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u64, rhs: u64) -> u64 {
            lhs + rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> u64 {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U64_OVERFLOWING_SUB: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u64, rhs: u64) -> u64 {
            lhs - rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> u64 {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U64_SAFE_DIVMOD: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u64, rhs: u64) -> (u64, u64) {
            let q = lhs / rhs;
            let r = lhs % rhs;

            (q, r)
        }

        fn run_test(lhs: felt252, rhs: felt252) -> (u64, u64) {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U64_EQUAL:(String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u64, rhs: u64) -> bool {
            lhs == rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> bool {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U64_IS_ZERO: (String, Program, SierraCasmRunner) = load_cairo! {
        use zeroable::IsZeroResult;

        extern fn u64_is_zero(a: u64) -> IsZeroResult<u64> implicits() nopanic;

        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(value: u64) -> bool {
            match u64_is_zero(value) {
                IsZeroResult::Zero(_) => true,
                IsZeroResult::NonZero(_) => false,
            }
        }

        fn run_test(value: felt252) -> bool {
            program(value.try_into().unwrap())
        }
    };
    static ref U64_SQRT: (String, Program, SierraCasmRunner) = load_cairo! {
        use core::integer::u64_sqrt;
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(value: u64) -> u64 {
            u64_sqrt(value)
        }

        fn run_test(value: felt252) -> u64 {
            program(value.try_into().unwrap())
        }
    };

    // U128

    static ref U128_OVERFLOWING_ADD: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u128, rhs: u128) -> u128 {
            lhs + rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> u128 {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U128_OVERFLOWING_SUB: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u128, rhs: u128) -> u128 {
            lhs - rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> u128 {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U128_SAFE_DIVMOD: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u128, rhs: u128) -> (u128, u128) {
            let q = lhs / rhs;
            let r = lhs % rhs;

            (q, r)
        }

        fn run_test(lhs: felt252, rhs: felt252) -> (u128, u128) {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U128_EQUAL: (String, Program, SierraCasmRunner) = load_cairo! {
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(lhs: u128, rhs: u128) -> bool {
            lhs == rhs
        }

        fn run_test(lhs: felt252, rhs: felt252) -> bool {
            program(lhs.try_into().unwrap(), rhs.try_into().unwrap())
        }
    };
    static ref U128_IS_ZERO: (String, Program, SierraCasmRunner) = load_cairo! {
        use zeroable::IsZeroResult;

        extern fn u128_is_zero(a: u128) -> IsZeroResult<u128> implicits() nopanic;

        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(value: u128) -> bool {
            match u128_is_zero(value) {
                IsZeroResult::Zero(_) => true,
                IsZeroResult::NonZero(_) => false,
            }
        }

        fn run_test(value: felt252) -> bool {
            program(value.try_into().unwrap())
        }
    };
    static ref U128_SQRT: (String, Program, SierraCasmRunner) = load_cairo! {
        use core::integer::u128_sqrt;
        use traits::TryInto;
        use core::option::OptionTrait;

        fn program(value: u128) -> u128 {
            u128_sqrt(value)
        }

        fn run_test(value: felt252) -> u128 {
            program(value.try_into().unwrap())
        }
    };
}

proptest! {
    #[test]
    fn u8_overflowing_add_proptest(a in 0..u8::MAX, b in 0..u8::MAX) {
        let program= &U8_OVERFLOWING_ADD;

  let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u8_overflowing_sub_proptest(a in 0..u8::MAX, b in 0..u8::MAX) {
        let program= &U8_OVERFLOWING_SUB;

        let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u8_safe_divmod_proptest(a in 0..u8::MAX, b in 0..u8::MAX) {
        let program= &U8_SAFE_DIVMOD;
        let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u8_equal_proptest(a in 0..u8::MAX, b in 0..u8::MAX) {
        let program= &U8_EQUAL;
        let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u8_is_zero_proptest(a in 0..u8::MAX) {
        let program= &U8_IS_ZERO;
let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    // u16

    #[test]
    fn u16_overflowing_add_proptest(a in 0..u16::MAX, b in 0..u16::MAX) {
        let program= &U16_OVERFLOWING_ADD;
let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u16_overflowing_sub_proptest(a in 0..u16::MAX, b in 0..u16::MAX) {
        let program= &U16_OVERFLOWING_SUB;
let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u16_safe_divmod_proptest(a in 0..u16::MAX, b in 0..u16::MAX) {
        let program= &U16_SAFE_DIVMOD;
        let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u16_equal_proptest(a in 0..u16::MAX, b in 0..u16::MAX) {
        let program= &U16_EQUAL;
       let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u16_is_zero_proptest(a in 0..u16::MAX) {
        let program= &U16_IS_ZERO;

        let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[Arg::Value(a.into())]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();


    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some(&[JITValue::Felt252(a.into())]), None, None, None).right().unwrap();

        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    // u32

    #[test]
    fn u32_overflowing_add_proptest(a in 0..u32::MAX, b in 0..u32::MAX) {
        let program= &U32_OVERFLOWING_ADD;
       let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u32_overflowing_sub_proptest(a in 0..u32::MAX, b in 0..u32::MAX) {
        let program= &U32_OVERFLOWING_SUB;
       let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u32_safe_divmod_proptest(a in 0..u32::MAX, b in 0..u32::MAX) {
        let program= &U32_SAFE_DIVMOD;
       let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u32_equal_proptest(a in 0..u32::MAX, b in 0..u32::MAX) {
        let program= &U32_EQUAL;
      let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u32_is_zero_proptest(a in 0..u32::MAX) {
        let program= &U32_IS_ZERO;
let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    // u64

    #[test]
    fn u64_overflowing_add_proptest(a in 0..u64::MAX, b in 0..u64::MAX) {
        let program= &U64_OVERFLOWING_ADD;
        let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u64_overflowing_sub_proptest(a in 0..u64::MAX, b in 0..u64::MAX) {
        let program= &U64_OVERFLOWING_SUB;
let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u64_safe_divmod_proptest(a in 0..u64::MAX, b in 0..u64::MAX) {
        let program= &U64_SAFE_DIVMOD;
       let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u64_equal_proptest(a in 0..u64::MAX, b in 0..u64::MAX) {
        let program= &U64_EQUAL;
      let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u64_is_zero_proptest(a in 0..u64::MAX) {
        let program= &U64_IS_ZERO;
       let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into())]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    // u128

    #[test]
    fn u128_overflowing_add_proptest(a in 0..u128::MAX, b in 0..u128::MAX) {
        let program= &U128_OVERFLOWING_ADD;
      let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u128_overflowing_sub_proptest(a in 0..u128::MAX, b in 0..u128::MAX) {
        let program= &U128_OVERFLOWING_SUB;
       let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u128_safe_divmod_proptest(a in 0..u128::MAX, b in 0..u128::MAX) {
        let program= &U128_SAFE_DIVMOD;
        let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u128_equal_proptest(a in 0..u128::MAX, b in 0..u128::MAX) {
        let program= &U128_EQUAL;
let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
            Arg::Value(b.into())
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into()), JITValue::Felt252(b.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }

    #[test]
    fn u128_is_zero_proptest(a in 0..u128::MAX) {
        let program= &U128_IS_ZERO;
        let (program_for_args, sierra_casm_runner) =
        ((program.0.clone(), program.1.clone()), program.2.borrow());

    let result_vm = run_native_or_vm_program(
        &program_for_args,
        "run_test",
        None,
        Some(&[
            Arg::Value(a.into()),
        ]),
        Some(sierra_casm_runner),
        Some(GAS),
    ).left().unwrap().unwrap();

    let result_native =
        run_native_or_vm_program(&program_for_args, "run_test", Some( &[JITValue::Felt252(a.into())]), None, None, None).right().unwrap();


        compare_outputs(
        &program_for_args.1,
        &sierra_casm_runner.find_function("run_test").unwrap().id,
            &result_vm,
            &result_native,
        )
        .unwrap();
    }
}
