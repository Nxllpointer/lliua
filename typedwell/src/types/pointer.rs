use inkwell::values::AnyValue;

use crate::{HasContext, Wrapper};

crate::create_wrapper! {
    #[derive(Clone, Copy)]
    pub struct PointerType[T][T] => inkwell::types::PointerType<'ctx> {
        pub(crate) pointee_ty: T
    }
}

pub trait AsPointerType<'ctx>: Sized {
    fn ptr_type(&self) -> PointerType<'ctx, Self>;
}

impl<'ctx, T: crate::BasicType<'ctx>> AsPointerType<'ctx> for T {
    fn ptr_type(&self) -> PointerType<'ctx, T> {
        PointerType {
            original: self
                .get_context()
                .get_original()
                .ptr_type(inkwell::AddressSpace::default()),
            ctx: self.get_context(),
            pointee_ty: *self,
        }
    }
}

impl<'ctx, T: crate::AnyType<'ctx>> PointerType<'ctx, T> {
    pub fn const_null(&self) -> crate::Value<'ctx, Self> {
        crate::Value {
            original: self.original.const_null().as_any_value_enum(),
            ctx: self.get_context(),
            ty: *self,
        }
    }
}

impl<'ctx, T: crate::BasicType<'ctx>>
    crate::Value<'ctx, PointerType<'ctx, crate::ArrayType<'ctx, T>>>
{
    pub fn decay_array(&self) -> crate::Value<'ctx, PointerType<'ctx, T>> {
        crate::Value {
            original: self.original,
            ctx: self.get_context(),
            ty: self.ty.pointee_ty.element_ty.ptr_type(),
        }
    }
}

impl<'ctx, F> crate::BasicBlock<'ctx, F> {
    pub fn build_load<T: crate::BasicType<'ctx>>(
        &self,
        ptr: &crate::Value<'ctx, PointerType<T>>,
        name: &str,
    ) -> crate::Value<'ctx, T> {
        let loaded_value = self
            .builder
            .build_load(
                ptr.ty.pointee_ty.into_original(),
                ptr.get_original().into_pointer_value(),
                name,
            )
            .unwrap();
        crate::Value {
            original: loaded_value.as_any_value_enum(),
            ctx: self.get_context(),
            ty: ptr.ty.pointee_ty,
        }
    }

    pub fn build_store<T: crate::BasicType<'ctx>>(
        &self,
        ptr: &crate::Value<'ctx, PointerType<T>>,
        value: &crate::Value<'ctx, T>,
    ) {
        self.builder
            .build_store(
                ptr.get_original().into_pointer_value(),
                inkwell::values::BasicValueEnum::try_from(*value.get_original()).unwrap(),
            )
            .unwrap();
    }
}
