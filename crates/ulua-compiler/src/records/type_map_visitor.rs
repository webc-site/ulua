//! Source: `Compiler/src/Types.cpp:253-951`
use alloc::vec::Vec;

use ulua_ast::{
  enums::ast_type_ref::AstTypeRef,
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal,
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_local::AstLocal,
    ast_name::AstName,
    ast_stat::AstStat,
    ast_stat_block::AstStatBlock,
    ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn,
    ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction,
    ast_stat_repeat::AstStatRepeat,
    ast_stat_type_alias::AstStatTypeAlias,
    ast_table_indexer::AstTableIndexer,
    ast_type::AstType,
    ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_table::AstTypeTable,
    ast_visitor::AstVisitor,
  },
  rtti::{ast_node_try_as, ast_node_try_as_mut},
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  enums::{luau_builtin_function::LuauBuiltinFunction, luau_bytecode_type::LuauBytecodeType},
  fflag,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  enums::global::Global,
  functions::{
    ast_slot_ref::ast_slot_ref,
    ast_slot_visit::{ast_slot_visit_expr, ast_slot_visit_stat},
    get_function_type::get_function_type,
    get_type::get_type,
    is_generic::NoGenerics,
    is_matching_global::is_matching_global,
    is_matching_global_member::is_matching_global_member,
  },
  records::{builtin_ast_types::BuiltinAstTypes, node::Node},
  type_aliases::library_member_type_callback::LibraryMemberTypeCallback,
};

/// TypeMapVisitor 构造参数（cpp `buildTypeMap` 形参表）。
///
/// 键位说明：functionTypes/localTypes/exprTypes/builtinCalls 的节点键与
/// resolved*/type_aliases 的值都是 arena 节点**身份句柄**（[`Node`]，对应 cpp
/// `DenseHashMap<AstX*, _>` 的指针即身份语义）。这些 map 与 Compiler/Codegen
/// 各阶段共享同一地址键空间；句柄只承担身份与物化，不表达生命周期。
#[derive(Debug)]
pub(crate) struct TypeMapVisitorArgs<'a, 'b> {
  pub function_types: &'a mut DenseHashMap<Node<AstExprFunction>, Vec<u8>>,
  pub local_types: &'a mut DenseHashMap<Node<AstLocal>, LuauBytecodeType>,
  pub expr_types: &'a mut DenseHashMap<Node<AstExpr>, LuauBytecodeType>,
  /// 宿主注册的向量类型名（已转为借用字节切片）。
  pub host_vector_type: Option<&'a [u8]>,
  pub userdata_types: &'a DenseHashMap<AstName, u8>,
  pub builtin_types: &'a BuiltinAstTypes,
  pub builtin_calls: &'a DenseHashMap<Node<AstExprCall>, i32>,
  pub globals: &'a DenseHashMap<AstName, Global>,
  pub library_member_type_cb: LibraryMemberTypeCallback,
  pub bytecode: &'a mut BytecodeBuilder<'b>,
}

#[derive(Debug)]
pub(crate) struct TypeMapVisitor<'a, 'b> {
  pub(crate) function_types: &'a mut DenseHashMap<Node<AstExprFunction>, Vec<u8>>,
  pub(crate) local_types: &'a mut DenseHashMap<Node<AstLocal>, LuauBytecodeType>,
  pub(crate) expr_types: &'a mut DenseHashMap<Node<AstExpr>, LuauBytecodeType>,
  /// 宿主注册的向量类型名切片。
  pub(crate) host_vector_type: Option<&'a [u8]>,
  pub(crate) userdata_types: &'a DenseHashMap<AstName, u8>,
  pub(crate) builtin_types: &'a BuiltinAstTypes,
  pub(crate) builtin_calls: &'a DenseHashMap<Node<AstExprCall>, i32>,
  pub(crate) globals: &'a DenseHashMap<AstName, Global>,
  pub(crate) library_member_type_cb: LibraryMemberTypeCallback,
  pub(crate) bytecode: &'a mut BytecodeBuilder<'b>,

