//! AST RTTI mechanism — Rust 安全模型下的 `AstRtti<T>::value` /
//! `LUAU_RTTI(Class)` / `AstNode::is<T>()` / `as<T>()` analog。
//! Reference: `luau/Ast/include/Luau/Ast.h` (the `AstNode` base + the
//! `LUAU_RTTI` macro)。
//!
//! C++ 侧每个节点携带构造期写入的 `const int classIndex`，`node->as<T>()` 即
//! `classIndex == T::ClassIndex() ? static_cast<T*>(this) : nullptr`；强转合法
//! 是因为节点是 standard-layout 单继承，基类子对象位于偏移 0。
//!
//! Rust 侧不再逐字移植该指针舞蹈，而是把门面收口成引用形态：
//!
//! - 全部节点类型经宏实现安全视图 trait [`AstNodeView`] / [`AstNodeViewMut`]
//!   （`#[repr(C)]` 首字段 `base` 的纯字段访问链，零 unsafe），判型/下转入口
//!   吃 `&impl AstNodeView` / `&mut impl AstNodeViewMut`，返回 `bool` /
//!   `Option<&T>` / `Option<&mut T>`——null 哨兵与 `unsafe trait` 一并消失。
//! - 下转的类型改写（cpp `static_cast<T*>` 的 analog）收口在两个私有一行
//!   原语 [`ref_cast`] / [`mut_cast`]，各带 `# Safety` 契约；safe 门面在类索引
//!   命中判定后调用它们。
//! - 仍环绕裸指针的 arena 消费侧（records 的 `*mut` 字段引用化是 records 分支
//!   的工作）经 [`AstNodePtr`] 视图 trait 进入边界门面 [`ast_node_try_as_ptr`] /
//!   [`ast_node_try_as_ptr_mut`] / [`ast_node_is_ptr`]，它们是唯一保留 `# Safety`
//!   契约的 pub unsafe 入口，内部同样只转调安全门面。
//!
//! class index 是类型名的编译期哈希（FNV-1a），没有运行期共享可变计数器 /
//! 注册表（区别于 C++ 的 `++gAstRttiIndex`）。AST 侧 impl 由
//! `ast_node_table`（`visit.rs`）回调集中生成（表即节点集合的单一事实来
//! 源），CST 侧经 `impl_cst_node_class!` 各节点文件一行登记。具体整数值无关
//! 紧要——只需每类型唯一且稳定——由 `tests/rtti.rs` 的 `rtti_indices_unique`
//! 在全节点集上强制。

use core::ptr;

use crate::records::{
  ast_attr::AstAttr,
  ast_expr::AstExpr,
  ast_expr_binary::AstExprBinary,
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
  ast_expr_table::AstExprTable,
  ast_expr_type_assertion::AstExprTypeAssertion,
  ast_expr_unary::AstExprUnary,
  ast_expr_varargs::AstExprVarargs,
  ast_generic_type::AstGenericType,
  ast_generic_type_pack::AstGenericTypePack,
  ast_node::AstNode,
  ast_stat::AstStat,
  ast_stat_assign::AstStatAssign,
  ast_stat_block::AstStatBlock,
  ast_stat_break::AstStatBreak,
  ast_stat_class::AstStatClass,
  ast_stat_compound_assign::AstStatCompoundAssign,
  ast_stat_continue::AstStatContinue,
  ast_stat_declare_extern_type::AstStatDeclareExternType,
  ast_stat_declare_function::AstStatDeclareFunction,
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
  ast_type_optional::AstTypeOptional,
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
  cst_node::CstNode,
  node_handle::{Node, OptNode},
};

/// FNV-1a 32 位参数：offset basis 与 prime。
const FNV1A_OFFSET_BASIS: u32 = 0x811c_9dc5;
const FNV1A_PRIME: u32 = 0x0100_0193;
/// 折叠掩码：保持索引为正值，为负数哨兵（如 -1）留位。
const CLASS_INDEX_MASK: u32 = 0x7fff_ffff;

/// Stable per-type class index, the analog of `AstRtti<Class>::value`. FNV-1a
/// over the class name, folded to a positive `i32` so it can never collide with
/// a future "no class" sentinel (e.g. `-1`).
pub const fn ast_rtti_index(name: &str) -> i32 {
  // 切片模式逐字节消费：const fn 兼容且免索引越界检查
  let mut hash: u32 = FNV1A_OFFSET_BASIS;
  let mut bytes = name.as_bytes();
  while let [b, rest @ ..] = bytes {
    hash ^= *b as u32;
    hash = hash.wrapping_mul(FNV1A_PRIME);
    bytes = rest;
  }
  (hash & CLASS_INDEX_MASK) as i32
}

