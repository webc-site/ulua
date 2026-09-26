//! Source: `Analysis/src/Normalize.cpp:2711-2967` (hand-ported)
/// RAII guard mirroring C++ `RecursionCounter _rc(&sharedState->counters.recursionCount)`.
use core::ptr::null;

use ulua_common::fflag;

use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type, get_type, is_prim::is_prim, max_scope::max,
    simplify_intersection_simplify::simplify_intersection,
  },
  records::{
    any_type::AnyType, metatable_type::MetatableType, never_type::NeverType,
    normalizer::Normalizer, primitive_type::Type as PrimType, property_type::Property,
    recursion_counter::RecursionCounter, table_indexer::TableIndexer, table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn intersection_of_tables(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
    self.consume_fuel();

    if here == there {
      return Some(here);
    }

    // 契约：shared_state 为构造期（或 TypeChecker Box 定址时）接线的
    // `Option<Handle<UnifierSharedState>>`，非空断言收在 shared_state_mut/ref；
    // 这里取 `counters.recursion_count` 交给 RAII 计数器，不留下跨语句的
    // `&mut`，因此下面再经 shared_state_ref 读 counters 不构成别名冲突。
    let _rc = RecursionCounter::recursion_counter_i32(
      &mut self.shared_state_mut().counters.recursion_count,
    );
    // 承接上一块——计数器已被 `_rc` 递增后才读取，两次读取都经接线断言直达
    // i32 字段（Copy），单线程顺序执行且此刻无其它借用者写 counters。
    let (recursion_limit, recursion_count) = {
      let state = self.shared_state_ref();
      (
        state.counters.recursion_limit,
        state.counters.recursion_count,
      )
    };
    if recursion_limit > 0 && recursion_limit < recursion_count {
      return None;
    }

    if is_prim(here, PrimType::Table) {
      return Some(there);
    } else if is_prim(there, PrimType::Table) {
      return Some(here);
    }

    if get_type::get::<NeverType>(here).is_some() {
      return Some(there);
    } else if get_type::get::<NeverType>(there).is_some() {
      return Some(here);
    } else if get_type::get::<AnyType>(here).is_some() {
      return Some(there);
    } else if get_type::get::<AnyType>(there).is_some() {
      return Some(here);
    }

    let mut htable = here;
    let mut hmtable: TypeId = null();
    if let Some(hmtv) = get_type::get::<MetatableType>(here) {
      htable = follow_type::follow(hmtv.table());
      hmtable = follow_type::follow(hmtv.metatable());
    }
    let mut ttable = there;
    let mut tmtable: TypeId = null();
    if let Some(tmtv) = get_type::get::<MetatableType>(there) {
      ttable = follow_type::follow(tmtv.table());
      tmtable = follow_type::follow(tmtv.metatable());
    }

    let httv = get_type::get::<TableType>(htable)?;
    let tttv = get_type::get::<TableType>(ttable)?;

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
                let ty = simplify_intersection(
                  self.builtin_types,
                  self.arena.expect("契约：归一化期 arena 已接线"),
                  hread,
                  tread,
                )
                .result;

                // If any property is going to get mapped to `never`, we can just call the entire table `never`.
                if get_type::get::<NeverType>(ty).is_some() {
                  // Safety: `self.builtin_types.as_ptr()` 为构造期注入、只读的 BuiltinTypes 地址；
                  // 该属性分支只 Copy 其 never_type 句柄返回，不写该表，也不与刚借用过的
                  // httv/tttv（arena 只读节点）重叠。
                  return Some({ self.builtin_types.get().never_type });
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
                let w = simplify_intersection(
                  self.builtin_types,
                  self.arena.expect("契约：归一化期 arena 已接线"),
                  hwrite,
                  twrite,
                )
                .result;
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
        result
          .get_or_insert_with(|| {
            TableType::table_type_table_state_type_level_scope(state, level, scope)
          })
          .props
          .insert(name.clone(), prop);
      }
    }

    for (name, tprop) in tttv.props.iter() {
      if !httv.props.contains_key(name) {
        result
          .get_or_insert_with(|| {
            TableType::table_type_table_state_type_level_scope(state, level, scope)
          })
          .props
          .insert(name.clone(), tprop.clone());
        here_sub_there = false;
      }
    }

    if let (Some(hindexer), Some(tindexer)) = (&httv.indexer, &tttv.indexer) {
      if fflag::LuauReadOnlyIndexers.get() {
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

        result
          .get_or_insert_with(|| {
            TableType::table_type_table_state_type_level_scope(state, level, scope)
          })
          .indexer = Some(idx);
      } else {
        // TODO: What should intersection of indexes be?
        let index = self.union_type(hindexer.index_type, tindexer.index_type);
        let index_result =
          self.intersection_type(hindexer.index_result_type, tindexer.index_result_type);
        let table = result.get_or_insert_with(|| {
          TableType::table_type_table_state_type_level_scope(state, level, scope)
        });
        table.indexer = Some(TableIndexer {
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
      result
        .get_or_insert_with(|| {
          TableType::table_type_table_state_type_level_scope(state, level, scope)
        })
        .indexer = httv.indexer;
      there_sub_here = false;
    } else if tttv.indexer.is_some() {
      result
        .get_or_insert_with(|| {
          TableType::table_type_table_state_type_level_scope(state, level, scope)
        })
        .indexer = tttv.indexer;
      here_sub_there = false;
    }

    let table: TypeId;
    if here_sub_there {
      table = htable;
    } else if there_sub_here {
      table = ttable;
    } else if let Some(tt) = result {
      // 契约：`self.arena` 归一化期已接线（wired_arena_mut 断言）且存活；把上面
      // 合并出的 TableType 落盘成一个新节点，可变借用止于本语句——`tt` 是本地拥有值
      // （前面 `Option<TableType>` 已被 move），httv/tttv 只读自另一批 arena 节点，
      // add_type 不会使它们失效。
      table = self.wired_arena_mut().add_type(tt);
    } else {
      // 契约：同一 arena 接线不变量；空属性交集退化为新建一个空表节点。
      table = self
        .wired_arena_mut()
        .add_type(TableType::table_type_table_state_type_level_scope(
          state, level, scope,
        ));
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
            // Safety: 双侧都有元表时，`mtable` 来自上面递归调用返回的新 arena 句柄；
            // 递归已返回并交还 `&mut self`，此处再向存活的 arena 追加一个 MetatableType，
            // 借用瞬态且不并存。
            Some(self.wired_arena_mut().add_type(MetatableType {
              table,
              metatable: mtable,
              synthetic_name: None,
            }))
          }
        }
        None => None,
      }
    } else if !hmtable.is_null() {
      if table == htable {
        Some(here)
      } else {
        // Safety: 仅 here 侧带元表：table 是本次刚算出的 arena 句柄（或原 htable），
        // 与 hmtable 同处存活 arena；追加节点的可变借用止于本表达式。
        Some(self.wired_arena_mut().add_type(MetatableType {
          table,
          metatable: hmtable,
          synthetic_name: None,
        }))
      }
    } else if !tmtable.is_null() {
      if table == ttable {
        Some(there)
      } else {
        // Safety: there 侧带元表的对称分支，同上——arena 存活、借用单发且不重叠。
        Some(self.wired_arena_mut().add_type(MetatableType {
          table,
          metatable: tmtable,
          synthetic_name: None,
        }))
      }
    } else {
      Some(table)
    }
  }
}
