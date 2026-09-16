use crate::macros::prop::PROP;
use crate::records::ast_json_encoder::AstJsonEncoder;
use ulua_ast::records::ast_node::AstNode;
use ulua_ast::records::ast_stat_for_in::AstStatForIn;

impl AstJsonEncoder {
    pub fn write_ast_stat_for_in(&mut self, node: *mut AstStatForIn) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatForIn", |e| {
            PROP!(e, vars, &n.vars);
            PROP!(e, values, &n.values);
            PROP!(e, body, &n.body);
            PROP!(e, hasIn, &n.has_in);
            PROP!(e, hasDo, &n.has_do);
        });
    }
}
