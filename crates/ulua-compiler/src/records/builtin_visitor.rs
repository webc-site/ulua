use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs, ast_local::AstLocal,
    ast_name::AstName, ast_name_table::AstNameTable, ast_visitor::AstVisitor,
  },
  rtti::ast_node_is,
};
use ulua_common::{
  enums::luau_builtin_function::LuauBuiltinFunction, records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::global::Global,
  functions::{
    get_builtin::get_builtin, get_builtin_function_id::get_builtin_function_id,
    get_global_state::get_global_state,
  },
  records::{builtin::Builtin, compile_options::CompileOptions, node::Node, variable::Variable},
};

/// 禁用位表槽数：内建 id 是 `#[repr(u8)]` 的
/// [`LuauBuiltinFunction`] 判别值（`-1` 表示非内建），故整值域即 `u8` 宽度。
const BUILTIN_DISABLED_SLOTS: usize = u8::MAX as usize + 1;

/// cpp `Builtins.cpp:388` 的 `BuiltinVisitor`：遍历整棵树登记可折叠的内建调用。
///
/// 四个只读/一次可变依赖按 cpp 的引用成员建模为生命周期 `'a` 的引用字段：
/// `AstVisitor` 分发持 `&mut self` 时，字段级拆分借用即可同时取用
/// `result` 的可变借用与其余只读借用，无需裸指针（旧实现以
/// "`&mut self` 整体冲突"为由退化成 `*mut`，实测不成立）。
#[derive(Debug)]
pub(crate) struct BuiltinVisitor<'a> {
  /// cpp `DenseHashMap<AstExprCall*, int>& result`：地址句柄键
  pub(crate) result: &'a mut DenseHashMap<Node<AstExprCall>, i32>,
  pub(crate) builtin_is_disabled: [bool; BUILTIN_DISABLED_SLOTS],
  /// cpp `const DenseHashMap<AstName, Global>& globals`
  pub(crate) globals: &'a DenseHashMap<AstName, Global>,
  /// cpp `const DenseHashMap<AstLocal*, Variable>& variables`
  pub(crate) variables: &'a DenseHashMap<Node<AstLocal>, Variable>,
  /// cpp `const CompileOptions& options`
  pub(crate) options: &'a CompileOptions,
}

impl<'a> BuiltinVisitor<'a> {
  /// 构造：按 `disabled_builtins`（"lib.member" / "global" 列表）预填禁用表
  pub(crate) fn new(
    result: &'a mut DenseHashMap<Node<AstExprCall>, i32>,
    globals: &'a DenseHashMap<AstName, Global>,
    variables: &'a DenseHashMap<Node<AstLocal>, Variable>,
    options: &'a CompileOptions,
    names: &AstNameTable,
  ) -> Self {
    let mut builtin_is_disabled = [false; BUILTIN_DISABLED_SLOTS];

    // 两个分支共享的收尾：member 名有效且仍为 Default 全局时，按 Builtin 反查
    // bfid 并置禁用位（object 侧由调用方保证合法：库分支要求非空，全局分支传空名）
    let mut disable_builtin = |object: AstName, member: AstName| {
      if !member.is_null() && get_global_state(globals, member) == Global::Default {
        let builtin = Builtin {
          object,
          method: member,
        };

        let bfid = get_builtin_function_id(&builtin, options);
        if bfid >= 0 && (bfid as usize) < BUILTIN_DISABLED_SLOTS {
          builtin_is_disabled[bfid as usize] = true;
        }
      }
    };

    for bytes in options.disabled_builtins() {
      if let Some(dot) = bytes.iter().position(|&b| b == b'.') {
        let library = names.get_slice(&bytes[..dot]);
        if !library.is_null() {
          let name = names.get_slice(&bytes[dot + 1..]);
          disable_builtin(library, name);
        }
      } else {
        let name = names.get_slice(bytes);
        disable_builtin(AstName::new(), name);
      }
    }

    Self {
      result,
      builtin_is_disabled,
      globals,
      variables,
      options,
    }
  }
}

impl BuiltinVisitor<'_> {
  /// 对应 cpp `BuiltinVisitor::visit(AstExprCall*)`：登记可折叠内建调用。
  /// 全程无 unsafe：依赖字段本就是引用，`node` 由分发链交付为 `&mut`
  /// （非空由类型证明），其地址仅作 result 表的键（句柄模型）。
  pub(crate) fn visit(&mut self, node: &mut AstExprCall) -> bool {
    let Self {
      result,
      builtin_is_disabled,
      globals,
      variables,
      options,
    } = self;

    let node_ptr = Node::from_mut(node);

    // 对应 cpp `getBuiltin(node->func, ...)`：传调用节点的 FUNCTION
    // （即 `math.max` 这一引用）而非整个调用表达式。旧模型误传 `node`
    // （调用本身），导致 get_builtin 永远解析不出内建、无可折叠项登记。
    let builtin = if node.self_ {
      Builtin::default()
    } else {
      get_builtin(node.func.into(), globals, variables)
    };

    if builtin.empty() {
      return true;
    }

    let mut bfid = get_builtin_function_id(&builtin, options);

    if bfid >= 0 && builtin_is_disabled[bfid as usize] {
      bfid = -1;
    }

    // getBuiltinFunctionId 乐观假设所有 select() 调用都是内建，但实际上
    // 第二参数必须是 vararg。
    // cpp: bfid == LBF_SELECT_VARARG && !(args.size == 2 && args.data[1]->is<AstExprVarargs>())
    if bfid == LuauBuiltinFunction::LBF_SELECT_VARARG as i32 {
      let is_select_arity_2 = node.args.len() == 2;
      // `iter_nodes` 把 len==2 守卫下的 args[1] 只读物化为 &AstExpr，判型走安全门面。
      let second_arg_is_vararg = is_select_arity_2
        && node
          .args
          .iter_nodes()
          .nth(1)
          .is_some_and(ast_node_is::<AstExprVarargs>);

      if !(is_select_arity_2 && second_arg_is_vararg) {
        bfid = -1;
      }
    }

    if bfid >= 0 {
      // cpp `result[node] = bfid` 为覆盖语义。
      *result.get_or_insert(node_ptr) = bfid;
    }

    true
  }
}

// cpp `BuiltinVisitor : AstVisitor` 覆写 `visit(AstExprCall*)`。旧实现里
// 生成的 `visit(&mut self, *mut AstExprCall) -> bool` 方法从未接入 visitor
// trait 分发——`analyzeBuiltins`（需遍历整棵树）因此什么都没登记，
// 优化级 2 的内建常量折叠全部静默失效。此 impl 把每个调用节点都分发过去。
impl<'a> AstVisitor for BuiltinVisitor<'a> {
  fn visit_expr_call(&mut self, node: &mut AstExprCall) -> bool {
    self.visit(node)
  }
}
