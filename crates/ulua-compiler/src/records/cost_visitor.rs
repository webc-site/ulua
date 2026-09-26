//! Source: `Compiler/src/CostModel.cpp:103-419`
use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_local::AstExprLocal,
    ast_local::AstLocal, ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak, ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal, ast_stat_repeat::AstStatRepeat,
    ast_stat_while::AstStatWhile, ast_visitor::AstVisitor,
  },
  rtti::ast_node_is,
  visit::ast_stat_visit_ref,
};
use ulua_common::{
  enums::luau_builtin_function::LuauBuiltinFunction, fflag, macros::luau_assert::LUAU_ASSERT,
  records::dense_hash_map::DenseHashMap,
};

use crate::{
  functions::{
    always_terminates::always_terminates,
    ast_slot_ref::{ast_slot_is, ast_slot_try_as},
    get_trip_count::get_trip_count,
    is_constant::{is_constant_false, is_constant_true},
  },
  records::{constant::Constant, cost::Cost, node::Node},
};

/// cpp `CostModel.cpp:101-107` 的 `const DenseHashMap&` 引用成员直接以
/// Rust 共享引用建模，查表路径不再有裸指针解引用。
#[derive(Debug)]
pub(crate) struct CostVisitor<'a> {
  pub(crate) builtins: &'a DenseHashMap<Node<AstExprCall>, i32>,
  pub(crate) constants: &'a DenseHashMap<Node<AstExpr>, Constant>,
  pub(crate) vars: DenseHashMap<Node<AstLocal>, u64>,
  pub(crate) result: Cost,
}

impl<'a> CostVisitor<'a> {
  /// 裸指针入参只在 crate 内流转，不对外暴露（`not_unsafe_ptr_arg_deref`
  /// 仅约束对外可见函数）；对外入口是 [`crate::functions::cost_model::cost_model`]。
  pub(crate) fn model(&mut self, node: Node<AstExpr>) -> Cost {
    if self.constants.find(&node).is_some() {
      return Cost::new(0, Cost::K_LITERAL);
    }

    match node.as_expr_ref() {
      AstExprRef::Group(expr) => self.model(expr.expr.into()),
      AstExprRef::ConstantNil(_)
      | AstExprRef::ConstantBool(_)
      | AstExprRef::ConstantNumber(_)
      | AstExprRef::ConstantString(_)
      | AstExprRef::ConstantInteger(_) => Cost::new(0, Cost::K_LITERAL),
      AstExprRef::Local(expr) => {
        let constant = self.vars.find(&expr.local.into()).copied().unwrap_or(0);
        Cost::new(0, constant)
      }
      AstExprRef::Global(_) => Cost::new(1, 0),
      AstExprRef::Varargs(_) => Cost::new(3, 0),
      AstExprRef::Call(expr) => {
        let builtin = self
          .builtins
          .find(&expr.into())
          .is_some_and(|&id| id != LuauBuiltinFunction::LBF_NONE as i32);
        let builtin_short = builtin
          && expr.args.size
            <= if fflag::LuauCompileFastcall3CostModel.get() {
              3
            } else {
              2
            };

        let mut cost = Cost::new(if builtin { 2 } else { 3 }, 0);

        if !builtin {
          cost = cost.add(&self.model(expr.func.into()));
        }

        for arg in expr.args.iter() {
          let ac = self.model((*arg).into());
          let arg_cost = if ac.model == 0 && !builtin_short {
            Cost::new(1, 0)
          } else {
            ac
          };
          cost = cost.add(&arg_cost);
        }

        cost
      }
      AstExprRef::IndexName(expr) => self.model(expr.expr.into()).add(&Cost::new(1, 0)),
      AstExprRef::IndexExpr(expr) => self
        .model(expr.expr.into())
        .add(&self.model(expr.index.into()))
        .add(&Cost::new(1, 0)),
      AstExprRef::Function(_) => Cost::new(10, 0),
      AstExprRef::Table(expr) => {
        let mut cost = Cost::new(10, 0);

        for item in expr.items.iter() {
          if !item.key.is_null() {
            cost = cost.add(&self.model(item.key.into()));
          }

          cost = cost.add(&self.model(item.value.into()));
          cost = cost.add(&Cost::new(1, 0));
        }

        cost
      }
      AstExprRef::Unary(expr) => Cost::fold(
        &self.model(expr.expr.into()),
        &Cost::new(0, Cost::K_LITERAL),
      ),
      AstExprRef::Binary(expr) => Cost::fold(
        &self.model(expr.left.into()),
        &self.model(expr.right.into()),
      ),
      AstExprRef::TypeAssertion(expr) => self.model(expr.expr.into()),
      AstExprRef::IfElse(expr) => self
        .model(expr.condition.into())
        .add(&self.model(expr.true_expr.into()))
        .add(&self.model(expr.false_expr.into()))
        .add(&Cost::new(2, 0)),
      AstExprRef::InterpString(expr) => {
        let mut cost = Cost::new(3, 0);
        for inner_expression in expr.expressions.iter() {
          cost = cost.add(&self.model((*inner_expression).into()));
        }
        cost
      }
      AstExprRef::Instantiate(expr) => self.model(expr.expr.into()),
      AstExprRef::Error(_) => {
        LUAU_ASSERT!(false, "Unknown expression type");
        Cost::default()
      }
    }
  }

