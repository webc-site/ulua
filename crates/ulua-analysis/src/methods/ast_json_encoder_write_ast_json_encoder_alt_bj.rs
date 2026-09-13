use crate::records::ast_json_encoder::AstJsonEncoder;
use ulua_ast::records::ast_stat_assign::AstStatAssign;

impl AstJsonEncoder {
    pub fn write_ast_stat_assign(&mut self, node: *mut AstStatAssign) {
        self.write_node_ast_node_string_view_f(node as *mut ulua_ast::records::ast_node::AstNode, "AstStatAssign", |e| {
            let n = unsafe { &*node };
            e.write("vars", &n.vars);
            e.write("values", &n.values);
        });
    }
}
