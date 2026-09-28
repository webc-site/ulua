use core::{
  ptr::{NonNull, null_mut},
  str::from_utf8,
};

use ulua_ast::{
  records::{
    ast_array::AstArray,
    ast_attr::AstAttr,
    ast_declared_extern_type_property::AstDeclaredExternTypeProperty,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal,
    ast_expr_table::{AstExprTable, Item, ItemKind},
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_expr_varargs::AstExprVarargs,
    ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack,
    ast_local::AstLocal,
    ast_node::AstNode,
    ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak,
    ast_stat_continue::AstStatContinue,
    ast_stat_expr::AstStatExpr,
    ast_stat_if::AstStatIf,
    ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn,
    ast_stat_while::AstStatWhile,
    ast_table_indexer::AstTableIndexer,
    ast_table_prop::AstTableProp,
    ast_type_error::AstTypeError,
    ast_type_function::AstTypeFunction,
    ast_type_intersection::AstTypeIntersection,
    ast_type_list::AstTypeList,
    ast_type_optional::AstTypeOptional,
    ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic,
    ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof,
    ast_type_union::AstTypeUnion,
    location::Location,
    position::Position,
  },
  type_aliases::ast_argument_name::AstArgumentName,
  visit::ast_node_visit,
};
use ulua_common::{functions::format_g::format_g, macros::luau_assert::LUAU_ASSERT};

use crate::{
  macros::{json_visit_delegator, write_json_node},
  methods::ast_json_encoder_write_primitives::WriteJson,
  records::ast_json_encoder::AstJsonEncoder,
};

