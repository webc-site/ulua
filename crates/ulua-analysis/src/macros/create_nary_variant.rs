//! TypeFunction n 元变体（union/intersection）create_* 构造骨架单点
//! （`TypeFunctionRuntime.cpp`）。
//!
//! C++ 侧 `createUnion`/`createIntersection` 逐行同形：逐参展平同类嵌套变体、
//! 跳过中性元类型、空集退化为对偶类型、单元素直接压栈、否则聚合成变体。只有
//! 展平类型、中性元类型与结果 variant 不同。[`create_nary_variant!`] 保留全部
//! 对外路径、函数名、签名与守卫语义不变，调用序契约文本单点维护在宏内，
//! 调用点只剩「cpp 出处 + 类型/variant 名」。

/// 生成一枚「展平 n 元同类变体、跳过中性元类型并聚合压栈」的 `pub fn` 入口。
/// r19 起首参收形为 vm 侧独占 `&mut LuaState`，内存安全前提由该接收者类型承载，
/// 故不再是 `unsafe fn`；余下裸操作收口在宏体内单一 `unsafe {}` 块。宏体一律
/// 书写全限定路径：`macro_rules!` 展开点的标识符在**调用点**解析，本文件的 `use`
/// 对展开不可见。
///
/// 用法：
/// ```ignore
/// create_nary_variant!(
///   /// 对应 C++ 原生 `static int createUnion(lua_State* L)`
///   /// （`cpp/Analysis/src/TypeFunctionRuntime.cpp:653`）。
///   create_union,
///   TypeFunctionUnionType,
///   TypeFunctionNeverType,
///   Never,
///   Union
/// );
/// ```
macro_rules! create_nary_variant {
  (
    $(#[$attr:meta])*
    $name:ident,
    $flat:ident,
    $neutral:ident,
    $neutral_variant:ident,
    $result_variant:ident $(,)?
  ) => {
    $(#[$attr])*
    ///
    /// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载）：
    /// `l` 须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的状态：VM 已把实参
    /// 压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
    /// 类型运行期数据。
    pub(crate) fn $name(
      l: &mut ulua_vm::records::lua_state::LuaState,
    ) -> i32 {
      let arg_size = l.get_top();
      let mut components: ::alloc::vec::Vec<
        crate::type_aliases::type_function_type_id::TypeFunctionTypeId,
      > = ::alloc::vec::Vec::with_capacity(arg_size as usize);

      for i in 1..=arg_size {
        let component = crate::functions::get_type_user_data::get_type_user_data(l, i);

        if let Some(nary_component) =
          crate::functions::get_type_function_runtime::get_type_function_type_id::<$flat>(
            component,
          )
        {
          components.extend(nary_component.components.iter().copied());
        } else if crate::functions::get_type_function_runtime::get_type_function_type_id::<
          $neutral,
        >(component)
          .is_some()
        {
          continue;
        } else {
          components.push(component);
        }
      }

      if components.is_empty() {
        crate::functions::alloc_type_user_data::alloc_type_user_data(
          l,
          crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant::$neutral_variant(
            $neutral::default(),
          ),
          false,
        );
      } else if components.len() == 1 {
        crate::functions::push_type::push_type(l, components[0]);
      } else {
        crate::functions::alloc_type_user_data::alloc_type_user_data(
          l,
          crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant::$result_variant(
            $flat { components },
          ),
          false,
        );
      }

      1
    }
  };
}

pub(crate) use create_nary_variant;
