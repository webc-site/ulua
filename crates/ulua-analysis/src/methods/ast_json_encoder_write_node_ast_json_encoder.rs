use ulua_ast::records::location::Location;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  // C++ template writeNode(AstNode*, string_view, F&& f). The [&] lambda
  // becomes FnOnce(&mut Self) so the body can keep writing through the
  // encoder while the wrapper holds the node frame open.
  pub(crate) fn write_node_ast_node_string_view_f<F: FnOnce(&mut Self)>(
    &mut self,
    location: &Location,
    name: &str,
    f: F,
  ) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view(name);
    self.write("location", location);
    f(self);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}
