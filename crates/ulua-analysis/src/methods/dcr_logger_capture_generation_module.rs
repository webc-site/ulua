use crate::{
  records::{
    annotation_types_at_location::AnnotationTypesAtLocation, dcr_logger::DcrLogger,
    expr_types_at_location::ExprTypesAtLocation,
  },
  type_aliases::module_ptr_module::ModulePtr,
};

impl DcrLogger {
  pub fn capture_generation_module(&mut self, module: ModulePtr) {
    let module_ref = &*module;

    self
      .generation_log
      .expr_type_locations
      .reserve(module_ref.ast_types.size());
    for (expr, ty) in module_ref.ast_types.iter() {
      let expr = *expr;
      let mut tys = ExprTypesAtLocation {
        // Safety: `expr` 是 `module_ref.ast_types` 的键——parse arena 分配、由同一
        // Module 内 `allocator: Arc<Allocator>` 保活的存活 AstExpr 句柄，`module_ref`
        // 的 Arc 借用覆盖整个循环；repr(C) 首字段基址重合，仅拷出 `base.location` 值。
        location: unsafe { (*expr).base.location },
        ty: *ty,
        expected_ty: None,
      };

      if let Some(expected_ty) = module_ref.ast_expected_types.find(&expr) {
        tys.expected_ty = Some(*expected_ty);
      }

      self.generation_log.expr_type_locations.push(tys);
    }

    self
      .generation_log
      .annotation_type_locations
      .reserve(module_ref.ast_resolved_types.size());
    for (annot, ty) in module_ref.ast_resolved_types.iter() {
      let annot = *annot;
      let tys = AnnotationTypesAtLocation {
        // Safety: 同上——`annot` 为 ast_resolved_types 的存活 AstType arena 键，
        // Arc 借用覆盖循环，仅按 repr(C) 基址重合只读拷出 `base.location`。
        location: unsafe { (*annot).base.location },
        resolved_ty: *ty,
      };

      self.generation_log.annotation_type_locations.push(tys);
    }
  }
}
