//! Source: `Analysis/src/Normalize.cpp:2711-2967` (hand-ported)
/// RAII guard mirroring C++ `RecursionCounter _rc(&sharedState->counters.recursionCount)`.
use core::ptr::null;

use ulua_common::FFlag;

use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_prim::is_prim, max_scope::max,
    simplify_intersection_simplify::simplify_intersection,
  },
  records::{
    any_type::AnyType, metatable_type::MetatableType, never_type::NeverType,
    normalizer::Normalizer, primitive_type::Type as PrimType, property_type::Property,
    table_indexer::TableIndexer, table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};
struct RcGuard {
  count: *mut i32,
}

impl RcGuard {
  fn new(count: *mut i32) -> Self {
    unsafe {
      *count += 1;
    }
    RcGuard { count }
  }
}

impl Drop for RcGuard {
  fn drop(&mut self) {
    unsafe {
      *self.count -= 1;
    }
  }
}

impl Normalizer {
  pub fn intersection_of_tables(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
    self.consume_fuel();

    if here == there {
      return Some(here);
    }

    let _rc = RcGuard::new(unsafe { &mut (*self.shared_state).counters.recursion_count });
    let recursion_limit = unsafe { (*self.shared_state).counters.recursion_limit };
    let recursion_count = unsafe { (*self.shared_state).counters.recursion_count };
    if recursion_limit > 0 && recursion_limit < recursion_count {
      return None;
    }

    if is_prim(here, PrimType::Table) {
      return Some(there);
    } else if is_prim(there, PrimType::Table) {
      return Some(here);
    }

    if get_type_id::<NeverType>(here).is_some() {
      return Some(there);
    } else if get_type_id::<NeverType>(there).is_some() {
      return Some(here);
    } else if get_type_id::<AnyType>(here).is_some() {
      return Some(there);
    } else if get_type_id::<AnyType>(there).is_some() {
      return Some(here);
    }

    let mut htable = here;
    let mut hmtable: TypeId = null();
    if let Some(hmtv) = get_type_id::<MetatableType>(here) {
      htable = follow_type_id(hmtv.table());
      hmtable = follow_type_id(hmtv.metatable());
    }
    let mut ttable = there;
    let mut tmtable: TypeId = null();
    if let Some(tmtv) = get_type_id::<MetatableType>(there) {
      ttable = follow_type_id(tmtv.table());
      tmtable = follow_type_id(tmtv.metatable());
    }

    let httv = get_type_id::<TableType>(htable)?;
    let tttv = get_type_id::<TableType>(ttable)?;

    if httv.state == TableState::Free || tttv.state == TableState::Free {
      return None;
    }
    if httv.state == TableState::Generic || tttv.state == TableState::Generic {
      return None;
    }

    let mut state = httv.state;
    if tttv.state == TableState::Unsealed {
      state = tttv.state;
    }

    // TypeLevel max(a, b) == if a.subsumes(b) { b } else { a }   (Unifiable.h:62)
    let level = if httv.level.subsumes(&tttv.level) {
      tttv.level
    } else {
      httv.level
    };
    // Scope* max(a, b)
    let scope = max(httv.scope, tttv.scope);

    let mut result: Option<TableType> = None;
    let mut here_sub_there = true;
    let mut there_sub_here = true;

    for (name, hprop) in httv.props.iter() {
      let mut prop: Property = hprop.clone();
      let tfound = tttv.props.get(name);
      match tfound {
        None => {
          there_sub_here = false;
        }
        Some(tprop) => {
          // TODO: variance issues here, which can't be fixed until we have read/write property types
          if self.use_new_luau_solver() {
            if let Some(hread) = hprop.read_ty {
              if let Some(tread) = tprop.read_ty {
                let ty = simplify_intersection(self.builtin_types, self.arena, hread, tread).result;

                // If any property is going to get mapped to `never`, we can just call the entire table `never`.
                if get_type_id::<NeverType>(ty).is_some() {
                  return Some(unsafe { (*self.builtin_types).never_type });
                }

                prop.read_ty = Some(ty);
                here_sub_there &= ty == hread;
                there_sub_here &= ty == tread;
              } else {
                prop.read_ty = Some(hread);
                there_sub_here = false;
              }
            } else if let Some(tread) = tprop.read_ty {
              prop.read_ty = Some(tread);
              here_sub_there = false;
            }

            if let Some(hwrite) = hprop.write_ty {
              if let Some(twrite) = tprop.write_ty {
                let w =
                  simplify_intersection(self.builtin_types, self.arena, hwrite, twrite).result;
                prop.write_ty = Some(w);
                here_sub_there &= w == hwrite;
                there_sub_here &= w == twrite;
              } else {
                prop.write_ty = Some(hwrite);
                there_sub_here = false;
              }
            } else if let Some(twrite) = tprop.write_ty {
              prop.write_ty = Some(twrite);
              here_sub_there = false;
            }
          } else {
            let h_dep = hprop.type_deprecated();
            let t_dep = tprop.type_deprecated();
            let inter = self.intersection_type(h_dep, t_dep);
            prop.set_type(inter);
            here_sub_there &= prop.type_deprecated() == h_dep;
            there_sub_here &= prop.type_deprecated() == t_dep;
          }
        }
      }

      // TODO: string indexers

      if prop.read_ty.is_some() || prop.write_ty.is_some() {
        if result.is_none() {
          result = Some(TableType::table_type_table_state_type_level_scope(
            state, level, scope,
          ));
        }
        result.as_mut().unwrap().props.insert(name.clone(), prop);
      }
    }

    for (name, tprop) in tttv.props.iter() {
      if !httv.props.contains_key(name) {
        if result.is_none() {
          result = Some(TableType::table_type_table_state_type_level_scope(
            state, level, scope,
          ));
        }
        result
          .as_mut()
          .unwrap()
          .props
          .insert(name.clone(), tprop.clone());
        here_sub_there = false;
      }
    }

    if let (Some(hindexer), Some(tindexer)) = (&httv.indexer, &tttv.indexer) {
      if FFlag::LuauReadOnlyIndexers.get() {
        let index = self.union_type(hindexer.index_type, tindexer.index_type);
        let mut idx = TableIndexer {
          index_type: index,
          index_result_type: null(),
          is_read_only: false,
        };

        if hindexer.is_read_only && tindexer.is_read_only {
          // Both read-only: covariant -> intersect values, keep read-only.
          idx.index_result_type =
            self.intersection_type(hindexer.index_result_type, tindexer.index_result_type);
          idx.is_read_only = true;
        } else {
          idx.index_result_type =
            self.intersection_type(hindexer.index_result_type, tindexer.index_result_type);
        }

        let here_mode_match = hindexer.is_read_only == idx.is_read_only;
        let there_mode_match = tindexer.is_read_only == idx.is_read_only;
        here_sub_there &= here_mode_match
          && (hindexer.index_type == index)
          && (hindexer.index_result_type == idx.index_result_type);
        there_sub_here &= there_mode_match
          && (tindexer.index_type == index)
          && (tindexer.index_result_type == idx.index_result_type);

        if result.is_none() {
          result = Some(TableType::table_type_table_state_type_level_scope(
            state, level, scope,
          ));
        }
        result.as_mut().unwrap().indexer = Some(idx);
      } else {
        // TODO: What should intersection of indexes be?
        let index = self.union_type(hindexer.index_type, tindexer.index_type);
        let index_result =
          self.intersection_type(hindexer.index_result_type, tindexer.index_result_type);
        if result.is_none() {
          result = Some(TableType::table_type_table_state_type_level_scope(
            state, level, scope,
          ));
        }
        result.as_mut().unwrap().indexer = Some(TableIndexer {
          index_type: index,
          index_result_type: index_result,
          is_read_only: false,
        });
        here_sub_there &=
          (hindexer.index_type == index) && (hindexer.index_result_type == index_result);
        there_sub_here &=
          (tindexer.index_type == index) && (tindexer.index_result_type == index_result);
      }
    } else if httv.indexer.is_some() {
      if result.is_none() {
        result = Some(TableType::table_type_table_state_type_level_scope(
          state, level, scope,
        ));
      }
      result.as_mut().unwrap().indexer = httv.indexer;
      there_sub_here = false;
    } else if tttv.indexer.is_some() {
      if result.is_none() {
        result = Some(TableType::table_type_table_state_type_level_scope(
          state, level, scope,
        ));
      }
      result.as_mut().unwrap().indexer = tttv.indexer;
      here_sub_there = false;
    }

    let table: TypeId;
    if here_sub_there {
      table = htable;
    } else if there_sub_here {
      table = ttable;
    } else if let Some(tt) = result {
      table = unsafe { (*self.arena).add_type(tt) };
    } else {
      table = unsafe {
        (*self.arena).add_type(TableType::table_type_table_state_type_level_scope(
          state, level, scope,
        ))
      };
    }

    if !tmtable.is_null() && !hmtable.is_null() {
      // NOTE: this assumes metatables are ivariant
      match self.intersection_of_tables(hmtable, tmtable) {
        Some(mtable) => {
          if table == htable && mtable == hmtable {
            Some(here)
          } else if table == ttable && mtable == tmtable {
            Some(there)
          } else {
            Some(unsafe {
              (*self.arena).add_type(MetatableType {
                table,
                metatable: mtable,
                synthetic_name: None,
              })
            })
          }
        }
        None => None,
      }
    } else if !hmtable.is_null() {
      if table == htable {
        Some(here)
      } else {
        Some(unsafe {
          (*self.arena).add_type(MetatableType {
            table,
            metatable: hmtable,
            synthetic_name: None,
          })
        })
      }
    } else if !tmtable.is_null() {
      if table == ttable {
        Some(there)
      } else {
        Some(unsafe {
          (*self.arena).add_type(MetatableType {
            table,
            metatable: tmtable,
            synthetic_name: None,
          })
        })
      }
    } else {
      Some(table)
    }
  }
}
