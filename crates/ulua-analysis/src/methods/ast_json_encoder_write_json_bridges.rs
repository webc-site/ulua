//! Source: `Analysis/src/AstJsonEncoder.cpp:89-1052` (hand-ported)
//! C++ resolves `write(prop_name, value)` by overload on the static type of
//! `value`; Rust resolves it through `WriteJson`. One impl per C++ overload,
//! each delegating to the inherent method holding that overload's body.
//! Base-node pointers go through `write(AstNode*) = node->visit(this)`
//! (AstJsonEncoder.cpp:271); concrete-node pointers bind their concrete
//! overload exactly as C++ overload resolution does.
// write(AstNode*) and the implicit base-pointer conversions: virtual dispatch.

use ulua_ast::{
  records::{
    ast_array::AstArray,
    ast_attr::AstAttr,
    ast_declared_extern_type_property::AstDeclaredExternTypeProperty,
    ast_expr::AstExpr,
    ast_expr_binary::AstExprBinaryOp,
    ast_expr_function::AstExprFunction,
    ast_expr_table::{Item, ItemKind},
    ast_expr_unary::AstExprUnaryOp,
    ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack,
    ast_local::AstLocal,
    ast_node::AstNode,
    ast_stat::AstStat,
    ast_stat_block::AstStatBlock,
    ast_table_indexer::AstTableIndexer,
    ast_table_prop::AstTableProp,
    ast_type::AstType,
    ast_type_list::AstTypeList,
    ast_type_or_pack::AstTypeOrPack,
    ast_type_pack::AstTypePack,
    location::Location,
    node_handle::{Node, Nodes, OptNode},
    position::Position,
  },
  rtti::AstNodePtr,
  type_aliases::ast_argument_name::AstArgumentName,
};