/// Implemented by every concrete AST node type — the analog of the
/// `LUAU_RTTI(Class)` macro, which expands to `static int ClassIndex()`.
///
/// A node `class X : Y` becomes `#[repr(C)] struct X { pub base: Y, ... }`；
/// `impl AstNodeClass for X`（`CLASS_INDEX = ast_rtti_index("X")`）由本模块的
/// `define_ast_node_class` 按 `ast_node_table`（`visit.rs`）全节点集统一生成。
pub trait AstNodeClass {
  /// The node's RTTI id; mirrors `T::ClassIndex()`.
  const CLASS_INDEX: i32;
}

/// cpp 隐式上转 `static_cast<AstNode*>(this)` 的引用 analog：任何节点类型都能
/// 经 `base` 字段链零成本升为 `&AstNode`。纯字段访问，safe，无布局魔法。
pub trait AstNodeView {
  /// 本节点基类 `AstNode` 的共享视图。
  fn as_ast_node(&self) -> &AstNode;
}

/// [`AstNodeView`] 的可变形态。
pub trait AstNodeViewMut {
  /// 本节点基类 `AstNode` 的独占视图。
  fn as_ast_node_mut(&mut self) -> &mut AstNode;
}

impl AstNodeView for AstNode {
  #[inline]
  fn as_ast_node(&self) -> &AstNode {
    self
  }
}

impl AstNodeViewMut for AstNode {
  #[inline]
  fn as_ast_node_mut(&mut self) -> &mut AstNode {
    self
  }
}

/// 为全部节点类型（含 `AstExpr`/`AstStat`/`AstType`/`AstTypePack` 四个基类）
/// 生成视图链：`self.base.as_ast_node()` 沿 repr(C) 首字段递归到 `AstNode`
/// 的 identity 实现，safe 且与手写 `&x.base.base` 完全等价。
macro_rules! impl_ast_node_view {
  ($($ty:ty),* $(,)?) => {
    $(
      impl AstNodeView for $ty {
        #[inline]
        fn as_ast_node(&self) -> &AstNode {
          self.base.as_ast_node()
        }
      }
      impl AstNodeViewMut for $ty {
        #[inline]
        fn as_ast_node_mut(&mut self) -> &mut AstNode {
          self.base.as_ast_node_mut()
        }
      }
    )*
  };
}

impl_ast_node_view!(
  AstExpr,
  AstStat,
  AstType,
  AstTypePack,
  AstStatBlock,
  AstAttr,
  AstExprBinary,
  AstExprCall,
  AstExprConstantBool,
  AstExprConstantInteger,
  AstExprConstantNil,
  AstExprConstantNumber,
  AstExprConstantString,
  AstExprError,
  AstExprFunction,
  AstExprGlobal,
  AstExprGroup,
  AstExprIfElse,
  AstExprIndexExpr,
  AstExprIndexName,
  AstExprInstantiate,
  AstExprInterpString,
  AstExprLocal,
  AstExprTable,
  AstExprTypeAssertion,
  AstExprUnary,
  AstExprVarargs,
  AstGenericType,
  AstGenericTypePack,
  AstStatAssign,
  AstStatBreak,
  AstStatClass,
  AstStatCompoundAssign,
  AstStatContinue,
  AstStatDeclareExternType,
  AstStatDeclareFunction,
  AstStatDeclareGlobal,
  AstStatError,
  AstStatExpr,
  AstStatFor,
  AstStatForIn,
  AstStatFunction,
  AstStatIf,
  AstStatLocal,
  AstStatLocalFunction,
  AstStatRepeat,
  AstStatReturn,
  AstStatTypeAlias,
  AstStatTypeFunction,
  AstStatWhile,
  AstTypeError,
  AstTypeFunction,
  AstTypeGroup,
  AstTypeIntersection,
  AstTypeOptional,
  AstTypePackExplicit,
  AstTypePackGeneric,
  AstTypePackVariadic,
  AstTypeReference,
  AstTypeSingletonBool,
  AstTypeSingletonString,
  AstTypeTable,
  AstTypeTypeof,
  AstTypeUnion
);