  /// cpp `DenseHashMap<AstName, AstStatTypeAlias*>`：值可为 null——块级别名出
  /// 作用域时 `pop_type_aliases` 以 null 覆写，表示「曾声明但已被遮蔽/失效」，
  /// 解析侧（`get_type`/`resolve_aliases_deprecated`）判空回落。§2 收口为
  /// `Option<Node<_>>`：cpp 的 null 哨兵在此就是「缺席」，不再有可空裸指针值。
  pub(crate) type_aliases: DenseHashMap<AstName, Option<Node<AstStatTypeAlias>>>,
  /// 别名遮蔽栈：`(名字, 被遮蔽的旧绑定)`，`None` 即 cpp 的 null 哨兵。
  pub(crate) type_alias_stack: Vec<(AstName, Option<Node<AstStatTypeAlias>>)>,
  /// 值同 `type_aliases`：`resolve_aliases_deprecated` 可解析失败落回 null，
  /// 读取侧（`try_get_table_type` 等）按缺席处理。
  pub(crate) resolved_locals: DenseHashMap<Node<AstLocal>, Option<Node<AstType>>>,
  pub(crate) resolved_exprs: DenseHashMap<Node<AstExpr>, Option<Node<AstType>>>,
  pub(crate) function_return_types: DenseHashMap<Node<AstLocal>, Option<Node<AstType>>>,
}

impl TypeMapVisitor<'_, '_> {
  /// bfid 校验后（非法 id 归入 `LBF_NONE`，与 cpp switch default 一致）的内建
  /// 结果类型节点；`None` 表示不记录类型。
  #[inline]
  pub(crate) fn builtin_result_type(&self, builtin: LuauBuiltinFunction) -> Option<Node<AstType>> {
    if builtin.is_integer_fastcall() && !fflag::LuauIntegerFastcalls.get() {
      return None;
    }
    self
      .builtin_types
      .node_for_bytecode_type(builtin.result_bytecode_type()?)
  }

  /// 对应 cpp 旧版 `resolveAliases`（无展开循环的弃用路径）：缺席早退，
  /// 经 `as_type_ref` 安全模式匹配解析别名，解析失败一律回落原 `ty`。
  ///
  /// §2：入出参与别名表值一律为 `Option<Node<AstType>>`——cpp 的 null 结果
  /// 即 `None`，本函数不再有裸解引用与 `unsafe` 块（旧注释里「收口为 unsafe
  /// 签名会沿 AstVisitor hook 无界传染」的顾虑随指针形参一起消失）。
  pub(crate) fn resolve_aliases_deprecated(
    &mut self,
    ty: Option<Node<AstType>>,
  ) -> Option<Node<AstType>> {
    let ty_node = ty?;

    // 外层 `None` = 未做解析（回落原 ty）；`Some(inner)` 中 inner 可为 `None`，
    // 即别名命中但其 `type_ptr` 缺席（cpp `return alias->typePtr` 交出 null）。
    let resolved = match ty_node.borrow().as_type_ref() {
      // 带 prefix 的限定引用（如 `pkg.Type`）不是裸别名，原样返回
      AstTypeRef::Reference(ref_node) if ref_node.prefix.is_some() => None,
      AstTypeRef::Reference(ref_node) => self
        .type_aliases
        .find(&ref_node.name)
        .copied()
        .flatten()
        // alias 由 push_type_aliases 建档，指向 arena 存活 AstStatTypeAlias；
        // 建档缺席视为无法解析，回落原 ty。
        .map(|alias| Node::try_new(alias.borrow().type_ptr)),
      _ => None,
    };

    resolved.unwrap_or(Some(ty_node))
  }

  /// expr 的类型若为表类型则取其类型节点（cpp `TypeSet` 中 resolvedExprs 下转
  /// `AstTypeTable` 的共用入口，props 扫描与 indexer 查找共享）。
  pub(crate) fn try_get_table_type(
    &self,
    expr: impl Into<Node<AstExpr>>,
  ) -> Option<&'static AstTypeTable> {
    self
      .resolved_exprs
      .find(&expr.into())
      .copied()
      // 值槽缺席（cpp 的 null）即无类型，与「无表类型」同结果。
      .flatten()
      .and_then(|type_node| match type_node.borrow().as_type_ref() {
        AstTypeRef::Table(tbl) => Some(tbl),
        _ => None,
      })
  }

  /// expr 的类型若为表类型则取其 indexer（cpp `TypeSet::tryGetTableIndexer`）。
  /// `Option` 收拢判空，调用方不再手写 null 检查 + 裸解引用。
  pub(crate) fn try_get_table_indexer(
    &self,
    expr: impl Into<Node<AstExpr>>,
  ) -> Option<&'static AstTableIndexer> {
    // 表类型节点的 indexer 槽可缺席（cpp 存 null）：`ast_slot_ref` 归一为 None。
    self
      .try_get_table_type(expr)
      .and_then(|table_ty| ast_slot_ref(table_ty.indexer))
  }
}

