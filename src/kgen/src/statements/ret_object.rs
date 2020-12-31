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

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), String> {
        let ret_val = self.expr.codegen(gen)?;

        gen.builder.build_return(Some(&ret_val));

        Ok(())
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
