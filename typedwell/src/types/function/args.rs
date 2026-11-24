use std::borrow::Borrow;

use impl_trait_for_tuples::impl_for_tuples;
use inkwell::{
    types::{AnyType, BasicType},
    values::AnyValue,
};

use crate::Wrapper;

pub trait FunctionArgs<'ctx>: Copy {
    type CallValues: ?Sized;
    type BodyValues;

    fn arg_types(&self) -> impl Borrow<[inkwell::types::BasicMetadataTypeEnum<'ctx>]>;
    fn call_values(
        values: &Self::CallValues,
    ) -> impl Borrow<[inkwell::values::BasicMetadataValueEnum<'ctx>]>;
    /// SAFETY: values has the same length and types as self
    unsafe fn body_values(
        &self,
        values: &impl Borrow<[inkwell::values::BasicMetadataValueEnum<'ctx>]>,
    ) -> Self::BodyValues;
}

impl<'ctx, T: crate::BasicType<'ctx>> FunctionArgs<'ctx> for &[T] {
    type CallValues = [crate::value::Value<'ctx, T>];
    type BodyValues = Vec<crate::value::Value<'ctx, T>>;

    fn arg_types(&self) -> impl Borrow<[inkwell::types::BasicMetadataTypeEnum<'ctx>]> {
        self.into_iter()
            .map(|ty| ty.get_original().as_basic_type_enum().into())
            .collect::<Vec<_>>()
    }

    fn call_values(
        values: &Self::CallValues,
    ) -> impl Borrow<[inkwell::values::BasicMetadataValueEnum<'ctx>]> {
        values
            .into_iter()
            .map(|value| {
                value
                    .get_original()
                    .as_any_value_enum()
                    .try_into()
                    .expect("Value was not properly restricted by trait bounds")
            })
            .collect::<Vec<_>>()
    }

    unsafe fn body_values(
        &self,
        values: &impl Borrow<[inkwell::values::BasicMetadataValueEnum<'ctx>]>,
    ) -> Self::BodyValues {
        assert_eq!(values.borrow().len(), self.len());

        values
            .borrow()
            .into_iter()
            .zip(self.into_iter().cloned())
            .map(|(value, ty)| {
                assert_eq!(
                    value.as_any_value_enum().get_type(),
                    ty.get_original().as_any_type_enum()
                );
                crate::Value {
                    original: value.as_any_value_enum(),
                    ctx: ty.get_context(),
                    ty,
                }
            })
            .collect::<Vec<_>>()
    }
}

#[impl_for_tuples(12)]
#[tuple_types_no_default_trait_bound]
impl<'ctx> FunctionArgs<'ctx> for Tuple {
    for_tuples!( where #( Tuple: crate::BasicType<'ctx> ),* );

    for_tuples!( type CallValues = (#( crate::value::Value<'ctx, Tuple> ),*); );
    for_tuples!( type BodyValues = (#( crate::value::Value<'ctx, Tuple> ),*); );

    fn arg_types(&self) -> impl Borrow<[inkwell::types::BasicMetadataTypeEnum<'ctx>]> {
        [for_tuples!(#(
            self.Tuple.get_original().as_basic_type_enum().into()
        ),*)]
    }
    fn call_values(
        values: &Self::CallValues,
    ) -> impl Borrow<[inkwell::values::BasicMetadataValueEnum<'ctx>]> {
        [for_tuples!(#(
            values.Tuple
                .get_original()
                .as_any_value_enum()
                .try_into()
                .unwrap()
        ),*)]
    }
    unsafe fn body_values(
        &self,
        values: &impl Borrow<[inkwell::values::BasicMetadataValueEnum<'ctx>]>,
    ) -> Self::BodyValues {
        const SELF_LEN: usize = <[()]>::len(&[for_tuples!(#(()),*)]);
        assert_eq!(values.borrow().len(), SELF_LEN);

        let mut values_iter = values.borrow().into_iter();
        (for_tuples!(#({
            let original = values_iter.next().unwrap().as_any_value_enum();
            let ty = self.Tuple;

            assert_eq!(original.get_type(), ty.get_original().as_any_type_enum());

            crate::Value {
                original,
                ctx: ty.get_context(),
                ty
            }
        }),*))
    }
}