impl<'a> TypeMapVisitor<'a, '_> {
  pub(crate) fn pop_type_aliases(&mut self, alias_stack_top: usize) {
    // 切出栈顶区间后自内向外逐个恢复（与原 `while len > top { pop().unwrap() }`
    // 的弹出顺序一致，免每轮长度判定与 unwrap）。
    let tail = self.type_alias_stack.split_off(alias_stack_top);
    for (name, ty) in tail.into_iter().rev() {
      // cpp `typeAliases[top.first] = top.second` — operator[] 覆盖写回，
      // 出块时恢复旧绑定（栈层与被遮蔽的缺席绑定都直接写回 `None`）。
      // 早前误用 try_insert 会让块级别名（如 do..end 内 `type Part = number`）
      // 泄漏进外层作用域。
      *self.type_aliases.get_or_insert(name) = ty;
    }
  }

  /// 记录 expr 的解析后类型：`resolved_exprs` 存解析结果（缺席即 cpp 的 null），
  /// `expr_types` 存其 bytecode 编码。`ty` 接受 `Node<AstType>`（内建/句柄化的
  /// 非空来源）与 `Option<Node<AstType>>`（cpp 可空槽经 [`Node::try_new`] 归一）
  /// 两种交付形态。
  pub(crate) fn record_resolved_type_ast_expr_ast_type(
    &mut self,
    expr: impl Into<Node<AstExpr>>,
    ty: impl Into<Option<Node<AstType>>>,
  ) -> LuauBytecodeType {
    let expr = expr.into();
    let ty = self.resolve_aliases_deprecated(ty.into());

    *self.resolved_exprs.get_or_insert(expr) = ty;

    let mut seen_aliases: DenseHashSet<AstName> = DenseHashSet::default();

    // ty 为 resolve_aliases_deprecated 结果：arena 存活节点句柄或缺席
    // （get_type 入口把 None 归一为 ANY）；self 各字段借用互不相交。
    let bty = get_type(
      ty,
      &NoGenerics,
      &self.type_aliases,
      self.host_vector_type,
      self.userdata_types,
      self.bytecode,
      &mut seen_aliases,
    );

    *self.expr_types.get_or_insert(expr) = bty;
    bty
  }

  /// [`Self::record_resolved_type_ast_expr_ast_type`] 的 AstLocal 版本。
  pub(crate) fn record_resolved_type_ast_local_ast_type(
    &mut self,
    local: impl Into<Node<AstLocal>>,
    ty: impl Into<Option<Node<AstType>>>,
  ) -> LuauBytecodeType {
    let local = local.into();
    let ty_resolved = self.resolve_aliases_deprecated(ty.into());

    *self.resolved_locals.get_or_insert(local) = ty_resolved;

    let mut seen_aliases = DenseHashSet::default();
    let bty = get_type(
      ty_resolved,
      &NoGenerics,
      &self.type_aliases,
      self.host_vector_type,
      self.userdata_types,
      self.bytecode,
      &mut seen_aliases,
    );

    if bty != LuauBytecodeType::LBC_TYPE_ANY {
      *self.local_types.get_or_insert(local) = bty;
    }

    bty
  }
}

