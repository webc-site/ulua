//! Source: `Compiler/src/ConstantFolding.cpp:944-1396`

use alloc::vec::Vec;
use core::{
  ffi::{c_char, c_void},
  ptr::null_mut,
};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs, ast_local::AstLocal, ast_name::AstName,
    ast_name_table::AstNameTable, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_local::AstStatLocal, ast_visitor::AstVisitor,
  },
  rtti::{ast_node_as, ast_node_is},
  visit::ast_stat_visit,
};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::{
    table_constant_kind::{TableConstantKind, TableConstantKind::ConstantTable},
    type_constant_folding::Type::{Boolean, Integer, Nil, Number, String, Table, Unknown, Vector},
  },
  functions::{
    fold_binary::fold_binary, fold_builtin::fold_builtin, fold_builtin_math::fold_builtin_math,
    fold_interp_string::fold_interp_string, fold_unary::fold_unary,
  },
  records::{
    constant::Constant, expr_constant_change::ExprConstantChange,
    local_constant_change::LocalConstantChange, variable::Variable,
  },
  type_aliases::{
    compile_constant::CompileConstant, expr_constant_change_log::ExprConstantChangeLog,
    library_member_constant_callback::LibraryMemberConstantCallback,
    local_constant_change_log::LocalConstantChangeLog,
  },
};

#[derive(Debug)]
pub struct ConstantVisitorArgs<'a> {
  pub constants: &'a mut DenseHashMap<*mut AstExpr, Constant>,
  pub variables: &'a mut DenseHashMap<*mut AstLocal, Variable>,
  pub locals: &'a mut DenseHashMap<*mut AstLocal, Constant>,
  pub builtins: *const DenseHashMap<*mut AstExprCall, i32>,
  pub fold_library_k: bool,
  pub library_member_constant_cb: LibraryMemberConstantCallback,
  pub string_table: &'a mut AstNameTable,
  pub constant_table_locals: &'a DenseHashMap<*mut AstLocal, TableConstantKind>,
  pub expr_change_log: *mut ExprConstantChangeLog,
  pub local_change_log: *mut LocalConstantChangeLog,
}

#[derive(Debug)]
pub struct ConstantVisitor<'a> {
  pub(crate) constants: &'a mut DenseHashMap<*mut AstExpr, Constant>,
  pub(crate) variables: &'a mut DenseHashMap<*mut AstLocal, Variable>,
  pub(crate) locals: &'a mut DenseHashMap<*mut AstLocal, Constant>,
  pub(crate) builtins: *const DenseHashMap<*mut AstExprCall, i32>,
  pub(crate) fold_library_k: bool,
  pub(crate) library_member_constant_cb: LibraryMemberConstantCallback,
  pub(crate) string_table: &'a mut AstNameTable,
  pub(crate) constant_tables: Vec<DenseHashMap<AstName, Constant>>,
  pub(crate) was_empty: bool,
  pub(crate) builtin_args: Vec<Constant>,
  pub(crate) constant_table_locals: &'a DenseHashMap<*mut AstLocal, TableConstantKind>,
  pub(crate) table_locals: DenseHashMap<*mut AstLocal, Constant>,
  pub(crate) expr_change_log: *mut ExprConstantChangeLog,
  pub(crate) local_change_log: *mut LocalConstantChangeLog,
}

impl<'a> ConstantVisitor<'a> {
  pub fn new(args: ConstantVisitorArgs<'a>) -> Self {
    // C++ 构造函数初始化列表对应：constant_tables 预留 16 容量，避免多次扩容
    let constant_tables = Vec::with_capacity(16);

    let table_locals = DenseHashMap::new(null_mut());

    let was_empty = args.constants.empty() && args.locals.empty();

    Self {
      constants: args.constants,
      variables: args.variables,
      locals: args.locals,
      builtins: args.builtins,
      fold_library_k: args.fold_library_k,
      library_member_constant_cb: args.library_member_constant_cb,
      string_table: args.string_table,
      constant_tables,
      was_empty,
      builtin_args: Vec::new(),
      constant_table_locals: args.constant_table_locals,
      table_locals,
      expr_change_log: args.expr_change_log,
      local_change_log: args.local_change_log,
    }
  }

