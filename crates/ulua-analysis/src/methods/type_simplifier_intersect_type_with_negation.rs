use alloc::collections::BTreeSet;

use ulua_common::records::variant::Variant2;

use crate::{
  enums::relation::Relation,
  functions::{
    begin_type::begin_union_type, follow_type, get_type, relate_simplify::relate_type_id_type_id,
  },
  records::{
    boolean_singleton::BooleanSingleton, intersection_type::IntersectionType,
    negation_type::NegationType, primitive_type::PrimitiveType, singleton_type::SingletonType,
    type_ids::TypeIds, type_simplifier::TypeSimplifier, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeSimplifier {
  pub fn intersect_type_with_negation(&mut self, left: TypeId, right: TypeId) -> TypeId {
    // 契约：self.builtin_types 对应构造期接线的 `Handle<BuiltinTypes>`（`NotNull<BuiltinTypes>`），非空、对齐且
    // 比本 TypeSimplifier 长寿的进程级只读单例；此处建只读共享借用，仅读取 never/false/true 等恒定型。
    let builtin_types = self.builtin_types.get();

    // C++ Simplify.cpp intersectTypeWithNegation: LUAU_ASSERT(leftNegation)
    let left_negation = get_type::get::<NegationType>(left).expect("left is NegationType");

    let negated_ty = follow_type::follow(left_negation.ty);

    if negated_ty == right {
      return builtin_types.never_type;
    }

    if let Some(ut) = get_type::get::<UnionType>(negated_ty) {
      // ~(A | B) & C
      // (~A & C) & (~B & C)
      let mut changed = false;
      let mut new_parts = TypeIds::new();

      // C++ 的 `for (TypeId part : ut)` 走防环且展平嵌套 union 的 TypeIterator。
      for part in begin_union_type(ut) {
        let r = relate_type_id_type_id(part, right);
        match r {
          // ~(false?) & nil
          // (~false & nil) & (~nil & nil)
          // nil & never
          //
          // fallthrough
          Relation::Coincident => {
            // ~(boolean | string) & true
            // (~boolean & true) & (~boolean & string)
            // never & string
            return builtin_types.never_type;
          }
          Relation::Superset => {
            return builtin_types.never_type;
          }
          Relation::Disjoint => {
            // ~nil & boolean
            new_parts.insert_type_id(right);
          }
          // ~false & boolean
          // fallthrough
          Relation::Subset | Relation::Intersects => {
            // FIXME: The mkNegation here is pretty unfortunate.
            // Memoizing this will probably be important.
            changed = true;
            new_parts.insert_type_id(right);
            let negation = self.mk_negation(part);
            new_parts.insert_type_id(negation);
          }
        }
      }

      if !changed {
        return right;
      }

      return self.intersect_from_parts(new_parts);
    }

    if let Some(right_union) = get_type::get::<UnionType>(right) {
      // ~A & (B | C)
      let mut changed = false;
      let mut new_parts: BTreeSet<TypeId> = BTreeSet::new();

      for part in begin_union_type(right_union) {
        let r = relate_type_id_type_id(negated_ty, part);
        match r {
          Relation::Coincident => {
            changed = true;
          }
          Relation::Disjoint => {
            new_parts.insert(part);
          }
          Relation::Superset => {
            changed = true;
          }
          // fallthrough
          Relation::Subset | Relation::Intersects => {
            changed = true;
            // 契约：self.arena 为构造接线的 `Handle<TypeArena>`，get_mut 物化的独占
            // 借用仅存活至紧随的 add_type，bump arena 追加节点不移动既有块，单线程无并存别名。
            let arena = self.arena.get_mut();
            new_parts.insert(arena.add_type(IntersectionType {
              parts: alloc::vec![left, part],
            }));
          }
        }
      }

      if !changed {
        return right;
      } else if new_parts.is_empty() {
        return builtin_types.never_type;
      } else if new_parts.len() == 1 {
        // 外层 `len() == 1` 判定蕴含唯一元素存在。
        return *new_parts
          .iter()
          .next()
          .expect("外层 new_parts.len() == 1 蕴含唯一元素存在");
      } else {
        // 契约：同上——get_mut 物化构造接线 arena 的独占借用，add_type 追加 UnionType 后即随
        // 本分支尾表达式归还；bump arena 不移动既有节点，单线程无并存别名。
        let arena = self.arena.get_mut();
        return arena.add_type(UnionType {
          options: new_parts.into_iter().collect(),
        });
      }
    }

    if let Some(pt) = get_type::get::<PrimitiveType>(right)
      && pt.r#type == PrimitiveType::BOOLEAN
      && let Some(st) = get_type::get::<SingletonType>(negated_ty)
    {
      if st.variant == Variant2::V0(BooleanSingleton::new(true)) {
        return builtin_types.false_type;
      } else if st.variant == Variant2::V0(BooleanSingleton::new(false)) {
        return builtin_types.true_type;
      } else {
        // boolean & ~"hello"
        return builtin_types.boolean_type;
      }
    }

    let r = relate_type_id_type_id(negated_ty, right);

    match r {
      Relation::Disjoint => {
        // ~boolean & string
        right
      }
      // ~string & string
      // fallthrough
      Relation::Coincident => {
        // ~string & "hello"
        builtin_types.never_type
      }
      Relation::Superset => builtin_types.never_type,
      // ~string & unknown
      // ~"hello" & string
      // fallthrough
      // ~("hello" | boolean) & string
      // fallthrough
      // default
      Relation::Subset | Relation::Intersects => {
        // 契约：get_mut 物化构造接线非空 arena 的独占借用，add_type 追加 IntersectionType 后
        // 借用作尾表达式归还；bump arena 地址稳定，单线程无并存别名。
        let arena = self.arena.get_mut();
        arena.add_type(IntersectionType {
          parts: alloc::vec![left, right],
        })
      }
    }
  }
}
