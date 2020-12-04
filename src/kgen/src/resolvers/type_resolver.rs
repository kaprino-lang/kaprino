use inkwell::types::BasicTypeEnum;
use super::dictionary::Dictionary;
use super::super::program_object::CodeGen;

#[derive(Debug, PartialEq)]
pub struct KMember<'ctx> {
    pub name: &'ctx str,
    pub type_name: &'ctx str
}

#[derive(Debug, PartialEq)]
pub struct KType<'ctx> {
    pub type_name: &'ctx str,
    pub members: Vec<KMember<'ctx>>
}

impl<'ctx> KMember<'ctx> {
    pub fn new(name: &'ctx str, type_name: &'ctx str) -> KMember<'ctx> {
        KMember { name, type_name }
    }

    pub fn get_type(&self, gen: &'ctx CodeGen) -> &KType<'ctx> {
        gen.type_resolver.find(self.name).unwrap()
    }
}

impl<'ctx> KType<'ctx> {
    pub fn get_member(&self, name: &'ctx str) -> Option<&KMember<'ctx>> {
        self.members
            .iter()
            .find(|member| member.name == name)
    }

    pub fn get_type(&'ctx self, gen: &'ctx CodeGen) -> BasicTypeEnum {
        match self.type_name {
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
            _ => {
                BasicTypeEnum::IntType(
                    gen.context.i32_type()
                )
            }
        }
    }
}

pub type TypeResolver<'ctx> = Dictionary<'ctx, KType<'ctx>>;

impl<'ctx> TypeResolver<'_> {
    pub fn init_default_types(&mut self) {
        self.add(
            "Z",
            KType {
                type_name: "Z",
                members: Vec::new()
            }
        );

        self.add(
            "R",
            KType {
                type_name: "R",
                members: Vec::new()
            }
        );
    }
}
