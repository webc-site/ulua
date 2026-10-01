use alloc::{
  string::{String, ToString},
  sync::Arc,
  vec::Vec,
};
use core::mem::take;

use ulua_ast::{
  enums::{ast_table_access::AstTableAccess, ast_type_ref::AstTypeRef},
  records::{ast_attr::AstAttrType, ast_type::AstType, ast_type_or_pack::AstTypeOrPack},
};

use crate::{
  enums::table_state::TableState,
  functions::{
    finite::finite, first::first, reduce_union::reduce_union, shared_mut::shared_mut,
    size_type_pack::size,
  },
  records::{
    arena_handle::alias_ref,
    function_argument::FunctionArgument,
    function_type::FunctionType,
    generic_error::GenericError,
    incorrect_generic_parameter_count::IncorrectGenericParameterCount,
    intersection_type::IntersectionType,
    property_type::Property,
    swapped_generic_type_parameter::SwappedGenericTypeParameter,
    table_indexer::TableIndexer,
    table_type::TableType,
    type_checker::TypeChecker,
    type_pack,
    type_pack::TypePack,
    union_type::UnionType,
    unknown_symbol::{Context, UnknownSymbol},
  },
  type_aliases::{
    name_type::Name, props_type::Props, scope_ptr_type::ScopePtr, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};
impl TypeChecker {
  pub fn resolve_type_worker(&mut self, scope: ScopePtr, annotation: &AstType) -> TypeId {
    let node = annotation;

    match node.as_type_ref() {
      AstTypeRef::Group(group) => self.resolve_type(scope, alias_ref(group.type_)),
      AstTypeRef::Error(_) => self.error_recovery_type_scope_ptr(&scope),
      AstTypeRef::Reference(reference) => {
        let name: Name = reference.name.as_str_or_empty().to_string();

        let alias = if let Some(prefix) = reference.prefix {
          let prefix: Name = prefix.as_str_or_empty().to_string();
          scope.lookup_imported_type(&prefix, &name)
        } else {
          scope.lookup_type(&name)
        };

        if let Some(tf) = alias {
          if reference.parameters.is_empty()
            && tf.type_params().is_empty()
            && tf.type_pack_params().is_empty()
          {
            return tf.r#type();
          }

          let mut parameter_count_error_reported = false;
          let has_default_types = tf
            .type_params()
            .iter()
            .any(|param| param.default_value.is_some());
          let has_default_packs = tf
            .type_pack_params()
            .iter()
            .any(|param| param.default_value.is_some());

          if !reference.has_parameter_list
            && ((!tf.type_params().is_empty() && !has_default_types)
              || (!tf.type_pack_params().is_empty() && !has_default_packs))
          {
            self.report_error_location_type_error_data(
              &annotation.base.location,
              GenericError::new(String::from("Type parameter list is required")).into(),
            );
            parameter_count_error_reported = true;
          }

          let mut type_params: Vec<TypeId> = Vec::new();
          let mut extra_types: Vec<TypeId> = Vec::new();
          let mut type_pack_params: Vec<TypePackId> = Vec::new();

          for &param in reference.parameters.iter() {
            // cpp `if (param.type) … else if (param.typePack) …`（TypeChecker.cpp
            // resolveTypeWorker）：变体载荷即 arena 存活注解节点，直接按引用交给
            // resolve；Error 形态两臂都不进，与 cpp 双空时静默跳过一致。
            match param {
              AstTypeOrPack::Type(param_ty) => {
                let ty = self.resolve_type(scope.clone(), param_ty);

                if type_params.len() < tf.type_params().len() || tf.type_pack_params().is_empty() {
                  type_params.push(ty);
                } else if type_pack_params.is_empty() {
                  extra_types.push(ty);
                } else {
                  self.report_error_location_type_error_data(
                    &annotation.base.location,
                    GenericError::new(String::from(
                      "Type parameters must come before type pack parameters",
                    ))
                    .into(),
                  );
                }
              }
              AstTypeOrPack::Pack(param_pack) => {
                let tp = self.resolve_type_pack_scope_ptr_ast_type_pack(scope.clone(), param_pack);

                if type_pack_params.is_empty() && !extra_types.is_empty() {
                  type_pack_params
                    .push(self.add_type_pack_type_pack(TypePack::from_vec(take(&mut extra_types))));
                }

                if type_params.len() < tf.type_params().len()
                // tp 刚由 resolveTypePack 产出，必为类型 arena 中存活的
                // TypePackId（bump 分配不移动）；size 的 log=None 等价 C++ 默认
                // 实参 nullptr，走无事务日志的直接 follow 路径，只读遍历
                // head/tail，借用止于本操作数。
                && size(tp, None) == 1
                // tp 为 arena 存活类型包；finite 的 log=None 即 C++ 默认 nullptr 形参，
                // 仅沿 tail 只读递归判定，无写操作与别名。
                && finite(tp, None)
                {
                  if let Some(first_ty) = first(tp, true) {
                    type_params.push(first_ty);
                  } else {
                    type_pack_params.push(tp);
                  }
                } else {
                  type_pack_params.push(tp);
                }
              }
              AstTypeOrPack::Error => {}
            }
          }

          if type_pack_params.is_empty() && !extra_types.is_empty() {
            type_pack_params.push(self.add_type_pack_type_pack(TypePack::from_vec(extra_types)));
          }

          let types_required = tf.type_params().len();
          let packs_required = tf.type_pack_params().len();
          let not_enough_parameters = (type_params.len() < types_required
            && type_pack_params.is_empty())
            || (type_params.len() == types_required && type_pack_params.len() < packs_required);

          if not_enough_parameters && (has_default_types || has_default_packs) {
            // 缺省参数补齐：遇 None 即停（map_while），与逐索引循环等价
            type_params.extend(
              tf.type_params()[type_params.len()..types_required]
                .iter()
                .map_while(|param| param.default_value),
            );

            type_pack_params.extend(
              tf.type_pack_params()[type_pack_params.len()..packs_required]
                .iter()
                .map_while(|param| param.default_value),
            );
          }

          if reference.parameters.is_empty() && type_pack_params.len() + 1 == packs_required {
            type_pack_params.push(self.add_type_pack_type_pack(TypePack::empty()));
          }

          if type_params.len() != types_required || type_pack_params.len() != packs_required {
            if !parameter_count_error_reported {
              self.report_error_location_type_error_data(
                &annotation.base.location,
                IncorrectGenericParameterCount {
                  name: name.clone(),
                  type_fun: tf.clone(),
                  actual_parameters: type_params.len(),
                  actual_pack_parameters: type_pack_params.len(),
                }
                .into(),
              );
            }

            // 补齐到必需数量：每轮都调用 self 方法求值，非恒定填充，故保留 while。
            while type_params.len() < types_required {
              type_params.push(self.error_recovery_type_scope_ptr(&scope));
            }

            while type_pack_params.len() < packs_required {
              type_pack_params.push(self.error_recovery_type_pack_scope_ptr(scope.clone()));
            }
          }

          let same_tys = type_params
            .iter()
            .zip(tf.type_params().iter())
            .all(|(arg, param)| *arg == param.ty);
          let same_tps = type_pack_params
            .iter()
            .zip(tf.type_pack_params().iter())
            .all(|(arg, param)| *arg == param.tp);

          if same_tys
            && same_tps
            && type_params.len() == tf.type_params().len()
            && type_pack_params.len() == tf.type_pack_params().len()
          {
            return tf.r#type();
          }

          return self.instantiate_type_fun(
            &scope,
            &tf,
            &type_params,
            &type_pack_params,
            &annotation.base.location,
          );
        }

        let mut type_name = String::new();
        if let Some(prefix) = reference.prefix {
          let prefix: Name = prefix.as_str_or_empty().to_string();
          type_name.push_str(&prefix);
          type_name.push('.');
        }
        type_name.push_str(&name);

        if scope.lookup_pack(&type_name).is_some() {
          self.report_error_location_type_error_data(
            &annotation.base.location,
            SwappedGenericTypeParameter {
              name: type_name,
              kind: SwappedGenericTypeParameter::TYPE,
            }
            .into(),
          );
        } else {
          self.report_error_location_type_error_data(
            &annotation.base.location,
            UnknownSymbol::new(type_name, Context::Type).into(),
          );
        }

        self.error_recovery_type_scope_ptr(&scope)
      }
      AstTypeRef::Optional(_) => self.nil_type,
      AstTypeRef::Table(table) => {
        let mut props = Props::default();

        for prop in table.props.iter() {
          match prop.access {
            AstTableAccess::Read => {
              self.report_error_location_type_error_data(
                &prop.access_location.unwrap_or_default(),
                GenericError::new(String::from("read keyword is illegal here")).into(),
              );
            }
            AstTableAccess::Write => {
              self.report_error_location_type_error_data(
                &prop.access_location.unwrap_or_default(),
                GenericError::new(String::from("write keyword is illegal here")).into(),
              );
            }
            AstTableAccess::ReadWrite => {
              let name: Name = prop.name.as_str_or_empty().to_string();
              let ty = if prop.r#type.is_null() {
                self.error_recovery_type_scope_ptr(&scope)
              } else {
                self.resolve_type(scope.clone(), alias_ref(prop.r#type))
              };
              let mut property = Property {
                type_location: Some(prop.location),
                ..Property::default()
              };
              property.read_ty = Some(ty);
              property.write_ty = Some(ty);
              props.insert(name, property);
            }
          }
        }

        let indexer = table.indexer.get().and_then(|indexer| {
          match indexer.access {
            AstTableAccess::Read => {
              self.report_error_location_type_error_data(
                &indexer.access_location.unwrap_or_default(),
                GenericError::new(String::from("read keyword is illegal here")).into(),
              );
              None
            }
            AstTableAccess::Write => {
              self.report_error_location_type_error_data(
                &indexer.access_location.unwrap_or_default(),
                GenericError::new(String::from("write keyword is illegal here")).into(),
              );
              None
            }
            // index/result 为句柄化恒非空子节点（cpp 同处无条件解引用），
            // 防御性 null 分支随槽位句柄化一并消除。
            AstTableAccess::ReadWrite => Some(TableIndexer {
              index_type: self.resolve_type(scope.clone(), indexer.index_type.get()),
              index_result_type: self.resolve_type(scope.clone(), indexer.result_type.get()),
              is_read_only: false,
            }),
          }
        });

        let table_ty =
          TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &props,
            indexer,
            scope.level,
            scope.as_ref() as *const _ as *mut _,
            TableState::Sealed,
          );
        self.add_type(&table_ty)
      }
      AstTypeRef::Function(func) => {
        let func_scope = self.child_scope(&scope, &func.base.base.location);
        // Safety: `shared_mut` 惯用法——func_scope 是 child_scope 刚新建、由本
        // 绑定独占持有的 Arc<Scope>，至少活过本块；检查器单线程运行，写入时
        // 该 Scope 无并存 Rust 借用。语义等同 C++ `funcScope->level =
        // scope->level.incr()`（TypeInfer.cpp:5943）。
        {
          let func_scope_raw = shared_mut(&func_scope);
          func_scope_raw.level = scope.level.incr();
        }

        let defs = self.create_generic_types(
          &func_scope,
          None,
          &annotation.base,
          &func.generics,
          &func.generic_packs,
          false,
        );

        let arg_types =
          self.resolve_type_pack_scope_ptr_ast_type_list(func_scope.clone(), &func.arg_types);
        let ret_types = if func.return_types.is_null() {
          self.add_type_pack_type_pack(type_pack::TypePack::empty())
        } else {
          self.resolve_type_pack_scope_ptr_ast_type_pack(
            func_scope.clone(),
            alias_ref(func.return_types),
          )
        };

        let mut ftv = FunctionType::function_type_new(arg_types, ret_types, None, false);
        ftv.level = func_scope.level;
        ftv.generics = defs.generic_types.iter().map(|def| def.ty).collect();
        ftv.generic_packs = defs.generic_packs.iter().map(|def| def.tp).collect();

        for arg_name in func.arg_names.iter() {
          ftv
            .arg_names
            .push(arg_name.map(|(name, location)| FunctionArgument {
              name: name.as_str_or_empty().to_string(),
              location,
            }));
        }

        ftv.is_checked_function = func.is_checked_function();
        let deprecated_attr = func.get_attribute(AstAttrType::Deprecated);
        ftv.is_deprecated_function = !deprecated_attr.is_null();
        if !deprecated_attr.is_null() {
          ftv.deprecated_info = Some(Arc::new(alias_ref(deprecated_attr).deprecated_info()));
        }

        self.add_type(&ftv)
      }
      AstTypeRef::Typeof(type_of) => {
        self
          .check_expr(&scope, alias_ref(type_of.expr), None, false)
          .r#type
      }
      AstTypeRef::Union(union) => {
        let mut parts = Vec::new();

        for part in union.types.iter() {
          if !part.is_null() {
            parts.push(self.resolve_type(scope.clone(), alias_ref(*part)));
          }
        }

        let reduced = reduce_union(&parts);
        match reduced.len() {
          0 => self.never_type,
          1 => reduced[0],
          _ => self.add_type(&UnionType { options: reduced }),
        }
      }
      AstTypeRef::Intersection(intersection) => {
        let mut parts = Vec::new();

        for part in intersection.types.iter() {
          if !part.is_null() {
            parts.push(self.resolve_type(scope.clone(), alias_ref(*part)));
          }
        }

        match parts.len() {
          0 => self.never_type,
          1 => parts[0],
          _ => self.add_type(&IntersectionType { parts }),
        }
      }
      AstTypeRef::SingletonBool(singleton_bool) => self.singleton_type_bool(singleton_bool.value),
      AstTypeRef::SingletonString(singleton_string) => {
        let bytes: Vec<u8> = singleton_string.value.as_bytes().to_vec();
        let value = String::from_utf8_lossy(&bytes).into_owned();
        self.singleton_type_string(value)
      }
    }
  }
}
