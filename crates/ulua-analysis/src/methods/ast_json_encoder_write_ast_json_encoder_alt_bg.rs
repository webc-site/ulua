use crate::macros::prop::PROP;
use crate::records::ast_json_encoder::AstJsonEncoder;
use ulua_ast::records::ast_stat_local::AstStatLocal;

impl AstJsonEncoder {
    pub fn write_ast_stat_local(&mut self, node: *mut AstStatLocal) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut ulua_ast::records::ast_node::AstNode, "AstStatLocal", |e| {
            PROP!(e, "vars", n.vars);
            PROP!(e, "values", n.values);
        });
    }
}
