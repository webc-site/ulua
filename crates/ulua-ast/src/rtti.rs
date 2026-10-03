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
//!   的工作）经 [`AstNodePtr`] 视图 trait 进入边界门面 [`ast_node_try_cast_ptr`]
//!   （指针身份形态：判型命中交出类型化 `NonNull`，只用于裸指针传递 / map key /
//!   判等，不进借用系统）。取引用一律走生命周期由 `&self`/`&mut self` 供给的
//!   安全形态 [`ast_node_try_as`] / [`ast_node_try_as_mut`] 或句柄
//!   `Node::try_as` / `OptNode::is`；早先返回假 `'static` 借用的
//!   `ast_node_try_as_ptr`/`ast_node_is_ptr` 门面已随消费点清零而退役。
//!   指针 → 借用的判空解引用再收口到 [`place_ref_at`] /
//!   [`place_mut_at`] 两枚私有原语（前者供 AST 指针身份门面、后者供 CST 独占
//!   下转门面；CST 只读下转无裸指针入口，消费点一律经 safe
//!   `optional_node::slot_opt` + [`cst_node_try_as`] 组合），AST/CST 全部边界不再各自
//!   重复 `is_null` 守卫与 `&mut *node` 舞步，内部同样只转调安全门面。
//!
//! class index 是类型名的编译期哈希（FNV-1a），没有运行期共享可变计数器 /
//! 注册表（区别于 C++ 的 `++gAstRttiIndex`）。AST 侧 impl 由
//! `ast_node_table`（`visit.rs`）回调集中生成（表即节点集合的单一事实来
//! 源），CST 侧经 `impl_cst_node_class!` 各节点文件一行登记。具体整数值无关
//! 紧要——只需每类型唯一且稳定——由 `tests/rtti.rs` 的 `rtti_indices_unique`
//! 在全节点集上强制。

use core::ptr::{self, NonNull};

// `ast_node_table!` 的三个表回调（class 索引 / 视图链 / 家族判别）都在本模块
// 展开，而 macro_rules 路径不具卫生性，故展开点须将全部节点类型导入作用域
// （同 `records/ast_visitor.rs`）。
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

/// 表回调：具体节点视图链由 `ast_node_table` 导出（此前是同集合的第二份手抄
/// 名单），四个基类不在表内（它们无 `visit` override），就地显式列出。
macro_rules! define_ast_node_view {
  ($(($variant:ident, $ty:ty, $hook:ident, $parent:ident)),+ $(,)?) => {
    impl_ast_node_view! {
      AstExpr,
      AstStat,
      AstType,
      AstTypePack,
      $($ty,)+
    }
  };
}

ast_node_table!(define_ast_node_view);

/// 表回调：`ast_node_table` 的消费方之一——按表内全部具体节点类型统一
/// 生成 [`AstNodeClass`]（`LUAU_RTTI(Class)` 的直译），取代此前散落在各
/// records 文件的逐文件手写 impl。CLASS_INDEX 仍是类型名的 FNV-1a 哈希
/// （`stringify!` 与原字面量逐字相同，索引值一位不差），只是节点集合归入
/// 分发表单一事实来源：增删节点不再需要同步抄写名单。
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

/// 表行第 4 字段（父 hook 名）→ 基类家族位：每个具体节点恰属
/// `Expr/Stat/Type/TypePack` 四家族之一，表根 hook（`visit_node`，即
/// `AstAttr`/`AstGenericType`/`AstGenericTypePack` 这类直接挂 `AstNode` 的
/// 节点）不属于任何基类家族，位值 0。编译期常量折叠，无运行开销。
macro_rules! family_bits {
  (visit_expr) => {
    1u8
  };
  (visit_stat) => {
    2u8
  };
  (visit_type) => {
    4u8
  };
  (visit_type_pack) => {
    8u8
  };
  (visit_node) => {
    0u8
  };
}

/// 表回调（第 5 处消费方）：从 [`visit::ast_node_table`] 的父 hook 列生成四个
/// 基类家族判别函数（cpp `node->asExpr()/asStat()/asType()/asTypePack()` 的判
/// 别面）。每行按家族位常量折叠：非本族行贡献恒 `false` 项，本族行贡献
/// `class_index == T::CLASS_INDEX`——与逐手抄的 `matches!` 表逐位等价，但节点
/// 集合归入分发表单一事实来源，增删节点不再需要同步维护判别名单（此前
/// 四份名单是漂移风险源）。
macro_rules! define_ast_family_predicates {
  ($(($variant:ident, $ty:ty, $hook:ident, $parent:ident)),+ $(,)?) => {
    /// 基类家族判别：`node->asExpr()` 的判别面（家族成员集由 `ast_node_table` 导出）。
    #[inline]
    pub fn is_expr_class(class_index: i32) -> bool {
      false $(|| family_bits!($parent) & 1 != 0
        && class_index == <$ty as $crate::rtti::AstNodeClass>::CLASS_INDEX)*
    }

    /// 基类家族判别：`node->asStat()` 的判别面（家族成员集由 `ast_node_table` 导出）。
    #[inline]
    pub(crate) fn is_stat_class(class_index: i32) -> bool {
      false $(|| family_bits!($parent) & 2 != 0
        && class_index == <$ty as $crate::rtti::AstNodeClass>::CLASS_INDEX)*
    }

    /// 基类家族判别：`node->asType()` 的判别面（家族成员集由 `ast_node_table` 导出）。
    #[inline]
    pub(crate) fn is_type_class(class_index: i32) -> bool {
      false $(|| family_bits!($parent) & 4 != 0
        && class_index == <$ty as $crate::rtti::AstNodeClass>::CLASS_INDEX)*
    }

    /// 基类家族判别：`node->asTypePack()` 的判别面（家族成员集由 `ast_node_table` 导出）。
    #[inline]
    pub(crate) fn is_type_pack_class(class_index: i32) -> bool {
      false $(|| family_bits!($parent) & 8 != 0
        && class_index == <$ty as $crate::rtti::AstNodeClass>::CLASS_INDEX)*
    }
  };
}