  pub(crate) fn assign(&mut self, expr: impl Into<Node<AstExpr>>) {
    // 门面判型+下转：expr 为存活语句（assign/compound-assign）var 槽中的非空表达式指针
    // （由调用方 visitor 交付），亦容忍 null；命中即类型正确。
    let Some(expr_local) = ast_slot_try_as::<AstExprLocal, _>(expr.into().as_ptr()) else {
      return;
    };

    // local 槽已句柄化恒非空（旧 is_null 死守卫随类型消失，等价 cpp 无判空形态）。
    let local = expr_local.local;
    if let Some(found) = self.vars.find_mut(&local.into()) {
      *found = 0;
    }
  }

  /// cpp `CostVisitor(const DenseHashMap&, const DenseHashMap&)` 引用成员直存。
  pub fn new(
    builtins: &'a DenseHashMap<Node<AstExprCall>, i32>,
    constants: &'a DenseHashMap<Node<AstExpr>, Constant>,
  ) -> Self {
    Self {
      builtins,
      constants,
      vars: DenseHashMap::default(),
      result: Cost::default(),
    }
  }

  /// cpp `getNumber(AstExpr*, double&)`：出参改返回值。`node` 仅作 constants
  /// 表的地址键使用（句柄模型），函数内不解引用。
  pub(crate) fn get_number(&self, node: impl Into<Node<AstExpr>>) -> Option<f64> {
    match self.constants.find(&node.into()) {
      Some(Constant::Number(n)) => Some(*n),
      _ => None,
    }
  }

  pub fn loop_item(&mut self, body: Option<&mut AstStatBlock>, iter_cost: Cost, factor: i32) {
    let before = self.result;

    self.result = Cost::default();

    if let Some(body) = body {
      self.visit_ast_stat_block(body);
    }

    self.result = before + (self.result + iter_cost) * factor;
  }

  pub fn visit_ast_stat_for_in(&mut self, node: &mut AstStatForIn) -> bool {
    for &expr_ptr in node.values.as_slice() {
      let cost = self.model(expr_ptr.into());
      // C++ `result += model(...)`——饱和加法（Cost::operator+=）。
      self.result += cost;
    }

    // body 已句柄化为 Node：`get_mut` 即非空 + 独占证明（分发链交付 &mut），
    // 死 unsafe 与裸指针 as_mut 门面消失；loop_item 借用止于本调用。
    self.loop_item(
      Some(node.body.get_mut()),
      Cost {
        model: 1,
        constant: 0,
      },
      3,
    );

    false
  }

  pub fn visit_ast_stat_while(&mut self, node: &mut AstStatWhile) -> bool {
    let condition = self.model(node.condition.into());
    // C++ `loop(node->body, condition)` 用缺省放大因子 3 而非 1。
    // body 已句柄化：可变借用沿 `&mut node` 传递，无裸指针解引用。
    self.loop_item(Some(node.body.get_mut()), condition, 3);

    false
  }

  pub fn visit_ast_stat_repeat(&mut self, node: &mut AstStatRepeat) -> bool {
    let condition = self.model(node.condition.into());

    // C++ `loop(node->body, condition)` 用缺省放大因子 3 而非 1。
    // body 已句柄化：可变借用沿 `&mut node` 传递，无裸指针解引用。
    self.loop_item(Some(node.body.get_mut()), condition, 3);

    false
  }

