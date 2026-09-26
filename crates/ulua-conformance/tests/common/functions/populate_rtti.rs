use ulua_analysis::{
  records::primitive_type::{PrimitiveType, Type as PrimitiveKind},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
use ulua_common::{LUAU_ASSERT, fflag::LuauIntegerType2, functions::c_str::with_c_str};
use ulua_vm::{
  functions::{lua_pushstring::lua_pushstring, lua_setfield::lua_setfield},
  macros::lua_newtable::lua_newtable,
  records::lua_state::LuaState,
};

use crate::common::functions::cstr::cstr;

unsafe fn push_literal(l: *mut LuaState, value: &'static [u8]) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    lua_pushstring(l, cstr(value));
  }
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
      PrimitiveKind::Boolean => unsafe { push_literal(l, b"boolean\0") },
      PrimitiveKind::NilType => unsafe { push_literal(l, b"nil\0") },
      PrimitiveKind::Number => unsafe { push_literal(l, b"number\0") },
      PrimitiveKind::Integer => {
        if LuauIntegerType2.get() {
          // Safety: 同上。
          unsafe { push_literal(l, b"integer\0") };
        }
      }
      PrimitiveKind::String => unsafe { push_literal(l, b"string\0") },
      PrimitiveKind::Thread => unsafe { push_literal(l, b"thread\0") },
      PrimitiveKind::Buffer => unsafe { push_literal(l, b"buffer\0") },
      _ => LUAU_ASSERT!(false, "Unknown primitive type"),
    },
    TypeVariant::Table(table) => {
      // Safety: `l` 活跃，newtable 压入的表由下方 setfield 逐属性填充、最终留在栈上。
      unsafe { lua_newtable(l) };

      for (name, prop) in &table.props {
        // cpp 语义等价：read_ty 优先、write_ty 兜底，两者皆无则跳过该属性。
        let Some(prop_ty) = prop.read_ty.or(prop.write_ty) else {
          continue;
        };

        // Safety: `l` 活跃；`prop_ty` 指向 arena 存活 Type（递归入口同契约）。
        unsafe { populate_rtti(l, prop_ty) };

        // Safety: `l` 活跃且栈顶为本分支压入的属性值；`with_c_str` 补 NUL 的临时
        // `field` 指针在闭包调用期内被 `lua_setfield` 消费（key 走 intern 表，callee
        // 当场复制），setfield 消费栈顶。
        with_c_str(name.as_str().as_bytes(), |field| unsafe {
          lua_setfield(l, -2, field);
        });
      }
    }
    TypeVariant::Function(_) => unsafe { push_literal(l, b"function\0") },
    TypeVariant::Any(_) => unsafe { push_literal(l, b"any\0") },
    TypeVariant::Intersection(intersection) => {
      for part in &intersection.parts {
        // Safety: 交集各 part 指向 arena 存活 Type（本函数 `# Safety` 契约）。
        LUAU_ASSERT!(unsafe { matches!((*(*part)).ty, TypeVariant::Function(_)) });
      }

      unsafe { push_literal(l, b"function\0") };
    }
    TypeVariant::Extern(extern_ty) => {
      let name = extern_ty.name.as_str();
      // Safety: `l` 活跃；`with_c_str` 补 NUL 的临时指针在闭包调用期内被
      // `lua_pushstring` 消费（callee 当场复制/驻留）。
      unsafe { with_c_str(name.as_bytes(), |key| lua_pushstring(l, key)) };
    }
    _ => LUAU_ASSERT!(false, "Unknown type"),
  }
}
