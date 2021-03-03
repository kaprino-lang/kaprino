use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::execution_engine::JitFunction;
use crate::ast::CodeGen;
use crate::parsers::exprs::expr_parser;
use crate::parsers::Span;

///
/// Execute an expression Just In Time.
///
pub fn execute_expr(text: &str) -> Result<u32, ()> {
    type TestFunc = unsafe extern "C" fn() -> u32;

    let context = &Context::create();
    let gen = CodeGen::new(context, "test");

    let i32_type = gen.context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);

    let sum = gen.module.add_function("calc", fn_type, None);
    let basic_block = gen.context.append_basic_block(sum, "entry");
    gen.builder.position_at_end(basic_block);

    let text = Span::new(text);
    let (_, val) = expr_parser(text).or(Err(()))?;
    let val = val.codegen(&gen).or(Err(()))?;

    gen.builder.build_return(Some(&val));

    let execution_engine = gen.module.create_jit_execution_engine(OptimizationLevel::None).or(Err(()))?;

    let func: JitFunction<TestFunc> = unsafe { execution_engine.get_function("calc") }.or(Err(()))?;
    let ret: u32 = unsafe { func.call() };
    Ok(ret)
}
