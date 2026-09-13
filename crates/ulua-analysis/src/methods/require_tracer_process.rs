use ulua_ast::records::{
  ast_expr_group::AstExprGroup, ast_expr_local::AstExprLocal,
  ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode, ast_type_group::AstTypeGroup,
  ast_type_typeof::AstTypeTypeof,
};

use crate::records::{
  file_resolver::FileResolver, module_info::ModuleInfo, require_tracer::RequireTracer,
  type_check_limits::TypeCheckLimits,
};

impl RequireTracer {
  pub fn process(&mut self, limits: &TypeCheckLimits) {
    let module_context = ModuleInfo {
      name: self.current_module_name.clone(),
      optional: false,
    };

    self.work.reserve(self.require_calls.len());
    for &require in &self.require_calls {
      unsafe {
        if (*require).args.size > 0 {
          self.work.push(*(*require).args.data.add(0) as *mut AstNode);
        }
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
          if let Some(context) = context {
            if (*expr).is::<AstExprLocal>()
              || (*expr).is::<AstExprGroup>()
              || (*expr).is::<AstTypeGroup>()
              || (*expr).is::<AstTypeTypeof>()
              || (*expr).is::<AstExprTypeAssertion>()
            {
              info = Some(context.clone());
            } else if let Some(as_expr) = (*expr).as_expr().as_mut() {
              info = FileResolver::resolve_module(self.file_resolver, context, as_expr, limits);
            }
          }
        } else if let Some(as_expr) = (*expr).as_expr().as_mut() {
          info = FileResolver::resolve_module(self.file_resolver, &module_context, as_expr, limits);
        }

        if let Some(info) = info {
          (*self.result).exprs.try_insert(expr, info);
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
          let info_copy = info.clone();
          (*self.result)
            .exprs
            .try_insert(require as *mut AstNode, info_copy);
        } else {
          (*self.result)
            .exprs
            .try_insert(require as *mut AstNode, ModuleInfo::default());
        }
      }
    }
  }
}
