use inkwell::values::AnyValue;

use crate::HasContext;

pub type F32Type<'ctx> = FloatType<'ctx, 32>;
pub type F64Type<'ctx> = FloatType<'ctx, 64>;

crate::create_wrapper! {
    #[derive(Clone, Copy)]
    pub struct FloatType[const BITS: usize][BITS] => inkwell::types::FloatType<'ctx> {}
}

impl<'ctx, const BITS: usize> FloatType<'ctx, BITS> {
    pub fn const_float(&self, value: f64) -> crate::Value<'ctx, Self> {
        crate::Value {
            original: self.original.const_float(value).as_any_value_enum(),
            ctx: self.get_context(),
            ty: *self,
        }
    }
}
