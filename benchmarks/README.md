# ulua 基准与回归测试套件

本目录包含 **ulua**（纯 Rust Luau 编译器与寄存器虚拟机）的性能基准测试用例、纯 Rust 进程内对比评测器与性能回归追踪体系。

---

## 目录结构

```
benchmarks/
├── cases/              # 核心基准测试用例 (Lua 通用脚本，被回归测试与对比测试统一共享复用)
│   ├── binarytrees.lua # 树节点高频分配与 GC 吞吐
│   ├── fib.lua         # 深度函数递归与调用栈开销
│   ├── mandel.lua      # 曼德博分形集合 (密集分支与浮点迭代)
│   ├── matmul.lua      # 320x320 稠密矩阵乘法 (数组表与连续访存)
│   ├── nbody.lua       # 45 万步天体轨道引力模拟 (重度浮点寄存器调度)
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
├── runner/             # 纯 Rust 进程内性能对比评测执行器 (ulua vs mlua / Luau C++)
├── regression/         # 性能回归测试历史与分析器
│   ├── history.json    # 性能变化历史数据 (供 CI 生成变化曲线)
│   └── record.js       # Divan 基准输出解析器与提交关联记录器
├── results.json         # 最新一次纯 Rust 进程内评测结果 (gitignore)
├── results-compile.json # 编译吞吐组最新结果 (gitignore)
└── results-analysis.json# 类型检查组最新结果 (gitignore)
```

---

## 统一用例复用机制

回归测试和性能对比测试 **100% 共享复用** `benchmarks/cases/*.lua` 中的通用 Lua 脚本：

1. **性能回归测试 (`./regression.sh`)**：
   - 编译期由 [crates/ulua/build.rs](file:///Users/z/git/db/luaur/crates/ulua/build.rs) 自动扫描 `benchmarks/cases/` 目录，生成 `for_each_benchmark!` 宏；
   - 在 [crates/ulua/benches/benchmarks.rs](file:///Users/z/git/db/luaur/crates/ulua/benches/benchmarks.rs) 中零拷贝 `include_str!` 嵌入，使用现代 Rust 评测框架 **Divan** 执行高精度微基准；
   - 由 [benchmarks/regression/record.js](file:///Users/z/git/db/luaur/benchmarks/regression/record.js) 提取耗时中位数，并追加至 `history.json`。

2. **性能对比测试 (`./bench.sh`)**：
   - 运行期由 [benchmarks/runner/src/main.rs](file:///Users/z/git/db/luaur/benchmarks/runner/src/main.rs) 动态扫描 `benchmarks/cases/` 目录；
   - 在同一个纯 Rust 进程内存空间中，同台对比 `ulua` 与 `mlua (Luau C++)`，杜绝子进程启动与磁盘 I/O 干扰；
   - 输出 `results.json` 并由 `website/scripts/benchConvert.js` 自动转换为官网前端数据模块。

---

## 快速运行

```sh
# 1. 运行纯 Rust 进程内对比测试并同步更新官网数据
./bench.sh

# 2. 运行原生性能回归测试并记录 Commit 性能变化曲线
./regression.sh

# 3. 编译吞吐组 (parse / parse+compile 到字节码, ulua vs mlua/luau)
cargo run --release --manifest-path benchmarks/runner/Cargo.toml -- --group=compile
#    结果写入 benchmarks/results-compile.json (独立文件, 不污染 results.json)

# 4. 类型检查组 (ulua-analysis 前端 strict 检查, globals 基线列可差减固定开销)
cargo run --release --manifest-path benchmarks/runner/Cargo.toml -- --group=analysis
#    结果写入 benchmarks/results-analysis.json

# 5. 分配计数 (cfg 门控: 仅 --features count-alloc 编译时生效, 默认输出零差异)
cargo run --release --manifest-path benchmarks/runner/Cargo.toml --features count-alloc -- --alloc
cargo run --release --manifest-path benchmarks/runner/Cargo.toml --features count-alloc -- --group=compile --alloc
#    口径: Rust GlobalAlloc 请求次数/字节; mlua 侧 C malloc 不经此路径, 只报 ulua 侧

# 各分组同样支持用例过滤器: --group=compile parse_dense types_annotations
```
