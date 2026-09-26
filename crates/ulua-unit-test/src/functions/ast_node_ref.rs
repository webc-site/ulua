//! 测试侧统一的"裸指针 → 引用"入口。
//!
//! 测试代码里原本散落的三步式（`rtti::ast_node_try_as*` 下转 + 判空 +
//! `unsafe { &*p }`）在这里收口：null 分支进 `Option`，`unsafe` 只留本模块，
//! 调用点零 `unsafe`。语义与 cpp 的 `node->as<T>()`（失败返回 nullptr）、
//! 裸指针解引用、`array.data[i]` 逐条对应。
//!
//! 生命周期约定：所有辅助的返回引用都由**入参借用**推导（`&self` 的省略生命周期
//! 或显式 `&'a AstArray`），不接受调用点凭空给出的 `'a`。指针槽本身位于
//! `Fixture`/`Allocator` 持有的 arena 内存里，槽借用存活 ⟹ 所属 arena 存活，
//! 于是返回引用的有效期被真实约束在 fixture 生命周期内；想把它塞进 `'static`
//! 位置会直接编译失败。下转本身统一委托 `ulua_ast::rtti::ast_node_try_as`，
//! 本模块不再自行比对 `class_index`。
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
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_node::AstNode,
    ast_stat::AstStat,
    ast_type::AstType,
    ast_type_pack::AstTypePack,
    cst_node::CstNode,
    node_handle::{Node, Nodes, OptNode},
  },
  rtti::{AstNodeClass, AstNodeView, CstNodeClass, ast_node_try_as, cst_node_try_as},
};

/// 本模块唯一的“裸指针槽 → 共享引用”物化入口（判空 + 解引用一体）。
///
/// # Safety
/// `p` 为空，或指向在返回引用的整个存续期内存活的 `T`，且该存续期内无该对象的
/// 独占借用。测试侧契约：`p` 取自 `Fixture`/`Allocator` 持有的 arena 指针槽，
/// 槽借用存活 ⟹ arena 存储不动（见模块头生命周期约定）。
#[inline]
unsafe fn view<'a, T>(p: *const T) -> Option<&'a T> {
  // Safety: 转调即函数级 `# Safety` 契约本身：null 折叠为 `None` 不解引用，
  // 非空按契约存活。
  unsafe { p.as_ref() }
}

/// [`view`] 的独占形态，对应 cpp `node->visit(AstVisitor*)` 写穿节点的 `this`。
///
/// # Safety
/// 同 [`view`]，另需调用方持有该 place 在返回引用存续期内的独占借用
/// （`&mut` 槽借用在测试内逐个遍历即满足）。
#[inline]
unsafe fn view_mut<'a, T>(p: *mut T) -> Option<&'a mut T> {
  // Safety: 转调即函数级 `# Safety` 契约本身。
  unsafe { p.as_mut() }
}

/// 可空裸指针的非空读法。
pub trait PtrRef<T> {
  /// cpp 直接解引用 `ptr`：null → `None`。返回引用借用指针槽，故不会比槽所在
  /// 的 arena 内存活得更久。
  fn as_ref_opt(&self) -> Option<&T>;
}

impl<T> PtrRef<T> for *const T {
  #[inline]
  fn as_ref_opt(&self) -> Option<&T> {
    // Safety: 输出生命周期即 `&self`（槽借用）给出的 `'a`，满足 [`view`] 契约
    // 的“槽存活 ⟹ arena 存活”测试侧约定。
    unsafe { view(*self) }
  }
}

impl<T> PtrRef<T> for *mut T {
  #[inline]
  fn as_ref_opt(&self) -> Option<&T> {
    // Safety: 同 `*const T` 分支——共享读法经 [`view`]，`*mut` 隐式降为 `*const`。
    unsafe { view(*self) }
  }
}