/// 表回调：`ast_node_table` 的第 4 处消费方——按表内 60 个具体节点类型统一
/// 生成 [`AstNodeClass`]（`LUAU_RTTI(Class)` 的直译），取代此前散落在各
/// records 文件的逐文件手写 impl。CLASS_INDEX 仍是类型名的 FNV-1a 哈希
/// （`stringify!` 与原字面量逐字相同，索引值一位不差），只是节点集合归入
/// 分发表单一事实来源：增删节点不再需要同步抄写第 4 份名单。
macro_rules! define_ast_node_class {
  ($(($variant:ident, $ty:ty, $hook:ident, $parent:ident)),+ $(,)?) => {
    $(
      impl AstNodeClass for $ty {
        const CLASS_INDEX: i32 = ast_rtti_index(stringify!($ty));
      }
    )+
  };
}

ast_node_table!(define_ast_node_class);

/// CST 节点的 [`CstNodeClass`] 一行式生成（`LUAU_CST_RTTI(Class)` analog）。
/// CST 与 AST 是独立索引空间、不进 `ast_node_table`，故保留各节点文件就地
/// 调用；哈希值与此前手写的 `ast_rtti_index("X")`/`cst_rtti_index("X")` 完全
/// 一致（两函数共用同一实现）。`$crate` 全路径展开，消费点免 import。
macro_rules! impl_cst_node_class {
  ($ty:ident) => {
    impl $crate::rtti::CstNodeClass for $ty {
      const CLASS_INDEX: i32 = $crate::rtti::cst_rtti_index(stringify!($ty));
    }
  };
}

/// AST 节点 `X::new(location, fields…)` 构造样板：`base` 经基类门面
/// `AstX::new` 写入类索引并携带 location，其余参数按同名直通为字段。
/// 展开点需将 `$ty`/`$base`/各字段类型置于作用域（macro_rules 路径不具
/// 卫生性），类索引与基类门面经 `$crate` 全路径免 import。
macro_rules! impl_ast_node_new {
  ($ty:ident, $base:ident, $location:ident : $locty:ty $(, $field:ident : $fty:ty)* $(,)?) => {
    impl $ty {
      pub fn new($location: $locty, $( $field: $fty ),*) -> Self {
        Self {
          base: $base::new(<Self as $crate::rtti::AstNodeClass>::CLASS_INDEX, $location),
          $( $field, )*
        }
      }
    }
  };
}

/// CST 节点 `X::new(fields…)` 构造样板：CST 基类无 location 字段，`base` 只写
/// 类索引；其余参数按同名直通为字段。
macro_rules! impl_cst_node_new {
  ($ty:ident $(, $field:ident : $fty:ty)* $(,)?) => {
    impl $ty {
      pub fn new($( $field: $fty ),*) -> Self {
        Self {
          base: $crate::records::cst_node::CstNode::new(
            <Self as $crate::rtti::CstNodeClass>::CLASS_INDEX,
          ),
          $( $field, )*
        }
      }
    }
  };
}

/// 本模块唯一的共享引用类型改写核心（cpp `static_cast<T*>(this)` 的收口点）。
///
/// # Safety
/// `node` 所属 place 的对象必须存活且动态类型就是 `T`：调用方已验证其
/// `class_index == T::CLASS_INDEX`，且 `T` 是 `#[repr(C)]`、首字段（传递地）为
/// 基节点的生成结构——基址重合，改写后的引用 denote 同一个 place；借用寿命
/// 由该对象的合法共享借用供给。
#[inline]
unsafe fn ref_cast<B, T>(node: &B) -> &T {
  // Safety: 函数级 # Safety 契约即合法性证明：repr(C) 首字段链保证
  // ptr::from_ref(node) 与 &T 同址同布局起点。
  unsafe { &*(ptr::from_ref(node).cast::<T>()) }
}

/// [`ref_cast`] 的独占形态。
///
/// # Safety
/// 同 [`ref_cast`]，另需调用方持有该 place 的独占借用（无重叠别名）。
#[inline]
unsafe fn mut_cast<B, T>(node: &mut B) -> &mut T {
  // Safety: 函数级 # Safety 契约，独占性沿 &mut 继承。
  unsafe { &mut *(ptr::from_mut(node).cast::<T>()) }
}

/// `node->is<T>()` — does this node have `T`'s dynamic type? 纯 safe：
/// class_index 是普通字段读取。
#[inline]
pub fn ast_node_is<T: AstNodeClass>(node: &impl AstNodeView) -> bool {
  node.as_ast_node().class_index == T::CLASS_INDEX
}

