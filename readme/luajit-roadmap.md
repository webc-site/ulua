# 追平并超越 LuaJIT 2.1：路线图

依据：23 基准 × (解释器 / JIT) 对 LuaJIT 2.1 同模式实测（2026-10-02，M2 Max，
results.json 时间戳 2026-10-02T10:53:59Z）+ 文献调研（出处见文内链接）。

## 现状

| 模式 | 几何平均（ulua/LuaJIT） | 落后重灾 |
|---|---|---|
| 解释器 vs 解释器 | 1.05 | nsieve 3.2、coroutines 2.5、fib 2.1、patterns/micro_call 1.5 |
| JIT vs JIT | 2.53 | oop 16.8、spectralnorm 8.7、micro_call 6.8、nsieve 5.6、fib 4.3、nbody 3.8 |
| ulua JIT vs LuaJIT 解释器 | ~0.8（多数 <1） | nsieve 2.8、coroutines 2.5、patterns 1.5 |

已定性排除：coroutine 差距 ≠ 栈切换（LuaJIT 同为可重入解释器模型，
yield=跳出循环存 pc/base、resume=重入；差距是 re-entry 固定开销）；
codegen pass 吞吐（实测 1.25ms/模块）；表增长 vs cpp（微基准反超 1.6x）。

## 解释器组（目标 1.05 → <1.0）

### E1 稀疏整数键 array 直达——**已实测否决（语义红线）**
nsieve 剖证：插入慢路径合计 ~40%（newkey 21.3% + resize 7.7% + rehash 6.5% +
s_settable/lua_v_settable 3.2%），COUNT 循环读走 lua_h_get 哈希查找 18.7%——
`sieve[j]=true`（j=i²..n 步进 i，稀疏升序）在哈希部分 + rehash 迁移；LuaJIT
`lj_tab_setinth` 对 1≤k≤MAXASIZE 直接扩 array part。已实现受控扩容
（4×sizearray+16 上界 + next-pow2 resize）：nsieve **-32.7%**（44.1→29.7ms），
但 `#t` border 随布局改变（b 案 16 vs cpp 4、h 案 3 vs cpp 1；base 与 cpp 逐位
一致），**conformance_tables 当场红**（另触发 close_state 断言）→ 判定：表布局
语义被 cpp oracle 钉死，LuaJIT 式增长属语义分叉，不可发货。nsieve 3.2x 记为
**语义受限差距**；残余可做：cpp 同构下哈希路径微成本（getfreepos 扫描、
setnodekey 拷贝宽度），收益有限。

### J4 census 裁决（2026-10-02，读数已入册）
oop native census（3 轮 + 预热，TSFB 全站点）：主体 proto#0 的 16 个 KS 站点每站
仅 4 次 miss（= 启动调用数，稳态零 miss——单态 IC 全命中）；热点集中在
proto#1 pc=6：**480K 次 miss、last_tag=7（table 恒定）** = spike 剧本一的
**插入型 SETTABLEKS**（每轮新表构造 `{x=x,y=y}`，键缺席 → IC 无槽可命中）。
- PIC-2 对插入型站点**无收益**（键缺席无缓存可命中）→ J4「2 槽 PIC」降级搁置；
- oop 的实际杠杆 = ① `SETTABLEKS` fallback 的**插入快路**（fresh-slot 直插，
  绕过 execute_settableks 全帧路径）+ ② NEWTABLE 尺寸提示消费；
  **J4b 实施方案（2026-10-02 核查定稿）**：
  - NEWTABLE 提示已被消费（oop 构造器 `NEWTABLE R 4 0` → 预分配 8 哈希槽，
    h_newtable `1<<(B-1)` cpp 同构）——插入 miss 是键缺席本质，非 rehash churn；
  - cpp `luaH_newkey` 语义核查：主位空（非 dummy）→ 直接 setnodekey 落位，
    **无 freepos/rehash 参与**（碰撞或 dummy 才走）——inline claim 语义有 cpp 背书；
  - 实施件：① `CheckNodeEmpty` 新原语（val.tt==NIL 且 node≠dummy——
    dummynode 地址需入 NativeContext）；② 落位三 store（value 拷自 k[aux] 的
    value 字段、extra=0、tt=STRING——cpp setnodekey 将 extra 清零，
    **不可用 StoreTvalue 16B 整拷**）；③ key 屏障（cpp `luaC_barriert(L,t,key)`，
    key 虽在 proto 常量表仍需屏障——proto 可能先于表死亡）；
  - 预期：插入路径 100+ 周期（全帧 helper call + VmFrame + 查找）→ ~25 周期
    （主位 hash+空判+3 store+屏障 ≈ 内联），oop 480K 次/轮 → **-30~40% oop JIT**；
  - 工作量：新 IrCmd ×2 + A64 lowering + NativeContext 槽 + census 读数，半天级。
