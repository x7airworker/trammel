use lang_c::ast::*;

#[derive(Debug)]
pub struct TypeAndName {
    pub param_type: String,
    pub name: String,
}

#[derive(Debug)]
pub struct RustDeclaration {
    pub name: String,
    pub type_declaration: RustDeclarationType,
}

#[derive(Debug)]
pub enum RustDeclarationType {
    Function(Vec<TypeAndName>),
    Struct(Vec<TypeAndName>),
}

pub struct Visitor(pub Vec<RustDeclaration>);

impl Default for Visitor {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Visitor {
    pub fn visit_translation_unit(&mut self, unit: &TranslationUnit) {
        // Visit each external declaration in the unit
        for declaration in &unit.0 {
            self.visit_external_declaration(&declaration.node);
        }
    }

    fn visit_external_declaration(&mut self, declaration: &ExternalDeclaration) {
        use ExternalDeclaration::*;
        match declaration {
            Declaration(decl) => self.visit_declaration(&decl.node),
            x => {
                println!("{:#?}", x);
                unimplemented!();
            }
        }
    }

    fn visit_declaration(&mut self, decl: &Declaration) {
        for specifier in &decl.specifiers {
            self.visit_declaration_specifier(&specifier.node);
        }

        for init_declarator in &decl.declarators {
            self.visit_init_declarator(&init_declarator.node);
        }
    }

    fn visit_declaration_specifier(&mut self, specifier: &DeclarationSpecifier) {
        // Handle the declaration specifier (typically things like int, long, etc.)
        // This is typically differentiated by an enum or similar structure
        match specifier {
            DeclarationSpecifier::TypeSpecifier(t) => {
                self.visit_typedef(&t.node);
            }
            _ => {}
        }
    }

    fn visit_init_declarator(&mut self, init_declarator: &InitDeclarator) {
        self.visit_declarator(&init_declarator.declarator.node);
    }

    fn visit_declarator(&mut self, declarator: &Declarator) -> String {
        let name = self.visit_declarator_kind(&declarator.kind.node);
        if let Some(derived) = &declarator.derived.first() {
            if let Some(type_declaration) = self.visit_derived_declarator(&derived.node) {
                self.0.push(RustDeclaration {
                    name: name.clone(),
                    type_declaration,
                });
            }
        }
        name
    }

    fn visit_derived_declarator(
        &mut self,
        derived_declarator: &DerivedDeclarator,
    ) -> Option<RustDeclarationType> {
        match derived_declarator {
            DerivedDeclarator::Function(f) => {
                let params: Vec<TypeAndName> = f
                    .node
                    .parameters
                    .iter()
                    .filter_map(|p| self.visit_function_param(&p.node))
                    .collect();

                Some(RustDeclarationType::Function(params))
            }
            x => {
                println!("Not implemented: {:?}", x);
                None
            }
        }
    }

    fn visit_function_param(&mut self, param: &ParameterDeclaration) -> Option<TypeAndName> {
        match param.specifiers.first() {
            Some(specifier) => match &specifier.node {
                DeclarationSpecifier::TypeSpecifier(typedef) => {
                    if let Some(name_declarator) = &param.declarator {
                        return Some(TypeAndName {
                            param_type: self.visit_typedef(&typedef.node),
                            name: self.visit_declarator_kind(&name_declarator.node.kind.node),
                        });
                    }
                    None
                }
                x => {
                    println!("{:?}", x);
                    unimplemented!();
                }
            },
            None => None,
        }
    }

    fn visit_specifier_qualifier(&mut self, qualifier: &SpecifierQualifier) -> String {
        match qualifier {
            SpecifierQualifier::TypeSpecifier(s) => self.visit_typedef(&s.node),
            x => {
                println!("{:#?}", x);
                unimplemented!();
            }
        }
    }

    fn visit_struct_declaration(&mut self, decl: &StructDeclaration) -> TypeAndName {
        match &decl {
            StructDeclaration::Field(f) => {
                let field_type =
                    self.visit_specifier_qualifier(&f.node.specifiers.first().unwrap().node);
                let field_name = self.visit_declarator(
                    &f.node
                        .declarators
                        .first()
                        .unwrap()
                        .node
                        .declarator
                        .as_ref()
                        .unwrap()
                        .node,
                );

                TypeAndName {
                    param_type: field_type,
                    name: field_name,
                }
            }
            x => {
                println!("{:?}", x);
                unimplemented!();
            }
        }
    }

    fn visit_typedef(&mut self, typedef: &TypeSpecifier) -> String {
        match typedef {
            TypeSpecifier::TypedefName(n) => n.node.name.clone(),
            TypeSpecifier::Unsigned => String::from("libc::c_uint"),
            TypeSpecifier::Char => String::from("libc::c_char"),
            TypeSpecifier::Long => String::from("libc::c_long"),
            TypeSpecifier::Double => String::from("libc::c_double"),
            TypeSpecifier::Short => String::from("libc::c_short"),
            TypeSpecifier::Float => String::from("libc::c_float"),
            TypeSpecifier::Int => String::from("libc::c_int"),
            TypeSpecifier::Void => String::from("libc::c_void"),
            TypeSpecifier::Struct(n) => match &n.node.identifier {
                Some(i) => {
                    if let Some(decls) = &n.node.declarations {
                        let fields = decls
                            .iter()
                            .map(|s| self.visit_struct_declaration(&s.node))
                            .collect::<Vec<TypeAndName>>();

                        self.0.push(RustDeclaration {
                            name: i.node.name.clone(),
                            type_declaration: RustDeclarationType::Struct(fields),
                        })
                    }
                    i.node.name.clone()
                }
                None => panic!("UNKNOWN_STRUCT"),
            },
            x => {
                println!("{:?}", x);
                unimplemented!();
            }
        }
    }

    fn visit_declarator_kind(&mut self, kind: &DeclaratorKind) -> String {
        match kind {
            DeclaratorKind::Identifier(i) => i.node.name.clone(),
            DeclaratorKind::Declarator(d) => self.visit_declarator_kind(&d.node.kind.node),
            x => {
                println!("{:?}", x);
                unimplemented!();
            }
        }
    }
}
