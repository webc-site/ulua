// 边界契约测试：null/指针 id 位系指针值语义正面测试（既有约定 review.md §2）
extern crate alloc;

// Source: `tests/LValue.test.cpp`
#[test]
fn l_value_hashing_lvalue_global_prop_access() {
  use std::sync::Arc;

  use ulua_analysis::{
    records::{
      builtin_types::BuiltinTypes, field::Field, l_value_hasher::LValueHasher, symbol::Symbol,
    },
    type_aliases::{l_value::LValue, refinement_map::RefinementMap},
  };
  use ulua_ast::records::ast_name::AstName;

  let t1 = b"t";
  let x1 = "x".to_string();
  let t_x1 = LValue::Field(Field {
    parent: Some(Arc::new(LValue::Symbol(Symbol::from_global(
      AstName::from_static(t1),
    )))),
    key: x1,
  });

  let t2 = b"t";
  let x2 = "x".to_string();
  let t_x2 = LValue::Field(Field {
    parent: Some(Arc::new(LValue::Symbol(Symbol::from_global(
      AstName::from_static(t2),
    )))),
    key: x2,
  });

  assert_eq!(t_x1, t_x1);
  assert_eq!(t_x1, t_x2);
  assert_eq!(t_x2, t_x2);

  let hasher = LValueHasher;
  assert_eq!(hasher.hash(&t_x1), hasher.hash(&t_x1));
  assert_eq!(hasher.hash(&t_x1), hasher.hash(&t_x2));
  assert_eq!(hasher.hash(&t_x2), hasher.hash(&t_x2));

  let builtin_types = BuiltinTypes::new();
  let mut m = RefinementMap::new();
  m.insert(t_x1, builtin_types.string_type());
  m.insert(t_x2, builtin_types.number_type());

  assert_eq!(1, m.len());
}

// Source: `tests/LValue.test.cpp`
#[test]
fn l_value_hashing_lvalue_local_prop_access() {
  use core::ptr::null_mut;
  use std::sync::Arc;

  use ulua_analysis::{
    records::{
      builtin_types::BuiltinTypes, field::Field, l_value_hasher::LValueHasher, symbol::Symbol,
    },
    type_aliases::{l_value::LValue, refinement_map::RefinementMap},
  };
  use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName, location::Location};

  let t1 = b"t";
  let x1 = "x".to_string();
  let mut localt1 = AstLocal::new(
    AstName::from_static(t1),
    Location::default(),
    null_mut(),
    0,
    0,
    null_mut(),
    false,
  );
  let t_x1 = LValue::Field(Field {
    parent: Some(Arc::new(LValue::Symbol(Symbol::from_local(&mut localt1)))),
    key: x1,
  });

  let t2 = b"t";
  let x2 = "x".to_string();
  let mut localt2 = AstLocal::new(
    AstName::from_static(t2),
    Location::default(),
    &mut localt1,
    0,
    0,
    null_mut(),
    false,
  );
  let t_x2 = LValue::Field(Field {
    parent: Some(Arc::new(LValue::Symbol(Symbol::from_local(&mut localt2)))),
    key: x2,
  });

  assert_eq!(t_x1, t_x1);
  assert_ne!(t_x1, t_x2);
  assert_eq!(t_x2, t_x2);

  let hasher = LValueHasher;
  assert_eq!(hasher.hash(&t_x1), hasher.hash(&t_x1));
  assert_ne!(hasher.hash(&t_x1), hasher.hash(&t_x2));
  assert_eq!(hasher.hash(&t_x2), hasher.hash(&t_x2));

  let builtin_types = BuiltinTypes::new();
  let mut m = RefinementMap::new();
  m.insert(t_x1, builtin_types.string_type());
  m.insert(t_x2, builtin_types.number_type());

  assert_eq!(2, m.len());
}

// Source: `tests/LValue.test.cpp`
#[test]
fn l_value_luau_merge_hashmap_order() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
    type_aliases::{l_value::LValue, refinement_map::RefinementMap},
  };
  use ulua_unit_test::functions::{merge::merge, mk_symbol::mk_symbol};

  let builtin_types = BuiltinTypes::new();
  let mut m = RefinementMap::new();
  m.insert(LValue::Symbol(mk_symbol("b")), builtin_types.string_type());
  m.insert(LValue::Symbol(mk_symbol("c")), builtin_types.number_type());

  let mut other = RefinementMap::new();
  other.insert(LValue::Symbol(mk_symbol("a")), builtin_types.string_type());
  other.insert(LValue::Symbol(mk_symbol("b")), builtin_types.string_type());
  other.insert(LValue::Symbol(mk_symbol("c")), builtin_types.boolean_type());

  let mut arena = TypeArena::default();
  merge(&mut arena, &mut m, &other);

  assert_eq!(3, m.len());
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("a"))));
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("b"))));
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("c"))));

  assert_eq!(
    "string",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("a"))).unwrap())
  );
  assert_eq!(
    "string",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("b"))).unwrap())
  );
  assert_eq!(
    "boolean | number",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("c"))).unwrap())
  );
}

