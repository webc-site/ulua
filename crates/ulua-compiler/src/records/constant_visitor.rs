//! Source: `Compiler/src/ConstantFolding.cpp:944-1396`

use alloc::vec::Vec;
use core::{hash::Hash, ptr::from_mut};

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_varargs::AstExprVarargs, ast_local::AstLocal, ast_name::AstName,
    ast_name_table::AstNameTable, ast_node::AstNode, ast_stat_local::AstStatLocal,
    ast_visitor::AstVisitor,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
use ulua_common::{
  enums::luau_builtin_function::LuauBuiltinFunction, macros::luau_assert::LUAU_ASSERT,
  records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::table_constant_kind::{TableConstantKind, TableConstantKind::ConstantTable},
  functions::{
    ast_slot_ref::ast_slot_ref, fold_binary::fold_binary, fold_builtin::fold_builtin,
    fold_builtin_math::fold_builtin_math, fold_interp_string::fold_interp_string,
    fold_unary::fold_unary, undo_changes_constant_folding::ChangeEntry,
  },
  records::{constant::Constant, node::Node, variable::Variable},
  type_aliases::{
    compile_constant::CompileConstant, expr_constant_change_log::ExprConstantChangeLog,
    library_member_constant_callback::LibraryMemberConstantCallback,
    local_constant_change_log::LocalConstantChangeLog,
  },
};

#[derive(Debug)]
pub struct ConstantVisitorArgs<'a> {
  pub constants: &'a mut DenseHashMap<Node<AstExpr>, Constant>,
  pub variables: &'a mut DenseHashMap<Node<AstLocal>, Variable>,
  pub locals: &'a mut DenseHashMap<Node<AstLocal>, Constant>,
  /// cpp `const foldBuiltinCall*` 空指针语义 → `Option` 借用，折叠表缺省即 `None`
  pub builtins: Option<&'a DenseHashMap<Node<AstExprCall>, i32>>,
  pub fold_library_k: bool,
  pub library_member_constant_cb: LibraryMemberConstantCallback,
  pub string_table: &'a mut AstNameTable,
  pub constant_table_locals: &'a DenseHashMap<Node<AstLocal>, TableConstantKind>,
  /// cpp `ExprConstantChangeLog*` 空指针语义 → `Option` 可变借用，不记录即 `None`
  pub expr_change_log: Option<&'a mut ExprConstantChangeLog>,
  pub local_change_log: Option<&'a mut LocalConstantChangeLog>,
}

/// 常量折叠 visitor（cpp `ConstantFolding.cpp` 的 `ConstantVisitor`）。
///
/// constants/locals/variables/builtins 等 map 的键为 [`Node`] 地址句柄：
/// 只作身份比较，与各编译阶段共享同一地址键空间。
#[derive(Debug)]
pub struct ConstantVisitor<'a> {
  /// 对外只读语义由调用方约定；提升为 pub 供集成测试检查折叠后常量表
  pub constants: &'a mut DenseHashMap<Node<AstExpr>, Constant>,
  pub(crate) variables: &'a mut DenseHashMap<Node<AstLocal>, Variable>,
  pub(crate) locals: &'a mut DenseHashMap<Node<AstLocal>, Constant>,
  pub(crate) builtins: Option<&'a DenseHashMap<Node<AstExprCall>, i32>>,
  pub(crate) fold_library_k: bool,
  pub(crate) library_member_constant_cb: LibraryMemberConstantCallback,
  pub(crate) string_table: &'a mut AstNameTable,
  pub(crate) constant_tables: Vec<DenseHashMap<AstName, Constant>>,
  pub(crate) was_empty: bool,
  pub(crate) builtin_args: Vec<Constant>,
  pub(crate) constant_table_locals: &'a DenseHashMap<Node<AstLocal>, TableConstantKind>,
  pub(crate) table_locals: DenseHashMap<Node<AstLocal>, Constant>,
  pub(crate) expr_change_log: Option<&'a mut ExprConstantChangeLog>,
  pub(crate) local_change_log: Option<&'a mut LocalConstantChangeLog>,
}

/// 表常量表预分配容量（C++ 构造函数初始化列表中的 reserve 值）
const CONSTANT_TABLES_INITIAL_CAPACITY: usize = 16;

