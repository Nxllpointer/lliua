crate::create_wrapper! {
    #[derive(Clone, Copy)]
    pub struct BasicTypeEnum => inkwell::types::BasicTypeEnum<'ctx> {}
}

pub trait BasicType<'ctx>: crate::AnyType<'ctx, Original: inkwell::types::BasicType<'ctx>> {}
impl<'ctx, T> BasicType<'ctx> for T where
    T: crate::AnyType<'ctx, Original: inkwell::types::BasicType<'ctx>>
{
}
