# ulua 消除 unsafe extern "C" 改造方案

## 调研结论(2025,当前代码库)

全仓库统计:

- `unsafe extern` 块/函数声明:881 处
- `#[unsafe(export_name = "ulua_...")]` / `#[unsafe(no_mangle)]`:294 处
- 内部 Rust↔Rust 调用占总调用数的绝大多数,真正需要 C ABI 的调用方只有 JIT 机器码

### `unsafe extern` 的调用方分三类

| 类别 | 例子 | 位置 | 能否消除 |
|------|------|------|----------|
| A. 编译期生成符号地址填 `NativeContext` | `luaV_lessthan`、`callProlog`、`executeGETGLOBAL` | `crates/ulua-code-gen/src/functions/init_functions.rs` + `crates/ulua-code-gen/src/records/native_context.rs` | **部分可**——`init_functions.rs` 中 250+ 个 `unsafe extern` 声明是给 Rust 侧"按名取地址"用的,可换成 `fn 指针常量`,消除 unsafe 块 |
| B. JIT 机器码运行时调用 | 机器码 `call [rNativeContext + offset]` | `crates/ulua-code-gen/src/functions/emit_fallback_emit_common_x_64.rs` | **不可消除**——机器码只认函数指针 + 固定调用约定,这是 `unsafe extern` 的唯一硬边界 |
| C. Conformance/C 库 FFI | `limited_realloc`、`mprotect`、`signal` | `crates/ulua-conformance/`、`crates/ulua-code-gen/src/functions/get_memory_size.rs` | **不可消除**——真 C ABI,保留即可 |

### 边界判断标准

**调用方是 Rust 代码 → 全部纯 Rust;调用方是 JIT 机器码 → 保留 unsafe 薄壳。**

## 改造方案(四步)

### 第 1 步: `init_functions.rs` 250+ 处 `unsafe extern` 声明 → Rust `fn` 指针(消除 unsafe 块)

`init_functions.rs` 现状:

```rust
unsafe extern "C-unwind" {
  #[link_name = "ulua_luaV_lessthan"]
  pub fn luaV_lessthan(L: *mut lua_State, ...) -> core::ffi::c_int;
  // ... 250+ 个
}
```

这些符号全部在本仓库内定义(VM crate / code-gen crate 内),并非外部 C 库——用 `extern` 块声明是借了"按名取地址"的技巧。改成 Rust 类型:

```rust
// crates/ulua-code-gen/src/records/native_fn.rs
use ulua_vm::type_aliases::{stk_id::StkId, t_value::TValue};
use crate::type_aliases::lua_state::lua_State;

pub type NativeFn = unsafe extern "C-unwind" fn(l: *mut lua_State, ...) -> core::ffi::c_int;

// 直接引用本仓库内定义的函数,无需 extern 块
pub const LUA_V_LESSTHAN: NativeFn = ulua_vm::functions::lua_v_lessthan::lua_v_lessthan;
pub const CALL_PROLOG: NativeFn = crate::functions::call_prolog::call_prolog;
```

调用点(`crates/ulua-code-gen/src/functions/init_functions.rs`):

```rust
context.lua_v_lessthan = Some(LUA_V_LESSTHAN);   // 而不是 Some(luaV_lessthan)
```

收益:
- `unsafe extern` 块整块消失(250+ 处)
- 符号名错误编译期报错,而不是链接期/运行期

### 第 2 步: `NativeContext` 指针签名 → Rust 索引签名(消除 unsafe 传播)

`crates/ulua-code-gen/src/records/native_context.rs` 现状:

```rust
pub lua_v_lessthan: Option<unsafe extern "C-unwind" fn(l: *mut lua_State, l: *const TValue, r: *const TValue) -> c_int>
```

机器码生成端(`emit_fallback_emit_common_x_64.rs` 之类)只往 `NativeContext` 里写 `fn 地址`,机器码执行端只读地址——**签名可以统一改成最小 unsafe 壳**:

