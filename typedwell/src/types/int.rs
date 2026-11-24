use inkwell::values::AnyValue;

use crate::{HasContext, Wrapper};

pub type U64Type<'ctx> = IntType<'ctx, 64, false>;
pub type I64Type<'ctx> = IntType<'ctx, 64, true>;
pub type U32Type<'ctx> = IntType<'ctx, 32, false>;
pub type I32Type<'ctx> = IntType<'ctx, 32, true>;
pub type U8Type<'ctx> = IntType<'ctx, 8, false>;
pub type I8Type<'ctx> = IntType<'ctx, 8, true>;

crate::create_wrapper! {
    #[derive(Clone, Copy)]
    pub struct IntType[const BITS: usize, const SIGNED: bool][BITS, SIGNED] => inkwell::types::IntType<'ctx> {}
}

impl<'ctx, const BITS: usize> IntType<'ctx, BITS, false> {
    pub fn const_uint(&self, value: u64) -> crate::Value<'ctx, Self> {
        crate::Value {
            original: self
                .get_original()
                .const_int(value, false)
                .as_any_value_enum(),
            ctx: self.get_context(),
            ty: *self,
        }
    }
}

impl<'ctx, const BITS: usize> IntType<'ctx, BITS, true> {
    pub fn const_sint(&self, value: i64) -> crate::Value<'ctx, Self> {
        crate::Value {
            original: self
                .get_original()
                .const_int(value as u64, true)
                .as_any_value_enum(),
            ctx: self.get_context(),
            ty: *self,
        }
    }
}

impl<'ctx, F> crate::BasicBlock<'ctx, F> {
    pub fn build_int_mul<const BITS: usize, const SIGNED: bool>(
        &self,
        lhs: &crate::Value<'ctx, IntType<'ctx, BITS, SIGNED>>,
        rhs: &crate::Value<'ctx, IntType<'ctx, BITS, SIGNED>>,
        name: &str,
    ) -> crate::Value<'ctx, IntType<'ctx, BITS, SIGNED>> {
        crate::Value {
            original: self
                .builder
                .build_int_mul(
                    lhs.get_original().into_int_value(),
                    rhs.get_original().into_int_value(),
                    name,
                )
                .unwrap()
                .as_any_value_enum(),
            ctx: self.get_context(),
            ty: lhs.ty,
        }
    }
}