/// 扫描 block 顶层的 `type X = ...` 语句并压入别名表，返回栈顶水位
/// （cpp `pushTypeAliases`）。block 由调用方交付 `&mut` 存活借用（分发链
/// 或已判空解引用的 arena 槽位），本函数只对子语句槽位做最小裸解引用。
pub(crate) fn push_type_aliases(
  this: &mut TypeMapVisitor<'_, '_>,
  block: &mut AstStatBlock,
) -> usize {
  let alias_stack_top = this.type_alias_stack.len();

  for stat_ptr in block.body.iter_nodes_mut() {
    // body 元素已句柄化（Nodes）：&mut block 即独占证明，get_mut 直接交出
    // 子语句可变借用，下转直接吃 `&mut stat.base`，无裸指针交接
    let stat: &mut AstStat = stat_ptr.get_mut();

    // Safety: 下转目标与 class_index 判定一致，且 stat 在借用期内独占
    if let Some(alias_ref) = ast_node_try_as_mut::<AstStatTypeAlias>(&mut stat.base) {
      // 旧绑定折叠为 Option：缺项与「建档为 null」同为 None（cpp 两者皆 null 哨兵）
      let prev_alias = this.type_aliases.find(&alias_ref.name).copied().flatten();

      this.type_alias_stack.push((alias_ref.name, prev_alias));

      // C++ `prevAlias = alias` 会覆写 typeAliases[name]；此前 try_insert 在同名
      // 别名已存在（嵌套重定义）时是空操作，令外层别名错误地留在作用域内。
      *this.type_aliases.get_or_insert(alias_ref.name) = Some(Node::from_mut(alias_ref));
    }
  }

  alias_stack_top
}

impl<'a, 'b> TypeMapVisitor<'a, 'b> {
  pub fn new(args: TypeMapVisitorArgs<'a, 'b>) -> Self {
    Self {
      function_types: args.function_types,
      local_types: args.local_types,
      expr_types: args.expr_types,
      host_vector_type: args.host_vector_type,
      userdata_types: args.userdata_types,
      builtin_types: args.builtin_types,
      builtin_calls: args.builtin_calls,
      globals: args.globals,
      library_member_type_cb: args.library_member_type_cb,
      bytecode: args.bytecode,
      type_aliases: DenseHashMap::default(),
      type_alias_stack: Vec::new(),
      resolved_locals: DenseHashMap::default(),
      resolved_exprs: DenseHashMap::default(),
      function_return_types: DenseHashMap::default(),
    }
  }
}

/// 具体表达式节点 → 基类 map 键（该基类句柄仅作 `expr_types`/
/// `resolved_exprs` 的地址键使用，不解引用——句柄模型）。
#[inline]
fn base_key<T>(node: &mut T) -> Node<AstExpr> {
  Node::from_mut(node).cast::<AstExpr>()
}

impl<'a, 'b> AstVisitor for TypeMapVisitor<'a, 'b> {
  fn visit_stat_block(&mut self, node: &mut AstStatBlock) -> bool {
    let alias_stack_top = push_type_aliases(self, node);

    // body 句柄化：iter_nodes_mut 沿 &mut node 交出槽位，写穿遍历仍走门面收口。
    for stat in node.body.iter_nodes_mut() {
      ast_slot_visit_stat(stat.as_ptr(), self);
    }

    self.pop_type_aliases(alias_stack_top);

    false
  }

  fn visit_stat_repeat(&mut self, node: &mut AstStatRepeat) -> bool {
    // body/condition 已句柄化为 Node（parser 非空由类型层承载），`get_mut` 直出
    // 独占可变借用（cpp `node->body` 直用），别名压入与子语句遍历共用同一借用。
    let body = node.body.get_mut();
    let alias_stack_top = push_type_aliases(self, body);

    // body 元素已句柄化：可变借用沿 &mut body 传递，写穿遍历仍走门面收口。
    for stat in body.body.iter_nodes_mut() {
      ast_slot_visit_stat(stat.as_ptr(), self);
    }

    ast_slot_visit_expr(node.condition.as_ptr(), self);

    self.pop_type_aliases(alias_stack_top);

    false
  }

  fn visit_stat_for(&mut self, node: &mut AstStatFor) -> bool {
    let ty = self.builtin_types.number_node();
    self.record_resolved_type_ast_local_ast_type(node.var, ty);
    true
  }

