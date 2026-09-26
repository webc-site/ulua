use ulua_common::fflag;

use crate::{
  functions::{
    add_intersection::add_intersection, assert_invariant::assert_invariant, get_type,
    is_top::is_top,
  },
  records::{
    intersection_type::IntersectionType, negation_type::NegationType, never_type::NeverType,
    normalized_type::NormalizedType, normalizer::Normalizer, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn type_from_normal(&mut self, norm: &NormalizedType) -> TypeId {
    assert_invariant(norm);

    // Safety: `self.builtin_types.as_ptr()` 对应 C++ `Normalizer::builtinTypes`
    // （`NotNull<BuiltinTypes>`），构造期注入即非空；BuiltinTypes 单例在
    // 内建类型构建完成后不再被改写，本方法的调用链（`is_top`、
    // `type_from_normal` 递归、`add_intersection`）对它都只取只读访问，
    // 故可整函数持有一个 `&`。下方各 `wired_arena_mut().add_type(..)` 改写的是
    // Normalizer 构造/接线期注入的外部 `TypeArena`（C++ `TypeArena* arena`，
    // 经 Option<Handle> 断言接线，与 BuiltinTypes 单例互不相交），不与该
    // 共享借用冲突。
    let builtin_types = self.builtin_types.get();

    if get_type::get::<NeverType>(norm.tops).is_none() {
      return norm.tops;
    }

    let mut result: Vec<TypeId> = Vec::new();

    if get_type::get::<NeverType>(norm.booleans).is_none() {
      result.push(norm.booleans);
    }

    if is_top(builtin_types, &norm.extern_types) {
      result.push(builtin_types.extern_type);
    } else if !norm.extern_types.is_never() {
      let mut parts: Vec<TypeId> = Vec::with_capacity(norm.extern_types.extern_types.len());

      for &norm_ty in norm.extern_types.ordering.iter() {
        // 成对登记不变式：`ordering` 是 `extern_types` 键的拓扑面列表，
        // 每个 ordering 元素都已登记于 extern_types，get 必命中。
        let norm_negations = norm
          .extern_types
          .extern_types
          .get(&norm_ty)
          .expect("ordering 元素必已成对登记于 extern_types");

        if norm_negations.empty()
          && (!fflag::LuauExternTypesNormalizeWithShapes.get()
            || norm.extern_types.shape_extensions.empty())
        {
          parts.push(norm_ty);
        } else {
          let mut intersection: Vec<TypeId> = Vec::with_capacity(norm_negations.size() + 1);

          intersection.push(norm_ty);
          for &negation in norm_negations.order.iter() {
            let negation_type = NegationType { ty: negation };
            // 契约：`self.arena` 是构造/接线期注入的外部 arena（wired_arena_mut
            // 断言非空，C++ `arena->addType(NegationType{negation})`），`negation`
            // 取自 `norm` 输入、指向 arena 驻留节点；本次 `&mut` 再借用随语句
            // 结束归还。
            intersection.push(self.wired_arena_mut().add_type(negation_type));
          }

          if fflag::LuauExternTypesNormalizeWithShapes.get() {
            for &shape in norm.extern_types.shape_extensions.order.iter() {
              intersection.push(shape);
            }
          }

          // 契约：`intersection` 各项均为 arena 驻留 `TypeId`（本分支上文
          // 刚写入或来自 `norm`），`add_type` 对 arena 的独占借用止于本
          // 语句——对应 C++ `arena->addType(IntersectionType{...})`。
          parts.push(self.wired_arena_mut().add_type(IntersectionType {
            parts: intersection,
          }));
        }
      }

      if parts.len() == 1 {
        result.push(parts[0]);
      } else if parts.len() > 1 {
        // 契约：`parts` 由上面各 arena 节点句柄组成，`add_type` 的独占
        // 借用随语句结束（C++ `arena->addType(UnionType{std::move(parts)})`）。
        result.push(
          self
            .wired_arena_mut()
            .add_type(UnionType { options: parts }),
        );
      }
    }

    if get_type::get::<NeverType>(norm.errors).is_none() {
      result.push(norm.errors);
    }

    if norm.functions.is_top {
      result.push(builtin_types.function_type);
    } else if !norm.functions.parts.is_never() {
      if norm.functions.parts.order.len() == 1 {
        result.push(norm.functions.parts.order[0]);
      } else {
        let mut parts: Vec<TypeId> = Vec::new();
        parts.extend(norm.functions.parts.order.iter().copied());
        // 契约：`parts` 全部来自 `norm.functions`（归一化输入持有的 arena
        // 句柄），独占借用 arena 一次建新 Intersection 节点（C++
        // `arena->addType(IntersectionType{...})`），借用止于本语句。
        result.push(self.wired_arena_mut().add_type(IntersectionType { parts }));
      }
    }

    if get_type::get::<NeverType>(norm.nils).is_none() {
      result.push(norm.nils);
    }

    if get_type::get::<NeverType>(norm.numbers).is_none() {
      result.push(norm.numbers);
    }

    if fflag::LuauIntegerType2.get() && get_type::get::<NeverType>(norm.integers).is_none() {
      result.push(norm.integers);
    }

    if norm.strings.is_string() {
      result.push(builtin_types.string_type);
    } else if norm.strings.is_union() {
      for ty in norm.strings.singletons.values() {
        result.push(*ty);
      }
    } else if norm.strings.is_intersection() {
      let mut parts: Vec<TypeId> = Vec::new();
      parts.push(builtin_types.string_type);

      for ty in norm.strings.singletons.values() {
        let negation_type = NegationType { ty: *ty };
        // 契约：`*ty` 是 `norm.strings` 单例集里的 arena 驻留句柄，
        // `wired_arena_mut` 对已接线的 arena 取一次性独占借用
        // （C++ `arena->addType(NegationType{*ty})`）。
        parts.push(self.wired_arena_mut().add_type(negation_type));
      }

      // 契约：`parts` 首项为共享只读的 builtin `string_type`，其余为
      // 上文刚建出的 arena Negation 节点；arena 独占短借用随语句结束。
      result.push(self.wired_arena_mut().add_type(IntersectionType { parts }));
    }

    if get_type::get::<NeverType>(norm.threads).is_none() {
      result.push(builtin_types.thread_type);
    }

    if get_type::get::<NeverType>(norm.buffers).is_none() {
      result.push(builtin_types.buffer_type);
    }

    if self.use_new_luau_solver() {
      result.reserve(norm.tables.size());
      for &table in norm.tables.order.iter() {
        result.push(table);
      }
    } else {
      result.extend(norm.tables.order.iter().copied());
    }

    for (&tyvar, intersect) in norm.tyvars.iter() {
      if get_type::get::<NeverType>(intersect.tops).is_some() {
        let ty = self.type_from_normal(intersect);
        // add_intersection 收 NotNull 语义的 Handle：构造期 null 哨兵只出现在
        // 模块外归一化上报分支，走到这里 None 属契约违例，确定性 panic
        // （与原 `Handle::from_ptr(null)` 断言语义等价）。
        result.push(add_intersection(
          self
            .arena
            .expect("Normalizer.arena 使用前必须已接线（add_intersection 要求 NotNull）"),
          self.builtin_types,
          &[tyvar, ty],
        ));
      } else {
        result.push(tyvar);
      }
    }

    if result.is_empty() {
      builtin_types.never_type
    } else if result.len() == 1 {
      result[0]
    } else {
      // 契约：`result` 各项要么是 `norm` 输入携带的、要么是上文刚写入
      // arena 的驻留句柄；对已接线的 arena 取一次性 `&mut` 建 Union 节点，
      // 借用随返回结束（C++ `arena->addType(UnionType{std::move(result)})`）。
      self
        .wired_arena_mut()
        .add_type(UnionType { options: result })
    }
  }
}
