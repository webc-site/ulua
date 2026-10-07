# ulua 基准与回归测试套件

本目录包含 **ulua**（纯 Rust Luau 编译器与寄存器虚拟机）的性能基准测试用例、纯 Rust 进程内对比评测器与性能回归追踪体系。

---

## 目录结构

```
benchmarks/
├── cases/              # 核心基准测试用例 (Lua 通用脚本，被回归测试与对比测试统一共享复用)
│   ├── binarytrees.lua # 树节点高频分配与 GC 吞吐
│   ├── coroutines.lua  # 40 万次协程乒乓切换 (create/resume/yield 往返)
│   ├── fannkuch.lua    # Fannkuch-redux 全排列翻饼 (整数运算与稠密表读写)
│   ├── fasta.lua       # 90 万次 LCG 伪随机 DNA 生成 (浮点运算与字符串构建)
│   ├── fib.lua         # 深度函数递归与调用栈开销
│   ├── life.lua        # 康威生命游戏 48x48×300 代 (二维嵌套表与分支密集邻居计数)
│   ├── mandel.lua      # 曼德博分形集合 (密集分支与浮点迭代)
│   ├── matmul.lua      # 320x320 稠密矩阵乘法 (数组表与连续访存)
│   ├── nbody.lua       # 45 万步天体轨道引力模拟 (重度浮点寄存器调度)
│   ├── nsieve.lua      # 埃拉托斯特尼筛法 30 万级×3 轮 (布尔哈希表插入/清除)
│   ├── oop.lua         # __index 继承链元表派发 (构造 + 虚方法调用 + 字段访问)
│   ├── patterns.lua    # 6 万次字符串模式匹配 (find/match/gsub/gmatch 回溯)
│   ├── queens.lua      # 八皇后回溯搜索 ×20 (递归回溯与提前剪枝)
│   ├── spectralnorm.lua# 1000 阶矩阵特征值谱范数逼近
│   ├── strings.lua     # 32 万次字符串格式化与拼接 (短串分配与垃圾回收)
│   └── tablesort.lua   # 14 万随机数原地快速排序与比较闭包
├── compile_cases/      # 编译吞吐组用例 (.luau, 仅供 runner --group=compile)
│   ├── parse_dense.luau        # 表达式/字符串插值密集的解析压力样本 (~290 KB)
│   ├── func_many.luau          # 海量独立函数 + 控制流的编译压力样本 (~350 KB)
│   └── types_annotations.luau  # Luau 类型注解密集的解析/编译样本 (~80 KB)
├── analysis_cases/     # 类型检查组用例 (.luau, 仅供 runner --group=analysis)
│   └── strict_lib.luau # 严格模式重类型库 (泛型/导出类型/记录, analyze 零诊断)
├── fixtures/           # 上述两组用例的确定性生成脚本 (node benchmarks/fixtures/generate.mjs)
├── runner/             # 纯 Rust 进程内性能对比评测执行器 (多后端引擎矩阵, 见下文)
├── regression/         # 性能回归测试历史与分析器
│   ├── history.json    # 性能变化历史数据 (供 CI 生成变化曲线)
│   └── record.js       # Divan 基准输出解析器与提交关联记录器
├── results.json         # 最新一次纯 Rust 进程内评测结果 (gitignore)
├── results-compile.json # 编译吞吐组最新结果 (gitignore)
└── results-analysis.json# 类型检查组最新结果 (gitignore)
```

用例选取对齐社区常用口径：Benchmarks Game / LuaJIT 经典集（binarytrees、fannkuch、
fasta、nsieve、mandel、nbody、spectralnorm）、上游 Luau `bench/tests`（life、
matrixmult、spectralnorm）与 VM 特性维度补充（fib 递归、coroutines 协程、oop 元表
派发、patterns 模式匹配、strings 字符串、tablesort 排序闭包、queens 回溯）。全部
用例限定在 Lua 5.1 / Luau 公共子集（无 bit32/goto/`//`/整型语义依赖），并用确定性
算法保证跨引擎返回值可精确比对（runner 会在指纹分歧时告警）。

---

## 统一用例复用机制

回归测试和性能对比测试 **100% 共享复用** `benchmarks/cases/*.lua` 中的通用 Lua 脚本：

