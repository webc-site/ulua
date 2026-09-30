use alloc::vec::Vec;

use ulua_ast::{
  enums::ast_type_pack_ref::AstTypePackRef,
  records::{ast_type_list::AstTypeList, ast_type_pack::AstTypePack},
};

use crate::{
  records::{
    swapped_generic_type_parameter::SwappedGenericTypeParameter,
    type_checker::TypeChecker,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
    unknown_symbol::{Context, UnknownSymbol},
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};

impl TypeChecker {
  pub(crate) fn resolve_type_pack_scope_ptr_ast_type_list(
    &mut self,
    scope: ScopePtr,
    types: &AstTypeList,
  ) -> TypePackId {
    if types.types.size == 0 && !types.tail_type.is_null() {
      // Safety: tail_type 刚经同行 is_null 判定非空；它指向解析 arena 中的
      // typePack 标注节点（SourceModule allocator 持有，分析全程存活不改写），
      // 只读再借用后 resolve 仅向下读标注、写入进 type arena，无别名冲突。
      return self.resolve_type_pack_scope_ptr_ast_type_pack(scope, unsafe { &*types.tail_type });
    } else if types.types.size > 0 {
      let mut head = Vec::with_capacity(types.types.size);
      for &ann in types.types.as_slice() {
        // Safety: parser 为标注列表逐槽分配 AstType 对象后填充 AstArray，
        // 长度内元素非空且随解析树存活；这里只建只读借用交 resolve_type。
        let ty = self.resolve_type(scope.clone(), unsafe { &*ann });
        head.push(ty);
      }

      let tail = if !types.tail_type.is_null() {
        // Safety: 与首分支同一判空门槛（刚查过 is_null）；tail_type 指向的
        // 标注节点在 arena 中有效，scope Arc 仅 clone 计数不受影响。
        let tail_ann = unsafe { &*types.tail_type };
        Some(self.resolve_type_pack_scope_ptr_ast_type_pack(scope.clone(), tail_ann))
      } else {
        None
      };

      return self.add_type_pack_type_pack(TypePack::new(head, tail));
    }

    self.add_type_pack_type_pack(TypePack::empty())
  }

  pub(crate) fn resolve_type_pack_scope_ptr_ast_type_pack(
    &mut self,
    scope: ScopePtr,
    annotation: &AstTypePack,
  ) -> TypePackId {
    match annotation.as_pack_ref() {
      AstTypePackRef::Explicit(explicit) => {
        self.resolve_type_pack_scope_ptr_ast_type_list(scope, &explicit.type_list)
      }
      AstTypePackRef::Variadic(variadic) => {
        let ty = if variadic.variadic_type.is_null() {
          self.error_recovery_type_scope_ptr(&scope)
        } else {
          // Safety: 本分支保证 variadic_type 非空；它指向解析 arena 的类型标注
          // 节点（parser 随 variadic 标注分配），存活期与解析树等长，只读借用。
          self.resolve_type(scope.clone(), unsafe { &*variadic.variadic_type })
        };

        self.add_type_pack_type_pack_var(TypePackVar::from(VariadicTypePack { ty, hidden: false }))
      }
      AstTypePackRef::Generic(generic) => {
        let name = generic.generic_name.as_str_or_empty().to_string();
        if let Some(generic_pack) = scope.lookup_pack(&name) {
          return generic_pack;
        }

        let location = generic.base.base.location;
        if scope.lookup_type(&name).is_some() {
          self.report_error_location_type_error_data(
            &location,
            SwappedGenericTypeParameter {
              name,
              kind: SwappedGenericTypeParameter::PACK,
            }
            .into(),
          );
        } else {
          self.report_error_location_type_error_data(
            &location,
            UnknownSymbol::new(name, Context::Type).into(),
          );
        }

        self.error_recovery_type_pack_scope_ptr(scope)
      }
    }
  }
}
