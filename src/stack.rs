pub struct Stack<'ctx> {
    module: inkwell::module::Module<'ctx>,
    pub vstack: Vec<inkwell::values::FloatValue<'ctx>>,
}

impl<'ctx> Stack<'ctx> {
    pub fn create(module: &inkwell::module::Module<'ctx>) -> Self {
        Self {
            module: module.clone(),
            vstack: Vec::default(),
        }
    }
}