impl<'a> ConstantVisitor<'a> {
  pub fn new(args: ConstantVisitorArgs<'a>) -> Self {
    let constant_tables = Vec::with_capacity(CONSTANT_TABLES_INITIAL_CAPACITY);

    let table_locals = DenseHashMap::default();

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

  fn analyze(&mut self, node: Node<AstExpr>) -> Constant {
    // C++ `Constant result; result.type = Constant::Type_Unknown;` — Default 即 Unknown
    let mut result = Constant::default();

    match node.as_expr_ref() {
      AstExprRef::Group(expr) => {
        result = self.analyze(expr.expr.into());
      }
      AstExprRef::ConstantNil(_) => {
        result = Constant::Nil;
      }
      AstExprRef::ConstantBool(expr) => {
        result = Constant::Boolean(expr.value);
      }
      AstExprRef::ConstantNumber(expr) => {
        result = Constant::Number(expr.value);
      }
      AstExprRef::ConstantInteger(expr) => {
        result = Constant::Integer(expr.value);
      }
      AstExprRef::ConstantString(expr) => {
        result = Constant::string(expr.value.data, expr.value.size as u32);
      }
      AstExprRef::Local(expr) => {
        if let Some(l) = self.locals.find(&expr.local.into()) {
          result = *l;
        } else if let Some(l) = self.table_locals.find(&expr.local.into()) {
          result = *l;
        }
      }
      AstExprRef::Global(_) | AstExprRef::Varargs(_) => {
        // 不折叠，直接落空
      }
      AstExprRef::Call(expr) => {
        self.analyze(expr.func.into());

        let bfid = self
          .builtins
          .and_then(|builtins| builtins.find(&node.cast::<AstExprCall>()));

        // 表内 LBF_NONE 条目（内建 apply/restore 会留下真实的 0 值，绝不可 FASTCALL）
        // 与「未登记内建」同走「仅递归实参」一路，两分支合并为一次过滤判定。
        match bfid.filter(|id| **id != LuauBuiltinFunction::LBF_NONE as i32) {
          Some(bfid_ptr) => {
            let offset = self.builtin_args.len();
            let mut can_fold = true;

            self.builtin_args.reserve(offset + expr.args.size);

            for arg in expr.args.iter() {
              let ac = self.analyze((*arg).into());

              if !ac.is_unknown() && !matches!(ac, Constant::Table(_)) {
                self.builtin_args.push(ac);
              } else {
                can_fold = false;
              }
            }
            if can_fold {
              LUAU_ASSERT!(self.builtin_args.len() == offset + expr.args.size);
              // 借用切片视图取代裸 `as_ptr().add(offset)`：可折参数已全部
              // 落进 `builtin_args[offset..]`，长度即实参数。
              result = fold_builtin(self.string_table, *bfid_ptr, &self.builtin_args[offset..]);
            }

            self.builtin_args.resize(offset, Constant::default());
          }
          None => {
            for arg in expr.args.iter() {
              self.analyze((*arg).into());
            }
          }
        }
      }
      AstExprRef::IndexName(expr) => {
        let value = self.analyze(expr.expr.into());
        match value {
          Constant::Table(table_index) => {
            LUAU_ASSERT!(table_index < self.constant_tables.len());
            if table_index < self.constant_tables.len()
              && let Some(prop) = self.constant_tables[table_index].find(&expr.index)
            {
              result = *prop;
            }
          }
          Constant::Vector(v) => match expr.index.as_bytes() {
            b"x" | b"X" => result = Constant::Number(f64::from(v[0])),
            b"y" | b"Y" => result = Constant::Number(f64::from(v[1])),
            b"z" | b"Z" => result = Constant::Number(f64::from(v[2])),
            _ => {}
          },
          _ if self.fold_library_k => {
            // expr.expr 已句柄化恒非空；经 as_ptr 桥接进 `ast_slot_ref` 只读物化后走安全引用判型。
            if let Some(eg) =
              ast_slot_ref(expr.expr.as_ptr()).and_then(|e| ast_node_try_as::<AstExprGlobal>(e))
            {
              if eg.name == "math" {
                result = fold_builtin_math(expr.index);
              }

              if let Some(cb) = self
                .library_member_constant_cb
                .filter(|_| result.is_unknown())
              {
                // C++ 传 `reinterpret_cast<CompileConstant*>(&result)`：交给回调的
                // 指针值必须恰为 &result 本身（CompileConstant 即 *mut ()）。
                let constant_ptr = from_mut(&mut result).cast::<CompileConstant>();
                // Safety: cb 为 Some 的非空 extern "C-unwind" 函数指针（宿主注册）；
                // eg.name.value / expr.index.value 是名字表登记的非空 NUL 结尾 C 串；
                // constant_ptr 是活跃栈对象 result 的地址——与 cpp 一致，回调只把该
                // “指针 VALUE”当作 CompileConstant 槽位转交 set_compile_constant_*，
                // 后者整体覆盖写回合法 Constant 值，不读取中间态。
                unsafe { cb(eg.name.value, expr.index.value, constant_ptr) };
              }
            }
          }
          _ => {}
        }
      }
      AstExprRef::IndexExpr(expr) => {
        let index_val = self.analyze(expr.index.into());
        let table_val = self.analyze(expr.expr.into());

        if let (Constant::Table(table_index), Constant::Str { .. }) = (&table_val, &index_val) {
          LUAU_ASSERT!(*table_index < self.constant_tables.len());
          if *table_index < self.constant_tables.len() && index_val.string_len() != 0 {
            let props = &self.constant_tables[*table_index];
            let index_name = self
              .string_table
              .get_or_add_slice(index_val.get_string_bytes());
            if let Some(prop) = props.find(&index_name) {
              result = *prop;
            }
          }
        }
      }
      AstExprRef::Function(expr) => {
        // cpp `expr->body->visit(this)`：body 静态类型即 AstStatBlock，直调免 class-index 分发。
        // Safety: expr.body 为 parser 保证非空存活的函数体 AstStatBlock；ConstantVisitor
        // 只写自身 map，不改 AST 节点。
        let body = unsafe { &mut *expr.body.as_ptr() };
        ast_stat_block_visit(body, self);
      }
      AstExprRef::Table(expr) => {
        let mut props = DenseHashMap::default();
        for item in expr.items.iter() {
          let value_val = self.analyze(item.value.into());

          if !item.key.is_null() {
            let key_val = self.analyze(item.key.into());

            if let (Constant::Str(s), _) = (&key_val, &value_val)
              && s.len != 0
              && !value_val.is_unknown()
              && !matches!(value_val, Constant::Table(_))
            {
              let const_key = self
                .string_table
                .get_or_add_slice(key_val.get_string_bytes());
              props.try_insert(const_key, value_val);
            }
          }
        }

        if props.size() == expr.items.size {
          result = Constant::Table(self.constant_tables.len());
          self.constant_tables.push(props);
        }
      }
      AstExprRef::Unary(expr) => {
        let arg = self.analyze(expr.expr.into());

        if !arg.is_unknown() {
          result = fold_unary(expr.op, &arg);
        }
      }
      AstExprRef::Binary(expr) => {
        let la = self.analyze(expr.left.into());
        let ra = self.analyze(expr.right.into());

        if !la.is_unknown() {
          result = fold_binary(expr.op, &la, &ra, self.string_table);
        }
      }
      AstExprRef::TypeAssertion(expr) => {
        let arg = self.analyze(expr.expr.into());
        result = arg;
      }
      AstExprRef::IfElse(expr) => {
        let cond = self.analyze(expr.condition.into());
        let true_expr = self.analyze(expr.true_expr.into());
        let false_expr = self.analyze(expr.false_expr.into());

        if !cond.is_unknown() {
          result = if cond.is_truthful() {
            true_expr
          } else {
            false_expr
          };
        }
      }
      AstExprRef::InterpString(expr) => {
        let mut only_constant_sub_expr = true;
        for sub in expr.expressions.iter() {
          if !matches!(self.analyze((*sub).into()), Constant::Str(_)) {
            only_constant_sub_expr = false;
          }
        }

        if only_constant_sub_expr {
          result = fold_interp_string(expr, self.constants, self.string_table);
        }
      }
      AstExprRef::Instantiate(expr) => {
        result = self.analyze(expr.expr.into());
      }
      AstExprRef::Error(_) => {
        LUAU_ASSERT!(false, "Unknown expression type");
      }
    }

    self.record_expr_constant(node, result);

    result
  }

  pub fn record_expr_constant(&mut self, key: Node<AstExpr>, value: Constant) {
    record_constant(
      self.constants,
      &mut self.expr_change_log,
      key,
      value,
      self.was_empty,
    );
  }

  pub(crate) fn record_local_constant(&mut self, key: Node<AstLocal>, value: Constant) {
    record_constant(
      self.locals,
      &mut self.local_change_log,
      key,
      value,
      self.was_empty,
    );
  }

  fn record_value(&mut self, local: Node<AstLocal>, value: Constant) {
    let v = self.variables.find_mut(&local).unwrap();

    if !v.written {
      if matches!(value, Constant::Table(_)) {
        v.constant = false;
        self.table_locals.try_insert(local, value);
      } else {
        v.constant = !value.is_unknown();
        self.record_local_constant(local, value);
      }
    }
  }
}

/// Expr / Local 两类常量登记同构（cpp `ConstantVisitor` 的 constants/locals 双
/// map）：键与日志条目泛型化后共用一份「Table 走独立 map、非 Unknown 覆写、
/// Unknown 清 STALE」骨架。
fn record_constant<K, Ent>(
  map: &mut DenseHashMap<K, Constant>,
  change_log: &mut Option<&mut Vec<Ent>>,
  key: K,
  value: Constant,
  was_empty: bool,
) where
  K: Copy + Clone + PartialEq + Hash,
  Ent: ChangeEntry<K>,
{
  if matches!(value, Constant::Table(_)) {
    // Table 常量记录在独立的 map 中，本分支有意留空
  } else if !value.is_unknown() {
    log_change(map, change_log, key, None);
    *map.get_or_insert(key) = value;
  } else if was_empty {
    // 进入时 map 本就为空则无需清理条目，本分支有意留空
  } else if let Some(old) = map.find(&key).copied() {
    // 对应 C++ `old->type = Unknown`：清掉 STALE 条目。try_insert 在键已
    // 存在时是 no-op，导致陈旧常量跨内联重折叠存活。
    // find + get_or_insert 两次哈希：借用安全前提下已是最少次数。
    log_change(map, change_log, key, Some(old));
    *map.get_or_insert(key) = Constant::default();
  }
}

/// 回滚日志登记：未携带旧值时单次 find 同时得出 old 与 was_absent。
fn log_change<K, Ent>(
  map: &DenseHashMap<K, Constant>,
  change_log: &mut Option<&mut Vec<Ent>>,
  key: K,
  existing: Option<Constant>,
) where
  K: Copy + Clone + PartialEq + Hash,
  Ent: ChangeEntry<K>,
{
  let Some(log) = change_log else {
    return;
  };

  let (old_value, was_absent) = match existing {
    Some(value) => (value, false),
    None => match map.find(&key).copied() {
      Some(value) => (value, false),
      None => (Constant::default(), true),
    },
  };

  log.push(Ent::from_parts(key, old_value, was_absent));
}

impl<'a> AstVisitor for ConstantVisitor<'a> {
  fn visit_node(&mut self, _node: &mut AstNode) -> bool {
    true
  }