1. **性能回归测试 (`./regression.sh`)**：
   - 编译期由 [crates/ulua/build.rs](../crates/ulua/build.rs) 自动扫描 `benchmarks/cases/` 目录，生成 `for_each_benchmark!` 宏；
   - 在 [crates/ulua/benches/benchmarks.rs](../crates/ulua/benches/benchmarks.rs) 中零拷贝 `include_str!` 嵌入，使用现代 Rust 评测框架 **Divan** 执行高精度微基准；
   - 由 [benchmarks/regression/record.js](regression/record.js) 提取耗时中位数，并追加至 `history.json`。

2. **性能对比测试 (`./bench.sh`)**：
   - 运行期由 [benchmarks/runner](runner/src/main.rs) 动态扫描 `benchmarks/cases/` 目录；
   - 在同一个纯 Rust 进程内存空间中同台对比 5 个引擎，杜绝子进程启动与磁盘 I/O 干扰；
   - 输出 `results.json` 并由 `website/scripts/benchConvert.js` 自动转换为官网前端数据模块。

### 引擎后端矩阵（全集对比）

mlua 0.12 的 ffi 模块把各 Lua 版本的 C 符号扁平 re-export（`pub use lua54::*`），
**同一二进制只能共链一种 C Lua**，故 runner 用互斥 cargo feature 三选一，`bench.sh`
依次跑三个后端并以 `--append` 把部分结果并集合并进同一 `results.json`：

| 后端 feature    | 引擎                                                     |
| --------------- | -------------------------------------------------------- |
| `engine-luajit` | `mlua/luajit-interp`、`mlua/luajit`（LuaJIT 2.1 解释 / JIT） |
| `engine-luau`   | `mlua/luau`、`mlua/luau-jit`（官方 Luau C++ 解释 / JIT）     |
| `engine-lua54`  | `mlua/lua5.4`（PUC Lua 5.4 解释）                           |

ulua 自家引擎与后端无关，任何 feature 组合下都可实测；互斥由 main.rs 顶部
`compile_error!` 守卫保证。所有引擎的测量口径完全对称：每样本新建 Lua state、
预热 1 轮 + `--runs` 轮交替采样取最小值、`eval` 捕获返回值指纹并跨引擎一致性校验。

---

## 快速运行

```sh
# 1. 全集对比评测（3 后端 exec + compile + analysis 三组）并同步更新官网数据
./bench.sh

# 2. 运行原生性能回归测试并记录 Commit 性能变化曲线
./regression.sh

# 3. 单后端 exec 组 (须显式选后端; --append 把结果并入既有 results.json)
cargo run --release --manifest-path benchmarks/runner/Cargo.toml --features engine-luajit -- --group=exec
cargo run --release --manifest-path benchmarks/runner/Cargo.toml --features engine-luau -- --group=exec --append --engines mlua/luau,mlua/luau-jit
cargo run --release --manifest-path benchmarks/runner/Cargo.toml --features engine-lua54 -- --group=exec --append --engines mlua/lua5.4
#    --engines k1,k2 只测指定引擎 key (多后端补测时跳过已测列); 空则测全部实测引擎

# 4. 编译吞吐组 (parse / parse+compile 到字节码)
cargo run --release --manifest-path benchmarks/runner/Cargo.toml --features engine-luajit -- --group=compile
#    结果写入 benchmarks/results-compile.json (独立文件, 不污染 results.json)

# 5. 类型检查组 (ulua-analysis 前端 strict 检查, globals 基线列可差减固定开销)
cargo run --release --manifest-path benchmarks/runner/Cargo.toml --features engine-luajit -- --group=analysis
#    结果写入 benchmarks/results-analysis.json

# 6. 分配计数 (cfg 门控: 仅 --features count-alloc 编译时生效, 默认输出零差异)
cargo run --release --manifest-path benchmarks/runner/Cargo.toml --features engine-luajit,count-alloc -- --alloc
cargo run --release --manifest-path benchmarks/runner/Cargo.toml --features engine-luajit,count-alloc -- --group=compile --alloc
#    口径: Rust GlobalAlloc 请求次数/字节; mlua 侧 C malloc 不经此路径, 只报 ulua 侧

# 各分组同样支持用例过滤器: --group=exec fib fannkuch / --group=compile parse_dense
```
