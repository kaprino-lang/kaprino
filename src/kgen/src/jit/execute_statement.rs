use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::execution_engine::JitFunction;
use crate::ast::CodeGen;
use crate::parsers::statements::statement_parser;
use crate::parsers::Span;

///
/// Execute a statement Just In Time.
///
pub fn execute_statement(text: &str) -> Result<u32, ()> {
    type TestFunc = unsafe extern "C" fn() -> u32;

    let context = &Context::create();
    let gen = CodeGen::new(context, "test");

    let i32_type = gen.context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);

    let sum = gen.module.add_function("calc", fn_type, None);
    let basic_block = gen.context.append_basic_block(sum, "entry");

    gen.builder.position_at_end(basic_block);

    let text = Span::new(text);
    let (_, val) = statement_parser(text).or(Err(()))?;

    val.codegen(&gen).unwrap();

    let execution_engine = gen.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

    let func: JitFunction<TestFunc> = unsafe { execution_engine.get_function("calc").unwrap() };
    let ret: u32 = unsafe { func.call() };

    Ok(ret)
}
