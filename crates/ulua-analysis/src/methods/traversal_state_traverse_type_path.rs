use alloc::{string::String, vec::Vec};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{pack_field::PackField, type_field::TypeField},
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    begin_type_pack::begin,
    end_type_pack::end,
    flatten_type_pack::flatten,
    follow_type,
    get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_type,
    get_type_or_pack::{
      get_type_or_pack as get_type_or_pack_ty, get_type_or_pack_mut as get_type_or_pack,
      get_type_or_pack_mut_2,
    },
    lookup_extern_type_prop::lookup_extern_type_prop,
  },
  records::{
    extern_type::ExternType, free_type::FreeType, function_type::FunctionType,
    generic_pack_mapping::GenericPackMapping, index::Index, intersection_type::IntersectionType,
    metatable_type::MetatableType, negation_type::NegationType, pack_slice::PackSlice,
    property_type::Property as PropertyType, property_type_path::Property, reduction::Reduction,
    table_indexer::TableIndexer, table_type::TableType, traversal_state::TraversalState,
    txn_log::TxnLog, type_pack::TypePack, union_type::UnionType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId, type_pack_id::TypePackId},
};

impl TraversalState {
  pub fn traverse_type_path_property(&mut self, property: &Property) -> bool {
    // `get_type_or_pack` 系列现为 `Option` 形态（null 哨兵即未命中之像）。
    if get_type_or_pack::<TypeId>(&self.current).is_none() {
      return false;
    }

    if self.check_invariants() {
      return false;
    }

    let mut prop: Option<&PropertyType> = None;

    let t = get_type_or_pack_ty::<TableType>(&self.current);
    if let Some(t) = t {
      if let Some(found) = t.props.get(&property.name) {
        prop = Some(found);
      }
    } else {
      let c = get_type_or_pack_ty::<ExternType>(&self.current);
      if let Some(c) = c {
        prop = lookup_extern_type_prop(c, &property.name);
      } else {
        let m = get_type_or_pack_ty::<MetatableType>(&self.current);
        if let Some(m) = m {
          // For a metatable type, the table takes priority; check that
          // before falling through to the metatable entry below.
          let pinned = self.current.clone();
          self.update_current_type_id(m.table);

          if self.traverse_type_path_property(property) {
            return true;
          }

          // Restore the old current type if we didn't traverse the
          // metatable successfully; we'll use the next branch to
          // address this.
          self.current = pinned;
        }
      }
    }

    if prop.is_none() {
      // `current_type` was captured before any `update_current`; re-read
      // the current type to mirror the C++ `*currentType` dereference.
      // `self.builtin_types` 由构造期从调用方 `&BuiltinTypes` 转成，遍历存续期间
      // 引用一直有效且非空（真 host 边界，保留窄 `unsafe`）。
      let cur_ty = get_type_or_pack::<TypeId>(&self.current).copied();
      if let Some(cur_ty) = cur_ty
        && let Some(m) = get_metatable_type_id_not_null_builtin_types(
          cur_ty,
          // SAFETY: `self.builtin_types` 为构造注入的非空 &BuiltinTypes，遍历期存活。
          unsafe { &*self.builtin_types },
        )
      {
        // Weird: rather than use findMetatableEntry, which requires a
        // lot of stuff that we don't have and don't want to pull in,
        // we use the path traversal logic to grab __index and then
        // re-enter the lookup logic there.
        self.update_current_type_id(m);

        // C++ `TypePath::Property::read("__index")` — a read-typed
        // path component for the `__index` property.
        let index_prop = Property {
          name: String::from("__index"),
          is_read: true,
        };
        if !self.traverse_type_path_property(&index_prop) {
          return false;
        }

        return self.traverse_type_path_property(property);
      }
    }

    if let Some(prop) = prop {
      let maybe_type = if property.is_read {
        prop.read_ty
      } else {
        prop.write_ty
      };

      if let Some(ty) = maybe_type {
        self.update_current_type_id(ty);
        return true;
      }
    }

    false
  }

