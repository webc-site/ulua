//! AST visitor dispatch — the faithful Rust form of Luau's virtual
//! `AstNode::visit(AstVisitor*)` traversal (`Ast/src/Ast.cpp`).
//!
//! In C++ each concrete node overrides `visit(AstVisitor*)`: it calls the typed
//! `visitor->visit(this)` (compile-time overload on `this`'s static type) and,
//! if that returns `true`, recurses into its children with `child->visit(v)` —
//! a *virtual* call dispatched on the child's dynamic type.
//!
//! Rust has no vtable here (nodes are thin `*mut AstExpr` etc. in the arena), so
//! the per-node override becomes `impl AstVisitable for X`, and the virtual
//! recursion becomes a `class_index` match in the `*_visit` dispatch functions
//! below — the central analog of the C++ vtable. A node's `visit` body calls
//! `crate::visit::ast_expr_visit(self.child, v)` for each child pointer (and
//! loops over `AstArray` children), never `child.visit(v)` directly, because the
//! static type of `self.child` is only the base.
//!
//! 分发链路（类型安全化骨架）：`dispatch_node` 按 `CLASS_INDEX` 命中后构造
//! [`AstNodeRefMut`]（每变体持 `&mut` 具体类型），交给
//! `AstVisitor::visit_any`；其默认实现把枚举解包转交到逐级类型化的
//! `visit_xxx(&mut 具体类型)` hook 链（cpp `visit(X*)` 重载族的直接形态），
//! 返回 `true` 时回调节点的 [`AstVisitable::visit_children`] 遍历子节点。
//! 全链路无裸指针 hook：节点引用自 `&mut` 借用派生，类型即证据。

use crate::{
  records::{
    ast_attr::AstAttr, ast_expr::AstExpr, ast_expr_binary::AstExprBinary,
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_integer::AstExprConstantInteger, ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass, ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue, ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile, ast_type::AstType,
    ast_type_error::AstTypeError, ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
    ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
    ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion, ast_visitor::AstVisitor,
  },
  rtti::{AstNodeClass, AstNodePtr, AstNodeViewMut},
};

/// C++ `AstX::visit(AstVisitor*)` override. Implemented once per concrete node
/// by that node's `visit` method item.
///
/// 参数是 `&mut self` 而非 `&self`：cpp 的 `visit(AstVisitor*)` 拿到的是非 const
/// `this`，observer（如 Analysis 的 TypeAttacher）会按 cpp 语义在 dispatch 期间
/// 写穿节点本身。因此沿 arena 的可变性一路保持 `&mut`：调用方
/// （`dispatch_node`）从 arena 裸指针取得独占借用，节点再以 `&mut Self`
/// 交给 visitor——共享借用上造可变指针（`&self` 在 IR 里带 readonly/noalias）
/// 是优化器可见的别名 UB，类型化引用链路从根上杜绝该形态。
///
/// The visitor travels as generic `V: AstVisitor + ?Sized`: concrete visitors
/// monomorphize the whole recursion into static calls (no per-node vtable
/// round-trip), while `V = dyn AstVisitor` still compiles for callers that must
/// stay dynamically dispatched.
pub trait AstVisitable {
  /// cpp `X::visit(AstVisitor*)` override 的统一形态：类型化 hook →
  /// `true` 则遍历子节点。分发骨架固定于此，节点只提供
  /// [`Self::as_ref_mut`]（自报类型）与 [`Self::visit_children`]。
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_any(self.as_ref_mut()) {
      self.visit_children(visitor);
    }
  }

  /// 自报具体类型：把 `&mut self` 包进判别枚举（静态类型已知，直接构造
  /// 变体，零开销）。
  fn as_ref_mut(&mut self) -> AstNodeRefMut<'_>;

  /// 子节点遍历（cpp `visit` override 的后半段）。无子节点的节点用默认空实现。
  fn visit_children<V: AstVisitor + ?Sized>(&mut self, _visitor: &mut V) {}
}

