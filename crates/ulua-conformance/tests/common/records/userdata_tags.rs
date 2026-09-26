//! conformance IR hook 用例的两个 userdata tag。
//!
//! 对应 cpp `ConformanceIrHooks.h:15-16` 的
//! `constexpr uint8_t kTagVec2 = 12; constexpr uint8_t kTagVertex = 13;`。
//! 原先 push/get/setup 三处各自复制一份，且类型在 `u8`/`i32` 之间漂移
//! （`lua_vec_2_get.rs` 甚至直接写裸 `12`），故集中为单一来源；
//! 交给 C API（形参为 `c_int`）时统一 `as c_int`，取值远小于 `c_int` 上限，转换无损。
//!
//! IR hook 侧的 `Vec2::TAG` / `Vertex::TAG`（形参为 `i32`，交给 `IrBuilder::constInt`）
//! 也由这两个常量派生，C API 路径与 IR hook 路径因此共用同一份取值；上游若调整
//! `kTagVec2`/`kTagVertex`，只需改本文件。
//! 注意 `DirectFieldAccess.test.cpp:21-22` 的 `kTagVec2 = 42 / kTagOther = 43` 是另一个
//! 上游常量的同名副本，与本文件无关（见 `functions/direct_field_access_k_tag_vec_2.rs`）。

/// `ConformanceIrHooks.h:15` `kTagVec2`。
pub const K_TAG_VEC2: u8 = 12;

/// `ConformanceIrHooks.h:16` `kTagVertex`。
pub const K_TAG_VERTEX: u8 = 13;
