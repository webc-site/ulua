use alloc::sync::Arc;
use core::ptr::from_ref;

use ulua_ast::records::{
  ast_stat_declare_extern_type::AstStatDeclareExternType, ast_stat_type_alias::AstStatTypeAlias,
};
use ulua_common::{functions::format::format, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type, get_mutable_type, get_type, magic_names::is_reserved_type_alias_name,
    shared_mut::shared_mut,
  },
  records::{
    arena_handle::{alias_arc, alias_ref},
    duplicate_type_definition::DuplicateTypeDefinition,
    extern_type::ExternType,
    free_type::FreeType,
    generic_error::GenericError,
    table_type::TableType,
    type_checker::TypeChecker,
    type_error::TypeError,
    type_fun::TypeFun,
    unknown_symbol::{Context, UnknownSymbol},
  },
  type_aliases::{
    name_type::Name, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData, type_id::TypeId,
  },
};

impl TypeChecker {
  pub fn prototype_scope_ptr_ast_stat_type_alias_i32(
    &mut self,
    scope: ScopePtr,
    typealias: &AstStatTypeAlias,
    sub_level: i32,
  ) {
    // AstName 由词法器 intern，必为合法 ASCII/UTF-8。
    // If the alias is missing a name, we can't do anything with it.  Ignore it.
    // Also, typeof is not a valid type alias name.  We will report an error for
    // this in check()
    if is_reserved_type_alias_name(typealias.name.as_bytes()) {
      return;
    }

    let name: Name = typealias.name.as_str_or_empty().to_string();

    let mut binding: Option<TypeFun> = None;
    if let Some(it) = scope.exported_type_bindings.get(&name) {
      binding = Some(it.clone());
    } else if let Some(it) = scope.private_type_bindings.get(&name) {
      binding = Some(it.clone());
    }

    if binding.is_some() {
      let location = *scope
        .type_alias_locations
        .get(&name)
        .expect("cpp typeAliasLocations.find(name)->second：类型别名绑定与位置信息成对登记");
      self.report_error_type_error(&TypeError::type_error_location_type_error_data(
        typealias.base.base.location,
        TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(
          name.clone(),
          Some(location),
        )),
      ));

      self
        .duplicate_type_aliases
        .insert((typealias.exported, name));
    } else {
      let global_scope = alias_ref(self.global_scope);
      let is_builtin = global_scope.builtin_type_names.contains(&name);
      if is_builtin {
        self.report_error_location_type_error_data(
          &typealias.base.base.location,
          TypeErrorData::DuplicateTypeDefinition(DuplicateTypeDefinition::new(name.clone(), None)),
        );
        self
          .duplicate_type_aliases
          .insert((typealias.exported, name));
      } else {
        // level 定制（cpp `aliasScope->level = scope->level.incr()` + `subLevel`）
        // 在 Arc 注册前完成，不再经裸指针回写已共享的 Scope。
        let mut level = scope.level.incr();
        level.sub_level = sub_level;
        let alias_scope = self.child_scope_with_level(&scope, &typealias.base.base.location, level);

        let defs = self.create_generic_types(
          &alias_scope,
          Some(scope.level),
          &typealias.base.base,
          &typealias.generics,
          &typealias.generic_packs,
          true,
        );

        let ty = self.fresh_type_scope_ptr(alias_scope.clone());
        // C++: FreeType* ftv = getMutable<FreeType>(ty); LUAU_ASSERT(ftv);
        // fresh_type_scope_ptr 刚分配的必是 FreeType。
        get_mutable_type::get_mutable::<FreeType>(ty)
          .expect("fresh_type_scope_ptr 刚分配的必是 FreeType（cpp LUAU_ASSERT(ftv)）")
          .forwarded_type_alias = true;

        let type_fun = TypeFun {
          type_params: defs.generic_types,
          type_pack_params: defs.generic_packs,
          r#type: ty,
          definition_location: Some(typealias.base.base.location),
        };

        // Arc<Scope> 注册后的就地写路径：别名物化收口进 alias_arc（单点
        // unsafe 契约），替代三段 `as_ref() as *const Scope as *mut Scope` 强转。
        if typealias.exported {
          alias_arc(&scope)
            .exported_type_bindings
            .insert(name.clone(), type_fun);
        } else {
          alias_arc(&scope)
            .private_type_bindings
            .insert(name.clone(), type_fun);
        }

        alias_arc(&scope)
          .type_alias_locations
          .insert(name.clone(), typealias.base.base.location);
        alias_arc(&scope)
          .type_alias_name_locations
          .insert(name, typealias.name_location);
      }
    }
  }

  pub fn prototype_scope_ptr_ast_stat_declare_extern_type(
    &mut self,
    scope: ScopePtr,
    declared_extern_type: &AstStatDeclareExternType,
  ) {
    // `self.builtin_types` 是 Handle（NonNull 编码非空，构造期接线、比本
    // TypeChecker 长寿），内建类型建好后不再改写；get() 只读物化借用取其
    // `Copy` 的 `extern_type` 句柄，契约收拢于 arena_handle 模块。
    let mut super_ty: Option<TypeId> = Some(self.builtin_types.get().extern_type);

    if let Some(super_name_astname) = declared_extern_type.super_name {
      let super_name: Name = super_name_astname.as_str_or_empty().to_string();
      let lookup_type = scope.lookup_type(&super_name);

      if lookup_type.is_none() {
        self.report_error_location_type_error_data(
          &declared_extern_type.base.base.location,
          TypeErrorData::UnknownSymbol(UnknownSymbol::new(super_name, Context::Type)),
        );
        self
          .incorrect_extern_type_definitions
          .insert(from_ref(declared_extern_type));
        return;
      }

      let lookup_type = lookup_type.expect("上方 is_none 分支已 return，此处必为 Some");

      // We don't have generic extern type_arguments, so this assertion _should_ never be hit.
      LUAU_ASSERT!(
        lookup_type.type_params().is_empty() && lookup_type.type_pack_params().is_empty()
      );
      super_ty = Some(lookup_type.r#type());

      if get_type::get::<ExternType>(follow_type::follow(
        super_ty.expect("上一分支刚以 Some(lookup_type.r#type()) 赋值"),
      ))
      .is_none()
      {
        let class_name = declared_extern_type.name.as_str_or_empty();
        self.report_error_location_type_error_data(
          &declared_extern_type.base.base.location,
          TypeErrorData::GenericError(GenericError::new(format(format_args!(
            "Cannot use non-class type '{}' as a superclass of class '{}'",
            (super_name_astname.as_str_or_empty()),
            class_name
          )))),
        );
        self
          .incorrect_extern_type_definitions
          .insert(from_ref(declared_extern_type));
        return;
      }
    }

    let class_name: Name = declared_extern_type.name.as_str_or_empty().to_string();

    let module_name = self.expect_current_module().name.clone();

    let scope_level = scope.level;
    // `TableType.scope` 为既有裸指针身份字段（键族，见 records/table_type.rs
    // 模块注释）：此处以 Arc 数据地址完成构造桥接，逐位同址于原三段强转。
    let scope_raw = Arc::as_ptr(&scope).cast_mut();

    // Safety: `shared_mut(self.current_module.as_ref().expect(..))` 裸化自 `self`
    // 独占持有的 `Arc<Module>`（本函数内单线程、模块存活），`as_ref` 借用仅用于
    // 取指针后即结束；`internal_types` 是 Module 内联的 bump TypeArena，块地址
    // 永不移动，`add_type` 的 `&mut` 独占短借用随本语句归还，返回的 ExternType
    // 节点驻留 arena。
    let class_ty: TypeId = {
      (shared_mut(self.expect_current_module()))
        .internal_types
        .add_type(ExternType {
          name: class_name.clone(),
          props: Default::default(),
          parent: super_ty,
          metatable: None,
          tags: Default::default(),
          user_data: None,
          definition_module_name: module_name,
          definition_location: Some(declared_extern_type.base.base.location),
          indexer: None,
          relation: None,
        })
    };
    // Safety: 与 class_ty 同一不变量——`current_module` 独占存活 Arc 裸化、
    // internal_types bump arena 地址稳定；本行重新对 arena 取独占短借用加
    // TableType，前一借用已随上一语句结束，时序不重叠。`scope_raw` 仅作身份
    // 句柄存入 TableType，本调用期间不解引用。
    let meta_ty: TypeId = {
      (shared_mut(self.expect_current_module()))
        .internal_types
        .add_type(TableType::table_type_table_state_type_level_scope(
          TableState::Sealed,
          scope_level,
          scope_raw,
        ))
    };

    // class_ty 刚以 ExternType 变体分配（TypeInfer.cpp:1711-1718），下转必然成功。
    get_mutable_type::get_mutable::<ExternType>(class_ty)
      .expect("class_ty 刚以 ExternType 变体分配（TypeInfer.cpp:1711-1718），下转必然成功")
      .metatable = Some(meta_ty);

    alias_arc(&scope).exported_type_bindings.insert(
      class_name,
      TypeFun {
        type_params: Default::default(),
        type_pack_params: Default::default(),
        r#type: class_ty,
        definition_location: Some(declared_extern_type.base.base.location),
      },
    );
  }
}
