use alloc::vec::Vec;
use core::{mem::take, ptr::from_ref};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_local::AstLocal, location::Location,
};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use super::constraint_generator_prototype_type_definitions::make_binding;
use crate::{
  enums::polarity::Polarity,
  functions::{
    arc_as_mut::arc_as_mut, as_mutable_type_pack::as_mutable_type_pack,
    begin_type::begin_union_type, bind_free_type::bind_free_type,
    emplace_type_pack::emplace_type_pack, extend_type_pack::extend_type_pack, follow_type,
    follow_type_pack, get_type, get_type_pack, is_optional::is_optional, is_prim::is_nil,
  },
  records::{
    any_type::AnyType,
    arena_handle::{Handle, alias, alias_opt, alias_ref},
    class_decl_record::ClassDeclRecord,
    constraint_generator::ConstraintGenerator,
    free_type::FreeType,
    function_argument::FunctionArgument,
    function_definition::FunctionDefinition,
    function_signature::FunctionSignature,
    function_type,
    symbol::Symbol,
    type_pack::TypePack,
    union_type::UnionType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};
impl ConstraintGenerator {
  /// C++ `checkFunctionSignature(const ScopePtr& parent, ClassDeclRecord* enclosingClass,
  /// AstExprFunction* fn, std::optional<TypeId> expectedType, std::optional<Location>)`
  /// （ConstraintGenerator.cpp:4285）。形参前提（原 `# Safety` 契约由签名承担）：
  /// * `parent`：调用方持有的存活 `ScopePtr` 共享借用，派生的 signature/body scope
  ///   登记为其 child；
  /// * `enclosing_class`：`DebugLuauUserDefinedClasses` 关闭时必须为 null（见函数首
  ///   `LUAU_ASSERT`）；非空时须指向约束生成期存活的 `ClassDeclRecord`（C++ 由
  ///   `classDeclRecords` map 持有，字段布局保持裸指针键/值），其 `ty` 为 arena 驻留
  ///   TypeId；
  /// * `fn_node`：parse arena 持有的 `AstExprFunction` 共享借用，存活至检查会话结束，
  ///   其 `generics/args/body/self_` 与各级 annotation 子节点由同一 AST 生命期担保；
  /// * `expected_type`：可选；给出时须为 arena 驻留、`follow` 后仍有效的 TypeId。
  ///
  /// 另要求 `self` 的构造期不变量成立：`arena`/`builtin_types` 为构造期注入的非空
  /// 句柄、`dfg` 为非空只读句柄、`module` 的 Arc 目标与会话同寿（对应 C++ NotNull）。
  pub fn check_function_signature(
    &mut self,
    parent: &ScopePtr,
    enclosing_class: *mut ClassDeclRecord,
    fn_node: &AstExprFunction,
    expected_type: Option<TypeId>,
    original_name: Option<Location>,
  ) -> FunctionSignature {
    LUAU_ASSERT!(
      fflag::DebugLuauUserDefinedClasses.get() || enclosing_class.is_null(),
      "check_function_signature: enclosing_class must be null when DebugLuauUserDefinedClasses is off"
    );

    let mut generic_types: Vec<TypeId> = Vec::new();
    let mut generic_type_packs: Vec<TypePackId> = Vec::new();

    let mut expected_type_opt = expected_type;
    if let Some(et) = expected_type_opt {
      expected_type_opt = Some(follow_type::follow(et));
    }

    let has_generics = !fn_node.generics.is_empty() || !fn_node.generic_packs.is_empty();

    // child_scope 契约要求 node 为存活 AstNode（repr(C) 首字段基址重合）、
    // parent 调用期有效（C++:4306 `childScope(fn, parent)`）。
    let signature_scope: ScopePtr = self.child_scope(&fn_node.base.base, parent);

    // We need to assign returnType before creating bodyScope so that the
    // return type gets propagated to bodyScope.
    let mut return_type: TypePackId = self.fresh_type_pack(&signature_scope, Polarity::Positive);
    // 写窗口经 alias 收口即时物化：signature_scope 已在 child_scope 内 clone 登记
    // 进 `self.scopes` 保活至会话末，单线程独占写 `return_type`
    // 即 C++:4313 `signatureScope->returnType = returnType`。
    alias(arc_as_mut(&signature_scope)).return_type = return_type;

    // fn_node.body 是 fn 节点契约担保的存活 AstBlock（C++:4315
    // `childScope(fn->body, signatureScope)`）；signature_scope 为上方刚创建并
    // 登记进 self.scopes 的存活 scope。
    let body_scope: ScopePtr = self.child_scope(&fn_node.body.get().base.base, &signature_scope);

    if has_generics {
      let generic_definitions = self.create_generics_nodes(
        &signature_scope,
        &fn_node.generics,
        // C++ `createGenerics(signatureScope, fn->generics)` uses the
        // default `useCache = false` (ConstraintGenerator.h:481): each
        // function signature gets fresh generics keyed to its own
        // signature scope. Passing `true` here aliased every same-named
        // generic (e.g. `S`) to the first one created, leaving its
        // `scope` pointing at an unrelated sibling scope so `subsumes`
        // failed and nested generic subtyping spuriously rejected.
        false,
        true,
      );
      let generic_pack_definitions = self.create_generic_packs_nodes(
        &signature_scope,
        &fn_node.generic_packs,
        // C++ `createGenericPacks(signatureScope, fn->genericPacks)` uses
        // the default `useCache = false` (ConstraintGenerator.h:498),
        // matching the `createGenerics` call above: fresh per-signature
        // generic packs rather than name-aliased cached ones.
        false,
        true,
      );

      for (_name, g) in &generic_definitions {
        generic_types.push(g.ty);
      }

      for (_name, g) in &generic_pack_definitions {
        generic_type_packs.push(g.tp);
      }

      expected_type_opt = None;
    }

    let mut arg_types: Vec<TypeId> = Vec::new();
    let mut arg_names: Vec<Option<FunctionArgument>> = Vec::new();
    let mut expected_arg_pack = TypePack::empty();

    // 对照 C++:4254 `const FunctionType* expectedFunction = expectedType ? get<FunctionType>(*expectedType) : nullptr;`
    let mut expected_function: Option<&function_type::FunctionType> =
      expected_type_opt.and_then(get_type::get::<function_type::FunctionType>);

    // This check ensures that expected_type is precisely optional and not any
    // (since any is also an optional type)
    if let Some(et) = expected_type_opt
      && is_optional(et)
      && get_type::get::<AnyType>(et).is_none()
      && let Some(ut) = get_type::get::<UnionType>(et)
    {
      // C++ `for (auto u : ut)`——UnionTypeIterator 展平嵌套 union 并
      // follow,裸遍历 options 会漏掉嵌套成员。
      for u in begin_union_type(ut) {
        // 对照 C++:4261-4266 `if (!isNil(u) && get<FunctionType>(follow(u)))`
        if let Some(ft) = get_type::get::<function_type::FunctionType>(u)
          && !is_nil(u)
        {
          expected_function = Some(ft);
          break;
        }
      }
    }

    if let Some(ef) = expected_function {
      // Safety: `self.arena.get_mut()` 是构造期注入 arena 句柄的独占短借用，
      // `self.builtin_types.as_ptr()` 同为构造期非空句柄（只读取 never/unknown 常量），
      expected_arg_pack = extend_type_pack(
        self.arena.get_mut(),
        Handle::from_ptr(self.builtin_types.as_ptr()),
        ef.arg_types,
        fn_node.args.len(),
        Vec::new(),
      );

      generic_types = ef.generics.clone();
      generic_type_packs = ef.generic_packs.clone();
    }

    // flag-off 时 has_explicit_self 恒为 false，与 C++ 分支合并后语义一致。
    let mut has_explicit_self = false;
    if fflag::DebugLuauUserDefinedClasses.get()
      && !enclosing_class.is_null()
      && !fn_node.args.is_empty()
    {
      let first_arg = fn_node.args[0].get();
      has_explicit_self = first_arg.name.as_str_or_empty() == "self";
    }
    let has_self = has_explicit_self || fn_node.self_.is_some();

    // flag-off 时 enclosing_class 必为 null（见函数首 LUAU_ASSERT），两路径可共用一支。
    if has_self {
      let self_type: TypeId = if !enclosing_class.is_null() {
        // 分支已判非空；本函数契约担保 `enclosing_class` 指向约束生成期存活的
        // ClassDeclRecord，其 `ty` 是登记时写入的 arena 驻留 TypeId
        // （C++:4378 `enclosingClass->ty`）；alias_ref 收口只读。
        alias_ref(enclosing_class).ty
      } else {
        self.fresh_type(&signature_scope, Polarity::Negative)
      };

      // self_local 只作符号身份键（键/断言用裸指针与记录布局同构），解引用
      // 统一经下方 alias_opt 收口。
      let mut self_local: *mut AstLocal = fn_node.self_.as_ptr();
      if self_local.is_null() && has_explicit_self {
        self_local = fn_node.args[0].as_ptr();
      }

      LUAU_ASSERT!(!self_local.is_null());

      arg_types.push(self_type);
      // 外层 `has_self` 条件（self_ 非空或 has_explicit_self）在逻辑上已保证两条
      // 赋值路径后 self_local 非空（与断言同此不变量）；其指向的 AstLocal 均由
      // parse arena 持有、alias_opt 收口后只读取 name/location 常量字段。
      let self_local_ref = alias_opt(self_local)
        .expect("has_self 不变量保证 self_local 非空（与上方 LUAU_ASSERT 同判据）");
      arg_names.push(Some(FunctionArgument {
        name: self_local_ref.name.as_str_or_empty().to_string(),
        location: self_local_ref.location,
      }));

      // 写窗口经 alias 收口即时物化；键 Symbol::from_local(self_local) 由存活
      // AstLocal 地址构造、值绑定里的 location 读自同一节点（C++:4388
      // `signatureScope->bindings[selfLocal]`）。
      alias(arc_as_mut(&signature_scope)).bindings.insert(
        Symbol::from_local(self_local),
        make_binding(self_type, self_local_ref.location),
      );

      // `self.dfg` 为构造期注入的只读 DataFlowGraph 句柄（C++ NotNull，
      // cpp:4391 `dfg->getDef`），经 dfg_ref 收口；`get_def_local` 只按指针身份查表。
      let def = self.dfg_ref().get_def_local(self_local);
      // 同一 signature_scope 独占写窗口；lvalue_types 键为 DFG 的 DefId
      // 身份指针、值 self_type 为存活 TypeId（C++:4392）。
      *alias(arc_as_mut(&signature_scope))
        .lvalue_types
        .get_or_insert(def) = self_type;
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&signature_scope, def, self_type);
    }

    for (i, local_node) in fn_node.args.iter_nodes().enumerate() {
      let local_ptr = local_node.as_ptr();
      let local = local_node.get();
      if fflag::DebugLuauUserDefinedClasses.get() && has_explicit_self && i == 0 {
        // 对照 C++:4333-4340：class 方法 self 参数禁止标注，但仍要解析一次，
        // 给 TypeChecker2 填充 astResolvedTypes，否则 TC2 查不到直接崩。
        let first_arg = local;
        if let Some(annotation) = alias_opt(first_arg.annotation) {
          self.resolve_type(
            &signature_scope,
            annotation,
            false,
            true,
            Polarity::Negative,
          );
        }
        continue;
      }

      let arg_ty: TypeId = if let Some(annotation) = alias_opt(local.annotation) {
        self.resolve_type(
          &signature_scope,
          annotation,
          false,
          true,
          Polarity::Negative,
        )
      } else if let Some(&ty) = expected_arg_pack.head.get(i) {
        ty
      } else {
        self.fresh_type(&signature_scope, Polarity::Negative)
      };

      arg_types.push(arg_ty);
      let arg_name = local.name.as_str_or_empty();
      arg_names.push(Some(FunctionArgument {
        name: arg_name.to_string(),
        location: local.location,
      }));

      // `self.dfg` 只读句柄不变量同 self 分支（C++:4445 `dfg->getDef(local)`），
      // 经 dfg_ref 收口。
      let def = self.dfg_ref().get_def_local(local_ptr);
      // signature_scope 的写窗口经 alias 收口即时物化；bindings/lvalue_types 的键
      // 均为存活 AstLocal/DefId 身份指针，值 arg_ty 为 arena 驻留 TypeId
      // （C++:4443-4446）。
      let signature_write = alias(arc_as_mut(&signature_scope));
      signature_write.bindings.insert(
        Symbol::from_local(local_ptr),
        make_binding(arg_ty, local.location),
      );
      *signature_write.lvalue_types.get_or_insert(def) = arg_ty;
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&signature_scope, def, arg_ty);
    }

    let mut vararg_pack: TypePackId;

    if fn_node.vararg {
      if let Some(vararg_annotation) = alias_opt(fn_node.vararg_annotation.as_ptr()) {
        vararg_pack = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
          &signature_scope,
          vararg_annotation,
          false,
          true,
          Polarity::Negative,
        );
      } else if expected_arg_pack
        .tail
        .is_some_and(|t| get_type_pack::get::<VariadicTypePack>(t).is_some())
      {
        // Safety: 同 if 支 `is_some_and` 判据蕴含 Some。
        vararg_pack = expected_arg_pack
          .tail
          .expect("is_some_and 同判据蕴含 tail 为 Some（cpp 同位直 deref）");
      } else {
        // Safety: `self.builtin_types.as_ptr()` 为构造期注入的非空单例句柄，`any_type_pack`
        // 是其不可变常量槽（C++:4465 `builtinTypes->anyTypePack`）。
        vararg_pack = self.builtin_types.get().any_type_pack;
      }

      // 对两个存活 Arc scope 的独占字段写（写窗口经 alias 收口即时物化）——
      // 目标已登记进 self.scopes 保活；值 vararg_pack 为存活 TypePackId
      // （C++:4467-4468）。
      alias(arc_as_mut(&signature_scope)).vararg_pack = Some(vararg_pack);
      alias(arc_as_mut(&body_scope)).vararg_pack = Some(vararg_pack);
    } else {
      // Safety: `&mut self.arena.get_mut()` 是构造期 arena 句柄的独占追加，内层读
      // builtin_types 的 any_type 常量槽，两指针非空不变量见函数头（C++ else 分支
      // `varargPack = arena->addTypePack(VariadicTypePack{builtinTypes->anyType, true})`）。
      vararg_pack = {
        self.arena.get_mut().add_type_pack_t(VariadicTypePack {
          ty: { self.builtin_types.get().any_type },
          hidden: true,
        })
      };

      // 同上方 vararg 写入块；此处按 C++ 对应分支显式清空 varargPack
      // （`signatureScope->varargPack = nullopt`）。
      alias(arc_as_mut(&signature_scope)).vararg_pack = None;
      alias(arc_as_mut(&body_scope)).vararg_pack = None;
    }

    LUAU_ASSERT!(!vararg_pack.is_null());

    if !fn_node.self_.is_null() {
      generic_types.push(arg_types[0]);
    }

    let offset: usize = if !fn_node.self_.is_null() { 1 } else { 0 };
    // arg_types 跳过 self 位置后与 AST 参数一一对应
    for (i, ast_arg) in fn_node.args.iter().enumerate() {
      let arg_ty = arg_types[i + offset];
      if ast_arg.annotation.is_null() {
        generic_types.push(arg_ty);
      }
    }

    vararg_pack = follow_type_pack::follow(vararg_pack);
    return_type = follow_type_pack::follow(return_type);
    if fn_node.vararg_annotation.is_null() {
      generic_type_packs.push(vararg_pack);
    }
    if fn_node.return_annotation.is_null() {
      generic_type_packs.push(return_type);
    }

    if let Some(return_annotation) = alias_opt(fn_node.return_annotation.as_ptr()) {
      let annotated_ret_type = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
        &signature_scope,
        return_annotation,
        false,
        true,
        Polarity::Negative,
      );
      // return_type 是 freshTypePack(signatureScope) 新建、此刻仍为
      // FreeTypePack（C++:4516 断言 get<FreeTypePack>），as_mutable_type_pack
      // 仅 const_cast 门面，emplace 一次成 Bound 无并发借用；annotated_ret_type 为
      // 存活 TypeId（C++:4517）。
      unsafe {
        emplace_type_pack(
          as_mutable_type_pack(return_type),
          TypePackVariant::Bound(annotated_ret_type),
        )
      };
    } else if let Some(expected_function) = expected_function {
      // 对照 C++:4432-4434 `else if (expectedFunction) emplaceTypePack<BoundTypePack>(...)`：
      // return_type 仍为独占可写 FreeTypePack，ret_types 从 expected_function
      // arena 节点读出、驻留有效（C++:4521）。
      unsafe {
        emplace_type_pack(
          as_mutable_type_pack(return_type),
          TypePackVariant::Bound(expected_function.ret_types),
        )
      };
    }

    let mut actual_function = function_type::FunctionType::function_type_new(
      // Safety: arena 独占追加窗口——arg_types 里的 TypeId 与 vararg_pack 均为
      // 存活 arena 驻留值，take 转移 Vec 所有权，无其它 arena 借用存活
      // （C++:4526 `arena->addTypePack(std::move(argTypes), varargPack)`）。
      {
        self
          .arena
          .get_mut()
          .add_type_pack_vector_type_id_optional_type_pack_id(
            take(&mut arg_types),
            Some(vararg_pack),
          )
      },
      return_type,
      None,
      if fflag::DebugLuauUserDefinedClasses.get() {
        has_self
      } else {
        !fn_node.self_.is_null()
      },
    );
    actual_function.generics = generic_types;
    actual_function.generic_packs = generic_type_packs;
    actual_function.arg_names = arg_names;

    let defn = FunctionDefinition {
      // Safety: module 由构造期接线恒 Some（见下 Safety 注），cpp NotNull<Module> 同位。
      definition_module_name: Some(
        self
          .module
          .as_ref()
          .expect(
            "ConstraintGenerator 随模块 check 会话构造，module 恒为 Some（cpp NotNull<Module>）",
          )
          .name
          .clone(),
      ),
      definition_location: fn_node.base.base.location,
      vararg_location: if fn_node.vararg {
        Some(fn_node.vararg_location)
      } else {
        None
      },
      original_name_location: original_name
        .unwrap_or(Location::with_length(fn_node.base.base.location.begin, 0)),
    };
    actual_function.definition = Some(defn);

    // Safety: arena 独占追加，actual_function 内所有字段均为存活 TypeId/
    // TypePackId 值拷贝（C++:4539 `arena->addType(std::move(actualFunction))`）。
    let actual_function_type = self.arena.get_mut().add_type(actual_function);
    LUAU_ASSERT!(!actual_function_type.is_null());
    // `self.module` 的 Arc 目标由构造期持有、与会话同寿（expect 已判 Some），
    // ast_types 键 fn_node 为存活 AST 指针、值为上一步 arena 驻留
    // TypeId（C++:4541 `module->astTypes[fn] = actualFunctionType`）；经 alias 收口写。
    let module_ptr = arc_as_mut(self.module.as_ref().expect(
      "ConstraintGenerator 随模块 check 会话构造，module 恒为 Some（cpp NotNull<Module>）",
    ));
    *alias(module_ptr)
      .ast_types
      .get_or_insert(from_ref(fn_node).cast::<AstExpr>()) = actual_function_type;

    if let Some(et) = expected_type_opt
      && get_type::get::<FreeType>(et).is_some()
    {
      bind_free_type(et, actual_function_type);
    }

    // `scope_to_function` 以 Scope 身份裸指针为键（记录布局不改），此处仅取
    // 地址作键、无解引用。
    *self
      .scope_to_function
      .get_or_insert(arc_as_mut(&signature_scope)) = actual_function_type;

    FunctionSignature {
      signature: actual_function_type,
      signature_scope,
      body_scope,
    }
  }
}
