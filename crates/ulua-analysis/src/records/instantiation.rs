use crate::records::{
  arena_handle::{Handle, alias_opt},
  builtin_types::BuiltinTypes,
  replace_generics::ReplaceGenerics,
  scope::Scope,
  substitution::Substitution,
  type_level::TypeLevel,
};

#[derive(Debug, Clone)]
pub struct Instantiation {
  pub base: Substitution,
  pub builtin_types: Handle<BuiltinTypes>,
  pub level: TypeLevel,
  /// cpp 即可空 `Scope*`（旧 solver 路径传 nullptr）；null 哨兵语义由
  /// [`Instantiation::scope_opt_ref`] 以 `Option<&Scope>` 表达。
  pub scope: *mut Scope,
  pub reusable_replace_generics: ReplaceGenerics,
}

impl Instantiation {
  /// `scope` 字段的收口读取：null 折叠为 `None`（cpp `Scope*` 可空语义）。
  pub(crate) fn scope_opt_ref(&self) -> Option<&'static Scope> {
    alias_opt(self.scope)
  }
}
