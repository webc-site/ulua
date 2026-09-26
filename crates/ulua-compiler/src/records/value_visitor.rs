use core::{mem, ptr::from_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_local::AstExprLocal, ast_local::AstLocal, ast_name::AstName,
    ast_stat_assign::AstStatAssign, ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_function::AstStatFunction,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
    ast_visitor::AstVisitor,
  },
  rtti::{ast_node_try_as, ast_node_try_as_mut},
  visit::{ast_expr_visit, ast_expr_visit_ref},
};
use ulua_common::{fflag, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::global::Global,
  functions::ast_slot_ref::ast_slot_ref,
  records::{node::Node, variable::Variable},
};

#[derive(Debug, Clone)]
pub(crate) struct ValueVisitor {
  pub(crate) globals: DenseHashMap<AstName, Global>,
  pub(crate) variables: DenseHashMap<Node<AstLocal>, Variable>,
  pub(crate) class_locals: DenseHashMap<AstName, Node<AstLocal>>,
}

// 节点级 hook 族由本文件 value tracking 区段直接以
// `AstVisitor` 类型化 hook（`&mut` 入参）实现，本文件只持有状态。

impl ValueVisitor {
  /// `var` 可空（None 早退，cpp null 哨兵的 Option 化）；借用证明节点存活且
  /// 独占——本 visitor 只写自身 map、不写 AST。
  pub fn assign(&mut self, var: Option<&mut AstExpr>) {
    let Some(var) = var else {
      return;
    };

    if let Some(local) = ast_node_try_as_mut::<AstExprLocal>(&mut var.base) {
      // 对应 C++ `variables[lv->local].written = true`：operator[] 在条目缺失时
      // 会新建，故用 get_or_insert（find_mut 会漏掉新键，如数值 for 循环变量
      // 在体内被赋值的情形）。
      self.variables.get_or_insert(local.local.into()).written = true;
      return;
    }

    if let Some(global) = ast_node_try_as::<AstExprGlobal>(&var.base) {
      // C++ `globals[gv->name] = Global::Written`：operator[] 覆写；try_insert
      // 会漏改已存在的 Default 条目。
      *self.globals.get_or_insert(global.name) = Global::Written;
      return;
    }

    // C++ `var->visit(this)`：经节点分发器跟踪复杂左值内的赋值，如
    // `t[function() t = nil end] = 5`。
    // Safety: var 借用证明存活且独占（函数契约）；分发器只读走链并经 visitor
    // 写自身 map，不回写 AST 节点。
    unsafe { ast_expr_visit(from_mut(var), self) };
  }

  pub fn new(
    globals: &mut DenseHashMap<AstName, Global>,
    variables: &mut DenseHashMap<Node<AstLocal>, Variable>,
    class_locals: &mut DenseHashMap<AstName, Node<AstLocal>>,
  ) -> Self {
    let globals_owned = mem::take(globals);
    let variables_owned = mem::take(variables);
    let class_locals_owned = mem::take(class_locals);
    ValueVisitor {
      globals: globals_owned,
      variables: variables_owned,
      class_locals: class_locals_owned,
    }
  }
}

impl AstVisitor for ValueVisitor {
  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    let values_len = node.values.len();

    // C++ `variables[vars[i]].init = values[i]`：operator[] 在条目缺失时创建
    // （缺省 written/constant），存在时仅覆写 `.init`。
    // zip 取较短一侧，等价于 `min(vars.len, values.len)` 循环。
    for (&var_ptr, &init_ptr) in node.vars.iter().zip(node.values.iter()) {
      self.variables.get_or_insert(var_ptr.into()).init = Some(init_ptr.into());
    }

    for &var_ptr in node.vars.iter().skip(values_len) {
      self.variables.get_or_insert(var_ptr.into()).init = None;
    }

    true
  }

  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    // vars/values 槽由 parser 保证为非空存活表达式指针；本 visitor 只写自身
    // map、不改 AST，arena 独占成立。
    for &var in node.vars.iter() {
      // var 为 parser 保证非空存活的表达式槽（见上）；句柄物化独占借用，null 折叠 None。
      self.assign((!var.is_null()).then(|| Node::new(var).borrow_mut()));
    }
    for &value in node.values.iter() {
      // Safety: value 同上，指向 arena 内存活 AstExpr。
      unsafe { ast_expr_visit(value, self) };
    }

    false
  }

  fn visit_stat_compound_assign(&mut self, node: &mut AstStatCompoundAssign) -> bool {
    // var/value 已句柄化（node_handle::Node）：`get_mut()` 即非空 + 独占证明，
    // 原 `is_null` 死守卫、`Node::new(..).borrow_mut()` 指针物化与子节点遍历的
    // unsafe 门面一并消失；本 visitor 只写自身 map、不改 AST。
    self.assign(Some(node.var.get_mut()));
    ast_expr_visit_ref(node.value.get_mut(), self);

    false
  }

  fn visit_stat_local_function(&mut self, node: &mut AstStatLocalFunction) -> bool {
    // name/func 由 parser 保证非空；指针仅作 map 键与 init 引用存储（同 cpp），
    // 不在本 visitor 内解引用。
    self.variables.get_or_insert(node.name.into()).init = Some(node.func.cast::<AstExpr>().into());

    true
  }

  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    // name/func 已句柄化（node_handle::Node）：`get_mut()` 即非空 + 独占证明，
    // 原 `is_null` 死守卫与 `Node::new(..).borrow_mut()` 指针物化一并消失。
    self.assign(Some(node.name.get_mut()));
    // C++ `node->func->visit(this)` 是 AST 节点分发——既跑 visit_expr_function
    // 回调（登记形参）又递归进函数体。此前只调裸回调跳过了函数体，
    // 函数内声明的 local 从未被跟踪（recordValue 随之 panic）。分发落 safe 引用形态。
    ast_expr_visit_ref(node.func.cast::<AstExpr>().get_mut(), self);

    false
  }

  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    // C++ `variables[arg].init = nullptr`：operator[] 对已存在条目覆写
    // `.init`（try_insert 会保持不变）。
    // args 元素为 parser 保证的形参局部指针；is_null 过滤沿用 cpp 的防御式写法。
    // args 句柄化后元素恒非空（构造端断言），cpp 的 null 防御分支随之退役。
    for arg in node.args.iter_nodes() {
      self.variables.get_or_insert(Node::from(*arg)).init = None;
    }

    true
  }

  fn visit_stat_class(&mut self, node: &mut AstStatClass) -> bool {
    if !fflag::DebugLuauUserDefinedClasses.get() {
      return false;
    }

    // name 槽只在具名类上非空（parser 保证指向 arena 存活 AstLocal），
    // 这里仅读取其 name 作 map 键（地址键句柄模型，不解引用写穿）。
    if let Some(name_local) = ast_slot_ref(node.name) {
      let name = name_local.name;
      self.class_locals.insert(name, node.name.into());
      self.variables.get_or_insert(node.name.into()).written = true;
    }

    true
  }
}