/// 可空裸指针的非空**独占**读法：cpp `AstNode::visit(AstVisitor*)` 的 `this` 非
/// const，遍历期间 visitor 会写穿节点，故测试侧跑 visitor 时要的是 `&mut`。
/// 与 [`PtrRef`] 分成两个 trait：`*const T` 无法给出独占借用。
pub trait PtrMutRef<T> {
  /// cpp 直接解引用 `ptr` 并允许写入：null → `None`。返回的独占借用同样由槽的
  /// 借用期给出，另需该 arena 在借用期内无其他并发借用（测试内逐个遍历即满足）。
  fn as_mut_ref_opt(&mut self) -> Option<&mut T>;
}

impl<T> PtrMutRef<T> for *mut T {
  #[inline]
  fn as_mut_ref_opt(&mut self) -> Option<&mut T> {
    // Safety: 槽独占借用（`&mut self`）即 [`view_mut`] 契约的独占性证明，
    // 存活约定同 [`PtrRef`]。
    unsafe { view_mut(*self) }
  }
}

/// AST 节点指针的读法。
pub trait NodePtr {
  /// cpp `node->as<T>()`：下转与非空判定合为一次，类型不符 → `None`。
  /// 返回引用的生命周期由 `&self`（指针槽借用）给出。
  fn as_node<T: AstNodeClass>(&self) -> Option<&T>;
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
        fn as_node<T: AstNodeClass>(&self) -> Option<&T> {
          // Safety: 基类与 `AstNode` 同为 repr(C) 单继承的首字段，基址重合，
          // 槽指向的对象按 [`view`] 契约在槽借用期内存活；判型与下转交给安全版
          // `ast_node_try_as`，其输出生命周期即此处由槽借用给出的匿名字命期。
          unsafe { view(self.cast::<AstNode>()) }.and_then(ast_node_try_as)
        }
      }

      impl NodePtr for *mut $base {
        #[inline]
        fn as_node<T: AstNodeClass>(&self) -> Option<&T> {
          // Safety: 同 `*const $base` 分支——`*mut` 隐式降为 `*const` 后经
          // [`view`] 物化，判空与下转分别由 null 折叠和安全版 `ast_node_try_as`
          // 完成。
          unsafe { view(self.cast::<AstNode>()) }.and_then(ast_node_try_as)
        }
      }
    )*
  };
}

impl_node_ptr!(AstNode, AstExpr, AstStat, AstType, AstTypePack);

/// 句柄形态的非空读法：records 波次把 `Node`/`OptNode` 槽位顶替裸指针后，
/// 调用点沿用 `as_ref_opt()/as_node::<T>()` 门面，判空与下转语义逐字对应。
impl<T> PtrRef<T> for Node<T> {
  #[inline]
  fn as_ref_opt(&self) -> Option<&T> {
    Some(self.get())
  }
}

impl<T> PtrRef<T> for OptNode<T> {
  #[inline]
  fn as_ref_opt(&self) -> Option<&T> {
    self.get()
  }
}

impl<T: AstNodeView> NodePtr for Node<T> {
  #[inline]
  fn as_node<U: AstNodeClass>(&self) -> Option<&U> {
    ast_node_try_as(self.get())
  }
}

impl<T: AstNodeView> NodePtr for OptNode<T> {
  #[inline]
  fn as_node<U: AstNodeClass>(&self) -> Option<&U> {
    self.get().and_then(|node| ast_node_try_as(node))
  }
}

/// 具体基类的引用形态读法：`AstTypeOrPack::as_type()` 等给出的已是 arena 存活
/// 引用（非裸指针槽），调用点直接链式 `as_node` 下转，判型语义与指针形态一致。
impl NodePtr for AstType {
  #[inline]
  fn as_node<T: AstNodeClass>(&self) -> Option<&T> {
    // repr(C) 单继承使 `base` 与 `self` 基址重合，下转判定收口到安全版门面。
    ast_node_try_as(&self.base)
  }
}

