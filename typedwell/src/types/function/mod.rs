use std::borrow::Borrow;

use inkwell::{types::BasicType, values::AnyValue};

use crate::{HasContext, Wrapper};

mod args;

crate::create_wrapper! {
    #[derive(Clone, Copy)]
    pub struct FunctionType[A: args::FunctionArgs<'ctx>, R: crate::AnyType<'ctx>][A, R] => inkwell::types::FunctionType<'ctx> {
        param_types: A,
        return_ty: R,
    }
}

crate::create_wrapper! {
    #[derive(Clone, Copy)]
    pub struct CallSiteType[R][R] => inkwell::types::AnyTypeEnum<'ctx> {}
}

pub trait AsFunctionType<'ctx, A, R>
where
    A: args::FunctionArgs<'ctx>,
    R: crate::AnyType<'ctx>,
{
    fn function_type(&self, param_types: A, is_var_args: bool) -> FunctionType<'ctx, A, R>;
}

impl<'ctx, A, R> AsFunctionType<'ctx, A, R> for R
where
    A: args::FunctionArgs<'ctx>,
    R: crate::AnyType<'ctx>,
    R::Original: inkwell::types::BasicType<'ctx>,
{
    fn function_type(&self, param_types: A, is_var_args: bool) -> FunctionType<'ctx, A, R> {
        FunctionType {
            original: {
                let original_param_types = param_types.arg_types();
                self.get_original()
                    .fn_type(original_param_types.borrow(), is_var_args)
            },
            ctx: self.get_context(),
            param_types,
            return_ty: *self,
        }
    }
}

impl<'ctx, A, R> crate::Value<'ctx, crate::FunctionType<'ctx, A, R>>
where
    A: args::FunctionArgs<'ctx>,
    R: crate::AnyType<'ctx>,
{
    pub fn get_args(&self) -> A::BodyValues {
        let original_args: Vec<inkwell::values::BasicMetadataValueEnum> = self
            .get_original()
            .into_function_value()
            .get_param_iter()
            .map(|arg| arg.into())
            .collect();

        unsafe { self.ty.param_types.body_values(&original_args) }
    }
}

impl<'ctx, P, R> crate::Value<'ctx, crate::FunctionType<'ctx, P, R>>
where
    P: args::FunctionArgs<'ctx>,
    R: crate::AnyType<'ctx>,
{
    pub fn append_basic_block(
        &self,
        name: &str,
    ) -> crate::BasicBlock<'ctx, crate::FunctionType<'ctx, P, R>> {
        let original = self
            .get_context()
            .get_original()
            .append_basic_block(self.get_original().into_function_value(), name);

        let builder = self.get_context().get_original().create_builder();
        builder.position_at_end(original);

        crate::BasicBlock {
            original,
            ctx: &self.get_context(),
            function_ty: self.ty,
            builder,
        }
    }
}
impl<'ctx, A: args::FunctionArgs<'ctx>, R: crate::BasicType<'ctx>>
    crate::BasicBlock<'ctx, crate::FunctionType<'ctx, A, R>>
{
    pub fn build_return(&self, value: Option<crate::Value<R>>) {
        match value {
            Some(value) => self
                .builder
                .build_return(Some(
                    &inkwell::values::BasicValueEnum::try_from(*value.get_original()).unwrap(),
                ))
                .unwrap(),
            None => self.builder.build_return(None).unwrap(),
        };
    }
}

impl<'ctx, F> crate::BasicBlock<'ctx, F> {
    pub fn build_call<P, R>(
        &self,
        function: crate::Value<'ctx, crate::FunctionType<'ctx, P, R>>,
        args: &P::CallValues,
        name: &str,
    ) -> crate::Value<'ctx, CallSiteType<'ctx, R>>
    where
        P: args::FunctionArgs<'ctx>,
        R: crate::AnyType<'ctx>,
    {
        self.build_variadic_call(function, args, &[], name)
    }

    pub fn build_variadic_call<P, R>(
        &self,
        function: crate::Value<'ctx, crate::FunctionType<'ctx, P, R>>,
        args: &P::CallValues,
        varargs: &[crate::Value<'ctx, crate::BasicTypeEnum<'ctx>>],
        name: &str,
    ) -> crate::Value<'ctx, CallSiteType<'ctx, R>>
    where
        P: args::FunctionArgs<'ctx>,
        R: crate::AnyType<'ctx>,
    {
        let combined_args: Vec<inkwell::values::BasicMetadataValueEnum> = std::iter::chain(
            P::call_values(args).borrow().iter().copied(),
            varargs.iter().map(|arg| {
                inkwell::values::BasicMetadataValueEnum::try_from(arg.original).unwrap()
            }),
        )
        .collect();

        let original = self
            .builder
            .build_call(
                function.get_original().into_function_value(),
                &combined_args,
                name,
            )
            .unwrap();

        crate::Value {
            original: original.as_any_value_enum(),
            ctx: self.get_context(),
            ty: CallSiteType {
                original: original.as_any_value_enum().get_type(),
                ctx: self.get_context(),
            },
        }
    }
}
