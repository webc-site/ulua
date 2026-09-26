//! `TypeChecker2::testPotentialLiteralIsSubtype`（TypeChecker2.cpp 对照）。
use alloc::{
  string::{String, ToString},
  vec::Vec,
};
use core::ptr::from_ref;

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_group::AstExprGroup,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_node,
    ast_node::AstNode,
  },
  rtti::ast_node_try_as,
};
use ulua_common::fflag;

use crate::{
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    extract_matching_table_type::extract_matching_table_type,
    extract_matching_table_type_deprecated::extract_matching_table_type_deprecated,
    follow_type, get_type,
    is_optional::is_optional,
    is_record::is_record,
    simplify_intersection_simplify::simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids,
  },
  records::{
    arena_handle::Handle,
    intersection_type::IntersectionType,
    missing_properties::{Context as MissingPropertiesContext, MissingProperties},
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    type_checker_2::TypeChecker2,
    type_ids::TypeIds,
    unexpected_array_like_table_item::UnexpectedArrayLikeTableItem,
    union_type::UnionType,
  },
  type_aliases::{
    singleton_variant::SingletonVariant, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl TypeChecker2 {
  pub fn test_potential_literal_is_subtype(
    &mut self,
    expr: &AstExpr,
    expected_type: TypeId,
  ) -> bool {
    let expr_type = follow_type::follow(self.lookup_type(expr));
    let expected_type = follow_type::follow(expected_type);

    // Safety: AstExpr 是 #[repr(C)] 单继承且首字段为 base: AstNode，
    // `*const AstExpr → *const AstNode` 的地址保持（偏移 0）；`expr` 是
    // 调用方借用的存活 AST 节点，转成 AstNode 引用仅读 class_index。
    let node = unsafe { &*((expr as *const AstExpr).cast::<ast_node::AstNode>()) };

    if let Some(group) = ast_node_try_as::<AstExprGroup>(node) {
      // expr 已句柄化恒非空：get() 只读借用出自 group 存活引用，递归借用合法。
      return self.test_potential_literal_is_subtype(group.expr.get(), expected_type);
    }

    if let Some(if_else) = ast_node_try_as::<AstExprIfElse>(node) {
      // 三子节点已句柄化恒非空：get() 只读借用出自 if_else 存活引用，递归借用合法。
      let mut passes =
        self.test_potential_literal_is_subtype(if_else.true_expr.get(), expected_type);
      passes &= self.test_potential_literal_is_subtype(if_else.false_expr.get(), expected_type);
      return passes;
    }

    if let Some(bin_expr) = ast_node_try_as::<AstExprBinary>(node)
      && bin_expr.op == AstExprBinaryOp::Or
    {
      // Safety: self.module 是 check_type_checker_2 传入的 Arc<Module>
      // 写穿句柄（check_frontend::check 单线程独占期），self.builtin_types.as_ptr()
      // 是该函数契约保证非空存活的 Frontend 成员；add_type 向
      // internal_types bump arena 追加 UnionType，返回的地址稳定指针。
      let relaxed_expected_lhs = unsafe {
        (*self.module).internal_types.add_type(UnionType {
          options: alloc::vec![self.builtin_types.get().falsy_type, expected_type],
        })
      };
      // left/right 已句柄化：get() 只读借用出自存活 &AstExprBinary，
      // 递归借用只读，不构成别名冲突。
      let mut passes =
        self.test_potential_literal_is_subtype(bin_expr.left.get(), relaxed_expected_lhs);
      passes &= self.test_potential_literal_is_subtype(bin_expr.right.get(), expected_type);
      return passes;
    }

    let expr_table = ast_node_try_as::<AstExprTable>(node);
    let expr_table_type = get_type::get::<TableType>(expr_type);
    let expected_table_type = get_type::get::<TableType>(expected_type);

    let (Some(expr_table), Some(_)) = (expr_table, expr_table_type) else {
      return self.test_is_subtype_type_id_type_id_location(
        expr_type,
        expected_type,
        expr.base.location,
      );
    };

    let Some(expected_table_type) = expected_table_type else {
      if let Some(expected_union) = get_type::get::<UnionType>(expected_type) {
        if fflag::LuauBidirectionalInferenceBetterUnionHandling.get() {
          if let Some(tt) = extract_matching_table_type(
            expected_union,
            expr_type,
            Handle::from_ptr(self.builtin_types.as_ptr()),
          ) {
            // Safety: `expr` 是本次调用的存活借用（解析 arena 节点），
            // 指针往返不改变地址；`tt` 是 extract_matching_table_type 从
            // union 成员里挑出的 TableType 所在 arena 节点（已 follow 非空）。
            return unsafe {
              self.test_literal_or_ast_type_is_subtype(from_ref(expr).cast_mut(), tt)
            };
          }
        } else {
          // C++ 直接传 utv，函数内走 TypeIterator 展平嵌套 union 并
          // follow，调用侧先展平对齐语义。
          let mut parts: Vec<TypeId> = begin_union_type(expected_union).collect();
          // Safety: parts 内每个 TypeId 都是模块 arena 驻留节点（union
          // 成员，已 follow），builtin_types 按上游契约非空存活。
          if let Some(tt) = unsafe {
            extract_matching_table_type_deprecated(
              &mut parts,
              expr_type,
              Handle::from_ptr(self.builtin_types.as_ptr()),
            )
          } {
            return self.test_potential_literal_is_subtype(expr, tt);
          }
        }
      }

      if let Some(expected_intersection) = get_type::get::<IntersectionType>(expected_type) {
        // C++ `parts.insert(begin(itv), end(itv))`——迭代器展平嵌套
        // intersection 并 follow，裸遍历 parts 会漏掉嵌套成员。
        let mut parts = TypeIds::new();
        for part in begin_intersection_type(expected_intersection) {
          parts.insert_type_id(part);
        }

        // Safety: self.builtin_types.as_ptr()/self.module 的来历同 falsy 分支
        // （check_type_checker_2 注入、单线程独占），&mut 借用
        // internal_types arena 供 simplify 追加新交集类型，调用期内
        // arena 指针稳定。
        let simplified = unsafe {
          simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids(
            self.builtin_types,
            Handle::from_mut(&mut (*self.module).internal_types),
            parts,
          )
        }
        .result;

        if get_type::get::<TableType>(simplified).is_some() {
          return self.test_potential_literal_is_subtype(expr, simplified);
        }
      }

      return self.test_is_subtype_type_id_type_id_location(
        expr_type,
        expected_type,
        expr.base.location,
      );
    };

    let mut missing_keys: Vec<String> = Vec::new();
    for (name, prop) in expected_table_type.props.iter() {
      if let Some(read_ty) = prop.read_ty
        && !is_optional(read_ty)
      {
        missing_keys.push(name.clone());
      }
    }

    let scope = self.find_innermost_scope(expr.base.location);
    let mut is_array_like = false;
    if let Some(indexer) = &expected_table_type.indexer {
      // subtyping 已句柄化（指向构造期内嵌 `_subtyping`，存活期覆盖整个
      // checker 运行）；scope 来自 find_innermost_scope，其起点是模块
      // scope 的写穿句柄、恒非空；builtin_types 按函数契约有效。
      let result = self
        .subtyping_mut()
        .is_subtype_type_id_type_id_not_null_scope(
          self.builtin_types.get().number_type,
          indexer.index_type,
          scope,
        );
      is_array_like = result.is_subtype
        || self.is_error_suppressing_location_type_id(expr.base.location, indexer.index_type);
    }

    let mut is_subtype = true;
    for item in expr_table.items.iter() {
      if is_record(item) {
        // Safety: is_record 对 General 显式判 item.key 非空且 class 为
        // AstExprConstantString；Record 项的 key 由解析器必写。非空 +
        // repr(C) 首字段偏移 0 保证 key→AstNode 的 cast 引用合法。
        let key_const_string = ast_node_try_as::<AstExprConstantString>(unsafe {
          &*((item.key as *const AstExpr).cast::<AstNode>())
        });
        let Some(key_const_string) = key_const_string else {
          continue;
        };

        let key_str = key_const_string.value.as_str().unwrap_or("").to_string();

        missing_keys.retain(|key| key != &key_str);

        if let Some(prop) = expected_table_type.props.get(&key_str) {
          if let Some(read_ty) = prop.read_ty {
            // Safety: self.module 为 Arc<Module> 写穿句柄（单线程独占），
            // ast_expected_types 以 AST 节点地址为键：item.value 是解析器
            // 写入的非空 arena 指针，get_or_insert 返回其 slot 的独占
            // *mut，写穿仅在 &mut self 期内发生。
            unsafe {
              *(*self.module)
                .ast_expected_types
                .get_or_insert(item.value as *const AstExpr) = read_ty;
            }
            // Safety: item.value 非空且指向随解析 arena 存活的表达式
            // 节点，&mut self 的递归检查只读 AST，不构成别名冲突。
            is_subtype &= unsafe { self.test_potential_literal_is_subtype(&*item.value, read_ty) };
          }
        } else if let Some(indexer) = &expected_table_type.indexer {
          // Safety: 块内两类指针来源均已在函数头论证——self.module 写穿
          // 句柄独占（key/value 槽位写入与 add_type 追加 arena），
          // item.key/item.value 是 is_record/解析器保证的非空 arena AST
          // 节点；(*item.key).base.location 仅读首字段，无并存借用。
          unsafe {
            *(*self.module)
              .ast_expected_types
              .get_or_insert(item.key as *const AstExpr) = indexer.index_type;
            *(*self.module)
              .ast_expected_types
              .get_or_insert(item.value as *const AstExpr) = indexer.index_result_type;
            let inferred_key_type = (*self.module).internal_types.add_type(SingletonType {
              variant: SingletonVariant::V1(StringSingleton {
                value: key_str.clone(),
              }),
            });
            is_subtype &= self.test_is_subtype_type_id_type_id_location(
              inferred_key_type,
              indexer.index_type,
              (*item.key).base.location,
            );
            is_subtype &=
              self.test_potential_literal_is_subtype(&*item.value, indexer.index_result_type);
          }
        }
      } else if item.kind == ItemKind::List {
        if !is_array_like {
          is_subtype = false;
          // Safety: List 形如 `{e}` 的项由解析器保证 value 非空
          // （key 为 null、value 指向 arena 表达式节点），读 location 合法。
          self.report_error_type_error_data_location(
            TypeErrorData::UnexpectedArrayLikeTableItem(UnexpectedArrayLikeTableItem::default()),
            unsafe { &(*item.value).base.location },
          );
        }

        if let Some(indexer) = &expected_table_type.indexer {
          // Safety: value 非空同上；self.module 写穿句柄在本迭代独占，
          // ast_expected_types 槽写入与递归只读检查不同时持有同一借用。
          unsafe {
            *(*self.module)
              .ast_expected_types
              .get_or_insert(item.value as *const AstExpr) = indexer.index_result_type;
            is_subtype &=
              self.test_potential_literal_is_subtype(&*item.value, indexer.index_result_type);
          }
        }
      } else if item.kind == ItemKind::General
        && let Some(indexer) = &expected_table_type.indexer
      {
        // Safety: General 项的 key/value 均由解析器写为非空 arena 表达式
        // 指针；对 self.module 的 ast_expected_types 写入处于 &mut self
        // 独占窗口，槽位指针与后续递归借用不同时活跃。
        unsafe {
          *(*self.module)
            .ast_expected_types
            .get_or_insert(item.key as *const AstExpr) = indexer.index_type;
          *(*self.module)
            .ast_expected_types
            .get_or_insert(item.value as *const AstExpr) = indexer.index_result_type;
          is_subtype &= self.test_potential_literal_is_subtype(&*item.key, indexer.index_type);
          is_subtype &=
            self.test_potential_literal_is_subtype(&*item.value, indexer.index_result_type);
        }
      }
    }

    if !missing_keys.is_empty() {
      self.report_error_type_error_data_location(
        TypeErrorData::MissingProperties(MissingProperties {
          super_type: expected_type,
          sub_type: expr_type,
          properties: missing_keys,
          context: MissingPropertiesContext::Missing,
        }),
        &expr.base.location,
      );
      return false;
    }

    is_subtype
  }
}