- 观测基建（P0 门控修复 + SETTABLEKS bump 对称接线 + census 探针）已就位。
- **cpp 对照定性（2026-10-03）**：cpp `IrTranslation.cpp:100` 对 ANY 接收者的
  CHECK_TAG miss target 同为 fallback（与我们逐位同构）——**元表实例的属性读在
  cpp native 同样每次走 helper**（Roblox DevForum 官方承认 native codegen 对 OOP
  无收益）。480K miss 的真因定性：**元表实例的属性读（含继承键）恒走 fallback**
  ——非形状轮换、非插入型，是 method-JIT 对 metatable'd read 的固有形态。
  **J4-real（超越 cpp 的新特性）**：__index 感知 IC——对带元表接收者缓存
  (metatable ptr, __index 表 node val 地址)，guard 链 = CheckTag(table) +
  mt cmp + 键校验，miss 落 helper。
  **J4b freepos 扩展探针终审（2026-10-03，诊断前置模式）**：480K miss 定性为
  **表满 rehash 形态**——Vec.new 的 `self.z = z` 插入 2 槽模板表（DUPTABLE 提示），
  每插必 rehash（2→4），48 万次无一幸免；主位空直插（J4b 已落地）前置条件永不满足。
  链写快路四原语实装验证：语义正确（acc 逐位一致、链走通）但 oop 纹丝不动 →
  按退出条款还原。可行方向需主控裁断：DUPTABLE 模板增槽（2→4）改 pairs() 迭代序，
  属 E1 判例的语义分叉红线。
  **opcode 已实测定案（TSFB opcode 读数）**：480K miss 站点 op=20 = **NAMECALL**
  ——`a:dot(b)` 方法派发在 __index 链实例上每次落 helper（我们的第二快路
  fastgettm(TM_INDEX)→类表槽查在该形态未命中，cpp 同构同症）。J4-real 的
  首个靶点即 NAMECALL 的 __index 链缓存。LuaJIT 以 trace 级做到；cpp 无；我们做成即
  oop JIT 从 25.3 向 interp-LuaJIT 差距方向实质逼近，且为对 cpp 的净超越项。
  工作量：大（guard 链 + GC 失效语义 + census 选点），需专项立项。
- **J4b 落地后 oop 实测（CPU 时间配对）= +0.6%（零收益）**：census 的 480K miss
  计数正确但其成本占比被高估——execute_settableks 的 direct_set 路径本已精瘦
  （VmFrame 门面 + set_str 主位查找 + patch_c），内联化省下的 call 开销 ≈ 测量噪声。
  合成插入热负载 -19% 为真（helper 调用占绝对主导时成立）。J4b 作为机制保留；
  oop 残余成本在别处（分配/GC/字符串 intern），非 fallback 路径。

### E2 CALL/RETURN 快路内联 + arity 特化（fib 2.1 / micro_call 1.5 / binarytrees 1.4）

**2026-10-03 J2 单态调用内联实施失败（已否决，未合并）**：子代理实现 CALL lowering
内联 native-callee 快路（守卫 + 内联建帧 + 直 br callee exectarget，合成负载
fib -25%/micro_call -22%/三靶束 -18%），但独立复核 **conformance_tables 红**
（getheaptrigger.rs:24 panic——内联快路与堆触发器交互缺陷）。**未合并**，分支
opt-j2-inline 6513efd9 保留供排查。教训：子代理自报门禁全绿不可信，独立复核
是合并前置必需（本次为独立复核第二次抓到子代理漏报）。
**同日闭环**：根因 = getheaptrigger.rs:24 的 i64 加法溢出（预存缺陷，J2 快路
提速 ~20% 使 GC 时序角落可达）——并行 r16-d1 票的 saturating 算术修复已在 dev，
J2 合并时冲突去重保留 dev 版后 **conformance_tables 显式复验通过**（6/6 绿），
J2 CALL 内联本体已随 8ce356de 在 dev 生效（fib/micro_call 三靶 -18~25% 待全量
bench 确认）。

