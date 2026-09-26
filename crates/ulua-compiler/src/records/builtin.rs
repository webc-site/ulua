use ulua_ast::records::ast_name::AstName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Builtin {
  /// cpp `Builtin::object`（Builtins.h:16）为公开成员，本端口对齐其数据形态：
  /// 纯识别键值对、无不变式需要字段级封装，tests/ 集成测试据以直接构表。
  pub object: AstName,
  /// cpp `Builtin::method`（Builtins.h:17）
  pub method: AstName,
}

impl Builtin {
  pub fn empty(&self) -> bool {
    self.object == AstName::default() && self.method == AstName::default()
  }

  /// 字节切片形参（不含 NUL），同 [`Builtin::is_method`]。
  pub fn is_global(&self, name: &[u8]) -> bool {
    if self.object != AstName::default() || self.method.is_null() {
      return false;
    }
    self.method.as_bytes() == name
  }

  /// 字节切片形参（不含 NUL）：与 `AstName` 的字节门面同族，宿主登记的 C ABI
  /// 名字串无需先经 UTF-8 解码即可参与 strcmp 语义比较（cpp `isMethod` 即字节比较）。
  pub fn is_method(&self, table: &[u8], name: &[u8]) -> bool {
    if self.object.is_null() || self.method.is_null() {
      return false;
    }
    self.object.as_bytes() == table && self.method.as_bytes() == name
  }
}
