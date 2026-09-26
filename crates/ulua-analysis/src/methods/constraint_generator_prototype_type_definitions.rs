//! Faithful 1:1 port of ConstraintGenerator::prototypeTypeDefinitions
//! (luau/Analysis/src/ConstraintGenerator.cpp lines 820-1274).
use alloc::{boxed::Box, collections::BTreeMap, format, string::String, sync::Arc, vec::Vec};
use core::ptr::{NonNull, eq};

use ulua_ast::{
  records::{
    ast_name::AstName, ast_stat_block::AstStatBlock, ast_stat_class::AstStatClass,
    ast_stat_declare_extern_type::AstStatDeclareExternType, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction, location::Location,
  },
  rtti::ast_node_try_as,
  visit::ast_stat_visit,
};
use ulua_common::{fflag, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{
    arc_as_mut::arc_as_mut,
    as_mutable_type::as_mutable_type_id,
    follow_type::follow,
    get_mutable_type, get_type,
    is_valid_class_metamethod::is_valid_class_metamethod,
    magic_names::{K_ERROR_ID, is_reserved_type_alias_name},
  },
  records::{
    arena_handle::{alias, alias_ref},
    binding::Binding,
    blocked_type::BlockedType,
    built_in_type_function_error::BuiltInTypeFunctionError,
    class_decl_record::ClassDeclRecord,
    constraint_generator::ConstraintGenerator,
    duplicate_type_definition::DuplicateTypeDefinition,
    extern_type::ExternType,
    function_type::FunctionType,
    generic_error::GenericError,
    generic_type::GenericType,
    generic_type_definition::GenericTypeDefinition,
    global_name_collector::GlobalNameCollector,
    klass::Klass,
    obj::Obj,
    property_type::Property,
    scope::Scope,
    scope_registry::{register_scope, resolve_scope},
    symbol::Symbol,
    table_type::TableType,
    type_fun::TypeFun,
    type_function_instance_type::TypeFunctionInstanceType,
    type_level::TypeLevel,
    user_defined_function_data::UserDefinedFunctionData,
  },
  type_aliases::{
    bound_type::BoundType, collections::HashMap, module_name_type::ModuleName, name_type::Name,
    nominal_relation::NominalRelation, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId, type_variant::TypeVariant,
  },
};
/// 类的默认构造/静态属性名（C++ `new`，两处同款）。
const NEW_PROP: &str = "new";

