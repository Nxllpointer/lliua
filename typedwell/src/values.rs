crate::create_wrapper! {
    pub struct Value[T][T] => inkwell::values::AnyValueEnum<'ctx> {
        pub(crate) ty: T
    }
}

impl<'ctx, T> Value<'ctx, T> {
    pub fn get_type(&self) -> &T {
        &self.ty
    }
}