/// `node->as<T>()` — 引用进、`Option<&T>` 出，动态类型不匹配即 `None`。
#[inline]
pub fn ast_node_try_as<T: AstNodeClass>(node: &impl AstNodeView) -> Option<&T> {
  let base = node.as_ast_node();
  (base.class_index == T::CLASS_INDEX).then(||
    // Safety: class_index 命中 ⇒ 该 place 动态类型为 T（class_index 由 Allocator
    // 按具体节点类型写入，与真实类型一一对应）；repr(C) 首字段链保证基址重合；
    // 共享借用寿命与独占性约束均由入参引用继承。
    unsafe { ref_cast::<AstNode, T>(base) })
}

/// [`ast_node_try_as`] 的可变形态：独占借用进、`Option<&mut T>` 出。
/// 输出生命周期由入参借用供给，入参即类型系统的存活 + 独占证明。
#[inline]
pub fn ast_node_try_as_mut<T: AstNodeClass>(node: &mut impl AstNodeViewMut) -> Option<&mut T> {
  let base = node.as_ast_node_mut();
  (base.class_index == T::CLASS_INDEX).then(||
    // Safety: `&mut` 入参即该 place 在借用期内独占且存活的证明，class_index
    // 命中 ⇒ 动态类型为 T；repr(C) 首字段链保证基址重合，独占性原样继承。
    unsafe { mut_cast::<AstNode, T>(base) })
}

/// 基于 `class_index` 匹配命中后的无开销强制下转（常用于 `match node.class_index` 分发表）。
///
/// # Safety
/// 调用方必须验证 `node.as_ast_node().class_index == T::CLASS_INDEX`。
#[inline]
pub unsafe fn ast_node_as_unchecked<T: AstNodeClass>(node: &impl AstNodeView) -> &T {
  unsafe { ref_cast::<AstNode, T>(node.as_ast_node()) }
}

/// 基类家族指针 → `*mut AstNode` 的零成本类型视图转换（safe，永不解引用）。
///
/// 对一切 `*mut T` 泛型可用：任何节点指针都能直接喂进下面的边界门面，调用点
/// 不必手写 `as *mut AstNode`。方法本身只改类型视图、保留地址与 null 性；
/// "T 必须是 repr(C) 节点类型" 由边界门面的 `# Safety` 契约兑现。
pub trait AstNodePtr: Copy {
  fn as_ast_node(self) -> *mut AstNode;
}

impl<T> AstNodePtr for *mut T {
  #[inline]
  fn as_ast_node(self) -> *mut AstNode {
    self.cast()
  }
}

impl<T> AstNodePtr for *const T {
  #[inline]
  fn as_ast_node(self) -> *mut AstNode {
    self.cast::<AstNode>().cast_mut()
  }
}

// 句柄化字段的过渡桥:`Node`/`OptNode` 直接喂进既有指针门面(消费点与 `*mut`
// 完全同形),records 全量引用化后这些消费点会随 ptr 门面一起退役。
impl<T> AstNodePtr for Node<T> {
  #[inline]
  fn as_ast_node(self) -> *mut AstNode {
    self.as_ptr().cast()
  }
}

impl<T> AstNodePtr for OptNode<T> {
  #[inline]
  fn as_ast_node(self) -> *mut AstNode {
    self.as_ptr().cast()
  }
}

/// [`ast_node_try_as`] 在仍环绕裸指针的 arena 消费侧的边界形态：null 与动态
/// 类型不匹配一并折叠为 `None`。records 引用化（并行的 records 分支）完成后
/// 本门面随之退役。
///
/// # Safety
/// `node` 须为 null 或指向存活的 repr(C) AST 节点（首字段传递地为 `AstNode`）；
/// 返回借用存续期内，调用方保证其所在 arena 无重叠别名。
#[inline]
pub unsafe fn ast_node_try_as_ptr<T: AstNodeClass>(node: impl AstNodePtr) -> Option<&'static T> {
  let node = node.as_ast_node();
  if node.is_null() {
    return None;
  }
  // Safety: 契约保证非空即存活节点，&'static 借用即既有 arena 只读独占纪律；
  // 判型与类型改写的合法性由安全门面 ast_node_try_as 兑现。
  ast_node_try_as::<T>(unsafe { &*node })
}