/// CST 节点基指针的读法：`cst_node_map` 给出的就是 `*mut CstNode`，故只需基类
/// impl（CST 无 `AstExpr` 那样的中间抽象层）。CST 是独立 RTTI 索引空间，下转走
/// ulua-ast 侧的 `cst_node_try_as`（与 AST 侧 `ast_node_try_as` 对应）。
pub trait CstNodePtr {
  /// cpp `cstNode->as<T>()`：下转与非空判定合为一次，类型不符 → `None`。
  fn as_cst<T: CstNodeClass>(&self) -> Option<&T>;
}

impl CstNodePtr for *const CstNode {
  #[inline]
  fn as_cst<T: CstNodeClass>(&self) -> Option<&T> {
    self.as_ref_opt().and_then(cst_node_try_as)
  }
}

impl CstNodePtr for *mut CstNode {
  #[inline]
  fn as_cst<T: CstNodeClass>(&self) -> Option<&T> {
    // Safety: 同 `*const CstNode` 分支——`*mut` 降为 `*const` 后经 [`view`]
    // 物化，槽指向的 `CstNode` 由 CST allocator 保活。
    unsafe { view(*self) }.and_then(cst_node_try_as)
  }
}

/// "指针槽数组"的下标视图：未迁移的 `AstArray<*mut U>` 与句柄化后的
/// `Nodes<U>` 在测试里同形（cpp `array.data[i]`），按下标统一折回裸指针槽。
pub trait NodeSlots<U> {
  /// cpp `array.data[index]`：返回该槽存的节点指针（可能为 null，由调用方门面
  /// 折叠）。引用的借用期即槽所在数组的存活证明，与两个既有助手同一约定。
  fn slot_ptr(&self, index: usize) -> *mut U;
}

impl<U> NodeSlots<U> for AstArray<*mut U> {
  #[inline]
  fn slot_ptr(&self, index: usize) -> *mut U {
    self[index]
  }
}

impl<U> NodeSlots<U> for Nodes<U> {
  #[inline]
  fn slot_ptr(&self, index: usize) -> *mut U {
    self.at(index).as_ptr()
  }
}

/// cpp `array.data[i]->as<T>()`：元素为节点指针的数组取下标并下转。
/// 返回引用的生命周期由 `array` 的借用给出。
pub fn as_node_at<T: AstNodeClass, U>(array: &dyn NodeSlots<U>, index: usize) -> Option<&T> {
  // Safety: `array` 本体在 arena 内，借用存活 ⟹ 其元素指针指向的节点
  // （同一 arena）对该借用期存活（[`view`] 契约的测试侧约定），判空由
  // null 折叠兑现，判型与下转收口到安全版 `ast_node_try_as`。
  unsafe { view(array.slot_ptr(index).cast_const().cast::<AstNode>()) }.and_then(ast_node_try_as)
}

/// cpp `array.data[i]`（元素为指针）：取下标并解引用，null → `None`。
/// 返回引用的生命周期由 `array` 的借用给出。
pub fn deref_at<T>(array: &dyn NodeSlots<T>, index: usize) -> Option<&T> {
  // Safety: 同 [`as_node_at`]——元素指针为 null 时折叠为 `None` 不解引用；
  // 非空时指向与 `array` 同 arena 的存活 `T`，物化的只读借用与 cpp
  // `array.data[i]` 解引用同形。
  unsafe { view(array.slot_ptr(index).cast_const()) }
}

/// cpp `array.data[i]`：按下标取元素引用（元素是指针还是值都由调用点决定）。
/// 下标越界直接 panic，与测试断言语义一致。
pub fn elem<T>(array: &AstArray<T>, index: usize) -> &T {
  &array[index]
}

/// 以节点地址作 map key 时（如 `RequireTraceResult::exprs`）由引用回推指针。
pub fn node_key<T>(node: &T) -> *mut AstNode {
  from_ref(node).cast::<AstNode>().cast_mut()
}