  pub fn traverse_type_path_index(&mut self, index: &Index) -> bool {
    if self.check_invariants() {
      return false;
    }

    let current_type = get_type_or_pack::<TypeId>(&self.current);
    if let Some(&current_type) = current_type {
      let current_type_id = follow_type::follow(current_type);
      let mut updated_current = false;

      if get_type::get::<ErrorType>(current_type_id).is_some() {
        self.encountered_error_suppression = true;
        return false;
      }

      if let Some(u) = get_type::get::<UnionType>(current_type_id) {
        // We want to track the index that updates the current type with
        // `idx` while still iterating through the entire union to check
        // for error types.
        for (idx, opt_ty) in begin_union_type(u).enumerate() {
          if get_type::get::<ErrorType>(opt_ty).is_some() {
            self.encountered_error_suppression = true;
          }
          if idx == index.index {
            self.update_current_type_id(opt_ty);
            updated_current = true;
          }
        }
      } else if let Some(i) = get_type::get::<IntersectionType>(current_type_id) {
        for (idx, part_ty) in begin_intersection_type(i).enumerate() {
          if get_type::get::<ErrorType>(part_ty).is_some() {
            self.encountered_error_suppression = true;
          }
          if idx == index.index {
            self.update_current_type_id(part_ty);
            updated_current = true;
          }
        }
      }

      updated_current
    } else {
      let current_pack = get_type_or_pack::<TypePackId>(&self.current).copied();
      LUAU_ASSERT!(current_pack.is_some());
      let is_pack = get_type_or_pack_mut_2::<TypePack>(&self.current).is_some();
      if is_pack && let Some(cp) = current_pack {
        // 条件推进后再读取（非纯遍历，无法直接 for-in），保留三件套写法；
        // TypePackIterator 的 std Iterator 仅供纯遍历调用点使用。
        let mut it = begin(cp);
        let mut i: usize = 0;
        while i < index.index && it != end(cp) {
          it.advance();
          i += 1;
        }

        if it != end(cp) {
          self.update_current_type_id(*it.current());
          return true;
        }
      }

      false
    }
  }

  pub fn traverse_type_path_type_field(&mut self, field: TypeField) -> bool {
    if self.check_invariants() {
      return false;
    }

    match field {
      TypeField::Table => {
        let mt = get_type_or_pack_ty::<MetatableType>(&self.current);
        if let Some(mt) = mt {
          self.update_current_type_id(mt.table);
          return true;
        }
        false
      }
      TypeField::Metatable => {
        let current_type = get_type_or_pack::<TypeId>(&self.current).copied();
        // `self.builtin_types` 为构造注入的非空 &BuiltinTypes，遍历期存活
        // （真 host 边界，保留窄 `unsafe`）。
        if let Some(current_type) = current_type
          && let Some(mt) = get_metatable_type_id_not_null_builtin_types(
            current_type,
            // SAFETY: 同上，`self.builtin_types` 非空且遍历期存活。
            unsafe { &*self.builtin_types },
          )
        {
          self.update_current_type_id(mt);
          return true;
        }
        false
      }
      TypeField::LowerBound | TypeField::UpperBound => {
        let ft = get_type_or_pack_ty::<FreeType>(&self.current);
        if let Some(ft) = ft {
          let target = if field == TypeField::LowerBound {
            ft.lower_bound
          } else {
            ft.upper_bound
          };
          self.update_current_type_id(target);
          return true;
        }
        false
      }
      TypeField::IndexLookup | TypeField::IndexResult => {
        let mut indexer: Option<&TableIndexer> = None;

        let tt = get_type_or_pack_ty::<TableType>(&self.current);
        if let Some(tab) = tt
          && tab.indexer.is_some()
        {
          indexer = tab.indexer.as_ref();
        } else {
          let mt = get_type_or_pack_ty::<MetatableType>(&self.current);
          if let Some(mt) = mt {
            // `.table`/`.metatable` 是 arena 内存活类型 id，follow/get_type_id
            // 均只读解链。
            let mt_tab = get_type::get::<TableType>(follow_type::follow(mt.table));
            if let Some(tab) = mt_tab
              && tab.indexer.is_some()
            {
              indexer = tab.indexer.as_ref();
            } else {
              let mt_mt = get_type::get::<TableType>(follow_type::follow(mt.metatable));
              if let Some(mt_tab) = mt_mt
                && mt_tab.indexer.is_some()
              {
                indexer = mt_tab.indexer.as_ref();
              }
            }
          } else {
            // Note: we don't appear to walk the class hierarchy for
            // indexers
            let ct = get_type_or_pack_ty::<ExternType>(&self.current);
            if let Some(ct) = ct {
              indexer = ct.indexer.as_ref();
            }
          }
        }

        if let Some(indexer) = indexer {
          let target = if field == TypeField::IndexLookup {
            indexer.index_type
          } else {
            indexer.index_result_type
          };
          self.update_current_type_id(target);
          return true;
        }
        false
      }
      TypeField::Negated => {
        let nt = get_type_or_pack_ty::<NegationType>(&self.current);
        if let Some(nt) = nt {
          self.update_current_type_id(nt.ty);
          return true;
        }
        false
      }
      TypeField::Variadic => {
        let vtp = get_type_or_pack_mut_2::<VariadicTypePack>(&self.current);
        if let Some(vtp) = vtp {
          self.update_current_type_id(vtp.ty);
          return true;
        }
        false
      }
    }
  }

