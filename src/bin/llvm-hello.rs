// https://github.com/MWGuy/llvm-hello/blob/master/main.cpp

fn main() {
    let context = inkwell::context::Context::create();
    let module = context.create_module("hello");
    let builder = context.create_builder();

    let puts_fn = module.add_function(
        "puts",
        context
            .i32_type()
            .fn_type(&[context.ptr_type(0.into()).into()], false),
        None,
    );

    let main_fn = module.add_function("main", context.i32_type().fn_type(&[], false), None);

    let entry = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(entry);

    let hello_world = builder
        .build_global_string_ptr("Hello world!", "hello")
        .expect("Unable to build string");

    builder
        .build_call(puts_fn, &[hello_world.as_pointer_value().into()], "")
        .expect("Call");

    builder
        .build_return(Some(&context.i32_type().const_zero()))
        .expect("Return");

    module.verify().unwrap();

    module.print_to_file("hello.ll").unwrap();

    inkwell::targets::Target::initialize_x86(&inkwell::targets::InitializationConfig::default());

    unsafe {
        module
            .create_jit_execution_engine(inkwell::OptimizationLevel::None)
            .unwrap()
            .run_function_as_main(main_fn, &[]);
    }
}
