pub type RealType<'ctx> = tw::F64Type<'ctx>;

pub struct Types<'ctx> {
    real: RealType<'ctx>,
}

impl<'ctx> Types<'ctx> {
    pub fn new(ctx: &'ctx tw::Context) -> Self {
        Self {
            real: ctx.f64_type(),
        }
    }
}
