use inkwell::values::BasicValueEnum;
use nom::IResult;
use nom::sequence::tuple;
use nom::combinator::opt;
use nom::combinator::map;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::bytes::complete::tag;
use super::super::exprs::EvaluableObject;
use super::super::exprs::expr_object::expr_parser;
use super::super::program_object::CodeGen;
use super::super::resolvers::parameter_resolver::KParameter;
use super::StatementObject;

#[derive(Debug,PartialEq)]
pub struct LetObject {
    param_name: String,
    type_name: String,
    expr: Option<EvaluableObject>
}

impl<'ctx> LetObject {
    pub fn new(param_name: String, type_name: String, expr: Option<EvaluableObject>) -> LetObject {
        LetObject {
            param_name,
            type_name,
            expr
        }
    }

    fn allocate_param(&self, gen: &CodeGen<'ctx>) -> Result<KParameter<'ctx>, String> {
        let type_value = gen.type_resolver.borrow()
            .find(&self.type_name)
            .ok_or(format!("Unknown types: {}", self.type_name))?
            .get_type(gen);

        let allocated = gen.builder.build_alloca(
            type_value,
            &self.param_name
        );

        Ok(KParameter {
            type_id: self.type_name.clone(),
            value: BasicValueEnum::PointerValue(allocated)
        })
    }

    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), String> {
        let param = self.allocate_param(gen)?;

        let mut param_mut = gen.param_resolver.borrow_mut();
        param_mut.add(&self.param_name, param);

        if let Some(expr) = &self.expr {
            let param = param_mut.find(&self.param_name)
                .ok_or(format!("Unknown parameter named {}", self.param_name))?;

            match param.value {
                BasicValueEnum::PointerValue(val) => {
                    gen.builder.build_store(val, expr.codegen(gen).unwrap());
                    Ok(())
                },
                _ => {
                    Err("Cannot assign some value to immutable one".to_string())
                }
            }
        }
        else {
            Ok(())
        }
    }
}

pub fn let_parser(text: &str) -> IResult<&str, StatementObject> {
    let assign_parser = map(tuple((
            space0,
            tag(":="),
            space0,
            expr_parser
        )),
        |val| {
            let (_, _, _, expr) = val;
            expr
        }
    );

    let type_parser = map(
        tuple((
            space0,
            tag("("),
            space0,
            tag("<-"),
            space0,
            alphanumeric1,
            space0,
            tag(")"),
            space0
        )),
        |val| {
            let (_, _, _, _, _, type_name, _, _, _) = val;
            type_name
        }
    );

    map(
        tuple((
            tag("#let"),
            space1,
            alphanumeric1,
            opt(
                assign_parser
            ),
            type_parser
        )),
        |val| {
            let (_, _, param_name, assign, type_name) = val;
            StatementObject::LetObject(
                Box::new(
                    LetObject::new(
                        param_name.to_string(),
                        type_name.to_string(),
                        assign
                    )
                )
            )
        }
    )(text)
}
