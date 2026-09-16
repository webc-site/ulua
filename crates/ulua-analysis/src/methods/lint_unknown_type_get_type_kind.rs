use alloc::string::String;

use crate::{enums::type_kind::TypeKind, records::lint_unknown_type::LintUnknownType};

impl LintUnknownType {
  pub fn get_type_kind(&mut self, name: &str) -> TypeKind {
    match name {
      "nil" | "boolean" | "userdata" | "number" | "string" | "table" | "function" | "thread"
      | "buffer" | "vector" => TypeKind::Primitive,
      _ => {
        let context = unsafe { &*self.context };
        if context.scope.lookup_type(&String::from(name)).is_some() {
          TypeKind::Userdata
        } else {
          TypeKind::Unknown
        }
      }
    }
  }
}
