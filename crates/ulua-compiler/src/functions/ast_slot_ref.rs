//! arena 接线的 AST 槽位只读门面：把 cpp 的「判空 + 解引用」两步样板收口一处。

use core::ptr::NonNull;

use ulua_ast::rtti::{
  AstNodeClass, AstNodeView, AstNodeViewMut, ast_node_try_as, ast_node_try_as_mut,
};

/// 解析一个由 parser 写入的节点槽位：null 归一为 `None`（对应 cpp 的 `if (!p)`
/// 早退分支），非空归一为共享借用。
///
/// 契约（本 crate 的 AST 槽位统一满足）：非空槽指向 ulua-parser 在 parse arena
/// 里接线、长寿于本次编译的节点——编译入口 `compile_or_throw` 持有 arena 至返回，
/// 期间 visitor/compiler 对这些节点只读，故升为共享引用不产生写者并发；返回的
/// `&'a T` 半径由调用方按使用点推断，不得跨出本次编译留存。
pub(crate) fn ast_slot_ref<'a, T>(slot: *mut T) -> Option<&'a T> {
  // Safety: `NonNull::new` 已排除 null，余下即上面的槽位存活契约。
  NonNull::new(slot).map(|ptr| unsafe { ptr.as_ref() })
}

/// [`ast_slot_ref`] + 安全下转 [`ast_node_try_as`] 的一步形态：null 与动态类型
/// 不匹配一并折叠为 `None`；生命周期由入参借用供给，不产出假 `'static`。
#[inline]
pub(crate) fn ast_slot_try_as<'a, T: AstNodeClass, V: AstNodeView + 'a>(
  slot: *mut V,
) -> Option<&'a T> {
  ast_slot_ref(slot).and_then(|node| ast_node_try_as::<T>(node.as_ast_node()))
}

/// [`ast_slot_try_as`] 的纯判型形态（不外传借用）：只读 class_index，null 折叠
/// 为 `false`。
#[inline]
pub(crate) fn ast_slot_is<T: AstNodeClass, V: AstNodeView>(slot: *mut V) -> bool {
  ast_slot_try_as::<T, V>(slot).is_some()
}

/// 槽位的独占可变形态：null 归一 `None`，非空归一可变借用。
///
/// 契约：在返回的借用区间内，调用方是该节点子树的唯一写者（cpp 非 const
/// 形参透传的同前提）；编译期对节点临时字段的写穿全部收口于此。
pub(crate) fn ast_slot_mut_ref<'a, T>(slot: *mut T) -> Option<&'a mut T> {
  // Safety: `NonNull::new` 已排除 null，余下即上面的槽位存活 + 独占写契约。
  NonNull::new(slot).map(|mut ptr| unsafe { ptr.as_mut() })
}

/// [`ast_slot_mut_ref`] + 可变下转 [`ast_node_try_as_mut`] 的一步形态：替代只读
/// 门面不够用、需写穿节点时的 `unsafe` 假 `'static` 可变借用形态（该门面已退役）。
#[inline]
pub(crate) fn ast_slot_try_as_mut<'a, T: AstNodeClass, V: AstNodeViewMut + 'a>(
  slot: *mut V,
) -> Option<&'a mut T> {
  ast_slot_mut_ref(slot).and_then(|node| ast_node_try_as_mut::<T>(node.as_ast_node_mut()))
}