  pub fn traverse_type_path_reduction(&mut self, reduction: Reduction) -> bool {
    if self.check_invariants() {
      return false;
    }
    self.update_current_type_id(reduction.result_type);
    true
  }

  pub fn traverse_type_path_pack_field(&mut self, field: PackField) -> bool {
    if self.check_invariants() {
      return false;
    }

    match field {
      PackField::Arguments | PackField::Returns => {
        let ft = get_type_or_pack_ty::<FunctionType>(&self.current);
        if let Some(ft) = ft {
          let target = if field == PackField::Arguments {
            ft.arg_types
          } else {
            ft.ret_types
          };
          self.update_current_type_pack_id(target);
          return true;
        }
        false
      }
      PackField::Tail => {
        let current_pack = get_type_or_pack::<TypePackId>(&self.current).copied();
        if let Some(cp) = current_pack {
          let mut it = begin(cp);
          while it != end(cp) {
            it.advance();
          }

          if let Some(tail) = it.tail() {
            self.update_current_type_pack_id(tail);
            return true;
          }
        }
        false
      }
    }
  }

  pub fn traverse_type_path_pack_slice(&mut self, slice: PackSlice) -> bool {
    if self.check_invariants() {
      return false;
    }

    let Some(cp) = get_type_or_pack::<TypePackId>(&self.current).copied() else {
      return false;
    };

    // Safety: `TxnLog::empty()` 返回 OnceLock 持有的进程寿只读单例地址（其
    // Sync/Send 证成见 txn_log_empty），转成 & 引用恒有效；flatten 对其只读。
    let (flat_head, flat_tail) = flatten(cp, unsafe { &*TxnLog::empty() });

    if flat_head.len() <= slice.start_index {
      return false;
    }

    // C++ walks `begin(flatHead)` advanced by `start_index` to `end(flatHead)`;
    // `flatHead` is a plain vector, so we slice it directly.
    let head_slice: Vec<TypeId> = flat_head.iter().skip(slice.start_index).copied().collect();

    // Safety: `self.arena` 由构造 helper 从调用方（traverse_type_root/
    // traverse_type_pack_root）传入的 `&mut TypeArena` 裸化而来；TraversalState 是
    // 该借用存续期内的局部值，此处经指针取回独占可变借用做追加分配，正是调用方
    // 交出 &mut 后唯一的写者，无并发 alias。
    let pack_slice = self
      .arena
      .get_mut()
      .add_type_pack_vector_type_id_optional_type_pack_id(head_slice, flat_tail);

    self.update_current_type_pack_id(pack_slice);

    true
  }

  pub fn traverse_type_path_generic_pack_mapping(&mut self, mapping: GenericPackMapping) -> bool {
    if self.check_invariants() {
      return false;
    }
    self.update_current_type_pack_id(mapping.mapped_type);
    true
  }
}
