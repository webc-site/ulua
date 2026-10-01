use ulua_analysis::{
  records::primitive_type::{PrimitiveType, Type as PrimitiveKind},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
use ulua_common::{LUAU_ASSERT, fflag::LuauIntegerType2};
use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::state_mut;

/// 压入静态字面量（`l` 须为活跃 `LuaState` 指针——conformance 测试侧模块级契约）。
fn push_literal(l: *mut LuaState, value: &'static [u8]) {
  let value = value.strip_suffix(b"\0").unwrap_or(value);
  state_mut(l).push_bytes(value);
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn populate_rtti(l: *mut LuaState, ty: TypeId) {
  LUAU_ASSERT!(!ty.is_null());

  // Safety: 函数级 `# Safety` 契约保证 `ty` 非空且指向 RTTI 填充期间存活的
  // Type（analysis arena 借出），解引用出的 `&TypeVariant` 随填充全程有效。
  let variant = unsafe { &(*ty).ty };

  // Safety: `l` 为活跃状态机（集成测试布线契约，同上）；`variant` 已是安全引用，
  // 内部裸指针操作由 [`populate_rtti_variant`] 的 `# Safety` 契约收口。
  unsafe { populate_rtti_variant(l, variant) };
}

/// 逐变体填充 RTTI——安全骨架（match/循环/断言）+ 不安全叶子（C API 压栈）。
///
/// # Safety
///
/// `l` 为活跃状态机；`variant` 内嵌的裸指针（交并部分、表属性类型）在填充期间存活。
unsafe fn populate_rtti_variant(l: *mut LuaState, variant: &TypeVariant) {
  match variant {
    TypeVariant::Primitive(PrimitiveType { r#type, .. }) => match r#type {
      // Safety: 各叶子仅要求 `l` 活跃且字面量为静态 NUL 结尾串（入口契约转承）。
      PrimitiveKind::Boolean => push_literal(l, b"boolean\0"),
      PrimitiveKind::NilType => push_literal(l, b"nil\0"),
      PrimitiveKind::Number => push_literal(l, b"number\0"),
      PrimitiveKind::Integer => {
        if LuauIntegerType2.get() {
          // Safety: 同上。
          push_literal(l, b"integer\0");
        }
      }
      PrimitiveKind::String => push_literal(l, b"string\0"),
      PrimitiveKind::Thread => push_literal(l, b"thread\0"),
      PrimitiveKind::Buffer => push_literal(l, b"buffer\0"),
      _ => LUAU_ASSERT!(false, "Unknown primitive type"),
    },
    TypeVariant::Table(table) => {
      // newtable 压入的表由下方 setfield 逐属性填充、最终留在栈上。
      state_mut(l).new_table();

      for (name, prop) in &table.props {
        // cpp 语义等价：read_ty 优先、write_ty 兜底，两者皆无则跳过该属性。
        let Some(prop_ty) = prop.read_ty.or(prop.write_ty) else {
          continue;
        };

        // `prop_ty` 指向 arena 存活 Type（递归入口同契约）。
        // Safety: `populate_rtti` 的 `# Safety` 契约由本函数契约转承。
        unsafe { populate_rtti(l, prop_ty) };

        state_mut(l).set_field_str(-2, name.as_str());
      }
    }
    // 入口契约转承：`l` 活跃，字面量为静态 NUL 结尾串。
    TypeVariant::Function(_) => push_literal(l, b"function\0"),
    TypeVariant::Any(_) => push_literal(l, b"any\0"),
    TypeVariant::Intersection(intersection) => {
      for part in &intersection.parts {
        // Safety: 交集各 part 指向 arena 存活 Type（本函数 `# Safety` 契约）。
        LUAU_ASSERT!(unsafe { matches!((*(*part)).ty, TypeVariant::Function(_)) });
      }

      push_literal(l, b"function\0");
    }
    TypeVariant::Extern(extern_ty) => {
      let name = extern_ty.name.as_str();
      // 直接以 `push_str` 压入外部类型名。
      state_mut(l).push_str(name);
    }
    _ => LUAU_ASSERT!(false, "Unknown type"),
  }
}
