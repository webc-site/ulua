use alloc::vec::Vec;

use ulua_vm::{functions::lua_gettop::lua_gettop, records::lua_state};

use crate::{
  enums::type_type_function_runtime::Type,
  functions::{
    alloc_type_user_data::alloc_type_user_data,
    allocate_type_function_type::allocate_type_function_type,
    get_type_function_runtime::{get_type_function_runtime, get_type_function_type_id},
    get_type_user_data::get_type_user_data,
    throw_type_error::throw_type_error,
  },
  macros::lua_check_args,
  records::{
    arena_handle::Handle, type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_union_type::TypeFunctionUnionType,
  },
  type_aliases::{
    lua_state::LuaState, type_function_type_id::TypeFunctionTypeId,
    type_function_type_variant::TypeFunctionTypeVariant,
  },
};
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
/// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
/// 类型运行期数据。对应 C++ 原生 `static int createOptional(lua_State* L)`（`cpp/Analysis/src/TypeFunctionRuntime.cpp:625`）。
pub unsafe fn create_optional(l: *mut LuaState) -> i32 {
  // Safety: l 为 VM 调注册闭包传入的存活 lua_State；实参个数先校验（throw_type_error 内部收口格式串
  // 字面量+匹配实参）；get_type_user_data 非 type 实参抛错，argument 指向 type_arena 存活
  // 节点；union_ty 判 is_null 命中后才解引用读 components（bump 块地址不移动，只读）；
  // runtime 句柄取回注册期写入主线程 thread data 的非空 TypeFunctionRuntime（null 由
  // Handle::from_ptr 收敛为 panic），allocate_type_function_type 返回其 arena 稳定指针，
  // nil_id 仅作裸指针入集合不被解引用。
  unsafe {
    let vm_l = l as *mut lua_state::LuaState;
    let runtime = Handle::from_ptr(get_type_function_runtime(l));
    lua_check_args!(vm_l, != 1, "types.optional: expected 1 argument, but got {}");

    let argument: TypeFunctionTypeId = get_type_user_data(l, 1);

    let mut components: Vec<TypeFunctionTypeId> = Vec::new();

    let union_ty = get_type_function_type_id::<TypeFunctionUnionType>(argument);
    if !union_ty.is_null() {
      components.reserve((*union_ty).components.len() + 1);
      components.extend((*union_ty).components.iter().copied());
    } else {
      components.push(argument);
    }

    let nil_type = TypeFunctionPrimitiveType::new(Type::NilType);
    let nil_variant = TypeFunctionTypeVariant::Primitive(nil_type);
    let nil_id = allocate_type_function_type(runtime, nil_variant);
    components.push(nil_id);

    let union_type = TypeFunctionUnionType { components };
    let union_variant = TypeFunctionTypeVariant::Union(union_type);
    alloc_type_user_data(l, union_variant, false);

    1
  }
}
