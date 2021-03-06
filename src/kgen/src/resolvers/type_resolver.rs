use inkwell::types::BasicTypeEnum;
use inkwell::AddressSpace;
use crate::ast::CodeGen;
use crate::resolvers::dictionary::Dictionary;

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
}

impl<'ctx> KType {
    pub fn get_member(&self, name: String) -> Option<&KMember> {
        self.members
            .iter()
            .find(|member| member.name == name)
    }

    pub fn get_type(&self, gen: &CodeGen<'ctx>) -> BasicTypeEnum<'ctx> {
        match self.type_name.as_str() {
            "Z" => {
                BasicTypeEnum::IntType(
                    gen.context.i32_type()
                )
            },
            "R" => {
                BasicTypeEnum::FloatType(
                    gen.context.f64_type()
                )
            },
            "Text" => {
                BasicTypeEnum::PointerType(
                    gen.context.i8_type().ptr_type(AddressSpace::Generic)
                )
            },
            _ => {
                BasicTypeEnum::IntType(
                    gen.context.i32_type()
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

        self.add(
            "Text",
            KType {
                type_name: "Text".to_string(),
                members: Vec::new()
            }
        );
    }
}
