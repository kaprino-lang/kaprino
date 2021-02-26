use inkwell::values::BasicValueEnum;
use crate::ast::CodeGen;
use crate::ast::exprs::EvaluableObject;
use crate::error::error_token::{ FilePosition, ErrorToken };
use crate::resolvers::parameter_resolver::KParameter;

///
/// `LetObject` is an object which represents a statement with the keyword `#let`.
///
#[derive(Debug,PartialEq)]
pub struct LetObject {
    pos: FilePosition,
    param_name: String,
    type_name: String,
    expr: Option<EvaluableObject>
}

impl<'ctx> LetObject {
    ///
    /// Create a `LetObject` instance.
    ///
    pub fn new(pos: FilePosition, param_name: String, type_name: String, expr: Option<EvaluableObject>) -> Self {
        Self {
            pos,
            param_name,
            type_name,
            expr
        }
    }

    ///
    /// Allocate memories for storing parameters.
    ///
    fn allocate_param(&self, gen: &CodeGen<'ctx>) -> Result<KParameter<'ctx>, ErrorToken> {
        let type_value = gen.type_resolver.borrow()
            .find(&self.type_name)
            .ok_or(
                ErrorToken::error(
                    self.pos.clone(),
                    format!("Not found a type named {}", self.type_name)
                )
            )?
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

    ///
    /// Generate the part of the abstract syntax tree.
    ///
    pub fn codegen(&self, gen: &CodeGen<'ctx>) -> Result<(), ErrorToken> {
        let param = self.allocate_param(gen)?;

        let mut param_mut = gen.param_resolver.borrow_mut();
        param_mut.add(&self.param_name, param);

        if let Some(expr) = &self.expr {
            let param = param_mut.find(&self.param_name)
                .ok_or(
                    ErrorToken::error(
                        self.pos.clone(),
                        format!("An unknown parameter named {} is referenced here", self.param_name)
                    )
                )?;

            match param.value {
                BasicValueEnum::PointerValue(val) => {
                    gen.builder.build_store(val, expr.codegen(gen)?);
                    Ok(())
                },
                _ => {
                    Err(ErrorToken::error(
                        self.pos.clone(),
                        "Cannot assign some value to immutable one".to_string()
                    ))
                }
            }
        }
        else {
            Ok(())
        }
    }
}
