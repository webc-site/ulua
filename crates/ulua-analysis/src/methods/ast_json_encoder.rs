//! `ast_json_encoder` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::{
  enums::type_lexer::Type,
  records::{
    ast_stat_assign::AstStatAssign, ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
    ast_stat_type_alias::AstStatTypeAlias, ast_type_or_pack::AstTypeOrPack,
    ast_type_reference::AstTypeReference, comment::Comment, location::Location,
  },
};

use crate::{
  functions::write_json_emitter::write_string,
  macros::write_json_node,
  methods::{
    ast_json_encoder_write_primitives::WriteJson,
    json_emitter::{CHUNK_SIZE, append_chunk},
  },
  records::ast_json_encoder::AstJsonEncoder,
};

impl AstJsonEncoder {
  pub fn append_chunk(&mut self, sv: &str) {
    append_chunk(&mut self.chunks, sv);
  }
}

impl AstJsonEncoder {
  pub fn ast_json_encoder_ast_json_encoder() -> Self {
    let mut encoder = AstJsonEncoder {
      chunks: Vec::new(),
      comma: false,
    };
    encoder.new_chunk();
    encoder
  }
}

impl AstJsonEncoder {
  pub fn new_chunk(&mut self) {
    self.chunks.push(String::with_capacity(CHUNK_SIZE));
  }
}

impl AstJsonEncoder {
  pub fn pop_comma(&mut self, c: bool) {
    self.comma = c;
  }
}

impl AstJsonEncoder {
  pub fn push_comma(&mut self) -> bool {
    let c = self.comma;
    self.comma = false;
    c
  }
}

