crate::create_wrapper! {
    pub struct Value[T][T] => inkwell::values::AnyValueEnum<'ctx> {
        pub(crate) ty: T
    }
}