impl ConstraintGenerator {
  /// C++ `prototypeTypeDefinitions(const ScopePtr& scope, AstStatBlock* block)`
  /// （ConstraintGenerator.cpp:820-1274）。`scope` 为调用方持有的存活 `ScopePtr`
  /// 共享借用（cpp `const ScopePtr&` 的 Rust 对应，经 `arc_as_mut`+`alias` 收口
  /// 写入，不再用 `Arc::from_raw`+`ManuallyDrop` 重建）；`block` 为解析器 arena
  /// 存活的 `AstStatBlock` 共享借用。本函数经 `scope` 写入
  /// bindings/lvalue_types/类型绑定表，单线程独占（crate 不变量 1/2）。
  pub fn prototype_type_definitions(&mut self, scope: &ScopePtr, block: &AstStatBlock) {
    // DenseHashMap<Name, Location> typeNameLocations{Name{}};
    let mut type_name_locations: BTreeMap<String, Location> = BTreeMap::new();

    // TODO: Clip these when clipping FFlag::LuauTidyTypePrototyping
    let mut deprecated_alias_definition_locations: BTreeMap<String, Location> = BTreeMap::new();
    let mut deprecated_class_definition_locations: BTreeMap<String, Location> = BTreeMap::new();

    let mut has_type_function = false;
    let mut type_function_env_scope: Option<ScopePtr> = None;

    // globalScope (used in the LuauDisallowRedefiningBuiltinTypes and env passes).
    let global_scope = self
      .global_scope
      .clone()
      .expect("global_scope 由构造期按 cpp ScopePtr 契约接线，恒非空");

    // scope 由调用方持有存活；绑定为引用，消除后续逐字段裸解引用
    let scope_ref: &mut Scope = alias(arc_as_mut(scope));

    // In order to enable mutually-recursive type aliases, we need to
    // populate the type bindings before we actually check any of the
    // alias statements.
    // body 数组（data+size）由解析器构造期写入，此处只整体读拷贝，元素均为活
    // AstStat。对照 C++:928 `for (AstStat* stat : block->body)`。
    let body = &block.body;
    for stat_node in body.iter_nodes() {
      let stat = stat_node.get();

      // if (auto alias = stat->as<AstStatTypeAlias>())
      if let Some(alias_ref) = ast_node_try_as::<AstStatTypeAlias>(stat) {
        let alias_name = alias_ref.name;
        let alias_name_str = ast_name_to_string(alias_name);
        let alias_location = alias_ref.base.base.location;

        if fflag::LuauDisallowRedefiningBuiltinTypes.get()
          && global_scope.builtin_type_names.contains(&alias_name_str)
        {
          self.report_error(
            alias_location,
            TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
              alias_name_str.clone(),
              None,
            )),
          );
          continue;
        }

        if fflag::LuauTidyTypePrototyping.get() {
          // A type alias might have no name if the code is syntactically
          // illegal. We mustn't prepopulate anything in this case.
          if is_reserved_type_alias_name(alias_name.as_bytes()) {
            continue;
          }

          if self.report_dup_tidy(&alias_name_str, alias_location, &type_name_locations) {
            continue;
          }
        } else {
          if self.report_dup_non_tidy(
            scope_ref,
            &alias_name_str,
            alias_location,
            &deprecated_alias_definition_locations,
            true,
          ) {
            continue;
          }

          // A type alias might have no name if the code is syntactically
          // illegal. We mustn't prepopulate anything in this case.
          if is_reserved_type_alias_name(alias_name.as_bytes()) {
            continue;
          }
        }

        // `childScope(alias, scope)`（C++:949）。
        let defn_scope = self.child_scope(&alias_ref.base.base, scope);

        // SAFETY: arena 为构造期（NonNull 实参）写入的活字段指针；可变借用止于
        // 本语句，新句柄产生前无并存别名。对照 C++:951 `arena->addType(BlockedType{})`。
        let initial_type = self.arena.get_mut().add_type(BlockedType::default());
        let mut initial_fun = TypeFun::type_fun_type_id(initial_type);

        /* The boolean toggle `addTypes` decides whether or not to introduce the generic
        type/pack param into the privateType/Pack bindings. See C++ comment. */
        let generics = alias_ref.generics;
        initial_fun.type_params.extend(
          self
            .create_generics(&defn_scope, generics, true, false)
            .into_iter()
            .map(|(_, r#gen)| r#gen),
        );

        let generic_packs = alias_ref.generic_packs;
        initial_fun.type_pack_params.extend(
          self
            .create_generic_packs(&defn_scope, generic_packs, true, false)
            .into_iter()
            .map(|(_, gen_pack)| gen_pack),
        );
        initial_fun.definition_location = Some(alias_location);

        if alias_ref.exported {
          scope_ref
            .exported_type_bindings
            .insert(alias_name_str.clone(), initial_fun);
        } else {
          scope_ref
            .private_type_bindings
            .insert(alias_name_str.clone(), initial_fun);
        }

        *self
          .ast_type_alias_defining_scopes
          .get_or_insert(alias_ref as *const AstStatTypeAlias) = Some(defn_scope.clone());
        if fflag::LuauTidyTypePrototyping.get() {
          type_name_locations.insert(alias_name_str.clone(), alias_location);
        } else {
          deprecated_alias_definition_locations.insert(alias_name_str.clone(), alias_location);
        }
        continue;
      }

      // else if (auto function = stat->as<AstStatTypeFunction>())
      if let Some(function_ref) = ast_node_try_as::<AstStatTypeFunction>(stat) {
        has_type_function = true;
        let function_name = function_ref.name;
        let function_name_str = ast_name_to_string(function_name);
        let function_location = function_ref.base.base.location;

        // If a type function w/ same name has already been defined, error for having duplicates
        if fflag::LuauTidyTypePrototyping.get() {
          if self.report_dup_tidy(&function_name_str, function_location, &type_name_locations) {
            continue;
          }
        } else {
          if self.report_dup_non_tidy(
            scope_ref,
            &function_name_str,
            function_location,
            &deprecated_alias_definition_locations,
            true,
          ) {
            continue;
          }
        }

        // Create TypeFunctionInstanceType
        let body_fn = unsafe { &*function_ref.body };
        let args = &body_fn.args;
        let args_size = args.len();

        let mut type_params: Vec<TypeId> = Vec::with_capacity(args_size);

        let mut quantified_type_params: Vec<GenericTypeDefinition> = Vec::with_capacity(args_size);

        // j 是类型参数位置序号（用于 T{j} 命名），语义量而非容器下标，保留索引遍历。
        for j in 0..args_size {
          let name: String = format!("T{j}");
          // SAFETY: arena 字段活（构造期写入），可变借用止于语句边界；name 为
          // 局部串，GenericType.name 是 Owned String，add_type 内已深拷贝。
          // 对照 C++:1002 `arena->addType(GenericType{name, Polarity::Unknown})`。
          let ty = {
            self
              .arena
              .get_mut()
              .add_type(GenericType::generic_type_name_polarity(
                &name,
                Polarity::Unknown,
              ))
          };
          type_params.push(ty);
          quantified_type_params.push(GenericTypeDefinition {
            ty,
            default_value: None,
          });
        }

        if fflag::LuauTypeFunctionStructuredErrors.get() {
          // SAFETY: type_function_runtime 为构造期（NonNull 实参）写入的活字段；
          // node 是刚判型的活 AstStatTypeFunction，runtime 仅登记其指针身份、
          // 存活期由解析器 arena 界定。对照 C++:1011 `registerFunction(function)`。
          if let Some(error) = unsafe {
            self
              .type_function_runtime
              .get_mut()
              .register_function(stat_node.as_ptr().cast::<AstStatTypeFunction>())
          } {
            self.report_error(
              function_location,
              TypeErrorData::BuiltInTypeFunctionError(BuiltInTypeFunctionError { error }),
            );
          }
        } else {
          // SAFETY: 同上——runtime 字段活、node 为活类型函数节点，仅按身份登记。
          // 对照 C++:1016 `registerFunction_DEPRECATED(function)`。
          if let Some(error) = unsafe {
            self
              .type_function_runtime
              .get_mut()
              .register_function_deprecated(stat_node.as_ptr().cast::<AstStatTypeFunction>())
          } {
            self.report_error(
              function_location,
              TypeErrorData::GenericError(GenericError::new(error)),
            );
          }
        }

        let mut udtf_data =
          UserDefinedFunctionData::new(Arc::downgrade(self.module.as_ref().expect(
            "cpp ConstraintGenerator 构造器直 deref module 填 sharedModuleName，接线后恒为 Some",
          )));
        // udtfData.owner = module; (set above) ; udtfData.definition = function;
        udtf_data.definition = stat_node.as_ptr().cast::<AstStatTypeFunction>();

        // SAFETY: builtin_types 为构造期写入的活 BuiltinTypes 指针，user_func 是
        // 其内嵌数据成员（生命期覆盖整个分析会话），此处仅对活对象取共享引用再
        // NonNull 化，作身份句柄存入 arena 类型。对照 C++:1026
        // `NotNull{&builtinTypes->typeFunctions->userFunc}`。
        let user_func = NonNull::from({ &self.builtin_types.get().type_functions.user_func });
        // SAFETY: arena 字段活，可变借用止于语句边界；实参（type_params 等）均
        // 为已构造的 Owned 值，user_func 的有效性见上一处论证。对照 C++:1025-1027。
        let type_function_ty = {
          self.arena.get_mut().add_type(TypeFunctionInstanceType::new(
            user_func,
            type_params,
            Vec::new(),
            Some(function_name),
            udtf_data,
          ))
        };

        let mut type_function =
          TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
            quantified_type_params,
            type_function_ty,
            None,
          );

        type_function.definition_location = Some(function_location);

        // Set type bindings and definition locations for this user-defined type function
        if function_ref.exported {
          scope_ref
            .exported_type_bindings
            .insert(function_name_str.clone(), type_function);
        } else {
          scope_ref
            .private_type_bindings
            .insert(function_name_str.clone(), type_function);
        }

        if fflag::LuauTidyTypePrototyping.get() {
          type_name_locations.insert(function_name_str.clone(), function_location);
        } else {
          deprecated_alias_definition_locations
            .insert(function_name_str.clone(), function_location);
        }
        continue;
      }

      // else if (auto classDeclaration = stat->as<AstStatDeclareExternType>())
      if let Some(class_decl_ref) = ast_node_try_as::<AstStatDeclareExternType>(stat) {
        let class_decl_name = class_decl_ref.name;
        let class_decl_name_str = ast_name_to_string(class_decl_name);
        let class_decl_location = class_decl_ref.base.base.location;

        if fflag::LuauTidyTypePrototyping.get() {
          // A class might have no name if the code is syntactically illegal.
          if ast_name_is(class_decl_name, K_ERROR_ID) {
            continue;
          }

          if self.report_dup_tidy(
            &class_decl_name_str,
            class_decl_location,
            &type_name_locations,
          ) {
            continue;
          }
        } else {
          if self.report_dup_non_tidy(
            scope_ref,
            &class_decl_name_str,
            class_decl_location,
            &deprecated_class_definition_locations,
            false,
          ) {
            continue;
          }

          // A class might have no name if the code is syntactically illegal.
          if ast_name_is(class_decl_name, K_ERROR_ID) {
            continue;
          }
        }

        // node_ref 为当前判过型的活 AstStat（extern-type 声明），scope 为调用方
        // 持有的活 ScopePtr；对照 C++:1054 `childScope(classDeclaration, scope)`。
        let defn_scope = self.child_scope(&class_decl_ref.base.base, scope);
        let _ = defn_scope; // mirrors C++ (defnScope is created but unused beyond this)

        // SAFETY: arena 字段活、借用止于语句边界。对照 C++:1056
        // `arena->addType(BlockedType{})`。
        let initial_type = self.arena.get_mut().add_type(BlockedType::default());
        let mut initial_fun = TypeFun::type_fun_type_id(initial_type);
        initial_fun.definition_location = Some(class_decl_location);
        scope_ref
          .exported_type_bindings
          .insert(class_decl_name_str.clone(), initial_fun);

        if fflag::LuauTidyTypePrototyping.get() {
          type_name_locations.insert(class_decl_name_str.clone(), class_decl_location);
        } else {
          deprecated_class_definition_locations
            .insert(class_decl_name_str.clone(), class_decl_location);
        }
        continue;
      }

      // else if (auto classDecl = stat->as<AstStatClass>())
      if let Some(class_decl_ref) = ast_node_try_as::<AstStatClass>(stat) {
        debug_assert!(fflag::DebugLuauUserDefinedClasses.get());

        let name_local = class_decl_ref.name;
        // name_local 取自上方判过型的活 AstStatClass 节点，解析器保证类
        // 声明的名字表达式非空且同 arena 保活；经 alias_ref 收口。对照 C++:1067
        // `classDecl->name->…` 的隐式解引用链。
        let name_local_ref = alias_ref(name_local);
        let decl_name: Name = ast_name_to_string(name_local_ref.name);
        // dfg 为构造期写入的活 DataFlowGraph，get_def_for_local 只读；
        // name_local 非空活节点，仅作身份键。对照 C++:1068
        // `dfg->getDef(classDecl->name)`。
        let the_def = self.dfg_ref().get_def_for_local(name_local);
        let class_decl_location = class_decl_ref.base.base.location;
        let name_local_location = name_local_ref.location;

        // SAFETY: builtin_types 字段活（构造期 NonNull 写入），error_type 只读
        // 句柄。对照 C++:1073 `builtinTypes->errorType`。
        let error_type = self.builtin_types.get().error_type;

        // 两路径仅查重表不同，错误处理完全一致（对照 C++ tidy/non-tidy 分支）。
        let decl_locations = if fflag::LuauTidyTypePrototyping.get() {
          &mut type_name_locations
        } else {
          &mut deprecated_class_definition_locations
        };
        if let Some(loc) = decl_locations.get(&decl_name).copied() {
          self.report_error(
            class_decl_location,
            TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
              decl_name.clone(),
              Some(loc),
            )),
          );
          scope_ref.bindings.insert(
            Symbol::from_global(name_local_ref.name),
            make_binding(error_type, class_decl_location),
          );
          *scope_ref.lvalue_types.get_or_insert(the_def) = error_type;
          continue;
        }
        decl_locations.insert(decl_name.clone(), class_decl_location);

