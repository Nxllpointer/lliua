use crate::{HasContext, Wrapper};

crate::create_wrapper! {
    #[derive(Clone, Copy)]
    pub struct AnyTypeEnum => inkwell::types::AnyTypeEnum<'ctx> {}
}

pub trait AnyType<'ctx>:
    crate::Wrapper<'ctx, Original: inkwell::types::AnyType<'ctx>> + crate::HasContext<'ctx> + Copy
{
}
impl<'ctx, T> AnyType<'ctx> for T
where
    T: crate::Wrapper<'ctx, Original: inkwell::types::AnyType<'ctx>>,
    T: crate::HasContext<'ctx>,
    T: Copy,
{
}

impl<'ctx> crate::Value<'ctx, AnyTypeEnum<'ctx>> {
    pub unsafe fn into_typed<T>(self, ty: T) -> crate::Value<'ctx, T> {
        crate::Value {
            ty,
            ctx: self.get_context(),
            original: self.into_original(),
        }
    }
}
