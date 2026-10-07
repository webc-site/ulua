//! `refine_type_scrubber` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{
  functions::{follow_type, get_type},
  macros::{substitution_entry::substitution_entry, substitution_vtable},
  records::{
    arena_handle::{Handle, alias, alias_ref},
    intersection_type::IntersectionType,
    never_type::NeverType,
    refine_type_scrubber::RefineTypeScrubber,
    substitution::Substitution,
    txn_log::TxnLog,
    type_function_context::TypeFunctionContext,
    type_ids::TypeIds,
    union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl RefineTypeScrubber {
  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    tp
  }

  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    // NOTE: this feels pretty similar to other places where we try to
    // filter over a set type, may be worth combining those in the future.
    // self.ctx 由 `RefineTypeScrubber::new` 以真实 `&mut` 借用接线为 `Handle`
    // （类型编码非空），指向类型函数归约会话存活的 TypeFunctionContext（solver/runtime
    // 持有，clean 遍历期不移动不释放），共享借用只读其字段。
    let ctx_ref = self.ctx.get();
    let builtins = alias_ref(ctx_ref.builtins.as_ptr());

    if let Some(ut) = get_type::get::<UnionType>(ty) {
      let mut new_options = TypeIds::new();
      for &option in &ut.options {
        let followed = follow_type::follow(option);
        if followed != self.needle && get_type::get::<NeverType>(followed).is_none() {
          new_options.insert_type_id(option);
        }
      }
      if new_options.empty() {
        builtins.never_type
      } else if new_options.size() == 1 {
        new_options.front()
      } else {
        alias(ctx_ref.arena.as_ptr()).add_type(UnionType {
          options: new_options.take(),
        })
      }
    } else if let Some(it) = get_type::get::<IntersectionType>(ty) {
      let mut new_parts = TypeIds::new();
      for &part in &it.parts {
        let followed = follow_type::follow(part);
        if followed != self.needle && get_type::get::<UnknownType>(followed).is_none() {
          new_parts.insert_type_id(part);
        }
      }
      if new_parts.empty() {
        builtins.unknown_type
      } else if new_parts.size() == 1 {
        new_parts.front()
      } else {
        alias(ctx_ref.arena.as_ptr()).add_type(IntersectionType {
          parts: new_parts.take(),
        })
      }
    } else if ty == self.needle {
      builtins.unknown_type
    } else {
      ty
    }
  }
}

impl RefineTypeScrubber {
  pub fn ignore_children_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    false
  }

  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    let is_union = get_type::get::<UnionType>(ty).is_some();
    let is_intersection = get_type::get::<IntersectionType>(ty).is_some();
    !(is_union || is_intersection)
  }
}

impl RefineTypeScrubber {
  pub fn is_dirty_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    false
  }

  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    if let Some(ut) = get_type::get::<UnionType>(ty) {
      for &option in &ut.options {
        if option == self.needle {
          return true;
        }
      }
    } else if let Some(it) = get_type::get::<IntersectionType>(ty) {
      for &part in &it.parts {
        if part == self.needle {
          return true;
        }
      }
    }
    ty == self.needle
  }
}

// C++ `RefineTypeScrubber::RefineTypeScrubber(NotNull<TypeFunctionContext> ctx,
// TypeId needle)` (BuiltinTypeFunctions.cpp:1083-1088). Base-inits the
// `Substitution` with `ctx->arena`, then stores `ctx` and `needle`.

// TypePackId 侧三覆写槽全为真实转发（见 substitution_vtable 模块文档的统一安全论证）。
substitution_vtable!(real, RefineTypeScrubber, ic = ignore_children_type_id);
impl RefineTypeScrubber {
  /// C++ `RefineTypeScrubber` ctor（BuiltinTypeFunctions.cpp:1083-1088）：以
  /// `ctx->arena` 基初始化 `Substitution`，并记下 `ctx` 与要剔除的 `needle`。
  ///
  /// 前置契约（本函数体经 safe 门面完成指针借用，无 unsafe 操作；以下为文档约定）
  /// - `ctx`：本次归约调用的独占借用，指向存活 [`TypeFunctionContext`]——本 ctor
  ///   立即读取其 `arena` 字段，且 `Substitution` 与后续所有覆写都会继续经它读写 arena。
  /// - `needle`：须是 `ctx.arena` 内一个存活的 `TypeId`（调用方传入的 instance 句柄）；
  ///   它只被按值比较，不被解引用。
  /// - 返回对象的 `ctx` 字段是 `Handle`（无生命周期追踪），其有效性完全继承自上面的
  ///   `ctx` 前提；使用该对象期间 `ctx` 所指记录不得失效、不得有并存改写。
  pub(crate) fn new(ctx: &mut TypeFunctionContext, needle: TypeId) -> Self {
    // Safety: 契约给出 ctx 为存活独占借用；此处只读其 arena 字段值（NonNull 拷贝）
    // 交给基类 ctor，不写该记录，也不延长这次共享再借用的生命周期。
    let ctx_ref = &*ctx;
    let base =
      Substitution::substitution_new(TxnLog::empty(), Some(Handle::from_nonnull(ctx_ref.arena)));
    RefineTypeScrubber {
      base,
      ctx: Handle::from_mut(ctx),
      needle,
    }
  }

  substitution_entry!(id);
}
