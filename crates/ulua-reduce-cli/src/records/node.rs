//! arena AST 节点句柄（review.md §2 的「arena + 句柄」形态）。
//!
//! AST 节点由 `Reducer` 持有的 `Box<Allocator>` 页分配、随 `Reducer` 一起释放，
//! 天然满足「句柄在 Reducer 借用期内存活」。句柄本身只是**数据坐标**
//! （与 cpp 的 `AstStatBlock*` 同构，`Copy`），不做任何解引用操作；解引用集中
//! 在 `get`/`get_mut` 两个门面——本 crate 唯一的节点访问 unsafe 站点。
//!
//! 契约（全体句柄构造点共同遵守）：
//! - 句柄只从 `Parser::parse` 的产出（`Node::new` 折叠空指针）、ulua-ast
//!   句柄化字段的 `get` 引用（`from_ref`/`From<&mut T>`），或 visitor 交回的
//!   可变借用收编而来，均指向存活节点（单线程、`Reducer` 独占整个 arena）；
//! - 节点写操作仅发生在 `Reducer::try_body` 的临时替换窗口与
//!   `write_temp_script` 的打印窗口，窗口结束后引用即收敛；同一节点上
//!   不存在并发的读写借用。

use core::ptr::NonNull;

use ulua_ast::{
  enums::ast_stat_ref::AstStatRef,
  records::{ast_stat::AstStat, ast_stat_block::AstStatBlock},
};

/// arena 节点句柄。`#[repr(transparent)]` 保证与裸指针 `*mut T` 布局一致，
/// 与 ulua-ast 侧的句柄同一坐标值互转（经 `get` 引用桥接，见 `Reducer::try_body`）。
#[derive(Debug)]
#[repr(transparent)]
pub struct Node<T> {
  raw: NonNull<T>,
}

// 句柄是坐标值：复制/比较只看地址，不要求 `T` 实现相应 trait（derive 会加
// `T: Copy` 之类不可能的边界，故手写）。
impl<T> Clone for Node<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T> Copy for Node<T> {}

impl<T> PartialEq for Node<T> {
  fn eq(&self, other: &Self) -> bool {
    self.raw == other.raw
  }
}

impl<T> Eq for Node<T> {}

impl<T> Node<T> {
  /// 从 arena 地址包装句柄：空指针（cpp 用它表达「无此分支/未落位」）随类型
  /// 一并收敛为 `None`，杜绝 null 哨兵。
  pub fn new(raw: *mut T) -> Option<Self> {
    NonNull::new(raw).map(|raw| Node { raw })
  }

  /// 由存活共享引用建槽（ulua-ast 字段句柄化后的读面桥接：
  /// `Node/OptNode/Nodes` 的 `get` 给出引用，引用即非空 + 存活证明，safe）。
  pub fn from_ref(node: &T) -> Self {
    Node {
      raw: NonNull::from(node),
    }
  }

  /// 节点的只读视图。
  pub fn get(&self) -> &T {
    // Safety: 契约见模块头——`raw` 指向 arena 存活节点，读取窗口内该节点无
    // 并发借用（全部写操作发生在 `get` 借用收敛之后）。
    unsafe { self.raw.as_ref() }
  }

  /// 节点的可变视图。仅 `Reducer::try_body`（body 试替换/提交/回滚）与
  /// `write_temp_script`（pretty printer 按 cpp 非 const 语义收 `&mut`）调用。
  pub fn get_mut(&mut self) -> &mut T {
    // Safety: 契约见模块头；调用方保证借用窗口内同一节点无其它存活借用。
    unsafe { self.raw.as_mut() }
  }
}

impl<'a, T> From<&'a mut T> for Node<T> {
  /// 从可变借用收编句柄（visitor 的 `visit(&mut AstStatBlock)` 记录坐标用）。
  fn from(node: &'a mut T) -> Self {
    Node {
      raw: NonNull::from(node),
    }
  }
}

/// 语句块句柄。
pub type Block = Node<AstStatBlock>;
/// 语句句柄。
pub type Stat = Node<AstStat>;

impl Node<AstStat> {
  /// 将语句句柄下转为具体引用枚举。
  #[inline]
  pub fn as_stat_ref(&self) -> AstStatRef<'_> {
    self.get().as_stat_ref()
  }

  /// 尝试将语句句柄下转为具体引用枚举。
  #[inline]
  pub fn try_as_stat_ref(&self) -> Option<AstStatRef<'_>> {
    self.get().try_as_stat_ref()
  }
}
