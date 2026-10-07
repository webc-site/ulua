use ulua_ast::records::location::Location;
use ulua_bytecode::records::string_ref::StringRef;

use crate::enums::kind::Kind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LValue {
  pub(crate) kind: Kind,
  pub(crate) reg: u8,
  pub(crate) upval: u8,
  pub(crate) index: u8,
  pub(crate) number: u8,
  /// cpp `LValue::name`（`sealed_ast::Array<const char>`）：指向 AST 名表的视图；
  /// 名表覆盖整个编译过程，故取 `'static`（与 `Compiler::bytecode` 的 arena 约定一致）。
  pub(crate) name: StringRef<'static>,
  pub(crate) location: Location,
}

impl Default for LValue {
  fn default() -> Self {
    Self {
      kind: Kind::Local,
      reg: 0,
      upval: 0,
      index: 0,
      number: 0,
      name: StringRef::default(),
      location: Location::default(),
    }
  }
}
