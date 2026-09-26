//! Printer：`prettyPrint`/`Visualizer` 的 Rust 打印器（cpp `Ast/src/PrettyPrinter.cpp`、
//! `Analysis/src/Visualizer.cpp` 的行为对应）。
//!
//! 本文件是全族单 impl 收口点：原先 `methods/printer_*.rs` 19 枚碎片文件（每枚一至数个
//! 方法、只属于 `Printer`、无跨 crate 路径消费者）并回此处。节点子指针一律经
//! [`slot_ref`](crate::functions::optional_node::slot_ref)（非空槽）/ [`slot_opt`](crate::functions::optional_node::slot_opt)
//! （可空槽，null 折叠为 `None`）读成共享引用；CST 侧经 [`lookup_cst_node`] 返回
//! `Option<&T>`，打印器只写 `Writer`、从不写穿节点。

use core::{
  mem::swap,
  ptr::{from_ref, null},
};

use ulua_common::{
  fflag, functions::format_g::format_g, macros::luau_assert::LUAU_ASSERT,
  records::variant::Variant2,
};

use crate::{
  enums::ast_table_access::AstTableAccess,
  functions::{
    escape_bytes::escape_bytes,
    optional_node::{node_opt, node_ref, slot_opt, slot_ref},
    to_string_ast::to_str_binary as to_str,
  },
  records::{
    arg_name_inserter::ArgNameInserter,
    ast_array::AstArray,
    ast_attr::AstAttr,
    ast_class_method::AstClassMethod,
    ast_class_property::AstClassProperty,
    ast_expr::AstExpr,
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
    ast_expr_instantiate::AstExprInstantiate,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_expr_varargs::AstExprVarargs,
    ast_local::AstLocal,
    ast_node::AstNode,
    ast_stat::AstStat,
    ast_stat_assign::AstStatAssign,
    ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue,
    ast_stat_declare_global::AstStatDeclareGlobal,
    ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr,
    ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction,
    ast_stat_if::AstStatIf,
    ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction,
    ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn,
    ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction,
    ast_stat_while::AstStatWhile,
    ast_type::AstType,
    ast_type_error::AstTypeError,
    ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection,
    ast_type_list::AstTypeList,
    ast_type_optional::AstTypeOptional,
    ast_type_or_pack::AstTypeOrPack,
    ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic,
    ast_type_reference::AstTypeReference,
    ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString,
    ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof,
    ast_type_union::AstTypeUnion,
    comma_separator_inserter::CommaSeparatorInserter,
    cst_expr_call::CstExprCall,
    cst_expr_constant_integer::CstExprConstantInteger,
    cst_expr_constant_number::CstExprConstantNumber,
    cst_expr_constant_string::CstExprConstantString,
    cst_expr_explicit_type_instantiation::CstExprExplicitTypeInstantiation,
    cst_expr_function::CstExprFunction,
    cst_expr_group::CstExprGroup,
    cst_expr_if_else::CstExprIfElse,
    cst_expr_index_expr::CstExprIndexExpr,
    cst_expr_interp_string::CstExprInterpString,
    cst_expr_op::CstExprOp,
    cst_expr_table::{
      CstExprTable, CstExprTableSeparator,
      CstExprTableSeparator::{Comma, Missing},
    },
    cst_expr_type_assertion::CstExprTypeAssertion,
    cst_generic_type::CstGenericType,
    cst_generic_type_pack::CstGenericTypePack,
    cst_stat_assign::CstStatAssign,
    cst_stat_compound_assign::CstStatCompoundAssign,
    cst_stat_do::CstStatDo,
    cst_stat_for::CstStatFor,
    cst_stat_for_in::CstStatForIn,
    cst_stat_function::CstStatFunction,
    cst_stat_local::CstStatLocal,
    cst_stat_local_function::CstStatLocalFunction,
    cst_stat_repeat::CstStatRepeat,
    cst_stat_return::CstStatReturn,
    cst_stat_type_alias::CstStatTypeAlias,
    cst_stat_type_function::CstStatTypeFunction,
    cst_type_function::CstTypeFunction,
    cst_type_group::CstTypeGroup,
    cst_type_instantiation::CstTypeInstantiation,
    cst_type_intersection::CstTypeIntersection,
    cst_type_pack_explicit::CstTypePackExplicit,
    cst_type_pack_generic::CstTypePackGeneric,
    cst_type_reference::CstTypeReference,
    cst_type_singleton_string::CstTypeSingletonString,
    cst_type_table::{
      CstTypeTable,
      CstTypeTableItemKind::{Indexer, StringProperty},
    },
    cst_type_typeof::CstTypeTypeof,
    cst_type_union::CstTypeUnion,
    location::Location,
    node_handle::{Node, OptNode},
    position::{EMPTY_POSITIONS, Position},
    writer::Writer,
  },
  rtti::{AstNodeClass, AstNodeView, CstNodeClass, ast_node_is, ast_node_try_as, cst_node_as_ref},
  type_aliases::{ast_argument_name::AstArgumentName, cst_node_map::CstNodeMap},
};

/// `visualize_*` 系列入参归一：裸指针与共享引用统一取 `*const T`，镜像 C++
/// 按指针/引用重载传节点的调用形态。
///
/// 打印器对 AST 只读（写入只发生在 `Writer` 上），故这里只交出 const 指针：
/// 从共享引用造 `*mut`/`&mut` 再写穿节点正是本 crate 消除的别名 UB 形态。
pub trait IntoNodePtr<T> {
  fn into_node_ptr(self) -> *const T;
}

impl<T> IntoNodePtr<T> for *mut T {
  fn into_node_ptr(self) -> *const T {
    self.cast_const()
  }
}

impl<T> IntoNodePtr<T> for *const T {
  fn into_node_ptr(self) -> *const T {
    self
  }
}

impl<T> IntoNodePtr<T> for Option<&T> {
  fn into_node_ptr(self) -> *const T {
    self.map_or(null(), |p| p)
  }
}

impl<T> IntoNodePtr<T> for &T {
  fn into_node_ptr(self) -> *const T {
    self
  }
}

impl<T> IntoNodePtr<T> for &*mut T {
  fn into_node_ptr(self) -> *const T {
    (*self).cast_const()
  }
}

// 句柄化字段的打印器桥:与 `AstNodePtr` 的同源过渡语义,引用化全量完成后退役。
impl<T> IntoNodePtr<T> for Node<T> {
  fn into_node_ptr(self) -> *const T {
    self.as_ptr().cast_const()
  }
}

impl<T> IntoNodePtr<T> for &Node<T> {
  fn into_node_ptr(self) -> *const T {
    self.as_ptr().cast_const()
  }
}

impl<T> IntoNodePtr<T> for OptNode<T> {
  fn into_node_ptr(self) -> *const T {
    self.as_ptr().cast_const()
  }
}

/// `W: Writer` 泛型替代 `&mut dyn Writer`：Writer 的实现者全部在本 crate 内，
/// 静态分发消除 visualize 热路径上逐 token 的虚调用（仍镜像 C++ 虚接口）。
pub struct Printer<'a, W: Writer> {
  pub(crate) write_types: bool,
  pub(crate) writer: &'a mut W,
  pub(crate) cst_node_map: &'a CstNodeMap,
}

/// `do <block> end` 等 advance 序列的位置入参归一（Value/&Value 两形态）。
pub trait IntoPosition {
  fn into_position(self) -> Position;
}

impl IntoPosition for Position {
  fn into_position(self) -> Position {
    self
  }
}

impl IntoPosition for &Position {
  fn into_position(self) -> Position {
    *self
  }
}

/// 数组表 indexer 的键类型名（解析器对 `{T}` 数组表固定生成 number 键）。
const NUMBER_KEY: &str = "number";

// cpp PrettyPrinter 高频 token 字面量单源化（review.md §6）：打印族重复出现的
// 符号/关键字收为 `'static str` 常量，消灭散点重复字符串。
const SYM_LPAREN: &str = "(";
const SYM_RPAREN: &str = ")";
const SYM_LBRACE: &str = "{";
const SYM_RBRACE: &str = "}";
const SYM_LBRACKET: &str = "[";
const SYM_LT: &str = "<";
const SYM_ASSIGN: &str = "=";
const SYM_COLON: &str = ":";
const SYM_ELLIPSIS: &str = "...";
const KWD_END: &str = "end";
const KWD_FUNCTION: &str = "function";
const KWD_EXPORT: &str = "export";
const KWD_DO: &str = "do";

/// `match node.class_index` 命中臂后的下转入口（review.md §2：本族 unsafe 归零）：
/// [`ast_node_try_as`] 是安全门面，内部复核 `class_index == T::CLASS_INDEX`；臂
/// 不变式保证复核恒命中，`Some` 侧 100% 成立、`unwrap` 不可触发（§6）。
/// 语义与 cpp `node->as<T>()`（失败即断言）一致。
#[inline]
fn as_node<T: AstNodeClass>(node: &impl AstNodeView) -> &T {
  ast_node_try_as(node).unwrap()
}

