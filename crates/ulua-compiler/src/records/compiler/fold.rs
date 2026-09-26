//! `Compiler` 常量折叠与编译期常量查询：`is_constant`/`get_constant*`/
//! `fold_constants`/常量 table 索引（对照 cpp `Compiler.cpp` 常量折叠段）。
use ulua_ast::records::{
  ast_expr::AstExpr,
  ast_expr_index_name::AstExprIndexName,
  ast_expr_local::AstExprLocal,
  ast_expr_table::{AstExprTable, ItemKind},
  ast_node::AstNode,
  location::Location,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::table_constant_kind::TableConstantKind,
  functions::{
    fold_constants::{FoldConstantsArgs, fold_constants},
    sref_compiler::sref_ast_array_u8,
    unwrap_expr_of_type::unwrap_expr_of_type,
  },
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::Compiler,
    constant::Constant,
    node::Node,
  },
};
impl Compiler {}

impl Compiler {
  pub(crate) fn check_constant(&mut self, constant: i32, location: &Location) {
    if constant < 0 {
      CompileError::raise(
        location,
        core::format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
      );
    }
  }

  /// 对 `root` 子树跑一遍常量折叠。折叠只写 constants/variables/locstants
  /// 映射与名字表、不改写 AST 节点；`record_changes` 控制是否登记可回滚变更。
  pub(crate) fn fold_constants(&mut self, root: &mut AstNode, record_changes: bool) {
    // 变更记录开关用 Option 借用表达（cpp 传 nullptr 或 &changeLog）；
    // 字段解构拆借用，各映射表/名表互不相交。
    let Self {
      constants,
      variables,
      locstants,
      builtins,
      builtins_fold,
      builtins_fold_library_k,
      options,
      names,
      table_constants,
      expr_changes,
      local_changes,
      ..
    } = self;
    // Safety: names 句柄在 Compiler 构造时由 `&mut AstNameTable` 接线（非空、
    // 唯一归属 self、比编译存活），该 &mut 仅用于向字符串表 intern 新名字。
    let string_table = unsafe { names.as_mut() };
    fold_constants(
      root,
      FoldConstantsArgs {
        constants,
        variables,
        locals: locstants,
        builtins: if *builtins_fold {
          Some(&*builtins)
        } else {
          None
        },
        fold_library_k: *builtins_fold_library_k,
        library_member_constant_cb: options.library_member_constant_cb,
        string_table,
        table_constants,
        expr_change_log: record_changes.then_some(expr_changes),
        local_change_log: record_changes.then_some(local_changes),
      },
    );
  }

  pub fn get_constant(&mut self, node: impl Into<Node<AstExpr>>) -> Constant {
    // Default 即 Unknown 占位（值为零串空指针），免去 union zeroed 的 unsafe
    self
      .constants
      .find(&node.into())
      .copied()
      .unwrap_or_default()
  }

  /// 本函数只读：`node` 以引用证明存活，命中 `constants` 表说明 fold 阶段已登记
  /// 该 parser 节点；字符串常量分支读取的 `string_data` 指向 parser 字符串表
  /// （编译期存活）。
  pub(crate) fn get_constant_index(&mut self, node: &AstExpr) -> i32 {
    // constants 表以节点句柄为键，由借用升格；引用存活证明即原指针契约。
    let constant = match self.constants.find(&node.into()) {
      Some(c) if !c.is_unknown() => c,
      _ => return -1,
    };
    let cid = match *constant {
      Constant::Nil => self.bc_mut().add_constant_nil(),
      Constant::Boolean(b) => self.bc_mut().add_constant_boolean(b),
      Constant::Number(n) => self.bc_mut().add_constant_number(n),
      Constant::Integer(l) => self.bc_mut().add_constant_integer(l),
      Constant::Vector([x, y, z, w]) => self.bc_mut().add_constant_vector(x, y, z, w),
      Constant::Str(_) => {
        let string_data = constant.get_string();
        self
          .bc_mut()
          .add_constant_string(sref_ast_array_u8(string_data))
      }
      // 仅 Unknown / Table 会到达
      _ => {
        LUAU_ASSERT!(false);
        return -1;
      }
    };

    self.check_constant(cid, &node.base.location);

    cid
  }

