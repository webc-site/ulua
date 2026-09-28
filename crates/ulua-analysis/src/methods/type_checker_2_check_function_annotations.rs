//! Faithful port of `TypeChecker2::checkFunctionAnnotations` (TypeChecker2.cpp:1332-1380).
//!
//! 当函数（顶层 `function`/`local function`、类方法或构造函数）的参数/返回值
//! 缺少源码标注且推导类型可读出时，报告 [`TypeAnnotationRequired`] 并给出建议
//! 标注文案。C++ 端 Constructor 语境下「构造函数不应有返回标注」分支对应
//! `ConstructorsShouldNotReturnAnything`（`Error.h:629-635`），该变体尚未移植，
//! 见 tests/type_infer_classes.rs 末尾缺口登记。

use alloc::vec::Vec;

use ulua_ast::records::{ast_expr_function::AstExprFunction, location::Location};

use crate::{
  enums::annotation_check_mode::AnnotationCheckMode,
  functions::{
    extend_type_pack::extend_type_pack, flatten_type_pack::flatten_type_pack_id, get_type,
  },
  records::{
    function_type::FunctionType, type_annotation_required::TypeAnnotationRequired,
    type_checker_2::TypeChecker2,
  },
};

impl TypeChecker2 {
  pub(crate) fn check_function_annotations(
    &mut self,
    func: &AstExprFunction,
    mode: AnnotationCheckMode,
    name_location: Location,
  ) {
    let ty = self.lookup_type(&func.base);
    let ft = get_type::get::<FunctionType>(ty);

    let mut missing = false;

    if let Some(ft) = ft {
      // C++ `extendTypePack(*module->internalTypes, builtinTypes, ft->argTypes, func->args.size)`
      // Safety: self.module 是 checker 为当前模块接线的存活 *mut Module（与 builtin_types
      // 句柄同为构造期接线，比本次调用长寿）；extend_type_pack 仅向模块 internal_types
      // arena 追加节点/原地改写正在跟随的 FreeTypePack 槽位，`ft` 为 &'static 裸指针
      // 解引用所得（C++ NotNull 解引用同义），不与 arena 的可变借用构成别名冲突。
      let args = unsafe {
        extend_type_pack(
          &mut (*self.module).internal_types,
          self.builtin_types,
          ft.arg_types,
          func.args.len(),
          Vec::new(),
        )
      };

      for (i, arg) in func.args.iter_nodes().enumerate() {
        if i == 0 && arg.name == "self" && mode != AnnotationCheckMode::Function {
          // Annotating the self parameter is already a syntax error.  We
          // don't need to report anything here.
        } else if arg.annotation.is_null() && i < args.head.len() {
          missing = true;
        }
      }

      if func.vararg && func.vararg_annotation.is_null() && args.tail.is_some() {
        missing = true;
      }

      if mode == AnnotationCheckMode::Constructor && !func.return_annotation.is_null() {
        // C++ 在此报告 ConstructorsShouldNotReturnAnything（`TypeChecker2.cpp:1358-1360`），
        // 该 TypeErrorData 变体尚未移植；`missing` 语义不受影响（该分支不会置位）。
      } else if func.return_annotation.is_null() {
        let (head, tail) = flatten_type_pack_id(ft.ret_types);

        if !head.is_empty() || tail.is_some() {
          missing = true;
        }
      }
    }

    if missing {
      let mut location = name_location;
      if let Some(arg_location) = func.arg_location {
        location.extend(&arg_location);
      }
      if let Some(return_annotation) = func.return_annotation.as_ref() {
        location.extend(&return_annotation.base.location);
      }
      self.report_error_type_error_data_location(TypeAnnotationRequired::new(ty).into(), &location);
    }
  }
}