ast_node_table!(define_ast_family_predicates);

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

/// AST 只读引用判别枚举（`AstExprRef`/`AstStatRef`/`AstTypeRef`/`AstTypePackRef`
/// 四件套）的整装生成器：enum + `try_from_*`/`from_*` + `as_ast_node` +
/// `location` + `From<&Base>` + `AstNodeView`。四个枚举此前逐份手抄同一段
/// per-variant 样板（判型臂逐条 `CLASS_INDEX =>`、上转臂逐条 `n.as_ast_node()`），
/// 收口于此单一来源；variant 表仍由各调用点显式给出（与 `ast_node_table`
/// 分工不同：Ref 枚举的 variant 命名可与记录类型名解耦，如
/// `DeclareClass(AstStatClass)`）。
///
/// 类型/路径经 `$crate` 全路径展开，调用点仅需将各 variant 的记录类型名置于
/// 作用域。判型臂经安全门面 [`ast_node_try_as`] 完成下转（内部复核
/// `class_index` 命中）：命中即动态类型为 `$ty`，repr(C) 首字段基址重合，
/// 整个宏展开零 `unsafe`。
macro_rules! define_ast_ref_enum {
  (
    $(#[$doc:meta])*
    $name:ident<'a> : $base:ident ;
    try $try_from:ident , from $from:ident , expect $panic:literal ;
    variants { $($variant:ident ($ty:ident)),+ $(,)? }
  ) => {
    $(#[$doc])*
    #[derive(Debug, Clone, Copy)]
    pub enum $name<'a> {
      $($variant(&'a $ty)),+
    }

    impl<'a> $name<'a> {
      /// 尝试从基类引用构建具体判别枚举。
      /// 若 `class_index` 不属于已知节点类型，返回 `None`。
      #[inline]
      pub fn $try_from(node: &'a $base) -> Option<Self> {
        match node.base.class_index {
          $(
            <$ty as $crate::rtti::AstNodeClass>::CLASS_INDEX => {
              $crate::rtti::ast_node_try_as::<$ty>(node).map(Self::$variant)
            }
          )+
          _ => None,
        }
      }

      /// 从基类引用构建具体判别枚举。
      ///
      /// # Panics
      /// 若 `node.base.class_index` 不属于已知节点类型，触发 panic。
      #[inline]
      pub fn $from(node: &'a $base) -> Self {
        Self::$try_from(node).expect($panic)
      }

      /// 获取该节点的基类 `AstNode` 引用。
      #[inline]
      pub fn as_ast_node(&self) -> &'a $crate::records::ast_node::AstNode {
        match *self {
          $(Self::$variant(n) => $crate::rtti::AstNodeView::as_ast_node(n)),+
        }
      }

      /// 获取该节点的源码位置。
      #[inline]
      pub fn location(&self) -> $crate::records::location::Location {
        self.as_ast_node().location
      }
    }

    impl<'a> From<&'a $base> for $name<'a> {
      #[inline]
      fn from(node: &'a $base) -> Self {
        Self::$from(node)
      }
    }

    impl $crate::rtti::AstNodeView for $name<'_> {
      #[inline]
      fn as_ast_node(&self) -> &$crate::records::ast_node::AstNode {
        self.as_ast_node()
      }
    }
  };
}

pub(crate) use define_ast_ref_enum;

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

/// 裸 `*mut B` → 共享借用的唯一解引用面（判空先行，null 折叠为 `None`）：
/// AST/CST 两侧的指针边界门面共用本对原语，`unsafe` 不再逐点重复。
///
/// 契约（safe fn + 内部 unsafe，先例 `handle_registry::resolve`）：`node` 须为
/// null 或指向存活的 `B` 实例（本模块内即 repr(C) 节点，首字段传递地为基节点），
/// 返回借用存续期内该 place 无独占别名——私有面，全部调用点即本模块 `pub unsafe`
/// 门面的入参，前提由门面的 `# Safety` 统一兑现，调用点无需携带 `unsafe`。
#[inline]
fn place_ref_at<'a, B>(node: *mut B) -> Option<&'a B> {
  // Safety: 上述契约——判空先行，非空才建共享借用。
  NonNull::new(node).map(|p| unsafe { p.as_ref() })
}

