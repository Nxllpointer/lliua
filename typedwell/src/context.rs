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

    pub fn u32_type(&self) -> crate::U32Type<'_> {
        crate::IntType {
            original: self.get_original().i32_type(),
            ctx: self,
        }
    }

    pub fn i32_type(&self) -> crate::I32Type<'_> {
        crate::IntType {
            original: self.get_original().i32_type(),
            ctx: self,
        }
    }

    pub fn u8_type(&self) -> crate::U8Type<'_> {
        crate::IntType {
            original: self.get_original().i8_type(),
            ctx: self,
        }
    }

    pub fn i8_type(&self) -> crate::I8Type<'_> {
        crate::IntType {
            original: self.get_original().i8_type(),
            ctx: self,
        }
    }

    pub fn f32_type(&self) -> crate::F32Type<'_> {
        crate::FloatType {
            original: self.get_original().f32_type(),
            ctx: self,
        }
    }

    pub fn f64_type(&self) -> crate::F64Type<'_> {
        crate::FloatType {
            original: self.get_original().f64_type(),
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