// Source: `tests/LValue.test.cpp`
#[test]
fn l_value_luau_merge_hashmap_order2() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
    type_aliases::{l_value::LValue, refinement_map::RefinementMap},
  };
  use ulua_unit_test::functions::{merge::merge, mk_symbol::mk_symbol};

  let builtin_types = BuiltinTypes::new();
  let mut m = RefinementMap::new();
  m.insert(LValue::Symbol(mk_symbol("a")), builtin_types.string_type());
  m.insert(LValue::Symbol(mk_symbol("b")), builtin_types.string_type());
  m.insert(LValue::Symbol(mk_symbol("c")), builtin_types.number_type());

  let mut other = RefinementMap::new();
  other.insert(LValue::Symbol(mk_symbol("b")), builtin_types.string_type());
  other.insert(LValue::Symbol(mk_symbol("c")), builtin_types.boolean_type());

  let mut arena = TypeArena::default();
  merge(&mut arena, &mut m, &other);

  assert_eq!(3, m.len());
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("a"))));
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("b"))));
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("c"))));

  assert_eq!(
    "string",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("a"))).unwrap())
  );
  assert_eq!(
    "string",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("b"))).unwrap())
  );
  assert_eq!(
    "boolean | number",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("c"))).unwrap())
  );
}

// Source: `tests/LValue.test.cpp`
#[test]
fn l_value_one_map_has_overlap_at_end_whereas_other_has_it_in_start() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
    type_aliases::{l_value::LValue, refinement_map::RefinementMap},
  };
  use ulua_unit_test::functions::{merge::merge, mk_symbol::mk_symbol};

  let builtin_types = BuiltinTypes::new();
  let mut m = RefinementMap::new();
  m.insert(LValue::Symbol(mk_symbol("a")), builtin_types.string_type());
  m.insert(LValue::Symbol(mk_symbol("b")), builtin_types.number_type());
  m.insert(LValue::Symbol(mk_symbol("c")), builtin_types.boolean_type());

  let mut other = RefinementMap::new();
  other.insert(LValue::Symbol(mk_symbol("c")), builtin_types.string_type());
  other.insert(LValue::Symbol(mk_symbol("d")), builtin_types.number_type());
  other.insert(LValue::Symbol(mk_symbol("e")), builtin_types.boolean_type());

  let mut arena = TypeArena::default();
  merge(&mut arena, &mut m, &other);

  assert_eq!(5, m.len());
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("a"))));
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("b"))));
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("c"))));
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("d"))));
  assert!(m.contains_key(&LValue::Symbol(mk_symbol("e"))));

  assert_eq!(
    "string",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("a"))).unwrap())
  );
  assert_eq!(
    "number",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("b"))).unwrap())
  );
  assert_eq!(
    "boolean | string",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("c"))).unwrap())
  );
  assert_eq!(
    "number",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("d"))).unwrap())
  );
  assert_eq!(
    "boolean",
    to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("e"))).unwrap())
  );
}

// —— String::from/.to_string 收口台账（r7-tstr17，5 个测试函数，矿面 4 枚）——
// 收 0 / 让 4（逐枚）：
//   1) l_value_hashing_lvalue_global_prop_access：L17 x1；2) 同函数 L26 x2；
//   3) l_value_hashing_lvalue_local_prop_access：L66 x1；4) 同函数 L82 x2。
// 让位理由：四枚均为 `"x".to_string()` 移入 `Field.key`，该字段为按值 `String`
//   （ulua-analysis/src/records/field.rs:10），已是单物化下限，直剥实测 E0308
//   （&str 非 String）；绑定通读实证单次消费（绑定即移入结构体字面量、无复用
//   分支），但内联不改变物化次数，运行期净省 0。Into 路线按 r7-tprops1 NO-GO
//   裁定系伪闸（`Into<String>` 收 &str 仍 1 malloc，纯剥壳），冻结勿复挖。
// 解锁条件：`Field.key` 换型 &str/驻留形或 Name→HipStr 跨席大票（见台账票77裁定）。
