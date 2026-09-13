//! Source: `Compiler/src/TableShape.cpp:27-149`

use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable, ast_local::AstLocal,
    ast_name::AstName, ast_node::AstNode, ast_stat_assign::AstStatAssign, ast_stat_for::AstStatFor,
    ast_stat_function::AstStatFunction, ast_stat_local::AstStatLocal, ast_visitor::AstVisitor,
  },
  rtti::ast_node_as,
  visit::ast_expr_visit,
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  functions::get_table_hint::get_table_hint,
  records::{hasher::Hasher, table_shape::TableShape},
};

#[derive(Debug)]
pub struct ShapeVisitor<'a> {
  pub(crate) shapes: &'a mut DenseHashMap<*mut AstExprTable, TableShape>,
  pub(crate) tables: DenseHashMap<*mut AstLocal, *mut AstExprTable>,
  pub(crate) fields: DenseHashSet<(*mut AstExprTable, AstName), Hasher>,
  pub(crate) loops: DenseHashMap<*mut AstLocal, u32>,
}

impl<'a> ShapeVisitor<'a> {
  pub fn new(shapes: &'a mut DenseHashMap<*mut AstExprTable, TableShape>) -> Self {
    ShapeVisitor {
      shapes,
      tables: DenseHashMap::new(null_mut()),
      fields: DenseHashSet::new((null_mut(), AstName::new())),
      loops: DenseHashMap::new(null_mut()),
    }
  }

  fn assign_field_name(&mut self, expr: *mut AstExpr, index: AstName) {
    let lv = unsafe { ast_node_as::<AstExprLocal>(expr as *mut AstNode) };
    if lv.is_null() {
      return;
    }

    let table_opt = self.tables.find(&unsafe { (*lv).local });
    if let Some(&table) = table_opt {
      let field = (table, index);

      if !self.fields.contains(&field) {
        self.fields.insert(field);
        // C++ `shapes[*table].hashSize += 1` — operator[] inserts a default
        // shape on miss. `find_mut` does NOT insert, so the FIRST field of
        // every table found no shape and never counted -> predictions 0.
        self.shapes.get_or_insert(table).hash_size += 1;
      }
    }
  }

  fn assign_field_expr(&mut self, expr: *mut AstExpr, index: *mut AstExpr) {
    let lv = unsafe { ast_node_as::<AstExprLocal>(expr as *mut AstNode) };
    if lv.is_null() {
      return;
    }

    let table_opt = self.tables.find(&unsafe { (*lv).local });
    let table = match table_opt {
      Some(t) => *t,
      None => return,
    };

    let number = unsafe { ast_node_as::<AstExprConstantNumber>(index as *mut AstNode) };
    if !number.is_null() {
      // C++ `shapes[*table]` inserts-on-miss; `find_mut` did not, so array
      // predictions never started.
      let shape = self.shapes.get_or_insert(table);
      if unsafe { (*number).value } == (shape.array_size as f64 + 1.0) {
        shape.array_size += 1;
      }
    } else {
      let iter = unsafe { ast_node_as::<AstExprLocal>(index as *mut AstNode) };
      if !iter.is_null()
        && let Some(&bound) = self.loops.find(&unsafe { (*iter).local })
      {
        let shape = self.shapes.get_or_insert(table);
        if shape.array_size == 0 {
          shape.array_size = bound;
        }
      }
    }
  }

  fn assign(&mut self, var: *mut AstExpr) {
    let index_name = unsafe { ast_node_as::<AstExprIndexName>(var as *mut AstNode) };
    if !index_name.is_null() {
      self.assign_field_name(unsafe { (*index_name).expr }, unsafe {
        (*index_name).index
      });
      return;
    }

    let index_expr = unsafe { ast_node_as::<AstExprIndexExpr>(var as *mut AstNode) };
    if !index_expr.is_null() {
      self.assign_field_expr(unsafe { (*index_expr).expr }, unsafe {
        (*index_expr).index
      });
    }
  }
}

impl<'a> AstVisitor for ShapeVisitor<'a> {
  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    let node = unsafe { &mut *(node as *mut AstStatLocal) };

    if node.vars.size == 1 && node.values.size == 1 {
      let value = unsafe { *node.values.data.add(0) };
      // C++ uses getTableHint, which unwraps `setmetatable(table_literal, ...)` to the
      // inner table literal. Casting the initializer straight to AstExprTable missed
      // that form, so a table behind setmetatable was never tracked and its predicted
      // shape stayed (0,0) -> NEWTABLE with size 0.
      let table = get_table_hint(value);
      if !table.is_null() && unsafe { (*table).items.size } == 0 {
        let var = unsafe { *node.vars.data.add(0) };
        self.tables.try_insert(var, table);
      }
    }

    true
  }

  fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    let node = unsafe { &mut *(node as *mut AstStatAssign) };

    for &var in node.vars.iter() {
      self.assign(var);
    }

    for &value in node.values.iter() {
      unsafe { ast_expr_visit(value, self as &mut dyn AstVisitor) };
    }

    false
  }

  fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    let node = unsafe { &mut *(node as *mut AstStatFunction) };

    self.assign(node.name);

    unsafe { ast_expr_visit(node.func as *mut AstExpr, self as &mut dyn AstVisitor) };

    false
  }

  fn visit_stat_for(&mut self, node: *mut c_void) -> bool {
    let node = unsafe { &mut *(node as *mut AstStatFor) };

    let from = unsafe { ast_node_as::<AstExprConstantNumber>(node.from as *mut AstNode) };
    let to = unsafe { ast_node_as::<AstExprConstantNumber>(node.to as *mut AstNode) };

    if !from.is_null() && !to.is_null() {
      let from_val = unsafe { (*from).value };
      let to_val = unsafe { (*to).value };

      if from_val == 1.0 && (1.0..=16.0).contains(&to_val) && node.step.is_null() {
        self.loops.try_insert(node.var, to_val as u32);
      }
    }

    true
  }
}
