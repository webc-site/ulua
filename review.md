# ulua Rust 改写规范 (rev.3 — 彻底 Rust 化)

## 0. 总纲(最高优先级,覆盖本文件其余所有条款)

- 不要求与 C++ 保持任何结构上的 1:1 映射。 `./cpp` 只是行为 oracle:它规定
  每个函数"给定输入应产出什么结果 / 什么边界行为",不规定用什么数据结构、
  指针、控制流、命名或模块划分来实现。
- 正确 + 高效 + 零 C 风格,三者并列,缺一不可。任何"这是照抄 cpp 所以保留"
  的理由,一律不成立。
- 验收仍是:clippy 无报警(不用 allow)、`./test.sh` 全绿、正确性以 `./cpp` 的
  输入→输出行为为准。行为等价即可,结构随你重写。
- 对 cpp 自身的缺陷/未完成项(如 SCCP Integer eq 的保守分支),Rust 侧取 Lua
  语义的正确方向,并在代码注释以 `DELIBERATE DEVIATION` 锚点记录出处与理由。

## 1. 工作流程

1. `./fork.sh <名>` 开 worktree 到 `/tmp/fork/<名>` 开发。
2. 运行 `./sh/clippy.sh`,修复所有报警。
3. 小步迭代:每完成一处改写,合并回主目录 → `./test.sh` 保绿 → 才继续下一处。
4. 子代理只改代码 + `cargo check`,不跑测试;主代理合并改动、对照行为修 `./test.sh` 报错、然后删分支。
5. 不断并发开全新子代理,自主决策打磨细节,循环推进,不需要人确认。
6. 主目录工作区可能被并行会话同时编辑:只 merge 分支或精确路径提交,
   禁 `git add -A`;每次合并后 `git log -1 dev` 核实到位(cd 状态坑)。

## 2. 裸指针 → Rust 类型(强制)

`rg "null_mut|\\*mut |\\*const |as \\*mut|as \\*const|unsafe" -t rs` 逐处消除:

- 可空指针 → `Option<&T>` / `Option<&mut T>`,不是 `null_mut()` 哨兵。
- 非空指针 → `&T` / `&mut T` / 索引 / `Box`/`Rc`/`Arc`,按所有权语义选。
- 数组下标裸指针算术(`p.add(i)`、`(*p).field`)→ 切片 `&p[..]` / 迭代器 / 具名字段。
- arena / 自引用图:优先 `Vec<节点> + 索引(新type Idx)` 句柄模型,或带生命周期的引用;
  确需内部可变时用 `&UnsafeCell` 局部封装,把 `unsafe` 关进一个有契约的最小边界,
  不得让 `unsafe` 和裸指针渗透到业务逻辑。
- `unsafe` 只允许存在于真正的 FFI 边界(`ulua-capi`、libc/wasi-libc 对接),
  且每处必须有 `# Safety` 契约。纯逻辑层的 `pub unsafe fn` 视为待消灭对象。
- 所有权的"转手"要有显式动作(置哨兵、`mem::forget` 的对称处理),防止
  Drop 守卫误释放已转移的资源(教训:`replace_registry_value` 的槽位转移)。

## 3. C 风格 → Rust 惯用法(强制)

- 索引循环 `for i in 0..v.len()` / `while i < v.len()` → 迭代器、`iter()`/`iter_mut()`/
  `enumerate()`、切片方法(`chunks`、`windows`、`split`、`zip`)。索引仅当它是数据
  (SSA/矩阵坐标/寄存器号)时保留,且此时仍优先 `iter().enumerate()`。
- 出参 → 返回值:函数把入参当输出用的(`&mut Vec` 只 push、`&mut Option` 只赋值、
  `&mut` 累加器),改为返回 `T` / `(T, U)` / `Result<T>`。builder 式写缓冲 `&mut Vec<u8>` sink 可保留(本就是 Rust 惯用)。
- 手动整数累加 `v = v*10 + d` → `fold` / `iter().try_fold` / 标准解析。
- `if cond { true } else { false }`、`&*x`、冗余 `return`、空 `else`、`label` 型多层
  嵌套 break → 消除(`?`、`matches!`、提前 return、`ControlFlow`、`?`+`and_then`)。
- C 风格字符串:`*const c_char`/NUL 结尾/`Vec<u8>` 当字符串 → `&str`/`&[u8]`/`Cow`。
  只在 FFI 边界保留 `c_char`。
- 别名/包装:`_alt_a/_alt_b/...` 这类照抄 C++ 重载而拆出的文件,若只是薄转发或重复,
  合并为惯用的单实现 + 可选参数 / trait / 默认值;命名去机器味,函数名短而清晰。