        // SAFETY: arena 字段活，借用止于语句边界；the_ty 尚未进入任何容器前
        // 只此一处构造。对照 C++:1079 `arena->addType(BlockedType{})`。
        let the_ty = self.arena.get_mut().add_type(BlockedType::default());
        scope_ref.bindings.insert(
          Symbol::from_global(name_local_ref.name),
          make_binding(the_ty, name_local_location),
        );
        *scope_ref.lvalue_types.get_or_insert(the_def) = the_ty;

        // Objects are ExternTypes (see C++ comment block).
        let mut static_props: BTreeMap<Name, Property> = BTreeMap::new();
        let mut props: BTreeMap<Name, Property> = BTreeMap::new();
        let mut instance_metatable_props: BTreeMap<Name, Property> = BTreeMap::new();
        let mut default_ctor_props: BTreeMap<Name, Property> = BTreeMap::new();
        let mut member_types: DenseHashMap<AstName, TypeId> = DenseHashMap::default();
        let mut has_explicit_constructor = false;
        let mut new_blocked_ty_opt = None;

        let members = class_decl_ref.members.as_slice();
        for member in members {
          if let Some(class_prop) = member.get_if_0() {
            // AstClassProperty branch
            if member_types.contains(&class_prop.name) {
              continue;
            }

            // 先挂 BlockedType 占位：属性标注可能引用尚未定义的别名或
            // typeof，真正的解析推迟到 visit(AstStatClass)。
            // SAFETY: arena 为构造期（NonNull 实参）写入的活字段，可变借用止于
            // 语句边界。对照 C++:1104 `try_insert(classProp.name, arena->addType(BlockedType{}))`。
            let blocked = self.arena.get_mut().add_type(BlockedType::default());
            member_types.try_insert(class_prop.name, blocked);

            let prop_name = ast_name_to_string(class_prop.name);
            let mut p = Property::rw_type_id(blocked);
            p.location = Some(class_prop.name_location);
            props.insert(prop_name.clone(), p);

            // 默认构造函数接收只读表：构造时不会回写传入的表。
            default_ctor_props.insert(prop_name, Property::readonly(blocked));
          } else if let Some(method) = member.get_if_1() {
            // AstClassMethod branch
            if member_types.contains(&method.function_name) {
              continue;
            }

            // SAFETY: 同上——arena 字段活、借用止于语句边界。对照 C++:1123
            // `try_insert(method.functionName, arena->addType(BlockedType{}))`。
            let blocked = self.arena.get_mut().add_type(BlockedType::default());
            member_types.try_insert(method.function_name, blocked);

            let method_name = ast_name_to_string(method.function_name);
            let mut prop = Property::readonly(blocked);
            prop.location = Some(method.name_location);

            let fn_ptr = method.function;
            // fn_ptr 取自判过型的活 AstStatClass 的 method 成员，解析器
            // 填入非空 AstExprFunction；args（data+size）构造期写入，只整体读拷贝。
            // 对照 C++:1127 `method.function->args` 的隐式解引用。
            let fn_args = &alias_ref(fn_ptr).args;
            // `&&` 短路保证 size>=1 才索引；切片元素是解析器写入的活
            // AstLocal 指针，此处仅读其 name 字段。对照 C++:1127
            // `method.function->args.data[0]->name != "self"`。
            let first_is_self = fn_args
              .get(0)
              .is_some_and(|arg| ast_name_is(arg.name, b"self"));
            if !first_is_self {
              static_props.insert(method_name.clone(), prop.clone());
            }
            // The parser will report an error for classes that define disallowed
            // metamethods. (See C++ comment.)
            if is_valid_class_metamethod(&method_name) {
              instance_metatable_props.insert(method_name.clone(), prop.clone());
            } else {
              props.insert(method_name.clone(), prop.clone());
            }

            if method_name == "__init" {
              has_explicit_constructor = true;
              // SAFETY: arena 字段活，借用止于语句边界。对照 C++:1140
              // `TypeId newBlockedTy = arena->addType(BlockedType{})`。
              let new_blocked_ty = self.arena.get_mut().add_type(BlockedType::default());
              new_blocked_ty_opt = Some(new_blocked_ty);
              static_props.insert(Name::from(NEW_PROP), Property::readonly(new_blocked_ty));
            }
          }
        }