/// 指针版 [`ast_node_try_as_mut`]。只读场景用 [`ast_node_try_as_ptr`]，把可变
/// 借用的半径压到最小。
///
/// # Safety
/// 同 [`ast_node_try_as_ptr`]，另需调用方在返回引用生命周期内独占该节点所在
/// arena（visitor 按 cpp `visit(AstVisitor*)` 非 const 语义写穿节点的场景）。
#[inline]
pub unsafe fn ast_node_try_as_ptr_mut<T: AstNodeClass>(
  node: impl AstNodePtr,
) -> Option<&'static mut T> {
  let node = node.as_ast_node();
  if node.is_null() {
    return None;
  }
  // Safety: 契约保证非空即存活且调用方可独占该 place；&'static mut 即既有
  // 独占写穿纪律，判型与类型改写转调安全门面。
  ast_node_try_as_mut::<T>(unsafe { &mut *node })
}

/// [`ast_node_is`] 在裸指针侧的边界形态：null 折叠为 `false`（与旧
/// `AstNodeRef::class_index` 的空指针行为一致），全程只读、不外传借用。
///
/// # Safety
/// `node` 须为 null 或指向存活的 repr(C) AST 节点（首字段传递地为 `AstNode`）。
#[inline]
pub unsafe fn ast_node_is_ptr<T: AstNodeClass>(node: impl AstNodePtr) -> bool {
  let node = node.as_ast_node();
  // Safety: null 早退不解引用；非空即存活节点，读偏移 0 基类的 class_index
  // 字段，判型转调安全门面。
  !node.is_null() && ast_node_is::<T>(unsafe { &*node })
}

/// 基类家族判别：`node->asExpr()` 的判别面（cpp `AstNode::asExpr()` 只在 21 个
/// `AstExpr` 派生类上非空）。与 [`is_stat_class`] 同理，判别表集中于此，供
/// 各消费侧共用，指针转换与类判别彻底解耦。
#[inline]
pub fn is_expr_class(class_index: i32) -> bool {
  matches!(
    class_index,
    AstExprBinary::CLASS_INDEX
      | AstExprCall::CLASS_INDEX
      | AstExprConstantBool::CLASS_INDEX
      | AstExprConstantInteger::CLASS_INDEX
      | AstExprConstantNil::CLASS_INDEX
      | AstExprConstantNumber::CLASS_INDEX
      | AstExprConstantString::CLASS_INDEX
      | AstExprError::CLASS_INDEX
      | AstExprFunction::CLASS_INDEX
      | AstExprGlobal::CLASS_INDEX
      | AstExprGroup::CLASS_INDEX
      | AstExprIfElse::CLASS_INDEX
      | AstExprIndexExpr::CLASS_INDEX
      | AstExprIndexName::CLASS_INDEX
      | AstExprInstantiate::CLASS_INDEX
      | AstExprInterpString::CLASS_INDEX
      | AstExprLocal::CLASS_INDEX
      | AstExprTable::CLASS_INDEX
      | AstExprTypeAssertion::CLASS_INDEX
      | AstExprUnary::CLASS_INDEX
      | AstExprVarargs::CLASS_INDEX
  )
}

/// 基类家族判别：`node->asStat()` 的判别面（cpp `AstNode::asStat()` 只在 21 个
/// `AstStat` 派生类上非空）。类索引判别与指针转换解耦，避免多份 `matches!`
/// 表漂移。
#[inline]
pub fn is_stat_class(class_index: i32) -> bool {
  matches!(
    class_index,
    AstStatAssign::CLASS_INDEX
      | AstStatBlock::CLASS_INDEX
      | AstStatBreak::CLASS_INDEX
      | AstStatClass::CLASS_INDEX
      | AstStatCompoundAssign::CLASS_INDEX
      | AstStatContinue::CLASS_INDEX
      | AstStatDeclareExternType::CLASS_INDEX
      | AstStatDeclareFunction::CLASS_INDEX
      | AstStatDeclareGlobal::CLASS_INDEX
      | AstStatError::CLASS_INDEX
      | AstStatExpr::CLASS_INDEX
      | AstStatFor::CLASS_INDEX
      | AstStatForIn::CLASS_INDEX
      | AstStatFunction::CLASS_INDEX
      | AstStatIf::CLASS_INDEX
      | AstStatLocal::CLASS_INDEX
      | AstStatLocalFunction::CLASS_INDEX
      | AstStatRepeat::CLASS_INDEX
      | AstStatReturn::CLASS_INDEX
      | AstStatTypeAlias::CLASS_INDEX
      | AstStatTypeFunction::CLASS_INDEX
      | AstStatWhile::CLASS_INDEX
  )
}

