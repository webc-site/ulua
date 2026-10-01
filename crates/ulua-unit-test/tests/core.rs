// r7 测试波碎片簇A 差集登记（对照 `cpp/tests/TypeInfer.test.cpp`，TypeInfer 核心套件）。
// 本席纪律=只落盘纯断言且现树可绿的枚；本文件所涉 5 枚缺口全部让位（生产码/旗标
// 缺失，禁改生产码），故本目标暂为零测试登记件，待上游 flag/全链路移植后回填。
// 命名沿用本目录主题前缀惯例（`core_*`，借 linter.rs `linter_*` 先例）。

// 缺口（未移植，对照 `tests/TypeInfer.test.cpp`，共 5 例，逐枚原因）：
// - generic_P_with_intersection_props_and_partial_table（:3047）——旗标
//   `LuauSubtypingMissingPropertiesAsNil` 已在位，但现树 generic-P widening
//   全链路（约束求解 → P 界选择 → 复查）未达上游语义：实测 createElement 的
//   `P?` 实参 `P | nil` 界判定误报 TypeMismatch（"accessing `tag` results in
//   `string`…"）。与姊妹欠账 frontend_generic_p_widening_with_cross_module_
//   recursive_type（frontend.rs:2108 #[ignore]，判因=cpp 侧 generic-P widening
//   链路差异，单改 Subtyping 属盲改）同根。faithful 断言（无错误）不可绿，让位。
// - generic_P_widening_with_recursive_optional_field（:3075）——同上，实测
//   同一 TypeMismatch 签名（递归 Node? 变体），同根让位。
// - fuzzer_relate_extern_table_1（:3099）/ fuzzer_relate_extern_table_2（:3116）
//   ——依赖 FFlag `LuauCheckReadTyWhenRelatingExtern`（relation 读 extern 表
//   属性类型判定），ulua 未同步该 flag 与对应判定；缺 flag 时
//   LUAU_REQUIRE_ERRORS 的成因路径不存在，faithful 断言不可表达，让位。
// - fuzzer_generic_binding_ice（:3133）——依赖 FFlag
//   `LuauDoNotIceForBindingGeneric`（generic 绑定失败降级为诊断而非 ICE），
//   ulua 未同步该 flag；缺 flag 时该 fuzzer 源在旧路径行为未定义
//   （LUAU_ASSERT 面），faithful 断言不可表达，让位。