  pub fn visit_ast_stat_if(&mut self, node: &mut AstStatIf) -> bool {
    // condition 按值取句柄（Copy），子树遍历的借用只从 `&mut node` 现取现还，
    // 与 cpp 的 `if (node->elsebody)` 同形的判空折叠进 OptNode 的 Option。
    let condition = node.condition;

    // C++ 用 isConstantFalse/isConstantTrue（对常量表做完整真值判定），而非
    // getNumber——已折叠为布尔/字符串/nil 的条件同样必须剪掉死分支。getNumber
    // 只认 Number，导致常量条件分支被超量建模（如 `if a == 1 then ...`）。
    if is_constant_false(self.constants, condition.into()) {
      if let Some(elsebody) = node.elsebody.get_mut() {
        ast_stat_visit_ref(elsebody, self);
      }
      return false;
    }

    if is_constant_true(self.constants, condition.into()) {
      // thenbody 已句柄化（Node 恒非空，cpp 的 null 守卫为恒真）：静态类型即
      // AstStatBlock，直走块分发，等价 cpp class-index 命中 StatBlock 分支。
      ast_stat_block_visit(node.thenbody.get_mut(), self);
      return false;
    }

    // 无条件的 'else' 可能在 'if' 体之后需要一个跳转
    // 注：此处忽略了 'then' 必然终止的情形，也假设了比较总需额外一条指令（可能不成立）
    let has_else = node.elsebody.is_some();
    // 门面判型：elsebody 为空槽时 as_ptr 落 null，`ast_slot_is` 折叠为 false
    // （判空守卫由 OptNode 的 Option 兑现，同 cpp is<T>()）。
    let else_is_if = has_else && ast_slot_is::<AstStatIf, _>(node.elsebody.as_ptr());
    let discount = i32::from(has_else && !else_is_if);

    // 经 += 实现 C++ `result += 1 + (elsebody && !elsebody->is<AstStatIf>())`。
    self.result += Cost::new(1 + discount, 0);

    true
  }

  pub fn visit_ast_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    for (i, &expr_ptr) in node.values.iter().enumerate() {
      let arg = self.model(expr_ptr.into());

      // C++ `i < vars.size()` 越界保护：values 可能多于 vars
      if arg.constant != 0 && i < node.vars.len() {
        let var_ptr = node.vars.as_slice()[i];
        self.vars.try_insert(var_ptr.into(), arg.constant);
      }

      self.result += arg;
    }

    false
  }

  pub fn visit_ast_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    for &var_ptr in node.vars.iter() {
      self.assign(var_ptr);
    }

    // 单次遍历实现 zip_longest 语义：任一侧耗尽即视为 Cost::default()
    // （全零加法单位元）；求值顺序 `vars[i]` 先于 `values[i]`，对齐 C++
    let mut vars = node.vars.iter();
    let mut values = node.values.iter();

    loop {
      let ac = match (vars.next(), values.next()) {
        (Some(&v), Some(&val)) => self.model(v.into()) + self.model(val.into()),
        (Some(&v), None) => self.model(v.into()),
        (None, Some(&val)) => self.model(val.into()),
        (None, None) => break,
      };

      // local→local 或 constant→local 的赋值并非零代价
      if ac.model == 0 {
        self.result += Cost::new(1, 0);
      } else {
        self.result += ac;
      }
    }

    false
  }

  pub fn visit_ast_stat_compound_assign(&mut self, node: &mut AstStatCompoundAssign) -> bool {
    // assign(node->var)
    self.assign(node.var);

    // 左值不是 local 时，赋值额外需要一次表操作
    // var 已句柄化：安全判型门面直取 `.get()`（同 cpp is<T>()，无裸指针）。
    let is_local = ast_node_is::<AstExprLocal>(node.var.get());
    let cost_increment = if is_local { 1 } else { 2 };
    self.result += Cost::new(cost_increment, 0);

    true
  }

  pub fn visit_ast_stat_break(&mut self) -> bool {
    self.result += Cost::new(1, 0);
    false
  }

  pub fn visit_ast_stat_continue(&mut self) -> bool {
    self.result += Cost::new(1, 0);

    false
  }

  pub fn visit_ast_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    visit_ast_stat_block(self, node)
  }
}

