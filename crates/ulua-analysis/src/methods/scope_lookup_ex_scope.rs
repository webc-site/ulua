use crate::records::{
  binding::Binding, scope::Scope, scope_registry::resolve_scope, symbol::Symbol,
};

impl Scope {
  /// 沿 parent 链查找 sym，返回（绑定，定义处作用域）。
  /// cpp/Scope.cpp:32 用 const_cast 取 &mut；本实现真只读（&self），无 unsafe。
  pub fn lookup_ex_symbol(&self, sym: Symbol) -> Option<(&Binding, &Scope)> {
    let mut cur = self;
    loop {
      if let Some(binding) = cur.bindings.get(&sym) {
        return Some((binding, cur));
      }
      cur = cur.parent.and_then(resolve_scope)?;
    }
  }
}
