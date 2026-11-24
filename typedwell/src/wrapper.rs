pub trait Wrapper<'ctx> {
    type Original;
    fn into_original(self) -> Self::Original;
    fn get_original(&self) -> &Self::Original;
}

pub trait HasContext<'ctx> {
    fn get_context(&self) -> &'ctx crate::Context;
}

#[macro_export]
macro_rules! create_wrapper {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident $(
            [$($genericsdef:tt)*]
            [$($genericsuse:tt)*]
        )? => $original:ty {
            $($fields:tt)*
        }
    ) => {
        $(#[$attr])*
        $vis struct $name<'ctx, $($($genericsdef)*)?> {
            pub(crate) original: <Self as crate::Wrapper<'ctx>>::Original,
            pub(crate) ctx: &'ctx crate::Context,
            $($fields)*
        }

        impl<'ctx, $($($genericsdef)*)?> crate::Wrapper<'ctx> for $name<'ctx, $($($genericsuse)*)?> {
            type Original = $original;

            fn into_original(self) -> Self::Original {
                self.original
            }

            fn get_original(&self) -> &Self::Original {
                &self.original
            }
        }

        impl<'ctx, $($($genericsdef)*)?> crate::HasContext<'ctx> for $name<'ctx, $($($genericsuse)*)?> {
            fn get_context(&self) -> &'ctx crate::Context {
                self.ctx
            }
        }

        impl<'ctx, $($($genericsdef)*)?> ::std::fmt::Debug for $name<'ctx, $($($genericsuse)*)?> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.original.fmt(f)
            }
        }
    };
}