/// 全节点分发表——本模块与 `records/ast_visitor.rs` 的单一事实来源。
///
/// 每行四元组：(`AstNodeRefMut` 判别变体, 具体节点类型, 类型化 hook 名,
/// 父 hook 名)。行序即 cpp `AstVisitor` 重载族的覆盖序，节点集合与 cpp
/// `Ast.cpp` 的具名 `visit` override 一一对应。四处消费方以回调宏共享此表，
/// 杜绝手抄漂移：
/// 1. [`define_ast_node_ref_mut`] —— 判别枚举本体；
/// 2. [`define_dispatch_node`] —— `dispatch_node` 的 class-index 跳转表；
/// 3. `records/ast_visitor.rs` 的 `ast_visitor_hooks` —— `visit_any` 转发与
///    逐级默认 hook（父 hook 名给出 cpp 的 `static_cast<Parent*>` 转发链）；
/// 4. `rtti.rs` 的 `define_ast_node_class` —— 全节点 [`crate::rtti::AstNodeClass`]
///    CLASS_INDEX 集（取代此前各节点文件的手写 impl）。
///
/// 标识符按 macro_rules 规则在展开点解析（路径不具卫生性），消费宏的模块
/// 需自行导入节点类型。表宏经 `lib.rs` 的 `#[macro_use] mod visit` 进入
/// crate 文本作用域，供 `records/ast_visitor.rs` 回调消费。
macro_rules! ast_node_table {
  ($cb:ident) => {
    $cb! {
      (Attr, AstAttr, visit_attr, visit_node),
      (ExprBinary, AstExprBinary, visit_expr_binary, visit_expr),
      (ExprCall, AstExprCall, visit_expr_call, visit_expr),
      (ExprConstantBool, AstExprConstantBool, visit_expr_constant_bool, visit_expr),
      (ExprConstantInteger, AstExprConstantInteger, visit_expr_constant_integer, visit_expr),
      (ExprConstantNil, AstExprConstantNil, visit_expr_constant_nil, visit_expr),
      (ExprConstantNumber, AstExprConstantNumber, visit_expr_constant_number, visit_expr),
      (ExprConstantString, AstExprConstantString, visit_expr_constant_string, visit_expr),
      (ExprError, AstExprError, visit_expr_error, visit_expr),
      (ExprFunction, AstExprFunction, visit_expr_function, visit_expr),
      (ExprGlobal, AstExprGlobal, visit_expr_global, visit_expr),
      (ExprGroup, AstExprGroup, visit_expr_group, visit_expr),
      (ExprIfElse, AstExprIfElse, visit_expr_if_else, visit_expr),
      (ExprIndexExpr, AstExprIndexExpr, visit_expr_index_expr, visit_expr),
      (ExprIndexName, AstExprIndexName, visit_expr_index_name, visit_expr),
      (ExprInstantiate, AstExprInstantiate, visit_expr_instantiate, visit_expr),
      (ExprInterpString, AstExprInterpString, visit_expr_interp_string, visit_expr),
      (ExprLocal, AstExprLocal, visit_expr_local, visit_expr),
      (ExprTable, AstExprTable, visit_expr_table, visit_expr),
      (ExprTypeAssertion, AstExprTypeAssertion, visit_expr_type_assertion, visit_expr),
      (ExprUnary, AstExprUnary, visit_expr_unary, visit_expr),
      (ExprVarargs, AstExprVarargs, visit_expr_varargs, visit_expr),
      (GenericType, AstGenericType, visit_generic_type, visit_node),
      (GenericTypePack, AstGenericTypePack, visit_generic_type_pack, visit_node),
      (StatAssign, AstStatAssign, visit_stat_assign, visit_stat),
      (StatBlock, AstStatBlock, visit_stat_block, visit_stat),
      (StatBreak, AstStatBreak, visit_stat_break, visit_stat),
      (StatClass, AstStatClass, visit_stat_class, visit_stat),
      (StatCompoundAssign, AstStatCompoundAssign, visit_stat_compound_assign, visit_stat),
      (StatContinue, AstStatContinue, visit_stat_continue, visit_stat),
      (StatDeclareExternType, AstStatDeclareExternType, visit_stat_declare_extern_type, visit_stat),
      (StatDeclareFunction, AstStatDeclareFunction, visit_stat_declare_function, visit_stat),
      (StatDeclareGlobal, AstStatDeclareGlobal, visit_stat_declare_global, visit_stat),
      (StatError, AstStatError, visit_stat_error, visit_stat),
      (StatExpr, AstStatExpr, visit_stat_expr, visit_stat),
      (StatFor, AstStatFor, visit_stat_for, visit_stat),
      (StatForIn, AstStatForIn, visit_stat_for_in, visit_stat),
      (StatFunction, AstStatFunction, visit_stat_function, visit_stat),
      (StatIf, AstStatIf, visit_stat_if, visit_stat),
      (StatLocal, AstStatLocal, visit_stat_local, visit_stat),
      (StatLocalFunction, AstStatLocalFunction, visit_stat_local_function, visit_stat),
      (StatRepeat, AstStatRepeat, visit_stat_repeat, visit_stat),
      (StatReturn, AstStatReturn, visit_stat_return, visit_stat),
      (StatTypeAlias, AstStatTypeAlias, visit_stat_type_alias, visit_stat),
      (StatTypeFunction, AstStatTypeFunction, visit_stat_type_function, visit_stat),
      (StatWhile, AstStatWhile, visit_stat_while, visit_stat),
      (TypeError, AstTypeError, visit_type_error, visit_type),
      (TypeFunction, AstTypeFunction, visit_type_function, visit_type),
      (TypeGroup, AstTypeGroup, visit_type_group, visit_type),
      (TypeIntersection, AstTypeIntersection, visit_type_intersection, visit_type),
      (TypeOptional, AstTypeOptional, visit_type_optional, visit_type),
      (TypePackExplicit, AstTypePackExplicit, visit_type_pack_explicit, visit_type_pack),
      (TypePackGeneric, AstTypePackGeneric, visit_type_pack_generic, visit_type_pack),
      (TypePackVariadic, AstTypePackVariadic, visit_type_pack_variadic, visit_type_pack),
      (TypeReference, AstTypeReference, visit_type_reference, visit_type),
      (TypeSingletonBool, AstTypeSingletonBool, visit_type_singleton_bool, visit_type),
      (TypeSingletonString, AstTypeSingletonString, visit_type_singleton_string, visit_type),
      (TypeTable, AstTypeTable, visit_type_table, visit_type),
      (TypeTypeof, AstTypeTypeof, visit_type_typeof, visit_type),
      (TypeUnion, AstTypeUnion, visit_type_union, visit_type),
    }
  };
}