**2026-10-02 补充实测**：本轮核查发现 `call_arm` 宏已是 cpp 式内联快路
（Lua→Lua 调用直接 `continue` 同一循环，无 performcall 重入）——E2 的
「重入开销」前提不成立。剩余差距 = 派发次数 + 每调用帧建立开销（Rust ~20+ 指令
vs LuaJIT asm ~10），前者已到融合收益边界：micro_call 的 `LOADN → JUMPIFNOTLE`
（13.7% 边）融合实测 +1.2%（不显著）、fib 付探针税 -10% 风险 → 还原弃用
（LOADN 尾过热，单一用例的边不配探针）。call-boundary 边（Subk→Callfb 等）
不可融合（帧转换）。fib/micro_call 的剩余差距记为**调用帧建立成本的结构性差距**；
可做残余：checkstackfornewci 与 ci 字段写的微成本（需 samply 精确归因后另立项）。

**2026-10-02（第二次）E2-a 复测终审**：`LOADN → JUMPIFNOTLE` 融合以 CPU 时间
（getrusage 配对，13 轮交错）复测——micro_call +0.8%、fib +0.2%、mtg +1.3%，
三用例一致中性偏负，永久弃用。根因定论：LOADN 尾已有 SETTABLE/JUMPIFNOTLT
双探针，micro_call 命中 JUMPIFNOTLE 前需吃 2 次 probe miss，与省下的 1 次派发
零和。LOADN 尾探针已饱和，后续任何 LOADN 边（不同负载后继不同）的融合预期
收益为负，不再尝试。

**2026-10-02 E2 残余归因的教训**：samply 剖证必须先看样本量——fib 归因仅 429 样本，
「SETTABLE 探测 miss 占 7.2%」实为 31 个样本的统计噪声；据此做的双探针换序
（JUMPIFNOTLT 先探）在 CPU 时间（getrusage RUSAGE_CHILDREN，抗调度噪声）配对下
fib -1.2%、mtg -7.5%、micro_call +3.4%——方向与逻辑预期相反，纯属布局彩票，
已还原。结论：①单热点归因需 ≥5000 样本（拉长采样窗或提高采样率）；②微改动
（探针顺序/代码位移）的 A/B 必须用 CPU 时间 + 多用例交叉验证，wall-clock 在
共享机器上不可用。
fib/micro_call/binarytrees 的转移表被 CALLFB/RETURN 边主导（各 15%），
融合无法跨调用帧。LuaJIT asm fast path：判 Lua 闭包 + nargs==nparams + 推帧
全在解释器循环内完成，不重入。落点：`luau_execute.rs` CALL 臂内联 precall 的
PCRLUA 分支（写 CallInfo、参数直拷、continue），`performcall.rs` 降级为 C 边界
专用；RETURN 臂消费 `LUA_CALLINFO_RETURN`。预期 fib→1.3、micro_call→1.1。

### E3 coroutine re-entry 轻量化（coroutines 2.5）
yield 只存 pc/base 快照（已有）；`resume_continue.rs` 恢复 4 状态量后直跳
dispatch，入口一次性逻辑（VM_HAS_NATIVE、SCHEDULED_REENTRY、checkcstack）
拆 `#[cold]` 首次分支。预期 coroutines→1.3。

### E4/E5（余量）：gslot_hit 判据合并；装载期超级指令融合（ADD+JMP 等运行期计数驱动）。

## JIT 组（目标 2.53 → <1.0）

路线 = J1 → J2 → J3 → J4，全部复用现有 feedback_vector_slot +
analyze_bytecode_types + 线性 IR/块链设施（LBBV 论文数据：95% 块单版本，
method JIT 内可拿 tracing 的大部分类型特化收益）：

- **J1 BBV 类型版本化**（Lazy Basic Block Versioning, Chevalier-Boisvert &
  Feeley, ECOOP'15）：guard 失败按反馈类型重编译块，后继沿类型传播——删 tag
  检查、int 索引直达 array。落点 `analyze_bytecode_types.rs`、
  `build_bytecode_blocks.rs`、`records/feedback_vector_slot.rs`。预期 JIT -30~40%。
