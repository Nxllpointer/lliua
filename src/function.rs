crate::rt_struct!(CallArgs(ctx, lliua: &crate::LliuaBackend<'ctx>, sig: &uiua::Signature) {
    main_args: ArrayType = lliua.types.rt_value_ptr.array_type(sig.args() as u32),
    under_args: ArrayType = lliua.types.rt_value_ptr.array_type(sig.under_args() as u32)
});

crate::rt_struct!(CallOutputs(ctx, lliua: &crate::LliuaBackend<'ctx>, sig: &uiua::Signature) {
    main_outputs: ArrayType = lliua.types.rt_value_ptr.array_type(sig.outputs() as u32),
    under_outputs: ArrayType = lliua.types.rt_value_ptr.array_type(sig.under_outputs() as u32)
});

fn iter_array<'ctx>(
    ctx: &'ctx inkwell::context::Context,
    builder: &inkwell::builder::Builder<'ctx>,
    array_type: inkwell::types::ArrayType<'ctx>,
    array_ptr: inkwell::values::PointerValue<'ctx>,
    mut block: impl FnMut(inkwell::values::PointerValue<'ctx>, u32),
) {
    for i in 0..array_type.len() {
        unsafe {
            let index_value = ctx.i64_type().const_int(i as u64, false);

            let elem_ptr = builder
                .build_in_bounds_gep(array_type, array_ptr, &[index_value], "")
                .expect("Cannot get array element");

            block(elem_ptr, i);
        }
    }
}

fn load_stack<'ctx>(
    ctx: &'ctx inkwell::context::Context,
    builder: &inkwell::builder::Builder<'ctx>,
    array_type: inkwell::types::ArrayType<'ctx>,
    array_ptr: inkwell::values::PointerValue<'ctx>,
) -> Vec<crate::LliuaValue<'ctx>> {
    let mut values = Vec::with_capacity(array_type.len() as usize);

    iter_array(ctx, builder, array_type, array_ptr, |elem, _| {
        values.push(crate::LliuaValue { rt_value: elem });
    });

    values
}

fn store_stack<'ctx>(
    ctx: &'ctx inkwell::context::Context,
    builder: &inkwell::builder::Builder<'ctx>,
    array_type: inkwell::types::ArrayType<'ctx>,
    array_ptr: inkwell::values::PointerValue<'ctx>,
    stack: Vec<crate::LliuaValue<'ctx>>,
) {
    iter_array(ctx, builder, array_type, array_ptr, |elem, i| {
        builder.build_store(elem, stack[i as usize].rt_value).expect("Cannot store stack element");
    });
}

impl<'ctx> crate::LliuaBackend<'ctx> {
    pub fn build_function_call(
        &self,
        builder: &inkwell::builder::Builder<'ctx>,
        stacks: &mut crate::Stacks<'ctx>,
        name: &str,
        sig: &uiua::Signature,
        body: impl Fn(&inkwell::builder::Builder<'ctx>, &mut crate::Stacks<'ctx>) -> uiua::UiuaResult,
    ) -> uiua::UiuaResult<inkwell::values::FunctionValue<'ctx>> {
        let id = format!(
            "fn_{}_a{}_o{}_ua{}_uo{}",
            name,
            sig.args(),
            sig.outputs(),
            sig.under_args(),
            sig.under_outputs()
        );

        let call_args = CallArgs::new(self.ctx, self, sig, &format!("CallArgs_{id}"));
        let call_outputs = CallOutputs::new(self.ctx, self, sig, &format!("CallOutputs_{id}"));

        let fn_type = call_outputs.struct_type.fn_type(
            &[self.ctx.ptr_type(inkwell::AddressSpace::default()).into()],
            false,
        );

        let function = match self.module.get_function(&id) {
            Some(function) => function,
            None => {
                let function = self.module.add_function(&id, fn_type, None);

                let body_args_ptr = function
                    .get_nth_param(0)
                    .expect("Cannot get args")
                    .into_pointer_value();
                let body_outputs_ptr = function
                    .get_nth_param(1)
                    .expect("Cannot get outputs")
                    .into_pointer_value();

                let body_builder = self.ctx.create_builder();
                let body_entry = self.ctx.append_basic_block(function, "entry");
                body_builder.position_at_end(body_entry);

                let mut body_stacks = crate::Stacks {
                    main: load_stack(
                        self.ctx,
                        builder,
                        call_args.main_args,
                        call_args
                            .build_gep_main_args(builder, body_args_ptr, &[], "")
                            .expect("Cannot get main args"),
                    ),
                    under: load_stack(
                        self.ctx,
                        builder,
                        call_args.under_args,
                        call_args
                            .build_gep_under_args(builder, body_args_ptr, &[], "")
                            .expect("Cannot get under args"),
                    ),
                };

                body(&body_builder, &mut body_stacks)?;

                store_stack(
                    self.ctx,
                    builder,
                    call_outputs.main_outputs,
                    call_outputs
                        .build_gep_main_outputs(builder, body_outputs_ptr, &[], "")
                        .expect("Cannot get main outputs"),
                    body_stacks.main,
                );

                store_stack(
                    self.ctx,
                    builder,
                    call_outputs.under_outputs,
                    call_outputs
                        .build_gep_under_outputs(builder, body_outputs_ptr, &[], "")
                        .expect("Cannot get under outputs"),
                    body_stacks.under,
                );

                function
            }
        };

        let args_ptr = builder
            .build_alloca(call_args.struct_type, "")
            .expect("Cannot alloc call args");

        let outputs_ptr = builder
            .build_alloca(call_outputs.struct_type, "")
            .expect("Cannot alloc call outputs");

        store_stack(
            self.ctx,
            builder,
            call_args.main_args,
            call_args
                .build_gep_main_args(builder, args_ptr, &[], "")
                .expect("Cannot get main args"),
            stacks.main.split_off(stacks.main.len() - sig.args()),
        );

        store_stack(
            self.ctx,
            builder,
            call_args.under_args,
            call_args
                .build_gep_under_args(builder, args_ptr, &[], "")
                .expect("Cannot get under args"),
            stacks
                .under
                .split_off(stacks.under.len() - sig.under_args()),
        );

        let _ = builder
            .build_call(function, &[args_ptr.into()], &name)
            .expect("Cannot call function");

        stacks.main.append(&mut load_stack(
            self.ctx,
            builder,
            call_outputs.main_outputs,
            call_outputs.build_gep_main_outputs(builder, outputs_ptr, &[], "").expect("Cannot get main outputs"),
        ));

        stacks.under.append(&mut load_stack(
            self.ctx,
            builder,
            call_outputs.under_outputs,
            call_outputs.build_gep_under_outputs(builder, outputs_ptr, &[], "").expect("Cannot get under outputs"),
        ));

        Ok(function)
    }
}
