use std::cell::RefCell;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::execution_engine::JitFunction;
use super::super::program_object::CodeGen;
use super::super::exprs::expr_object;

pub fn execute_expr(text: &str) -> Option<u64> {
    type TestFunc = unsafe extern "C" fn(u64) -> u64;

    let context = &Context::create();
    let gen = RefCell::new(CodeGen::new(context, "test"));

    let i64_type = gen.borrow().context.i64_type();
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);

    let sum = gen.borrow().module.add_function("calc", fn_type, None);
    let basic_block = gen.borrow().context.append_basic_block(sum, "entry");
    gen.borrow().builder.position_at_end(basic_block);

    match expr_object::expr_parser(text) {
        Ok((_, val1)) => {
            let generated = val1.codegen(&gen);
            match generated {
                Ok(val2) => {
                    gen.borrow().builder.build_return(Some(&val2));

                    let execution_engine = gen.borrow().module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

                    let func: JitFunction<TestFunc> = unsafe { execution_engine.get_function("calc").unwrap() };
                    let ret: u64 = unsafe { func.call(0) };
                    Some(ret)
                },
                Err(_) => None
            }
        },
        Err(_) => None
    }
}
