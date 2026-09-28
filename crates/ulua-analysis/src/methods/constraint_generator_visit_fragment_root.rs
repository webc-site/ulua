use alloc::vec::Vec;

use ulua_ast::records::{ast_stat_block::AstStatBlock, location::Location};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{as_mutable_type::as_mutable_type_id, follow_type, get_type},
  records::{
    arena_handle::{alias, alias_ref},
    blocked_type::BlockedType,
    constraint_generator::ConstraintGenerator,
    interior_free_types::InteriorFreeTypes,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_variant::TypeVariant},
};

impl ConstraintGenerator {
  /// `visitModuleRoot`/`visitFragmentRoot` 共同的 localTypes 泛化收尾
  /// （cpp 两函数尾部同体）：对每个 BlockedType 域求 never-union 并把该槽
  /// 覆写为 Bound(domainTy)。
  pub(crate) fn generalize_local_types(&mut self, scope: &ScopePtr, location: Location) {
    let local_types_pairs: Vec<(TypeId, Vec<TypeId>)> = self
      .local_types
      .iter()
      .map(|(ty, domain)| (*ty, domain.order.clone()))
      .collect();
    for (ty, domain) in local_types_pairs {
      // FIXME: This isn't the most efficient thing.
      // Safety: builtin_types 是构造期接线的 *mut BuiltinTypes——非空、比生成器
      // 长寿、类型检查期不再写入，这里只读 never_type 的 Copy 句柄。
      let mut domain_ty = self.builtin_types.get().never_type;
      for d in domain {
        let d_followed = follow_type::follow(d);
        if d_followed == ty {
          continue;
        }
        domain_ty = self.simplify_union(scope.clone(), location, domain_ty, d_followed);
      }

      LUAU_ASSERT!(get_type::get::<BlockedType>(ty).is_some());
      // Safety: `ty` 取自上方 local_types 快照的 arena 句柄——TypeId 约定非空且
      // 对齐；紧邻的 LUAU_ASSERT 经 get_type_id 证明其当前变体仍是 BlockedType，
      // 即节点存活。此处把 Blocked 覆写为 Bound 是对非持久化自有节点的就地更新
      // （bump 块地址不移动），此刻无任何并存借用指向该节点：simplify_union 的
      // 新节点追加已返回，domain 向量是先前 clone 的快照。
      alias(as_mutable_type_id(ty)).ty = TypeVariant::Bound(domain_ty);
    }
  }

  // ConstraintGenerator::visitFragmentRoot (ConstraintGenerator.cpp).
  /// `block` 为本 fragment 的根 `AstStatBlock` 裸指针入口（记录字段直传边界），
  /// 经 alias_ref 收口为共享引用后全程只读。
  pub(crate) fn visit_fragment_root(&mut self, resume_scope: &ScopePtr, block: *mut AstStatBlock) {
    // We prepopulate global data in the resumeScope to avoid writing data into the old modules scopes
    let global_scope = self.global_scope.clone().unwrap();
    let block_ref = alias_ref(block);
    self.prepopulate_global_scope_for_fragment_typecheck(&global_scope, resume_scope, block_ref);
    // Pre
    self.interior_free_types.push(InteriorFreeTypes::default());
    // resume_scope 为调用方借出的存活 Arc；block 为会话存活的 parse arena
    // 节点。visit_block_without_child_scope 只经 scope 改 Scope 自有表，
    // 与 &mut self 驱动的遍历状态按调用时序串接。
    self.visit_block_without_child_scope(resume_scope, block_ref);
    // Post
    self.interior_free_types.pop();

    self.fill_in_inferred_bindings(resume_scope, block_ref);

    if !self.logger.is_null() {
      // 上方显式判空；logger 由 typecheck 入口接线为会话存活的 DcrLogger
      // （记录布局不改），解引用经 alias 收口；capture 仅接收 module
      // 的 Arc 克隆，不写生成器状态。
      alias(self.logger).capture_generation_module(self.module.clone().unwrap());
    }

    self.generalize_local_types(resume_scope, resume_scope.as_ref().location);
  }
}
