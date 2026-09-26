use ulua_ast::{
  records::{
    ast_expr_group::AstExprGroup, ast_expr_local::AstExprLocal,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode, ast_type_group::AstTypeGroup,
    ast_type_typeof::AstTypeTypeof,
  },
  rtti::ast_node_is_ptr,
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
    for &require in &self.require_calls {
      // SAFETY: cpp 由 visit(AstExprCall*) 的 `args.size >= 1` 保证首参存在；
      // require 指向 arena 存活节点，一次解引用换得引用读 args/location。
      let require_ref = unsafe { &*require };
      self
        .work
        .push(require_ref.args.as_slice()[0].cast::<AstNode>());
    }

    let mut i = 0;
    while i < self.work.len() {
      let node = self.work[i];
      if !node.is_null() {
        // Safety: node 已判非空，且 work 内所有元素均为 AST arena 存活节点
        // （arena bump 分配地址不移动、活过 trace 全程）；get_dependent 的
        // 契约即「node 非空且指向存活 repr(C) 节点」，其内部只读字段做
        // class-index 下转，self.locals/result 在本遍历期只经 &self 访问。
        let dep = unsafe { self.get_dependent(node) };
        if !dep.is_null() {
          self.work.push(dep);
        }
      }
      i += 1;
    }

    // Safety: 块内解引用只触及三类长寿对象——self.result 为构造期接线的
    // *mut RequireTraceResult（C++ 引用成员等价，指向调用方 trace_requires
    // 持有的活对象，比本 tracer 长寿）；work/require_calls 的元素全部源自
    // AST arena 节点（地址不移动、活过整个 trace），require 的 args 首参存
    // 在性由 cpp visit(AstExprCall*) 的 args.size>=1 访问契约保证（与上方
    // 23-24 行同一论证）；(*expr).as_expr() 为 class-index 判型，结果判空
    // 后才再借用。get_dependent 与 exprs.find/insert 均单线程串行独占，
    // file_resolver 是 &mut 'a 借用字段，块内无并存别名、无悬垂读。
    unsafe {
      while let Some(expr) = self.work.pop() {
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
            && (ast_node_is_ptr::<AstExprLocal>(expr)
              || ast_node_is_ptr::<AstExprGroup>(expr)
              || ast_node_is_ptr::<AstTypeGroup>(expr)
              || ast_node_is_ptr::<AstTypeTypeof>(expr)
              || ast_node_is_ptr::<AstExprTypeAssertion>(expr))
          {
            info = context.cloned();
          } else {
            if let Some(as_expr) = (*expr).as_expr() {
              info = self
                .file_resolver
                .resolve_module(context, as_expr.as_ref(), limits);
            }
          }
        } else {
          if let Some(as_expr) = (*expr).as_expr() {
            info =
              self
                .file_resolver
                .resolve_module(Some(&module_context), as_expr.as_ref(), limits);
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
        // SAFETY 同上：arena 存活，args 首参存在。
        let require_ref = &*require;
        let arg = require_ref.args.as_slice()[0].cast::<AstNode>();
        if let Some(info) = (*self.result).exprs.find(&arg) {
          (*self.result)
            .require_list
            .push((info.name.clone(), require_ref.base.base.location));
          // cpp `result.exprs[require] = std::move(infoCopy)`：先取出副本再写入，
          // 因为写入会使 info 失效。
          let info_copy = info.clone();
          (*self.result)
            .exprs
            .insert(require.cast::<AstNode>(), info_copy);
        } else {
          // cpp: mark require as unresolved
          (*self.result)
            .exprs
            .insert(require.cast::<AstNode>(), ModuleInfo::default());
        }
      }
    }
  }
}