  fn analyze(&mut self, node: *mut AstExpr) -> Constant {
    // C++ `Constant result; result.type = Constant::Type_Unknown;`
    let mut result = Constant {
      r#type: Unknown,
      ..Constant::default()
    };

    let _node_ref = unsafe { &*node };

    if let Some(expr) = unsafe { ast_node_as::<AstExprGroup>(node as *mut AstNode).as_mut() } {
      result = self.analyze(expr.expr);
    } else if ast_node_is::<AstExprConstantNil>(node as *mut AstNode) {
      result.r#type = Nil;
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprConstantBool>(node as *mut AstNode).as_mut() }
    {
      result.r#type = Boolean;
      result.data.value_boolean = expr.value;
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprConstantNumber>(node as *mut AstNode).as_mut() }
    {
      result.r#type = Number;
      result.data.value_number = expr.value;
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprConstantInteger>(node as *mut AstNode).as_mut() }
    {
      result.r#type = Integer;
      result.data.value_integer64 = expr.value;
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprConstantString>(node as *mut AstNode).as_mut() }
    {
      result.r#type = String;
      result.data.value_string = expr.value.data as *const c_char;
      result.string_length = expr.value.size as u32;
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprLocal>(node as *mut AstNode).as_mut() }
    {
      if let Some(l) = self.locals.find(&expr.local) {
        result = *l;
      } else if FFlag::LuauCompileFoldOptimize.get()
        && let Some(l) = self.table_locals.find(&expr.local)
      {
        result = *l;
      }
    } else if ast_node_is::<AstExprGlobal>(node as *mut AstNode)
      || ast_node_is::<AstExprVarargs>(node as *mut AstNode)
    {
      // nope
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprCall>(node as *mut AstNode).as_mut() }
    {
      self.analyze(expr.func);

      let bfid = if !self.builtins.is_null() {
        unsafe { &*self.builtins }.find(&(expr as *mut AstExprCall))
      } else {
        None
      };

      if let Some(bfid_ptr) = bfid {
        if *bfid_ptr != 0 {
          let offset = self.builtin_args.len();
          let mut can_fold = true;

          self.builtin_args.reserve(offset + expr.args.size);

          for &arg in expr.args.iter() {
            let ac = self.analyze(arg);

            if FFlag::LuauCompilePropagateTableProps2.get() {
              if ac.r#type == Unknown || ac.r#type == Table {
                can_fold = false;
              } else {
                self.builtin_args.push(ac);
              }
            } else if ac.r#type == Unknown {
              can_fold = false;
            } else {
              self.builtin_args.push(ac);
            }
          }
          if can_fold {
            LUAU_ASSERT!(self.builtin_args.len() == offset + expr.args.size);
            result = unsafe {
              fold_builtin(
                self.string_table,
                *bfid_ptr,
                self.builtin_args.as_ptr().add(offset),
                expr.args.size,
              )
            };
          }

          self.builtin_args.resize(offset, Constant::default());
        } else {
          for &arg in expr.args.iter() {
            self.analyze(arg);
          }
        }
      } else {
        for &arg in expr.args.iter() {
          self.analyze(arg);
        }
      }
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprIndexName>(node as *mut AstNode).as_mut() }
    {
      let value = self.analyze(expr.expr);
      if FFlag::LuauCompilePropagateTableProps2.get() && value.r#type == Table {
        let table_index = unsafe { value.data.value_table };
        LUAU_ASSERT!(table_index < self.constant_tables.len());
        if table_index < self.constant_tables.len() {
          let props = &self.constant_tables[table_index];
          if let Some(prop) = props.find(&expr.index) {
            result = *prop;
          }
        }
      } else if value.r#type == Vector {
        match expr.index.as_bytes() {
          b"x" | b"X" => {
            result.r#type = Number;
            result.data.value_number = unsafe { value.data.value_vector[0] as f64 };
          }
          b"y" | b"Y" => {
            result.r#type = Number;
            result.data.value_number = unsafe { value.data.value_vector[1] as f64 };
          }
          b"z" | b"Z" => {
            result.r#type = Number;
            result.data.value_number = unsafe { value.data.value_vector[2] as f64 };
          }
          _ => {}
        }

        // Do not handle 'w' component because it isn't known if the runtime will be configured in 3-wide or 4-wide mode
        // In 3-wide, access to 'w' will call unspecified metamethod or fail
      } else if self.fold_library_k
        && let Some(eg) =
          unsafe { ast_node_as::<AstExprGlobal>(expr.expr as *mut AstNode).as_mut() }
      {
        if eg.name == "math" {
          result = fold_builtin_math(expr.index);
        }

        if let Some(cb) = self
          .library_member_constant_cb
          .filter(|_| result.r#type == Unknown)
        {
          // C++ passes reinterpret_cast<CompileConstant*>(&result): the
          // pointer VALUE handed to the callback must be &result itself.
          let constant_ptr = &mut result as *mut Constant as *mut CompileConstant;
          unsafe { cb(eg.name.value, expr.index.value, constant_ptr) };
        }
      }
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprIndexExpr>(node as *mut AstNode).as_mut() }
    {
      let index_val = self.analyze(expr.index);
      let table_val = self.analyze(expr.expr);

      if FFlag::LuauCompilePropagateTableProps2.get()
        && table_val.r#type == Table
        && index_val.r#type == String
      {
        let table_index = unsafe { table_val.data.value_table };
        LUAU_ASSERT!(table_index < self.constant_tables.len());
        if table_index < self.constant_tables.len() && index_val.string_length != 0 {
          let props = &self.constant_tables[table_index];
          let index_name = self
            .string_table
            .get_or_add_slice(index_val.get_string_bytes());
          if let Some(prop) = props.find(&index_name) {
            result = *prop;
          }
        }
      }
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprFunction>(node as *mut AstNode).as_mut() }
    {
      unsafe {
        ast_stat_visit(expr.body as *mut AstStat, self as &mut dyn AstVisitor);
      }
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprTable>(node as *mut AstNode).as_mut() }
    {
      if FFlag::LuauCompilePropagateTableProps2.get() {
        let mut props = DenseHashMap::new(AstName::new());
        for item in expr.items.iter() {
          let value_val = self.analyze(item.value);

          if !item.key.is_null() {
            let key_val = self.analyze(item.key);

            if key_val.r#type == String
              && value_val.r#type != Unknown
              && value_val.r#type != Table
              && key_val.string_length != 0
            {
              let const_key = self
                .string_table
                .get_or_add_slice(key_val.get_string_bytes());
              props.try_insert(const_key, value_val);
            }
          }
        }

        if props.size() == expr.items.size {
          result.r#type = Table;
          result.data.value_table = self.constant_tables.len();
          self.constant_tables.push(props);
        }
      } else {
        for item in expr.items.iter() {
          if !item.key.is_null() {
            self.analyze(item.key);
          }

          self.analyze(item.value);
        }
      }
    } else if let Some(expr) = unsafe { ast_node_as::<AstExprUnary>(node as *mut AstNode).as_mut() }
    {
      let arg = self.analyze(expr.expr);

      if arg.r#type != Unknown {
        fold_unary(&mut result, expr.op, &arg);
      }
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprBinary>(node as *mut AstNode).as_mut() }
    {
      let la = self.analyze(expr.left);
      let ra = self.analyze(expr.right);

      if la.r#type != Unknown {
        fold_binary(&mut result, expr.op, &la, &ra, self.string_table);
      }
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprTypeAssertion>(node as *mut AstNode).as_mut() }
    {
      let arg = self.analyze(expr.expr);
      result = arg;
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprIfElse>(node as *mut AstNode).as_mut() }
    {
      let cond = self.analyze(expr.condition);
      let true_expr = self.analyze(expr.true_expr);
      let false_expr = self.analyze(expr.false_expr);

      if cond.r#type != Unknown {
        result = if cond.is_truthful() {
          true_expr
        } else {
          false_expr
        };
      }
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprInterpString>(node as *mut AstNode).as_mut() }
    {
      // C++ analyzes EVERY sub-expression (no early break) so that constants
      // are recorded for all of them — including local references nested in
      // later, non-constant interpolations. A `break` here left those refs
      // unfolded; a constant local whose declaration is then elided
      // (areLocalsRedundant) would reach compile_expr with no register and
      // trip the upvalue assert.
      let mut only_constant_sub_expr = true;
      for &sub in expr.expressions.iter() {
        if self.analyze(sub).r#type != String {
          only_constant_sub_expr = false;
        }
      }

      if only_constant_sub_expr {
        unsafe { fold_interp_string(&mut result, expr, self.constants, self.string_table) };
      }
    } else if let Some(expr) =
      unsafe { ast_node_as::<AstExprInstantiate>(node as *mut AstNode).as_mut() }
    {
      result = self.analyze(expr.expr);
    } else {
      LUAU_ASSERT!(false, "Unknown expression type");
    }

    self.record_expr_constant(node, result);

    result
  }

  fn record_expr_constant(&mut self, key: *mut AstExpr, value: Constant) {
    if FFlag::LuauCompileFoldOptimize.get() && FFlag::LuauCompilePropagateTableProps2.get() {
      if value.r#type == Table {
        // Table constants are recorded in a separate map
      } else if value.r#type != Unknown {
        self.log_expr_change(key, None);
        *self.constants.get_or_insert(key) = value;
      } else if self.was_empty {
        // No need to clear out entries if we started with empty maps
      } else if let Some(old) = self.constants.find(&key).copied() {
        self.log_expr_change(key, Some(old));
        // C++ `old->type = Unknown`: clear the STALE entry. try_insert is a no-op
        // when the key exists, so the stale constant survived across inline re-folds.
        *self.constants.get_or_insert(key) = Constant::default();
      }
    } else {
      if value.r#type != Unknown {
        *self.constants.get_or_insert(key) = value;
      } else if self.was_empty && !FFlag::LuauCompilePropagateTableProps2.get() {
        // nothing
      } else if self.constants.find(&key).is_some() {
        // C++ `old->type = Unknown`: clear the STALE entry. try_insert is a no-op
        // when the key exists, so the stale constant survived across inline re-folds.
        *self.constants.get_or_insert(key) = Constant::default();
      }
    }
  }

  fn record_local_constant(&mut self, key: *mut AstLocal, value: Constant) {
    if FFlag::LuauCompileFoldOptimize.get() && FFlag::LuauCompilePropagateTableProps2.get() {
      if value.r#type == Table {
        // Table constants are recorded in a separate map
      } else if value.r#type != Unknown {
        self.log_local_change(key, None);
        *self.locals.get_or_insert(key) = value;
      } else if self.was_empty {
        // No need to clear out entries if we started with empty maps
      } else if let Some(old) = self.locals.find(&key).copied() {
        self.log_local_change(key, Some(old));
        *self.locals.get_or_insert(key) = Constant::default();
      }
    } else {
      if value.r#type != Unknown {
        *self.locals.get_or_insert(key) = value;
      } else if self.was_empty && !FFlag::LuauCompilePropagateTableProps2.get() {
        // nothing
      } else if self.locals.find(&key).is_some() {
        *self.locals.get_or_insert(key) = Constant::default();
      }
    }
  }

  fn log_expr_change(&mut self, key: *mut AstExpr, existing: Option<Constant>) {
    if self.expr_change_log.is_null() {
      return;
    }

    let old = existing
      .or_else(|| self.constants.find(&key).copied())
      .unwrap_or_default();
    let was_absent = existing.is_none() && self.constants.find(&key).is_none();

    let log = unsafe { &mut *self.expr_change_log };
    log.push(ExprConstantChange {
      key,
      old_value: old,
      was_absent,
    });
  }

  fn log_local_change(&mut self, key: *mut AstLocal, existing: Option<Constant>) {
    if self.local_change_log.is_null() {
      return;
    }

    let old = existing
      .or_else(|| self.locals.find(&key).copied())
      .unwrap_or_default();
    let was_absent = existing.is_none() && self.locals.find(&key).is_none();

    let log = unsafe { &mut *self.local_change_log };
    log.push(LocalConstantChange {
      key,
      old_value: old,
      was_absent,
    });
  }

  fn record_value(&mut self, local: *mut AstLocal, value: Constant) {
    let v = self.variables.find_mut(&local).unwrap();

    if !v.written {
      if FFlag::LuauCompileFoldOptimize.get() && FFlag::LuauCompilePropagateTableProps2.get() {
        if value.r#type == Table {
          v.constant = false;
          self.table_locals.try_insert(local, value);
        } else {
          v.constant = value.r#type != Unknown;
          self.record_local_constant(local, value);
        }
      } else {
        v.constant = if FFlag::LuauCompilePropagateTableProps2.get() {
          value.r#type != Unknown && value.r#type != Table
        } else {
          value.r#type != Unknown
        };
        self.record_local_constant(local, value);
      }
    }
  }

  fn visit_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
    let node_ref = unsafe { &*node };

    // zip 取较短的数组，等价于 C++ 的 `min(vars.size, values.size)` 循环
    for (&local, &rhs) in node_ref.vars.iter().zip(node_ref.values.iter()) {
      let arg = self.analyze(rhs);

      if FFlag::LuauCompilePropagateTableProps2.get() && arg.r#type == Table {
        let kind = self.constant_table_locals.find(&local);
        if let Some(k) = kind {
          if *k == ConstantTable {
            self.record_value(local, arg);
          } else {
            self.record_value(local, Constant::default());
          }
        } else {
          self.record_value(local, Constant::default());
        }
      } else {
        self.record_value(local, arg);
      }
    }

    if node_ref.vars.size > node_ref.values.size {
      let last = if node_ref.values.size > 0 {
        Some(unsafe { *node_ref.values.data.add(node_ref.values.size - 1) })
      } else {
        None
      };
      let mult_ret = last.is_some_and(|l| {
        ast_node_is::<AstExprCall>(l as *mut AstNode)
          || ast_node_is::<AstExprVarargs>(l as *mut AstNode)
      });

      if !mult_ret {
        for i in node_ref.values.size..node_ref.vars.size {
          let nil = Constant {
            r#type: Nil,
            string_length: 0,
            data: Default::default(),
          };
          self.record_value(unsafe { *node_ref.vars.data.add(i) }, nil);
        }
      }
    } else {
      for i in node_ref.vars.size..node_ref.values.size {
        self.analyze(unsafe { *node_ref.values.data.add(i) });
      }
    }

    false
  }
}

impl<'a> AstVisitor for ConstantVisitor<'a> {
  fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_expr(&mut self, node: *mut c_void) -> bool {
    self.analyze(node as *mut AstExpr);
    false
  }

  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    self.visit_stat_local(node as *mut AstStatLocal);
    false
  }
}
