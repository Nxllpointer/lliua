const TEST_SOURCE: &str = include_str!("test_source.ua");

fn main() {
    let mut compiler = uiua::Compiler::with_backend(uiua::SafeSys::new());
    compiler.load_str(TEST_SOURCE).expect("Failed to compile assembly!!!!??!");

    let uasm = compiler.finish();

    dbg!(&uasm.root);

    lliua::transform(uasm);
}
