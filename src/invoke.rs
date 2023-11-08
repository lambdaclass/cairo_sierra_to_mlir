//! A Rusty interface to provide parameters to JIT calls.

use std::{alloc::Layout, ptr::NonNull};

use bumpalo::Bump;
use cairo_felt::Felt252;
use cairo_lang_sierra::{
    extensions::core::{CoreLibfunc, CoreType, CoreTypeConcrete},
    ids::ConcreteTypeId,
    program_registry::ProgramRegistry,
};

use crate::{
    metadata::syscall_handler::SyscallHandlerMeta,
    types::TypeBuilder,
    utils::{felt252_bigint, get_integer_layout, u32_vec_to_felt},
};

#[derive(Debug, Clone)]
pub enum InvokeArg {
    Felt252(Felt252),
    Array(Vec<Self>),  // all elements need to be same type
    Struct(Vec<Self>), // element types can differ
    Span(Vec<Self>),   // like a array, used specially when passing parameters to contracts
    Enum { tag: u64, value: Box<Self> },
    Box(Box<Self>), // can't be null
    Nullable(Option<Box<Self>>),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
}

#[derive(Debug, Default)]
pub struct InvokeContext<'s> {
    pub gas: Option<u128>,
    // Starknet syscall handler
    pub system: Option<&'s SyscallHandlerMeta>,
    pub bitwise: bool,
    pub range_check: bool,
    pub pedersen: bool,
    // call args
    pub args: Vec<InvokeArg>,
}

#[derive(Debug, Default)]
pub struct InvokeResult {
    pub gas: Option<u128>,
    pub outputs: Vec<InvokeArg>,
}

// Conversions

impl From<Felt252> for InvokeArg {
    fn from(value: Felt252) -> Self {
        InvokeArg::Felt252(value)
    }
}

impl From<u8> for InvokeArg {
    fn from(value: u8) -> Self {
        InvokeArg::Uint8(value)
    }
}

impl From<u16> for InvokeArg {
    fn from(value: u16) -> Self {
        InvokeArg::Uint16(value)
    }
}

impl From<u32> for InvokeArg {
    fn from(value: u32) -> Self {
        InvokeArg::Uint32(value)
    }
}

impl From<u64> for InvokeArg {
    fn from(value: u64) -> Self {
        InvokeArg::Uint64(value)
    }
}

impl From<u128> for InvokeArg {
    fn from(value: u128) -> Self {
        InvokeArg::Uint128(value)
    }
}

impl InvokeArg {
    pub fn to_jit(
        &self,
        arena: &Bump,
        registry: &ProgramRegistry<CoreType, CoreLibfunc>,
        type_id: &ConcreteTypeId,
    ) -> NonNull<()> {
        let ty = registry.get_type(type_id).unwrap();

        unsafe {
            match self {
                InvokeArg::Felt252(value) => {
                    let ptr = arena.alloc_layout(get_integer_layout(252)).cast();

                    let data = felt252_bigint(value.to_bigint());
                    ptr.cast::<[u32; 8]>().as_mut().copy_from_slice(&data);
                    ptr
                }
                InvokeArg::Array(_) => todo!(),
                InvokeArg::Struct(members) => {
                    if let CoreTypeConcrete::Struct(info) = ty {
                        let mut layout: Option<Layout> = None;
                        let mut data = Vec::with_capacity(info.members.len());

                        for (member_type_id, member) in info.members.iter().zip(members) {
                            let member_ty = registry.get_type(member_type_id).unwrap();
                            let member_layout = member_ty.layout(registry).unwrap();

                            let (new_layout, offset) = match layout {
                                Some(layout) => layout.extend(member_layout).unwrap(),
                                None => (member_layout, 0),
                            };
                            layout = Some(new_layout);

                            data.push((
                                member_layout,
                                offset,
                                member.to_jit(arena, registry, member_type_id),
                            ));
                        }

                        let ptr = arena
                            .alloc_layout(layout.unwrap_or(Layout::new::<()>()))
                            .cast();

                        for (layout, offset, member_ptr) in data {
                            std::ptr::copy_nonoverlapping(
                                member_ptr.cast::<u8>().as_ptr(),
                                NonNull::new(((ptr.as_ptr() as usize) + offset) as *mut u8)
                                    .unwrap()
                                    .cast()
                                    .as_ptr(),
                                layout.size(),
                            );
                        }

                        ptr
                    } else {
                        panic!("wrong type")
                    }
                }
                InvokeArg::Span(_) => todo!(),
                InvokeArg::Enum { tag, value } => todo!(),
                InvokeArg::Box(_) => todo!(),
                InvokeArg::Nullable(_) => todo!(),
                InvokeArg::Uint8(value) => {
                    let ptr = arena.alloc_layout(Layout::new::<u8>()).cast();
                    *ptr.cast::<u8>().as_mut() = *value;

                    ptr
                }
                InvokeArg::Uint16(value) => {
                    let ptr = arena.alloc_layout(Layout::new::<u16>()).cast();
                    *ptr.cast::<u16>().as_mut() = *value;

                    ptr
                }
                InvokeArg::Uint32(value) => {
                    let ptr = arena.alloc_layout(Layout::new::<u32>()).cast();
                    *ptr.cast::<u32>().as_mut() = *value;

                    ptr
                }
                InvokeArg::Uint64(value) => {
                    let ptr = arena.alloc_layout(Layout::new::<u64>()).cast();
                    *ptr.cast::<u64>().as_mut() = *value;

                    ptr
                }
                InvokeArg::Uint128(value) => {
                    let ptr = arena.alloc_layout(Layout::new::<u128>()).cast();
                    *ptr.cast::<u128>().as_mut() = *value;

                    ptr
                }
            }
        }
    }