impl<'a, W: Writer> Printer<'a, W> {
  /// 查 ast→cst 映射并下转为 `T`。unsafe 收口于此：映射值指向 arena 中存活的
  /// CST 节点，`cst_node_as` 经 class_index 命中后 repr(C) 布局保证下转有效。
  ///
  /// 生命周期取自入参借用 `'b`（不再借用 Printer 的 writer 生命周期 `'a`）：
  /// AST 与 CST 同处一个 arena，故「被查询的 AST 节点在 `'b` 内存活」即蕴含
  /// 其映射到的 CST 节点在 `'b` 内存活。键只按地址比较，从不解引用写回。
  pub(crate) fn lookup_cst_node<'b, T: CstNodeClass>(
    &self,
    ast_node: &'b AstNode,
  ) -> Option<&'b T> {
    // cpp 侧键型为 `AstNode*`；这里仅把地址重新拼回该形态用于查表。
    let key = from_ref(ast_node).cast_mut();
    let cst_node = *self.cst_node_map.find(&key)?;
    // Safety: 同函数级注释；CST 节点与 `ast_node` 同 arena，存活期覆盖 `'b`。
    unsafe { cst_node_as_ref::<T>(cst_node) }
  }

  pub fn new(writer: &'a mut W, cst_node_map: &'a CstNodeMap) -> Self {
    Self {
      write_types: false,
      writer,
      cst_node_map,
    }
  }

  pub fn advance<P: IntoPosition>(&mut self, new_pos: P) {
    let new_pos = new_pos.into_position();
    LUAU_ASSERT!(new_pos.has_value());
    self.writer.advance(&new_pos);
  }

  pub fn advance_before(&mut self, new_pos: Position, token_length: u32) {
    // cpp `advanceBefore`：列够减则回退 token 宽度，否则原样前进（§3 消双分支样板）
    let column = new_pos
      .column
      .checked_sub(token_length)
      .unwrap_or(new_pos.column);
    self.advance(Position::new(new_pos.line, column));
  }

  /// cpp `is_integerish`：可无损往返 i32 且排除 `-0.0`。
  pub fn printer_is_integerish(d: f64) -> bool {
    (i32::MIN as f64..=i32::MAX as f64).contains(&d)
      && (d as i32 as f64) == d
      && !(d == 0.0 && d.is_sign_negative())
  }

  /// cpp `Printer::visit(AstExpr::Binary*)` 无 CST 分支：间距按操作符字宽
  /// 逐档给定（2/3/4）。`OpCount` 值域外，断言后落 0（cpp `default` 同形）。
  fn binary_operand_gap(op: AstExprBinaryOp) -> i32 {
    match op {
      AstExprBinaryOp::Add
      | AstExprBinaryOp::Sub
      | AstExprBinaryOp::Mul
      | AstExprBinaryOp::Div
      | AstExprBinaryOp::FloorDiv
      | AstExprBinaryOp::Mod
      | AstExprBinaryOp::Pow
      | AstExprBinaryOp::CompareLt
      | AstExprBinaryOp::CompareGt => 2,
      AstExprBinaryOp::Concat
      | AstExprBinaryOp::CompareNe
      | AstExprBinaryOp::CompareEq
      | AstExprBinaryOp::CompareLe
      | AstExprBinaryOp::CompareGe
      | AstExprBinaryOp::Or => 3,
      AstExprBinaryOp::And => 4,
      AstExprBinaryOp::OpCount => {
        LUAU_ASSERT!(false);
        0
      }
    }
  }

  /// cpp `Printer::visit(AstStatCompoundAssign*)` 的复合赋值符号表；
  /// 非复合操作符为值域外（断言后落空串，调用侧以空串短路）。
  fn compound_assign_symbol(op: AstExprBinaryOp) -> &'static str {
    match op {
      AstExprBinaryOp::Add => "+=",
      AstExprBinaryOp::Sub => "-=",
      AstExprBinaryOp::Mul => "*=",
      AstExprBinaryOp::Div => "/=",
      AstExprBinaryOp::FloorDiv => "//=",
      AstExprBinaryOp::Mod => "%=",
      AstExprBinaryOp::Pow => "^=",
      AstExprBinaryOp::Concat => "..=",
      _ => {
        LUAU_ASSERT!(false);
        ""
      }
    }
  }

  pub fn maybe_advance_and_write(&mut self, pos: &Position, s: &str, always_write: bool) {
    // 实参恒为编译期符号字面量；write 走字节通道，此处收口转换。
    if pos.has_value() {
      self.advance(pos);
      self.writer.write(s.as_bytes());
    } else if always_write {
      self.writer.write(s.as_bytes());
    }
  }

  /// CST 位置存在时校准前进并写符号，CST 缺失时无条件写符号；CST 存在但
  /// 位置无值时不写。合并 cpp 的 `maybeAdvanceAndWrite(pos, s)` + else
  /// `writer.symbol(s)` 双分支（visualize 系列高频同款）。
  pub(crate) fn maybe_advance_or_symbol(&mut self, pos: Option<&Position>, s: &str) {
    match pos {
      Some(pos) if pos.has_value() => {
        self.advance(pos);
        self.writer.symbol(s);
      }
      None => self.writer.symbol(s),
      Some(_) => {}
    }
  }

  pub fn write_end(&mut self, loc: &Location) {
    let mut end_pos = loc.end;
    if end_pos.column >= 3 {
      end_pos.column -= 3;
    }
    self.advance(end_pos);
    self.writer.keyword(KWD_END);
  }

  /// 表访问关键字（read/write）：CST 有 access_location 时校准前进并写
  /// （cpp `visualizeTypeTable` 内三处同款合并）。
  pub(crate) fn write_table_access(
    &mut self,
    access: AstTableAccess,
    access_location: Option<Location>,
  ) {
    if let Some(loc) = access_location {
      self.advance(loc.begin);
      self.writer.keyword(if access == AstTableAccess::Read {
        "read"
      } else {
        "write"
      });
    }
  }

  pub fn visualize_ast_local_position(&mut self, local: &AstLocal, colon_position: Position) {
    self.advance(local.location.begin);

    self.writer.identifier(local.name.as_bytes());
    if self.write_types
      && let Some(annotation) = slot_opt(local.annotation)
    {
      self.maybe_advance_and_write(&colon_position, SYM_COLON, true);
      self.visualize_type_annotation(annotation);
    }
  }

  pub fn visualize_ast_expr<E: IntoNodePtr<AstExpr>>(&mut self, expr: E) {
    // expr 指向 arena 中存活的 AstExpr 派生节点（null 槽位折叠为 None 直接返回，
    // 与旧判空早退等价）；共享借用供只读递归打印，打印器不写节点，无 `&mut` 别名。
    let Some(expr_ref) = slot_opt(expr.into_node_ptr()) else {
      return;
    };
    // cpp 侧 `AstNode* node = expr` 的只读形态：下转与 CST 查表都以共享借用为
    // 入参（打印器只写 Writer，从不写节点）。
    let node = &expr_ref.base;
    self.advance(expr_ref.base.location.begin);

    match node.class_index {
      AstExprGroup::CLASS_INDEX => {
        let a = as_node::<AstExprGroup>(node);
        self.writer.symbol(SYM_LPAREN);
        self.visualize_ast_expr(a.expr);

        let cst_node = self.lookup_cst_node::<CstExprGroup>(node);
        if let Some(cst_node) = cst_node {
          self.maybe_advance_and_write(&cst_node.close_position, SYM_RPAREN, false);
        } else {
          self.advance_before(a.base.base.location.end, 1);
          self.writer.symbol(SYM_RPAREN);
        }
      }
      AstExprConstantNil::CLASS_INDEX => {
        self.writer.keyword("nil");
      }
      AstExprConstantBool::CLASS_INDEX => {
        let a = as_node::<AstExprConstantBool>(node);
        self.writer.keyword(if a.value { "true" } else { "false" });
      }
      AstExprConstantNumber::CLASS_INDEX => {
        let a = as_node::<AstExprConstantNumber>(node);
        if let Some(cst_node) = self.lookup_cst_node::<CstExprConstantNumber>(node) {
          // 数字源文本直切片（非 fixup 产物）；literal 走字节通道。
          self.writer.literal(cst_node.value.as_bytes());
        } else if a.value.is_infinite() {
          self.writer.literal(if a.value.is_sign_positive() {
            "1e500".as_bytes()
          } else {
            "-1e500".as_bytes()
          });
        } else if a.value.is_nan() {
          self.writer.literal("0/0".as_bytes());
        } else if Self::printer_is_integerish(a.value) {
          // itoa 栈上缓冲直写，免 String 堆分配
          let mut buf = itoa::Buffer::new();
          self.writer.literal(buf.format(a.value as i32).as_bytes());
        } else {
          self.writer.literal(format_g(a.value, 17).as_bytes());
        }
      }
      AstExprConstantInteger::CLASS_INDEX => {
        let a = as_node::<AstExprConstantInteger>(node);
        if let Some(cst_node) = self.lookup_cst_node::<CstExprConstantInteger>(node) {
          // 数字源文本直切片（非 fixup 产物）；literal 走字节通道。
          self.writer.literal(cst_node.value.as_bytes());
        } else if a.value >= 0 {
          // cpp `snprintf(buffer, "%lldi")`：栈上字符数组拼接。itoa 本身无分配
          // （内联 Buffer），只需把它的字节抄进栈数组再补 'i'，避免 `format!`
          // 的 String 堆分配。i64::MAX 十进制 19 位 + 'i'，24 足够。
          let mut buf = itoa::Buffer::new();
          let digits = buf.format(a.value).as_bytes();
          let mut out = [0u8; 24];
          out[..digits.len()].copy_from_slice(digits);
          out[digits.len()] = b'i';
          self.writer.literal(&out[..digits.len() + 1]);
        } else {
          // cpp `snprintf(buffer, "0x%llxi", (unsigned long long)value)`：
          // 负数以补码无符号形态打小写十六进制（`%llx` 无前导零）。逐位取 nibble
          // 写入栈数组，同样免 `format!` 堆分配。u64 最长 16 位 + "0x" + 'i'。
          const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
          let mut digits = [0u8; 16];
          let mut first = 16usize;
          let mut rest = a.value as u64;
          loop {
            first -= 1;
            digits[first] = HEX_DIGITS[(rest & 0xf) as usize];
            rest >>= 4;
            if rest == 0 {
              break;
            }
          }
          let count = 16 - first;
          let mut out = [0u8; 19];
          out[..2].copy_from_slice(b"0x");
          out[2..2 + count].copy_from_slice(&digits[first..]);
          out[2 + count] = b'i';
          self.writer.literal(&out[..3 + count]);
        }
      }
      AstExprConstantString::CLASS_INDEX => {
        let a = as_node::<AstExprConstantString>(node);
        if let Some(cst_node) = self.lookup_cst_node::<CstExprConstantString>(node) {
          // 源文本直切片（引号内原文，非 fixup 产物）。
          self.writer.source_string(
            cst_node.source_string.as_bytes(),
            cst_node.quote_style,
            cst_node.block_depth,
          );
        } else {
          // value 经 lexer_fixup_quoted_bytes，可含任意字节（`"\xff"` 非
          // UTF-8），必须走字节通道（cpp `std::string_view` 直写语义）。
          self.writer.string(a.value.as_bytes());
        }
      }
      AstExprLocal::CLASS_INDEX => {
        let a = as_node::<AstExprLocal>(node);
        // local 槽已句柄化恒非空：.get() 安全借用读 name。
        self.writer.identifier(a.local.get().name.as_bytes());
      }
      AstExprGlobal::CLASS_INDEX => {
        let a = as_node::<AstExprGlobal>(node);
        self.writer.identifier(a.name.as_bytes());
      }
      AstExprVarargs::CLASS_INDEX => {
        self.writer.symbol(SYM_ELLIPSIS);
      }
      AstExprCall::CLASS_INDEX => {
        let a = as_node::<AstExprCall>(node);
        self.visualize_ast_expr(a.func);

        let cst_node = self.lookup_cst_node::<CstExprCall>(node);
        // explicit_types 是 Option<NonNull<CstTypeInstantiation>>：Some 即 parser 本次
        // 解析写入的存活 CST 结构，`node_ref` 门面物化为共享引用只读消费。
        let explicit_types = cst_node.and_then(|cst| node_ref(cst.explicit_types));

        if self.write_types && (a.type_arguments.size > 0 || explicit_types.is_some()) {
          self.visualize_explicit_type_instantiation(a.type_arguments, explicit_types);
        }

        self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.open_parens), SYM_LPAREN);

        let mut comma = CommaSeparatorInserter::new(
          cst_node.map_or(EMPTY_POSITIONS, |cst| cst.comma_positions.as_slice()),
        );
        for arg in a.args.iter() {
          comma.write(self.writer);
          self.visualize_ast_expr(*arg);
        }

        self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.close_parens), SYM_RPAREN);
      }
      AstExprIndexName::CLASS_INDEX => {
        let a = as_node::<AstExprIndexName>(node);
        self.visualize_ast_expr(a.expr);
        self.advance(a.op_position);
        // 单字符编码进栈缓冲，免 String 堆分配（cpp `std::string(1, a->op)`）
        let mut opbuf = [0u8; 4];
        self.writer.symbol((a.op as char).encode_utf8(&mut opbuf));
        self.advance(a.index_location.begin);
        self.writer.write(a.index.as_bytes());
      }
      AstExprIndexExpr::CLASS_INDEX => {
        let a = as_node::<AstExprIndexExpr>(node);
        let cst_node = self.lookup_cst_node::<CstExprIndexExpr>(node);
        self.visualize_ast_expr(a.expr);

        self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.open_bracket_position), SYM_LBRACKET);

        self.visualize_ast_expr(a.index);

        self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.close_bracket_position), "]");
      }
      AstExprFunction::CLASS_INDEX => {
        let a = as_node::<AstExprFunction>(node);
        for attr in a.attributes.iter_nodes() {
          self.visualize_attribute(attr);
        }
        self.writer.keyword(KWD_FUNCTION);
        self.visualize_function_body(a);
      }
      AstExprTable::CLASS_INDEX => {
        let a = as_node::<AstExprTable>(node);
        self.writer.symbol(SYM_LBRACE);

        let cst_node = self.lookup_cst_node::<CstExprTable>(node);
        // CST 与 AST items 由解析器成对构造，长度一致（C++ 同处有断言）
        let cst_items = match cst_node {
          Some(cst) => {
            ulua_common::LUAU_ASSERT!(cst.items.len() == a.items.len());
            cst.items.as_slice()
          }
          None => &[],
        };

        for (index, item) in a.items.iter().enumerate() {
          // 无 CST 时逐项补逗号分隔符
          if cst_items.is_empty() && index > 0 {
            self.writer.symbol(",");
          }

          match cst_items.get(index) {
            Some(cst_item) => match item.kind {
              ItemKind::List => {}
              ItemKind::Record => {
                // Record 项 key 由解析器保证为常量字符串（断言同一不变式），class_index
                // 匹配后经安全 try_as 下转。
                let key = slot_opt(item.key)
                  .and_then(|k| ast_node_try_as::<AstExprConstantString>(&k.base));
                ulua_common::LUAU_ASSERT!(key.is_some());
                if let Some(key) = key {
                  self.advance(key.base.base.location.begin);
                  self.writer.identifier(key.value.as_bytes());
                  self.advance(cst_item.equals_position);
                  self.writer.symbol(SYM_ASSIGN);
                }
              }
              ItemKind::General => {
                ulua_common::LUAU_ASSERT!(cst_item.indexer_open_position.has_value());
                self.maybe_advance_and_write(&cst_item.indexer_open_position, SYM_LBRACKET, true);
                self.visualize_ast_expr(item.key);
                self.maybe_advance_and_write(&cst_item.indexer_close_position, "]", false);
                self.maybe_advance_and_write(&cst_item.equals_position, SYM_ASSIGN, false);
              }
            },
            None => match item.kind {
              ItemKind::List => {}
              ItemKind::Record => {
                // 同上（Record 项 key 为常量字符串）
                let key = slot_opt(item.key)
                  .and_then(|k| ast_node_try_as::<AstExprConstantString>(&k.base));
                ulua_common::LUAU_ASSERT!(key.is_some());
                if let Some(key) = key {
                  self.advance(key.base.base.location.begin);
                  self.writer.identifier(key.value.as_bytes());
                  self
                    .writer
                    .maybe_space(&slot_ref(item.value).base.location.begin, 1);
                  self.writer.symbol(SYM_ASSIGN);
                }
              }
              ItemKind::General => {
                self.writer.symbol(SYM_LBRACKET);
                self.visualize_ast_expr(item.key);
                self.writer.symbol("]");
                self
                  .writer
                  .maybe_space(&slot_ref(item.value).base.location.begin, 1);
                self.writer.symbol(SYM_ASSIGN);
              }
            },
          }

          let value = slot_ref(item.value);
          self.advance(value.base.location.begin);
          self.visualize_ast_expr(value);

          if let Some(cst_item) = cst_items.get(index) {
            let separator = cst_item.separator;
            if separator != CstExprTableSeparator::Missing {
              ulua_common::LUAU_ASSERT!(cst_item.separator_position.has_value());
              self.maybe_advance_and_write(
                &cst_item.separator_position,
                if separator == CstExprTableSeparator::Comma {
                  ","
                } else {
                  ";"
                },
                true,
              );
            }
          }
        }

        let mut end_pos = expr_ref.base.location.end;
        if end_pos.column > 0 {
          end_pos.column -= 1;
        }
        self.advance(end_pos);
        self.writer.symbol(SYM_RBRACE);
        self.advance(expr_ref.base.location.end);
      }
      AstExprUnary::CLASS_INDEX => {
        let a = as_node::<AstExprUnary>(node);
        if let Some(cst_node) = self.lookup_cst_node::<CstExprOp>(node) {
          self.advance(cst_node.op_position);
        }

        match a.op {
          AstExprUnaryOp::Not => self.writer.keyword("not"),
          AstExprUnaryOp::Minus => self.writer.symbol("-"),
          AstExprUnaryOp::Len => self.writer.symbol("#"),
        }
        self.visualize_ast_expr(a.expr);
      }
      AstExprBinary::CLASS_INDEX => {
        let a = as_node::<AstExprBinary>(node);
        self.visualize_ast_expr(a.left);

        if let Some(cst_node) = self.lookup_cst_node::<CstExprOp>(node) {
          self.advance(cst_node.op_position);
        } else {
          // 操作符宽度决定与右操作数的间距（C++ 同名逻辑）
          let gap = Self::binary_operand_gap(a.op);
          self
            .writer
            .maybe_space(&a.right.get().base.location.begin, gap);
        }

        self.writer.symbol(to_str(a.op));
        self.visualize_ast_expr(a.right);
      }
      AstExprTypeAssertion::CLASS_INDEX => {
        let a = as_node::<AstExprTypeAssertion>(node);
        self.visualize_ast_expr(a.expr);

        if self.write_types {
          match self.lookup_cst_node::<CstExprTypeAssertion>(node) {
            Some(cst_node) => self.advance(cst_node.op_position),
            // annotation 已句柄化恒非空：.get() 安全借用读基坐标。
            None => self
              .writer
              .maybe_space(&a.annotation.get().base.location.begin, 2),
          }
          self.writer.symbol("::");
          self.visualize_type_annotation(a.annotation);
        }
      }
      AstExprIfElse::CLASS_INDEX => {
        let a = as_node::<AstExprIfElse>(node);
        self.writer.keyword("if");
        self.visualize_else_if_expr(a);
      }
      AstExprInterpString::CLASS_INDEX => {
        let a = as_node::<AstExprInterpString>(node);
        let cst_node = self.lookup_cst_node::<CstExprInterpString>(node);

        self.writer.symbol("`");

        let expressions = a.expressions.as_slice();

        for (index, string) in AstArray::iter(&a.strings).enumerate() {
          if let Some(cst) = cst_node {
            if index > 0 {
              // string_positions 与 strings 等长（解析器成对构造）；越界仅解析
              // 器 bug，跳过 advance。
              if let Some(pos) = cst.string_positions.as_slice().get(index) {
                self.advance(*pos);
              }
              self.writer.symbol(SYM_RBRACE);
            }

            // source_strings 与 strings 等长（解析器成对构造）；越界仅解析器
            // bug，跳过该段（cpp 侧 data[index] 同样依赖该不变式）。
            if let Some(source_string) = cst.source_strings.as_slice().get(index) {
              // 源文本直切片（引号内原文，非 fixup 产物）。
              self.writer.write_multiline(source_string.as_bytes());
            }
          } else {
            // strings 经 fixup 可含任意字节；escape_bytes 字节版转义后直写。
            self.writer.write(&escape_bytes(string.as_bytes(), true));
          }

          if let Some(&expression) = expressions.get(index) {
            self.writer.symbol(SYM_LBRACE);
            self.visualize_ast_expr(expression);
            if cst_node.is_none() {
              self.writer.symbol(SYM_RBRACE);
            }
          }
        }

        self.writer.symbol("`");
      }
      AstExprError::CLASS_INDEX => {
        let a = as_node::<AstExprError>(node);
        self.writer.symbol("(error-expr");

        for (i, &expression) in a.expressions.iter().enumerate() {
          self.writer.symbol(if i == 0 { ": " } else { ", " });
          self.visualize_ast_expr(expression);
        }

        self.writer.symbol(SYM_RPAREN);
      }
      AstExprInstantiate::CLASS_INDEX => {
        let a = as_node::<AstExprInstantiate>(node);
        self.visualize_ast_expr(a.expr);

        if self.write_types {
          let cst_expr_node = self.lookup_cst_node::<CstExprExplicitTypeInstantiation>(node);
          self.visualize_explicit_type_instantiation(
            a.type_arguments,
            cst_expr_node.map(|cst| &cst.instantiation),
          );
        }
      }
      _ => {
        ulua_common::LUAU_ASSERT!(false);
      }
    }
  }

  pub fn visualize_ast_stat<S: IntoNodePtr<AstStat>>(&mut self, program: S) {
    // program 指向 arena 中存活的 AstStat 派生节点
    let program = slot_ref(program.into_node_ptr());
    // cpp 侧 `AstNode* node = program` 的只读形态：下转与 CST 查表都以共享借用
    // 为入参（打印器只写 Writer，从不写节点）。
    let node = &program.base;
    self.advance(program.base.location.begin);

    match node.class_index {
      AstStatBlock::CLASS_INDEX => {
        let block = as_node::<AstStatBlock>(node);
        if let Some(cst) = self.lookup_cst_node::<CstStatDo>(node) {
          self.writer.keyword(KWD_DO);
          self.advance(cst.stats_start_position);
          for s in block.body.iter() {
            self.visualize_ast_stat(s);
          }
          self.maybe_advance_and_write(&cst.end_position, KWD_END, false);
        } else {
          for s in block.body.iter() {
            self.visualize_ast_stat(s);
          }
          self.advance(block.base.base.location.end);
          self.write_end(&program.base.location);
        }
      }
      AstStatIf::CLASS_INDEX => {
        let a = as_node::<AstStatIf>(node);
        self.writer.keyword("if");
        self.visualize_else_if(a);
      }
      AstStatWhile::CLASS_INDEX => {
        let a = as_node::<AstStatWhile>(node);
        self.writer.keyword("while");
        self.visualize_ast_expr(a.condition);
        self.visualize_do_block(a.do_location, a.body.as_ptr());
      }
      AstStatRepeat::CLASS_INDEX => {
        let a = as_node::<AstStatRepeat>(node);
        self.writer.keyword("repeat");
        self.visualize_block_ast_stat_block(a.body);
        match self.lookup_cst_node::<CstStatRepeat>(node) {
          Some(cst) => self.maybe_advance_and_write(&cst.until_position, "until", false),
          None => {
            self.advance_before(a.condition.get().base.location.begin, 6);
            self.writer.keyword("until");
          }
        }
        self.visualize_ast_expr(a.condition);
      }
      AstStatBreak::CLASS_INDEX => {
        self.writer.keyword("break");
      }
      AstStatContinue::CLASS_INDEX => {
        self.writer.keyword("continue");
      }
      AstStatReturn::CLASS_INDEX => {
        let a = as_node::<AstStatReturn>(node);
        self.writer.keyword("return");
        let mut comma = CommaSeparatorInserter::new(
          self
            .lookup_cst_node::<CstStatReturn>(node)
            .map_or(EMPTY_POSITIONS, |cst| cst.comma_positions.as_slice()),
        );
        for expr in AstArray::iter(&a.list) {
          comma.write(self.writer);
          self.visualize_ast_expr(expr);
        }
      }
      AstStatExpr::CLASS_INDEX => {
        let a = as_node::<AstStatExpr>(node);
        self.visualize_ast_expr(a.expr);
      }
      AstStatLocal::CLASS_INDEX => {
        let a = as_node::<AstStatLocal>(node);
        let cst_node = self.lookup_cst_node::<CstStatLocal>(node);
        if fflag::LuauExportValueSyntax.get() && a.is_exported {
          self.writer.keyword(KWD_EXPORT);
          if let Some(cst) = cst_node {
            self.advance(cst.declaration_keyword_position);
          }
        }
        self
          .writer
          .keyword(if a.is_const { "const" } else { "local" });
        let mut var_comma = CommaSeparatorInserter::new(
          cst_node.map_or(EMPTY_POSITIONS, |cst| cst.vars_comma_positions.as_slice()),
        );
        self.visualize_local_vars(
          &a.vars,
          &mut var_comma,
          cst_node.map(|cst| cst.vars_annotation_colon_positions.as_slice()),
        );
        if let Some(loc) = a.equals_sign_location {
          self.advance(loc.begin);
          self.writer.symbol(SYM_ASSIGN);
        }
        let mut value_comma = CommaSeparatorInserter::new(
          cst_node.map_or(EMPTY_POSITIONS, |cst| cst.values_comma_positions.as_slice()),
        );
        for value in AstArray::iter(&a.values) {
          value_comma.write(self.writer);
          self.visualize_ast_expr(value);
        }
      }
      AstStatFor::CLASS_INDEX => {
        let a = as_node::<AstStatFor>(node);
        let cst_node = self.lookup_cst_node::<CstStatFor>(node);
        self.writer.keyword("for");
        self.visualize_ast_local_position(
          a.var.get(),
          cst_node.map_or_else(Position::missing, |cst| cst.annotation_colon_position),
        );
        if let Some(cst) = cst_node {
          self.advance(cst.equals_position);
        }
        self.writer.symbol(SYM_ASSIGN);
        self.visualize_ast_expr(a.from);
        self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.end_comma_position), ",");
        self.visualize_ast_expr(a.to);
        // step 落可空 OptNode（for 无步长）：`get()` 即 Option 视图，Some 侧
        // 共享借用只读打印。
        if let Some(step) = a.step.get() {
          if let Some(cst) = cst_node {
            self.advance(cst.step_comma_position);
          }
          self.writer.symbol(",");
          self.visualize_ast_expr(step);
        }
        self.visualize_do_block(a.do_location, a.body.as_ptr());
      }
      AstStatForIn::CLASS_INDEX => {
        let a = as_node::<AstStatForIn>(node);
        let cst_node = self.lookup_cst_node::<CstStatForIn>(node);
        self.writer.keyword("for");
        let mut var_comma = CommaSeparatorInserter::new(
          cst_node.map_or(EMPTY_POSITIONS, |cst| cst.vars_comma_positions.as_slice()),
        );
        self.visualize_local_vars(
          &a.vars,
          &mut var_comma,
          cst_node.map(|cst| cst.vars_annotation_colon_positions.as_slice()),
        );
        self.advance(a.in_location.begin);
        self.writer.keyword("in");
        let mut val_comma = CommaSeparatorInserter::new(
          cst_node.map_or(EMPTY_POSITIONS, |cst| cst.values_comma_positions.as_slice()),
        );
        for val in AstArray::iter(&a.values) {
          val_comma.write(self.writer);
          self.visualize_ast_expr(val);
        }
        self.advance(a.do_location.begin);
        self.writer.keyword(KWD_DO);
        // body 指向 arena 存活的 AstStatBlock；end 为 Copy 值，提前读取与末尾读取等价
        let body_end = a.body.get().base.base.location.end;
        self.visualize_block_ast_stat_block(a.body);
        self.advance(body_end);
        self.writer.keyword(KWD_END);
      }
      AstStatAssign::CLASS_INDEX => {
        let a = as_node::<AstStatAssign>(node);
        let cst_node = self.lookup_cst_node::<CstStatAssign>(node);
        let mut var_comma = CommaSeparatorInserter::new(
          cst_node.map_or(EMPTY_POSITIONS, |cst| cst.vars_comma_positions.as_slice()),
        );
        for var in AstArray::iter(&a.vars) {
          var_comma.write(self.writer);
          self.visualize_ast_expr(var);
        }
        match cst_node {
          Some(cst) => self.maybe_advance_and_write(&cst.equals_position, SYM_ASSIGN, false),
          None => {
            self.writer.space();
            self.writer.symbol(SYM_ASSIGN);
          }
        }
        let mut value_comma = CommaSeparatorInserter::new(
          cst_node.map_or(EMPTY_POSITIONS, |cst| cst.values_comma_positions.as_slice()),
        );
        for value in AstArray::iter(&a.values) {
          value_comma.write(self.writer);
          self.visualize_ast_expr(value);
        }
      }
      AstStatCompoundAssign::CLASS_INDEX => {
        let a = as_node::<AstStatCompoundAssign>(node);
        let cst_node = self.lookup_cst_node::<CstStatCompoundAssign>(node);
        self.visualize_ast_expr(a.var);
        if let Some(cst) = cst_node {
          self.advance(cst.op_position);
        }
        let symbol = Self::compound_assign_symbol(a.op);
        // 操作符宽度决定与右操作数的间距（cpp 每分支显式 2/3，symbol.len 同值）
        if !symbol.is_empty() {
          if cst_node.is_none() {
            self
              .writer
              .maybe_space(&a.value.get().base.location.begin, symbol.len() as i32);
          }
          self.writer.symbol(symbol);
        }
        self.visualize_ast_expr(a.value);
      }
      AstStatFunction::CLASS_INDEX => {
        let a = as_node::<AstStatFunction>(node);
        let func = a.func.get();
        for attr in func.attributes.iter_nodes() {
          self.visualize_attribute(attr);
        }
        if let Some(cst) = self.lookup_cst_node::<CstStatFunction>(node) {
          self.advance(cst.function_keyword_position);
        }
        self.writer.keyword(KWD_FUNCTION);
        self.visualize_ast_expr(a.name);
        self.visualize_function_body(func);
      }
      AstStatLocalFunction::CLASS_INDEX => {
        let a = as_node::<AstStatLocalFunction>(node);
        let func = a.func.get();
        for attr in func.attributes.iter_nodes() {
          self.visualize_attribute(attr);
        }
        let cst_node = self.lookup_cst_node::<CstStatLocalFunction>(node);
        if let Some(cst) = cst_node {
          self.advance(cst.local_keyword_position);
        }
        let name = a.name.get();
        if fflag::LuauExportValueSyntax.get() && name.is_exported {
          self.writer.keyword(KWD_EXPORT);
        } else if name.is_const {
          self.writer.keyword("const");
        } else {
          self.writer.keyword("local");
        }
        match cst_node {
          Some(cst) => self.advance(cst.function_keyword_position),
          None => self.writer.space(),
        }
        self.writer.keyword(KWD_FUNCTION);
        self.advance(name.location.begin);
        self.writer.identifier(name.name.as_bytes());
        self.visualize_function_body(func);
      }
      AstStatTypeAlias::CLASS_INDEX => {
        let a = as_node::<AstStatTypeAlias>(node);
        if self.write_types {
          let cst_node = self.lookup_cst_node::<CstStatTypeAlias>(node);
          if a.exported {
            self.writer.keyword(KWD_EXPORT);
          }
          if let Some(cst) = cst_node {
            self.advance(cst.type_keyword_position);
          }
          self.writer.keyword("type");
          self.advance(a.name_location.begin);
          self.writer.identifier(a.name.as_bytes());
          if a.generics.size > 0 || a.generic_packs.size > 0 {
            if let Some(cst) = cst_node {
              self.advance(cst.generics_open_position);
            }
            self.writer.symbol(SYM_LT);
            let mut comma = CommaSeparatorInserter::new(cst_node.map_or(EMPTY_POSITIONS, |cst| {
              cst.generics_comma_positions.as_slice()
            }));
            for o in a.generics.iter_nodes() {
              comma.write(self.writer);
              self.writer.advance(&o.base.location.begin);
              self.writer.identifier(o.name.as_bytes());
              if let Some(default_value) = node_ref(o.default_value) {
                if let Some(cst) = self.lookup_cst_node::<CstGenericType>(&o.base) {
                  self.advance(cst.default_equals_position);
                } else {
                  self
                    .writer
                    .maybe_space(&default_value.base.location.begin, 2);
                }
                self.writer.symbol(SYM_ASSIGN);
                self.visualize_type_annotation(default_value);
              }
            }
            for o in a.generic_packs.iter_nodes() {
              comma.write(self.writer);
              let generic_type_pack_cst_node = self.lookup_cst_node::<CstGenericTypePack>(&o.base);
              self.writer.advance(&o.base.location.begin);
              self.writer.identifier(o.name.as_bytes());
              if let Some(cst) = generic_type_pack_cst_node {
                self.maybe_advance_and_write(&cst.ellipsis_position, SYM_ELLIPSIS, false);
              } else {
                self.writer.symbol(SYM_ELLIPSIS);
              }
              if let Some(default_value) = node_ref(o.default_value) {
                if let Some(cst) = generic_type_pack_cst_node {
                  self.advance(cst.default_equals_position);
                } else {
                  self
                    .writer
                    .maybe_space(&default_value.base.location.begin, 2);
                }
                self.writer.symbol(SYM_ASSIGN);
                self.visualize_type_pack_annotation(default_value, false, true, false);
              }
            }
            match cst_node {
              Some(cst) => self.maybe_advance_and_write(&cst.generics_close_position, ">", false),
              None => self.writer.symbol(">"),
            }
          }
          match cst_node {
            Some(cst) => self.maybe_advance_and_write(&cst.equals_position, SYM_ASSIGN, false),
            None => {
              self
                .writer
                .maybe_space(&slot_ref(a.type_ptr).base.location.begin, 2);
              self.writer.symbol(SYM_ASSIGN);
            }
          }
          self.visualize_type_annotation(a.type_ptr);
        }
      }
      AstStatTypeFunction::CLASS_INDEX => {
        let t = as_node::<AstStatTypeFunction>(node);
        if self.write_types {
          let cst_node = self.lookup_cst_node::<CstStatTypeFunction>(node);
          if t.exported {
            self.writer.keyword(KWD_EXPORT);
          }
          match cst_node {
            Some(cst) => self.advance(cst.type_keyword_position),
            None => self.writer.space(),
          }
          self.writer.keyword("type");
          match cst_node {
            Some(cst) => self.advance(cst.function_keyword_position),
            None => self.writer.space(),
          }
          self.writer.keyword(KWD_FUNCTION);
          self.advance(t.name_location.begin);
          self.writer.identifier(t.name.as_bytes());
          self.visualize_function_body(t.body);
        }
      }
      AstStatError::CLASS_INDEX => {
        let a = as_node::<AstStatError>(node);
        self.writer.symbol("(error-stat");
        let no_statements = a.statements.is_empty();
        for (i, &expression) in a.expressions.iter().enumerate() {
          self
            .writer
            .symbol(if i == 0 && no_statements { ": " } else { ", " });
          self.visualize_ast_expr(expression);
        }
        let no_expressions = a.expressions.is_empty();
        for (i, &statement) in a.statements.iter().enumerate() {
          self
            .writer
            .symbol(if i == 0 && no_expressions { ": " } else { ", " });
          self.visualize_ast_stat(statement);
        }
        self.writer.symbol(SYM_RPAREN);
      }
      AstStatDeclareGlobal::CLASS_INDEX => {
        let a = as_node::<AstStatDeclareGlobal>(node);
        self.writer.keyword("declare");
        self.advance(a.name_location.begin);
        self.writer.identifier(a.name.as_bytes());
        self.writer.symbol(SYM_COLON);
        self.visualize_type_annotation(a.type_);
      }
      AstStatClass::CLASS_INDEX => {
        let c = as_node::<AstStatClass>(node);
        if fflag::DebugLuauUserDefinedClasses.get() {
          self.writer.keyword("class");
          let name = slot_ref(c.name);
          self.advance(name.location.begin);
          self.writer.identifier(name.name.as_bytes());
          if let Some(super_) = slot_opt(c.super_) {
            self.writer.keyword("extends");
            self.visualize_ast_expr(super_);
          }
          for member in AstArray::iter(&c.members) {
            match member {
              Variant2::V0(prop) => {
                let prop: &AstClassProperty = prop;
                self.advance(prop.qualifier_location.begin);
                self.writer.keyword("public");
                self.advance(prop.name_location.begin);
                self.writer.identifier(prop.name.as_bytes());
                if self.write_types
                  && let Some(prop_type) = slot_opt(prop.ty)
                {
                  LUAU_ASSERT!(prop.type_colon_location.is_some());
                  if let Some(colon) = prop.type_colon_location {
                    self.advance(colon.begin);
                  }
                  self.writer.symbol(SYM_COLON);
                  self.visualize_type_annotation(prop_type);
                }
              }
              Variant2::V1(method) => {
                let method: &AstClassMethod = method;
                if let Some(qualifier_location) = method.qualifier_location {
                  self.advance(qualifier_location.begin);
                  self.writer.keyword("public");
                }
                self.advance(method.keyword_location.begin);
                self.writer.keyword(KWD_FUNCTION);
                self.advance(method.name_location.begin);
                self.writer.identifier(method.function_name.as_bytes());
                self.visualize_function_body(method.function);
              }
            }
          }
          self.writer.newline();
          self.writer.keyword(KWD_END);
          self.writer.newline();
        }
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }

    if program.has_semicolon {
      self.advance_before(program.base.location.end, 1);
      self.writer.symbol(";");
    }
  }

  /// `do <block> end` 收尾：while / for / for-in 三处共用（cpp 在三个分支里各
  /// 抄了一遍同一段落），解引用也从 3 处收口到 1 处。
  fn visualize_do_block(&mut self, do_location: Location, body: *mut AstStatBlock) {
    self.advance(do_location.begin);
    self.writer.keyword(KWD_DO);
    // body 指向 arena 存活的 AstStatBlock；end 为 Copy 值，visualize
    // 只写 writer，提前读取与末尾读取等价
    let body_end = slot_ref(body).base.base.location.end;
    self.visualize_block_ast_stat_block(body);
    self.advance(body_end);
    self.writer.keyword(KWD_END);
  }

  /// 遍历局部变量列表，逐个写逗号分隔符与本地位置；`colon_positions` 为
  /// CST 提供的冒号位置（无 CST 时用 `Position::missing()`，cpp 同款
  /// `Position{0, 0}` 分支此处以 missing 收口）。
  fn visualize_local_vars(
    &mut self,
    vars: &AstArray<*mut AstLocal>,
    var_comma: &mut CommaSeparatorInserter,
    colon_positions: Option<&[Position]>,
  ) {
    for (i, var) in vars.iter_nodes().enumerate() {
      var_comma.write(self.writer);
      // vars 与 colon_positions 成对构造（解析器保证等长，cpp 同处有断言）；
      // 越界仅解析器 bug，退化 missing（cpp 侧 `data[i]` 同样依赖该不变式）。
      let colon_position = colon_positions
        .and_then(|c| c.get(i))
        .copied()
        .unwrap_or_else(Position::missing);
      self.visualize_ast_local_position(var, colon_position);
    }
  }

  /// cpp `Printer::visit(AstAttr*)` 的打印：节点只读，写入只发生在 `Writer`。
  pub fn visualize_attribute(&mut self, attribute: &AstAttr) {
    self.advance(attribute.base.location.begin);
    self.writer.symbol("@");
    self.writer.identifier(attribute.name.as_bytes());
  }

  pub fn visualize_block_ast_stat_block<B: IntoNodePtr<AstStatBlock>>(&mut self, block: B) {
    // block 指向 arena 中存活的 AstStatBlock
    let block = slot_ref(block.into_node_ptr());
    // body 已句柄化（Nodes）：元素非空由 Node 构造端证明，直接共享借用递归打印。
    for stat in block.body.iter() {
      self.visualize_ast_stat(stat);
    }
    self.advance(block.base.base.location.end);
  }

  pub fn visualize_block_ast_stat<S: IntoNodePtr<AstStat>>(&mut self, stat: S) {
    // stat 指向 arena 中存活的 AstStat 派生节点；class_index 匹配后
    // #[repr(C)] 单继承布局保证下转有效（与 ast_node_try_as 同一收口）。
    let stat = slot_ref(stat.into_node_ptr());
    if let Some(block_ref) = ast_node_try_as::<AstStatBlock>(&stat.base) {
      self.visualize_block_ast_stat_block(block_ref);
      return;
    }

    ulua_common::LUAU_ASSERT!(false);
  }

  /// cpp `Printer::visit(AstStatIf* elseif)` 的 else-if 分支：打印器只写
  /// `Writer`，节点全程共享借用。
  pub fn visualize_else_if(&mut self, elseif: &AstStatIf) {
    // `condition`/`thenbody` 已句柄化：`get` 交出的共享引用即 parser 的
    // 非空 + 存活证明，打印器全程只读遍历。
    let condition = elseif.condition.get();
    let thenbody = elseif.thenbody.get();

    self.visualize_ast_expr(condition);

    if let Some(ref loc) = elseif.then_location {
      self.advance(loc.begin);
    }

    self.writer.keyword("then");

    self.visualize_block_ast_stat_block(thenbody);

    // `elsebody` 可空（无 else 时为空槽）：句柄 `get` 直接折叠为 Option，全程只读打印。
    let elsebody = elseif.elsebody.get();
    match elsebody {
      None => {
        self.advance(thenbody.base.base.location.end);
        self.writer.keyword(KWD_END);
      }
      Some(elsebody) => {
        if let Some(elseifelseif) = ast_node_try_as::<AstStatIf>(&elsebody.base) {
          if let Some(ref loc) = elseif.else_location {
            self.advance(loc.begin);
          }
          self.writer.keyword("elseif");
          self.visualize_else_if(elseifelseif);
        } else {
          if let Some(ref loc) = elseif.else_location {
            self.advance(loc.begin);
          }
          self.writer.keyword("else");

          self.visualize_block_ast_stat(elsebody);
          self.advance(elsebody.base.location.end);
          self.writer.keyword(KWD_END);
        }
      }
    }
  }

  /// cpp `Printer::visit(AstExprIfElse* elseif)`：elseif 链递归展开时子节点
  /// 同样按共享借用读取（打印器只写 `Writer`）。
  pub fn visualize_else_if_expr(&mut self, elseif: &AstExprIfElse) {
    let cst_node = self.lookup_cst_node::<CstExprIfElse>(&elseif.base.base);

    // condition/true_expr/false_expr 已句柄化（node_handle）：只读遍历经 .get() 安全借用。
    let condition = elseif.condition.get();
    self.visualize_ast_expr(condition);

    if let Some(cst_node) = cst_node {
      self.maybe_advance_and_write(&cst_node.then_position, "then", false);
    } else {
      self.writer.keyword("then");
    }

    let true_expr = elseif.true_expr.get();
    self.visualize_ast_expr(true_expr);

    if elseif.has_else {
      if let Some(cst_node) = cst_node {
        self.advance(cst_node.else_position);
      }

      // elseif 链：else 分支本身仍是 IfElse 节点时递归展开（CST 以 is_else_if 标记）；
      // false_expr 已句柄化恒非空，null 折叠守卫随类型消失（等价 cpp 无判空形态）。
      let elseifelseif = ast_node_try_as::<AstExprIfElse>(&elseif.false_expr.get().base)
        .filter(|_| cst_node.is_none_or(|cst| cst.is_else_if));
      if let Some(elseifelseif) = elseifelseif {
        self.writer.keyword("elseif");
        self.visualize_else_if_expr(elseifelseif);
        return;
      }

      self.writer.keyword("else");
      self.visualize_ast_expr(elseif.false_expr.get());
    }
  }

  pub(crate) fn visualize_explicit_type_instantiation(
    &mut self,
    type_arguments: AstArray<AstTypeOrPack>,
    cst_node: Option<&CstTypeInstantiation>,
  ) {
    // C<<T>>：两个连续 "<" 与 ">"，位置可各自被 CST 校准
    for (open, close) in [
      (cst_node.map(|cst| &cst.left_arrow_1_position), SYM_LT),
      (cst_node.map(|cst| &cst.left_arrow_2_position), SYM_LT),
    ] {
      match open {
        Some(pos) => self.maybe_advance_and_write(pos, close, false),
        None => self.writer.symbol(close),
      }
    }

    let mut comma = CommaSeparatorInserter::new(
      cst_node.map_or(EMPTY_POSITIONS, |cst| cst.comma_positions.as_slice()),
    );

    for type_or_pack in type_arguments.as_slice() {
      comma.write(self.writer);

      // 槽位形态由 `AstTypeOrPack` 的变体直接给出，无需再判空；两侧皆空的错误态在
      // cpp 会解引用 null `typePack`，这里与迁移后的其它消费点一致仅断言。
      match *type_or_pack {
        AstTypeOrPack::Type(t) => self.visualize_type_annotation(t),
        AstTypeOrPack::Pack(pack) => {
          self.visualize_type_pack_annotation(pack, false, true, false);
        }
        AstTypeOrPack::Error => LUAU_ASSERT!(false),
      }
    }

    for (close, sym) in [
      (cst_node.map(|cst| &cst.right_arrow_1_position), ">"),
      (cst_node.map(|cst| &cst.right_arrow_2_position), ">"),
    ] {
      match close {
        Some(pos) => self.maybe_advance_and_write(pos, sym, false),
        None => self.writer.symbol(sym),
      }
    }
  }

  pub fn visualize_function_body<F: IntoNodePtr<AstExprFunction>>(&mut self, func: F) {
    let func = slot_ref(func.into_node_ptr());
    let cst_node = self.lookup_cst_node::<CstExprFunction>(&func.base.base);

    if !func.generics.is_empty() || !func.generic_packs.is_empty() {
      let mut comma = CommaSeparatorInserter::new(
        cst_node.map_or(&[], |cst| cst.generics_comma_positions.as_slice()),
      );

      self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.open_generics_position), SYM_LT);

      for generic_ty in func.generics.iter_nodes() {
        comma.write(self.writer);
        self.writer.advance(&generic_ty.base.location.begin);
        self.writer.identifier(generic_ty.name.as_bytes());
      }

      for pack in func.generic_packs.iter_nodes() {
        comma.write(self.writer);
        self.writer.advance(&pack.base.location.begin);
        self.writer.identifier(pack.name.as_bytes());

        if let Some(cst) = self.lookup_cst_node::<CstGenericTypePack>(&pack.base) {
          self.advance(cst.ellipsis_position);
        }

        self.writer.symbol(SYM_ELLIPSIS);
      }

      self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.close_generics_position), ">");
    }

    if let Some(arg_location) = func.arg_location.as_ref() {
      self.advance(arg_location.begin);
    }
    self.writer.symbol(SYM_LPAREN);

    let mut comma =
      CommaSeparatorInserter::new(cst_node.map_or(&[], |cst| cst.args_comma_positions.as_slice()));
    // CST 冒号位置与 args 成对构造（解析器保证等长）；越界仅解析器 bug，
    // 退化直接写 ":"。
    let colon_positions = cst_node.map(|cst| cst.args_annotation_colon_positions.as_slice());

    for (i, local) in func.args.iter_nodes().enumerate() {
      comma.write(self.writer);
      self.advance(local.location.begin);
      self.writer.identifier(local.name.as_bytes());

      // annotation 可空（无注解）：`slot_opt` 折叠为 None，Some 侧只读打印。
      if self.write_types
        && let Some(annotation) = slot_opt(local.annotation)
      {
        match colon_positions.and_then(|c| c.get(i)) {
          Some(colon) => self.maybe_advance_and_write(colon, SYM_COLON, false),
          None => self.writer.symbol(SYM_COLON),
        }

        self.visualize_type_annotation(annotation);
      }
    }

    if func.vararg {
      comma.write(self.writer);

      self.advance(func.vararg_location.begin);
      self.writer.symbol(SYM_ELLIPSIS);

      if self.write_types
        && let Some(vararg_annotation) = func.vararg_annotation.get()
      {
        self.maybe_advance_or_symbol(
          cst_node.map(|cst| &cst.vararg_annotation_colon_position),
          SYM_COLON,
        );

        self.visualize_type_pack_annotation(vararg_annotation, true, true, false);
      }
    }

    if let Some(arg_location) = func.arg_location.as_ref() {
      self.advance_before(arg_location.end, 1);
    }
    self.writer.symbol(SYM_RPAREN);

    // return_annotation 可空：`slot_opt` 折叠为 None。
    if self.write_types
      && let Some(return_annotation) = func.return_annotation.get()
    {
      self.maybe_advance_or_symbol(
        cst_node.map(|cst| &cst.return_specifier_position),
        SYM_COLON,
      );

      if cst_node.is_none() {
        self.writer.space();
      }

      self.visualize_type_pack_annotation(return_annotation, false, false, true);
    }

    let body = func.body.get();
    self.visualize_block_ast_stat_block(body);
    self.advance(body.base.base.location.end);
    self.writer.keyword(KWD_END);
  }

  pub fn visualize_named_type_list(
    &mut self,
    list: &AstTypeList,
    unconditionally_parenthesize: bool,
    open_parentheses_position: Position,
    close_parentheses_position: Position,
    comma_positions: &[Position],
    arg_names: &[Option<AstArgumentName>],
    arg_names_colon_positions: &[Position],
  ) {
    let type_count = list.types.len() + usize::from(node_opt(list.tail_type).is_some());

    if type_count == 0 {
      self.maybe_advance_and_write(
        &open_parentheses_position,
        SYM_LPAREN,
        unconditionally_parenthesize,
      );
      self.maybe_advance_and_write(
        &close_parentheses_position,
        SYM_RPAREN,
        unconditionally_parenthesize,
      );
    } else if type_count == 1 {
      // Safety: 首元素指针指向 arena 存活的 AstType 节点；class_index 匹配后
      // 经安全 try_as 下转。首元素仅在 types 非空时解引用（cpp `data[0]`
      // 同样以 size==0 短路保护）。
      let first_type = list.types.as_slice().first().map(|&p| slot_ref(p));

      let should_parenthesize = unconditionally_parenthesize
        && first_type.is_none_or(|t| ast_node_try_as::<AstTypeGroup>(&t.base).is_none());

      self.maybe_advance_and_write(&open_parentheses_position, SYM_LPAREN, should_parenthesize);

      let mut arg_name = ArgNameInserter::new(arg_names, arg_names_colon_positions);
      arg_name.write(self.writer);

      match first_type {
        Some(t) => self.visualize_type_annotation(t),
        None => {
          // types 空：仅 variadic tail（cpp 同款）
          // Safety: tail_type 指向 arena 存活的 AstTypePack（非空已判）
          self.visualize_type_pack_annotation(list.tail_type, false, true, false);
        }
      }

      self.maybe_advance_and_write(&close_parentheses_position, SYM_RPAREN, should_parenthesize);
    } else {
      self.maybe_advance_and_write(
        &open_parentheses_position,
        SYM_LPAREN,
        unconditionally_parenthesize,
      );

      // 逗号与参数名游标跨调用持有（cpp 同款；不持有 writer，无借用冲突）。
      let mut comma = CommaSeparatorInserter::new(comma_positions);
      let mut arg_name = ArgNameInserter::new(arg_names, arg_names_colon_positions);

      for t in list.types.iter_nodes() {
        comma.write(self.writer);
        arg_name.write(self.writer);
        self.visualize_type_annotation(t);
      }

      // Safety: tail_type 为 arena 存活 AstTypePack 或 null（无尾部 pack）；as_ref 判空
      // 折叠为 None，Some 侧共享借用只读打印。
      if let Some(tail_type) = slot_opt(list.tail_type) {
        comma.write(self.writer);
        self.visualize_type_pack_annotation(tail_type, false, true, false);
      }

      self.maybe_advance_and_write(
        &close_parentheses_position,
        SYM_RPAREN,
        unconditionally_parenthesize,
      );
    }
  }

  pub fn visualize_type_annotation<T: IntoNodePtr<AstType>>(&mut self, type_annotation: T) {
    // type_annotation 指向 arena 中存活的 AstType 派生节点
    let type_annotation = slot_ref(type_annotation.into_node_ptr());
    self.advance(type_annotation.base.location.begin);

    match type_annotation.base.class_index {
      AstTypeReference::CLASS_INDEX => {
        let a = as_node::<AstTypeReference>(&type_annotation.base);
        let cst_node = self.lookup_cst_node::<CstTypeReference>(&a.base.base);

        if let Some(prefix) = a.prefix {
          self.writer.write(prefix.as_bytes());
          if let Some(cst) = cst_node {
            self.advance(cst.prefix_point_position);
          }
          self.writer.symbol(".");
        }

        self.advance(a.name_location.begin);
        self.writer.write(a.name.as_bytes());

        if a.parameters.size > 0 || a.has_parameter_list {
          let mut comma = CommaSeparatorInserter::new(cst_node.map_or(EMPTY_POSITIONS, |cst| {
            cst.parameters_comma_positions.as_slice()
          }));

          if let Some(cst) = cst_node {
            self.advance(cst.open_parameters_position);
          }
          self.writer.symbol(SYM_LT);

          for o in AstArray::iter(&a.parameters) {
            comma.write(self.writer);

            match *o {
              AstTypeOrPack::Type(param_type) => self.visualize_type_annotation(param_type),
              AstTypeOrPack::Pack(param_pack) => {
                self.visualize_type_pack_annotation(param_pack, false, true, false);
              }
              AstTypeOrPack::Error => LUAU_ASSERT!(false),
            }
          }

          self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.close_parameters_position), ">");
        }
      }
      AstTypeFunction::CLASS_INDEX => {
        let a = as_node::<AstTypeFunction>(&type_annotation.base);
        let cst_node = self.lookup_cst_node::<CstTypeFunction>(&a.base.base);

        if a.generics.size > 0 || a.generic_packs.size > 0 {
          let mut comma = CommaSeparatorInserter::new(cst_node.map_or(EMPTY_POSITIONS, |cst| {
            cst.generics_comma_positions.as_slice()
          }));

          if let Some(cst) = cst_node {
            self.advance(cst.open_generics_position);
          }
          self.writer.symbol(SYM_LT);

          for o in a.generics.iter_nodes() {
            comma.write(self.writer);
            self.writer.advance(&o.base.location.begin);
            self.writer.identifier(o.name.as_bytes());
          }

          for o in a.generic_packs.iter_nodes() {
            comma.write(self.writer);
            self.writer.advance(&o.base.location.begin);
            self.writer.identifier(o.name.as_bytes());

            if let Some(cst) = self.lookup_cst_node::<CstGenericTypePack>(&o.base) {
              self.advance(cst.ellipsis_position);
            }
            self.writer.symbol(SYM_ELLIPSIS);
          }

          self.maybe_advance_or_symbol(cst_node.map(|cst| &cst.close_generics_position), ">");
        }

        let (open_args, close_args) = match cst_node {
          Some(cst) => (cst.open_args_position, cst.close_args_position),
          None => (Position::missing(), Position::missing()),
        };
        let comma_pos = cst_node.map_or(EMPTY_POSITIONS, |cst| {
          cst.arguments_comma_positions.as_slice()
        });
        let colon_pos = cst_node.map_or(EMPTY_POSITIONS, |cst| {
          cst.argument_name_colon_positions.as_slice()
        });

        self.visualize_named_type_list(
          &a.arg_types,
          cst_node.is_none(),
          open_args,
          close_args,
          comma_pos,
          a.arg_names.as_slice(),
          colon_pos,
        );

        if let Some(cst) = cst_node {
          self.advance(cst.return_arrow_position);
        }
        self.writer.symbol("->");

        self.visualize_type_pack_annotation(a.return_types, false, cst_node.is_none(), false);
      }
      AstTypeTable::CLASS_INDEX => {
        let a = as_node::<AstTypeTable>(&type_annotation.base);
        let indexer = slot_opt(a.indexer);
        let index_type = indexer
          .and_then(|idx| slot_opt(idx.index_type))
          .and_then(|t| ast_node_try_as::<AstTypeReference>(&t.base));

        self.writer.symbol(SYM_LBRACE);

        let cst_node = self.lookup_cst_node::<CstTypeTable>(&a.base.base);
        match cst_node {
          Some(cst) if cst.is_array => {
            LUAU_ASSERT!(a.props.size == 0 && index_type.is_some_and(|t| t.name == NUMBER_KEY));
            if let Some(idx) = indexer {
              self.write_table_access(idx.access, idx.access_location);
              self.visualize_type_annotation(idx.result_type);
            }
          }
          Some(cst) => {
            let mut props = a.props.as_slice().iter();

            for item in cst.items.as_slice() {
              if item.kind == Indexer {
                LUAU_ASSERT!(indexer.is_some());
                if let Some(idx) = indexer {
                  self.write_table_access(idx.access, idx.access_location);
                  self.advance(item.indexer_open_position);
                  self.writer.symbol(SYM_LBRACKET);
                  self.visualize_type_annotation(idx.index_type);
                  self.maybe_advance_and_write(&item.indexer_close_position, "]", false);
                  self.maybe_advance_and_write(&item.colon_position, SYM_COLON, false);
                  self.visualize_type_annotation(idx.result_type);
                }
              } else {
                let Some(prop) = props.next() else { continue };
                self.write_table_access(prop.access, prop.access_location);

                if item.kind == StringProperty {
                  if item.indexer_open_position.has_value() {
                    self.maybe_advance_and_write(&item.indexer_open_position, SYM_LBRACKET, false);
                  }
                  self.advance(item.string_position);
                  let info = slot_ref(item.string_info);
                  self.writer.source_string(
                    info.source_string.as_bytes(),
                    info.quote_style,
                    info.block_depth,
                  );
                  if item.indexer_close_position.has_value() {
                    self.maybe_advance_and_write(&item.indexer_close_position, "]", false);
                  }
                } else {
                  self.advance(prop.location.begin);
                  self.writer.identifier(prop.name.as_bytes());
                }

                self.maybe_advance_and_write(&item.colon_position, SYM_COLON, false);
                self.visualize_type_annotation(prop.r#type);
              }

              if item.separator != Missing {
                LUAU_ASSERT!(item.separator_position.has_value());
                self.maybe_advance_and_write(
                  &item.separator_position,
                  if item.separator == Comma { "," } else { ";" },
                  true,
                );
              }
            }
          }
          None => {
            if a.props.size == 0 && index_type.is_some_and(|t| t.name == NUMBER_KEY) {
              if let Some(idx) = indexer {
                self.visualize_type_annotation(idx.result_type);
              }
            } else {
              let mut comma = CommaSeparatorInserter::new(EMPTY_POSITIONS);

              for prop in AstArray::iter(&a.props) {
                comma.write(self.writer);

                self.advance(prop.location.begin);
                self.writer.identifier(prop.name.as_bytes());
                if let Some(prop_type) = slot_opt(prop.r#type) {
                  self.writer.symbol(SYM_COLON);
                  self.visualize_type_annotation(prop_type);
                }
              }
              if let Some(idx) = indexer {
                comma.write(self.writer);

                self.writer.symbol(SYM_LBRACKET);
                self.visualize_type_annotation(idx.index_type);
                self.writer.symbol("]");
                self.writer.symbol(SYM_COLON);
                self.visualize_type_annotation(idx.result_type);
              }
            }
          }
        }

        let mut end_pos = type_annotation.base.location.end;
        if end_pos.column > 0 {
          end_pos.column -= 1;
        }
        self.advance(end_pos);
        self.writer.symbol(SYM_RBRACE);
      }
      AstTypeTypeof::CLASS_INDEX => {
        let a = as_node::<AstTypeTypeof>(&type_annotation.base);
        self.writer.keyword("typeof");
        let cst_node = self.lookup_cst_node::<CstTypeTypeof>(&a.base.base);
        match cst_node {
          Some(cst) => {
            self.maybe_advance_and_write(&cst.open_position, SYM_LPAREN, false);
            self.visualize_ast_expr(a.expr);
            self.maybe_advance_and_write(&cst.close_position, SYM_RPAREN, false);
          }
          None => {
            self.writer.symbol(SYM_LPAREN);
            self.visualize_ast_expr(a.expr);
            self.writer.symbol(SYM_RPAREN);
          }
        }
      }
      AstTypeUnion::CLASS_INDEX => {
        let a = as_node::<AstTypeUnion>(&type_annotation.base);
        let cst_node = self.lookup_cst_node::<CstTypeUnion>(&a.base.base);

        if cst_node.is_none()
          && let [l_ptr, r_ptr] = a.types.as_slice()
        {
          let mut l = *l_ptr;
          let mut r = *r_ptr;

          let l_is_nil =
            ast_node_try_as::<AstTypeReference>(&slot_ref(l).base).is_some_and(|t| t.name == "nil");
          let r_is_optional = ast_node_is::<AstTypeOptional>(&slot_ref(r).base);
          if l_is_nil && !r_is_optional {
            swap(&mut l, &mut r);
          }

          let r_is_nil =
            ast_node_try_as::<AstTypeReference>(&slot_ref(r).base).is_some_and(|t| t.name == "nil");
          if r_is_nil {
            let wrap = ast_node_is::<AstTypeIntersection>(&slot_ref(l).base)
              || ast_node_is::<AstTypeFunction>(&slot_ref(l).base);
            if wrap {
              self.writer.symbol(SYM_LPAREN);
            }
            self.visualize_type_annotation(l);
            if wrap {
              self.writer.symbol(SYM_RPAREN);
            }
            self.writer.symbol("?");
            return;
          }
        }

        if let Some(cst) = cst_node {
          self.maybe_advance_and_write(&cst.leading_position, "|", false);
        }

        let mut seps = cst_node.map(|cst| cst.separator_positions.as_slice().iter());

        for (i, t) in a.types.iter_nodes().enumerate() {
          if let Some(optional) = ast_node_try_as::<AstTypeOptional>(&t.base) {
            self.advance(optional.base.base.location.begin);
            self.writer.symbol("?");
            continue;
          }

          if i > 0 {
            match seps.as_mut().and_then(Iterator::next) {
              Some(pos) => self.advance(*pos),
              None => self.writer.maybe_space(&t.base.location.begin, 2),
            }
            self.writer.symbol("|");
          }

          let wrap = cst_node.is_none()
            && (ast_node_is::<AstTypeIntersection>(&t.base)
              || ast_node_is::<AstTypeFunction>(&t.base));
          if wrap {
            self.writer.symbol(SYM_LPAREN);
          }
          self.visualize_type_annotation(t);
          if wrap {
            self.writer.symbol(SYM_RPAREN);
          }
        }
      }
      AstTypeIntersection::CLASS_INDEX => {
        let a = as_node::<AstTypeIntersection>(&type_annotation.base);
        let cst_node = self.lookup_cst_node::<CstTypeIntersection>(&a.base.base);

        if let Some(cst) = cst_node {
          self.maybe_advance_and_write(&cst.leading_position, "&", false);
        }

        let mut seps = cst_node.map(|cst| cst.separator_positions.as_slice().iter());

        for (i, t) in a.types.iter_nodes().enumerate() {
          if i > 0 {
            match seps.as_mut().and_then(Iterator::next) {
              Some(pos) => self.advance(*pos),
              None => self.writer.maybe_space(&t.base.location.begin, 2),
            }
            self.writer.symbol("&");
          }

          let wrap = cst_node.is_none()
            && (ast_node_is::<AstTypeUnion>(&t.base) || ast_node_is::<AstTypeFunction>(&t.base));
          if wrap {
            self.writer.symbol(SYM_LPAREN);
          }
          self.visualize_type_annotation(t);
          if wrap {
            self.writer.symbol(SYM_RPAREN);
          }
        }
      }
      AstTypeGroup::CLASS_INDEX => {
        let a = as_node::<AstTypeGroup>(&type_annotation.base);
        self.writer.symbol(SYM_LPAREN);
        self.visualize_type_annotation(a.type_);

        let cst_node = self.lookup_cst_node::<CstTypeGroup>(&a.base.base);
        match cst_node {
          Some(cst) => self.maybe_advance_and_write(&cst.close_position, SYM_RPAREN, false),
          None => {
            self.advance_before(type_annotation.base.location.end, 1);
            self.writer.symbol(SYM_RPAREN);
          }
        }
      }
      AstTypeSingletonBool::CLASS_INDEX => {
        let a = as_node::<AstTypeSingletonBool>(&type_annotation.base);
        self.writer.keyword(if a.value { "true" } else { "false" });
      }
      AstTypeSingletonString::CLASS_INDEX => {
        let a = as_node::<AstTypeSingletonString>(&type_annotation.base);
        match self.lookup_cst_node::<CstTypeSingletonString>(&a.base.base) {
          Some(cst) => {
            self.writer.source_string(
              cst.source_string.as_bytes(),
              cst.quote_style,
              cst.block_depth,
            );
          }
          None => {
            self.writer.string(a.value.as_bytes());
          }
        }
      }
      AstTypeError::CLASS_INDEX => {
        self.writer.symbol("%error-type%");
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }

  pub fn visualize_type_list(
    &mut self,
    list: &AstTypeList,
    unconditionally_parenthesize: bool,
    open_parentheses_position: Position,
    close_parentheses_position: Position,
    comma_positions: &[Position],
  ) {
    self.visualize_named_type_list(
      list,
      unconditionally_parenthesize,
      open_parentheses_position,
      close_parentheses_position,
      comma_positions,
      &[],
      &[],
    );
  }

  pub fn visualize_type_pack_annotation<T: IntoNodePtr<AstTypePack>>(
    &mut self,
    annotation: T,
    for_var_arg: bool,
    unconditionally_parenthesize: bool,
    for_function_return: bool,
  ) {
    // Safety: annotation 指向 arena 中存活的 AstTypePack 派生节点
    let annotation = slot_ref(annotation.into_node_ptr());
    self.advance(annotation.base.location.begin);

    match annotation.base.class_index {
      AstTypePackVariadic::CLASS_INDEX => {
        let tp = as_node::<AstTypePackVariadic>(&annotation.base);
        if !for_var_arg {
          self.writer.symbol(SYM_ELLIPSIS);
        }
        self.visualize_type_annotation(tp.variadic_type);
      }
      AstTypePackGeneric::CLASS_INDEX => {
        let tp = as_node::<AstTypePackGeneric>(&annotation.base);
        self.writer.symbol(tp.generic_name.as_str_or_empty());

        if let Some(cst_node) = self.lookup_cst_node::<CstTypePackGeneric>(&annotation.base) {
          self.advance(cst_node.ellipsis_position);
        }

        self.writer.symbol(SYM_ELLIPSIS);
      }
      AstTypePackExplicit::CLASS_INDEX => {
        let tp = as_node::<AstTypePackExplicit>(&annotation.base);
        LUAU_ASSERT!(!for_var_arg);

        if let Some(cst_node) = self.lookup_cst_node::<CstTypePackExplicit>(&annotation.base) {
          self.visualize_type_list(
            &tp.type_list,
            false,
            cst_node.open_parentheses_position,
            cst_node.close_parentheses_position,
            cst_node.comma_positions.as_slice(),
          );
          return;
        }

        if for_function_return {
          let pack_size =
            tp.type_list.types.size + usize::from(node_opt(tp.type_list.tail_type).is_some());

          self.visualize_type_list(
            &tp.type_list,
            pack_size != 1,
            Position::missing(),
            Position::missing(),
            &[],
          );
          return;
        }

        self.visualize_type_list(
          &tp.type_list,
          unconditionally_parenthesize,
          Position::missing(),
          Position::missing(),
          &[],
        );
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }
}