/// 表回调：判别枚举本体——cpp `AstVisitor` 类型化重载的 Rust 形态，
/// 每变体持一个具体节点类型的可变引用。由 `dispatch_node` 按 RTTI 命中
/// 构造，消费方是 `AstVisitor::visit_any`。
macro_rules! define_ast_node_ref_mut {
  ($(($variant:ident, $ty:ty, $hook:ident, $parent:ident)),+ $(,)?) => {
    /// 判别枚举：每变体持一个具体节点类型的可变引用——cpp `AstVisitor`
    /// 类型化重载的 Rust 形态。由 `dispatch_node` 按 RTTI 命中构造，
    /// 消费方是 `AstVisitor::visit_any`；变体集合由 [`ast_node_table`] 生成。
    pub enum AstNodeRefMut<'a> {
      $($variant(&'a mut $ty),)+
    }
  };
}

ast_node_table!(define_ast_node_ref_mut);

/// 具体节点 `impl AstVisitable` 的共享骨架——46 个节点文件此前逐字复制
/// 「`impl AstVisitable for X` + `as_ref_mut` 自报变体 + `visit_children` 签名」
/// 这三段，只有子遍历体不同（形态对照 `ast_node_table!` 的单源化先例）。此处收
/// 为单一宏，节点只提供「类型 + [`AstNodeRefMut`] 变体 (+ 子遍历体)」：
///
/// - 二元形态（叶子节点）只自报形态，子遍历沿用 trait 默认空实现；
/// - 三元形态的 `$body` 原样成为 `visit_children` 的函数体，各节点自带的
///   `// Safety` 契约逐字保留在体内。体与形参名都按 call-site token 处理：
///   `$body` 用 `tt`（原始 token 流）而非 `block`（AST fragment 带定义 site
///   卫生性，体内标识符展开时会失去绑定），`this`/`visitor` 由调用点给出并
///   原样替换进 `let` 与形参位置，两侧卫生标记一致故可见。
///
/// 展开点需把 [`AstVisitable`]、[`AstNodeRefMut`]、[`crate::records::ast_visitor::AstVisitor`]
/// 与节点类型、体内用到的门面置于作用域（macro_rules 路径不具卫生性）。
macro_rules! impl_visitable {
  ($ty:ty, $variant:ident) => {
    impl AstVisitable for $ty {
      fn as_ref_mut(&mut self) -> AstNodeRefMut<'_> {
        AstNodeRefMut::$variant(self)
      }
    }
  };
  ($ty:ty, $variant:ident, |$this:ident, $visitor:ident| $body:tt) => {
    impl AstVisitable for $ty {
      fn as_ref_mut(&mut self) -> AstNodeRefMut<'_> {
        AstNodeRefMut::$variant(self)
      }

      fn visit_children<V: AstVisitor + ?Sized>(&mut self, $visitor: &mut V) {
        let $this = self;
        $body
      }
    }
  };
}

