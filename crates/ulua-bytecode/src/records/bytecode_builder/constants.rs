//! `BytecodeBuilder` 之 常量/原型/import id 登记（add_constant* 族与打包布局）。

use ulua_common::{
  enums::luau_feedback_type::LuauFeedbackType,
  functions::import_layout::{
    K_IMPORT_COMPONENT_MASK, K_IMPORT_COUNT_SHIFT, import_component_shift,
  },
  macros::luau_assert::LUAU_ASSERT,
};

use super::{BytecodeBuilder, K_MAX_CLOSURE_COUNT, K_MAX_CONSTANT_COUNT};
use crate::records::{
  class_shape::ClassShape, constant::Constant, string_ref::StringRef, table_shape::TableShape,
};

// ── abs-r139：并自 `methods/bytecode_builder_add_child_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn add_child_function(&mut self, fid: u32) -> i16 {
    if let Some(cache) = self.proto_map.find(&fid) {
      return *cache;
    }

    let id = self.protos.len() as u32;

    if id >= K_MAX_CLOSURE_COUNT {
      return -1;
    }

    self.proto_map.try_insert(fid, id as i16);
    self.protos.push(fid);

    id as i16
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_class_shape.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn add_class_shape(&mut self, shape: ClassShape) -> i32 {
    let id = self.constants.len() as u32;

    if id >= K_MAX_CONSTANT_COUNT {
      return -1;
    }

    let c = Constant::ClassShape(self.class_shapes.len() as u32);

    self.class_shapes.push(shape);
    self.constants.push(c);

    id as i32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_constant.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// 常量去重表入口：缓存键由常量本身推导（`Constant::key`），调用点不再手搭。
  pub(crate) fn add_constant(&mut self, value: Constant) -> i32 {
    let key = value.key();
    if let Some(cache) = self.constant_map.find(&key) {
      return *cache;
    }

    let id = self.constants.len() as u32;

    if id >= K_MAX_CONSTANT_COUNT {
      return -1;
    }

    self.constant_map.try_insert(key, id as i32);
    self.constants.push(value);

    id as i32
  }

  pub fn add_constant_nil(&mut self) -> i32 {
    self.add_constant(Constant::Nil)
  }

  pub fn add_constant_boolean(&mut self, value: bool) -> i32 {
    self.add_constant(Constant::Boolean(value))
  }

  pub fn add_constant_number(&mut self, value: f64) -> i32 {
    self.add_constant(Constant::Number(value))
  }

  pub fn add_constant_integer(&mut self, value: i64) -> i32 {
    self.add_constant(Constant::Integer(value))
  }

  pub fn add_constant_string(&mut self, value: StringRef<'a>) -> i32 {
    let index = self.add_string_table_entry(value);
    self.add_constant(Constant::String(index))
  }

  /// cpp `addConstantVectorf`：键打包布局（x/y 进 value、z/w 进 extra）收敛在
  /// `Constant::key`，此处只构造常量本体。
  pub fn add_constant_vector(&mut self, x: f32, y: f32, z: f32, w: f32) -> i32 {
    self.add_constant(Constant::Vector([x, y, z, w]))
  }

  /// cpp `addConstantVectord`：四分量键打包布局收敛在 `Constant::key`。
  pub fn add_constant_vector_d(&mut self, x: f64, y: f64, z: f64, w: f64) -> i32 {
    self.add_constant(Constant::Vectord([x, y, z, w]))
  }

  pub fn add_constant_closure(&mut self, fid: u32) -> i32 {
    self.add_constant(Constant::Closure(fid))
  }

  pub fn add_constant_table(&mut self, shape: &TableShape) -> i32 {
    // 去重键不得撞上 `DenseHashMap` 的空键哨兵：撞上时 `find` 恒 `None`、
    // `try_insert` 会把哨兵写进槽位，故在入口把这条不变量显式化。
    debug_assert_ne!(
      *shape,
      TableShape::EMPTY_KEY_SENTINEL,
      "TableShape 去重键不得与 DenseHashMap 空键哨兵相撞"
    );

    if let Some(cache) = self.table_shape_map.find(shape) {
      return *cache;
    }

    let id = self.constants.len() as u32;

    if id >= K_MAX_CONSTANT_COUNT {
      return -1;
    }

    // C++ `value.valueTable = uint32_t(tableShapes.size())`: Table 载荷
    // indexes table_shapes, NOT constants. The previous `id` (= constants
    // length) over-indexed table_shapes and panicked in write_function.
    let value = Constant::Table(self.table_shapes.len() as u32);

    self.table_shape_map.try_insert(*shape, id as i32);
    self.table_shapes.push(*shape);
    self.constants.push(value);

    id as i32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_fb_slot.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn add_fb_slot(&mut self, t: LuauFeedbackType) -> u32 {
    LUAU_ASSERT!(t == LuauFeedbackType::LFT_CALLTARGET);
    self.fb_slots.push(self.get_instruction_count() as u32);
    (self.fb_slots.len() - 1) as u32
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_add_import.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn add_import(&mut self, iid: u32) -> i32 {
    self.add_constant(Constant::Import(iid))
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_decompose_import_id.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// 元组返回 `(count, id0, id1, id2)` 替代 cpp 的三个 `int32_t&` 出参。
  /// 与 aux 解码同用 `K_IMPORT_*` 布局常量与 `import_component_shift`（import id 与 aux 同构打包）。
  pub(crate) fn decompose_import_id(ids: u32) -> (i32, i32, i32, i32) {
    let count = (ids >> K_IMPORT_COUNT_SHIFT) as i32;
    let component =
      |k: u32| (ids >> import_component_shift(k)) as i32 & K_IMPORT_COMPONENT_MASK as i32;
    let id0 = if count > 0 { component(0) } else { -1 };
    let id1 = if count > 1 { component(1) } else { -1 };
    let id2 = if count > 2 { component(2) } else { -1 };
    (count, id0, id1, id2)
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_import_id_bytecode_builder.rs` ──
/// import id 打包（与 GETIMPORT 的 aux 字同构）的单一算术源：高 2 位为组件数，
/// 各组件按 `import_component_shift` 布局从高到低填入 10 位槽。公开三目
/// `get_import_id`/`get_import_id2`/`get_import_id3` 对应 cpp `getImportId`
/// 的三个重载，只定元数；掩码校验（各组件按位或后仍须 ⊆ 10 位掩码）收口于此。
fn pack_import_id(ids: &[i32]) -> u32 {
  let or_all = ids.iter().fold(0u32, |acc, &id| acc | id as u32);
  LUAU_ASSERT!(or_all <= K_IMPORT_COMPONENT_MASK);

  let mut word = (ids.len() as u32) << K_IMPORT_COUNT_SHIFT;
  for (k, &id) in ids.iter().enumerate() {
    word |= (id as u32) << import_component_shift(k as u32);
  }
  word
}

impl<'a> BytecodeBuilder<'a> {
  /// 打包 import id（cpp `BytecodeBuilder::getImportId`）：解码侧
  /// `decompose_import_id` 共用同一组布局常量（import id 与 aux 同构打包）。
  pub fn get_import_id(id0: i32) -> u32 {
    pack_import_id(&[id0])
  }

  pub fn get_import_id2(id0: i32, id1: i32) -> u32 {
    pack_import_id(&[id0, id1])
  }

  pub fn get_import_id3(id0: i32, id1: i32, id2: i32) -> u32 {
    pack_import_id(&[id0, id1, id2])
  }
}
