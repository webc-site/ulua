# J1：BBV 类型版本化——调查与设计（2026-10-02）

目标：在现有 per-block method JIT 内拿到 tracing 的类型特化收益
（LBBV, Chevalier-Boisvert & Feeley, ECOOP'15：95% 块单版本）。
预期 JIT 组 -30~40%（spectralnorm/nbody/matmul 的 tag 检查与装箱消除）。

## 现有基建盘点（可复用面）

| 组件 | 现状 | J1 角色 |
|---|---|---|
| `analyze_bytecode_types.rs`（516 行） | 编译期静态类型推导（cpp IrAnalysis 移植），LBC_TYPE 沿字节码传播 | **类型传播引擎直接复用**：把「观测到的运行时类型」作为入口类型喂给它重跑，即得特化版类型表 |
| `feedback_vector_slot.rs`（53 行） | 运行时反馈槽已存在，唯一变体 CallTarget{pc,proto,hits}（CALLFB 用） | 扩新变体 `TypeSlot{tag:u8,hits:u32}` |
| `NativeProtoExecData` extra_data | execdata 尾部有 `extra_data_count` 扩展区（现有机制） | **类型反馈侧表挂这里**，不改字节码格式（关键决策：字节码格式改动触碰编译器/装载器全链，否决） |
| fallback helpers（FallbackGettableks 等） | guard miss 时已在 VM 侧、持有失败 pc 与**真实运行时 tag** | 天然的类型观测点：miss 时把观测 tag 记入侧表 |
| `build_bytecode_blocks.rs`（90 行）+ lowering | 块构建/降级以 `get_bytecode_types_at(pc)` 取静态类型 | 块版本化的编译入口：同一 pc 可按不同类型上下文产出多版本 |

## 三阶段设计

### Phase 1：类型反馈采集（无版本化，可独立落地）
- execdata extra_data 追加 `TypeSlot[pc]`（仅热点指令类别：GET/SETTABLEKS、
  ADD/SUB/MUL、COMPARE、NAMECALL；其余 pc 不占槽）。
- 两处观测：① guard miss → fallback helper 记「失败前真实 tag」；② 快路命中 →
  记「守卫通过的 tag」（置位数组，relaxed store）。
- 落点：`native_proto_exec_data_*`（侧表布局）、各 fallback helper（观测写）、
  `check_table_tag_guard`/`check_number_tag_guard` 的 miss 分支。
- 交付后即可回答「哪些站点多态、什么类型占多少」——J2/J3/J4 的选点依据。

### Phase 2：暖重编译（warm recompile，BBV 核心）
- 侧表 hits 越阈（如 1024）且 tag 占比 >90% → 该 proto 标记「按观测类型重编译」：
  把观测 tag 作为 `analyze_bytecode_types` 的**入口约束**重跑（静态引擎已有，
  输入从 ANY 换成观测类型），lowering 产出删 guard 的特化版本。
- execdata `instruction_offsets[]` 是可变数组：特化版 asm 写入后**原子换 offsets**
  （entry 不动，旧代码段保留给已在途帧）。
- 风险与对策：① fallback helper 内触发重编译的重入（code allocator 锁 +
  自改代码）——重编译**投递到 VM 出口**（return 前检查 pending 标记），
  不在 native 执行流内做；② 多版本空间——只保留两版（any 版 + 观测版），
  观测版 guard miss 回 any 版即自然去优化。
- 落点：`compile_internal.rs`（带入口约束的重编译入口）、
  `native_proto_exec_data_*`（offsets 原子换装）、fallback helpers（投递标记）。

### Phase 3：链式传播 + 去优化闭环
- 观测入口类型沿 `analyze_bytecode_types` 传播 → 后继块链的类型全部特化
  （引擎已有，零新增）；观测版 guard miss → 侧表记失败 tag → 回 any 版。
- 循环超块（J5）在此形态上才值得做。

## 风险登记
1. **重编译重入**（最高）：执行流内禁止 code alloc，必须出口投递。
2. execdata 换装的原子性：offsets 数组整体替换（指针换装，非逐项改写）。
3. 与 LOADN→SETTABLE 等臂尾融合的交互：融合链假设「单版本」，版本化后
   探针判定要在版本内重新校（Phase 2 落地时回归全部融合用例）。
4. 编译吞吐：暖重编译在运行期付费——仅对过阈热点 proto 触发，
   codegen pass 现为 1.25ms/模块量级，可接受。

## 与路线图其余项的关系
J2（单态内联）依赖 Phase 1 的 CallTarget 反馈已有雏形；J3/J4 在 Phase 1 的
类型分布数据上选点。Phase 1 独立可交付且为 J2-J4 铺路，建议作为 J1 的第一票。
