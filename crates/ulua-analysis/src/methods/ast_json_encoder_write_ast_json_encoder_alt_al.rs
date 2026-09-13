//! Source: `Analysis/src/AstJsonEncoder.cpp:484-494` (hand-ported)
use ulua_ast::records::ast_generic_type::AstGenericType;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `generic_type` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_generic_type(&mut self, generic_type: *mut AstGenericType) {
    let g = unsafe { &*generic_type };
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view("AstGenericType");
    self.write("name", &g.name);
    if !g.default_value.is_null() {
      self.write("luauType", &g.default_value);
    }
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}