/// 基类家族判别：`node->asType()` 的判别面（cpp `AstNode::asType()` 只在 11 个
/// `AstType` 派生类上非空）。与 [`is_expr_class`] 同理。
#[inline]
pub fn is_type_class(class_index: i32) -> bool {
  matches!(
    class_index,
    AstTypeError::CLASS_INDEX
      | AstTypeFunction::CLASS_INDEX
      | AstTypeGroup::CLASS_INDEX
      | AstTypeIntersection::CLASS_INDEX
      | AstTypeOptional::CLASS_INDEX
      | AstTypeReference::CLASS_INDEX
      | AstTypeSingletonBool::CLASS_INDEX
      | AstTypeSingletonString::CLASS_INDEX
      | AstTypeTable::CLASS_INDEX
      | AstTypeTypeof::CLASS_INDEX
      | AstTypeUnion::CLASS_INDEX
  )
}

/// CST spelling of [`ast_rtti_index`] (CST and AST share the index function but
/// separate index spaces). Provided so `LUAU_CST_RTTI(Class)` translations can
/// read naturally as `cst_rtti_index("CstX")`.
#[inline]
pub const fn cst_rtti_index(name: &str) -> i32 {
  ast_rtti_index(name)
}

/// CST analog of [`AstNodeClass`] — the `LUAU_CST_RTTI(Class)` macro, which
/// expands to `static int CstClassIndex()`. CST nodes form a separate RTTI
/// space (`gCstRttiIndex`) and a `CstNode*` is never cross-cast to an `AstNode*`,
/// so reusing [`ast_rtti_index`] for the index value is sound — uniqueness only
/// has to hold among CST names (见 `tests/rtti.rs` 的 `cst_rtti_indices_unique`)。
pub trait CstNodeClass {
  /// The node's CST RTTI id; mirrors `T::CstClassIndex()`.
  const CLASS_INDEX: i32;
}

/// `cstNode->as<T>()` 的 safe 引用形态，与 [`ast_node_try_as`] 同形。CST 与 AST
/// 是两套独立的 RTTI 索引空间（cpp `gCstRttiIndex` / `gAstRttiIndex`），故 CST
/// 侧需要自己的下转入口，不能复用 `ast_node_try_as`。
#[inline]
pub fn cst_node_try_as<T: CstNodeClass>(node: &CstNode) -> Option<&T> {
  (node.class_index == T::CLASS_INDEX).then(||
    // Safety: class_index 命中 ⇒ 动态类型为 T；CST 节点同为 repr(C) 首字段基类
    // 布局；共享借用寿命由入参引用继承。
    unsafe { ref_cast::<CstNode, T>(node) })
}

/// CST 下转的裸指针边界（crate 内 parser/printer 的 ast→cst 映射消费），null
/// 折叠为 `None`。
///
/// 生命周期 `'b` 由调用方传入的对同 arena AST 节点的借用供给：AST 与 CST 同步
/// 构造于同一 arena，「该 AST 节点在 `'b` 内被独占」即蕴含其映射的 CST 节点在
/// `'b` 内可独占写。
///
/// # Safety
/// `node` 须为 null 或指向存活的 repr(C) CST 节点（首字段传递地为 `CstNode`），
/// 且调用方持有该 CST 节点所在 arena 区间的独占。
#[inline]
pub(crate) unsafe fn cst_node_as<'b, T: CstNodeClass>(node: *mut CstNode) -> Option<&'b mut T> {
  if node.is_null() {
    return None;
  }
  // Safety: 契约保证非空即存活且可独占；判型与类型改写收口到独占形态。
  let base = unsafe { &mut *node };
  (base.class_index == T::CLASS_INDEX).then(|| unsafe {
    // Safety: class_index 命中 ⇒ 动态类型为 T，repr(C) 基址重合，独占性继承。
    mut_cast::<CstNode, T>(base)
  })
}

/// [`cst_node_as`] 的只读形态。
///
/// # Safety
/// 同 [`cst_node_as`]，但 `'b` 由共享借用供给，无独占要求。
#[inline]
pub(crate) unsafe fn cst_node_as_ref<'b, T: CstNodeClass>(node: *mut CstNode) -> Option<&'b T> {
  if node.is_null() {
    return None;
  }
  // Safety: 契约保证非空即存活节点，转调 safe 引用门面。
  cst_node_try_as::<T>(unsafe { &*node })
}
