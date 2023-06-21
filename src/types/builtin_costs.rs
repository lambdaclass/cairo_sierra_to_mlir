use super::TypeBuilder;
use crate::metadata::MetadataStorage;
use cairo_lang_sierra::{
    extensions::{types::InfoOnlyConcreteType, GenericLibfunc, GenericType},
    program_registry::ProgramRegistry,
};
use melior::{
    dialect::llvm,
    ir::{r#type::IntegerType, Module, Type},
    Context,
};

pub fn build<'ctx, TType, TLibfunc>(
    context: &'ctx Context,
    _module: &Module<'ctx>,
    _registry: &ProgramRegistry<TType, TLibfunc>,
    _metadata: &mut MetadataStorage,
    _info: &InfoOnlyConcreteType,
) -> Result<Type<'ctx>, std::convert::Infallible>
where
    TType: GenericType,
    TLibfunc: GenericLibfunc,
    <TType as GenericType>::Concrete: TypeBuilder,
{
    Ok(llvm::r#type::array(IntegerType::new(context, 8).into(), 0))
}