```rust
#[repr(C)]
pub struct NativeContext {
  // 机器码调用的槽位:最小 unsafe(只指针解引用,签名用最小 unsafe fn)
  pub lua_v_lessthan: Option<unsafe extern "C-unwind" fn(*mut lua_State, *const TValue, *const TValue) -> c_int>,
  // ...
}
```

Shrink unsafe:
- Rust 侧调用点不再标 `unsafe`,标 `#[safe]`(2024 edition 允许在 2021 风格的 extern fn 上手动标 safe)
- unsafe 面积从"每个调用点"缩到"每个 fn 类型定义"

### 第 3 步: 机器码 fallback 调用端保留薄壳(唯一不可消除的 unsafe)

`crates/ulua-code-gen/src/functions/emit_fallback_emit_common_x_64.rs` 生成 `call [rNativeContext + offset]` 机器指令——这里 `unsafe extern` 不可消除,但**可以收缩到一行**:

```rust
// 导出函数只保留 unsafe extern 壳,unsafe 体全部转调内部 Rust 函数
#[unsafe(export_name = "ulua_executeGETGLOBAL")]
pub unsafe extern "C-unwind" fn execute_getglobal_shim(
  l: *mut lua_State, pc: *const u32, base: StkId, k: *mut TValue,
) -> *const u32 {
  // unsafe 收缩到指针解引用一处,函数体走纯 Rust
  execute_getglobal(&mut *l, pc, base, k)
}

// 纯 Rust 实现,unsafe 面积为 0
fn execute_getglobal(l: &mut LuaState, ...) -> *const u32 { ... }
```

本仓库 `crates/ulua-code-gen/src/functions/get_memory_size.rs` 已经是这个模式:

```rust
pub unsafe fn get_memory_size(_l: *mut lua_State, proto: *mut Proto) -> usize { ... }

#[unsafe(export_name = "ulua_get_memory_size")]
pub unsafe extern "C-unwind" fn get_memory_size_export(l: *mut lua_State, proto: *mut Proto) -> usize {
  unsafe { get_memory_size(l, proto) }
}
```

推广到全部 90+ 个 fallback(`execute_*`、`forg_*`、`call_*`、`luaV_*`、`luaH_*`、`luaC_*`):
- 导出壳 unsafe 面积 = 一行指针解引用
- 函数体 unsafe 面积 = 0

### 第 4 步: Conformance 侧真 C FFI 保留(不动)

`crates/ulua-conformance/` 中 `limited_realloc`、`mprotect`、`signal` 等,调用方是 OS C 库,`unsafe extern` 必须保留,不改。

## 改造后 unsafe 面积对比

| 项 | 改造前 | 改造后 |
|----|--------|--------|
| `unsafe extern` 块声明 | 250+ 处(`init_functions.rs`) | 0(换成 Rust fn 指针常量) |
| `NativeContext` 签名 | 每个字段全签名 | 每个字段最小签名(定义处) |
| 导出函数体 | 每个函数 90+ 处函数体 | 每个函数一行指针解引用 |
| 真 C 库 FFI | 保留 | 保留(conformance) |

## 预期性能影响

- **中性**:Rust fn 指针 + `#[inline(always)]` 转调,LLVM 优化后机器码与现状一致
- **bounds check 消除**:`debug_assert!(idx < len)` 让 Release 构建跳过检查;热循环用 `get_unchecked` 把检查收缩到一处
- **净收益**:正确性提升(悬垂指针→索引错误 Debug 直接 panic),性能不退化

## 实施顺序与验证

1. **先做第 1 步**(`init_functions.rs` 250+ 处符号声明),纯类型层改造,不影响机器码生成
2. **再做第 3 步**(fallback 薄壳化),逐个函数替换,每替换一个跑 `cargo test -p ulua-conformance` + `benchmarks/harness.py` 验证 checksum 一致
3. **最后做第 2 步**(`NativeContext` 签名最小化),需要确认 Rust 侧调用点全部已加 `#[safe]` 标注后统一收缩

验证标准:
- `cargo build --release` 通过
- `benchmarks/harness.py` 全部 checksum 与 C++ Luau 一致
- 运行速度相对当前 baseline 退化 < 2%(harness 中 fib/nbody/mandelbrot 中位数)
