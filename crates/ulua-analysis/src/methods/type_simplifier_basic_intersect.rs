use ulua_common::records::variant::Variant2;

use crate::{
  enums::{relation::Relation, table_state::TableState},
  functions::{
    follow_type, get_type, is_approximately_falsy_type::is_approximately_falsy_type,
    is_approximately_truthy_type::is_approximately_truthy_type,
    relate_simplify::relate_type_id_type_id,
  },
  records::{
    any_type::AnyType, boolean_singleton::BooleanSingleton, negation_type::NegationType,
    never_type::NeverType, primitive_type::PrimitiveType, singleton_type::SingletonType,
    table_type::TableType, type_level::TypeLevel, type_simplifier::TypeSimplifier,
    union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

impl TypeSimplifier {
  pub fn basic_intersect(&mut self, left: TypeId, right: TypeId) -> Option<TypeId> {
    // Safety: builtin_types 由 simplify_intersection* 入口从 TypeChecker 会话指针
    // 接线进本结构——非空、比 simplifier 长寿、类型检查期不再写入；在此提为一次
    // 共享借用，后续仅读 error/never/true/false 等 Copy 句柄字段。
    let builtin_types = self.builtin_types.get();
    let left = follow_type::follow(left);
    let right = follow_type::follow(right);

    if get_type::get::<AnyType>(left).is_some() && get_type::get::<ErrorType>(right).is_some() {
      return Some(right);
    }
    if get_type::get::<AnyType>(right).is_some() && get_type::get::<ErrorType>(left).is_some() {
      return Some(left);
    }
    if get_type::get::<AnyType>(left).is_some() {
      // 契约：arena 为 Handle 接线的会话 TypeArena；本分支求值后立即 return，
      // 可变借用不跨越任何可能再次触碰 arena 的调用（递归发生在后面的 table
      // 分支），单线程串行下此刻无并存借用。
      let arena = self.arena.get_mut();
      return Some(arena.add_type(UnionType {
        options: alloc::vec![right, builtin_types.error_type],
      }));
    }
    if get_type::get::<AnyType>(right).is_some() {
      // 契约：与 left 分支同理——借用随早退 return 结束，不跨后续可能重入
      // arena 的路径；Handle 契约保证目标非空、对齐、指向会话存活 arena。
      let arena = self.arena.get_mut();
      return Some(arena.add_type(UnionType {
        options: alloc::vec![left, builtin_types.error_type],
      }));
    }
    if get_type::get::<UnknownType>(left).is_some() {
      return Some(right);
    }
    if get_type::get::<UnknownType>(right).is_some() {
      return Some(left);
    }
    if get_type::get::<NeverType>(left).is_some() {
      return Some(left);
    }
    if get_type::get::<NeverType>(right).is_some() {
      return Some(right);
    }

    if let Some(pt) = get_type::get::<PrimitiveType>(left) {
      if pt.r#type == PrimitiveType::BOOLEAN {
        if let Some(st) = get_type::get::<SingletonType>(right)
          && st.variant.get_if_0().is_some()
        {
          return Some(right);
        }
        if let Some(nt) = get_type::get::<NegationType>(right)
          && let Some(st) = get_type::get::<SingletonType>(follow_type::follow(nt.ty))
          && st.variant.get_if_0().is_some()
        {
          if st.variant == Variant2::V0(BooleanSingleton::new(true)) {
            return Some(builtin_types.false_type);
          } else {
            return Some(builtin_types.true_type);
          }
        }
      }
    } else if let Some(pt) = get_type::get::<PrimitiveType>(right)
      && pt.r#type == PrimitiveType::BOOLEAN
    {
      if let Some(st) = get_type::get::<SingletonType>(left)
        && st.variant.get_if_0().is_some()
      {
        return Some(left);
      }
      if let Some(nt) = get_type::get::<NegationType>(left)
        && let Some(st) = get_type::get::<SingletonType>(follow_type::follow(nt.ty))
        && st.variant.get_if_0().is_some()
      {
        if st.variant == Variant2::V0(BooleanSingleton::new(true)) {
          return Some(builtin_types.false_type);
        } else {
          return Some(builtin_types.true_type);
        }
      }
    }

    if let (Some(lt), Some(rt)) = (
      get_type::get::<TableType>(left),
      get_type::get::<TableType>(right),
    ) {
      if lt.props.len() == 1 {
        // 外层 `len() == 1` 判定蕴含首元素存在。
        let (prop_name, left_prop) = lt
          .props
          .iter()
          .next()
          .expect("外层 lt.props.len() == 1 蕴含首元素存在");
        let left_prop_is_refinable = left_prop.is_shared() || left_prop.is_read_only();

        if let Some(right_prop) = rt.props.get(prop_name)
          && left_prop_is_refinable
          && right_prop.is_shared()
        {
          let relation = relate_type_id_type_id(
            left_prop.read_ty.expect("refinable property has read type"),
            right_prop.read_ty.expect("shared property has read type"),
          );

          match relation {
            Relation::Disjoint => return Some(builtin_types.never_type),
            Relation::Superset | Relation::Coincident => return Some(right),
            Relation::Subset => {
              if rt.props.len() == 1 && left_prop.is_shared() {
                return Some(left);
              }
            }
            Relation::Intersects => {}
          }
        }
      } else if rt.props.len() == 1 {
        return self.basic_intersect(right, left);
      }

      if lt.indexer.is_none()
        && rt.indexer.is_none()
        && lt.state == TableState::Sealed
        && rt.state == TableState::Sealed
      {
        if rt.props.is_empty() {
          return Some(left);
        }

        let are_disjoint = lt.props.keys().all(|name| !rt.props.contains_key(name));

        if are_disjoint {
          let mut merged = TableType::table_type_table_state_type_level_scope(
            TableState::Sealed,
            TypeLevel::default(),
            lt.scope,
          );
          merged.props = lt.props.clone();

          for (name, prop) in &rt.props {
            merged.props.insert(name.clone(), prop.clone());
          }

          // 契约：同早退分支——借用随 return 立即结束，本路径其后再无任何
          // arena 访问或递归（basic_intersect 的重入都发生在进入本借用之前）；
          // arena 由 Handle 契约保证指向会话存活 TypeArena。
          let arena = self.arena.get_mut();
          return Some(arena.add_type(merged));
        }
      }

      return None;
    }

    if is_approximately_truthy_type(left)
      && let Some(res) = self.basic_intersect_with_truthy(right)
    {
      return Some(res);
    }

    if is_approximately_truthy_type(right)
      && let Some(res) = self.basic_intersect_with_truthy(left)
    {
      return Some(res);
    }

    if is_approximately_falsy_type(left)
      && let Some(res) = self.basic_intersect_with_falsy(right)
    {
      return Some(res);
    }

    if is_approximately_falsy_type(right)
      && let Some(res) = self.basic_intersect_with_falsy(left)
    {
      return Some(res);
    }

    let relation = relate_type_id_type_id(left, right);
    if left == right || Relation::Coincident == relation {
      return Some(left);
    }
    match relation {
      Relation::Disjoint => Some(builtin_types.never_type),
      Relation::Subset => Some(left),
      Relation::Superset => Some(right),
      _ => None,
    }
  }
}