        // arena 字段活、借用止于本表达式；instance_metatable_props 以
        // 共享引用传入并在语句末结束 borrow；scope 的 Arc 由调用方持有存活，
        // arc_as_mut 句柄仅按 C++ `scope.get()` 同款语义作身份句柄存入 TableType。
        // 对照 C++:1150。
        let instance_metatable = {
          self.arena.get_mut().add_type(
            TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
              &instance_metatable_props,
              None,
              TypeLevel::default(),
              arc_as_mut(scope),
              TableState::Sealed,
            ),
          )
        };

        // SAFETY: builtin_types 为构造期写入的活字段指针，object_type 只读句柄。
        // 对照 C++:1154-1155 `builtinTypes->objectType`。
        let object_type = self.builtin_types.get().object_type;
        // SAFETY: 同上，class_type 只读句柄。对照 C++:1168 `builtinTypes->classType`。
        let class_type = self.builtin_types.get().class_type;
        let module_name = self
          .module
          .as_ref()
          .expect(
            "cpp ConstraintGenerator 构造器直 deref module 填 sharedModuleName，接线后恒为 Some",
          )
          .name
          .clone();

        // SAFETY: arena 字段活、可变借用止于本表达式；props 已被 make_extern_type
        // 接管为 Owned 数据，object_type/instance_metatable 均为 arena 句柄值。
        // 对照 C++:1153-1156 的 ExternType classInstanceTy。
        let class_instance_ty = {
          self.arena.get_mut().add_type(make_extern_type(
            decl_name.clone(),
            props,
            Some(object_type),
            Some(instance_metatable),
            module_name.clone(),
            class_decl_location,
          ))
        };

        if !has_explicit_constructor {
          // SAFETY: arena 字段活；scope 的 Arc 由调用方持有，arc_as_mut 句柄仅作
          // TableType 父句柄（同 instance_metatable 处论证），default_ctor_props
          // 借用止于语句边界。对照 C++:1162。
          let ctor_arg_ty = {
            self.arena.get_mut().add_type(
              TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                &default_ctor_props,
                None,
                TypeLevel::default(),
                arc_as_mut(scope),
                TableState::Sealed,
              ),
            )
          };
          // SAFETY: arena 字段活；块内两次 add_type_pack 走 self 的安全方法，仅
          // 末尾 add_type 派生 arena 的 &mut，各借用顺序发生、不并生。对照 C++:1163
          // `arena->addType(FunctionType{arena->addTypePack(...), ...})`。
          let ctor_ty = {
            let arg_pack = self.add_type_pack(alloc::vec![ctor_arg_ty], None);
            let ret_pack = self.add_type_pack(alloc::vec![class_instance_ty], None);
            self
              .arena
              .get_mut()
              .add_type(FunctionType::function_type_new(
                arg_pack, ret_pack, None, false,
              ))
          };
          static_props.insert(Name::from(NEW_PROP), Property::readonly(ctor_ty));
        }

        // SAFETY: arena 字段活、借用止于本表达式。对照 C++:1166-1168
        // `arena->addType(ExternType{declName, staticProps, builtinTypes->classType, ...})`。
        let extern_ty = {
          self.arena.get_mut().add_type(make_extern_type(
            decl_name.clone(),
            static_props,
            Some(class_type),
            None,
            module_name.clone(),
            class_decl_location,
          ))
        };

        // Setup a bidirectional relationship between classes and objects
        // 对照 C++:1173-1174：extern_ty / class_instance_ty 均为刚分配的 ExternType，必命中
        get_mutable_type::get_mutable::<ExternType>(extern_ty)
          .expect("上方注释契约：刚分配的 ExternType，get_mutable 必命中")
          .relation = Some(NominalRelation::V0(Obj {
          ty: class_instance_ty,
        }));
        get_mutable_type::get_mutable::<ExternType>(class_instance_ty)
          .expect("上方注释契约：刚分配的 ExternType，get_mutable 必命中")
          .relation = Some(NominalRelation::V1(Klass { ty: extern_ty }));

        debug_assert!(get_type::get::<BoundType>(the_ty).is_none());
        // 对照 C++:1182-1183 `if (auto bt = get<BlockedType>(theTy); bt && bt->getOwner() == nullptr)`
        let bt = get_type::get::<BlockedType>(the_ty);
        debug_assert!(bt.is_some());
        let bt = bt.expect("上一行 debug_assert 同判据：the_ty 由本函数以 BlockedType 分配");
        debug_assert!(bt.get_owner().is_null());

        // emplaceType<BoundType>(asMutable(theTy), externTy)
        // the_ty 由本函数分配，上方 debug_assert 已确认其为无 owner 的
        // BlockedType；as_mutable_type_id 派生的可变句柄经 alias 收口，对照 C++:1180。
        alias(as_mutable_type_id(the_ty)).ty = TypeVariant::Bound(extern_ty);

        let export_name = ast_name_to_string(name_local_ref.name);
        if class_decl_ref.exported {
          scope_ref.exported_type_bindings.insert(
            export_name,
            TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
              Vec::new(),
              class_instance_ty,
              Some(class_decl_location),
            ),
          );
        } else {
          scope_ref.private_type_bindings.insert(
            export_name,
            TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
              Vec::new(),
              class_instance_ty,
              Some(class_decl_location),
            ),
          );
        }

        *self.class_decl_records.get_or_insert(name_local) = ClassDeclRecord {
          ty: class_instance_ty,
          member_types,
          new_blocked_ty: new_blocked_ty_opt,
        };
        continue;
      }
    }

    if has_type_function {
      // typeFunctionEnvScope = std::make_shared<Scope>(typeFunctionRuntime->rootScope);
      // SAFETY: type_function_runtime 为构造期（NonNull 实参）写入的活字段；
      // root_scope 是其内持有的 ScopePtr（Arc 有效对象），clone 仅增计数。
      // 对照 C++:1192 `std::make_shared<Scope>(typeFunctionRuntime->rootScope)`。
      let root_scope = self.type_function_runtime.get_mut().root_scope.clone();
      let env_scope: ScopePtr = Arc::new(Scope::new(&root_scope, 0));
      register_scope(&env_scope);
      type_function_env_scope = Some(env_scope);
    }

    let mut created_type_functions: Vec<*mut TypeFunctionInstanceType> = Vec::new();
    // DenseHashMap<AstStatTypeFunction*, const TypeFunctionInstanceType*> referencedTypeFunctions{nullptr};
    let mut referenced_type_functions: BTreeMap<
      *mut AstStatTypeFunction,
      *const TypeFunctionInstanceType,
    > = BTreeMap::new();

    // Additional pass for user-defined type functions to fill in their environments completely
    for stat_node in body.iter_nodes() {
      let stat = stat_node.get();
      let Some(function_ref) = ast_node_try_as::<AstStatTypeFunction>(stat) else {
        continue;
      };
      let function = function_ref as *const AstStatTypeFunction;
      let function_name = function_ref.name;
      let function_name_str = ast_name_to_string(function_name);
      let function_location = function_ref.base.base.location;

      let env_scope = type_function_env_scope.clone().expect(
        "第一遍扫同一 body：存在 type function 语句时 has_type_function 已置位并建好 env scope",
      );
      // 从 Arc 指针（而非 &Scope 共享引用）派生可变指针：后续会经它写入 bindings，
      // 经共享引用转发写入属未定义行为；与 global_scope_raw/it_raw 的 Arc::as_ptr 惯用法一致。
      let env_scope_raw = arc_as_mut(&env_scope);

      // Similar to global pre-population, create a binding for each type function in the scope upfront
      // SAFETY: arena 字段活、可变借用止于本语句；新 BlockedType 句柄仅随后
      // 经 make_binding 进入 env scope bindings。对照 C++:1203。
      let bt = self.arena.get_mut().add_type(BlockedType::default());
      scope_insert_binding(
        alias(env_scope_raw),
        Symbol::from_global(function_name),
        make_binding(bt, function_location),
      );
      *self
        .ast_type_function_environment_scopes
        .get_or_insert(function) = Some(env_scope.clone());

      // Find the type function we have already created
      // 对照 C++:1215-1225 `TypeFunctionInstanceType* mainTypeFun = nullptr; ...`
      let mut main_type_fun: Option<&'static mut TypeFunctionInstanceType> = None;

      if let Some(it) = scope_ref.private_type_bindings.get(&function_name_str) {
        main_type_fun = get_mutable_type::get_mutable::<TypeFunctionInstanceType>(it.r#type);
      }

      if main_type_fun.is_none()
        && let Some(it) = scope_ref.exported_type_bindings.get(&function_name_str)
      {
        main_type_fun = get_mutable_type::get_mutable::<TypeFunctionInstanceType>(it.r#type);
      }

      // Fill it with all visible type functions and referenced type aliases
      if let Some(mtf) = main_type_fun {
        created_type_functions.push(mtf as *mut TypeFunctionInstanceType);

        let mut global_name_collector = GlobalNameCollector::new();
        // SAFETY: stat 为解析器 arena 持有的活语句，且本循环内 node_ref/function_ref
        // 均为只读借用、未与 visit 的可变访问并存；collector 仅收集全局名集合。
        // 对照 C++:1225 `stat->visit(&globalNameCollector)`。
        unsafe { ast_stat_visit(stat_node.as_ptr(), &mut global_name_collector) };

        // Go up the scopes to register type functions and aliases, but without reaching
        // into the global scope.
        let mut level: usize = 0;
        // 句柄化上溯：parent 为 ScopeId，经 resolve_scope 只读还原（scope_registry
        // 契约 1），与全局 scope 的终止判据由 Arc::ptr_eq 改为同址指针比较。
        let mut curr: Option<&Scope> = Some(&**scope);
        while let Some(curr_ref) = curr {
          if eq(curr_ref, global_scope.as_ref()) {
            break;
          }
          // Scope 由注册表保活，本迭代内只读。
          self.proto_register_bindings(
            &curr_ref.private_type_bindings,
            mtf,
            env_scope_raw,
            level,
            &global_name_collector,
            &mut referenced_type_functions,
          );
          self.proto_register_bindings(
            &curr_ref.exported_type_bindings,
            mtf,
            env_scope_raw,
            level,
            &global_name_collector,
            &mut referenced_type_functions,
          );

          level += 1;
          curr = curr_ref.parent.and_then(resolve_scope);
        }
      }
    }

    // Finally, we need to include aliases from functions we might call
    for &type_ptr in &created_type_functions {
      // type_ptr 为刚创建的类型实例（arena 持有），本迭代内只读；
      // 写入 environment_alias 前 type_ref 已结束存活期（NLL）。
      let type_ref = alias_ref(type_ptr);
      // Go over all functions in our environment.
      let environment_function_pairs: Vec<(Name, (*mut AstStatTypeFunction, usize))> = type_ref
        .user_func_data
        .environment_function
        .iter()
        .map(|(n, v)| (n.clone(), *v))
        .collect();

      for (_target_func_name, definition_and_level) in environment_function_pairs {
        if let Some(it) = referenced_type_functions.get(&definition_and_level.0) {
          // target 为 referenced_type_functions 记录的有效实例指针。
          let target_ref = alias_ref(*it);
          let target_alias_pairs: Vec<(Name, (*mut TypeFun, usize))> = target_ref
            .user_func_data
            .environment_alias
            .iter()
            .map(|(n, v)| (n.clone(), *v))
            .collect();
          for (alias_name, type_and_level) in target_alias_pairs {
            if type_ref
              .user_func_data
              .environment_alias
              .find(&alias_name)
              .is_some()
            {
              continue;
            }
            // Combine definition levels because we are viewing target function
            // aliases from the perspective of the target function.
            // type_ref 存活期已结束（上方 find 为 last use），经 alias 收口写入；
            // environment_alias 为该实例独占容器（C++ 同款）。
            *alias(type_ptr)
              .user_func_data
              .environment_alias
              .get_or_insert(alias_name.clone()) =
              (type_and_level.0, type_and_level.1 + definition_and_level.1);
          }
        }
      }
    }
  }

  /// 把一层 scope 的类型绑定登记进求值环境（private/exported 两处同款）。
  /// 先收集克隆再写入，避免 self 与 curr 的借用别名（C++ 同款两阶段）。
  fn proto_register_bindings(
    &mut self,
    bindings: &HashMap<Name, TypeFun>,
    main_type_fun: &mut TypeFunctionInstanceType,
    env_scope_raw: *mut Scope,
    level: usize,
    global_name_collector: &GlobalNameCollector,
    referenced_type_functions: &mut BTreeMap<
      *mut AstStatTypeFunction,
      *const TypeFunctionInstanceType,
    >,
  ) {
    let pairs: Vec<(Name, TypeFun)> = bindings
      .iter()
      .map(|(n, tf)| (n.clone(), tf.clone()))
      .collect();
    for (name, tf) in pairs {
      self.proto_add_to_environment(
        main_type_fun,
        env_scope_raw,
        &name,
        tf,
        level,
        global_name_collector,
        referenced_type_functions,
      );
    }
  }

  /// Port of the local `addToEnvironment` lambda (L1196-1234).
  fn proto_add_to_environment(
    &mut self,
    main_type_fun: &mut TypeFunctionInstanceType,
    env_scope_raw: *mut Scope,
    name: &Name,
    tf: TypeFun,
    level: usize,
    global_name_collector: &GlobalNameCollector,
    referenced_type_functions: &mut BTreeMap<
      *mut AstStatTypeFunction,
      *const TypeFunctionInstanceType,
    >,
  ) {
    let followed = follow(tf.r#type);

    // 对照 C++:1231 `if (auto ty = get<TypeFunctionInstanceType>(follow(tf.type)); ty && ty->userFuncData.definition)`
    if let Some(ty) = get_type::get::<TypeFunctionInstanceType>(followed)
      && !ty.user_func_data.definition.is_null()
    {
      let definition = ty.user_func_data.definition;
      let user_func_data = &mut main_type_fun.user_func_data;
      if user_func_data.environment_function.find(name).is_some() {
        return;
      }

      *user_func_data
        .environment_function
        .get_or_insert(name.clone()) = (definition, level);

      referenced_type_functions.insert(definition, ty as *const TypeFunctionInstanceType);

      if let Some(it) = self
        .ast_type_function_environment_scopes
        .find(&(definition as *const AstStatTypeFunction))
        .and_then(|o| o.clone())
        && let Some(existing) = it.linear_search_for_binding(name, false)
      {
        // definition 为 arena 持有的有效节点指针（上方判非空）。
        let def_ref = alias_ref(definition);
        scope_insert_binding(
          alias(env_scope_raw),
          Symbol::from_global(def_ref.name),
          make_binding(existing.type_id, def_ref.base.base.location),
        );
      }
    } else if get_type::get::<TypeFunctionInstanceType>(followed).is_none() {
      let user_func_data = &mut main_type_fun.user_func_data;
      if user_func_data.environment_alias.find(name).is_some() {
        return;
      }

      // AstName astName = module->names->get(name.c_str());
      let module = self
        .module
        .as_ref()
        .expect(
          "cpp ConstraintGenerator 构造器直 deref module 填 sharedModuleName，接线后恒为 Some",
        )
        .clone();
      let ast_name = module
        .names
        .as_ref()
        .expect("Module.names 在解析构造期接线，check 期恒非空（cpp 同名直 deref）")
        .get_str(name.as_str());

      // Only register globals that we have detected to be used
      if global_name_collector.names.find(&ast_name).is_none() {
        return;
      }

      // Function evaluation environment needs a stable reference to the alias.
      // module->typeFunctionAliases.push_back(make_unique<TypeFun>(tf));
      let def_loc = tf.definition_location.unwrap_or_default();
      let module_raw = arc_as_mut(&module);
      let back_ptr: *mut TypeFun = {
        // module 由 Arc 持有，非空；push 后取尾指针（push 可能重定位）。
        let m = alias(module_raw);
        m.type_function_aliases.push(Box::new(tf));
        m.type_function_aliases
          .last_mut()
          .map(|b| b.as_mut() as *mut TypeFun)
          .expect("上一行刚 push，last_mut 必为 Some")
      };

      *user_func_data.environment_alias.get_or_insert(name.clone()) = (back_ptr, level);

      // TODO: create a specific type alias type
      // SAFETY: builtin_types 为构造期写入的活字段指针，any_type 是只读句柄。
      // 对照 C++:1266 `Binding{builtinTypes->anyType, ...}`。
      let any_type = self.builtin_types.get().any_type;
      scope_insert_binding(
        alias(env_scope_raw),
        Symbol::from_global(ast_name),
        make_binding(any_type, def_loc),
      );
    }
  }

  /// tidy 路径的重复定义检测（alias/function/class_declaration 三处同款）。
  /// 返回 true 表示已上报，调用方须 continue。
  fn report_dup_tidy(
    &mut self,
    name_str: &Name,
    location: Location,
    locations: &BTreeMap<String, Location>,
  ) -> bool {
    if let Some(loc) = locations.get(name_str) {
      self.report_error(
        location,
        TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
          name_str.clone(),
          Some(*loc),
        )),
      );
      return true;
    }
    false
  }

  /// 非 tidy 路径的重复定义检测（alias/function/class_declaration 三处同款）。
  /// 返回 true 表示已上报，调用方须 continue。
  fn report_dup_non_tidy(
    &mut self,
    scope_ref: &Scope,
    name_str: &Name,
    location: Location,
    locations: &BTreeMap<String, Location>,
    check_private: bool,
  ) -> bool {
    if scope_ref.exported_type_bindings.contains_key(name_str)
      || (check_private && scope_ref.private_type_bindings.contains_key(name_str))
    {
      let it = locations.get(name_str);
      debug_assert!(it.is_some());
      let prev = it.copied();
      self.report_error(
        location,
        TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
          name_str.clone(),
          prev,
        )),
      );
      return true;
    }
    false
  }
}

