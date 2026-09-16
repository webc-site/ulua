use crate::records::ast_json_encoder::AstJsonEncoder;
use crate::records::ast_node::AstNode;
use crate::macros::prop::PROP;
use ulua_ast::records::ast_expr::AstExpr;
use ulua_ast::records::ast_expr_function::AstExprFunction;
use ulua_ast::records::ast_stat_function::AstStatFunction;

impl AstJsonEncoder {
    pub fn write_ast_stat_function(&mut self, node: *mut AstStatFunction) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatFunction", |e| {
            e.write("name", &n.name);
            e.write_ast_expr_function(n.func);
        });
    }
}
