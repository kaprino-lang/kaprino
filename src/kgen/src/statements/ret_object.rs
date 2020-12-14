use inkwell::values::BasicValueEnum;
use nom::IResult;
use nom::sequence::tuple;
use nom::combinator::map;
use nom::character::complete::space1;
use nom::bytes::complete::tag;
use super::super::exprs::EvaluableObject;
use super::super::exprs::expr_object::expr_parser;
use super::super::program_object::CodeGen;
use super::StatementObject;

#[derive(Debug,PartialEq)]
pub struct RetObject {
    expr: EvaluableObject
}

impl<'ctx> RetObject {
    pub fn new(expr: EvaluableObject) -> Self {
        Self {
            expr
        }
    }

    fn get_ret_val(&self, gen: &CodeGen<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        match self.expr.codegen(gen) {
            Ok(val) => Some(val),
            Err(_) => None
        }
    }

    pub fn codegen(&self, gen: &CodeGen<'ctx>) {
        let ret_val = self.get_ret_val(gen);

        match ret_val {
            Some(val) => {
                gen.builder.build_return(Some(&val));
            },
            None => {
                gen.builder.build_return(None);
            }
        }
    }
}

pub fn ret_parser(text: &str) -> IResult<&str, StatementObject> {
    map(
        tuple((
            tag("#ret"),
            space1,
            expr_parser
        )),
        |val| {
            let (_, _, expr) = val;
            StatementObject::RetObject(
                Box::new(
                    RetObject::new(expr)
                )
            )
        }
    )(text)
}
