//! 测试侧统一的"裸指针 → 引用"入口。
//!
//! 测试代码里原本散落的三步式（`rtti::ast_node_as::<T>(p)` 下转 + 判空 +
//! `unsafe { &*p }`）在这里收口：null 分支进 `Option`，`unsafe` 只留本模块，
//! 调用点零 `unsafe`。语义与 cpp 的 `node->as<T>()`（失败返回 nullptr）、
//! 裸指针解引用、`array.data[i]` 逐条对应。
//!
//! 指针读法做成扩展 trait 方法而非自由函数：rustc 禁止在裸指针上写 inherent
//! impl（E0390，提示"consider using an extension trait instead"），而 trait
//! 方法的 `&self` receiver 既符合 `as_*` 命名的 `wrong_self_convention` 约定，
//! 也不落进 `not_unsafe_ptr_arg_deref` 的射程（该 lint 只针对安全函数解引用
//! 裸指针*参数*）。下转入口再按节点类型逐个铺开 impl，调用点连 `.cast()` 上转
//! 都不必写：`expr.as_node::<AstExprCall>()` 即 cpp 的 `expr->as<AstExprCall>()`。

use std::ptr::from_ref;

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat,
    ast_type::AstType, ast_type_pack::AstTypePack, cst_node::CstNode,
  },
  rtti::{AstNodeClass, CstNodeClass, ast_node_try_as},
};

/// 可空裸指针的非空读法。
pub trait PtrRef<T> {
  /// cpp 直接解引用 `ptr`：null → `None`。生命周期 `'a` 由调用点的使用范围约束
  /// （arena 内存活期长于所属节点树）。
  fn as_ref_opt<'a>(&self) -> Option<&'a T>;
}

impl<T> PtrRef<T> for *const T {
  #[inline]
  fn as_ref_opt<'a>(&self) -> Option<&'a T> {
    // SAFETY: self 为 arena 内存活对象或 null；'a 由调用点约束。
    unsafe { (*self).as_ref() }
  }
}

impl<T> PtrRef<T> for *mut T {
  #[inline]
  fn as_ref_opt<'a>(&self) -> Option<&'a T> {
    self.cast_const().as_ref_opt()
  }
}

/// AST 节点指针的读法。
pub trait NodePtr {
  /// cpp `node->as<T>()`：下转与非空判定合为一次，类型不符 → `None`。
  fn as_node<'a, T: AstNodeClass>(&self) -> Option<&'a T>;
}

/// 节点指针按类型铺开 impl：`AstNode` + 4 个抽象基类（AST 字段几乎都是
/// `*mut AstExpr`/`*mut AstType` 这样的基类指针）。不用
/// `impl<N: AstNodeClass> NodePtr for *const N` 的 blanket：基类不实现
/// `AstNodeClass`，而 blanket 与基类 impl 会被 coherence 以后向兼容为由判成
/// 重叠（E0119），故宏内逐个列举；拿到具体节点类型指针（`*mut AstExprCall` 等）
/// 的调用点先 `.cast::<AstNode>()` 上转，或直接走 `ast_node_try_as(&*p)`。
macro_rules! impl_node_ptr {
  ($($base:ty),* $(,)?) => {
    $(
      impl NodePtr for *const $base {
        #[inline]
        fn as_node<'a, T: AstNodeClass>(&self) -> Option<&'a T> {
          // SAFETY: 同上——基类同样 base 首字段，与 AstNode 基址重合。
          unsafe { (*self).cast::<AstNode>().as_ref() }.and_then(ast_node_try_as)
        }
      }

      impl NodePtr for *mut $base {
        #[inline]
        fn as_node<'a, T: AstNodeClass>(&self) -> Option<&'a T> {
          self.cast_const().as_node()
        }
      }
    )*
  };
}

impl_node_ptr!(AstNode, AstExpr, AstStat, AstType, AstTypePack);

/// cpp `cstNode->as<T>()` 的引用形态：类标命中才下转（AST 侧对应
/// `rtti::ast_node_try_as`，CST 是独立 RTTI 索引空间，故 ulua-ast 侧只有
/// 裸指针版 `rtti::cst_node_as`）。
pub fn cst_node_try_as<T: CstNodeClass>(node: &CstNode) -> Option<&T> {
  if node.class_index == T::CLASS_INDEX {
    // SAFETY: 动态类型已由 class_index 判定；CST 节点同为 repr(C) 单继承
    // （`base: CstNode` 首字段），基址与 CstNode 重合；引用保证非空与存活。
    Some(unsafe { &*(node as *const CstNode).cast::<T>() })
  } else {
    None
  }
}

/// CST 节点基指针的读法：`cst_node_map` 给出的就是 `*mut CstNode`，故只需基类
/// impl（CST 无 `AstExpr` 那样的中间抽象层）。
pub trait CstNodePtr {
  /// cpp `cstNode->as<T>()`：下转与非空判定合为一次，类型不符 → `None`。
  fn as_cst<'a, T: CstNodeClass>(&self) -> Option<&'a T>;
}

impl CstNodePtr for *const CstNode {
  #[inline]
  fn as_cst<'a, T: CstNodeClass>(&self) -> Option<&'a T> {
    // SAFETY: self 为 arena 内存活 CST 节点或 null；命中 class_index 后 repr(C)
    // 单继承保证 T 与 CstNode 基址重合。
    self.as_ref_opt().and_then(cst_node_try_as)
  }
}

impl CstNodePtr for *mut CstNode {
  #[inline]
  fn as_cst<'a, T: CstNodeClass>(&self) -> Option<&'a T> {
    self.cast_const().as_cst()
  }
}

/// cpp `array.data[i]->as<T>()`：元素为节点指针的数组取下标并下转。
pub fn as_node_at<'a, T: AstNodeClass, U>(array: &AstArray<*mut U>, index: usize) -> Option<&'a T> {
  array.as_slice()[index]
    .cast_const()
    .cast::<AstNode>()
    .as_node()
}

/// cpp `array.data[i]`（元素为指针）：取下标并解引用，null → `None`。
pub fn deref_at<'a, T>(array: &AstArray<*mut T>, index: usize) -> Option<&'a T> {
  array.as_slice()[index].as_ref_opt()
}

/// cpp `array.data[i]`：按下标取元素引用（元素是指针还是值都由调用点决定）。
/// 下标越界直接 panic，与测试断言语义一致。
pub fn elem<T>(array: &AstArray<T>, index: usize) -> &T {
  // SAFETY: array 的 data/size 由 arena 保证为有效区间，存活期长于所属节点树。
  unsafe { array.as_slice().get_unchecked(index) }
}

/// 以节点地址作 map key 时（如 `RequireTraceResult::exprs`）由引用回推指针。
pub fn node_key<T>(node: &T) -> *mut AstNode {
  from_ref(node).cast::<AstNode>().cast_mut()
}