/// Build a `Binding{ty, loc}`（Binding 无构造函数；prototype/visit 共用）。
pub(crate) fn make_binding(type_id: TypeId, location: Location) -> Binding {
  Binding {
    type_id,
    location,
    deprecated: false,
    deprecated_suggestion: String::new(),
    documentation_symbol: None,
  }
}

/// Build an `ExternType{name, props, parent, metatable, Tags{}, nullptr, module_name, location}`.
fn make_extern_type(
  name: Name,
  props: BTreeMap<Name, Property>,
  parent: Option<TypeId>,
  metatable: Option<TypeId>,
  definition_module_name: ModuleName,
  definition_location: Location,
) -> ExternType {
  ExternType {
    name,
    props,
    parent,
    metatable,
    tags: Vec::new(),
    user_data: None,
    definition_module_name,
    definition_location: Some(definition_location),
    indexer: None,
    relation: None,
  }
}

/// AstName -> owned String (for map keys / names).
fn ast_name_to_string(name: AstName) -> String {
  name.as_str_or_empty().to_string()
}

/// AstName == literal (byte comparison of the C string).
fn ast_name_is(name: AstName, literal: &[u8]) -> bool {
  // as_bytes 空名 → 空切片；literal 均为非空常量，语义与原 null 判 false 一致。
  name.as_bytes() == literal
}

/// 向 scope 写入单条绑定（prototype/visit 共用）。
pub(crate) fn scope_insert_binding(scope: &mut Scope, sym: Symbol, binding: Binding) {
  scope.bindings.insert(sym, binding);
}
