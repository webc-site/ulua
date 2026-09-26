//! Source: `Analysis/src/AstJsonEncoder.cpp:1012-1018` (hand-ported)
use core::ptr::NonNull;

use ulua_ast::records::ast_type_or_pack::AstTypeOrPack;

use crate::{
  methods::ast_json_encoder_write_primitives::WriteJson, records::ast_json_encoder::AstJsonEncoder,
};

impl AstJsonEncoder {
  pub fn write_ast_type_or_pack(&mut self, node: &AstTypeOrPack) {
    // cpp 读 `node.type ? write(node.type) : write(node.typePack)`；null 指针桥接按
    // `ast_node_visit` 契约对 null 无操作，故 Error 形态（cpp 双侧皆 null）同样不输出。
    // 非空引用经 `as_ptr` 还原为同一地址，交回按动态类型分派的裸指针桥接，编码逐位不变。
    match *node {
      AstTypeOrPack::Type(ty) => NonNull::from(ty).as_ptr().write_json(self),
      AstTypeOrPack::Pack(pack) => NonNull::from(pack).as_ptr().write_json(self),
      AstTypeOrPack::Error => {}
    }
  }
}
