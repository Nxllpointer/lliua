crate::create_wrapper! {
    pub struct BasicBlock[F][F] => inkwell::basic_block::BasicBlock<'ctx> {
        pub(crate) builder: inkwell::builder::Builder<'ctx>,
        pub(crate) function_ty: F
    }
}
