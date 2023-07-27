//! # Casting libfuncs
//!
//! TODO

use super::{LibfuncBuilder, LibfuncHelper};
use crate::{
    error::{
        libfuncs::{Error, Result},
        CoreTypeBuilderError,
    },
    metadata::MetadataStorage,
    types::TypeBuilder,
};
use cairo_lang_sierra::{
    extensions::{
        casts::{CastConcreteLibfunc, DowncastConcreteLibfunc},
        lib_func::SignatureOnlyConcreteLibfunc,
        ConcreteLibfunc, GenericLibfunc, GenericType,
    },
    program_registry::ProgramRegistry,
};
use melior::{
    dialect::arith::{self, CmpiPredicate},
    ir::{attribute::IntegerAttribute, r#type::IntegerType, Block, Location},
    Context,
};

/// Select and call the correct libfunc builder function from the selector.
pub fn build<'ctx, 'this, TType, TLibfunc>(
    context: &'ctx Context,
    registry: &ProgramRegistry<TType, TLibfunc>,
    entry: &'this Block<'ctx>,
    location: Location<'ctx>,
    helper: &LibfuncHelper<'ctx, 'this>,
    metadata: &mut MetadataStorage,
    selector: &CastConcreteLibfunc,
) -> Result<()>
where
    TType: GenericType,
    TLibfunc: GenericLibfunc,
    <TType as GenericType>::Concrete: TypeBuilder<TType, TLibfunc, Error = CoreTypeBuilderError>,
    <TLibfunc as GenericLibfunc>::Concrete: LibfuncBuilder<TType, TLibfunc, Error = Error>,
{
    match selector {
        CastConcreteLibfunc::Downcast(info) => {
            build_downcast(context, registry, entry, location, helper, metadata, info)
        }
        CastConcreteLibfunc::Upcast(info) => {
            build_upcast(context, registry, entry, location, helper, metadata, info)
        }
    }
}

/// Generate MLIR operations for the `downcast` libfunc.
pub fn build_downcast<'ctx, 'this, TType, TLibfunc>(
    context: &'ctx Context,
    registry: &ProgramRegistry<TType, TLibfunc>,
    entry: &'this Block<'ctx>,
    location: Location<'ctx>,
    helper: &LibfuncHelper<'ctx, 'this>,
    metadata: &mut MetadataStorage,
    info: &DowncastConcreteLibfunc,
) -> Result<()>
where
    TType: GenericType,
    TLibfunc: GenericLibfunc,
    <TType as GenericType>::Concrete: TypeBuilder<TType, TLibfunc, Error = CoreTypeBuilderError>,
    <TLibfunc as GenericLibfunc>::Concrete: LibfuncBuilder<TType, TLibfunc, Error = Error>,
{
    let src_ty = registry
        .get_type(&info.from_ty)?
        .build(context, helper, registry, metadata)?;
    let dst_ty = registry
        .get_type(&info.to_ty)?
        .build(context, helper, registry, metadata)?;
    assert!(info.from_nbits >= info.to_nbits);

    if info.from_nbits == info.to_nbits {
        let k0 = entry
            .append_operation(arith::constant(
                context,
                IntegerAttribute::new(0, IntegerType::new(context, 1).into()).into(),
                location,
            ))
            .result(0)?
            .into();

        entry.append_operation(helper.cond_br(
            k0,
            [1, 0],
            [
                &[entry.argument(0)?.into()],
                &[entry.argument(0)?.into(), entry.argument(1)?.into()],
            ],
            location,
        ));
    } else {
        let k1 = entry
            .append_operation(arith::constant(
                context,
                IntegerAttribute::new(1, src_ty).into(),
                location,
            ))
            .result(0)?
            .into();

        let n_bits = entry
            .append_operation(arith::constant(
                context,
                IntegerAttribute::new(info.to_nbits.try_into()?, src_ty).into(),
                location,
            ))
            .result(0)?
            .into();
        let max_value_plus_one = entry
            .append_operation(arith::shli(k1, n_bits, location))
            .result(0)?
            .into();

        let is_in_range = entry
            .append_operation(arith::cmpi(
                context,
                CmpiPredicate::Ult,
                entry.argument(1)?.into(),
                max_value_plus_one,
                location,
            ))
            .result(0)?
            .into();

        let result = entry
            .append_operation(arith::trunci(entry.argument(1)?.into(), dst_ty, location))
            .result(0)?
            .into();

        entry.append_operation(helper.cond_br(
            is_in_range,
            [0, 1],
            [
                &[entry.argument(0)?.into(), result],
                &[entry.argument(0)?.into()],
            ],
            location,
        ));
    };

    Ok(())
}

/// Generate MLIR operations for the `upcast` libfunc.
pub fn build_upcast<'ctx, 'this, TType, TLibfunc>(
    context: &'ctx Context,
    registry: &ProgramRegistry<TType, TLibfunc>,
    entry: &'this Block<'ctx>,
    location: Location<'ctx>,
    helper: &LibfuncHelper<'ctx, 'this>,
    _metadata: &mut MetadataStorage,
    info: &SignatureOnlyConcreteLibfunc,
) -> Result<()>
where
    TType: GenericType,
    TLibfunc: GenericLibfunc,
    <TType as GenericType>::Concrete: TypeBuilder<TType, TLibfunc, Error = CoreTypeBuilderError>,
    <TLibfunc as GenericLibfunc>::Concrete: LibfuncBuilder<TType, TLibfunc, Error = Error>,
{
    let src_ty = registry.get_type(&info.param_signatures()[0].ty)?;
    let dst_ty = registry.get_type(&info.branch_signatures()[0].vars[0].ty)?;

    let src_width = src_ty.integer_width().unwrap();
    let dst_width = dst_ty.integer_width().unwrap();
    assert!(src_width <= dst_width);

    let result = if src_width == dst_width {
        entry.argument(0)?.into()
    } else {
        entry
            .append_operation(arith::extui(
                entry.argument(0)?.into(),
                IntegerType::new(context, dst_width.try_into()?).into(),
                location,
            ))
            .result(0)?
            .into()
    };

    entry.append_operation(helper.br(0, &[result], location));
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::utils::test::{load_cairo, run_program};
    use cairo_lang_sierra::program::Program;
    use lazy_static::lazy_static;
    use serde_json::json;

    lazy_static! {
        static ref DOWNCAST: (String, Program) = load_cairo! {
            use core::integer::downcast;

            fn run_test(v8: u8, v16: u16, v32: u32, v64: u64, v128: u128) -> (
                (Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>),
                (Option<u16>, Option<u16>, Option<u16>, Option<u16>),
                (Option<u32>, Option<u32>, Option<u32>),
                (Option<u64>, Option<u64>),
                (Option<u128>,)
            ) {
                (
                    (downcast(v128), downcast(v64), downcast(v32), downcast(v16), downcast(v8)),
                    (downcast(v128), downcast(v64), downcast(v32), downcast(v16)),
                    (downcast(v128), downcast(v64), downcast(v32)),
                    (downcast(v128), downcast(v64)),
                    (downcast(v128),),
                )
            }
        };
        static ref UPCAST: (String, Program) = load_cairo! {
            use core::integer::upcast;

            fn run_test(v8: u8, v16: u16, v32: u32, v64: u64, v128: u128) -> (
                (u8,),
                (u16, u16),
                (u32, u32, u32),
                (u64, u64, u64, u64),
                (u128, u128, u128, u128, u128)
            ) {
                (
                    (upcast(v8),),
                    (upcast(v8), upcast(v16)),
                    (upcast(v8), upcast(v16), upcast(v32)),
                    (upcast(v8), upcast(v16), upcast(v32), upcast(v64)),
                    (upcast(v8), upcast(v16), upcast(v32), upcast(v64), upcast(v128)),
                )
            }
        };
    }

    #[test]
    fn downcast() {
        let r = |v8, v16, v32, v64, v128| {
            run_program(&DOWNCAST, "run_test", json!([(), v8, v16, v32, v64, v128]))
        };

        assert_eq!(
            r(u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX),
            json!([
                (),
                [
                    [[1, []], [1, []], [1, []], [1, []], [0, u8::MAX]],
                    [[1, []], [1, []], [1, []], [0, u16::MAX]],
                    [[1, []], [1, []], [0, u32::MAX]],
                    [[1, []], [0, u64::MAX]],
                    [[0, u128::MAX]],
                ]
            ])
        );
    }

    #[test]
    fn upcast() {
        assert_eq!(
            run_program(
                &UPCAST,
                "run_test",
                json!([u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX])
            ),
            json!([[
                [u8::MAX],
                [u8::MAX, u16::MAX],
                [u8::MAX, u16::MAX, u32::MAX],
                [u8::MAX, u16::MAX, u32::MAX, u64::MAX],
                [u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX],
            ]])
        );
    }
}