// Source: `Analysis/src/AstJsonEncoder.cpp:341-351` (hand-ported)
// 安全化端口：C++ `write(AstExprLocal*)`。节点以共享引用进入，帧头经
// `base` 链取 `location`；`local` 字段槽（arena 裸指针）仅作只读转发，
// 由 `*mut AstLocal` 的 `WriteJson` 桥接负责句柄化。
write_json_node!(write_ast_expr_local, AstExprLocal, "AstExprLocal", [
  "local": local,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:353-363` (hand-ported)
write_json_node!(write_ast_expr_global, AstExprGlobal, "AstExprGlobal", [
  "global": name,
]);

write_json_node!(write_ast_expr_varargs, AstExprVarargs, "AstExprVarargs", []);

// Source: `Analysis/src/AstJsonEncoder.cpp:370-385` (hand-ported)
// C++ template write(AstArray<T>): "[" elem ("," elem)* "]". The AstArray<char>
// specialization lives in `write_ast_array_u8` (its own bridge impl), which
// is sound because u8 never implements WriteJson directly.
impl<T: WriteJson> WriteJson for AstArray<T> {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_raw_string_view("[");
    let mut comma = false;
    for a in self.iter() {
      if comma {
        enc.write_raw_string_view(",");
      } else {
        comma = true;
      }
      a.write_json(enc);
    }
    enc.write_raw_string_view("]");
  }
}

impl AstJsonEncoder {
  pub fn write_ast_array_u8(&mut self, arr: AstArray<u8>) {
    if let Ok(s) = arr.as_str() {
      self.write_string_view(s);
    }
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:394-407` (hand-ported)
write_json_node!(write_ast_expr_call, AstExprCall, "AstExprCall", [
  "func": func,
  "args": args,
  "self": self_,
  "argLocation": arg_location,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:409-422` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_expr_index_name(&mut self, node: &AstExprIndexName) {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstExprIndexName", |e| {
      e.write("expr", &node.expr);
      e.write("index", &node.index);
      e.write("indexLocation", &node.index_location);
      // write("op", node->op) -- C++ char overload: a one-char string
      if e.comma {
        e.write_raw_string_view(",");
      }
      e.comma = true;
      e.write_raw_string_view("\"op\":");
      e.write_u8(node.op);
    });
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:424-435` (hand-ported)
write_json_node!(write_ast_expr_index_expr, AstExprIndexExpr, "AstExprIndexExpr", [
  "expr": expr,
  "index": index,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:437-462` (hand-ported)
impl AstJsonEncoder {
  /// self_/return_annotation/vararg_annotation 三个可缺省字段是裸指针槽位，
  /// 仅经 `is_null()` 过滤后作为指针下传给桥接 write，从不在此解引用。
  pub fn write_ast_expr_function(&mut self, node: &AstExprFunction) {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstExprFunction", |e| {
      e.write("attributes", &node.attributes);
      e.write("generics", &node.generics);
      e.write("genericPacks", &node.generic_packs);
      if !node.self_.is_null() {
        e.write("self", &node.self_);
      }
      e.write("args", &node.args);
      if !node.return_annotation.is_null() {
        e.write("returnAnnotation", &node.return_annotation);
      }
      e.write("vararg", &node.vararg);
      e.write("varargLocation", &node.vararg_location);
      if !node.vararg_annotation.is_null() {
        e.write("varargAnnotation", &node.vararg_annotation);
      }
      e.write("body", &node.body);
      e.write("functionDepth", &node.function_depth);
      e.write("debugname", &node.debugname);
    });
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:472-482` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_type_list(&mut self, type_list: &AstTypeList) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view("AstTypeList");
    self.write("types", &type_list.types);
    if !type_list.tail_type.is_null() {
      self.write("tailType", &type_list.tail_type);
    }
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:484-494` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_generic_type(&mut self, generic_type: &AstGenericType) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view("AstGenericType");
    self.write("name", &generic_type.name);
    if generic_type.default_value.is_some() {
      self.write(
        "luauType",
        &generic_type
          .default_value
          .map_or(null_mut(), NonNull::as_ptr),
      );
    }
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:496-506` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_generic_type_pack(&mut self, generic_type_pack: &AstGenericTypePack) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view("AstGenericTypePack");
    self.write("name", &generic_type_pack.name);
    if generic_type_pack.default_value.is_some() {
      self.write(
        "luauType",
        &generic_type_pack
          .default_value
          .map_or(null_mut(), NonNull::as_ptr),
      );
    }
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:508-519` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_expr_table_item_kind(&mut self, kind: ItemKind) {
    match kind {
      ItemKind::List => self.write_string("item"),
      ItemKind::Record => self.write_string("record"),
      ItemKind::General => self.write_string("general"),
    }
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:521-539` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_expr_table_item(&mut self, item: &Item) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view("AstExprTableItem");
    self.write("kind", &item.kind);
    match item.kind {
      ItemKind::List => {
        self.write("value", &item.value);
      }
      _ => {
        self.write("key", &item.key);
        self.write("value", &item.value);
      }
    }
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:541-555` (hand-ported)
write_json_node!(write_ast_expr_if_else, AstExprIfElse, "AstExprIfElse", [
  "condition": condition,
  "hasThen": has_then,
  "trueExpr": true_expr,
  "hasElse": has_else,
  "falseExpr": false_expr,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:557-568` (hand-ported)
write_json_node!(write_ast_expr_interp_string, AstExprInterpString, "AstExprInterpString", [
  "strings": strings,
  "expressions": expressions,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:570-580` (hand-ported)
write_json_node!(write_ast_expr_table, AstExprTable, "AstExprTable", [
  "items": items,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:582-593` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_expr_unary_op(&mut self, op: AstExprUnaryOp) {
    self.write_string(op.into());
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:595-606` (hand-ported)
write_json_node!(write_ast_expr_unary, AstExprUnary, "AstExprUnary", [
  "op": op,
  "expr": expr,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:608-647` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_expr_binary_op(&mut self, op: AstExprBinaryOp) {
    if op == AstExprBinaryOp::OpCount {
      LUAU_ASSERT!(false);
      return;
    }
    self.write_string(op.into());
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:649-661` (hand-ported)
write_json_node!(write_ast_expr_binary, AstExprBinary, "AstExprBinary", [
  "op": op,
  "left": left,
  "right": right,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:663-674` (hand-ported)
write_json_node!(write_ast_expr_type_assertion, AstExprTypeAssertion, "AstExprTypeAssertion", [
  "expr": expr,
  "annotation": annotation,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:676-687` (hand-ported)
write_json_node!(write_ast_expr_error, AstExprError, "AstExprError", [
  "expressions": expressions,
  "messageIndex": message_index,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:689-712` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_stat_block(&mut self, node: &AstStatBlock) {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstStatBlock", |e| {
      e.write_raw_string_view(",\"hasEnd\":");
      node.has_end.write_json(e);
      e.write_raw_string_view(",\"body\":");
      node.body.write_json(e);
    });
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:714-728` (hand-ported)
impl AstJsonEncoder {
  /// elsebody 是可空指针槽位，仅在 `is_null()` 过滤后才被下传。
  pub fn write_ast_stat_if(&mut self, node: &AstStatIf) {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstStatIf", |e| {
      e.write("condition", &node.condition);
      e.write("thenbody", &node.thenbody);
      if !node.elsebody.is_null() {
        e.write("elsebody", &node.elsebody);
      }
      e.write("hasThen", &node.then_location.is_some());
    });
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:730-742` (hand-ported)
write_json_node!(write_ast_stat_while, AstStatWhile, "AstStatWhile", [
  "condition": condition,
  "body": body,
  "hasDo": has_do,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:744-755` (hand-ported)
write_json_node!(write_ast_stat_repeat, AstStatRepeat, "AstStatRepeat", [
  "condition": condition,
  "body": body,
]);

write_json_node!(write_ast_stat_break, AstStatBreak, "AstStatBreak", []);

write_json_node!(
  write_ast_stat_continue,
  AstStatContinue,
  "AstStatContinue",
  []
);

// Source: `Analysis/src/AstJsonEncoder.cpp:767-777` (hand-ported)
write_json_node!(write_ast_stat_return, AstStatReturn, "AstStatReturn", [
  "list": list,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:779-789` (hand-ported)
write_json_node!(write_ast_stat_expr, AstStatExpr, "AstStatExpr", [
  "expr": expr,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:942-953` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_declared_extern_type_property(&mut self, prop: &AstDeclaredExternTypeProperty) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write("name", &prop.name);
    self.write("nameLocation", &prop.name_location);
    self.write_type_string_view("AstDeclaredClassProp");
    self.write("luauType", &prop.ty);
    self.write("location", &prop.location);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:1010-1022` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_table_prop(&mut self, prop: &AstTableProp) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write("name", &prop.name);
    self.write_type_string_view("AstTableProp");
    self.write("location", &prop.location);
    self.write("propType", &prop.r#type);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:1024-1035` (hand-ported)
write_json_node!(write_ast_type_table, AstTypeTable, "AstTypeTable", [
  "props": props,
  "indexer": indexer,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:1037-1053` (hand-ported)
impl AstJsonEncoder {
  /// `None` 对应 C++ 空 optional indexer（表类型未写 indexer），写出字面
  /// `null`；可空指针槽位由调用桥接经 `Handle::from_opt_ptr` 判别。
  pub fn write_ast_table_indexer(&mut self, indexer: Option<&AstTableIndexer>) {
    if let Some(i) = indexer {
      self.write_raw_string_view("{");
      let c = self.push_comma();
      self.write("location", &i.location);
      self.write("indexType", &i.index_type);
      self.write("resultType", &i.result_type);
      self.pop_comma(c);
      self.write_raw_string_view("}");
    } else {
      self.write_raw_string_view("null");
    }
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:1055-1070` (hand-ported)
write_json_node!(write_ast_type_function, AstTypeFunction, "AstTypeFunction", [
  "attributes": attributes,
  "generics": generics,
  "genericPacks": generic_packs,
  "argTypes": arg_types,
  "argNames": arg_names,
  "returnTypes": return_types,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:1072-1082` (hand-ported)
write_json_node!(write_ast_type_typeof, AstTypeTypeof, "AstTypeTypeof", [
  "expr": expr,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:108-131` (hand-ported)
impl AstJsonEncoder {
  pub fn write_f64(&mut self, d: f64) {
    if d.is_infinite() {
      self.write_raw_string_view(if d < 0.0 { "-Infinity" } else { "Infinity" });
    } else if d.is_nan() {
      self.write_raw_string_view("NaN");
    } else {
      let formatted = format_g(d, 17);
      self.write_raw_string_view(&formatted);
    }
  }
}

write_json_node!(
  write_ast_type_optional,
  AstTypeOptional,
  "AstTypeOptional",
  []
);

// Source: `Analysis/src/AstJsonEncoder.cpp:1089-1099` (hand-ported)
write_json_node!(write_ast_type_union, AstTypeUnion, "AstTypeUnion", [
  "types": types,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:1101-1111` (hand-ported)
write_json_node!(write_ast_type_intersection, AstTypeIntersection, "AstTypeIntersection", [
  "types": types,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:1113-1124` (hand-ported)
write_json_node!(write_ast_type_error, AstTypeError, "AstTypeError", [
  "types": types,
  "messageIndex": message_index,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:1126-1136` (hand-ported)
write_json_node!(write_ast_type_pack_explicit, AstTypePackExplicit, "AstTypePackExplicit", [
  "typeList": type_list,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:1138-1148` (hand-ported)
write_json_node!(write_ast_type_pack_variadic, AstTypePackVariadic, "AstTypePackVariadic", [
  "variadicType": variadic_type,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:1150-1160` (hand-ported)
write_json_node!(write_ast_type_pack_generic, AstTypePackGeneric, "AstTypePackGeneric", [
  "genericName": generic_name,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:1162-1172` (hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_attr(&mut self, node: &AstAttr) {
    self.write_node_ast_node_string_view_f(&node.base.location, "AstAttr", |e| {
      e.write("name", &node.name);
    });
  }

  /// 批 2 存储面：cpp `write(char)`（AstJsonEncoder.cpp:156 一字符 STRING 写）
  /// 承接 `AstExprIndexName::op` 的 u8 字节域，字节→char 逐位同值。
  pub fn write_u8(&mut self, c: u8) {
    let buf = [c];
    self.write_string(from_utf8(&buf).unwrap_or(""));
  }

  pub fn write_i32(&mut self, i: u32) {
    let s = i.to_string();
    self.write_raw_string_view(&s);
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:188-191, hand-ported)
impl AstJsonEncoder {
  // write(std::string_view str) — writeString(str) expansion
  pub fn write_string_view(&mut self, str: &str) {
    self.write_string(str);
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:210-219, hand-ported)
impl AstJsonEncoder {
  pub fn write_ast_argument_name(&mut self, name: AstArgumentName) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view("AstArgumentName");
    self.write("name", &name.0);
    self.write("location", &name.1);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }

  pub fn write_position(&mut self, position: &Position) {
    self.write_i32(position.line);
    self.write_raw_string_view(",");
    self.write_i32(position.column);
  }

  pub fn write_location(&mut self, location: &Location) {
    self.write_raw_string_view("\"");
    self.write_position(&location.begin);
    self.write_raw_string_view(" - ");
    self.write_position(&location.end);
    self.write_raw_string_view("\"");
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:237-252` (hand-ported)
impl AstJsonEncoder {
  /// annotation 是可空指针槽位，仅经 `is_null()` 判断后决定走 write 或写字面
  /// null，从不解引用空值。
  pub fn write_ast_local(&mut self, local: &AstLocal) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    if !local.annotation.is_null() {
      self.write("luauType", &local.annotation);
    } else {
      // C++ write("luauType", nullptr)
      if self.comma {
        self.write_raw_string_view(",");
      }
      self.comma = true;
      self.write_raw_string_view("\"luauType\":");
      self.write_raw_string_view("null");
    }
    self.write("name", &local.name);
    self.write("isConst", &local.is_const);
    self.write_type_string_view("AstLocal");
    self.write("location", &local.location);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:271-274` (hand-ported)
impl AstJsonEncoder {
  /// # Safety
  /// `node` 允许为 null（对应 C++ 里对空 optional 指针的 visit，dispatch 内先
  /// 判空返回）；非 null 时须指向存活的 `AstNode`，且调用方在本轮编码期间独占
  /// 该节点所在 arena（无并发读者/写者）——`ast_node_visit` 按 C++ 非 const
  /// `visit(AstVisitor*)` 语义允许 visitor 写穿节点，本编码器的 hook 只回读。
  /// 本方法是解析 arena 裸指针域（各节点字段槽位的 `*mut AstExpr`/`*mut AstType`
  /// 等）与类型化 visitor 分发（ulua-ast 侧 `unsafe fn` API）之间的边界：指针
  /// 无法在 arena 字段上表达为引用，只能在桥接处按上述契约下传。
  // C++ write(AstNode* node) { node->visit(this); } -- virtual dispatch on
  // the node's dynamic type, landing in this encoder's visit overrides.
  pub unsafe fn write_ast_node(&mut self, node: *mut AstNode) {
    // Safety: 契约给出「node 为 null 或指向存活节点、调用方独占 arena」，与
    // ast_node_visit 自身契约逐字一致；指针原样转发，本函数不自行解引用。
    unsafe {
      ast_node_visit(node, self);
    }
  }
}

// Source: `Analysis/src/AstJsonEncoder.cpp:276-286` (hand-ported)
write_json_node!(write_ast_expr_group, AstExprGroup, "AstExprGroup", [
  "expr": expr,
]);

write_json_node!(
  write_ast_expr_constant_nil,
  AstExprConstantNil,
  "AstExprConstantNil",
  []
);

// Source: `Analysis/src/AstJsonEncoder.cpp:293-303` (hand-ported)
write_json_node!(write_ast_expr_constant_bool, AstExprConstantBool, "AstExprConstantBool", [
  "value": value,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:305-315` (hand-ported)
write_json_node!(write_ast_expr_constant_number, AstExprConstantNumber, "AstExprConstantNumber", [
  "value": value,
]);

// Source: `Analysis/src/AstJsonEncoder.cpp:317-327` (hand-ported)
write_json_node!(write_ast_expr_constant_integer, AstExprConstantInteger, "AstExprConstantInteger", [
  "value": value,
]);

json_visit_delegator!(
  pub(crate),
  visit_ast_expr_constant_integer,
  write_ast_expr_constant_integer,
  AstExprConstantInteger
);

// Source: `Analysis/src/AstJsonEncoder.cpp:329-339` (hand-ported)
write_json_node!(write_ast_expr_constant_string, AstExprConstantString, "AstExprConstantString", [
  "value": value,
]);
