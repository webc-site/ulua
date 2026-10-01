use alloc::vec::Vec;

use hashbrown::HashSet;
use ulua_analysis::{
  functions::{follow_type, get_type, merge as merge_analysis},
  records::{type_arena::TypeArena, union_type::UnionType},
  type_aliases::{refinement_map::RefinementMap, type_id::TypeId},
};

pub fn merge(arena: &mut TypeArena, l: &mut RefinementMap, r: &RefinementMap) {
  let arena = arena as *mut TypeArena;

  merge_analysis::merge(l, r, |a, b| {
    // 单趟去重展开：union 成员直取全局类型表借用（零拷贝），非 union 折叠为
    // 自身；先 a 后 b，首个出现者胜出。
    let mut seen: HashSet<TypeId> = HashSet::default();
    let mut options: Vec<TypeId> = Vec::new();
    for ty in [a, b] {
      match get_type::get::<UnionType>(follow_type::follow(ty)) {
        Some(union) => options.extend(
          union
            .options
            .iter()
            .filter(|option| seen.insert(**option)),
        ),
        None => {
          if seen.insert(ty) {
            options.push(ty);
          }
        }
      }
    }

    if options.len() == 1 {
      options[0]
    } else {
      // Safety: arena 为调用方传入 &mut TypeArena 的字段地址（本闭包捕获期间
      // 调用方借用唯一且存活、非空）；cpp TypeBuilder 捕获 arena& 同形，add_type
      // 对 arena 独占顺序追加，闭包调用点单线程无第二可变借用。
      unsafe { (*arena).add_type(UnionType { options }) }
    }
  })
}