  /// 表达式基类 hook：`analyze` 以节点地址为常量表键（句柄模型），
  /// `from_mut` 仅造键指针、不解引用。返回 false：分析在本 hook 内完成。
  fn visit_expr(&mut self, node: &mut AstExpr) -> bool {
    self.analyze(Node::from_mut(node));
    false
  }

  /// Source: cpp `ConstantVisitor::visit(AstStatLocal*)`
  ///
  /// 分发链交付 `&mut`（非空由类型证明），局部变量的常量性在此登记；
  /// 返回 false：子树已由 `Self::analyze` 逐值分析，不再走通用 walker。
  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    // zip 取较短的数组，等价于 C++ 的 `min(vars.size, values.size)` 循环
    for (&local, &rhs) in node.vars.iter().zip(node.values.iter()) {
      let arg = self.analyze(rhs.into());

      if matches!(arg, Constant::Table(_)) {
        // 表常量仅在其 local 被标记为 ConstantTable 时才记录，否则按 Unknown 处理
        let is_constant_table = self
          .constant_table_locals
          .find(&local.into())
          .is_some_and(|k| *k == ConstantTable);
        self.record_value(
          local.into(),
          if is_constant_table {
            arg
          } else {
            Constant::default()
          },
        );
      } else {
        self.record_value(local.into(), arg);
      }
    }

    if node.vars.size > node.values.size {
      // 尾变量是否多值返回：取决于最后一个值是否为 call/varargs
      let last = node.values.iter().last().copied();
      let mult_ret =
        // values 槽位为 arena 存活 AstExpr 指针；`ast_slot_ref` + 安全判型门面。
        last.is_some_and(|l| {
          ast_slot_ref(l).is_some_and(|e| {
            ast_node_is::<AstExprCall>(e) || ast_node_is::<AstExprVarargs>(e)
          })
        });

      if !mult_ret {
        // 尾部多余变量折叠为 nil 常量
        for &var in node.vars.iter().skip(node.values.size) {
          self.record_value(var.into(), Constant::Nil);
        }
      }
    } else {
      // 值多于变量：仍需分析以传播函数体内的常量信息
      for &value in node.values.iter().skip(node.vars.size) {
        self.analyze(value.into());
      }
    }

    false
  }
}
