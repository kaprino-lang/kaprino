use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::execution_engine::JitFunction;
use super::super::program_object::CodeGen;
use super::super::functions::function_object::function_parser;

pub fn execute_function(text: &str, func_name: &str, arg: u32) -> Option<u32> {
    type TestFunc = unsafe extern "C" fn(u32) -> u32;

    let context = &Context::create();
    let gen = CodeGen::new(context, "test");
    gen.init();

    match function_parser(text) {
        Ok((_, func)) => {
            func.codegen(&gen).unwrap();

            let execution_engine = gen.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

            let func: JitFunction<TestFunc> = unsafe { execution_engine.get_function(func_name).unwrap() };
            let ret: u32 = unsafe { func.call(arg) };

            Some(ret)
        },
        Err(_) => None
    }
}
