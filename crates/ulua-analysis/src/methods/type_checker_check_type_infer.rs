use alloc::{collections::btree_map::Entry, string::String, sync::Arc, vec::Vec};
use core::ptr::null;

use ulua_ast::{
  enums::{ast_table_access::AstTableAccess, mode::Mode},
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_binary::AstExprBinary,
    ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable, ast_local::AstLocal,
    ast_stat::AstStat, ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak, ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_continue::AstStatContinue,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile,
    ast_type_or_pack::AstTypeOrPack, location::Location, position::Position,
  },
  rtti::{AstNodeClass, ast_node_is, ast_node_try_as, ast_node_try_as_ptr},
};
use ulua_common::fflag;

use crate::{
  enums::{control_flow::ControlFlow, table_state::TableState, value_context::ValueContext},
  functions::{
    arc_as_mut::arc_as_mut, as_mutable_type_pack::as_mutable_type_pack,
    ast_node_downcast::ast_node_downcast as stat_downcast, begin_type_pack::begin,
    end_type_pack::end, first::first, flatten_type_pack::flatten_type_pack_id, follow_type,
    follow_type_pack, get_mutable_table_type::get_mutable_table_type, get_mutable_type,
    get_mutable_type_pack, get_table_type::get_table_type, get_type, get_type_pack,
    is_generic::is_generic, is_metamethod_type_infer::is_metamethod, is_prim::is_nil,
    is_table_intersection::is_table_intersection, match_require::match_require,
    match_set_metatable::match_set_metatable, matches::matches, maybe_generic::maybe_generic,
    unwrap_group::unwrap_group,
  },
  methods::type_checker_check_function_signature::scope_mut,
  records::{
    any_type::AnyType,
    arena_handle::Handle,
    binding::Binding,
    cannot_call_non_function::CannotCallNonFunction,
    cannot_extend_table::{CannotExtendTable, Context},
    count_mismatch::CountMismatchContext,
    demoter::Demoter,
    extern_type::ExternType,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    function_argument::FunctionArgument,
    function_definition::FunctionDefinition,
    function_type::FunctionType,
    generic_error::GenericError,
    generic_type::GenericType,
    generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    never_type::NeverType,
    only_tables_can_have_methods::OnlyTablesCanHaveMethods,
    property_type::Property,
    source_module::SourceModule,
    symbol::Symbol,
    table_indexer::TableIndexer,
    table_type::TableType,
    txn_log::TxnLog,
    type_checker::TypeChecker,
    type_fun::TypeFun,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
    union_type::UnionType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    error_type::ErrorType, error_type_pack::ErrorTypePack, module_ptr_module::ModulePtr,
    name_type::Name, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};

impl TypeChecker {
  pub fn check_source_module(
    &mut self,
    module: &SourceModule,
    mode: Mode,
    environment_scope: Option<ScopePtr>,
  ) -> ModulePtr {
    self.check_without_recursion_check(module, mode, environment_scope)
  }

  pub fn check_stat(&mut self, scope: &ScopePtr, program: &AstStat) -> ControlFlow {
    // AstStat 以 base 字段内嵌 AstNode（repr(C) 单继承），分发只读类索引。
    let node = &program.base;

    // match 臂的类索引与 `ast_node_try_as` 判定完全同一（cpp `as<T>()` 命中后
    // 亦直接 static_cast），下转必然成功，expect 为逻辑不可达分支；类索引互斥
    // 由 rtti 测试保证。原 if-let 链顺序不携带语义（各臂类互斥），保持行为。
    match node.class_index {
      AstStatBlock::CLASS_INDEX => {
        self.check_stat_block(scope, stat_downcast::<AstStatBlock>(node))
      }
      AstStatIf::CLASS_INDEX => self.check_stat_if(scope, stat_downcast::<AstStatIf>(node)),
      AstStatWhile::CLASS_INDEX => {
        self.check_stat_while(scope, stat_downcast::<AstStatWhile>(node))
      }
      AstStatRepeat::CLASS_INDEX => {
        self.check_stat_repeat(scope, stat_downcast::<AstStatRepeat>(node))
      }
      AstStatBreak::CLASS_INDEX => ControlFlow::Breaks,
      AstStatContinue::CLASS_INDEX => ControlFlow::Continues,
      AstStatReturn::CLASS_INDEX => {
        self.check_stat_return(scope, stat_downcast::<AstStatReturn>(node))
      }
      AstStatExpr::CLASS_INDEX => {
        let expr = stat_downcast::<AstStatExpr>(node);
        // expr 字段已句柄化（node_handle::Node）：`.get()` 即 arena 只读视图，
        // 原 unsafe 解引用与 parser 非空契约注释随类型一并消失。
        self.check_expr_pack(scope, expr.expr.get());
        ControlFlow::None
      }
      AstStatLocal::CLASS_INDEX => {
        self.check_stat_local(scope, stat_downcast::<AstStatLocal>(node))
      }
      AstStatFor::CLASS_INDEX => self.check_stat_for(scope, stat_downcast::<AstStatFor>(node)),
      AstStatForIn::CLASS_INDEX => {
        self.check_stat_for_in(scope, stat_downcast::<AstStatForIn>(node))
      }
      AstStatAssign::CLASS_INDEX => {
        self.check_stat_assign(scope, stat_downcast::<AstStatAssign>(node))
      }
      AstStatCompoundAssign::CLASS_INDEX => {
        self.check_stat_compound_assign(scope, stat_downcast::<AstStatCompoundAssign>(node))
      }
      AstStatFunction::CLASS_INDEX | AstStatLocalFunction::CLASS_INDEX => {
        self.ice_string_location(
          "Should not be calling two-argument check() on a function statement",
          &program.base.location,
        );
        ControlFlow::None
      }
      AstStatTypeAlias::CLASS_INDEX => {
        self.check_stat_type_alias(scope, stat_downcast::<AstStatTypeAlias>(node))
      }
      AstStatTypeFunction::CLASS_INDEX => {
        self.check_stat_type_function(scope, stat_downcast::<AstStatTypeFunction>(node))
      }
      AstStatDeclareGlobal::CLASS_INDEX => {
        self.check_stat_declare_global(scope, stat_downcast::<AstStatDeclareGlobal>(node))
      }
      AstStatDeclareFunction::CLASS_INDEX => {
        self.check_stat_declare_function(scope, stat_downcast::<AstStatDeclareFunction>(node))
      }
      AstStatDeclareExternType::CLASS_INDEX => {
        self.check_stat_declare_extern_type(scope, stat_downcast::<AstStatDeclareExternType>(node))
      }
      AstStatError::CLASS_INDEX => {
        self.check_stat_error(scope, stat_downcast::<AstStatError>(node))
      }
      AstStatClass::CLASS_INDEX if fflag::DebugLuauUserDefinedClasses.get() => {
        let class_statement = stat_downcast::<AstStatClass>(node);
        // SAFETY: AstStatClass.name 由解析器保证非空（类声明必有名字节点）。
        self.report_error_location_type_error_data(
          unsafe { &(*class_statement.name).location },
          TypeErrorData::GenericError(GenericError::new(String::from(
            "class keyword is illegal here",
          ))),
        );
        ControlFlow::None
      }
      // 原 if-let 链尾部的兜底：未知类型与 class-flag 关闭时均返回 None。
      _ => ControlFlow::None,
    }
  }

  pub fn check_stat_block(&mut self, scope: &ScopePtr, block: &AstStatBlock) -> ControlFlow {
    let child = self.child_scope(scope, &block.base.base.location);
    let flow = self.check_block(&child, block);

    // SAFETY: `scope` 是本帧有效的 &ScopePtr（Arc 在调用期内存活），arc_as_mut 按
    // 其契约派生受控 Scope 裸指针；类型检查单线程，child_scope 创建的 `child` 是
    // 另一个 Arc，inherit_refinements 只在块内写 scope 指向的 Scope，期间无其它
    // 存活的 &mut/引用指向它（对应 C++ shared_ptr<Scope> 的直接可变访问）。
    unsafe {
      let scope_mut = arc_as_mut(scope);
      (*scope_mut).inherit_refinements(&child);
    }

    flow
  }

  pub fn check_stat_if(&mut self, scope: &ScopePtr, statement: &AstStatIf) -> ControlFlow {
    let result = self.check_expr(scope, &statement.condition, None, false);

    // SAFETY: thenbody 同为 parser 必建节点（then 块必然存在）；此处仅读其
    // 双重 base 内嵌的 AstNode.location，repr(C) 偏移 0 保证布局正确。
    let then_scope = self.child_scope(scope, &statement.thenbody.base.base.location);
    self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &then_scope, true);

    let else_location = if let Some(else_stat) = statement.elsebody.get() {
      else_stat.base.location
    } else {
      statement.base.base.location
    };
    // `statement.elsebody->location` -> AstStat.base(AstNode).location;
    // `statement.location` -> AstStatIf.base(AstStat).base(AstNode).location.
    let else_scope = self.child_scope(scope, &else_location);
    self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &else_scope, false);

    let thencf = self.check_stat(&then_scope, &statement.thenbody.base);
    let mut elsecf = ControlFlow::None;
    if let Some(else_stat) = statement.elsebody.get() {
      elsecf = self.check_stat(&else_scope, else_stat);
    }

    if thencf != ControlFlow::None && elsecf == ControlFlow::None {
      // SAFETY: else_scope 是 child_scope 新建的 Arc，本行只读它；arc_as_mut(scope)
      // 派生的写指针指向 &Scope 参数管理的 Scope，单线程独占期内无别名（同块级
      // inherit_refinements 的契约）。
      unsafe {
        let scope_mut = arc_as_mut(scope);
        (*scope_mut).inherit_refinements(&else_scope);
      }
    } else if thencf == ControlFlow::None && elsecf != ControlFlow::None {
      // SAFETY: 与上一分支对称，写入对象同样是 arc_as_mut 契约下的独占 Scope。
      unsafe {
        let scope_mut = arc_as_mut(scope);
        (*scope_mut).inherit_refinements(&then_scope);
      }
    }

    if thencf == elsecf {
      thencf
    } else if matches(thencf, ControlFlow::Returns | ControlFlow::Throws)
      && matches(elsecf, ControlFlow::Returns | ControlFlow::Throws)
    {
      ControlFlow::Returns
    } else {
      ControlFlow::None
    }
  }

  pub fn check_stat_while(&mut self, scope: &ScopePtr, statement: &AstStatWhile) -> ControlFlow {
    let result = self.check_expr(scope, &statement.condition, None, false);
    let while_scope = self.child_scope(scope, &statement.body.base.base.location);
    self.resolve_predicate_vec_scope_ptr_bool(&result.predicates, &while_scope, true);
    self.check_stat_block(&while_scope, &statement.body);
    ControlFlow::None
  }

  pub fn check_stat_repeat(
    &mut self,
    _scope: &ScopePtr,
    _statement: &AstStatRepeat,
  ) -> ControlFlow {
    let rep_scope = self.child_scope(_scope, &{ _statement.base.base.location });
    // body 与 until 条件已句柄化为 Node（repeat 语法强制 `until` 表达式、parser
    // 必建，非空由类型层承载），`.get()` 直出安全引用，死 unsafe 消解。
    self.check_stat_block(&rep_scope, _statement.body.get());
    self.check_expr(&rep_scope, _statement.condition.get(), None, false);
    ControlFlow::None
  }

  pub fn check_stat_return(&mut self, scope: &ScopePtr, return_: &AstStatReturn) -> ControlFlow {
    let mut expected_types: Vec<Option<TypeId>> = Vec::with_capacity(return_.list.size);

    let mut expected_ret_curr = begin(scope.return_type);
    let expected_ret_end = end(scope.return_type);

    // 每个 return 表达式配一个期望类型：直接遍历返回表达式槽位（不触碰元素），
    // 迭代次数与原 0..list.size 一致。
    for _ in return_.list.iter() {
      if expected_ret_curr != expected_ret_end {
        expected_types.push(Some(*expected_ret_curr.current()));
        expected_ret_curr.advance();
      } else if let Some(expected_args_tail) = expected_ret_curr.tail()
        && let Some(vtp) =
          get_type_pack::get::<VariadicTypePack>(follow_type_pack::follow(expected_args_tail))
      {
        expected_types.push(Some(vtp.ty));
      }
    }

    // SAFETY: current_module 的 Arc 在 self 内存活整个调用期，arc_as_mut 派生其
    // 内部字段地址；internal_types 是 Module 直接成员（随 Module 分配固定），
    // Handle::from_mut 物化的 arena 可变借用止于构造语句，交给 Demoter 仅在本
    // 函数内顺序使用，期间无其它 &mut 借用该字段（C++ shared_ptr<Module> 可变
    // 访问同契约）。
    let arena = Handle::from_mut(unsafe {
      &mut (*(arc_as_mut(self.expect_current_module()))).internal_types
    });
    let mut demoter = Demoter::new(arena, self.builtin_types);
    demoter.demote(&mut expected_types);

    let ret_pack = self
      .check_expr_list(
        scope,
        &return_.base.base.location,
        &return_.list,
        false,
        &Vec::new(),
        &expected_types,
      )
      .r#type;

    // HACK: Nonstrict mode gets a bit too smart and strict for us when we
    // start typechecking everything across module boundaries.
    let module_return_type = {
      (*self.expect_current_module())
        .get_module_scope()
        .return_type
    };
    if self.is_nonstrict_mode()
      && follow_type_pack::follow(scope.return_type) == follow_type_pack::follow(module_return_type)
    {
      let errors = self.try_unify_type_pack_id_type_pack_id_scope_ptr_location(
        ret_pack,
        scope.return_type,
        scope.clone(),
        &return_.base.base.location,
      );

      if !errors.is_empty() {
        let any_pack = self.add_type_pack_initializer_list_type_id(&[self.any_type]);
        // SAFETY: get_module_scope() 返回的 Arc<Scope> 局部存活；对其受控 Scope 的
        // return_type 覆写与 C++ `moduleScope->returnType = anyPack` 相同，单线程
        // 无别名，arc_as_mut 契约成立。
        unsafe {
          let module_scope = (*self.expect_current_module()).get_module_scope();
          let module_scope_mut = arc_as_mut(&module_scope);
          (*module_scope_mut).return_type = any_pack;
        }
      }

      return ControlFlow::Returns;
    }

    self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
      ret_pack,
      scope.return_type,
      scope,
      &return_.base.base.location,
      CountMismatchContext::Return,
    );

    ControlFlow::Returns
  }

  pub fn check_stat_assign(&mut self, scope: &ScopePtr, assign: &AstStatAssign) -> ControlFlow {
    let mut expected_types: Vec<Option<TypeId>> = Vec::with_capacity(assign.vars.size);

    let module_scope = self.expect_current_module().get_module_scope();

    // vars/values 数组元素的解引用收口在 AstArray::iter_nodes（只读遍历，cpp 同序）。
    for dest in assign.vars.iter_nodes() {
      let dest_node = &dest.base;

      if let Some(a_local) = ast_node_try_as::<AstExprLocal>(dest_node) {
        // AstExprLocal l-values will have to be checked again because their type
        // might have been mutated during checkExprList later
        expected_types.push(scope.lookup_symbol(Symbol::from_local(a_local.local.as_ptr())));
      } else if let Some(a_global) = ast_node_try_as::<AstExprGlobal>(dest_node) {
        // AstExprGlobal l-values lookup is inlined here to avoid creating a global
        // binding before checkExprList
        match module_scope
          .bindings
          .get(&Symbol::from_global(a_global.name))
        {
          Some(binding) => expected_types.push(Some(binding.type_id)),
          None => expected_types.push(None),
        }
      } else {
        expected_types.push(Some(self.check_l_value(scope, dest, ValueContext::LValue)));
      }
    }

    let value_pack = self
      .check_expr_list(
        scope,
        &assign.base.base.location,
        &assign.values,
        false,
        &Vec::new(),
        &expected_types,
      )
      .r#type;

    let mut value_iter = begin(value_pack);
    let value_end = end(value_pack);

    let mut growing_pack: Option<&'static mut TypePack> = None;

    // values 槽位解引用收口 iter_nodes（collect 保持原顺序，供按下标取 location）。
    let value_nodes: Vec<&AstExpr> = assign.values.iter_nodes().collect();

    for (i, dest_ref) in assign.vars.iter_nodes().enumerate() {
      let is_local = ast_node_is::<AstExprLocal>(dest_ref);
      let is_global = ast_node_is::<AstExprGlobal>(dest_ref);

      let left: TypeId = if is_local || is_global {
        self.check_l_value(scope, dest_ref, ValueContext::LValue)
      } else {
        expected_types[i].expect("非 local/global 赋值目标在上方预检循环按同一判据 push Some(check_l_value)，下标对齐必为 Some")
      };

      let mut right: TypeId = null();

      let loc = value_nodes
        .get(i)
        .or_else(|| value_nodes.last())
        .map(|val| val.base.location)
        .unwrap_or(assign.base.base.location);

      if value_iter != value_end {
        right = follow_type::follow(*value_iter.current());
        value_iter.advance();
      } else if let Some(growing) = growing_pack.as_deref_mut() {
        growing.head.push(left);
        continue;
      } else if let Some(tail) = value_iter.tail() {
        // tail 是从 value pack 链上取出的 TypePackId 句柄（arena/log 存活节点）；
        // follow 是安全函数，仅沿 Bound/空头链前进，不新增别名。
        let tail_pack = follow_type_pack::follow(tail);
        if get_type_pack::get::<ErrorTypePack>(tail_pack).is_some() {
          right = self.error_recovery_type_scope_ptr(scope);
        } else if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tail_pack) {
          right = vtp.ty;
        } else if get_type_pack::get::<FreeTypePack>(tail_pack).is_some() {
          // SAFETY: tail_pack 经 follow 确认是 arena 内存活的 FreeTypePack 节点；
          // as_mutable_type_pack 仅去 const（C++ const_cast 同义），此处把该节点的
          // 变体就地改写为 TypePack{head:[left],tail:None}——对应 C++ unifier 对
          // free var 的 `tail->ty = TypePack{...}` 赋型；写后立刻由
          // get_mutable_type_pack_id 重新取视图，块内无并存 &mut。
          unsafe {
            (*as_mutable_type_pack(tail_pack)).ty =
              TypePackVariant::TypePack(TypePack::single(left));
          }
          growing_pack = get_mutable_type_pack::get_mutable::<TypePack>(tail_pack);
        }
      }

      if !right.is_null() {
        if !fflag::LuauInstantiateInSubtyping.get() && !maybe_generic(left) && is_generic(right) {
          right = self.instantiate(scope, right, loc, TxnLog::empty());
        }

        // Setting a table entry to nil doesn't mean nil is the type of the indexer,
        // it is just deleting the entry
        let mut dest_table_type_receiving_nil: Option<&TableType> = None;
        // dest_ref 已是 iter_nodes 给出的 &AstExpr 视图，&base 直接做 RTTI 匹配。
        let index_expr = ast_node_try_as::<AstExprIndexExpr>(&dest_ref.base);
        if is_nil(right)
          && let Some(index_expr) = index_expr
        {
          let expr_ty = self
            .check_expr(
              scope,
              // index_expr.expr 已句柄化恒非空：.get() 安全借用。
              index_expr.expr.get(),
              None,
              false,
            )
            .r#type;
          dest_table_type_receiving_nil = get_table_type(expr_ty);
        }

        if dest_table_type_receiving_nil.is_none_or(|t| t.indexer.is_none()) {
          // In nonstrict mode, any assignments where the lhs is free and rhs isn't
          // a function, we give it any type.
          if self.is_nonstrict_mode()
            && get_type::get::<FreeType>(follow_type::follow(left)).is_some()
            && get_type::get::<FunctionType>(follow_type::follow(right)).is_none()
          {
            self.unify_type_id_type_id_scope_ptr_location(self.any_type, left, scope, &loc);
          } else {
            self.unify_type_id_type_id_scope_ptr_location(right, left, scope, &loc);
          }
        }
      }
    }

    ControlFlow::None
  }

  pub fn check_stat_compound_assign(
    &mut self,
    scope: &ScopePtr,
    assign: &AstStatCompoundAssign,
  ) -> ControlFlow {
    // var/value 已句柄化：与 AstExprBinary.left/right 同型，原
    // `NonNull::new(..).expect(..)` 非空重建与其契约注释一并消失。
    let expr = AstExprBinary::new(
      assign.base.base.location,
      assign.op,
      assign.var,
      assign.value,
    );

    // 合成节点 left/right 已句柄化（源出 compound assign 的 var/value，parser 必建）：
    // get() 即安全只读借用。
    let left = self.check_expr(scope, expr.left.get(), None, false).r#type;

    let right = self.check_expr(scope, expr.right.get(), None, false).r#type;

    let result = self.check_binary_operation(scope, &expr, left, right, &Default::default());

    self.unify_type_id_type_id_scope_ptr_location(result, left, scope, &assign.base.base.location);

    ControlFlow::None
  }

  pub fn check_stat_local(&mut self, scope: &ScopePtr, local: &AstStatLocal) -> ControlFlow {
    let mut variable_types: Vec<TypeId> = Vec::new();
    let mut expected_types: Vec<Option<TypeId>> = Vec::new();
    let mut bindings: Vec<(*mut AstLocal, Binding)> = Vec::new();

    variable_types.reserve(local.vars.size);
    expected_types.reserve(local.vars.size);
    bindings.reserve(local.vars.size);

    let vars = local.vars.as_slice();
    let values = local.values.as_slice();
    // values 槽位解引用收口 iter_nodes（collect 保序），供按下标做 RTTI 分类。
    let value_nodes: Vec<&AstExpr> = local.values.iter_nodes().collect();

    for (i, var) in local.vars.iter_nodes().enumerate() {
      // annotation 是可空裸指针字段，从 &AstLocal 只读拷贝出指针值即可，判空在下一分支。
      let annotation = var.annotation;
      let rhs_is_table = value_nodes
        .get(i)
        .is_some_and(|val| ast_node_try_as::<AstExprTable>(&val.base).is_some());

      // 原「空指针 = 尚未定出类型」局部占位是可空语义（A 型），改由 Option 表达；
      // 标注解析失败（ErrorType）与无标注两支折叠为 unwrap_or_else 的缺省。
      let annotated: Option<TypeId> = if annotation.is_null() {
        None
      } else {
        // SAFETY: annotation 已通过判空分支，指 arena 的 AstType 节点。
        let resolved = self.resolve_type(scope.clone(), unsafe { &*annotation });
        (get_type::get::<ErrorType>(follow_type::follow(resolved)).is_none()).then_some(resolved)
      };

      let ty = annotated.unwrap_or_else(|| {
        if rhs_is_table || !self.is_nonstrict_mode() {
          self.fresh_type_scope_ptr(scope.clone())
        } else {
          self.any_type
        }
      });

      variable_types.push(ty);
      expected_types.push(Some(ty));
      bindings.push((
        // 由 iter_nodes 借用身份回推同一 arena 裸指针（同址），绑定键恒等性不变。
        (var as *const AstLocal).cast_mut(),
        Binding {
          type_id: ty,
          location: var.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      ));
    }

    if !values.is_empty() {
      let variable_tail = self.fresh_type_pack_scope_ptr(scope);
      let variable_pack = self
        .add_type_pack_vector_type_id_optional_type_pack_id(&variable_types, Some(variable_tail));
      let value_pack = self
        .check_expr_list(
          scope,
          &local.base.base.location,
          &local.values,
          true,
          &Vec::new(),
          &expected_types,
        )
        .r#type;

      let mut ctx = CountMismatchContext::ExprListResult;
      if let [first_val] = values {
        // SAFETY: first_val 是 values 唯一元素；unwrap_group 沿 AstExprGroup.expr
        // （parser 必建）剥括号，返回值要么保持原非空指针、要么取组内表达式，
        // 二者皆 arena 存活节点。
        let expr = unwrap_group(*first_val);
        // SAFETY: expr 为上方 unwrap_group 返回的 arena 存活节点（原非空指针或
        // 组内表达式），&(*expr).base 仅做 RTTI 分类只读。
        if ast_node_try_as::<AstExprCall>(unsafe { &(*expr).base }).is_some() {
          ctx = CountMismatchContext::FunctionResult;
        }
      }

      self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
        value_pack,
        variable_pack,
        scope,
        &local.base.base.location,
        ctx,
      );

      if let (&[var], &[rhs]) = (vars, values)
        && let Some(ty) = first(value_pack, true)
      {
        // SAFETY: var 为 vars 数组唯一元素（AstLocal arena 节点，parser 必建），
        // 一次转共享引用供下方 name 只读访问（与原逐点解引用同一地址、同寿）。
        let var_ref = unsafe { &*var };
        // rhs 判型走指针版 RTTI 门面：null/不匹配折叠为 None，语义同前。
        if unsafe { ast_node_try_as_ptr::<AstExprTable>(rhs) }.is_some() {
          if let Some(ttv) = get_mutable_type::get_mutable::<TableType>(follow_type::follow(ty))
            && ttv.name.is_none()
            && let Some(current_module) = self.current_module.as_ref()
          {
            let module_scope = current_module.get_module_scope();
            if Arc::ptr_eq(scope, &module_scope) {
              // AstName 判空与 as_str_or_empty 均为安全方法，借用期覆盖整个分支。
              if !var_ref.name.is_null() {
                ttv.synthetic_name = Some(var_ref.name.as_str_or_empty().to_string());
              }
            }
          }
        } else {
          let call = unsafe { ast_node_try_as_ptr::<AstExprCall>(rhs) };
          if call.is_some_and(match_set_metatable)
            && let Some(mtv) =
              get_mutable_type::get_mutable::<MetatableType>(follow_type::follow(ty))
            && !var_ref.name.is_null()
          {
            mtv.synthetic_name = Some(var_ref.name.as_str_or_empty().to_string());
          }
        }
      }
    }

    // SAFETY: scope 为 &ScopePtr（Arc 在帧内存活），arc_as_mut 按共享指针取可变访问
    // 是 C++ `ScopePtr` 直传同义；类型检查单线程，本 &mut 存续期内 self 的调用只会
    // 经各自 Arc 句柄读写其它 Scope，不会二次可变借用同一 Scope。
    let scope_mut = unsafe { &mut *(arc_as_mut(scope)) };
    // var/value 两个数组的槽位解引用收口 iter_nodes（同长 zip、cpp 同序）。
    for (var, value) in local.vars.iter_nodes().zip(local.values.iter_nodes()) {
      let Some(call) = ast_node_try_as::<AstExprCall>(&value.base) else {
        continue;
      };

      let Some(require) = match_require(call) else {
        continue;
      };

      let Some(current_module) = self.current_module.as_ref() else {
        continue;
      };

      // SAFETY: require 是 match_require 校验过的 call.args[0]（AstExprRequire 的
      // arena 节点），非空。
      let require_expr = unsafe { &*require };
      let module_info = self
        .resolver_ref()
        .resolve_module_info(&current_module.name, require_expr);

      let Some(module_info) = module_info else {
        continue;
      };

      // AstName 判空与 as_str_or_empty 在 &AstLocal 视图上是安全方法。
      if var.name.is_null() {
        continue;
      }

      let name: Name = var.name.as_str_or_empty().to_string();

      let required_module = self.resolver_ref().get_module(&module_info.name);
      if let Some(module) = required_module {
        scope_mut
          .imported_type_bindings
          .insert(name.clone(), module.exported_type_bindings.clone());
        scope_mut
          .imported_modules
          .insert(name.clone(), module_info.name.clone());

        for require_cycle in &self.require_cycles {
          if !require_cycle.path.is_empty()
            && require_cycle.path[0] == module_info.name
            && let Some(imported_bindings) = scope_mut.imported_type_bindings.get_mut(&name)
          {
            for type_fun in imported_bindings.values_mut() {
              *type_fun = TypeFun::type_fun_type_id(self.any_type);
            }
          }
        }
      }
    }

    for (local, binding) in bindings {
      scope_mut
        .bindings
        .insert(Symbol::from_local(local), binding);
    }

    ControlFlow::None
  }

  pub fn check_stat_for(&mut self, scope: &ScopePtr, expr: &AstStatFor) -> ControlFlow {
    // ScopePtr loopScope = childScope(scope, expr.location);
    let loop_scope = self.child_scope(scope, &expr.base.base.location);

    // TypeId loopVarType = number_type;
    let loop_var_type: TypeId = self.number_type;

    // if (expr.var->annotation)
    //     unify(loopVarType, resolveType(scope, *expr.var->annotation), scope, expr.location);
    // var 已句柄化为非空 Node（数值 for 必有循环变量），`.get()` 即安全引用，
    // 仅读出其 annotation 字段（可空裸指针，下一行判空）。
    let annotation = expr.var.get().annotation;
    if !annotation.is_null() {
      // SAFETY: annotation 已过判空，指 arena 的 AstType 注解节点。
      let resolved = self.resolve_type(scope.clone(), unsafe { &*annotation });
      self.unify_type_id_type_id_scope_ptr_location(
        loop_var_type,
        resolved,
        scope,
        &expr.base.base.location,
      );
    }

    // loopScope->bindings[expr.var] = {loopVarType, expr.var->location};
    // SAFETY: loop_scope 为本帧 child_scope 新建 Arc；arc_as_mut 后单点写入其
    // bindings（键只作身份指针存储，不解引用）。var 已句柄化为非空 Node，
    // as_ptr/get 桥交身份键与 location 读取；所读 AstLocal 随 module/AST 存活，
    // 满足 arc_as_mut 独占写契约。
    unsafe {
      let loop_scope_mut = arc_as_mut(&loop_scope);
      (*loop_scope_mut).bindings.insert(
        Symbol::from_local(expr.var.as_ptr()),
        Binding {
          type_id: loop_var_type,
          location: expr.var.get().location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    // cpp `if (!expr->from) ice(...)` / `if (!expr->to) ice(...)` 的防御分支：
    // from/to 已句柄化为非空 Node（parser 必建上下界），null 态在类型层不存在，
    // ICE 守卫随句柄化折叠为死代码删除。
    // unify(checkExpr(loopScope, *expr.from).type, loopVarType, scope, expr.from->location);
    let from_ty = self
      .check_expr(&loop_scope, expr.from.get(), None, false)
      .r#type;
    self.unify_type_id_type_id_scope_ptr_location(
      from_ty,
      loop_var_type,
      scope,
      &expr.from.get().base.location,
    );

    // unify(checkExpr(loopScope, *expr.to).type, loopVarType, scope, expr.to->location);
    let to_ty = self
      .check_expr(&loop_scope, expr.to.get(), None, false)
      .r#type;
    self.unify_type_id_type_id_scope_ptr_location(
      to_ty,
      loop_var_type,
      scope,
      &expr.to.get().base.location,
    );

    // if (expr.step)
    //     unify(checkExpr(loopScope, *expr.step).type, loopVarType, scope, expr.step->location);
    // step 落可空 OptNode：cpp 判空守卫由 `get()` 的 Option 承接。
    if let Some(step) = expr.step.get() {
      let step_ty = self.check_expr(&loop_scope, step, None, false).r#type;
      self.unify_type_id_type_id_scope_ptr_location(
        step_ty,
        loop_var_type,
        scope,
        &step.base.location,
      );
    }

    // check(loopScope, *expr.body);
    // body 已句柄化为非空 Node（parser 必建循环体），`.get()` 直出安全引用。
    self.check_stat_block(&loop_scope, expr.body.get());

    // return ControlFlow::None;
    ControlFlow::None
  }

  pub fn check_stat_for_in(&mut self, scope: &ScopePtr, forin: &AstStatForIn) -> ControlFlow {
    // ScopePtr loopScope = childScope(scope, forin.location);
    let loop_scope = self.child_scope(scope, &forin.base.base.location);

    // std::vector<TypeId> varTypes; varTypes.reserve(forin.vars.size);
    let mut var_types: Vec<TypeId> = Vec::with_capacity(forin.vars.size);

    // vars 槽位解引用收口 iter_nodes（cpp `for (AstLocal* var : forin.vars)` 同序）。
    for var in forin.vars.iter_nodes() {
      // AstType* ann = vars[i]->annotation;
      // TypeId ty = ann ? resolveType(scope, *ann) : anyIfNonstrict(freshType(loopScope));
      // annotation 是可空裸指针字段，从引用拷贝指针值即安全，判空在下一分支。
      let ann = var.annotation;
      let ty = if !ann.is_null() {
        // SAFETY: ann 非 null，指 arena 的 AstType 注解节点。
        self.resolve_type(scope.clone(), unsafe { &*ann })
      } else {
        let fresh = self.fresh_type_scope_ptr(loop_scope.clone());
        self.any_if_nonstrict(fresh)
      };

      // loopScope->bindings[vars[i]] = {ty, vars[i]->location};
      // SAFETY: `&loop_scope` 是本帧局部 Arc，转 *const ScopePtr 传入 unsafe fn
      // scope_mut 满足其入参有效契约；scope_mut 内部 arc_as_mut 的写入前提
      // （类型检查单线程、调用期无其它路径写该 Scope）同样成立。var 由引用
      // 身份回推同一 arena AstLocal 指针（同址），location 按值拷贝。
      unsafe {
        (*scope_mut(&loop_scope)).bindings.insert(
          Symbol::from_local((var as *const AstLocal).cast_mut()),
          Binding {
            type_id: ty,
            location: var.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }

      // varTypes.push_back(ty);
      var_types.push(ty);
    }

    let values_slice = forin.values.as_slice();
    // A 型：原「空 = 未解析到迭代器表达式」哨兵改 Option；空属 ice 违例路径，
    // 原实现在 ice 上报后仍解引用空指针（UB），收口为确定性 panic。
    let first_value: Option<*mut AstExpr> = values_slice.first().copied();
    if first_value.is_none() {
      self.ice_string("expected at least an iterator function value, but we parsed nothing");
    }
    let first_value =
      first_value.expect("forin.values 非空为 ice 前置契约，空值已在上方 ice 分支兜底");
    // SAFETY: first_value 非 null，指向 AST arena 节点。
    let first_value_ref = unsafe { &*first_value };

    // TypeId iterTy = nullptr;
    // TypePackId callRetPack = nullptr;
    let mut iter_ty: TypeId;
    let mut call_ret_pack: TypePackId = null();

    // if (forin.values.size == 1 && firstValue->is<AstExprCall>())
    // SAFETY: repr(C) base 偏移 0，cast 有效。
    let first_value_call = ast_node_try_as::<AstExprCall>(unsafe { &(*first_value).base });
    if forin.values.size == 1
      && let Some(expr_call) = first_value_call
    {
      // callRetPack = checkExprPack(scope, *exprCall).type;
      // callRetPack = follow(callRetPack);
      call_ret_pack = self.check_expr_pack(scope, &expr_call.base).r#type;
      // follow 为安全函数；call_ret_pack 是刚由 checkExprPack 产出的 arena pack 句柄。
      call_ret_pack = follow_type_pack::follow(call_ret_pack);

      // if (get<FreeTypePack>(callRetPack))
      if get_type_pack::get::<FreeTypePack>(call_ret_pack).is_some() {
        // iterTy = freshType(scope);
        iter_ty = self.fresh_type_scope_ptr(scope.clone());

        // unify(callRetPack, addTypePack({{iterTy}, freshTypePack(scope)}), scope, forin.location);
        let fresh_tail = self.fresh_type_pack_scope_ptr(scope);
        let expected = self.add_type_pack_vector_type_id_optional_type_pack_id(
          &alloc::vec![iter_ty],
          Some(fresh_tail),
        );
        self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
          call_ret_pack,
          expected,
          scope,
          &forin.base.base.location,
          CountMismatchContext::Arg,
        );
      }
      // else if (get<ErrorTypePack>(callRetPack) || !first(callRetPack))
      else if get_type_pack::get::<ErrorTypePack>(call_ret_pack).is_some()
        || first(call_ret_pack, true).is_none()
      {
        // for (TypeId var : varTypes)
        //     unify(errorRecoveryType(scope), var, scope, forin.location);
        let err_ty = self.error_recovery_type_scope_ptr(scope);
        for &var in &var_types {
          self.unify_type_id_type_id_scope_ptr_location(
            err_ty,
            var,
            scope,
            &forin.base.base.location,
          );
        }

        // return check(loopScope, *forin.body);
        // body 已句柄化为非空 Node（for-in 必有循环体），本 error 恢复分支
        // 直接进入 body 检查。
        return self.check_stat_block(&loop_scope, forin.body.get());
      }
      // else
      else {
        // iterTy = *first(callRetPack);
        iter_ty = first(call_ret_pack, true)
          .expect("上方分支已对同一 pack 判 first(..).is_some()，else 支必为 Some");
        // iterTy = instantiate(scope, iterTy, exprCall->location);
        iter_ty = self.instantiate(
          scope,
          iter_ty,
          expr_call.base.base.location,
          TxnLog::empty(),
        );
      }
    } else {
      // iterTy = instantiate(scope, checkExpr(scope, *firstValue).type, firstValue->location);
      let checked = self.check_expr(scope, first_value_ref, None, false).r#type;
      iter_ty = self.instantiate(
        scope,
        checked,
        first_value_ref.base.location,
        TxnLog::empty(),
      );
    }

    // iterTy = stripFromNilAndReport(iterTy, firstValue->location);
    iter_ty = self.strip_from_nil_and_report(iter_ty, &first_value_ref.base.location);

    // if (std::optional<TypeId> iterMM = findMetatableEntry(iterTy, "__iter", firstValue->location, /* addErrors= */ true))
    if self
      .find_metatable_entry(
        iter_ty,
        String::from("__iter"),
        &first_value_ref.base.location,
        true,
      )
      .is_some()
    {
      // if __iter metamethod is present, it will be called and the results are going to be called as if they are functions
      // for (TypeId var : varTypes)
      //     unify(any_type, var, scope, forin.location);
      let any_type = self.any_type;
      for &var in &var_types {
        self.unify_type_id_type_id_scope_ptr_location(
          any_type,
          var,
          scope,
          &forin.base.base.location,
        );
      }

      // return check(loopScope, *forin.body);
      // 走到 __iter 元方法分支后循环体照常检查；body 句柄非空性如前，且本分支
      // 之前对 body 无任何写操作。
      return self.check_stat_block(&loop_scope, forin.body.get());
    }

    // if (const TableType* iterTable = get<TableType>(iterTy))
    if let Some(iter_table) = get_table_type(iter_ty) {
      // if (iterTable->indexer)
      if let Some(indexer) = iter_table.indexer {
        // if (varTypes.size() > 0)
        //     unify(iterTable->indexer->index_type, varTypes[0], scope, forin.location);
        if !var_types.is_empty() {
          self.unify_type_id_type_id_scope_ptr_location(
            indexer.index_type,
            var_types[0],
            scope,
            &forin.base.base.location,
          );
        }

        // if (varTypes.size() > 1)
        //     unify(iterTable->indexer->indexResultType, varTypes[1], scope, forin.location);
        if var_types.len() > 1 {
          self.unify_type_id_type_id_scope_ptr_location(
            indexer.index_result_type,
            var_types[1],
            scope,
            &forin.base.base.location,
          );
        }

        // for (size_t i = 2; i < varTypes.size(); ++i)
        //     unify(nil_type, varTypes[i], scope, forin.location);
        let nil_type = self.nil_type;
        for &var in &var_types[2..] {
          self.unify_type_id_type_id_scope_ptr_location(
            nil_type,
            var,
            scope,
            &forin.base.base.location,
          );
        }
      } else {
        // for (TypeId var : varTypes)
        //     unify(unknown_type, var, scope, forin.location);
        let unknown_type = self.unknown_type;
        for &var in &var_types {
          self.unify_type_id_type_id_scope_ptr_location(
            unknown_type,
            var,
            scope,
            &forin.base.base.location,
          );
        }
      }

      // return check(loopScope, *forin.body);
      // table 可迭代分支收尾：forin.body 已句柄化为非空 Node（parser 必建），
      // loop_scope 在本分支存活并对应 C++ 的 shared_ptr<Scope> 传参。
      return self.check_stat_block(&loop_scope, forin.body.get());
    }

    // const FunctionType* iterFunc = get<FunctionType>(iterTy);
    // if (!iterFunc)
    let Some(iter_func) = get_type::get::<FunctionType>(iter_ty) else {
      // TypeId varTy = get<AnyType>(iterTy) ? any_type : errorRecoveryType(loopScope);
      let var_ty = if get_type::get::<AnyType>(iter_ty).is_some() {
        self.any_type
      } else {
        self.error_recovery_type_scope_ptr(&loop_scope)
      };

      // for (TypeId var : varTypes)
      //     unify(varTy, var, scope, forin.location);
      for &var in &var_types {
        self.unify_type_id_type_id_scope_ptr_location(
          var_ty,
          var,
          scope,
          &forin.base.base.location,
        );
      }

      // if (!get<ErrorType>(iterTy) && !get<AnyType>(iterTy) && !get<FreeType>(iterTy) && !get<NeverType>(iterTy))
      //     reportError(firstValue->location, CannotCallNonFunction{iterTy});
      if get_type::get::<ErrorType>(iter_ty).is_none()
        && get_type::get::<AnyType>(iter_ty).is_none()
        && get_type::get::<FreeType>(iter_ty).is_none()
        && get_type::get::<NeverType>(iter_ty).is_none()
      {
        self.report_error_location_type_error_data(
          &first_value_ref.base.location,
          CannotCallNonFunction { ty: iter_ty }.into(),
        );
      }

      // return check(loopScope, *forin.body);
      // iterTy 非函数走错误恢复后仍须检查循环体；body 句柄同上（parser 必建）。
      return self.check_stat_block(&loop_scope, forin.body.get());
    };
    // We only need the function's argTypes/retTypes; capture them up front so we
    // can keep mutably borrowing `self` for the remaining unifications.
    let iter_func_arg_types = iter_func.arg_types;
    let iter_func_ret_types = iter_func.ret_types;

    // if (forin.values.size == 1)
    if forin.values.size == 1 {
      // TypePackId argPack = nullptr;
      let arg_pack: TypePackId;

      // if (firstValue->is<AstExprCall>())
      if first_value_call.is_some() {
        // Extract the remaining return values of the call
        // auto [types, tail] = flatten(callRetPack);
        let (types, tail) = flatten_type_pack_id(call_ret_pack);

        if !types.is_empty() {
          // std::vector<TypeId> argTypes = std::vector<TypeId>(types.begin() + 1, types.end());
          // argPack = addTypePack(TypePackVar{TypePack{std::move(argTypes), tail}});
          let arg_types: Vec<TypeId> = types[1..].to_vec();
          arg_pack =
            self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::new(arg_types, tail)));
        } else {
          // argPack = addTypePack(TypePack{});
          arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::empty()));
        }
      } else {
        // Check if iterator function accepts 0 arguments
        // argPack = addTypePack(TypePack{});
        arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack::empty()));
      }

      // Unifier state = mkUnifier(loopScope, firstValue->location);
      let mut state = self.mk_unifier(&loop_scope, &first_value_ref.base.location);

      // checkArgumentList(loopScope, *firstValue, state, argPack, iterFunc->argTypes, /*argLocations*/ {});
      self.check_argument_list(
        &loop_scope,
        first_value_ref,
        &mut state,
        arg_pack,
        iter_func_arg_types,
        &Vec::new(),
      );

      // state.log.commit();
      state.log.commit();

      // reportErrors(state.errors);
      let state_errors = state.errors.clone();
      self.report_errors(&state_errors);
    }

    // TypePackId retPack = iterFunc->retTypes;
    let mut ret_pack: TypePackId = iter_func_ret_types;

    // if (forin.values.size >= 2)
    if values_slice.len() >= 2 {
      let arguments = AstArray::from_slice(&values_slice[1..]);

      // Position start = firstValue->location.begin;
      // Position end = values[forin.values.size - 1]->location.end;
      let start: Position = first_value_ref.base.location.begin;
      // 末元素只读 location 走 iter_nodes（len>=2 守卫在前，取不到仅为不可达）。
      let end: Position = forin
        .values
        .iter_nodes()
        .last()
        .expect("values 至少 2 项（上方 len>=2 守卫）")
        .base
        .location
        .end;

      // AstExprCall exprCall{Location(start, end), firstValue, arguments, /* self= */ false, AstArray<AstTypeOrPack>{}, Location()};
      let expr_call = AstExprCall::new(
        Location::new(start, end),
        first_value,
        arguments,
        false,
        // C 型：手写空数组哨兵改用 ulua-ast 的既有空值门面（空集即 EMPTY）。
        AstArray::<AstTypeOrPack>::EMPTY,
        Location::default(),
      );

      // retPack = checkExprPack(scope, exprCall).type;
      // 栈上合成节点（C++ 同为栈对象）；repr(C) base 偏移 0，&base 与原 cast 写法指针值一致。
      ret_pack = self.check_expr_pack(scope, &expr_call.base).r#type;
    }

    // We need to remove 'nil' from the set of options of the first return value
    // if (std::optional<TypeId> fty = first(retPack); fty && !varTypes.empty())
    if let Some(fty) = first(ret_pack, true)
      && !var_types.is_empty()
    {
      // TypeId keyTy = follow(*fty);
      let mut key_ty = follow_type::follow(fty);

      // if (get<UnionType>(keyTy))
      //     if (std::optional<TypeId> ty = tryStripUnionFromNil(keyTy)) keyTy = *ty;
      if get_type::get::<UnionType>(key_ty).is_some()
        && let Some(stripped) = self.try_strip_union_from_nil(key_ty)
      {
        key_ty = stripped;
      }

      // unify(keyTy, varTypes.front(), scope, forin.location);
      self.unify_type_id_type_id_scope_ptr_location(
        key_ty,
        var_types[0],
        scope,
        &forin.base.base.location,
      );

      // We have already handled the first variable type, make it match in the pack check
      // varTypes.front() = *fty;
      var_types[0] = fty;
    }

    // TypePackId varPack = addTypePack(TypePackVar{TypePack{std::move(varTypes), freshTypePack(scope)}});
    let fresh_var_tail = self.fresh_type_pack_scope_ptr(scope);
    let var_pack =
      self.add_type_pack_vector_type_id_optional_type_pack_id(&var_types, Some(fresh_var_tail));

    // unify(retPack, varPack, scope, forin.location);
    self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
      ret_pack,
      var_pack,
      scope,
      &forin.base.base.location,
      CountMismatchContext::Arg,
    );

    // check(loopScope, *forin.body);
    // for-in 正常路径末尾执行循环体，forin.body 已句柄化为非空 Node（parser
    // 必建，arena 生命周期覆盖整个模块检查）。
    self.check_stat_block(&loop_scope, forin.body.get());

    // return ControlFlow::None;
    ControlFlow::None
  }

  pub fn check_stat_function(
    &mut self,
    scope: &ScopePtr,
    mut ty: TypeId,
    fun_scope: &ScopePtr,
    function: &AstStatFunction,
  ) -> ControlFlow {
    // name 已句柄化为 Node<AstExpr>（repr(C) 基类视图经首字段取，免裸指针 cast）。
    let name_node = &function.name.get().base;

    if let Some(expr_name) = ast_node_try_as::<AstExprGlobal>(name_node) {
      let module_scope = self.expect_current_module().get_module_scope();
      let name = Symbol::from_global(expr_name.name);
      let previously_defined =
        self.is_nonstrict_mode() && module_scope.bindings.contains_key(&name);
      let old_binding = if previously_defined {
        module_scope.bindings.get(&name).cloned()
      } else {
        None
      };

      // SAFETY: module_scope 为模块根 scope 的 Arc 引用（get_module_scope），同步执行期内
      // 无其他借用，scope_mut 的独占写契约成立；此处先登记未量化 ty，body 检查后再覆写。
      unsafe {
        (*scope_mut(&module_scope)).bindings.insert(
          name.clone(),
          Binding {
            type_id: ty,
            location: expr_name.base.base.location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }

      // func 已句柄化为 Node（`function f()...end` 全局名分支，parser 必建，
      // arena 生命周期覆盖整个模块），`.get()` 直出只读引用交检查体。
      self.check_function_body(fun_scope, ty, function.func.get());

      let final_binding = if let Some(old) = old_binding {
        old
      } else {
        Binding {
          type_id: self.quantify(fun_scope, ty, expr_name.base.base.location),
          location: expr_name.base.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        }
      };

      // SAFETY: 对同一 module_scope Arc 的第二次写（quantify 后的最终绑定），
      // 前一处 scope_mut 产生的引用已不存在，独占写前提再次成立。
      unsafe {
        (*scope_mut(&module_scope))
          .bindings
          .insert(name, final_binding);
      }

      return ControlFlow::None;
    }

    if let Some(local_name) = ast_node_try_as::<AstExprLocal>(name_node) {
      let symbol = Symbol::from_local(local_name.local.as_ptr());
      // AstExprLocal 的 local 槽已句柄化恒非空（parser 符号解析时生成的 arena
      // AstLocal 绑定节点），此处 .get() 只读 location 字段。
      let name_location = local_name.local.get().location;

      // SAFETY: scope 为形参传入的 Arc<Scope>，insert 在块内即时完成（对应 C++
      // scope->bindings.insert），瞬时独占写满足 scope_mut 契约。
      unsafe {
        (*scope_mut(scope)).bindings.insert(
          symbol.clone(),
          Binding {
            type_id: ty,
            location: name_location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }

      // func 句柄化后 `.get()` 即安全引用，同全局分支；此分支先写入未量化绑定，
      // body 内的递归自引用即依赖它在 scope.bindings 中可见。
      self.check_function_body(fun_scope, ty, function.func.get());

      let quantified_ty = self.quantify(fun_scope, ty, name_location);
      let quantified = self.any_if_nonstrict(quantified_ty);
      // SAFETY: quantify（含 any_if_nonstrict 降级）后覆盖先前占位绑定；
      // 此刻对 scope Arc 无并发写者，独占契约仍然成立。
      unsafe {
        (*scope_mut(scope)).bindings.insert(
          symbol,
          Binding {
            type_id: quantified,
            location: name_location,
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: None,
          },
        );
      }

      return ControlFlow::None;
    }

    if let Some(index_name) = ast_node_try_as::<AstExprIndexName>(name_node) {
      let expr_ty = self
        .check_expr(
          scope,
          // index_name.expr 已句柄化恒非空：.get() 安全借用。
          index_name.expr.get(),
          None,
          false,
        )
        .r#type;
      let mut ttv = get_mutable_table_type(expr_ty);
      // SAFETY: index.value 为 NUL 结尾 C 字符串（AST arena 持有）。
      let prop_name = index_name.index.as_str_or_empty().to_string();

      if self
        .get_index_type_from_type(
          scope.clone(),
          expr_ty,
          &prop_name,
          &index_name.index_location,
          false,
        )
        .is_none()
      {
        let data = if ttv.is_some() || is_table_intersection(expr_ty) {
          TypeErrorData::CannotExtendTable(CannotExtendTable {
            table_type: expr_ty,
            context: Context::Property,
            prop: prop_name.clone(),
          })
        } else {
          TypeErrorData::OnlyTablesCanHaveMethods(OnlyTablesCanHaveMethods {
            table_type: expr_ty,
          })
        };

        self.report_error_location_type_error_data(&function.base.base.location, data);
      }

      ty = follow_type::follow(ty);
      if let Some(ttv) = ttv.as_deref_mut()
        && ttv.state != TableState::Sealed
      {
        let property = ttv.props.entry(prop_name.clone()).or_default();
        property.set_type(ty);
        property.location = Some(index_name.index_location);
      }

      // 方法糖 `t:m()` 路径，func 句柄化后 `.get()` 即安全引用（parser 必建）；
      // 检查体之前 property 已挂到 expr_ty 的表上。
      self.check_function_body(fun_scope, ty, function.func.get());

      // SAFETY: 同上，check_function_body 可能改变 state，故再次读取。
      if let Some(ttv) = ttv
        && ttv.state != TableState::Sealed
      {
        let quantified =
          follow_type::follow(self.quantify(fun_scope, ty, index_name.index_location));
        let property = ttv.props.entry(prop_name).or_default();
        property.set_type(quantified);
        property.location = Some(index_name.index_location);
      }

      return ControlFlow::None;
    }

    // 名字既非 global/local/indexName 的兜底分支，func 仍是 parser 必建的句柄
    // （errorRecovery 路径亦由 parser 填充占位函数体），`.get()` 即安全引用。
    self.check_function_body(fun_scope, ty, function.func.get());
    ControlFlow::None
  }

  pub fn check_stat_local_function(
    &mut self,
    scope: &ScopePtr,
    ty: TypeId,
    fun_scope: &ScopePtr,
    function: &AstStatLocalFunction,
  ) -> ControlFlow {
    // C++: scope->bindings[localFunction->name] = {ty, location}; Rust 侧 Scope 装在 Arc 中，
    // 经 arc_as_mut 取独占写指针（契约要求调用期内无其他 &mut 引用）。
    // SAFETY: scope 为调用方持有的存活 Arc<Scope>，arc_as_mut 独占写惯用法；
    // function 为安全引用指向的存活 AST 节点，name 已句柄化为 Node（as_ptr 仅供
    // Symbol 指针身份键），base 字段仅只读。
    unsafe {
      let scope_mut = arc_as_mut(scope);
      (*scope_mut).bindings.insert(
        Symbol::from_local(function.name.as_ptr()),
        Binding {
          type_id: ty,
          location: function.base.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    // func/name 已句柄化为 Node（local function 语法上必带函数体与绑定名，parser
    // 必建非空由类型层承载），`.get()` 直出安全引用。
    self.check_function_body(fun_scope, ty, function.func.get());

    let name_location = function.name.get().location;
    let quantified = self.quantify(fun_scope, ty, name_location);
    // SAFETY: quantify 之后用量化结果覆盖同名绑定；arc_as_mut 契约同上前一处（单线程独占写），
    // 且自引用函数体已在上面检查完毕，此刻无其他活动借用。
    unsafe {
      let scope_mut = arc_as_mut(scope);
      (*scope_mut).bindings.insert(
        Symbol::from_local(function.name.as_ptr()),
        Binding {
          type_id: quantified,
          location: name_location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    ControlFlow::None
  }

  pub fn check_stat_type_alias(
    &mut self,
    scope: &ScopePtr,
    typealias: &AstStatTypeAlias,
  ) -> ControlFlow {
    // AstName 由词法器 intern，必为合法 ASCII/UTF-8；空名（null）按 "" 处理。
    if typealias.name.as_bytes() == b"%error-id%" || typealias.name.as_bytes() == b"typeof" {
      return ControlFlow::None;
    }

    let name: Name = typealias.name.as_str_or_empty().to_string();

    if self
      .duplicate_type_aliases
      .contains(&(typealias.exported, name.clone()))
    {
      return ControlFlow::None;
    }

    let binding = if typealias.exported {
      scope.exported_type_bindings.get(&name).cloned()
    } else {
      scope.private_type_bindings.get(&name).cloned()
    };

    let Some(binding) = binding else {
      return ControlFlow::None;
    };

    let alias_scope = self.child_scope(scope, &typealias.base.base.location);
    {
      // SAFETY: alias_scope 刚由 child_scope 分配，除局部变量外唯一共享引用在 self 的
      // scope 栈里且此刻未被借用，满足 scope_mut 的单线程独占写契约。
      let alias_scope_mut = unsafe { &mut *scope_mut(&alias_scope) };
      alias_scope_mut.level = scope.level.incr();

      for param in binding.type_params() {
        if let Some(generic) = get_type::get::<GenericType>(param.ty) {
          alias_scope_mut
            .private_type_bindings
            .insert(generic.name.clone(), TypeFun::type_fun_type_id(param.ty));
        }
      }

      for param in binding.type_pack_params() {
        if let Some(generic) = get_type_pack::get::<GenericTypePack>(param.tp) {
          alias_scope_mut
            .private_type_pack_bindings
            .insert(generic.name.clone(), param.tp);
        }
      }
    }

    // SAFETY: type_ptr 指向 AST arena 节点（parser 保证非空）。
    let mut ty = self.resolve_type(alias_scope.clone(), unsafe { &*typealias.type_ptr });

    // `get_mutable` requires a followed type (it asserts the arg is not a
    // BoundType). `ty` here is the raw result of `resolve_type`, which for
    // a self-referential / chained alias (e.g. `type A = A`, or `type T =
    // Pt; type Pt = ... T ...`) is a Bound — so follow before inspecting it,
    // matching the sibling `check_stat_local`. (C++ Luau's
    // assert is compiled out in release, masking this; our fuzz build arms
    // it, where it aborted.)
    ty = follow_type::follow(ty);
    if let Some(table) = get_mutable_type::get_mutable::<TableType>(ty) {
      let type_params: Vec<_> = binding.type_params().iter().map(|param| param.ty).collect();
      let type_pack_params: Vec<_> = binding
        .type_pack_params()
        .iter()
        .map(|param| param.tp)
        .collect();

      let same_tys = table.instantiated_type_params == type_params;
      let same_tps = table.instantiated_type_pack_params == type_pack_params;

      if table.name.is_some() && (table.name.as_ref() != Some(&name) || !same_tys || !same_tps) {
        let mut clone = table.clone();
        clone.name = Some(name.clone());
        clone.instantiated_type_params = type_params;
        clone.instantiated_type_pack_params = type_pack_params;
        ty = self.add_type(&clone);
      } else {
        table.name = Some(name.clone());
        table.instantiated_type_params = type_params;
        table.instantiated_type_pack_params = type_pack_params;
      }
    } else if let Some(metatable) = get_mutable_type::get_mutable::<MetatableType>(ty) {
      metatable.synthetic_name = Some(name.clone());
    }

    // SAFETY: 长借用跨 unify 调用，对应 C++ `auto& scope = *scope;` 的引用语义；
    // scope 由外层 Arc 保活，unify 期间无其他 Arc 克隆持有写权（scope_mut 契约）。
    let scope_ref = unsafe { &mut *scope_mut(scope) };
    let bindings = if typealias.exported {
      &mut scope_ref.exported_type_bindings
    } else {
      &mut scope_ref.private_type_bindings
    };

    if let Some(binding) = bindings.get_mut(&name) {
      self.unify_type_id_type_id_scope_ptr_location(
        ty,
        binding.r#type,
        &alias_scope,
        &typealias.base.base.location,
      );

      // C++：follow 后仍为 Free 时覆写，否则也覆写（两分支同值，保持原翻译）。
      binding.r#type = ty;
    }

    ControlFlow::None
  }

  pub fn check_stat_type_function(
    &mut self,
    _scope: &ScopePtr,
    typefunction: &AstStatTypeFunction,
  ) -> ControlFlow {
    self.report_error_location_type_error_data(
      &typefunction.base.base.location,
      TypeErrorData::GenericError(GenericError::new(
        "This syntax is not supported".to_string(),
      )),
    );

    ControlFlow::None
  }

  pub fn check_stat_declare_extern_type(
    &mut self,
    scope: &ScopePtr,
    declared_extern_type: &AstStatDeclareExternType,
  ) -> ControlFlow {
    // SAFETY: name.value 为 NUL 结尾 C 字符串（AST arena 持有）。
    let class_name: Name = declared_extern_type.name.as_str_or_empty().to_string();

    if self
      .incorrect_extern_type_definitions
      .contains(&(declared_extern_type as *const AstStatDeclareExternType))
    {
      return ControlFlow::None;
    }

    let binding: Option<TypeFun> = scope.exported_type_bindings.get(&class_name).cloned();
    let Some(binding) = binding else {
      self.ice_string("Extern type not predeclared");
      return ControlFlow::None;
    };

    let extern_ty = binding.r#type;
    let Some(etv) = get_mutable_type::get_mutable::<ExternType>(extern_ty) else {
      self.ice_string("Extern type binding was not an extern type");
      return ControlFlow::None;
    };

    let Some(metatable_ty) = etv.metatable else {
      self.ice_string("No metatable for declared extern type");
      return ControlFlow::None;
    };

    if !declared_extern_type.indexer.is_null() {
      // SAFETY: indexer 非 null，指向 AST arena 节点。
      let indexer = unsafe { &*declared_extern_type.indexer };
      // SAFETY: 注解节点指向 AST arena 节点。
      let index_type = self.resolve_type(scope.clone(), unsafe { &*indexer.index_type });
      // SAFETY: result_type 注解字段与 index_type 同源（存活 indexer 节点内），
      // 解引用仅只读。
      let result_type = self.resolve_type(scope.clone(), unsafe { &*indexer.result_type });
      etv.indexer = Some(TableIndexer {
        index_type,
        index_result_type: result_type,
        is_read_only: indexer.access == AstTableAccess::Read,
      });
    }

    let Some(metatable) = get_mutable_type::get_mutable::<TableType>(metatable_ty) else {
      self.ice_string("Declared extern type metatable was not a table");
      return ControlFlow::None;
    };

    for prop in declared_extern_type.props.iter() {
      // SAFETY: prop.name.value 为 NUL 结尾 C 字符串。
      let prop_name: Name = prop.name.as_str_or_empty().to_string();
      // SAFETY: prop.ty 指向 AST arena 节点。
      let prop_ty = self.resolve_type(scope.clone(), unsafe { &*prop.ty });
      let assign_to_metatable = is_metamethod(&prop_name);

      if prop.is_method
        && let Some(ftv) = get_mutable_type::get_mutable::<FunctionType>(prop_ty)
      {
        ftv.arg_names.insert(
          0,
          Some(FunctionArgument {
            name: "self".to_string(),
            location: prop.location,
          }),
        );
        let old_arg_types = ftv.arg_types;
        ftv.arg_types =
          self.add_type_pack_type_pack(TypePack::new(Vec::from([extern_ty]), Some(old_arg_types)));
        ftv.has_self = true;
        ftv.definition = Some(FunctionDefinition {
          definition_module_name: Some(self.expect_current_module().name.clone()),
          definition_location: prop.location,
          vararg_location: None,
          original_name_location: prop.name_location,
        });
      }

      let assign_to = if assign_to_metatable {
        &mut metatable.props
      } else {
        &mut etv.props
      };

      match assign_to.entry(prop_name.clone()) {
        Entry::Vacant(e) => {
          e.insert(
            Property::property_type_id_bool_string_optional_location_tags_optional_string_optional_location(
              prop_ty,
              false,
              String::new(),
              Some(prop.location),
              Default::default(),
              None,
              None,
            ),
          );
        }
        Entry::Occupied(mut e) => {
          let property = e.get_mut();
          let current_ty = property.type_deprecated();
          let current_ty = follow_type::follow(current_ty);

          if let Some(current_intersection) = get_type::get::<IntersectionType>(current_ty) {
            let mut options = current_intersection.parts.clone();
            options.push(prop_ty);
            let new_itv = self.add_type(&IntersectionType { parts: options });
            property.read_ty = Some(new_itv);
            property.write_ty = Some(new_itv);
          } else if get_type::get::<FunctionType>(current_ty).is_some() {
            let intersection = self.add_type(&IntersectionType {
              parts: Vec::from([current_ty, prop_ty]),
            });
            property.read_ty = Some(intersection);
            property.write_ty = Some(intersection);
          } else {
            self.report_error_location_type_error_data(
              &declared_extern_type.base.base.location,
              TypeErrorData::GenericError(GenericError::new(format!(
                "Cannot overload non-function class member '{}'",
                prop_name
              ))),
            );
          }
        }
      }
    }

    ControlFlow::None
  }

  pub fn check_stat_declare_function(
    &mut self,
    scope: &ScopePtr,
    global: &AstStatDeclareFunction,
  ) -> ControlFlow {
    let fun_scope = self.child_function_scope(scope, &global.base.base.location, 0);

    let defs = self.create_generic_types(
      &fun_scope,
      None,
      &global.base.base,
      &global.generics,
      &global.generic_packs,
      false,
    );

    let generic_tys = defs.generic_types.iter().map(|def| def.ty).collect();
    let generic_tps = defs.generic_packs.iter().map(|def| def.tp).collect();

    let arg_pack =
      self.resolve_type_pack_scope_ptr_ast_type_list(fun_scope.clone(), &global.params);
    let ret_pack = if global.ret_types.is_null() {
      self.add_type_pack_type_pack(TypePack::empty())
    } else {
      // SAFETY: ret_types 的可空性已由上一行 is_null 判定排除，此处指向
      // AstTypeList arena 节点（declare 语法 `-> (types)` 部分）。
      self
        .resolve_type_pack_scope_ptr_ast_type_pack(fun_scope.clone(), unsafe { &*global.ret_types })
    };

    let module_raw = arc_as_mut(self.expect_current_module());
    let defn = FunctionDefinition {
      definition_module_name: Some(
        self.expect_current_module()
          .name
          .clone(),
      ),
      definition_location: global.base.base.location,
      vararg_location: if global.vararg {
        Some(global.vararg_location)
      } else {
        None
      },
      original_name_location: global.name_location,
    };

    let mut ftv = FunctionType::function_type_new(arg_pack, ret_pack, Some(defn), false);
    ftv.level = fun_scope.level;
    ftv.generics = generic_tys;
    ftv.generic_packs = generic_tps;

    for (name, location) in global.param_names.iter() {
      if !name.value.is_null() {
        ftv.arg_names.push(Some(FunctionArgument {
          name: name.as_str_or_empty().to_string(),
          location: *location,
        }));
      } else {
        ftv.arg_names.push(None);
      }
    }

    let fn_ty = self.add_type(&ftv);
    let fn_name = global.name.as_str_or_empty().to_string();

    // SAFETY: module_raw 由 arc_as_mut 从仍存活的 current_module Arc 取得
    // （expect 保证非 None 且 Arc 在 self 中保活）；scope_raw 同理来自形参 Arc。
    // 对二者的写入瞬时完成、单线程独占，符合 C++ shared_ptr<Module>/Scope 直接改写语义。
    unsafe {
      (*module_raw).declared_globals.insert(fn_name, fn_ty);

      let scope_raw = arc_as_mut(scope);
      (*scope_raw).bindings.insert(
        Symbol::from_global(global.name),
        Binding {
          type_id: fn_ty,
          location: global.base.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    ControlFlow::None
  }

  pub fn check_stat_declare_global(
    &mut self,
    scope: &ScopePtr,
    global: &AstStatDeclareGlobal,
  ) -> ControlFlow {
    let global_ty = if global.type_.is_null() {
      self.error_recovery_type_scope_ptr(scope)
    } else {
      // SAFETY: is_null 判定在前，type_ 为 declare 语法显式给出的 AstType arena 节点。
      self.resolve_type(scope.clone(), unsafe { &*global.type_ })
    };
    let global_name = global.name.as_str_or_empty().to_string();

    // SAFETY: 与 declare_function 尾部同一模式——current_module/scope 两个 Arc 均
    // 存活（expect/形参保活），declared_globals 与 bindings 的插入为块内瞬时独占写。
    unsafe {
      let module = arc_as_mut(self.expect_current_module());
      (*module).declared_globals.insert(global_name, global_ty);

      let scope_raw = arc_as_mut(scope);
      (*scope_raw).bindings.insert(
        Symbol::from_global(global.name),
        Binding {
          type_id: global_ty,
          location: global.base.base.location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    ControlFlow::None
  }

  pub fn check_stat_error(
    &mut self,
    scope: &ScopePtr,
    error_statement: &AstStatError,
  ) -> ControlFlow {
    let module_ptr = arc_as_mut(self.expect_current_module());
    // 经 Arc 共享引用读取错误条目数（与 (*module_ptr).errors.len() 等价的写法），无需 deref。
    let old_size = self.expect_current_module().errors.len();

    // AstStatError 的 statements/expressions 为 parser 记录的错误恢复数组，
    // 元素指针的解引用收口在 AstArray::iter_nodes（只读遍历、cpp 同序）。
    for statement in error_statement.statements.iter_nodes() {
      self.check_stat(scope, statement);
    }

    for expr in error_statement.expressions.iter_nodes() {
      self.check_expr(scope, expr, None, false);
    }

    // SAFETY: 嵌套 check 期间新追加的 errors 均经同一 module_ptr（单线程顺序写入，
    // Vec 地址由 Module 内嵌稳定）；truncate 回滚 old_size 之后的临时错误。
    unsafe {
      (*module_ptr).errors.truncate(old_size);
    }

    ControlFlow::None
  }
}
