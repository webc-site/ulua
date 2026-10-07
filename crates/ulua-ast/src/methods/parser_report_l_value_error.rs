use ulua_common::fflag;

use crate::{
  functions::optional_node::slot_ref,
  records::{ast_expr::AstExpr, ast_expr_local::AstExprLocal, parser::Parser},
  rtti::ast_node_try_as,
};

impl Parser {
  pub(crate) fn report_l_value_error(&mut self, expr: *mut AstExpr) -> *mut AstExpr {
    let location = slot_ref(expr).base.location;

    // 常量 local 赋值有专门报错；非常量走通用报错
    // `expr` 由调用方保证指向 arena 存活节点；`ast_node_try_as::<AstExprLocal>` 按 RTTI
    // 命中才 Some。`local.local` 已句柄化恒非空，判空折叠随类型消失。
    let const_local = ast_node_try_as::<AstExprLocal>(&slot_ref(expr).base)
      .map(|local| local.local.get())
      .filter(|local| local.is_const);

    if let Some(local) = const_local {
      let expressions = self.copy_initializer_list_t(&[expr]);
      let name = local.name;
      return self.report_expr_error(
        location,
        expressions,
        format_args!("Variable '{}' is constant and may not be reassigned", name),
      );
    }

    // cpp:2051 类名全局被赋值有专门报错，指回类定义行。
    if fflag::DebugLuauUserDefinedClasses.get() {
      // get_matching_class 交出自有 `Node<AstStatClass>`（Option 侧即 arena 存活类
      // 声明）：读嵌套 name/location 字段经句柄 Deref，且句柄不携带对 self 的借用，
      // 下方以 `&mut self` 报错无需假 'static。
      if let Some(class_stat) = self.get_matching_class(expr) {
        // name 指向 arena 存活的 Name 表项（classes_within_module 登记时写入；
        // AstStatClass::name 槽位仍是 cpp 形态裸指针，records 引用化后续波次收口）。
        let name = slot_ref(class_stat.name).name;
        let line = class_stat.base.base.location.begin.line + 1;
        let expressions = self.copy_initializer_list_t(&[expr]);
        return self.report_expr_error(
          location,
          expressions,
          format_args!(
            "'{}' refers to a class and cannot be used as a variable name (defined on line {})",
            name, line
          ),
        );
      }
    }

    let expressions = self.copy_initializer_list_t(&[expr]);
    self.report_expr_error(
      location,
      expressions,
      format_args!("Assigned expression must be a variable or a field"),
    )
  }
}