- 死代码、未用变量、重复逻辑 → 删除/抽函数抽模块复用。

## 4. 泛型化,消除 dyn 与虚分派(强制)

`rg "dyn " -t rs` 逐处评估:能用单态泛型 `<T: Trait>`、`impl Trait`、
或 `enum_dispatch`(`cargo add enum_dispatch`)替代热路径 `dyn` 的,一律替代。
仅当类型集合运行期开放、或泛型会导致代码爆炸/编译期成本不合理时才保留 `Box<dyn>`,
并说明理由。

## 5. 高性能库替代手写实现(强制)

参照 `.agents/skills/rust_review/SKILL.md`。依赖一律 `cargo add`(直接写 Cargo.toml 视为违规):

- 数字转串 `itoa`,浮点转串 `zmij`,时间戳 `coarsetime`,时间格式化 `jiff`,
  哈希 map/set `gxhash`,并发字典 `papaya`,消息队列 `crossfire`,锁 `parking_lot`,
  子串查找 `memchr`,CRC `crc32fast`,随机 `fastrand`,紧凑串 `hipstr`,
  JSON `sonic-rs`(禁 serde_json),值↔枚举 `strum`,位图 `roaring`,日志 `log`
  (测试用 `ctor` + `log_init::init()`)。
- 手写循环能压成一次 `fold`/迭代器链且更快更清晰的,压;能不 `collect` 就不 `collect`,
  能 `find`/`take_while`/`try_for_each` 提前终止就不全遍历。
- 可 `get_unchecked`/`unwrap_unchecked` 的 100% 安全热路径去掉越界检查(须论证安全)。

## 6. 错误处理与类型

- 用 `thiserror` 在 `error.rs`/独立模块定义错误;第三方错误 `#[error(transparent)]` 转发。
  禁 C 风格错误码/字符串异常/`panic!`(除绝对不可触发)。`unwrap` 只在 100% 安全处。
- `as` 数字转换:100% 安全可大胆用;可能静默溢出/截断处显式处理。
- 编译期能算的算成 `const`/`const fn`;消灭魔法数字与雷同字符串(提为常量)。

## 7. 架构与模块

- 模块低耦合高内聚;超大文件/函数拆为模块文件夹与小函数。
- crate 对外接口清晰、最小化,禁二次导出;内部结构体直接暴露字段,别套 getter。
- `rg ffi -t rs`:非 FFI 边界处误用的 `ffi` 类型改回对应 Rust 原生类型。
- `rg "#\\[allow" -t rs`:必须为 0。
- 零死代码:pub API 孤儿(零调用导出函数、死 getter、无人消费的导出)定期清理,
  删除后连带清理 re-export 与文档引用。清理前 rg 交叉核对消费方(评审转述不可尽信)。

## 8. 测试

- 集成测试放 `tests/`;`src/` 内只保留需要访问私有/`pub(crate)` 实现的单元测试。
  把只测公开 API 的 src 内 `#[test]` 迁到 `tests/`;被迫 `pub` 泄漏内部才能迁的,不迁。
- 对照 `./cpp` 的测试行为(输入→期望输出),补 cpp 覆盖而 Rust 缺的用例;
  删除 AI 编造的无依据假 mock / 未闭环竞态垃圾测试。
- 以行为等价为准重写实现后,测试必须证明新实现与 `./cpp` oracle 产出一致(可留
  cross-check 型 oracle 测试对照 libc/C++ 输出)。

## 9. 验收指标

1. `./sh/clippy.sh` 无报警、无 allow。
2. `./test.sh` 全绿,且测试覆盖相对改写前不减少。
3. 全库 `rg` 自检:`dyn`(除已论证处)、`null_mut`/裸指针、`unsafe`(除 FFI 边界)、
   `#[test]` 在 src、`_alt_` 拆分、`ffi` 误用 —— 命中项要么清零,要么有明确保留理由。
4. 行为与 `./cpp` 一致(正确),关键路径不比改写前慢(高效)。
5. 被根 `Cargo.toml` `exclude` 的游离 workspace(`fuzz/`、`benchmarks/runner/`)不在
   `./sh/clippy.sh` 的遍历范围内,须另跑 `./sh/clippy_extra.sh` 保绿(它自带 `cargo fmt --all`)。

## 10. C 字符串 Rust 化(新增,强制)

crate `rg "CStr|CString -t rs` ； 全部 Rust 化 ( ulua-capi 是 C ABI 实现层,豁免 )
