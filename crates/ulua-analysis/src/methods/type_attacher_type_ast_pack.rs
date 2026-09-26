use core::mem::size_of;

use ulua_ast::records::{ast_array::AstArray, ast_type::AstType};

use crate::{
  functions::flatten_type_pack::flatten_type_pack_id,
  records::{
    type_attacher::TypeAttacher, type_rehydration_options::TypeRehydrationOptions,
    type_rehydration_visitor::TypeRehydrationVisitor,
  },
  type_aliases::{synthetic_names::SyntheticNames, type_pack_id::TypePackId},
};
impl TypeAttacher {
  pub fn type_ast_pack(&mut self, r#type: TypePackId) -> AstArray<*mut AstType> {
    let (v, _tail) = flatten_type_pack_id(r#type);

    let size = v.len();
    // Safety: `self.allocator` 由 `TypeAttacher` 构造点以 `arc_as_mut(&source.allocator)`
    // 接线自 `Arc<Allocator>` 存活句柄（NotNull 语义，attach 全程持有）；
    // `Allocator::allocate` 恒返回非空、≥8 对齐的块首址，`*mut AstType` 元素对齐
    // ≤8，且 bump arena 的块此后不移动——size 为 0 时返回页内地址但无人解引用。
    let data =
      unsafe { (*self.allocator).allocate(size * size_of::<*mut AstType>()) as *mut *mut AstType };

    for (index, item) in v.iter().enumerate() {
      // C++ `result.data[i] = Luau::visit(TypeRehydrationVisitor(allocator, &synthetic_names), v[i]->ty);`
      let mut rehydrator =
        TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
          self.allocator,
          &mut self.synthetic_names as *mut SyntheticNames,
          &TypeRehydrationOptions::default(),
        );
      // `visit_type` 已降 safe（变体读取收口于 `type_variant_of`）；`*item` 是
      // flatten 出的存活 TypeId，满足其 arena 契约。
      let ast_type = rehydrator.visit_type(*item);
      // Safety: `data` 是上方刚切出的 `size` 个元素连续块（非空、arena 不移动），
      // `index` 由 `v.iter().enumerate()` 供给、恒 `< size`，写入均在块界内。
      unsafe {
        *data.add(index) = ast_type;
      }
    }

    AstArray { data, size }
  }
}
