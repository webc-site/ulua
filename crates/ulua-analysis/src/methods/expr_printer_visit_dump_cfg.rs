//! ExprPrinter 的表达式访问器，对齐 C++ `DumpCFG.cpp` 的 `ExprPrinter : AstVisitor`。

extern crate alloc;

use alloc::string::String;
use core::fmt::Write as _;

use ulua_ast::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  functions::to_string_ast::{to_str as unary_op_to_string, to_str_binary as binary_op_to_string},
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_local::AstExprLocal,
    ast_expr_unary::AstExprUnary,
  },
  rtti::ast_node_is,
};

use crate::{
  functions::{dump_def::dump_def, get_local_name::get_local_name},
  records::expr_printer::ExprPrinter,
};

impl ExprPrinter {
  /// # Safety
  /// 调用方须保证 `node` 有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr(&mut self, node: *mut AstExpr) -> bool {
    // Safety: 由函数级契约，node 为 DumpCFG 遍历期存活的非空 AstExpr 裸句柄
    // （parser/CFG 构造产物，AST 活过整个 dump），`&(*node).base` 只读其首字段；
    // 各 `ast_node_is::<T>` 按 class index 判定，命中后 `node as *mut T` 因
    // repr(C) 单继承基址重合而指向真实 T 节点，未命中分支不解引用；下游
    // unsafe fn 的 # Safety 前置（节点真实类型、子指针有效）都由该分派满足。
    unsafe {
      let base = &(*node).base;
      if ast_node_is::<AstExprLocal>(base) {
        self.visit_ast_expr_local(node.cast::<AstExprLocal>())
      } else if ast_node_is::<AstExprConstantNumber>(base) {
        self.visit_ast_expr_constant_number(node.cast::<AstExprConstantNumber>())
      } else if ast_node_is::<AstExprConstantString>(base) {
        self.visit_ast_expr_constant_string(node.cast::<AstExprConstantString>())
      } else if ast_node_is::<AstExprConstantBool>(base) {
        self.visit_ast_expr_constant_bool(node.cast::<AstExprConstantBool>())
      } else if ast_node_is::<AstExprConstantNil>(base) {
        self.visit_ast_expr_constant_nil(node.cast::<AstExprConstantNil>())
      } else if ast_node_is::<AstExprBinary>(base) {
        self.visit_ast_expr_binary(node.cast::<AstExprBinary>())
      } else if ast_node_is::<AstExprUnary>(base) {
        self.visit_ast_expr_unary(node.cast::<AstExprUnary>())
      } else {
        self.result.push_str("<expr>");
        false
      }
    }
  }

  /// # Safety
  /// 仅由 `visit_ast_expr` 在 `ast_node_is::<AstExprLocal>` 判定后调用：`node` 须
  /// 指向一个真实 `AstExprLocal`，其 `local` 字段须为存活的 `*mut AstLocal`
  /// （`get_local_name` 会解引用它）。
  unsafe fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    // Safety: node 仅经 visit_ast_expr 的 class index 命中后转入，确为存活
    // AstExprLocal（repr(C) 基址重合），`(*node).local` 按值读出其指针；
    // local 由 parser/scope 构造期接线为非空 AstLocal 句柄，活过整个 dump，
    // 满足 get_local_name 前置；def 从 use_defs 按值拷出，为构建期
    // register_sym_def 发放的 SymDef 句柄，dump_def 经注册表只读解析（safe）。
    unsafe {
      if let Some(&def) = self.use_defs.get(&(node.cast::<AstExpr>())) {
        self.result.push_str(&dump_def(def));
      } else {
        // local 槽已句柄化恒非空；get_local_name 为既有裸指针 API，经 as_ptr 桥接。
        self
          .result
          .push_str(&get_local_name((*node).local.as_ptr()));
        self.result.push('?');
      }
    }
    false
  }

  /// # Safety
  /// `node` 须指向一个真实 `AstExprConstantNumber`（经 `visit_ast_expr` 的类型
  /// 判定分派），本函数以 `&*node` 解引用其 `parse_result`/`value` 字段。
  unsafe fn visit_ast_expr_constant_number(&mut self, node: *mut AstExprConstantNumber) -> bool {
    // Safety: node 由 visit_ast_expr 的 ast_node_is::<AstExprConstantNumber>
    // 命中后分派，repr(C) 基址重合 ⇒ &*node 只读借出真实节点；数值字段按值读。
    unsafe {
      let node = &*node;
      // 对齐 C++：整数且精确解析时按 i64 输出，否则按 %f 六位小数
      if node.parse_result == ConstantNumberParseResult::Ok
        && node.value == node.value as i64 as f64
      {
        let _ = write!(self.result, "{}", node.value as i64);
      } else {
        let _ = write!(self.result, "{:.6}", node.value);
      }
    }
    false
  }

  /// # Safety
  /// `node` 须指向一个真实 `AstExprConstantString`，且其 `value.data`/`value.size`
  /// 须描述一段可读的 `size` 字节内存（`from_raw_parts` 要求指针与长度构成有效
  /// 切片：`data` 非空或 `size` 为 0）。
  unsafe fn visit_ast_expr_constant_string(&mut self, node: *mut AstExprConstantString) -> bool {
    // Safety: node 由 visit_ast_expr 的 ast_node_is::<AstExprConstantString>
    // 命中后分派，repr(C) 基址重合 ⇒ 指向真实存活节点；value.data/size 由
    // parser arena 保证构成可读字节区间（as_bytes 与 AstArray::as_slice 共用
    // 同一有效性约定：null 数据仅与 size==0 并存），且缓冲活过整个 dump。
    let node = unsafe { &*node };
    self.result.push('"');
    self
      .result
      .push_str(&String::from_utf8_lossy(node.value.as_bytes()));
    self.result.push('"');
    false
  }

  /// # Safety
  /// `node` 须指向一个真实 `AstExprConstantBool`（经 `visit_ast_expr` 的类型判定
  /// 分派），本函数以 `(*node).value` 解引用读取其布尔值。
  unsafe fn visit_ast_expr_constant_bool(&mut self, node: *mut AstExprConstantBool) -> bool {
    // Safety: node 经 visit_ast_expr 的 ast_node_is::<AstExprConstantBool> 命中
    // 后分派，repr(C) 基址重合 ⇒ (*node).value 读取真实节点的布尔字段。
    self.result.push_str(if unsafe { (*node).value } {
      "true"
    } else {
      "false"
    });
    false
  }

  /// # Safety
  /// 仅作为 `visit_ast_expr` 对 `AstExprConstantNil` 分支的适配器保留；本函数不解
  /// 引用 `node`（参数被忽略），但 `unsafe` 签名与其余访问器一致，要求传入的
  /// 指针确实指向一个 `AstExprConstantNil` 节点。
  unsafe fn visit_ast_expr_constant_nil(&mut self, _: *mut AstExprConstantNil) -> bool {
    self.result.push_str("nil");
    false
  }

  /// # Safety
  /// `node` 须指向一个真实 `AstExprBinary`，且其 `left`/`right` 子指针须为有效的
  /// `*mut AstExpr`（以 `&*node` 解读取 `op`，并对子节点递归调用 `visit_ast_expr`）。
  unsafe fn visit_ast_expr_binary(&mut self, node: *mut AstExprBinary) -> bool {
    // Safety: node 经 ast_node_is::<AstExprBinary> 命中后分派，&*node 借出真实
    // 存活节点；left/right 为 parser 语法必选子节点（非空不变量，递归再按
    // class index 分派），op 按值读出。
    unsafe {
      let node = &*node;
      // left/right 已句柄化；visit_ast_expr 为既有裸指针 API，经 as_ptr 桥接。
      self.visit_ast_expr(node.left.as_ptr());
      self.result.push(' ');
      self.result.push_str(binary_op_to_string(node.op));
      self.result.push(' ');
      self.visit_ast_expr(node.right.as_ptr());
    }
    false
  }

  /// # Safety
  /// `node` 须指向一个真实 `AstExprUnary`，其 `expr` 子指针须为有效的
  /// `*mut AstExpr`（以 `&*node` 读取 `op`，并对子节点递归调用 `visit_ast_expr`）。
  unsafe fn visit_ast_expr_unary(&mut self, node: *mut AstExprUnary) -> bool {
    // Safety: 与 binary 分支同理——ast_node_is::<AstExprUnary> 命中后分派，
    // &*node 借出真实节点；expr 为 parser 语法必选子表达式（非空），op 按值读。
    unsafe {
      let node = &*node;
      self.result.push_str(unary_op_to_string(node.op));
      // expr 已句柄化；visit_ast_expr 为既有裸指针 API，经 as_ptr 桥接。
      self.visit_ast_expr(node.expr.as_ptr());
    }
    false
  }
}
