use tw::{AsFunctionType, AsPointerType};

mod types;

pub fn compile() {
    Lliua::with_new(|lliua| {
        let main = lliua.module.add_function(
            "main",
            lliua.ctx.i32_type().function_type((), false),
            Some(tw::Linkage::External),
        );

        let entry = main.append_basic_block("entry");
        lliua.build_call_printf(
            &entry,
            "Hello world %d",
            &[lliua.ctx.u32_type().const_uint(123).into_basic_value_enum()],
        );
        entry.build_return(Some(lliua.ctx.i32_type().const_sint(456)));

        println!("{}", lliua.module.to_string());
    });
}

pub struct Lliua<'ctx> {
    ctx: &'ctx tw::Context,
    module: tw::Module<'ctx>,
    types: crate::types::Types<'ctx>,
    printf: tw::Value<'ctx, crate::types::PrintfType<'ctx>>,
}

impl Lliua<'_> {
    fn with_new<R>(block: impl FnOnce(&mut Lliua) -> R) -> R {
        let ctx = tw::Context::new();
        let module = ctx.create_module("lliua_program");
        let types = crate::types::Types::new(&ctx);

        let printf = module.add_function(
            "printf",
            ctx.i32_type()
                .function_type((ctx.u8_type().ptr_type(),), true),
            Some(tw::Linkage::External),
        );

        let mut lliua = Lliua {
            ctx: &ctx,
            module,
            types,
            printf,
        };

        block(&mut lliua)
    }
}

impl<'ctx> Lliua<'ctx> {
    fn build_call_printf<F>(
        &self,
        bb: &tw::BasicBlock<'ctx, F>,
        format: &str,
        values: &[tw::Value<'ctx, tw::BasicTypeEnum<'ctx>>],
    ) {
        let format_string = self.ctx.const_string(format!("{format}\0").as_bytes());
        let format_str_ptr = bb.build_alloca(*format_string.get_type(), "format_str");
        bb.build_store(format_str_ptr, format_string);

        bb.build_variadic_call(self.printf, &(format_str_ptr.decay_array(),), values, "");
    }
}