  fn visit_stat_for_in(&mut self, node: &mut AstStatForIn) -> bool {
    for &expr_ptr in node.values.as_slice() {
      ast_slot_visit_expr(expr_ptr, self);
    }

    // 与 Compiler 匹配内建迭代的方式相近，但这里还处理广义迭代形态
    if node.vars.len() == 2 && node.values.len() == 1 {
      let vars = node.vars.as_slice();
      let value_ptr = node.values.as_slice()[0];

      // value_ptr 为 null 或 arena 存活节点：门面判空 + 安全引用下转。
      let call = ast_slot_ref(value_ptr).and_then(|v| ast_node_try_as::<AstExprCall>(v));

      if let Some(call) = call.filter(|call| call.args.len() == 1) {
        // arg 由解析器保证为 arena 存活节点（单实参 call 的槽位非空）。
        let arg = call.args.as_slice()[0];

        if is_matching_global(self.globals, ast_slot_ref(call.func), "ipairs") {
          // 纯只读查找，判空下转在门面内收口。
          if let Some(indexer) = self.try_get_table_indexer(arg) {
            let number_ty = self.builtin_types.number_node();
            self.record_resolved_type_ast_local_ast_type(vars[0], Some(number_ty));
            self
              .record_resolved_type_ast_local_ast_type(vars[1], Node::try_new(indexer.result_type));
          }
        } else if is_matching_global(self.globals, ast_slot_ref(call.func), "pairs")
          && let Some(indexer) = self.try_get_table_indexer(arg)
        {
          self.record_resolved_type_ast_local_ast_type(vars[0], Node::try_new(indexer.index_type));
          self.record_resolved_type_ast_local_ast_type(vars[1], Node::try_new(indexer.result_type));
        }
      } else if let Some(indexer) = self.try_get_table_indexer(value_ptr) {
        self.record_resolved_type_ast_local_ast_type(vars[0], Node::try_new(indexer.index_type));
        self.record_resolved_type_ast_local_ast_type(vars[1], Node::try_new(indexer.result_type));
      }
    }

    for &var_ptr in node.vars.iter() {
      // vars 元素由解析器保证指向 arena 存活 AstLocal；`ast_slot_ref` 只读物化，
      // 此处只读 annotation 句柄化视图。
      let Some(var) = ast_slot_ref(var_ptr) else {
        continue;
      };

      // cpp `if (var->annotation)`：注解缺席即未标注，不记录。
      if let Some(annotation) = Node::try_new(var.annotation) {
        self.record_resolved_type_ast_local_ast_type(var_ptr, Some(annotation));
      }
    }

    // cpp `node->body->visit(this)`：body 静态类型即 AstStatBlock，直调免 class-index 分发。
    // body 已句柄化为非空 Node（cpp Ast.cpp:861 无守卫下钻），判空早退门面随类型折叠。
    ast_stat_block_visit(node.body.get_mut(), self);

    false
  }

