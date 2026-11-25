use inkwell::{types::BasicType, values::AnyValue};

use crate::{AsPointerType, HasContext};

crate::create_wrapper! {
    pub struct BasicBlock[F][F] => inkwell::basic_block::BasicBlock<'ctx> {
        pub(crate) builder: inkwell::builder::Builder<'ctx>,
        pub(crate) function_ty: F
    }
}

impl<'ctx, F> BasicBlock<'ctx, F> {
    pub fn build_alloca<T: crate::BasicType<'ctx>>(
        &self,
        ty: T,
        name: &str,
    ) -> crate::Value<'ctx, crate::PointerType<'ctx, T>> {
        crate::Value {
            original: self
                .builder
                .build_alloca(ty.get_original().as_basic_type_enum(), name)
                .unwrap()
                .as_any_value_enum(),
            ctx: self.get_context(),
            ty: ty.ptr_type(),
        }
    }
}
