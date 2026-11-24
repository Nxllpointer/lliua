use crate::{HasContext, Wrapper};

pub use inkwell::module::Linkage;
use inkwell::values::AnyValue;

crate::create_wrapper! {
    pub struct Module => inkwell::module::Module<'ctx> {}
}

impl<'ctx> Module<'ctx> {
    pub fn add_function<T>(
        &self,
        name: &str,
        ty: T,
        linkage: Option<Linkage>,
    ) -> crate::Value<'ctx, T>
    where
        T: crate::Wrapper<'ctx, Original = inkwell::types::FunctionType<'ctx>>,
    {
        crate::Value {
            original: self
                .get_original()
                .add_function(name, *ty.get_original(), linkage)
                .as_any_value_enum(),
            ctx: self.get_context(),
            ty,
        }
    }
}
