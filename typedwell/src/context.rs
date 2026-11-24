use crate::Wrapper;

pub struct Context {
    pub(crate) original: inkwell::context::Context,
}

impl Context {
    pub fn new() -> Self {
        Self {
            original: inkwell::context::Context::create(),
        }
    }

    pub fn create_module(&self, name: &str) -> crate::Module<'_> {
        crate::Module {
            original: self.get_original().create_module(name),
            ctx: self,
        }
    }

    // pub fn create_builder(&self) -> crate::Builder<'_> {
    //     crate::Builder {
    //         original: self.get_original().create_builder(),
    //         ctx: self,
    //     }
    // }

    pub fn u32_type(&self) -> crate::IntType<'_, 32, false> {
        crate::IntType {
            original: self.get_original().i32_type(),
            ctx: self,
        }
    }

    pub fn i32_type(&self) -> crate::IntType<'_, 32, true> {
        crate::IntType {
            original: self.get_original().i32_type(),
            ctx: self,
        }
    }
}

impl<'ctx: 'static> crate::Wrapper<'ctx> for Context {
    type Original = inkwell::context::Context;

    fn into_original(self) -> Self::Original {
        self.original
    }

    fn get_original(&self) -> &Self::Original {
        &self.original
    }
}
