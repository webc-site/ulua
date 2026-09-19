use ulua_ast::records::{
  ast_expr_group::AstExprGroup, ast_expr_local::AstExprLocal,
  ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode, ast_type_group::AstTypeGroup,
  ast_type_typeof::AstTypeTypeof,
};

use crate::records::{
  module_info::ModuleInfo, require_tracer::RequireTracer, type_check_limits::TypeCheckLimits,
};

impl RequireTracer<'_> {
  pub fn process(&mut self, limits: &TypeCheckLimits) {
    let module_context = ModuleInfo {
      name: self.current_module_name.clone(),
      optional: false,
    };

    self.work.reserve(self.require_calls.len());
    unsafe {
      for &require in &self.require_calls {
        // cpp 由 visit(AstExprCall*) 的 `args.size >= 1` 保证首参存在
        self.work.push(*(*require).args.data.add(0) as *mut AstNode);
      }
    }

    let mut i = 0;
    while i < self.work.len() {
      let node = self.work[i];
      if !node.is_null() {
        let dep = { unsafe { self.get_dependent(node) } };
        if !dep.is_null() {
          self.work.push(dep);
        }
      }
      i += 1;
    }

    unsafe {
      let mut i = self.work.len();
      while i > 0 {
        let expr = self.work[i - 1];
        i -= 1;

        if (*self.result).exprs.find(&expr).is_some() {
          continue;
        }

        let mut info: Option<ModuleInfo> = None;
        let dep = self.get_dependent(expr);

        if !dep.is_null() {
          let context = (*self.result).exprs.find(&dep);
          // cpp 的 if/else-if 链：有上下文时透传类型节点，否则交由 resolver
          // 处理（context 可能为空，对应 cpp 传入 nullptr）。
          if context.is_some()
            && ((*expr).is::<AstExprLocal>()
              || (*expr).is::<AstExprGroup>()
              || (*expr).is::<AstTypeGroup>()
              || (*expr).is::<AstTypeTypeof>()
              || (*expr).is::<AstExprTypeAssertion>())
          {
            info = context.cloned();
          } else {
            let as_expr = (*expr).as_expr();
            if !as_expr.is_null() {
              info = self
                .file_resolver
                .resolve_module(context, &*as_expr, limits);
            }
          }
        } else {
          let as_expr = (*expr).as_expr();
          if !as_expr.is_null() {
            info = self
              .file_resolver
              .resolve_module(Some(&module_context), &*as_expr, limits);
          }
        }

        if let Some(info) = info {
          (*self.result).exprs.insert(expr, info);
        }
      }

      (*self.result)
        .require_list
        .reserve(self.require_calls.len());
      for &require in &self.require_calls {
        let arg = *(*require).args.data.add(0) as *mut AstNode;
        if let Some(info) = (*self.result).exprs.find(&arg) {
          (*self.result)
            .require_list
            .push((info.name.clone(), (*require).base.base.location));
          // cpp `result.exprs[require] = std::move(infoCopy)`：先取出副本再写入，
          // 因为写入会使 info 失效。
          let info_copy = info.clone();
          (*self.result)
            .exprs
            .insert(require as *mut AstNode, info_copy);
        } else {
          // cpp: mark require as unresolved
          (*self.result)
            .exprs
            .insert(require as *mut AstNode, ModuleInfo::default());
        }
      }
    }
  }
}