// CostVisitor 的成本累加住在各 inherent visit_ast_* 方法里（与
// CostModel.cpp 各 visit() 覆写一一对应）。AstVisitor trait 的分发必须
// 委派到它们；若像旧桩实现那样在此返回 false，会跳过全部成本累加，
// 于是每个函数都算出成本 0、循环永远被展开。
// C++ 未覆写的语句种类（return/expr/function/...）落到 trait 默认实现
// （继续下钻），其子表达式经由 visit_expr -> visit_ast_expr -> model() 计费。
impl AstVisitor for CostVisitor<'_> {
  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
    // 分发链交付 &mut（非空由类型证明），不再有旧实现的判空早退；
    // from_mut 造的指针只作为 model/constants 查表的地址键（句柄模型）。
    let cost = self.model(Node::from_mut(node));
    // C++ `result += model(node)`——饱和加法并清零常量掩码。
    self.result.add_assign(&cost);

    false
  }

  fn visit_stat_for(&mut self, node: &mut AstStatFor) -> bool {
    visit_ast_stat_for(self, node)
  }

  fn visit_stat_for_in(&mut self, node: &mut AstStatForIn) -> bool {
    self.visit_ast_stat_for_in(node)
  }

  fn visit_stat_while(&mut self, node: &mut AstStatWhile) -> bool {
    self.visit_ast_stat_while(node)
  }

  fn visit_stat_repeat(&mut self, node: &mut AstStatRepeat) -> bool {
    self.visit_ast_stat_repeat(node)
  }

  fn visit_stat_if(&mut self, node: &mut AstStatIf) -> bool {
    self.visit_ast_stat_if(node)
  }

  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    self.visit_ast_stat_local(node)
  }

  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    self.visit_ast_stat_assign(node)
  }

  fn visit_stat_compound_assign(&mut self, node: &mut AstStatCompoundAssign) -> bool {
    self.visit_ast_stat_compound_assign(node)
  }

  fn visit_stat_break(&mut self, _node: &mut AstStatBreak) -> bool {
    self.visit_ast_stat_break()
  }

  fn visit_stat_continue(&mut self, _node: &mut AstStatContinue) -> bool {
    self.visit_ast_stat_continue()
  }

  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    self.visit_ast_stat_block(node)
  }
}

pub(crate) fn visit_ast_stat_for(this: &mut CostVisitor<'_>, stat_for: &AstStatFor) -> bool {
  // C++ `result += model(...)` 走 Cost::operator+=（逐字节饱和加法，并清零常量
  // 掩码）。此前这里的裸 u64 `+=` 既错记了常量掩码，又在乘法放大后的循环
  // 代价逼近 u64::MAX 时发生溢出。
  let c_from = this.model(stat_for.from.into());
  this.result += c_from;

  let c_to = this.model(stat_for.to.into());
  this.result += c_to;

  // step 已落可空 OptNode：is_some 承接 cpp 判空，as_ptr 桥交指针门面。
  if stat_for.step.is_some() {
    let c_step = this.model(stat_for.step.as_ptr().into());
    this.result += c_step;
  }

  // cpp：三个 number 常量齐备才估算迭代次数；缺省 step 指针即 1.0（与旧出参
  // 初值一致）。get_number 为纯查表、无副作用，Option 化与 && 短路等价。
  let step_val = if stat_for.step.is_none() {
    Some(1.0)
  } else {
    this.get_number(stat_for.step.as_ptr())
  };
  let trip_count = match (
    this.get_number(stat_for.from),
    this.get_number(stat_for.to),
    step_val,
  ) {
    (Some(from_val), Some(to_val), Some(step_val)) => get_trip_count(from_val, to_val, step_val),
    _ => -1,
  };

  let factor = if trip_count < 0 { 3 } else { trip_count };
  // Safety: body 句柄出自 parser 存活契约（本入口按 & 借用消费，as_ptr 桥回
  // 裸指针重建 Option<&mut> 与 cpp 直传 node->body 同语义）；loop_item 调用内
  // 遍历完毕即释放借用，CostVisitor 只写自身 result。
  this.loop_item(
    Some(unsafe { &mut *stat_for.body.as_ptr() }),
    Cost {
      model: 1,
      constant: 0,
    },
    factor,
  );

  false
}

pub(crate) fn visit_ast_stat_block(this: &mut CostVisitor<'_>, block: &mut AstStatBlock) -> bool {
  for stat in block.body.iter_nodes_mut() {
    // body 元素已句柄化（Nodes）：&mut block 派生自分发链独占借用，get_mut 直接
    // 交出子语句可变借用，visit_ref 门面走 class-index 分发，无裸指针交接。
    ast_stat_visit_ref(stat.get_mut(), this);

    // C++ 在块内遇到无条件终止语句（return/break/continue，或所有分支都
    // 终止的 if）后即停止为该块建模——其后是死代码。此前占位的恒假判定
    // 把 return 之后的代码也重复计入了代价。`always_terminates` 是安全门面，
    // 不必留在 unsafe 内。
    if always_terminates(this.constants, stat.as_ptr()) {
      break;
    }
  }

  false
}
