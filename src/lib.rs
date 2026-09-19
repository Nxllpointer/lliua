mod stack;

struct LliuaBackend<'a, 'ctx> {
    ctx: inkwell::context::ContextRef<'ctx>,
    module: &'a inkwell::module::Module<'ctx>,
    types: Types<'ctx>,
    rand_global: inkwell::values::GlobalValue<'ctx>,
}

struct Types<'ctx> {
    real: inkwell::types::FloatType<'ctx>,
}

pub fn transform(uasm: uiua::Assembly) {
    let ctx = inkwell::context::Context::create();
    let module = ctx.create_module("uiua");

    let types = Types {
        real: ctx.f64_type(),
    };

    let rand_global = module.add_global(types.real, None, "rand");
    rand_global.set_initializer(&ctx.f64_type().const_float(0.5));

    let lliua_backend = LliuaBackend {
        ctx: module.get_context(),
        module: &module,
        types,
        rand_global,
    };

    lliua_backend.entrypoint(uasm.root);

    lliua_backend.module.print_to_stderr();
    lliua_backend.module.print_to_file("uiua-test.ll").unwrap();

    lliua_backend
        .module
        .verify()
        .expect("Module verification failed");

    let output = unsafe {
        lliua_backend
            .module
            .create_execution_engine()
            .unwrap()
            .run_function(module.get_first_function().unwrap(), &[])
            .as_float(&ctx.f64_type())
    };

    println!("Uiua computed: {output}")
}

impl<'ctx> LliuaBackend<'_, 'ctx> {
    fn entrypoint(&self, root_node: uiua::Node) -> inkwell::values::FunctionValue<'ctx> {
        let mut stack = crate::stack::Stack::create(&self.module);

        let main_fn =
            self.module
                .add_function("uiua_main", self.types.real.fn_type(&[], false), None);

        let entry = self.ctx.append_basic_block(main_fn, "entry");
        let builder = self.ctx.create_builder();
        builder.position_at_end(entry);

        for node in root_node {
            match node {
                uiua::Node::Push(value, _) => self.op_push(&mut stack, value),
                uiua::Node::Prim(uiua::Primitive::Rand, _) => self.op_random(&mut stack, &builder),
                uiua::Node::Prim(uiua::Primitive::Mul, _) => self.op_mul(&mut stack, &builder),
                uiua::Node::Prim(uiua::Primitive::Sub, _) => self.op_sub(&mut stack, &builder),
                _ => todo!("Node not implemented: {node:?}"),
            }
        }

        builder
            .build_return(Some(&stack.vstack.pop().unwrap()))
            .unwrap();

        return main_fn;
    }

    fn op_random(
        &self,
        stack: &mut crate::stack::Stack<'ctx>,
        b: &inkwell::builder::Builder<'ctx>,
    ) {
        let rand_float = b
            .build_load(self.types.real, self.rand_global.as_pointer_value(), "")
            .unwrap()
            .into_float_value();

        stack.vstack.push(rand_float);
    }

    fn op_push(&self, stack: &mut crate::stack::Stack<'ctx>, value: uiua::Value) {
        stack
            .vstack
            .push(self.ctx.f64_type().const_float(value_to_float(value)));
    }

    fn op_mul(&self, stack: &mut crate::stack::Stack<'ctx>, b: &inkwell::builder::Builder<'ctx>) {
        let o1 = stack.vstack.pop().expect("Empty stack");
        let o2 = stack.vstack.pop().expect("Empty stack");

        stack
            .vstack
            .push(b.build_float_mul(o2, o1, "").expect("Cant mul"));
    }

    fn op_sub(&self, stack: &mut crate::stack::Stack<'ctx>, b: &inkwell::builder::Builder<'ctx>) {
        let o1 = stack.vstack.pop().expect("Empty stack");
        let o2 = stack.vstack.pop().expect("Empty stack");

        stack
            .vstack
            .push(b.build_float_sub(o2, o1, "").expect("Cant sub"));
    }
}

fn value_to_float(value: uiua::Value) -> f64 {
    match value {
        uiua::Value::Byte(array) => array.convert(),
        uiua::Value::Num(array) => array,
        _ => todo!("Only real numbers supported for now"),
    }
    .into_scalar()
    .expect("Only scalars supported for now")
}
