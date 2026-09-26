use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    follow_type,
    get_mutable_type::get_mutable,
    get_type,
    is_approximately_falsy_type::is_approximately_falsy_type,
    is_approximately_truthy_type::is_approximately_truthy_type,
    is_type_variable::is_type_variable,
    shallow_clone_clone::shallow_clone,
  },
  records::{
    any_type::AnyType, clone_state::CloneState, intersection_type::IntersectionType,
    negation_type::NegationType, never_type::NeverType, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType, type_ids::TypeIds,
    type_simplifier::TypeSimplifier, union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

impl TypeSimplifier {
  pub fn intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
    &self,
    target: TypeId,
    discriminant: TypeId,
    seen: &mut DenseHashSet<TypeId>,
  ) -> Option<TypeId> {
    // Safety: `self.builtin_types` 对应 C++ `NotNull<BuiltinTypes>`——构造期即
    // 非空、指向分析期全局唯一的 BuiltinTypes 单例且从不被改写；此处共享借用
    // 只读其内建类型句柄，与后续各 `&mut arena` 短借用指向不同对象。
    let builtin_types = self.builtin_types.get();
    if seen.contains(&target) {
      return None;
    }
    let target = follow_type::follow(target);
    let discriminant = follow_type::follow(discriminant);
    // C++ 只在 follow 前查一次 seen；这里补一次 follow 后的检查，
    // 用于挡住经 BoundType 别名指回自身的环状 union/intersection
    // （C++ 原实现对这类环会无限递归）。
    if seen.contains(&target) {
      return None;
    }

    if let Some(ut) = get_type::get::<UnionType>(target) {
      seen.insert(target);
      let mut options = TypeIds::new();
      // C++ 的 `for (TypeId option : ut)` 走防环且展平嵌套 union 的 TypeIterator。
      for option in begin_union_type(ut) {
        let res = self.intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
          option,
          discriminant,
          seen,
        )?;
        if get_type::get::<UnknownType>(res).is_some() {
          return Some(builtin_types.unknown_type);
        }
        if get_type::get::<NeverType>(res).is_some() {
          continue;
        }
        options.insert_type_id(res);
      }
      if options.empty() {
        return Some(builtin_types.never_type);
      }
      if options.size() == 1 {
        return Some(options.front());
      }
      // Safety: `self.arena` 对应 C++ `NotNull<TypeArena>`（module 的
      // internalTypes），非空且随 module 存活；分析为单线程，本短借用仅覆盖
      // 这一次 add_type 写入，结束于语句末，期间无其他对 arena 的活动借用。
      let arena = self.arena.get_mut();
      return Some(arena.add_type(UnionType {
        options: options.take(),
      }));
    }

    if let Some(it) = get_type::get::<IntersectionType>(target) {
      seen.insert(target);
      let mut parts = TypeIds::new();
      // C++ 的 `for (TypeId part : it)` 走防环且展平嵌套 intersection 的 TypeIterator。
      for part in begin_intersection_type(it) {
        let res = self.intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
          part,
          discriminant,
          seen,
        )?;

        if get_type::get::<NeverType>(res).is_some() {
          return Some(builtin_types.never_type);
        }

        if let Some(sub_intersection) = get_type::get::<IntersectionType>(res) {
          for sub_option in begin_intersection_type(sub_intersection) {
            if get_type::get::<NeverType>(sub_option).is_some() {
              return Some(builtin_types.never_type);
            }
            if get_type::get::<UnknownType>(sub_option).is_none() {
              parts.insert_type_id(sub_option);
            }
          }
        } else if get_type::get::<UnknownType>(res).is_none() {
          parts.insert_type_id(res);
        }
      }

      if parts.empty() {
        return Some(builtin_types.unknown_type);
      }
      if parts.size() == 1 {
        return Some(parts.front());
      }
      // Safety: `self.arena` 对应 C++ `NotNull<TypeArena>`，非空且随 module
      // 存活；短借用仅覆盖本次 add_type，与 union 分支的同源借用互斥不同路径，
      // 单线程下写入时无其他活动借用。
      let arena = self.arena.get_mut();
      return Some(arena.add_type(IntersectionType {
        parts: parts.take(),
      }));
    }

    if let Some(ttv) = get_type::get::<TableType>(target)
      && let Some(disc_ttv) = get_type::get::<TableType>(discriminant)
    {
      // The precondition of this function is that `discriminant` is
      // simple, so if it's a table it *must* be a sealed table with
      // a single property and no indexer.
      LUAU_ASSERT!(disc_ttv.props.len() == 1 && disc_ttv.indexer.is_none());
      // 紧邻 LUAU_ASSERT(len()==1) 蕴含首元素存在。
      let (disc_prop_name, disc_prop) = disc_ttv
        .props
        .iter()
        .next()
        .expect("紧邻 LUAU_ASSERT(props.len() == 1) 蕴含首元素存在");
      if let Some(ty_prop) = ttv.props.get(disc_prop_name) {
        let property = self.intersect_property(ty_prop, disc_prop, seen)?;
        if let Some(read_ty) = property.read_ty
          && get_type::get::<NeverType>(follow_type::follow(read_ty)).is_some()
        {
          return Some(builtin_types.never_type);
        }
        if let Some(write_ty) = property.write_ty
          && get_type::get::<NeverType>(follow_type::follow(write_ty)).is_some()
        {
          return Some(builtin_types.never_type);
        }

        // If the property we get back is pointer identical to the
        // original property, return the underlying property as an
        // optimization.
        if ty_prop.read_ty == property.read_ty && ty_prop.write_ty == property.write_ty {
          return Some(target);
        }

        // 契约：C++ CloneState 形参即非 const `BuiltinTypes&`；句柄物化的
        // `&mut` 只为匹配 `CloneState::new` 签名，借用半径止于本次调用，且本
        // 分支随后立即 return，不与函数头 `builtin_types` 只读借用重叠。
        let mut cs = CloneState::new(self.builtin_types.get_mut());
        // Safety: `target` 为 follow 后的存活 arena 类型句柄；TypeArena 分配是
        // 追加式（TypedAllocator 块内地址稳定），shallow_clone 只新建克隆节点，
        // 不使 ttv/disc_ttv 等既有只读借用失效，且它们指向原节点、写入落在新
        // 节点，单线程下无重叠可写别名；`cs` 为本函数独占的局部克隆态。
        let result = unsafe {
          shallow_clone(
            target,
            self.arena.get_mut(),
            &mut cs,
            /* clonePersistentTypes */ true,
          )
        };
        // C++: LUAU_ASSERT(resultTtv)
        let result_ttv = get_mutable::<TableType>(result).expect("shallow_clone yields TableType");
        result_ttv.props.insert(disc_prop_name.clone(), property);
        // Shallow cloning clears out scopes, so let's put back the
        // scope from the original type.
        result_ttv.scope = ttv.scope;
        return Some(result);
      }

      // 契约：与 props.get 命中分支同一论证——句柄物化的 &mut 仅为
      // CloneState::new 的签名形态，半径止于本调用；该分支直接 return。
      let mut cs = CloneState::new(self.builtin_types.get_mut());
      // Safety: 同 props.get 命中分支——arena 追加式分配只新建克隆节点，
      // 不使 disc_ttv/ttv 等既有只读借用失效；target/cs 均为本作用域内有效
      // 句柄与独占局部状态。
      let result = unsafe {
        shallow_clone(
          target,
          self.arena.get_mut(),
          &mut cs,
          /* clonePersistentTypes */ true,
        )
      };
      // C++: LUAU_ASSERT(resultTtv)
      let result_ttv = get_mutable::<TableType>(result).expect("shallow_clone yields TableType");
      // C++ `props.emplace` only inserts if the key is absent; the
      // `ty_prop` lookup above already established it is absent here.
      result_ttv
        .props
        .entry(disc_prop_name.clone())
        .or_insert_with(|| disc_prop.clone());
      // Shallow cloning clears out scopes, so let's put back the
      // scope from the original type.
      result_ttv.scope = ttv.scope;
      return Some(result);
    }

    // At this point, we're doing something like:
    //
    //  { ... } & ~nil
    //
    // Which can be handled via fallthrough.

    if is_type_variable(target) || get_type::get::<TypeFunctionInstanceType>(target).is_some() {
      return None;
    }

    if is_approximately_truthy_type(discriminant) {
      return self.basic_intersect_with_truthy(target);
    }
    if is_approximately_truthy_type(target) {
      return self.basic_intersect_with_truthy(discriminant);
    }
    if is_approximately_falsy_type(discriminant) {
      return self.basic_intersect_with_falsy(target);
    }
    if is_approximately_falsy_type(target) {
      return self.basic_intersect_with_falsy(discriminant);
    }

    if get_type::get::<AnyType>(target).is_some() {
      // Safety: 同 union/intersection 分支——arena 非空存活，追加式分配只写
      // 新节点；本短借用止于 add_type 返回，单线程无其他活动可写借用。
      let arena = self.arena.get_mut();
      return Some(arena.add_type(UnionType {
        options: alloc::vec![builtin_types.error_type, discriminant],
      }));
    }
    if get_type::get::<ErrorType>(target).is_some() {
      return Some(builtin_types.error_type);
    }
    if let Some(nty) = get_type::get::<NegationType>(discriminant) {
      return self.subtract_one(target, nty.ty);
    }

    self.intersect_one(target, discriminant)
  }
}
