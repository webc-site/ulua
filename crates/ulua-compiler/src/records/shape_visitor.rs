//! Source: `Compiler/src/TableShape.cpp:27-149`

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_expr::AstExpr, ast_expr_table::AstExprTable, ast_local::AstLocal, ast_name::AstName,
    ast_stat_assign::AstStatAssign, ast_stat_for::AstStatFor, ast_stat_function::AstStatFunction,
    ast_stat_local::AstStatLocal, ast_visitor::AstVisitor,
  },
  visit::ast_expr_visit,
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  functions::get_table_hint::get_table_hint,
  records::{hasher::Hasher, node::Node, table_shape::TableShape},
};

/// 循环上界预测上限（C++ `kMaxLoopBound`）
const K_MAX_LOOP_BOUND: f64 = 16.0;

#[derive(Debug)]
pub(crate) struct ShapeVisitor<'a> {
  pub(crate) shapes: &'a mut DenseHashMap<Node<AstExprTable>, TableShape>,
  pub(crate) tables: DenseHashMap<Node<AstLocal>, Node<AstExprTable>>,
  pub(crate) fields: DenseHashSet<(Node<AstExprTable>, AstName), Hasher>,
  pub(crate) loops: DenseHashMap<Node<AstLocal>, u32>,
}

impl<'a> ShapeVisitor<'a> {
  pub fn new(shapes: &'a mut DenseHashMap<Node<AstExprTable>, TableShape>) -> Self {
    ShapeVisitor {
      shapes,
      tables: DenseHashMap::default(),
      // 元组键无单元素 `default()` 门面形：空槽占位只参与存储、占用判定由
      // 位图负责（悬垂句柄永不与真实 arena 地址相撞）。
      fields: DenseHashSet::new((Node::default(), AstName::new())),
      loops: DenseHashMap::default(),
    }
  }

  fn assign_field_name(&mut self, expr: Node<AstExpr>, index: AstName) {
    let AstExprRef::Local(lv) = expr.as_expr_ref() else {
      return;
    };

    if let Some(&table) = self.tables.find(&lv.local.into()) {
      let field = (table, index);

      if !self.fields.contains(&field) {
        self.fields.insert(field);
        // 对应 C++ `shapes[*table].hashSize += 1`——operator[] 未命中时插入
        // 默认 shape。`find_mut` 不做插入，导致每张表的第一个字段查不到
        // shape、永不计数 → 预测恒为 0。
        self.shapes.get_or_insert(table).hash_size += 1;
      }
    }
  }

  fn assign_field_expr(&mut self, expr: Node<AstExpr>, index: Node<AstExpr>) {
    let AstExprRef::Local(lv) = expr.as_expr_ref() else {
      return;
    };
    let Some(&table) = self.tables.find(&lv.local.into()) else {
      return;
    };

    // index 的两种 kind 探测保持惰性（同 cpp if/else-if）：仅在前两道守卫
    // 命中后才读其 class index 与字段。
    match index.as_expr_ref() {
      AstExprRef::ConstantNumber(number) => {
        // C++ `shapes[*table]` 未命中即插入；此前 find_mut 不插入，数组预测
        // 因此从未启动。
        let shape = self.shapes.get_or_insert(table);
        if number.value == (shape.array_size as f64 + 1.0) {
          shape.array_size += 1;
        }
      }
      AstExprRef::Local(iter) => {
        if let Some(&bound) = self.loops.find(&iter.local.into()) {
          let shape = self.shapes.get_or_insert(table);
          if shape.array_size == 0 {
            shape.array_size = bound;
          }
        }
      }
      _ => {}
    }
  }

  fn assign(&mut self, var: Node<AstExpr>) {
    // assign 入口为赋值语句左值（parser 保证非空存活，见 `Node::borrow` 契约）。
    match var.as_expr_ref() {
      AstExprRef::IndexName(index_name) => {
        self.assign_field_name(index_name.expr.into(), index_name.index);
      }
      AstExprRef::IndexExpr(index_expr) => {
        self.assign_field_expr(index_expr.expr.into(), index_expr.index.into());
      }
      _ => {}
    }
  }
}

impl<'a> AstVisitor for ShapeVisitor<'a> {
  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    if node.vars.size == 1 && node.values.size == 1 {
      // size == 1 已验证：as_slice 界内取首槽，无需 unsafe
      let value = node.values.as_slice()[0];
      // C++ 用 getTableHint 把 `setmetatable(表字面量, ...)` 解包到内层表字面量。
      // 旧模型把初始化式直接强转成 AstExprTable，漏掉该形态，导致 setmetatable
      // 背后的表从未被跟踪、预测形状停在 (0,0) → size 0 的 NEWTABLE。
      let table = get_table_hint(value.into());
      // get_table_hint 只会返回存活 AST 中的表字面量句柄（同 cpp getTableHint）。
      if let Some(table) = table
        && table.borrow().items.size == 0
      {
        // 首行已验证 size == 1：vars 首槽经 as_slice 界内取回
        let var = node.vars.as_slice()[0];
        self.tables.try_insert(var.into(), table);
      }
    }

    true
  }

  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    for var in node.vars.iter() {
      self.assign((*var).into());
    }

    for value in node.values.iter().map(|v| Node::from(*v)) {
      // values 元素为 parser 保证非空存活的 AstExpr（`Node` 契约）；ShapeVisitor
      // 只写自身 tables/shapes/loops map，不写 AST，arena 独占成立。
      unsafe { ast_expr_visit(value.as_ptr(), self) };
    }

    false
  }

  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.assign(node.name.into());

    // node.func 为 parser 保证非空存活的 AstExprFunction（`Node` 契约），向上转
    // AstExpr 依 repr(C) 前缀重合合法；visitor 不写 AST。
    unsafe { ast_expr_visit(Node::from(node.func).cast::<AstExpr>().as_ptr(), self) };

    false
  }

  fn visit_stat_for(&mut self, node: &mut AstStatFor) -> bool {
    // from/to 非空由解析器保证（仅 step 可空），与 cpp `node->from->as<...>()`
    // 直接下转同一前提；下转在引用形态上做。
    if let (AstExprRef::ConstantNumber(from), AstExprRef::ConstantNumber(to)) =
      (node.from.as_expr_ref(), node.to.as_expr_ref())
      && from.value == 1.0
      && (1.0..=K_MAX_LOOP_BOUND).contains(&to.value)
      && node.step.is_none()
    {
      // step 的 is_none 判空对应 cpp `!node->step`：可空 OptNode 只探不解引用。
      self.loops.try_insert(node.var.into(), to.value as u32);
    }

    true
  }
}