  /// 本函数只读：`node` 以引用证明存活，constants 表键由借用还原为地址。
  pub(crate) fn get_constant_number(&mut self, node: &AstExpr) -> i32 {
    // Constant 为 Copy：copied() 立即取走值，常量表借用先于 bc_mut 结束。
    if let Some(Constant::Number(n)) = self.constants.find(&node.into()).copied() {
      let cid = self.bc_mut().add_constant_number(n);
      self.check_constant(cid, &node.base.location);
      return cid;
    }

    -1
  }

  pub fn is_constant(&self, node: impl Into<Node<AstExpr>>) -> bool {
    self
      .constants
      .find(&node.into())
      .is_some_and(|cv| !cv.is_unknown())
  }

  pub(crate) fn is_constant_integer(&self, node: impl Into<Node<AstExpr>>) -> bool {
    self
      .constants
      .find(&node.into())
      .is_some_and(|cv| matches!(cv, Constant::Integer(_)))
  }

  pub(crate) fn is_constant_vector(&self, node: impl Into<Node<AstExpr>>) -> bool {
    self
      .constants
      .find(&node.into())
      .is_some_and(|cv| matches!(cv, Constant::Vector(_)))
  }

  /// C++ `Compiler::getExprLocal`：即 `unwrapExprOfType<AstExprLocal>`。
  /// 旧上游按 `LuauCompileInlineTableFunctions` 分叉的两份实现已合并。
  /// null 哨兵已 Rust 化为 `Option`。
  pub(crate) fn get_expr_local(
    &mut self,
    node: impl Into<Node<AstExpr>>,
  ) -> Option<Node<AstExprLocal>> {
    unwrap_expr_of_type::<AstExprLocal>(node.into())
  }

  /// 对应 cpp `Compiler::tryIndexConstantTable`：对常量表 local 的下标求值。
  /// 原 `pub unsafe fn` + null 哨兵返回已 Rust 化：入参收紧为引用（判空由调用方
  /// 的类型系统承担），未命中路径统一返回 `None`，命中返回表项值表达式句柄
  /// （arena 地址身份，后续仅作 map 键/只读遍历入参）。
  pub(crate) fn try_index_constant_table(&self, expr: &AstExprIndexName) -> Option<Node<AstExpr>> {
    let table_local =
      unwrap_expr_of_type::<AstExprLocal>(expr.expr.into()).map(|node| node.borrow())?;

    let lv = *self.variables.find(&table_local.local.into())?;
    if lv.written {
      return None;
    }
    // cpp `lv->init == nullptr` 判空 → Option（`?` 即早退）
    let init = lv.init?;

    if *self.table_constants.find(&table_local.local.into())? != TableConstantKind::ConstantTable {
      return None;
    }

    let table = unwrap_expr_of_type::<AstExprTable>(init).map(|node| node.borrow())?;

    // 语义同 cpp `match_value`：命中的表项会被后续「常量未登记」的表项重置，
    // 折叠为迭代器 scan 保留该行为。
    table.items.as_slice().iter().fold(None, |acc, item| {
      if !matches!(item.kind, ItemKind::Record | ItemKind::General) {
        return acc;
      }
      match self.constants.find(&item.key.into()) {
        Some(Constant::Str(s)) if s.len != 0 => {
          // Safety: self.names 句柄在构造点由 `&mut AstNameTable` 接线（非空、
          // 比 self 长寿）；get_or_add_slice 只在名表内 intern，与 self 其余字段
          // 不相交。s.ptr/s.len 来自已录入的 Str 常量键，为合法字符串区间且
          // 外层已判 len != 0。
          let key_name = unsafe { (*self.names.as_ptr()).get_or_add_slice(s.bytes()) };
          if key_name == expr.index {
            Some(item.value.into())
          } else {
            acc
          }
        }
        Some(_) => acc,
        None => None,
      }
    })
  }
}
