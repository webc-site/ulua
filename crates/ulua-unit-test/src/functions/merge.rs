use ulua_analysis::{
  functions::{follow_type, get_type, merge as merge_analysis},
  records::{type_arena::TypeArena, union_type::UnionType},
  type_aliases::{refinement_map::RefinementMap, type_id::TypeId},
};
use ulua_common::collections::HashSet;
fn add_union_options(options: &mut Vec<TypeId>, seen: &mut HashSet<TypeId>, ty: TypeId) {
  let followed = follow_type::follow(ty);
  let union = get_type::get::<UnionType>(followed);

  if let Some(union) = union {
    for option in &union.options {
      if seen.insert(*option) {
        options.push(*option);
      }
    }
  } else if seen.insert(ty) {
    options.push(ty);
  }
}

pub fn merge(arena: &mut TypeArena, l: &mut RefinementMap, r: &RefinementMap) {
  let arena = arena as *mut TypeArena;

  merge_analysis::merge(l, r, |a, b| {
    let mut options = Vec::new();
    let mut seen = HashSet::default();

    add_union_options(&mut options, &mut seen, a);
    add_union_options(&mut options, &mut seen, b);

    if options.len() == 1 {
      options[0]
    } else {
      // Safety: arena 为调用方传入 &mut TypeArena 的字段地址（行 22，本闭包捕获期间调用方借用唯一且存活、非空）；cpp TypeBuilder 捕获 arena& 同形，add_type 对 arena 独占顺序追加，闭包调用点单线程无第二可变借用。
      unsafe { (*arena).add_type(UnionType { options }) }
    }
  })
}
