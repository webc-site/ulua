impl AstJsonEncoder {
    pub fn write_ast_type_or_pack(&mut self, node: &AstTypeOrPack) {
        if !node.r#type.is_null() {
            self.visit_ast_type_group(node.r#type as *mut crate::records::ast_type::AstType);
        } else {
            self.visit_ast_type_pack(node.type_pack as *mut crate::records::ast_type_pack::AstTypePack);
        }
    }
}
