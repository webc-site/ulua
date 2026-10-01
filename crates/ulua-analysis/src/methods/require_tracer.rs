//! `require_tracer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_group::AstExprGroup,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode,
    ast_type_group::AstTypeGroup, ast_type_typeof::AstTypeTypeof, node_handle::OptNode,
  },
  rtti::{AstNodePtr, ast_node_try_as},
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    arena_handle::{alias_opt, alias_ref},
    file_resolver::FileResolver,
    module_info::ModuleInfo,
    require_trace_result::RequireTraceResult,
    require_tracer::RequireTracer,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};

impl RequireTracer<'_> {
  /// §2：可空返回收口为 `Option`（原 3 处 `null_mut()` 逻辑层哨兵 → `None`）。
  /// 载荷仍是 AST arena 的 `*mut AstNode` 身份句柄（字段/队列的裸指针形态属
  /// 结构层既有约定，不改）。
  pub(crate) fn get_dependent(&self, node: *mut AstNode) -> Option<*mut AstNode> {
    let node_ref = alias_opt(node)?;

    if let Some(expr) = ast_node_try_as::<AstExprLocal>(node_ref) {
      // local 槽已句柄化恒非空；locals 键值为既有裸指针形态，经 as_ptr 桥接。
      return self
        .locals
        .find(&(expr.local.as_ptr()))
        .copied()
        .map(AstNodePtr::as_ast_node);
    } else if let Some(expr) = ast_node_try_as::<AstExprIndexName>(node_ref) {
      return Some(expr.expr.as_ast_node());
    } else if let Some(expr) = ast_node_try_as::<AstExprIndexExpr>(node_ref) {
      return Some(expr.expr.as_ast_node());
    } else if let Some(expr) = ast_node_try_as::<AstExprCall>(node_ref) {
      if expr.self_ {
        // 对照 C++ `static_cast<AstExprIndexName*>(func)`：self-call 不变式保证
        // func 实为 AstExprIndexName。
        if let Some(func) = alias_opt(expr.func.cast::<AstExprIndexName>()) {
          return Some(func.expr.as_ast_node());
        }
      }
    } else if let Some(expr) = ast_node_try_as::<AstExprGroup>(node_ref) {
      return Some(expr.expr.as_ast_node());
    } else if let Some(expr) = ast_node_try_as::<AstExprTypeAssertion>(node_ref) {
      return Some(expr.annotation.as_ast_node());
    } else if let Some(expr) = ast_node_try_as::<AstTypeGroup>(node_ref) {
      return Some(expr.type_.as_ast_node());
    } else if let Some(expr) = ast_node_try_as::<AstTypeTypeof>(node_ref) {
      return Some(expr.expr.as_ast_node());
    }

    None
  }
}

impl RequireTracer<'_> {
  pub fn process(&mut self, limits: &TypeCheckLimits) {
    let module_context = ModuleInfo {
      name: self.current_module_name.clone(),
      optional: false,
    };

    self.work.reserve(self.require_calls.len());
    for &require in &self.require_calls {
      let require_ref = alias_ref(require);
      self
        .work
        .push(require_ref.args.as_slice()[0].cast::<AstNode>());
    }

    // 工作队列在遍历中自增长（dep push 进 self.work），无法用迭代器表达，保留下标。
    let mut i = 0;
    while i < self.work.len() {
      let node = self.work[i];
      // 原 `!node.is_null()` 守卫并入 alias_opt 的 None 折叠（判据等价）。
      if let Some(dep) = self.get_dependent(node) {
        self.work.push(dep);
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
        if let Some(dep) = self.get_dependent(expr) {
          let context = (*self.result).exprs.find(&dep);
          // cpp 的 if/else-if 链：有上下文时透传类型节点，否则交由 resolver
          // 处理（context 可能为 None，对应 cpp 传入 nullptr）。
          if context.is_some() && {
            // 判型折叠为可空句柄 `OptNode::is`：null 折叠 false，只读 class_index、
            // 不外传借用。
            let h = OptNode::<AstNode>::from_ptr(expr);
            h.is::<AstExprLocal>()
              || h.is::<AstExprGroup>()
              || h.is::<AstTypeGroup>()
              || h.is::<AstTypeTypeof>()
              || h.is::<AstExprTypeAssertion>()
          } {
            info = context.cloned();
          } else if let Some(as_expr) = (*expr).as_expr() {
            info = self
              .file_resolver
              .resolve_module(context, as_expr.as_ref(), limits);
          }
        } else if let Some(as_expr) = (*expr).as_expr() {
          info = self
            .file_resolver
            .resolve_module(Some(&module_context), as_expr.as_ref(), limits);
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

impl<'a> RequireTracer<'a> {
  /// `file_resolver` 原样存入字段（`dyn` 保留理由见 [`RequireTracer::file_resolver`]
  /// 字段注：`FileResolver` 实现方集合运行期开放）。
  pub fn new(
    result: *mut RequireTraceResult,
    file_resolver: &'a mut dyn FileResolver,
    current_module_name: ModuleName,
  ) -> Self {
    RequireTracer {
      result,
      file_resolver,
      current_module_name,
      locals: DenseHashMap::default(),
      work: Vec::new(),
      require_calls: Vec::new(),
    }
  }
}