- **J2 单态调用内联 + fastcall**（PIC ECOOP'91）：调用站点 guard 闭包指纹，
  内联被调体/直跳入口。预期 oop 16.8→4、micro_call 6.8→2。
- **J3 HREFK 式表特化**（lj_asm.c asm_hrefk）：常量键 hash 编译期折入 +
  next 字段验证（已部分具备）。预期 nsieve/micro_arraywrite 残余。
- **J4 IC 单槽 → 2~4 槽 PIC**；**J5 profile-guided block ordering +
  循环超块**（Pettis & Hansen PLDI'90）；**J6 regalloc2**（spill 热度确认后）。

诚实上限：纯 FP 循环（spectralnorm）是 method JIT 对 trace JIT 最难档，
J1+J5 到 1.1~1.3x；综合 <1.0 需要 J1+J2+J3 全部到位且站点类基准反超拉开。

## 明确不做

- corosensei 切栈协程（模型不等价且对本基准更慢）
- cranelift/inkwell 后端、dynasm-rs 迁移（现有手写 assembler 已工作）
- patchouly/copy-and-patch（nightly 依赖 + 定位是 baseline tier 不是优化 tier）

## 实测记录

- 2026-10-02：nsieve 融合补全（Fornloop→Loadb / Gettable→Jumpif /
  Jumpif→Fornloop，含 fuse 函数路径的探针安装）——功能正确（探针证实触发、
  指纹一致），但派发仅占 nsieve 16%，实测中性（-0~2% med，不达 5% 门）→
  还原弃用。教训：融合前先看剖面，nsieve 的时间在表慢路径（40%）与哈希读
  （19%），不在派发。

- 2026-10-04：JIT 环级 tag 外提（loop-level tag hoisting）——**实测证伪，
  故未采纳**。机制假设：method JIT 热环每迭代付 LoadTag+CheckTag（TValue
  16B 装箱税），把「环内已证 number」的 tag 检查外提出循环可让算术落
  XMM/V 短链。实现（已还原）：全函数 tag MUST 数据流（工作表迭代不动点，
  传播规则镜像 OptimizeConstProp 的 save/invalidate 臂），产出各块入口
  已证 tag，经 `setup_block_entry_state` 灌入唯一一次块链遍历，kill 走
  既有 CheckTag/LoadTag 臂；旗标 `LuauCodegenLoopTagHoist`。
  - 静态面成立：micro_arith 环内 CHECK_TAG 11→4（剩余 4 处均为 fallback
    DO_ARITH 写同槽的语义必要检查，元表可返回任意类型，静态不可证）；
    a64 原生码热环 tag 检查三连（ldr w,[x25,N]; cmp #3; b.ne）12→1，
    proto 代码量 1008→704B（-30%），i%48→fadd→fmul 全程驻 d 寄存器连续
    浮点链。定向单测钉住 kill 面与 fallback 保守面均按设计工作。
  - 配对评测定谳（同二进制 `--fflag` 开关，ABBA 交错，off 自身地板
    2.2~5.4%）：JIT geomean **-0.4%（噪音内）**；直接靶 micro_arith
    +2.4%（地板 4.9%，中性）；spectralnorm/mandel +1.4%/+0.5%（地板内）；
    **nbody +4.9% 稳定超地板回退**（地板 2.2%，两轮独立复测 +6.6%/+4.9%），
    踩「回退 ≤2%」红线。nbody 环体 970→893 条（检查确实被杀）但读数变慢，
    定性为 fallback 块连带死亡引发的全局块布局重排彩票。
  - 成因链：a64 微架构下 `ldr+cmp+b.ne` tag 检查与浮点链无数据依赖，
    分支预测命中 + 超标量窗口把检查税基本吸收——**依赖链主导的 FP 环里
    删检查不产生收益，只产生布局扰动**。与 FORNLOOP 整数快路证伪同构
    （该处检查税同样被 FP 依赖链吸收）。结论：解释器侧/JIT 侧残余差距
    （spectralnorm 5.2x、nbody 3.98x）不在此形态，杠杆仍在 J1 BBV 类型
    版本化（GETTABLE 读侧类型传播删检查——写侧证据缺失正是本轮保守面
    不许证的部分）与 J5 布局排序。代码已完整还原，无残留实现。