use crate::{
  methods::ast_json_encoder_write_primitives::WriteJson,
  records::{arena_handle::Handle, ast_json_encoder::AstJsonEncoder},
};
impl WriteJson for *mut AstNode {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // Safety: 指针值取自本轮被编码节点的基类指针槽位/数组元素（解析 arena 中
    // 随 SourceModule allocator 存活的对象，整轮编码期内不释放、不改写），允许
    // 为 null；与 write_ast_node 的 # Safety（null 或指向首字段为 AstNode 的
    // 存活节点）逐项吻合，dispatch 内先判空再按 class_index 虚分派。编码是
    // 单线程只读遍历，输出缓冲在 enc 内与 arena 内存不相交。
    unsafe { enc.write_ast_node(*self) };
  }
}
impl WriteJson for *mut AstExpr {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // Safety: C++ 将 write(prop, AstExpr*) 绑定到基类 write(AstNode*) 重载；
    // repr(C) 单继承布局使 *mut AstExpr→*mut AstNode 上转为基址保持（地址数值
    // 不变），指针的存活/空性不变，callee 契约直接继承。来源如 binary/unary
    // 节点的左右操作数槽位，编码期内随解析树有效。
    unsafe { enc.write_ast_node((*self).as_ast_node()) };
  }
}
impl WriteJson for *mut AstStat {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // Safety: 基址保持上转，同 AstExpr 桥接；指针源自 AstStatBlock::body 等
    // 语句数组元素，parser 逐语句分配、非空且随树存活，满足 write_ast_node
    // 对存活基指针的要求。
    unsafe { enc.write_ast_node((*self).as_ast_node()) };
  }
}
impl WriteJson for *mut AstType {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // Safety: 基址保持上转；指针来自 AstTypeList::types 元素、annotation 槽位
    // 等类型标注字段，为解析 arena 存活节点或 null（可缺省标注），后者由
    // write_ast_node 契约吸收。编码期内无人写树，无别名冲突。
    unsafe { enc.write_ast_node((*self).as_ast_node()) };
  }
}
impl WriteJson for *mut AstTypePack {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // Safety: 基址保持上转；指针源自 typeList.tailType 之类的 typePack 标注
    // 槽位（可空，空槽即 C++ 未写 tailType 的情形），非空时指向随父节点存活
    // 的 arena 对象，落入 write_ast_node 的 null-或-live 契约。
    unsafe { enc.write_ast_node((*self).as_ast_node()) };
  }
}
// Concrete static types bind their exact overload (no virtual call in C++).
// arena 字段槽位的非空裸指针经 `Handle::from_ptr`（null 属契约违例、确定性
// panic）物化为共享引用后调用 safe write；被调方全程只读，输出缓冲与 arena
// 内存不相交，满足 arena_handle 模块级契约。
impl WriteJson for *mut AstStatBlock {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // 该具体类型槽位（如 AstStatWhile::body、AstStatIf::thenbody 的
    // *mut AstStatBlock 字段）由 parser 随语句必配分配，编码时非空且存活。
    enc.write_ast_stat_block(Handle::from_ptr(*self).get());
  }
}
impl WriteJson for *mut AstExprFunction {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // 源自 AstStatFunction/AstStatLocalFunction 的 func 槽位，函数表达式随
    // 声明一体分配、非空存活；callee 内部对可缺省字段先判空再下传。
    enc.write_ast_expr_function(Handle::from_ptr(*self).get());
  }
}
impl WriteJson for *mut AstLocal {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // AstExprLocal::local 与绑定同生命周期，parser 为已解析的局部变量引用
    // 必附 AstLocal，非空且随解析树存活；annotation 可空由 callee 判空处理。
    enc.write_ast_local(Handle::from_ptr(*self).get());
  }
}
impl WriteJson for *mut AstAttr {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // 函数 attributes 数组元素（@native 标注），parser 逐元素分配，长度内
    // 元素非空、parse arena 独立分配到编码结束。
    enc.write_ast_attr(Handle::from_ptr(*self).get());
  }
}
impl WriteJson for *mut AstGenericType {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // generics 数组元素，由 parser 随函数/类型别名一并分配；数组长度内指针
    // 非空且随 arena 存活，default_value 的可空处理在 callee 内。
    enc.write_ast_generic_type(Handle::from_ptr(*self).get());
  }
}
impl WriteJson for *mut AstGenericTypePack {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // genericPacks 数组元素，同 AstGenericType 的分配方式（parse arena 独立
    // 对象、长度内非空）。
    enc.write_ast_generic_type_pack(Handle::from_ptr(*self).get());
  }
}
impl WriteJson for *mut AstTableIndexer {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    // 源自 AstTypeTable::indexer 字段槽位，可为 null（表类型未写 indexer 的
    // C++ 空 optional），经 `from_opt_ptr` 表达为 Option 引用，null 分支由
    // callee 写字面 null，两分支输出与原指针版一致。
    enc.write_ast_table_indexer(Handle::from_opt_ptr(*self).map(|h| h.get()));
  }
}
impl WriteJson for Location {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_location(self);
  }
}
impl WriteJson for Position {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_position(self);
  }
}
// AstArgumentName = std::pair<AstName, Location>
impl WriteJson for AstArgumentName {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_ast_argument_name(*self);
  }
}
impl WriteJson for AstTypeList {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_ast_type_list(self);
  }
}
impl WriteJson for Item {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_ast_expr_table_item(self);
  }
}
impl WriteJson for ItemKind {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_ast_expr_table_item_kind(*self);
  }
}
impl WriteJson for AstExprUnaryOp {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_ast_expr_unary_op(*self);
  }
}
impl WriteJson for AstExprBinaryOp {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_ast_expr_binary_op(*self);
  }
}
impl WriteJson for AstTypeOrPack {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_ast_type_or_pack(self);
  }
}
impl WriteJson for AstTableProp {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_ast_table_prop(self);
  }
}
impl WriteJson for AstDeclaredExternTypeProperty {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_ast_declared_extern_type_property(self);
  }
}
// NB: no WriteJson for u8 itself -- that would make AstArray<u8>
// overlap the generic AstArray<T: WriteJson> impl. The one char-typed field
// (AstExprIndexName::op) calls `write_u8` explicitly instead.
// write(AstArray<char>): the chars as one string (AstJsonEncoder.cpp:387).
// 批 2 存储面：cpp `AstArray<char>` → Rust `AstArray<u8>`（同字节域）。
impl WriteJson for AstArray<u8> {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_ast_array_u8(*self);
  }
}

impl<T> WriteJson for Node<T>
where
  *mut T: WriteJson,
{
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    self.as_ptr().write_json(enc);
  }
}

impl<T> WriteJson for OptNode<T>
where
  *mut T: WriteJson,
{
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    self.as_ptr().write_json(enc);
  }
}

impl<T> WriteJson for Nodes<T>
where
  Node<T>: WriteJson,
{
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_raw_string_view("[");
    let mut comma = false;
    for a in self.iter_nodes() {
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
