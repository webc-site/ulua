//! Source: `Analysis/src/AstJsonEncoder.cpp:891-905` (hand-ported)
use crate::macros::prop::PROP;
use crate::records::ast_json_encoder::AstJsonEncoder;
use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;

impl AstJsonEncoder {
    pub fn write_ast_stat_type_alias(&mut self, node: *mut AstStatTypeAlias) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut ulua_ast::records::ast_node::AstNode, "AstStatTypeAlias", |e| {
            PROP(name);
            PROP(generics);
            PROP(generic_packs);
            e.write("value", &n.type_ptr);
            PROP(exported);
        });
    }
}
