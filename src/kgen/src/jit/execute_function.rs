use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::execution_engine::JitFunction;
use crate::ast::CodeGen;
use crate::parsers::functions::function_parser;
use crate::parsers::Span;

///
/// Execute a function Just In Time.
///
pub fn execute_function(text: &str, func_name: &str, arg: u32) -> Result<u32, ()> {
    type TestFunc = unsafe extern "C" fn(u32) -> u32;

    let context = &Context::create();
    let gen = CodeGen::new(context, "test");

    let text = Span::new(text);
    let (_, func) = function_parser(text).or(Err(()))?;

    func.codegen(&gen).unwrap();

    let execution_engine = gen.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

    let func: JitFunction<TestFunc> = unsafe { execution_engine.get_function(func_name).unwrap() };
    let ret = unsafe { func.call(arg) };

    Ok(ret)
}
