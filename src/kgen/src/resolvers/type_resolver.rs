use std::cell::RefCell;
use inkwell::types::BasicTypeEnum;
use super::dictionary::Dictionary;
use super::super::program_object::CodeGen;

#[derive(Debug, PartialEq)]
pub struct KMember {
    pub name: String,
    pub type_name: String
}

#[derive(Debug, PartialEq)]
pub struct KType {
    pub type_name: String,
    pub members: Vec<KMember>
}

impl<'ctx> KMember {
    pub fn new(name: String, type_name: String) -> KMember {
        KMember { name, type_name }
    }

    pub fn get_type(&self, gen: &'ctx CodeGen) -> &'ctx KType {
        gen.type_resolver.find(&self.name).unwrap()
    }
}

impl<'ctx> KType {
    pub fn get_member(&self, name: String) -> Option<&KMember> {
        self.members
            .iter()
            .find(|member| member.name == name)
    }

    pub fn get_type(&self, gen: &'ctx RefCell<CodeGen>) -> BasicTypeEnum<'ctx> {
        match self.type_name.as_str() {
            "Z" => {
                BasicTypeEnum::IntType(
                    gen.borrow().context.i32_type()
                )
            },
            "R" => {
                BasicTypeEnum::FloatType(
                    gen.borrow().context.f64_type()
                )
            },
            _ => {
                BasicTypeEnum::IntType(
                    gen.borrow().context.i32_type()
                )
            }
        }
    }
}

pub type TypeResolver<'ctx> = Dictionary<KType>;

impl<'ctx> TypeResolver<'_> {
    pub fn init_default_types(&mut self) {
        self.add(
            "Z",
            KType {
                type_name: "Z".to_string(),
                members: Vec::new()
            }
        );

        self.add(
            "R",
            KType {
                type_name: "R".to_string(),
                members: Vec::new()
            }
        );
    }
}
