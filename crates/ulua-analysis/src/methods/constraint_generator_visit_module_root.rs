use alloc::{sync::Arc, vec::Vec};
use core::ptr::from_ref;

use ulua_ast::records::{ast_node::AstNode, ast_stat_block::AstStatBlock, location::Location};
use ulua_common::{
  fflag,
  macros::{luau_assert::LUAU_ASSERT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE},
};

use crate::{
  enums::{control_flow::ControlFlow, polarity::Polarity},
  functions::{
    add_all_as_dependencies::add_all_as_dependencies, checkpoint::checkpoint,
    for_each_constraint::for_each_constraint, get_mutable_type::get_mutable,
    shared_mut::shared_mut,
  },
  records::{
    arena_handle::alias, blocked_type::BlockedType, constraint::Constraint,
    constraint_generator::ConstraintGenerator, function_type::FunctionType,
    generalization_constraint::GeneralizationConstraint, interior_free_types::InteriorFreeTypes,
    pack_subtype_constraint::PackSubtypeConstraint, scope::Scope, scope_registry::register_scope,
    simplify_constraint::SimplifyConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr},
};

impl ConstraintGenerator {
  /// C++ `visitModuleRoot(AstStatBlock* block)`（ConstraintGenerator.cpp:367）。
  /// `block` 为整个 check 会话存活的模块 AST 根节点共享借用（唯一调用方
  /// [`ConstraintGenerator::run`] 传入已句柄化的 `source_module.root`，经
  /// `Handle::get` 收口）。
  /// 且须在约束生成开始前调用：`scopes` 为空、`root_scope` 为 null
  /// （体内 `LUAU_ASSERT!` 断言）。Scope/Module 裸句柄均以存活对象为锚，
  /// 写入经 alias 门面即时物化。
  pub fn visit_module_root(&mut self, block: &AstStatBlock) {
    LUAU_TIMETRACE_SCOPE!("ConstraintGenerator::visitModuleRoot", "Typechecking");

    LUAU_ASSERT!(self.scopes.is_empty());
    LUAU_ASSERT!(self.root_scope.is_none());

    let scope: ScopePtr = Arc::new(Scope::new(self.global_scope.as_ref().unwrap(), 0));
    register_scope(&scope);
    self.root_scope = Some(scope.clone());
    self.scopes.push((block.base.base.location, scope.clone()));
    // `scope` Arc 已 clone 登记进 `self.scopes`、存活至会话末；`alias` 收口对
    // Arc 目标的 C++ 式独占改写（C++ `rootScope->location = block->location`）。
    alias(shared_mut(self.root())).location = block.base.base.location;
    if let Some(module) = &self.module {
      let module_ptr = shared_mut(module);
      // `module` Arc 由 `self.module` 持有（构造断言 Some），经 alias 收口写
      // `module->astScopes`（对应 C++ `module->astScopes[block] =
      // NotNull{scope.get()}`）；键 `block` 与值 scope 均存活至会话末。
      *alias(module_ptr)
        .ast_scopes
        .get_or_insert(from_ref(block).cast::<AstNode>()) = shared_mut(&scope);
    }

    self.interior_free_types.push(InteriorFreeTypes::default());

    let local_type_function_scope: ScopePtr =
      Arc::new(Scope::new(self.type_function_scope.as_ref().unwrap(), 0));
    register_scope(&local_type_function_scope);
    // `local_type_function_scope` Arc 此刻仍归本局部量独占，alias 收口写
    // `.location` 发生在移交所有权（下一块）之前。
    alias(shared_mut(&local_type_function_scope)).location = block.base.base.location;
    // Safety: `self.type_function_runtime.as_ptr()` 是构造期 `NonNull<TypeFunctionRuntime>`
    // 注入、与宿主（`check_frontend` 局部量）同寿的句柄；此处把 scope Arc 所有权移交给
    // runtime（C++ `typeFunctionRuntime->rootScope = localTypeFunctionScope`），移交后
    // 由 runtime 负责保活。
    {
      self.type_function_runtime.get_mut().root_scope = local_type_function_scope;
    }

    let return_type = self.fresh_type_pack(&scope, Polarity::Positive);
    // `root_scope` 仍指向上方登记进 `self.scopes` 的存活模块根 scope，
    // 写 `.return_type` 与后续 arena/builtin 访问同 C++（cpp:389-390）。
    alias(shared_mut(self.root())).return_type = return_type;
    // Safety: `self.arena.as_ptr()`/`self.builtin_types.as_ptr()` 均为构造期注入、与会话同寿的句柄，
    // `add_type` 是 arena 独占追加，取 `any_type_pack` 为常量槽读取
    // （C++ `moduleFnTy = addType(FunctionType{builtinTypes->anyTypePack, returnType})`）。
    let module_fn_ty = {
      self
        .arena
        .get_mut()
        .add_type(FunctionType::function_type_new(
          self.builtin_types.get().any_type_pack,
          return_type,
          None,
          false,
        ))
    };

    // `scope` 是存活根 scope、`block` 为本函数参数里的存活共享借用。
    self.prepopulate_global_scope(&scope, block);

    let start = checkpoint(self);

    // `scope`（即 rootScope）为本地存活 Arc，`block` 为契约存活的 arena 根节点，
    // 逐一对应 C++ `visitBlockWithoutChildScope(rootScope, block)`。
    let cf = self.visit_block_without_child_scope(&scope, block);
    if cf == ControlFlow::None {
      // Safety: 读构造期注入的 `builtin_types` 句柄取 `empty_type_pack` 常量槽
      // （C++ `builtinTypes->emptyTypePack`）。
      let empty_type_pack = self.builtin_types.get().empty_type_pack;
      self.add_constraint_scope_ptr_location_constraint_v(
        &scope,
        // `block` 是本函数参数里的存活 AST 根，仅读其 location
        // （C++ `addConstraint(scope, block->location, PackSubtypeConstraint{..})`）。
        block.base.base.location,
        ConstraintV::PackSubtype(PackSubtypeConstraint {
          sub_pack: empty_type_pack,
          super_pack: return_type,
          returns: false,
        }),
      );
    }

    let end = checkpoint(self);

    // Safety: `self.arena.as_ptr()` 构造期 normalizer arena 句柄，`add_type` 独占追加。
    let result = self.arena.get_mut().add_type(BlockedType::default());
    let gen_constraint = self.add_constraint_scope_ptr_location_constraint_v(
      &scope,
      // 存活节点 `block` 再读 location（C++ `addConstraint(scope, block->location,
      // GeneralizationConstraint{...})`）。
      block.base.base.location,
      ConstraintV::Generalization(GeneralizationConstraint {
        generalized_type: result,
        source_type: module_fn_ty,
        interior_types: Vec::new(),
        has_deprecated_attribute: false,
        deprecated_info: Default::default(),
        no_generics: true,
      }),
    );

    // Safety: `root_scope` 仍指向上方登记进 `self.scopes` 且存活至会话末的模块根
    // Scope；`interior_free_types` 栈在函数开头 push 后，嵌套作用域的 push 均已
    // 成对 pop，`last().unwrap()` 必命中模块级帧（C++ `rootScope->interiorFreeTypes =
    // std::move(interiorFreeTypes.back().types)` 的 clone 等价写法，其后
    // `pop()` 移除同一帧）。
    let root = alias(shared_mut(self.root()));
    root.interior_free_types = Some(self.interior_free_types.last().unwrap().types.clone());
    root.interior_free_type_packs =
      Some(self.interior_free_types.last().unwrap().type_packs.clone());

    // result 刚由 add_type(BlockedType) 分配，必命中；对照 C++:422
    // `getMutable<BlockedType>(result)->setOwner(genConstraint)`
    let blocked = get_mutable::<BlockedType>(result).unwrap();
    blocked.set_owner(gen_constraint);

    if fflag::LuauConstraintGraph.get() {
      // Safety: flag 开启时前端（check_frontend）才分配 ConstraintGraph，
      add_all_as_dependencies(start, end, self, gen_constraint);
    } else {
      // Safety: 闭包收到的 `c` 是 `[start,end)` 区间内、已由 `Box::into_raw` 交付
      // ConstraintSet 持有、存活至会话末的约束指针；`gen_constraint` 在 `end`
      // 之后创建，不在被遍历区间内，写其 `deprecated_dependencies` 字段与
      // `for_each_constraint` 对区间 Vec 的不可变借用不相交（C++ 同段
      // `get<GeneralizationConstraint>(*genConstraint)->deprecatedDependencies.push_back(c)`）。
      for_each_constraint(start, end, self, |c: *mut Constraint| {
        alias(gen_constraint).deprecated_dependencies.push(c);
      });
    }

    self.interior_free_types.pop();

    self.fill_in_inferred_bindings(&scope, block);

    if !self.logger.is_null() {
      // 先经 `is_null` 短路，`logger` 非空时是构造期注入、与 check 会话同寿的
      // `DcrLogger` 裸句柄（记录布局不改），解引用经 alias 收口；`self.module`
      // 构造断言 Some，其 Arc clone 移交后仍由 generator 持有（对应 C++
      // `logger->captureGenerationModule(module)`）。
      alias(self.logger).capture_generation_module(self.module.clone().unwrap());
    }

    // 与 `visit_fragment_root` 共用的 localTypes 泛化收尾（cpp 两函数尾部同体）。
    self.generalize_local_types(&scope, Location::default());

    let unions_to_simplify = self.unions_to_simplify.clone();
    for ty in unions_to_simplify {
      self.add_constraint_scope_ptr_location_constraint_v(
        &scope,
        // 再读存活节点 `block` 的 location（C++ `addConstraint(scope,
        // block->location, SimplifyConstraint{ty})`）。
        block.base.base.location,
        ConstraintV::Simplify(SimplifyConstraint { ty }),
      );
    }
  }
}