impl AstJsonEncoder {
  pub fn str(&mut self) -> String {
    self.chunks.join("")
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:838-849` (hand-ported)

write_json_node!(write_ast_stat_assign, AstStatAssign, "AstStatAssign", [
  "vars": vars,
  "values": values,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:851-863` (hand-ported)

write_json_node!(write_ast_stat_compound_assign, AstStatCompoundAssign, "AstStatCompoundAssign", [
  "op": op,
  "var": var,
  "value": value,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:955-969` (hand-ported)

impl AstJsonEncoder {
  pub fn write_ast_stat_declare_extern_type(&mut self, node: &AstStatDeclareExternType) {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstStatDeclareClass", |e| {
      e.write("name", &node.name);
      if let Some(super_name) = node.super_name {
        e.write("superName", &super_name);
      }
      e.write("props", &node.props);
      e.write("indexer", &node.indexer);
    });
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:907-926` (hand-ported)

write_json_node!(write_ast_stat_declare_function, AstStatDeclareFunction, "AstStatDeclareFunction", [
  "attributes": attributes,
  "name": name,
  "nameLocation": name_location,
  "params": params,
  "paramNames": param_names,
  "vararg": vararg,
  "varargLocation": vararg_location,
  "retTypes": ret_types,
  "generics": generics,
  "genericPacks": generic_packs,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:928-940` (hand-ported)

write_json_node!(write_ast_stat_declare_global, AstStatDeclareGlobal, "AstStatDeclareGlobal", [
  "name": name,
  "nameLocation": name_location,
  "type": type_,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:971-982` (hand-ported)

write_json_node!(write_ast_stat_error, AstStatError, "AstStatError", [
  "expressions": expressions,
  "statements": statements,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:804-820` (hand-ported)

impl AstJsonEncoder {
  /// 可缺省的 `step` 是裸指针槽位，仅以 `is_null()` 判空后作为指针下传桥接。
  pub fn write_ast_stat_for(&mut self, node: &AstStatFor) {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstStatFor", |e| {
      e.write("var", &node.var);
      e.write("from", &node.from);
      e.write("to", &node.to);
      if !node.step.is_null() {
        e.write("step", &node.step);
      }
      e.write("body", &node.body);
      e.write("hasDo", &node.has_do);
    });
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:822-836` (hand-ported)

write_json_node!(write_ast_stat_for_in, AstStatForIn, "AstStatForIn", [
  "vars": vars,
  "values": values,
  "body": body,
  "hasIn": has_in,
  "hasDo": has_do,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:865-876` (hand-ported)

write_json_node!(write_ast_stat_function, AstStatFunction, "AstStatFunction", [
  "name": name,
  "func": func,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:791-802` (hand-ported)

write_json_node!(write_ast_stat_local, AstStatLocal, "AstStatLocal", [
  "vars": vars,
  "values": values,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:878-889` (hand-ported)

write_json_node!(write_ast_stat_local_function, AstStatLocalFunction, "AstStatLocalFunction", [
  "name": name,
  "func": func,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:891-905` (hand-ported)

write_json_node!(write_ast_stat_type_alias, AstStatTypeAlias, "AstStatTypeAlias", [
  "name": name,
  "generics": generics,
  "genericPacks": generic_packs,
  "value": type_ptr,
  "exported": exported,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:1012-1018` (hand-ported)

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

// Source: `Analysis/src/AstJsonEncoder.cpp:992-1008` (hand-ported)

impl AstJsonEncoder {
  /// `prefix` 是可空指针槽位（`Option` 仅承载缺省语义），判空后原样下传桥接。
  pub fn write_ast_type_reference(&mut self, node: &AstTypeReference) {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstTypeReference", |e| {
      if node.prefix.is_some() {
        e.write("prefix", &node.prefix);
      }
      if let Some(prefix_location) = node.prefix_location {
        e.write("prefixLocation", &prefix_location);
      }
      e.write("name", &node.name);
      e.write("nameLocation", &node.name_location);
      e.write("parameters", &node.parameters);
    });
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:1519-1552, hand-ported)

impl AstJsonEncoder {
  pub fn write_comments(&mut self, comment_locations: Vec<Comment>) {
    let mut comment_comma = false;
    for comment in comment_locations {
      if comment_comma {
        self.write_raw_string_view(",");
      } else {
        comment_comma = true;
      }
      self.write_raw_string_view("{");
      let c = self.push_comma();
      match comment.r#type {
        Type::COMMENT => self.write_type_string_view("Comment"),
        Type::BLOCK_COMMENT => self.write_type_string_view("BlockComment"),
        Type::BROKEN_COMMENT => self.write_type_string_view("BrokenComment"),
        _ => {}
      }
      self.write("location", &comment.location);
      self.pop_comma(c);
      self.write_raw_string_view("}");
    }
  }
}

impl AstJsonEncoder {
  // C++ template writeNode(AstNode*, string_view, F&& f). The [&] lambda
  // becomes FnOnce(&mut Self) so the body can keep writing through the
  // encoder while the wrapper holds the node frame open.
  pub(crate) fn write_node_ast_node_string_view_f<F: FnOnce(&mut Self)>(
    &mut self,
    location: &Location,
    name: &str,
    f: F,
  ) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view(name);
    self.write("location", location);
    f(self);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}

impl AstJsonEncoder {
  // writeRaw(std::string_view) — pinned overload name
  pub fn write_raw_string_view(&mut self, sv: &str) {
    self.append_chunk(sv);
  }

  /// 单字节写入（cpp `writeRaw(char)`）：与 `JsonEmitter::write_raw_byte` 同形态，
  /// 去除照抄 C 字符的旧名/泛型 trait。ASCII 逐字节等价；非 ASCII 旧实现为 UB，
  /// 现按 Latin-1 码位的 UTF-8 编码落盘（行为有定义）。
  #[inline]
  pub fn write_raw_byte(&mut self, c: u8) {
    let mut buf = [0u8; 4];
    let s = (c as char).encode_utf8(&mut buf);
    self.write_raw_string_view(s);
  }
}

impl AstJsonEncoder {
  pub fn write_string(&mut self, sv: &str) {
    write_string(sv, |part| self.write_raw_string_view(part));
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:83-86, hand-ported)

impl AstJsonEncoder {
  // writeType(std::string_view propValue) — write("type", propValue) expansion
  pub fn write_type_string_view(&mut self, prop_value: &str) {
    if self.comma {
      self.write_raw_string_view(",");
    }
    self.comma = true;
    self.write_raw_string_view("\"type\":");
    // write(std::string_view) — JSON-escaped string write
    self.write_raw_string_view("\"");
    self.write_raw_string_view(prop_value);
    self.write_raw_string_view("\"");
  }
}