/// [`place_ref_at`] 的独占形态（契约同，另需调用方可独占该 place）。
#[inline]
fn place_mut_at<'a, B>(node: *mut B) -> Option<&'a mut B> {
  // Safety: 契约即前置条件，判空先行，非空才建独占借用。
  if node.is_null() {
    None
  } else {
    Some(unsafe { &mut *node })
  }
}

/// 基类家族（`AstExpr`/`AstStat`/`AstType`/`AstTypePack`）的成员判别谓词：
/// 四个 `is_*_class` 谓词由 [`crate::visit::ast_node_table`] 的家族位列导出，
/// 这里只做「类型 ↔ 谓词」的一次性接线，供 [`ast_node_as_family`] /
/// [`ast_node_as_family_mut`] 两个下转入口共用。
pub(crate) trait AstFamily {
  /// cpp `node->asX() != nullptr` 的判别面：`class_index` 是否属于本家族。
  fn is_member(class_index: i32) -> bool;
}

impl AstFamily for AstExpr {
  #[inline]
  fn is_member(class_index: i32) -> bool {
    is_expr_class(class_index)
  }
}

impl AstFamily for AstStat {
  #[inline]
  fn is_member(class_index: i32) -> bool {
    is_stat_class(class_index)
  }
}

impl AstFamily for AstType {
  #[inline]
  fn is_member(class_index: i32) -> bool {
    is_type_class(class_index)
  }
}

impl AstFamily for AstTypePack {
  #[inline]
  fn is_member(class_index: i32) -> bool {
    is_type_pack_class(class_index)
  }
}

/// cpp `const AstNode::asExpr()/asStat()/asType()/asTypePack()` 的引用形态：
/// 家族判别命中即给出家族基类的共享视图，不命中折叠为 `None`（cpp 的 null）。
/// 四族共用本单点，取代此前逐族手抄的 `NonNull::new_unchecked(….cast_mut())`
/// 指针往返——从共享引用造裸 `*mut` 再解引用成 `&` 是别名 UB 的高危形态。
#[inline]
pub(crate) fn ast_node_as_family<T: AstFamily>(node: &impl AstNodeView) -> Option<&T> {
  T::is_member(node.as_ast_node().class_index).then(||
    // Safety: 家族判别命中 ⇒ 该 place 的动态类型是 `T` 的派生类；`T` 与各派生类
    // 同为 `#[repr(C)]`、首字段传递地为 `AstNode`，故偏移 0 即 `T` 子对象；共享
    // 借用寿命与只读性由入参引用继承。
    unsafe { ref_cast::<AstNode, T>(node.as_ast_node()) })
}

/// [`ast_node_as_family`] 的独占形态：交出 `NonNull<T>` 而非 `&mut T`——arena 节点
/// 随后仍会经裸指针句柄被写穿（parser/visitor），长命 `&mut` 与既有形态互斥
/// （同 `optional_node` 对 `NonNull` 的取舍）。纯地址改写，零 `unsafe`。
#[inline]
pub(crate) fn ast_node_as_family_mut<T: AstFamily>(
  node: &mut impl AstNodeViewMut,
) -> Option<NonNull<T>> {
  if !T::is_member(node.as_ast_node_mut().class_index) {
    return None;
  }
  Some(NonNull::from(node.as_ast_node_mut()).cast::<T>())
}

/// 指针身份形态：类位命中即交出类型化 [`NonNull`] 指针而非借用，只用于裸指针
/// 传递 / map key / 判等（比身份不取引用），不进借用系统，也就不制造假
/// `'static`。判型命中后指针改写纯为地址类型视图，零解引用承诺由契约给出。
///
/// # Safety
/// `node` 须为 null 或指向存活的 repr(C) AST 节点（首字段传递地为 `AstNode`）。
#[inline]
pub unsafe fn ast_node_try_cast_ptr<T: AstNodeClass>(node: impl AstNodePtr) -> Option<NonNull<T>> {
  // Safety: 契约保证 null 不解引用；非空即存活节点，只读偏移 0 基类的
  // class_index 判型，命中后指针 cast 仅改类型视图不改地址（repr(C) 基址重合），
  // 指针出自存活借用，NonNull 非空性由 place_ref_at 的成功先行兑现。
  place_ref_at(node.as_ast_node())
    .filter(|base| ast_node_is::<T>(*base))
    .map(|base| NonNull::from(base).cast::<T>())
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

/// CST 下转的裸指针边界（crate 内 parser 的 ast→cst 映射独占改写消费；printer
/// 只读侧已改走 safe [`slot_opt`] + [`cst_node_try_as`] 组合），null
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
  // Safety: 契约保证非空即存活且可独占；判型与类型改写收口到独占形态。
  let base = place_mut_at(node)?;
  (base.class_index == T::CLASS_INDEX).then(||
    // Safety: class_index 命中 ⇒ 动态类型为 T，repr(C) 基址重合，独占性继承。
    unsafe { mut_cast::<CstNode, T>(base) })
}
