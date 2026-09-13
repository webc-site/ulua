use crate::records::ast_json_encoder::AstJsonEncoder;
use crate::macros::prop::PROP;
use ulua_ast::records::ast_expr::AstExpr;
use ulua_ast::records::ast_local::AstLocal;
use ulua_ast::records::ast_stat_for::AstStatFor;
use ulua_ast::records::ast_stat_block::AstStatBlock;

impl AstJsonEncoder {
    pub fn write_ast_stat_for(&mut self, node: *mut AstStatFor) {
        self.write_node_ast_node_string_view_f(node as *mut ulua_ast::records::ast_node::AstNode, "AstStatFor", |e| {
            let n = unsafe { &*node };
            e.write("var", &n.var);
            e.write("from", &n.from);
            e.write("to", &n.to);
            if !n.step.is_null() {
                e.write("step", &n.step);
            }
            e.write("body", &n.body);
            e.write("hasDo", &n.has_do);
        });
    }
}