/// 基类家族 `*mut AstX` 裸指针门面的共享骨架（cpp `X*->visit(visitor)` 的 RTTI
/// 分发形态）：null 早退 + 一次 repr(C) 基址改写后转交 `dispatch_node`。
/// `ast_expr_visit`/`ast_stat_visit`/`ast_type_visit`/`ast_type_pack_visit` 四个门面
/// 此前逐字复制同一段 body 与 `// Safety` 注释，此处收口为单源；各门面的具名
/// `# Safety` 契约经文档属性原样挂到被生成的函数上，签名与可见性逐字保持。
macro_rules! impl_ast_ptr_visit {
  (
    $(
      $(#[$attr:meta])*
      $vis:vis fn $name:ident($ptr:ident : *mut $base:ty);
    )+
  ) => {
    $(
      $(#[$attr])*
      $vis unsafe fn $name<V: AstVisitor + ?Sized>($ptr: *mut $base, visitor: &mut V) {
        // Safety: `$ptr.as_ast_node()` 借 repr(C) 基址重合把基类指针零偏移改写为
        // `*mut AstNode`（保留 null）；`as_mut` 对 null 返回 None（等价旧 `dispatch_node`
        // 的 null 早退），非空时本函数文档的 `# Safety` 契约（null 或存活 repr(C) 节点、
        // 调用方独占其 arena）即 `&mut AstNode` 所需的存活 + 独占证明，原样交给 `dispatch_node`。
        if let Some(node) = unsafe { $ptr.as_ast_node().as_mut() } {
          dispatch_node(node, visitor);
        }
      }
    )+
  };
}

impl_ast_ptr_visit! {
  /// `expr->visit(visitor)` where `expr` is a base `*mut AstExpr` — dispatch to the
  /// concrete override by RTTI class index.
  ///
  /// # Safety
  /// `expr` 须为 null 或指向以 `AstExpr` 为前缀字段的存活节点。
  ///
  /// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
  /// 的非 const 语义写穿节点，本门面据此向 `dispatch_node` 交出 `&mut AstNode`）。
  pub fn ast_expr_visit(expr: *mut AstExpr);

  /// `stat->visit(visitor)` for a base `*mut AstStat`.
  ///
  /// # Safety
  /// `stat` 须为 null 或指向以 `AstStat` 为前缀字段的存活节点。
  ///
  /// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
  /// 的非 const 语义写穿节点，本门面据此向 `dispatch_node` 交出 `&mut AstNode`）。
  pub fn ast_stat_visit(stat: *mut AstStat);

  /// `ty->visit(visitor)` for a base `*mut AstType`.
  ///
  /// # Safety
  /// `ty` 须为 null 或指向以 `AstType` 为前缀字段的存活节点。
  ///
  /// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
  /// 的非 const 语义写穿节点，本门面据此向 `dispatch_node` 交出 `&mut AstNode`）。
  pub fn ast_type_visit(ty: *mut AstType);

  /// `pack->visit(visitor)` for a base `*mut AstTypePack`.
  ///
  /// # Safety
  /// `pack` 须为 null 或指向以 `AstTypePack` 为前缀字段的存活节点。
  ///
  /// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
  /// 的非 const 语义写穿节点，本门面据此向 `dispatch_node` 交出 `&mut AstNode`）。
  pub(crate) fn ast_type_pack_visit(pack: *mut AstTypePack);
}

/// 基类家族 `&mut AstX` 引用门面的共享骨架（records 句柄引用化 §2 的下游入口）：
/// 入参独占借用即「该节点在借用期内存活且可独占」的类型系统证明，零 unsafe
/// 直接转交 `dispatch_node`。四个 `_ref` 门面的 body 逐字相同，收口为单源；
/// 契约说明集中在 [`ast_expr_visit_ref`] 文档，其余三个以「契约同」引用之。
macro_rules! impl_ast_ref_visit {
  (
    $(
      $(#[$attr:meta])*
      $vis:vis fn $name:ident($arg:ident : &mut $base:ty);
    )+
  ) => {
    $(
      $(#[$attr])*
      $vis fn $name<V: AstVisitor + ?Sized>($arg: &mut $base, visitor: &mut V) {
        dispatch_node($arg.as_ast_node_mut(), visitor);
      }
    )+
  };
}

impl_ast_ref_visit! {
  /// `expr->visit(visitor)` 的**引用形态**（records 句柄引用化 §2 的下游入口）：
  /// `&mut AstExpr` 即该节点在借用期内存活且可独占的类型系统证明，全链路 safe。
  /// 已把子槽迁为 [`crate::records::node_handle`] 的节点（`AstStatBlock`/`AstExprFunction`
  /// 主干）经此递归，不再落回裸指针门面。
  pub fn ast_expr_visit_ref(expr: &mut AstExpr);

  /// `stat->visit(visitor)` 的引用形态，契约同 [`ast_expr_visit_ref`]。
  pub fn ast_stat_visit_ref(stat: &mut AstStat);

  /// `ty->visit(visitor)` 的引用形态，契约同 [`ast_expr_visit_ref`]。
  pub fn ast_type_visit_ref(ty: &mut AstType);

  /// `pack->visit(visitor)` 的引用形态，契约同 [`ast_expr_visit_ref`]。
  pub(crate) fn ast_type_pack_visit_ref(pack: &mut AstTypePack);
}

/// `node->visit(visitor)` for any base `*mut AstNode`.
///
/// # Safety
/// `node` 须为 null 或指向存活的 AST 节点。
///
/// 另需：调用方独占该节点所在 arena（visitor 按 cpp `visit(AstVisitor*)`
/// 的非 const 语义写穿节点，本门面据此向 `dispatch_node` 交出 `&mut AstNode`）。
pub unsafe fn ast_node_visit<V: AstVisitor + ?Sized>(node: *mut AstNode, visitor: &mut V) {
  // Safety: `node` 已是基类指针，无需改写；`as_mut` 对 null 返回 None（等价旧 `dispatch_node` 的 null 早退），
  // 非空时本函数契约（node 为 null 或存活 AST 节点、调用方独占其 arena）即 `&mut AstNode` 所需的存活 + 独占证明。
  if let Some(node) = unsafe { node.as_mut() } {
    dispatch_node(node, visitor);
  }
}

/// 表回调：`dispatch_node`——cpp vtable 的中央 analog，一次 `match`
/// class-index 完成下转 + 类型化回调 + 子遍历。
///
/// 臂的模式直接引用 `<$ty>::CLASS_INDEX`（[`crate::rtti::AstNodeClass`] 关联
/// 常量），与 C++ `ClassIndex()` 强绑定——类型改名或索引变化时臂自动跟随。
/// 关联常量直接作 match 模式（i32 结构匹配合法）：编译器可将其优化为跳转表，
/// guard 写法 `x if x == ..` 会强制逐臂线性求值。
macro_rules! define_dispatch_node {
  ($(($variant:ident, $ty:ty, $hook:ident, $parent:ident)),+ $(,)?) => {
    /// The central class-index dispatcher — the analog of the C++ vtable. One arm
    /// per concrete node type; each downcast is sound for the same reason as
    /// [`crate::rtti::mut_cast`] (standard-layout, base at offset 0).
    ///
    /// 入参为 `&mut AstNode`：引用即「该 place 在借用期内存活且可独占」的类型系统
    /// 证明，故本函数是 safe fn——空指针早退交回调用方（各 `*_visit` 门面），
    /// 唯一的裸指针下转收口在臂内的 `// Safety:` 块。
    ///
    /// 静态类型已是 `AstStatBlock` 的入口（cpp `block->visit(visitor)` 直调 override，
    /// 无需 class-index 分发）不走这里，改调
    /// [`crate::methods::ast_stat_block_visit::ast_stat_block_visit`]。
    pub fn dispatch_node<V: AstVisitor + ?Sized>(node: &mut AstNode, visitor: &mut V) {
      match node.class_index {
        $(
          <$ty>::CLASS_INDEX => {
            // Safety: class_index 已命中 `<$ty>`，#[repr(C)] 单继承保证基址重合；`&mut` 入参
            // 即该 arena place 在借用期内的独占证明，visitor 串行写穿，无重叠 `&mut` 别名。
            let typed = unsafe { &mut *(node as *mut AstNode).cast::<$ty>() };
            if visitor.visit_any(AstNodeRefMut::$variant(typed)) {
              typed.visit_children(visitor);
            }
          }
        )+
        _ => {
          // cpp 经虚表分发不可达此处：每个 AstNode 具体子类都覆写 visit。
          // 未知 class index 意味着 arena 内存损坏，属不可恢复的程序错误，
          // panic 携带类号便于定位损坏来源（改返回会吞掉唯一诊断信号）。
          panic!("dispatch_node: 未知 AST class index {}", node.class_index);
        }
      }
    }
  };
}

ast_node_table!(define_dispatch_node);