  fn visit_stat_local_function(&mut self, node: &mut AstStatLocalFunction) -> bool {
    // func 已句柄化为 Node<AstExprFunction>（类型层非空），`ast_slot_ref` 判空门面
    // 随非空类型折叠为直接 `.get()`；return_annotation 仍走可空门面。
    if let Some(type_pack) = node
      .func
      .get()
      .return_annotation
      .get()
      .and_then(|t| ast_node_try_as::<AstTypePackExplicit>(t))
    {
      let types = type_pack.type_list.types.as_slice();
      if let Some(&first_type) = types.first() {
        // 槽内 null 记为缺席值（条目仍插入，读取侧据此区分「有记录无类型」与「无记录」）。
        self
          .function_return_types
          .try_insert(node.name.into(), Node::try_new(first_type));
      }
    }

    true // Let generic visitor step into all expressions
  }

  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    let type_str = get_function_type(
      node,
      &self.type_aliases,
      self.host_vector_type,
      self.userdata_types,
      self.bytecode,
    );
    if !type_str.is_empty() {
      *self.function_types.get_or_insert(node.into()) = type_str;
    }
    true
  }

  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    // local 槽已句柄化恒非空：.get() 给出 arena 存活 AstLocal 共享引用
    //（旧判空早退分支类型端不可达，随类型消失）。
    let local = node.local.get();

    // 键位直接用节点指针槽（与 cpp `local` 同值），仅作 map 键不解引用。
    let local_ptr = node.local;

    if let Some(annotation) = Node::try_new(local.annotation) {
      let ty = self.record_resolved_type_ast_expr_ast_type(base_key(node), Some(annotation));

      if ty != LuauBytecodeType::LBC_TYPE_ANY {
        self.local_types.try_insert(local_ptr.into(), ty);
      }
    } else if let Some(ty) = self.resolved_locals.find(&local_ptr.into()).copied() {
      // 条目命中即沿用其解析结果（值可为缺席），与 cpp「present-with-null」同形。
      let ty = self.record_resolved_type_ast_expr_ast_type(base_key(node), ty);
      self.local_types.try_insert(local_ptr.into(), ty);
    }

    false
  }

  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    // 先访问全部值表达式
    for &expr_ptr in node.values.as_slice() {
      ast_slot_visit_expr(expr_ptr, self);
    }

    // 把类型从值传播到变量
    let vars = node.vars.as_slice();
    let values = node.values.as_slice();

    // zip 取较短一侧，等价于 `i >= values.len()` 即跳过的原循环
    for (&var_ptr, &value_ptr) in vars.iter().zip(values.iter()) {
      // 从被赋值的表达式传播类型；此简化传播不处理尾位 type pack。
      // var/value 由解析器保证 arena 存活或缺席（缺席即跳过），键用节点身份句柄。
      if let Some(var) = ast_slot_ref(var_ptr)
        && Node::try_new(var.annotation).is_none()
        && let Some(ty) =
          Node::try_new(value_ptr).and_then(|value| self.resolved_exprs.find(&value).copied())
      {
        // cpp `resolvedLocals[var] = resolvedExpr[value]`：值可为缺席，条目仍写入。
        self.resolved_locals.try_insert(var_ptr.into(), ty);
      }
    }

    false
  }

  fn visit_expr_index_expr(&mut self, node: &mut AstExprIndexExpr) -> bool {
    // expr/index 已句柄化；ast_slot_visit_expr 为既有裸指针槽位门面，经 as_ptr 桥接。
    let (expr, index) = (node.expr, node.index);

    ast_slot_visit_expr(expr.as_ptr(), self);
    ast_slot_visit_expr(index.as_ptr(), self);

    // 单次查找同时完成判空与取结果类型，替代原先查两遍 try_get_table_indexer
    if let Some(indexer) = self.try_get_table_indexer(expr) {
      // 先取句柄，避免把表节点借用带进随后 &mut self 的记录调用。
      let result_type = Node::try_new(indexer.result_type);
      self.record_resolved_type_ast_expr_ast_type(base_key(node), result_type);
    }

    false
  }

  fn visit_expr_index_name(&mut self, node: &mut AstExprIndexName) -> bool {
    // expr 已句柄化恒非空；ast_slot_visit_expr 为既有裸指针槽位门面，经 as_ptr 桥接。
    ast_slot_visit_expr(node.expr.as_ptr(), self);

    // 表类型判定共用 `try_get_table_type` 入口，消除重复下转样板
    if let Some(table_ty) = self.try_get_table_type(node.expr) {
      for prop in table_ty.props.iter() {
        // AstName 的 Eq 即指针身份，与 cpp `prop.name.value == node->index.value` 等价
        // （cpp 原语保留在注释；Rust 侧 AstName 判等即驻留指针比较）。
        if prop.name == node.index {
          self.record_resolved_type_ast_expr_ast_type(base_key(node), Node::try_new(prop.r#type));
          return false;
        }
      }
    }

    if let Some(&type_bc) = self.expr_types.find(&node.expr.into())
      && type_bc == LuauBytecodeType::LBC_TYPE_VECTOR
      && matches!(
        node.index.as_bytes(),
        b"X" | b"Y" | b"Z" | b"x" | b"y" | b"z"
      )
    {
      self.record_resolved_type_ast_expr_ast_type(base_key(node), self.builtin_types.number_node());
      return false;
    }

    if is_matching_global_member(self.globals, node, "vector", "zero")
      || is_matching_global_member(self.globals, node, "vector", "one")
    {
      self.record_resolved_type_ast_expr_ast_type(base_key(node), self.builtin_types.vector_node());
      return false;
    }

    if let Some(library_member_type_cb) = self.library_member_type_cb
      // expr 已句柄化恒非空；门面判型 + 安全下转，经 as_ptr 桥接。
      && let Some(object) =
        ast_slot_ref(node.expr.as_ptr()).and_then(|e| ast_node_try_as::<AstExprGlobal>(e))
    {
      // Safety: library_member_type_cb 为宿主注册的 extern "C-unwind" 回调（同 cpp 直接调用）
      // AstName 身份桥 as_ptr 透出 `*const u8`，与 library_member_type_cb 纯 Rust 裸指针契约一致。
      let raw_ty = unsafe { library_member_type_cb(object.name.as_ptr(), node.index.as_ptr()) };
      let ty = LuauBytecodeType(raw_ty as u16);

      if ty != LuauBytecodeType::LBC_TYPE_ANY {
        let builtin_node = self.builtin_types.node_for_bytecode_type(ty);
        let node_key = base_key(node);
        if let Some(t) = builtin_node {
          self.resolved_exprs.try_insert(node_key, Some(t));
        }

        self.expr_types.try_insert(node_key, ty);
        return false;
      }
    }

    false
  }

  fn visit_expr_unary(&mut self, node: &mut AstExprUnary) -> bool {
    // expr 已句柄化；ast_slot_visit_expr 为既有裸指针槽位门面，经 as_ptr 桥接。
    let expr = node.expr;

    ast_slot_visit_expr(expr.as_ptr(), self);

    match node.op {
      AstExprUnaryOp::Not => {
        self.record_resolved_type_ast_expr_ast_type(
          base_key(node),
          self.builtin_types.boolean_node(),
        );
      }
      AstExprUnaryOp::Minus => {
        let type_ptr = self.resolved_exprs.find(&expr.into());
        let bc_type_ptr = self.expr_types.find(&expr.into());

        if let (Some(&ty), Some(&bc_ty)) = (type_ptr, bc_type_ptr)
          && (bc_ty == LuauBytecodeType::LBC_TYPE_VECTOR
            || bc_ty == LuauBytecodeType::LBC_TYPE_NUMBER)
        {
          self.record_resolved_type_ast_expr_ast_type(base_key(node), ty);
        }
      }
      AstExprUnaryOp::Len => {
        self
          .record_resolved_type_ast_expr_ast_type(base_key(node), self.builtin_types.number_node());
      }
    }

    false
  }

  fn visit_expr_binary(&mut self, node: &mut AstExprBinary) -> bool {
    // left/right 已句柄化；ast_slot_visit_expr 为既有裸指针槽位门面，经 as_ptr 桥接。
    let left = node.left;
    let right = node.right;

    ast_slot_visit_expr(left.as_ptr(), self);
    ast_slot_visit_expr(right.as_ptr(), self);

    let op = node.op;

    if op.is_comparison() {
      self
        .record_resolved_type_ast_expr_ast_type(base_key(node), self.builtin_types.boolean_node());
      return false;
    }

    if matches!(
      op,
      AstExprBinaryOp::Concat | AstExprBinaryOp::And | AstExprBinaryOp::Or
    ) {
      return false;
    }

    // 单次 tuple let 同时完成四次查找与取值，替代两轮 is_none 早退 + 三处 unwrap
    if let (Some(left_type), Some(left_bc_type), Some(right_type), Some(right_bc_type)) = (
      self.resolved_exprs.find(&left.into()).copied(),
      self.expr_types.find(&left.into()).copied(),
      self.resolved_exprs.find(&right.into()).copied(),
      self.expr_types.find(&right.into()).copied(),
    ) {
      if left_bc_type == LuauBytecodeType::LBC_TYPE_VECTOR {
        self.record_resolved_type_ast_expr_ast_type(base_key(node), left_type);
      } else if right_bc_type == LuauBytecodeType::LBC_TYPE_VECTOR {
        self.record_resolved_type_ast_expr_ast_type(base_key(node), right_type);
      } else if left_bc_type == LuauBytecodeType::LBC_TYPE_NUMBER
        && right_bc_type == LuauBytecodeType::LBC_TYPE_NUMBER
      {
        self.record_resolved_type_ast_expr_ast_type(base_key(node), left_type);
      }
    }

    false
  }

  fn visit_expr_group(&mut self, node: &mut AstExprGroup) -> bool {
    // expr 已句柄化；ast_slot_visit_expr 为既有裸指针槽位门面，经 as_ptr 桥接。
    let expr = node.expr;

    ast_slot_visit_expr(expr.as_ptr(), self);

    if let Some(&ty_ptr) = self.resolved_exprs.find(&expr.into()) {
      self.record_resolved_type_ast_expr_ast_type(base_key(node), ty_ptr);
    }

    false
  }

  fn visit_expr_type_assertion(&mut self, node: &mut AstExprTypeAssertion) -> bool {
    // expr/annotation 已句柄化恒非空；ast_slot_visit_expr 为既有裸指针槽位门面，
    // 经 as_ptr 桥接，annotation 直接以句柄入类型图。
    ast_slot_visit_expr(node.expr.as_ptr(), self);

    self.record_resolved_type_ast_expr_ast_type(base_key(node), Some(Node::from(node.annotation)));

    false
  }

  fn visit_expr_constant_bool(&mut self, node: &mut AstExprConstantBool) -> bool {
    self.record_resolved_type_ast_expr_ast_type(base_key(node), self.builtin_types.boolean_node());

    false
  }

  fn visit_expr_constant_number(&mut self, node: &mut AstExprConstantNumber) -> bool {
    self.record_resolved_type_ast_expr_ast_type(base_key(node), self.builtin_types.number_node());

    false
  }

  fn visit_expr_constant_integer(&mut self, node: &mut AstExprConstantInteger) -> bool {
    self.record_resolved_type_ast_expr_ast_type(base_key(node), self.builtin_types.integer_node());

    false
  }

  fn visit_expr_constant_string(&mut self, node: &mut AstExprConstantString) -> bool {
    self.record_resolved_type_ast_expr_ast_type(base_key(node), self.builtin_types.string_node());

    false
  }

  fn visit_expr_interp_string(&mut self, node: &mut AstExprInterpString) -> bool {
    self.record_resolved_type_ast_expr_ast_type(base_key(node), self.builtin_types.string_node());
    false
  }

  fn visit_expr_if_else(&mut self, node: &mut AstExprIfElse) -> bool {
    // 三子节点已句柄化；槽位访问门面为既有裸指针 API，经 as_ptr 桥接。
    ast_slot_visit_expr(node.condition.as_ptr(), self);
    ast_slot_visit_expr(node.true_expr.as_ptr(), self);
    ast_slot_visit_expr(node.false_expr.as_ptr(), self);

    // 单次 tuple let 同时完成三次查找与取值，替代三个 Option + 多层解引用
    if let (Some(true_type), Some(true_bc_type), Some(false_bc_type)) = (
      self.resolved_exprs.find(&node.true_expr.into()).copied(),
      self.expr_types.find(&node.true_expr.into()).copied(),
      self.expr_types.find(&node.false_expr.into()).copied(),
    ) {
      // 乐观地检查两侧表达式同种即可——AstType* 之间无法比较
      if true_bc_type == false_bc_type {
        self.record_resolved_type_ast_expr_ast_type(base_key(node), true_type);
      }
    }

    false
  }

  fn visit_expr_call(&mut self, node: &mut AstExprCall) -> bool {
    let node_key = base_key(node);
    let builtin_calls_entry = self.builtin_calls.find(&node.into()).copied();

    if let Some(bfid) = builtin_calls_entry {
      // bfid 校验后再转换（`as u8` 截断 + 非法判别值 transmute 是 UB）；
      // 非法 id 归入 LBF_NONE（下方首个 no-op 组），与 C++ switch default 一致。
      // cpp `TypeSet` 分组映射与 integer fastcall 旗标门控拆入
      // [`TypeMapVisitor::builtin_result_type`]，此处只保留「查表 → 记录」主干。
      let builtin = LuauBuiltinFunction::from_id(bfid).unwrap_or(LuauBuiltinFunction::LBF_NONE);
      if let Some(ty) = self.builtin_result_type(builtin) {
        self.record_resolved_type_ast_expr_ast_type(node_key, ty);
      }
    }
    // func 由解析器保证为 null 或 arena 存活节点；门面判空 + 安全下转。
    else if let Some(local_expr) =
      ast_slot_ref(node.func).and_then(|f| ast_node_try_as::<AstExprLocal>(f))
    {
      let local = local_expr.local;
      if let Some(&type_ptr) = self.function_return_types.find(&local.into()) {
        self.record_resolved_type_ast_expr_ast_type(node_key, type_ptr);
      }
    }

    true
  }
}
