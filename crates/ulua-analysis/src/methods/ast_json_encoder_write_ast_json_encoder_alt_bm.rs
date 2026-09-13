use crate::macros::prop::PROP;
use crate::records::ast_json_encoder::AstJsonEncoder;
use ulua_ast::records::ast_node::AstNode;
use ulua_ast::records::ast_stat_local_function::AstStatLocalFunction;

impl AstJsonEncoder {
    pub fn write_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) {
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatLocalFunction", |e| {
            PROP!(e, name);
            PROP!(e, func);
        });
    }
}
