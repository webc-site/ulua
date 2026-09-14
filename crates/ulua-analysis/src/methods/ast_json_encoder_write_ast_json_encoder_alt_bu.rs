use crate::macros::prop::PROP;
use crate::records::ast_json_encoder::AstJsonEncoder;
use ulua_ast::records::ast_type_reference::AstTypeReference;

impl AstJsonEncoder {
    pub fn write_ast_type_reference(&mut self, node: *mut AstTypeReference) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(
            node as *mut crate::records::ast_node::AstNode,
            "AstTypeReference",
            |e| {
                if n.prefix.is_some() {
                    PROP!(e, prefix);
                }
                if let Some(ref loc) = n.prefix_location {
                    e.write("prefixLocation", loc);
                }
                PROP!(e, name);
                PROP!(e, name_location);
                PROP!(e, parameters);
            },
        );
    }
}
