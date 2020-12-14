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

    fn allocate_param(&self, gen: &CodeGen<'ctx>) -> KParameter<'ctx> {
        let type_inst = match gen.type_resolver.borrow().find(&self.type_name) {
            Some(val) => {
                Ok(val.get_type(gen))
            },
            None => {
                Err(format!("Unknown types: {}", self.type_name))
            }
        };

        let allocated = gen.builder.build_alloca(
            type_inst.unwrap(),
            &self.param_name
        );

        KParameter {
            type_id: self.type_name.clone(),
            value: BasicValueEnum::PointerValue(allocated)
        }
    }

    pub fn codegen(&self, gen: &CodeGen<'ctx>) {
        let param = self.allocate_param(gen);

        let mut param_mut = gen.param_resolver.borrow_mut();
        param_mut.add(&self.param_name, param);

        if let Some(expr) = &self.expr {
            let param = param_mut.find(&self.param_name).unwrap();

            match param.value {
                BasicValueEnum::PointerValue(val) => {
                    gen.builder.build_store(val, expr.codegen(gen).unwrap());
                },
                _ => {
                    panic!("Cannot assign some value to immutable one");
                }
            };
        };
    }
}

pub fn let_parser(text: &str) -> IResult<&str, StatementObject> {
    let assign_parser = map(tuple((
            tag(":="),
            space0,
            expr_parser
        )),
        |val| {
            let (_, _, expr) = val;
            expr
        }
    );

    let type_parser = map(
        tuple((
            tag("("),
            space0,
            tag("<-"),
            space0,
            alphanumeric1,
            space0,
            tag(")")
        )),
        |val| {
            let (_, _, _, _, type_name, _, _) = val;
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
