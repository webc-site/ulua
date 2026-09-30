//! ulua 性能基准评测套件 (基于现代 Rust 原生性能评测框架 Divan)
//!
//! 零拷贝、编译期嵌入 Lua 基准脚本，分离编译吞吐量与 VM 执行性能。VM 执行进一步
//! 拆成解释器 (`vm_execution`) 与 JIT 原生码 (`vm_execution_jit`) 两组，端到端
//! 高级 API 同样拆双份，方便对同一用例做解释/JIT 直接对比。JIT 相关 bench 只在
//! `--features jit` 时编译；未开启该 feature 时本文件与旧行为一致。
//! 消除操作系统进程创建 (fork/exec) 与磁盘 I/O 干扰，获取纳秒/微秒级极高精度的性能数据。

use divan::{Bencher, black_box, counter::BytesCount};
use mimalloc::MiMalloc;
use ulua::{Lua, compile};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
  divan::main();
}

// 编译期由 build.rs 动态扫描 benchmarks/cases/ 目录生成 for_each_benchmark! 宏分发
include!(concat!(env!("OUT_DIR"), "/benchmarks_cases.rs"));

/// 构造一个已启用 JIT 的 `Lua`；当前平台 `is_supported()==false` 时返回 `None`，
/// 由 bench 主体据此跳过（divan 没有运行期 skip API，返回即视为空样本）。
#[cfg(feature = "jit")]
fn lua_with_jit() -> Option<Lua> {
  let lua = Lua::new();
  lua.enable_jit(true).ok()?;
  Some(lua)
}

/// 纯字节码解释执行性能评测（预先完成编译，纯测量 VM 解释器执行速度）
#[divan::bench_group]
mod vm_execution {
  use super::*;

  macro_rules! bench_vm {
    ($name:ident, $src:expr) => {
      #[divan::bench]
      fn $name(bencher: Bencher) {
        let bytecode = compile($src).expect(concat!("编译 ", stringify!($name), ".lua 失败"));
        bencher
          .counter(BytesCount::of_slice(&bytecode))
          .with_inputs(Lua::new)
          .bench_local_values(|lua| lua.load_bytecode(black_box(&bytecode)).exec());
      }
    };
  }

  for_each_benchmark!(bench_vm);
}

/// JIT 原生码执行性能评测（稳态）。
///
/// **为何不能用 `with_inputs`**：divan 默认把 `gen_input` 的耗时算进样本（见
/// `divan-0.1.21/src/benchmark/mod.rs:622-625`，只有 `skip_ext_time` 才排除）；
/// 而 `Chunk::load_bytecode` 在 `jit_enabled=true` 时会同步调用
/// `luau_codegen_compile`（`ulua-rt/src/chunk.rs:173-179`）。两者叠加 = 每个样本
/// 重跑一次原生编译，测到的是「冷启次数 × N」而不是稳态执行速度。这里把
/// `Lua::new + enable_jit + load_bytecode → Function` 全链放在 `fn` 体内，一次
/// 性触发 CodeGen 并把结果缓存到 Proto 上；`bench_local` 反复调 `Function::call`
/// 就是纯原生码吞吐量。
///
/// 与 `vm_execution`（每样本冷启解释器）不是同一口径；对比时请把它当作
/// "JIT 稳态"独立基线追踪跨 commit 回归，端到端 JIT 冷启看 `high_level_lua_jit`。
#[cfg(feature = "jit")]
#[divan::bench_group(name = "vm_execution_jit")]
mod vm_execution_jit {
  use super::*;

  macro_rules! bench_vm_jit {
    ($name:ident, $src:expr) => {
      #[divan::bench]
      fn $name(bencher: Bencher) {
        let bytecode = compile($src).expect(concat!("编译 ", stringify!($name), ".lua 失败"));
        // `_lua` 只是持有 state 生命周期：Function 只存 registry 引用，Lua 必须先
        // 于其 drop，故把它作为 fn 局部而非 with_inputs 内构造。
        let Some(_lua) = lua_with_jit() else {
          eprintln!(
            "JIT 平台不支持，跳过 vm_execution_jit::{}",
            stringify!($name)
          );
          return;
        };
        // into_function 内部即 luau_codegen_compile；此处完成一次性 JIT 编译。
        let func = _lua
          .load_bytecode(&bytecode)
          .into_function()
          .expect(concat!("JIT 加载 ", stringify!($name), " 失败"));
        bencher
          .counter(BytesCount::of_slice(&bytecode))
          .bench_local(|| func.call::<()>(()));
      }
    };
  }

  for_each_benchmark!(bench_vm_jit);
}

/// 编译器编译吞吐量评测（从源码到 Luau 字节码的生成耗时与源码处理吞吐量）
#[divan::bench_group]
mod compile_speed {
  use super::*;

  macro_rules! bench_compile {
    ($name:ident, $src:expr) => {
      #[divan::bench]
      fn $name(bencher: Bencher) {
        bencher
          .counter(BytesCount::of_str($src))
          .bench(|| compile(black_box($src)));
      }
    };
  }

  for_each_benchmark!(bench_compile);
}

/// 端到端高级 API 运行评测 (ulua::Lua 实例构建、编译与解释执行全链路)
#[divan::bench_group]
mod high_level_lua {
  use super::*;

  macro_rules! bench_high_level {
    ($name:ident, $src:expr) => {
      #[divan::bench]
      fn $name(bencher: Bencher) {
        bencher.counter(BytesCount::of_str($src)).bench(|| {
          let lua = Lua::new();
          lua.load(black_box($src)).exec()
        });
      }
    };
  }

  for_each_benchmark!(bench_high_level);
}

/// 端到端高级 API + JIT：`lua.load(src).exec()` 全链路含 Luau 编译 + 一次
/// CodeGen + 首次执行，反映真实用户打开 JIT 后跑一次性脚本的墙钟时间；与
/// `high_level_lua` 一一对应。稳态执行性能请看 `vm_execution_jit`。
#[cfg(feature = "jit")]
#[divan::bench_group(name = "high_level_lua_jit")]
mod high_level_lua_jit {
  use super::*;

  macro_rules! bench_high_level_jit {
    ($name:ident, $src:expr) => {
      #[divan::bench]
      fn $name(bencher: Bencher) {
        if lua_with_jit().is_none() {
          eprintln!(
            "JIT 平台不支持，跳过 high_level_lua_jit::{}",
            stringify!($name)
          );
          return;
        }
        bencher.counter(BytesCount::of_str($src)).bench(|| {
          let lua = lua_with_jit().expect("JIT 支持已在计时外探测过");
          lua.load(black_box($src)).exec()
        });
      }
    };
  }

  for_each_benchmark!(bench_high_level_jit);
}
