use crate::records::ast_json_encoder::AstJsonEncoder;
use ulua_ast::records::ast_node::AstNode;
use ulua_ast::records::ast_stat_declare_function::AstStatDeclareFunction;
use crate::macros::prop::PROP;

impl AstJsonEncoder {
    pub fn write_ast_stat_declare_function(&mut self, node: *mut AstStatDeclareFunction) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatDeclareFunction", |e| {
            PROP!(e, n, attributes);
            PROP!(e, n, name);
            PROP!(e, n, name_location);
            PROP!(e, n, params);
            PROP!(e, n, param_names);
            PROP!(e, n, vararg);
            PROP!(e, n, vararg_location);
            PROP!(e, n, ret_types);
            PROP!(e, n, generics);
            PROP!(e, n, generic_packs);
        });
    }
}
