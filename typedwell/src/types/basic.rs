use inkwell::types::BasicType as OriginalBasicType;

use crate::HasContext;

crate::create_wrapper! {
    #[derive(Clone, Copy)]
    pub struct BasicTypeEnum => inkwell::types::BasicTypeEnum<'ctx> {}
}

pub trait BasicType<'ctx>: crate::AnyType<'ctx, Original: inkwell::types::BasicType<'ctx>> {}
impl<'ctx, T> BasicType<'ctx> for T where
    T: crate::AnyType<'ctx, Original: inkwell::types::BasicType<'ctx>>
{
}

impl<'ctx, T: BasicType<'ctx>> crate::Value<'ctx, T> {
    pub fn into_basic_value_enum(self) -> crate::Value<'ctx, BasicTypeEnum<'ctx>> {
        crate::Value {
            original: self.original,
            ctx: self.get_context(),
            ty: BasicTypeEnum {
                original: self.ty.into_original().as_basic_type_enum(),
                ctx: self.ty.get_context(),
            },
        }
    }
}