    pub fn from_jit(
        ptr: NonNull<()>,
        type_id: &ConcreteTypeId,
        registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    ) -> InvokeArg {
        let ty = registry.get_type(type_id).unwrap();

        unsafe {
            match ty {
                CoreTypeConcrete::Array(_) => todo!(),
                CoreTypeConcrete::Bitwise(_) => todo!(),
                CoreTypeConcrete::Box(_) => todo!(),
                CoreTypeConcrete::EcOp(_) => todo!(),
                CoreTypeConcrete::EcPoint(_) => todo!(),
                CoreTypeConcrete::EcState(_) => todo!(),
                CoreTypeConcrete::Felt252(_) => {
                    let data = ptr.cast::<[u32; 8]>().as_ref();
                    let data = u32_vec_to_felt(data);
                    InvokeArg::Felt252(data)
                }
                CoreTypeConcrete::GasBuiltin(_) => todo!(),
                CoreTypeConcrete::BuiltinCosts(_) => todo!(),
                CoreTypeConcrete::Uint8(_) => InvokeArg::Uint8(*ptr.cast::<u8>().as_ref()),
                CoreTypeConcrete::Uint16(_) => InvokeArg::Uint16(*ptr.cast::<u16>().as_ref()),
                CoreTypeConcrete::Uint32(_) => InvokeArg::Uint32(*ptr.cast::<u32>().as_ref()),
                CoreTypeConcrete::Uint64(_) => InvokeArg::Uint64(*ptr.cast::<u64>().as_ref()),
                CoreTypeConcrete::Uint128(_) => InvokeArg::Uint128(*ptr.cast::<u128>().as_ref()),
                CoreTypeConcrete::Uint128MulGuarantee(_) => todo!(),
                CoreTypeConcrete::Sint8(_) => todo!(),
                CoreTypeConcrete::Sint16(_) => todo!(),
                CoreTypeConcrete::Sint32(_) => todo!(),
                CoreTypeConcrete::Sint64(_) => todo!(),
                CoreTypeConcrete::Sint128(_) => todo!(),
                CoreTypeConcrete::NonZero(_) => todo!(),
                CoreTypeConcrete::Nullable(_) => todo!(),
                CoreTypeConcrete::RangeCheck(_) => todo!(),
                CoreTypeConcrete::Uninitialized(_) => todo!(),
                CoreTypeConcrete::Enum(_) => todo!(),
                CoreTypeConcrete::Struct(info) => {
                    let mut layout: Option<Layout> = None;
                    let mut members = Vec::with_capacity(info.members.len());

                    for member_ty in &info.members {
                        let member = registry.get_type(member_ty).unwrap();
                        let member_layout = member.layout(registry).unwrap();

                        let (new_layout, offset) = match layout {
                            Some(layout) => layout.extend(member_layout).unwrap(),
                            None => (member_layout, 0),
                        };
                        layout = Some(new_layout);

                        members.push(InvokeArg::from_jit(
                            NonNull::new(((ptr.as_ptr() as usize) + offset) as *mut ()).unwrap(),
                            member_ty,
                            registry,
                        ));
                    }

                    InvokeArg::Struct(members)
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
                CoreTypeConcrete::Bytes31(_) => todo!(),
            }
        }
    }
}
