#[cfg(test)]
pub fn jit_expr(source: &'static str) -> u64 {
    use inkwell::context::Context;
    use inkwell::execution_engine::JitFunction;
    use super::super::expr_object;
    use super::super::super::program_object::CodeGen;

    type TestFunc = unsafe extern "C" fn(u64) -> u64;

    let context = &Context::create();
    let gen = CodeGen::new(context, "test");

    let i64_type = gen.context.i64_type();
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);

    let sum = gen.module.add_function("calc", fn_type, None);
    let basic_block = gen.context.append_basic_block(sum, "entry");
    gen.builder.position_at_end(basic_block);

    match expr_object::expr_parser(source) {
        Ok((_, val1)) => {
            let generated = val1.codegen(&gen);
            match generated {
                Ok(val2) => {
                    use inkwell::OptimizationLevel;

                    gen.builder.build_return(Some(&val2));

                    let execution_engine = gen.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

                    let func: JitFunction<TestFunc> = unsafe { execution_engine.get_function("calc").unwrap() };
                    let ret: u64 = unsafe { func.call(0) };
                    ret
                },
                Err(text) => {
                    panic!("{}", text);
                }
            }
        },
        Err(text) => {
            panic!("{}", text);
        }
    }
}
