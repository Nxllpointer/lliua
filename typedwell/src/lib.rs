mod basic_block;
mod context;
mod module;
mod types;
mod value;
mod wrapper;

pub use basic_block::*;
pub use context::*;
pub use module::*;
pub use types::*;
pub use value::*;

pub(crate) use wrapper::*;

#[cfg(test)]
mod test {
    use crate::{AsFunctionType, AsPointerType};

    #[test]
    fn integration_test() {
        let ctx = crate::Context::new();
        let module = ctx.create_module("testmod");
        let main_function = module.add_function(
            "main",
            ctx.i32_type().function_type(
                (ctx.i32_type(), ctx.u32_type(), ctx.i32_type().ptr_type()),
                false,
            ),
            None,
        );
        let (_, _, ptr_arg) = main_function.get_args();
        let entry = main_function.append_basic_block("entry");
        entry.build_call(
            main_function,
            &(
                ctx.i32_type().const_sint(123),
                ctx.u32_type().const_uint(456),
                ctx.i32_type().ptr_type().const_null(),
            ),
            "call_result",
        );
        let loaded_ptr = entry.build_load(ptr_arg, "loaded_ptr");
        entry.build_return(Some(loaded_ptr));

        module.original.print_to_stderr();
    }
}
