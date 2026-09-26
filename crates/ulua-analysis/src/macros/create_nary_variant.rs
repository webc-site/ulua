//! TypeFunction n 元变体（union/intersection）create_* 构造骨架单点
//! （`TypeFunctionRuntime.cpp`）。
//!
//! C++ 侧 `createUnion`/`createIntersection` 逐行同形：逐参展平同类嵌套变体、
//! 跳过中性元类型、空集退化为对偶类型、单元素直接压栈、否则聚合成变体。只有
//! 展平类型、中性元类型与结果 variant 不同。[`create_nary_variant!`] 保留全部
//! 对外路径、函数名、签名与守卫语义不变，`# Safety` 契约文本单点维护在宏内，
//! 调用点只剩「cpp 出处 + 类型/variant 名」。

/// 生成一枚「展平 n 元同类变体、跳过中性元类型并聚合压栈」的
/// `pub unsafe fn` 入口。
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
    /// # Safety
    /// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
    /// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
    /// 类型运行期数据。
    pub unsafe fn $name(
      l: *mut crate::type_aliases::lua_state::LuaState,
    ) -> i32 {
      // Safety: l 为 VM 调注册闭包传入的存活 lua_State；get_type_user_data 对非 type 实参先抛
      // 错、返回的 TypeFunctionTypeId 指向 type_arena 存活节点（bump 块、地址不移动）；
      // get_type_function_type_id 按 variant tag 判别、未命中返回 null，判空/as_ref 命中后才
      // 读 components 且只读，写入对象是本地 Vec；push_type/alloc_type_user_data 前置同族
      // 闭包约定满足。
      unsafe {
        let vm_l = l as *mut ulua_vm::records::lua_state::LuaState;
        let arg_size = ulua_vm::functions::lua_gettop::lua_gettop(vm_l);
        let mut components: ::alloc::vec::Vec<
          crate::type_aliases::type_function_type_id::TypeFunctionTypeId,
        > = ::alloc::vec::Vec::with_capacity(arg_size as usize);

        for i in 1..=arg_size {
          let component = crate::functions::get_type_user_data::get_type_user_data(l, i);

          if let Some(nary_component) =
            crate::functions::get_type_function_runtime::get_type_function_type_id::<$flat>(
              component,
            )
            .as_ref()
          {
            components.extend(nary_component.components.iter().copied());
          } else if !crate::functions::get_type_function_runtime::get_type_function_type_id::<
            $neutral,
          >(component)
            .is_null()
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
    }
  };
}

pub(crate) use create_nary_variant;
