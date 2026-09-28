// r7 测试波碎片簇A 差集登记（对照 `cpp/tests/TypeInfer.generics.test.cpp`）。
// 本席纪律=只落盘纯断言且现树可绿的枚；本文件所涉 4 枚缺口全部让位（旗标缺失，
// 禁改生产码），故本目标暂为零测试登记件，待 flag 同步后回填。
// 命名沿用本目录主题前缀惯例（借 type_infer_generics.rs 前缀先例，回填时取
// `generics_*`）。

// 缺口（未移植，对照 `tests/TypeInfer.generics.test.cpp`，共 4 例，逐枚原因）：
// - generic_function_parameter_rejects_incompatible_argument（:62）
// - generic_function_parameter_accepts_same_generic（:83）
// - generic_function_parameter_rejects_union_containing_generic（:101）
// - generic_function_parameter_nested_in_table_accepts_incompatible_property
//   （:118）
// 四枚均以 ScopedFastFlag{FFlag::LuauSoundGenericMismatches, true} 为前置
// （sound generic mismatch 诊断链），ulua 未同步该 flag 与对应判定——同
// type_infer_generics.rs:3426 既有缺口登记。缺 flag 时 TypeMismatch 断言的
// 成因路径不存在，faithful 断言（错误数/ wantedType-givenType 串）不可表达，让位。
