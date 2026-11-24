use inkwell::{types::BasicType, values::AnyValue};

use crate::{HasContext, Wrapper};

pub type StringType<'ctx> = ArrayType<'ctx, crate::U8Type<'ctx>>;

crate::create_wrapper! {
    #[derive(Clone, Copy)]
    pub struct ArrayType[T][T] => inkwell::types::ArrayType<'ctx> {
        element_ty: T
    }
}

pub trait AsArrayType<'ctx>: Sized {
    fn array_type(&self, size: u32) -> ArrayType<'ctx, Self>;
}

impl<'ctx, T> AsArrayType<'ctx> for T
where
    T: crate::BasicType<'ctx>,
{
    fn array_type(&self, size: u32) -> ArrayType<'ctx, Self> {
        ArrayType {
            original: self.into_original().array_type(size),
            ctx: self.get_context(),
            element_ty: *self,
        }
    }
}
