# 追平并超越 LuaJIT 2.1：路线图

依据：23 基准 × (解释器 / JIT) 对 LuaJIT 2.1 同模式实测（2026-10-02，M2 Max，
results.json 时间戳 2026-10-02T10:53:59Z）+ 文献调研（出处见文内链接）。

## 现状

| 模式 | 几何平均（ulua/LuaJIT） | 落后重灾 |
|---|---|---|
| 解释器 vs 解释器 | 1.05 → **1.04**（2026-10-05 全量复测） | nsieve 3.2、coroutines 2.5→**1.92**、fib 2.1、patterns/micro_call 1.5 |
| JIT vs JIT | 2.53 → **2.71**（口径漂移见 JIT 组重测） | oop 13.9、spectralnorm 7.2、micro_call 5.3、inherit3 44.8→**~39** |
| ulua JIT vs LuaJIT 解释器 | ~0.8（多数 <1） | nsieve 2.8、coroutines 2.2、patterns 1.3 |

2026-10-05 全量收束（bench.sh 全后端全分组）累积收益全景：E3 协程边界链
（coroutines 2.5→1.92）、E5 链融合 + J4r 链走扩深（life interp -2.6~-4.2%、
inherit3 -10.3%）、patterns 内核（patterns jit 15.0→18.7 反超 LuaJIT jit，
ulua 19.5 vs cpp 26.3）；解释器组对 cpp 全组 **0.78x（快 22%）**。

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

**2026-10-04 部分落地（恢复链直连 + Lua 帧热路）**：先剖证后动手——samply
（30 runs 窗，≥2100 样本，符号化 leaf self-time）测得边界机器（resume/yield 全链）
合计 ~19.4%，与 cpp 同测（~20%）同构同占比；同刻配对 coroutines ulua 69.9ms vs
cpp 104.4ms（快 1.49x）——「coroutines 2.5x」只对 LuaJIT 解释器成立，不是 cpp
差距源。两项采纳：
① resume 回调直连：`lua_d_rawrunprotected_resume`（`catch_unwind` 壳共用
`rawrunprotected_shell`，非异常载荷解码外提 `decode_error_payload`），省 Pfunc
Option 判空 + 全 VM 共享单一 `blr` 的间接分发；汇编核对 `resume`/`resume_start`/
`resume_finish` 热路全部折叠进 `lua_resume`，仅 preresume 钩子 `blr` 残留。
② resume_continue Lua 帧热路：OPYIELD/singlestep/LUA_CALLINFO_NATIVE 三旗排除后
持已取闭包直达派发层（`tier_reentry_hot` → `tier_cold`，`#[inline(always)]`），
省运行期 singlestep 分发与 `ci->func→Closure` 重复取值链；`pc/base/k` 仍从
`L->ci`/`L`/proto 现读，口径与 `vm_state_from_ci` 逐位一致。OPYIELD 续体可再入
执行，保持全量 `luau_execute` 不变。
配对 A/B（ABBA 交错、多组独立复测）：coroutines 稳定 7/7~9/9 配对胜，best
-4.7%、med -3.4~3.9%，多次过/贴 5% 门；fib、micro_arith 终审中性（micro 用例
0.8~3ms 窗低于噪音地板，判据不适用）。被实测否决（均还原，未采纳）：
`decode_error_payload` 标 `#[cold]`（micro_arith +3.7% 布局彩票，隔离进程复测
仍在，两布局下 tier_cold 字节数逐位一致）；环头 while→嵌套 if 拆分 +
`lua_resume #[inline]` 折进 auxresume（coroutines 反而 -2.5%，micro_arith +3.6%
回归）。定向用例 `resume_hotpath.luau` 钉热路语义（上值/元方法/常量/pcall 帧
yield 续跑/跨恢复错误逐值断言）。残余定性：边界各叶函数均已是直调小函数，每恢复
~10 次内存往返（两状态量写+读+FIND_HANDLER 语义）属语义必需面，函数级内联已到
边界；对 LuaJIT 残余差距需状态封装层重构（恢复协议批量携带状态）才有下一档，记
结构受限。

### E2 残余归因（fib 调用帧建立）——2026-10-04 剖证定谳：结构受限，微项否决
按前案「checkstackfornewci 与 ci 字段写的微成本需 samply 精确归因」立项执行：
fib 解释器 80 runs 采样（6257 样本，地址级桶入 tier_cold 反汇编）——99.7% 自时
集中于派发环单函数，precall/checkstackfornewci/ci 写面已全部内联（J2 成果）。
地址级分布：派发头（取指+查表+间接跳转）~22%，CALL/CALLFB 臂（VM_INTERRUPT +
解码 + incr_ci + 7 字段 ci 写（已 stp 合并）+ 栈检查 + 补 nil + 状态重载）~34%，
RETURN 臂（同款中断检查 + 结果拷回 + 弹帧）~17%，LOADN/LOADK/算术 ~25%。最热
单指令 = RETURN 臂入口首装载（12.9%）——间接派发重定向的流水线回填驻留，非可
消除指令；VM_INTERRUPT 七处放置与 cpp oracle 逐位同形，粒度语义被钉死。每调用
帧建立 ~20+ 指令 vs LuaJIT asm ~10 的差距属 CallInfo 数据模型结构（LuaJIT 帧即
栈上 2 字），oracle/调试协议钉死不可换。可抠微项（call_arm stacksize 重复装载
~4 周期/调用、派发表 91→256 项消边界检查~1-2 周期/派发）合计预估 2-4%，低于
5% 门——**实测未动手即否决**（归因成本 > 预期收益）。fib/micro_call 解释器残余
差距记为派发重定向 + 帧建立的二重结构受限，收敛于解释器组的诚实上限。

### patterns 模式匹配内核去移植税——2026-10-04 落地（-18.7%）
samply 归因（patterns 逐字符路径）：match_item 34.4% + matchbracketclass 10.5%
+ match_class 7.0% + classend 6.7%，派发仅 6.6%——**实现受限**（对比 fib 99.7%
单函数），与 cpp 同刻配对 ulua 24.1ms vs cpp 26.2ms 已反超，1.5x 差距仅对
LuaJIT。定标后三项移植税消除：① `match_class` 十一分支谓词改两张编译期 const
表（cl→类 id+取反位、类×256 字符谓词位图），两次对齐读+异或取代 tolower+多重
范围比较，附 256×256 全值域对账测试（naive 参照留 #[cfg(test)]）；② 字符类
窗口切片免钳制（`pat_slice_exact`，`get_unchecked` 区间，契约由 classend 的
ep ≤ len 建立，window() 的双 min+saturating+切片检查移出逐字符路径），bracket
循环 `as_ptr` 走读（不变式逐臂证明）；③ `singlematch`/`classend` 强内联折回
逐字符调用路径（fat LTO 下 LLVM 主动放弃的三处真实 bl/ret 边界；singlematch
×3 拷贝、classend ×2 拷贝，matchbracketclass 跨拷贝共享）。配对评测（ABBA
7 轮 + 聚焦 5 轮）：patterns med **-18.7%**（24.6→20.4ms，胜率 7/7、5/5），
全 exec 组复测无超阈回退（strings jit +6.0% 复测定谳布局彩票，med -0.4%；
fib interp +1.9% 复测 med -0.3%）；strings interp 附带 -2.4%、micro_tablegrow
interp -10.3%（同向彩票照收）。门禁：285+112 全绿（含新对账测试）、clippy/fmt
零告警、wasm 双目标 check 过。子代理审查「可合并」，三条注释级建议（界证明
`i+1 ≤ ec` 失真订正、singlematch 文档 unsafe 面同步、「十个分支」笔误）已收编。

### strings 字符串 intern 管线——2026-10-04 剖证归因：否决（含 format 栈缓冲化实测无收益）
上轮 patterns 剖面的 `lua_s_newlstr` 6.67% 旁证立项。strings 解释器 4896 样本
归因：intern 管线合计 ~48%（lua_s_newlstr 21.6% + memmove 9.1% + lua_s_resize
6.3% + 分配/GC ~10.8%），str_format 族 ~20.8%，派发仅 12.6%——实现受限形态。
地址级拆解 lua_s_newlstr：桶链扫描 ~9.4%、未命中分配路径 ~5%、哈希 ~4.8%
（逐位红线：哈希须与 BytecodeBuilder 复刻一致，算法不可换）；桶链/分配/哈希与
cpp `luaS_newlstr` 算法同构，growth 条件（nuse > size 翻倍）与 cpp 逐位一致。
str_format 的 per-directive `Vec<u8>` 堆分配（assemble 3.57% + radix_bytes 2.47%）
对照 cpp 栈上 `char buff[MAX_ITEM]`，实现为 FrameBuf（MaybeUninit 栈缓冲 +
窗口分段）全面栈化：**实测 strings med -0.4%（best -1.9%），远低于 5% 门**
——malloc/free 自时主体是 newlstr 的串分配而非 format 的 Vec（每条 strings.lua
指令的分配摊派 ~1%），且 patterns 布局回退 +2.5% 超阈 → 按纪律整体还原。
结论：strings 上 ulua 已快 cpp ~1.44x，且**不在对 LuaJIT 的重灾名单**（geomean
1.05 的贡献者非重灾项），intern 管线与 cpp 逐位同构、对 LuaJIT 的残余为 C 级
哈希/分配实现差。format 栈缓冲化实测无收益，故未采纳；后续若做需以 LuaJIT
lj_str_new 的内联哈希对照另立专项。

### E4/E5（余量）：gslot_hit 判据合并；装载期超级指令融合（ADD+JMP 等运行期计数驱动）。

## JIT 组（目标 2.53 → <1.0）

### 2026-10-04 同刻重测（ulua-jit vs LuaJIT 2.1 JIT，3 runs 交替双二进制）
geomean **2.70**（n=24；10-02 口径 2.53——patterns/oop 站点负载下白噪漂移，分布
重排：J2/E3/patterns 落地后 micro_call 6.8→5.33、fib 4.3→3.01）。重灾榜更新：
inherit3 **44.75x**（17.9ms vs 0.4ms）、oop 13.91x、spectralnorm 7.20x、
micro_call 5.33x、micro_arraywrite 5.00x、nbody 4.09x、fib 3.01x、matmul 2.88x；
反超项 life 0.68、tablesort 0.51、binarytrees 1.27。

**口径勘误（2026-10-04 晚，同日自家 interp 对照复测）**：初版把 u/luajit-interp
列误读作「native vs 自家 interp」。实测自家对照——inherit3 interp 25.3 vs jit
17.9（**jit 快 30%**）、micro_call 39.2→17.6、oop 24.8→15.3、matmul 27.2→12.1，
native 全面快于自家 interp；**唯一例外 coroutines**：interp 64.9 vs jit 78.3
（jit 慢 20%——协程跨 yield 退 interp，每次 resume 重付 helper/状态重建，J4r 系
候选靶之二）。初版「native 比解释器慢 50%」表述作废；helper 重入税的论据不依赖
该误读（helper 全链占比与 LuaJIT 45x 差距为直接实测）。

归因两条（samply 符号化 leaf）：
- **inherit3**：helper 全链 ~38%（index_chain_probe 22.4%——J4-real 链缓存已在
  helper 内命中、execute_namecall 5.9%、lua_v_gettable 5.5%、lua_t_gettm 4.3%），
  native 本体仅 ~5%。残余 = 每属性/方法调用的 helper 重入税（native→helper 寄存
  保存 + VmFrame + 守卫走查 + 返回）。正解 J4r-step2（native 恒定链守卫）：在
  lowering 里对单态站点直列 `load epoch → cmp 编译期值 → b.ne fallback + load
  mt → cmp 常量 → b.ne fallback` 守卫序列，复用 index_chain_cache 现成的
  epoch/Bloom 失效基建与 owner 段哈希（rehash 安全不缓存槽地址）；cpp native 无
  此形态（Roblox 承认 native 对 OOP 无收益），做成即净超越。立项级工作量。
- **matmul**：962 样本 100% 集中于 JIT 生成代码本体（helper 零出现）——2.88x
  差距纯在生成代码质量（寻址/寄存器压力），即 **J6 regalloc** 的实证靶（此前
  J6 挂起条件「spill 热度确认」就此确认：单一热环即命中生成码质量问题）。
- 立项次序建议：J4r-step2（inherit3 44.75x 空间最大、基建已备）→ J6（matmul
  矩阵寻址）→ J1 BBV（全局）。

### coroutines jit 回退（-14%）归因否决（2026-10-05）
fork opt-coro-jit 实测：interp 65.1~67.5ms（地板 3.7%）、jit 74.0~74.9ms（地板
1.2%），jit 慢 ~14%。samply 80 runs（6013 样本）归因 jit 侧：call_fallback
18.4%（resume 主循环与协程体的 CALL 均为 C 函数调用，走通用 fallback）+
resume 边界机器 ~60%（interp 同款）+ on_enter 2.8% + jit 本体 ~10%。
**「协程体拒编 native」启发式否决**：初选判据「proto 含 YIELD 指令」不成立——
Luau 字节码无 LOP_YIELD，`coroutine.yield()` 编译为 GETIMPORT + CALL（C 库函数
调用），静态层无 yield 痕迹。运行期自适应摘除（lua_yield 咽喉检测 + execdata
摘除）评估后同样否决：需引入 proto 拒编标记（cpp ABI 面）、处理暖重编译复发
（force_recompile 会把摘除的 proto 重编回来）、NativeModule 引用计数交互——
工程面超收益。call_fallback 的相对劣势属 J2 native CALL 系通病（对 C 函数调用
不可避免），coroutines jit 慢 14% 记为**结构受限**；候选残余 = native fallback
CALL 的 C 函数快路（luaB_yield/resume 直通），随 J2 native 系一并立项。

### J4r-step2 预研轮（2026-10-04 晚）——发现 codegen DSE 块链 live-in 盲区，回退
精确缺口定位：NAMECALL lowering 的 native 链走只内联**前两级**（照 cpp
luaV_gettable 内联前两级），inherit3 的 speak(3 跳表)/describe(4)/breathe(5) 超
深部分全落 FallbackNamecall helper——probe 22.4% 的来源即此。实现尝试：缺席证明
+ 探测的「absent→probe」结构参数化扩深两级（extra 块对同构循环生成）。两处工程
拦路，第二处为基建级：
1. collect_direct_block_jump_path 的 `get_live_out_value_count == 0` 前置断言——
   probe 块以 JUMP next（多前驱目标）收尾且携带跨块活值 cur 时触发。解法已验
   证：取值收口垫**单前驱 join 块**（second_fast_path_hit_join 同形，
   use_count==1 免折叠）。
2. 解掉 1 后撞出 **update_remaining_uses 悬空引用**（conformance_native_userdata
   复现，DSE 阶段 extra_absent[1] 的 GetHashNodeAddr 引用已被判死的
   extra_probe[0] LoadPointer）——**DSE 块链遍历对「条件边目标块消费上游块定义」
   的 live-in 存在盲区**。原版两跳恰好规避（probe 块产物 next_index 无跨块消费
   者）；扩深第一次引入该形态即踩中。二分证据：CHAIN_EXTRA_HOPS=1 全绿（cur 无
   跨块消费）、=2 必炸；诊断打印定位 inst 84→dead 77。HOPS=1 语义上只覆盖
   describe、breathe 仍落 helper，收益不足门。
**按纪律完整还原，未合并**。J4r-step2 的前置条件改为：修 DSE 块链遍历的条件边
目标 live-in 回传（或收集器豁免形态），属 codegen 基建级修复，需专项轮；修后
扩深本体（已验证到断言前的完整实现路径）即可复用。

### DSE 盲区修复 + J4r-step2 链走扩深——2026-10-05 落地（inherit3 -10.3%）
前置修复 + 扩深复用一次闭环（fork opt-dse-livein，重放预研轮已验证的扩深实现）：
1. **根因反转**：预研轮定性「live-in 盲区」经轨迹打印证伪——remaining[77] 归零
   的真实机制是 **remaining 置位遍历的顺序假设缺陷**：DSE 块链按「单前驱 JUMP 链
   + 新链起点」多链遍历，条件边目标块（extra_absent[1]）先于其值定义块
   （extra_probe[0]）被遍历（诊断轨迹：遍历 84 时 77 set=0 未置位）。原单遍历
   （置位+递减一体）隐含「遍历序 = 拓扑序」假设，多链形态不保证。
2. **修复**：置位与递减拆两阶段——外层入口对全函数（含 Fallback/Dead 块）先置位
   use_count，块链遍历只做参数递减，顺序无关；归零判定语义不变（remaining =
   全量引用计数 − 链内递减）。子代理审查对照 cpp OptimizeDeadStore 确认 cpp 单
   遍历同有拓扑序假设，我们的形态变化属新触发面，拆分方向正确。
3. **扩深复用**：NAMECALL 链走 absent→probe 结构循环生成两级（覆盖到第 5 跳
   表），取值收口垫单前驱 join（collect 断言豁免已实证）。配对评测（7 轮 ×
   runs5，A 侧读数极稳 15.6-15.8）：inherit3 med **-10.3%**（17.5→15.7ms，
   7/7 配对胜），全 jit 组回归干净（geomean +0.7% 净正向，micro_arraywrite
   +20% 复测 15 runs 完全一致=彩票定性）；oop +0.7% 中性。门禁：6615+112 全绿、
   clippy/fmt 零告警、wasm 双目标过；审查「可合并」，签名收敛建议已收编。
44.75x 的 inherit3 由此压至 ~39x（17.5→15.7 对 0.4ms），残余为 helper 重入税
的调用层（probe 本体已命中缓存）与 trace 级特化差距。

### J4r 剩余税削减复审（2026-10-05）——「主位命中直取」实测负收益，否决
重剖（DSE 修复+扩深后，100 runs）：probe 17.6%（绝对量降、占比仍最大），根因
定性：扩深的直列链走**全靠站点提示槽命中，而 native 直列不回写提示槽**——
describe/breathe 的每级探测恒 miss，全落 absent→probe→helper。初判「absent 块
主位命中 → 直取代替 helper 裁决」可实现（CheckSlotMatch 键等+值非 nil 判定已
完成，直取与 luaH_getstr 主位语义一致）。实现后以 69f4cf1b（含扩深）为基线
7 轮配对：inherit3 med **+1.3%**（15.6→15.8，0/7 配对胜）——**负收益**。
定性：主位命中场景消 helper 的收益 < absent 块顺序流变重（取值序列 + 逐级
zero 常量）与生成码增大的布局成本；且 helper 内 index_chain_probe 本体已命中
链缓存（一次守卫走查即回），调用税差额有限。按纪律完整还原。J4r 剩余 helper
税的两案（运行期提示槽回写 / 主位直取）均否决收束，inherit3 ~39x 现状封存，
进一步压缩需 J1 级架构手段。

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

### FORN 环整数化实现轮（2026-10-05）——IR 消除实测零收益，a64 吸收论第二例证
按上轮切口完整实现（fork opt-j4r-chain-depth→opt-j6-forn-int）：LoopInfo 加
`int_capable/idx_slot`、before_inst_for_n_prep 编译期判定改为**紧邻 LOADN 回扫**
（初值槽由 LOADN 定义 ⟹ 初值恒整数 + step==1 递增封闭 ⟹ 环内槽值恒整数——
limit 只定停点不参与整性论证，**零运行期守卫零新块**，绕开 FORNPREP 无 fallback
占位语境与 DSE 盲区两类拦路）；GET/SET 直路 `NUM_TO_INT`（现成指令，a64 fcvtzs
无验证分支）替换 `TRY_NUM_TO_INDEX`。IR dump 验证：折叠热块 bb_linear_25 的
index 直用 NumToInt 产物、TRY_NUM_TO_INDEX 从热路径消失，非折叠路径保守保留。
**配对评测（7 轮 × runs5）：matmul +0.0%、fasta/nbody/life 全部噪声内（配对胜
2-3/7）→ 按纪律完整还原**。定性：与「环级 tag 外提证伪」同构的第二例证——
fjcvtzs/LoadDouble 类**非依赖链指令被 a64 超标量窗口完全吸收**，IR 指令数的削减
不转化为周期。matmul j 环的真依赖链 = ci[j] load→add→store 的 FP 链与寻址发射，
matmul 2.88x 残余差距的候选维度修正为：**FMA 折叠缺失**（LuaJIT trace 把
mul+add 折 fmadd 单指令）、GET_ARR_ADDR 寻址发射宽度、生成码布局。J6（regalloc）
与 FORN 整数化两案皆否决后，单用例专项的 geomean 回报天花板（matmul 权重 1/24，
10% 用例收益 ≈ 0.4% 总量）也提示：后续大杠杆应优先 J1 BBV（全局性删除检查）
而非逐用例微整形。

### J6 归因细化（2026-10-05，matmul IR 级证据链）——靶点从 regalloc 修正为 FORN 环整数化
用 lowering_fixture 通道 dump matmul 主函数 a64 生成 IR（codegenasm/CLI 模式只出
字节码；IR 工具链直通）。j 环热路径（bb_bytecode_3 + bb_linear_35，线性化与 CSE
均正常工作——bk[j]/ci[j] 的 index 正确共享、k 环不变量 R12/R13 正确复用）每迭代
四类冗余：
① 环变量 index 转换链：FORN 变量 j 以 double 存 VM 栈槽，每处 GET/SET 走
   `LOAD_DOUBLE + TRY_NUM_TO_INDEX（fcvtzs + double 整数性验证分支）+ SUB_INT`——
   FORN 变量恒连续整数，验证分支与重转换纯冗余，j 环 4 处 ≈ 15-20 条 a64 指令/迭代；
② k 环不变量重付：bk 表的 CHECK_TAG/CHECK_ARRAY_SIZE/CHECK_NO_METATABLE/
   LOAD_POINTER（%327 重载 R13，k 环 %55 已算过）每迭代重复；
③ INTERRUPT/SET_SAVEDPC 每字节码一条（cpp 同构，暂不动）；
④ 寄存器分配本身无 spill 证据（--dump-regspills 该 build 未实现，IR 寄存器域
  未见 spill 序列）——**J6「regalloc 缺陷」假说不成立**，matmul 2.88x 的主因
   是 ①②的守卫/转换冗余，靶点修正为 **FORN 环整数化（J1 FORN 快路的 native 版，
   上游 LuauCodegenA64ForgLoopArray 占位旗标已登记未接线）**。
实现切口（已定稿待实现轮）：FORNPREP 编译期判 step 常量==1（numeric_loop_stack
已有 loop_info.step）建立整数模式，入口加 double 整数性守卫（floor 检查，非整数
落 helper，每环一次摊销为零）；FORNLOOP 产整数化 idx inst；GET/SET 翻译时查
numeric_loop_stack 顶的 idx 槽 == rc → 走 INT_CAST 直路（消 TRY_NUM_TO_INDEX 验
证分支）。预期 matmul 8-15%（j 环 4 处 × 省 3-4 条验证指令）；life 同形态受益。
上轮「环级 tag 外提」证伪教训的边界澄清：该轮只外提 tag（吸收论证成立）；本轮
靶是 TRY_NUM_TO_INDEX 的**分支序列**（fcvtzs+cmp+branch，不在吸收论证覆盖内）。

### FORN 整数化实现轮（2026-10-05）——IR 消除实测零收益，a64 吸收论第二例证
按上轮切口完整实现（fork opt-j6-forn-int）：LoopInfo 加 `int_capable/idx_slot`、
before_inst_for_n_prep 编译期判定改为**紧邻 LOADN 回扫**（初值槽由 LOADN 定义 ⟹
初值恒整数 + step==1 递增封闭 ⟹ 环内槽值恒整数——limit 只定停点不参与整性论证，
**零运行期守卫零新块**，绕开 FORNPREP 无 fallback 占位语境与 DSE 盲区两类拦路）；
GET/SET 直路 `NUM_TO_INT`（现成指令，a64 fcvtzs 无验证分支）替换
`TRY_NUM_TO_INDEX`。IR dump 验证：折叠热块 bb_linear_25 的 index 直用 NumToInt
产物、TRY_NUM_TO_INDEX 从热路径消失，非折叠路径保守保留。**配对评测（7 轮 ×
runs5）：matmul +0.0%、fasta/nbody/life 全部噪声内（配对胜 2-3/7）→ 按纪律完整
还原**。定性：与「环级 tag 外提证伪」同构的第二例证——fjcvtzs/LoadDouble 类
**非依赖链指令被 a64 超标量窗口完全吸收**，IR 指令数的削减不转化为周期。

### JIT 组单用例专项收束（2026-10-05）——matmul/spectralnorm 两案归因终审
- **matmul FMA 折叠否决（语义红线）**：`fmadd` 单舍入 ≠ `mul+add` 双舍入，破坏
  与解释器/cpp 的**位一致性**（仓库红线：跨引擎返回值指纹精确比对会告警）；整数
  域可证精确但通用 FP 输入不可证，无法条件化。
- **寻址单指令化否决（布局红线）**：LuaJIT 单指令 `ldr [table+idx*16+OFF]` 依赖
  array 紧随 Table 头的布局；我们 array 为指针字段（cpp ABI 同构红线），必须两级。
- **spectralnorm 归因**（250 runs，3468 样本）：**100% 生成代码本体、helper 零
  出现**——与 matmul 同形态，7.2x 差距纯在 FP 本体的 trace 级特化（跨环寄存器
  驻留 + 链调度），method JIT 结构性不可达。
- **J1 预期下调的实测依据**：LBBV 论文的 -30~40% 基于 x86/解释器对照场景；本机
  a64 三轮独立实测（tag 外提、FORN 整数化、守卫链吸收）一致证明「删非依赖链
  守卫/转换指令」这一 J1 主要收益形态**被超标量吸收，实测零收益**。J1 剩余价值
  形态（类型化指令选择/索引直达）需重估后另行立项，不再列为无脑优先项。
- **JIT 组收束结论**：单用例专项（inherit3/oop/matmul/spectralnorm/nbody）在
  M2 Max a64 上的可动杠杆已穷尽——残余差距为 trace JIT 结构性（跨环驻留/特化）
  与语义红线（位一致性/cpp ABI），geomean 2.70 的进一步压缩需 J1 重估或架构级
  变更，均超连续小步迭代范畴。jit 组维持「明确不做」清单外的现状收束。

诚实上限：纯 FP 循环（spectralnorm）是 method JIT 对 trace JIT 最难档，
J1+J5 到 1.1~1.3x；综合 <1.0 需要 J1+J2+J3 全部到位且站点类基准反超拉开。

### E4 gslot_hit 判据合并归因否决（2026-10-05，life 归因无靶）
fork opt-e4-gslot 实测 life 解释器（80 runs，1967 样本）：87% 自时集中于
tier_cold 单函数，但构成**不是 GETTABLEKS/字符串键路径**——life 热环是 8 次
`(j±2)%w+1` 整数键 GETTABLE（含 MODK 模运算链）+ SETTABLEN 数组写 + rehash
4.8%（row[j] 数组扩容）。gslot_hit/KS 判据在 life 无热点，E4 判据合并无靶否决。
life interp 的真实可动面 = 整数键 GETTABLE×8/迭代 + MODK→ADD→GETTABLE 前缀链，
正解为 E5 装载期前缀超级指令融合（现有 fuse_succ_* 只覆盖臂尾/后继形态，前缀
融合无先例，工程大）；jit 模式已快 interp 2.7x（吸收论覆盖守卫链），投入产出
比低。life interp 23.2ms 现状收束，E4 记否决、E5 记工程大待专项。

### E5 修正落地（2026-10-05）：SUBK→MODK 链环补全——life -2.6~-4.2%，前缀融合判定证伪
E5 的「前缀融合无先例、工程大」定性**证伪**：复核发现 fuse 家族已覆盖
`MODK → ADDK → GETTABLE → JUMPIFNOT/FORNLOOP` 链（fuse_succ_addk 文档自证
life 四指令环压到 1 次派发），缺口仅剩 **SUBK→MODK** 一环——是**臂尾后继融合**
（家族既有纪律），非前缀融合。实现 `fuse_succ_modk`（数值快路与 h_modk 逐位
一致，miss 原样交回；顺势接 fuse_succ_addk 既有链）+ h_subk 接线。配对评测
（7 轮 + 11 轮 + 独立 runs8 复测 5 轮）：life med **-2.6~-4.2%**（23.8→22.8ms），
**21/22 配对胜**、三组方向一致——低于 5% 默认门，依 E3 先例（coroutines
-3.4~-3.9% 多组一致合并）以复测一致性为据合并，如实标注。全 interp 组回归
geomean +0.8% 无系统回退；6615+112 全绿、clippy/fmt/wasm 过；审查「可合并」，
两建议（fuse_succ_subk 文档过时、modk 尾部 fornloop 重复收尾）已收编。审查同
时确认：融合链每级 miss 停在未执行指令边界、无重复副作用、链深静态上界；
cpp SUBK/MODK/ADDK 快路均无 VM_INTERRUPT/savedpc 检查点，融合不丢检查点。
life jit 已 2.7x 于 interp 的现状下，本项为 interp 侧余量的最后一块实收益。

### E5 扩展终审（2026-10-05 晚）——两实现形态实测均失败，扩展封存
上轮否决后的翻案实验（fork opt-e5-miss）：把 MODK/SUBK 试探加进 fuse_succ_addk
的 miss 分支，**两形态均失败**——
① **rb-miss 分支形态**（初版）：试探放「rb 非数字」分支——子代理审查证伪该补丁
是**死代码**：入口已判 op==ADDK，rb-miss 可达时 op 恒为 ADDK，MODK/SUBK 试探
（各以 opcode 前缀判定）必然原样返回同一 pc，行为零差异；当时实测的 life
-5.1%（7/7 配对胜、A 侧极稳）**与本 diff 无因果**——rust-base 与 rust 两个
CARGO_TARGET_DIR 的构建布局差异，属布局彩票（教训：配对评测的两二进制必须
树内容有因果差异，否则数据失效）。
② **opcode-miss 分支形态**（修正版）：试探移入入口 opcode 判的 miss 分支
（真正的非 ADDK 后继场景），并按审查建议 1 加 pc 单调说明——**实测行为回归**：
conformance_errors 行号断言失败且**随修改漂移**（errors.luau:121→:125），而
试探命中诊断打印零出现——存在未定位的执行语义问题（疑似 fuse_succ_addk 的
opcode-miss 分支可达边不限于顺序流，试探在非顺序到达场景执行产生可见偏差）。
调试超本轮收敛范围，按纪律**完整还原**。结论：E5 的 SUBK/MODK 扩展两形态
（rb-miss 死代码、opcode-miss 语义回归）全部否决，试探税抵论证（88cefa3e）
**维持有效**；E5 更长链融合封存，除非先定位 ②的精确语义破绽。

### E5 语义谜团破案 + opcode-miss 形态终判（2026-10-05 深夜）
带着「② 的行号漂移未定位」重做实验（fork opt-e5-semdbg，正确重放 opcode-miss
试探——rb 的 is_number 检查留在 addk 命中路径、miss 分支只含 modk/subk 试探）：
**errors 121/125 全过、6615+112 全绿**——上轮的崩溃定性反转为「**patch 错位**
（替换目标块漏掉 fuse_ok/`is_number` 段导致语义损坏）产物，不存在神秘语义破绽」。
但性能终判同样反转：正确实现下 life **+5.5% 纯增税**（23.8→25.1ms，0/7 配对胜，
A 侧稳定）——E5 链（fuse_succ_modk）已把 SUBK→MODK→ADDK→GETTABLE 融合为单次
派发，opcode-miss 试探命中的场景只剩链尾出口，试探成本无收益场景对冲，纯为
额外比较税。**E5 opcode-miss 形态终判否决**（语义无辜、性能死刑）。至此 E5
扩展三形态（rb-miss 死代码、opcode-miss 错位崩溃、opcode-miss 正确实现增税）
全部实测收束，试探税抵论证三次独立成立，E5 链融合领域彻底封存。

### E5 ②语义破绽定位终审（2026-10-05 深夜）——破绽不存在，②关闭
上轮「② 未定位」的翻案复现（fork opt-e5-loc，干净重放 opcode-miss 实现——rb
的 is_number 检查留在 addk 命中路径、miss 分支只含 modk/subk 试探）：errors
121/125 **全过**、6615+112 **全绿**——**② 的「语义破绽」不存在**，上轮的崩溃
终验确认为 patch 错位（替换目标块漏 fuse_ok/`is_number` 段）单一成因。性能终审
同步完成：life med **+11.1% 纯增税**（22.6→25.1，0/7 配对胜，双侧极稳）——
opcode-miss 试探的死刑由「错位崩溃」升级为「语义正确但性能死刑」双重实锤。
**② 关闭**：E5 扩展的归档完整性补齐（三形态各有终审结论），E5 领域维持封存。

### DSE 两阶段修复复用排查（2026-10-05）——同款形态全 codegen 唯一，排查闭环
排查「计数数组 + 遍历置位/递减 + 多链起点」的顺序假设缺陷是否有第二处：
- `remaining` 计数数组全 codegen 触点仅在 DSE 家族（mark_dead_stores_in_* 四文件，
  已两阶段化）——无第二处计数时序形态；
- 其余 visited 触点（block ordering/dominance children/linear blocks/const_prop
  块链）均为**顺序无关的已处理标记**，无「先消费后置位」时序；
- const_prop 的替换缓存跨块消费为**保守正确**：映射仅在定义者被遍历时写入，
  消费块先到时映射缺失即不替换（保守回退原指令），缺失不产生错误替换；
- regalloc 的 get_live_out/live_in 按块精确计算（J4r 轮已实证行为差异源自
  remaining 置位序而非 live 集）。
结论：DSE 顺序假设缺陷为孤例，两阶段修复无需泛化；J4r 扩深块结构（多链起点 +
条件边目标消费上游定义）是唯一触发形态，已被修复覆盖。排查归档，无代码改动。

### J1 寄存器驻留立项（2026-10-05）——量化归因完成，基建级专项待启动
**量化归因**：matmul j 迭代 13.2 cycles（12.1ms/2.744M 迭代 @3GHz）vs LuaJIT
4.6——差距 ~8.6 cycles 的最大构成 = **FORNLOOP 回边处 `state.clear()`（cpp 同构
保守语义）**使环内 versioned loads（ci/bk 表指针 LOAD_POINTER ×2 + 环变量
LOAD_DOUBLE ×3）每迭代重载 ≈ 8-10 cycles，解释力 ~70%。次级构成 = TRY_NUM_
TO_INDEX 转换链（FORN 整数化轮已实测 IR 消除零收益——被吸收）。
**实现切口（已定稿）**：const-prop 回边活值传播（FORNLOOP 回边不再全清，改为
沿回边合并入口态的活值集——需环体数据流分析保证不动点收敛）+ golden IR 全量
重录（回边传播会翻新全部含环 proto 的 IR 基线）+ 正确性论证（回边合并的不动点
安全性：数值/指针版本在环内的 killed-by 写需精确枚举）。前置 = golden IR 重录
基建；工程量 = 数天级专项。
**立项依据充分**（解释力 70% + 切口定稿），待专项预算启动；启动前 J1 相关的
微整形切片（FORN 整数化/守卫链吸收/tag 外提）维持已归档的否决结论。

### J1 重写级专项立项（2026-10-05 深夜）——三遍遍历架构设计定稿 + 分片计划
前置清单逐项突破的**架构解**定稿（放弃位级移植改为行为移植的 const-prop 重写）：

**三遍遍历架构**（解决三重前置的统一形态）：
- **第 1 遍（collect）**：现 const-prop 逻辑但 **IR 零修改**（substitute/kill 全部
  收集为计划表，state 登记照常）——产出环尾出口快照 v1（versioned loads 的
  version 计数与 value_map 条目）。此遍 IR 未动 → version 轨迹即「无替换基准
  轨迹」，**前置 1 解**（第二遍的快照 version 以此基准表达，跨遍稳定）；
- **第 2 遍（propagate）**：环头 setup 时恢复 v1 快照（version 计数 + value_map
  条目按第 1 遍基准回填），重走环体（仍 IR 零修改）→ 环尾出口快照 v2。v2 ⊇ v1
  （恢复态只增已知），若 v2 ≠ v1 则第 3 遍再迭代——**前置 3 的不动点收敛**，
  实测 matmul 形态 2 遍即收敛（环体 killed-by 写 = ci[j] 数组写，不触 versioned
  键寄存器本身）；
- **第 3 遍（apply）**：以收敛态 v* 恢复环头入口，重走环体**就地执行替换/kill**
  ——替换立即改写 IR（保留级联 CSE 语义，**前置 2 解**：第 3 遍无需跨遍一致，
  就地改写与现位级语义同构）。
正确性论证核心：第 1/2 遍零修改 → 快照 version 可靠；第 3 遍的替换基于收敛态
v*，其「回边合并态」是两前驱路径值域的**交集保守近似**——合并态下成立的
替换在第 1 遍基准轨迹中逐位可复核。
**分片计划**：片 1 = 三遍 PoC（matmul 场景验证 versioned loads 跨回边命中，
需完整预算会话）；片 2 = golden IR 全量重录基建；片 3 = 全 FP 组推广 + A/B。
**预算**：数天级（片 1 约半天）。

### 片 1 实现轮（2026-10-05 深夜）——被前置 0（最深层）阻断，还原
实现两遍 PoC（fork opt-j1-pass1）：collect_only 标志（substitute/kill 16+1 调用点
单点拦截）、两遍外层（collect 遍→全块出口快照 v1→propagate 遍环头恢复重走）。
**lower_impl.rs:309 的 expected 一致性断言再次阻断**（conformance_api_calls 复现），
且第 2 遍后复位链痕迹仍炸——根因升级：**linear-block pass 基于替换后 live_out
重建 expected 链，与 lower 线性化期望错位**——即「expected 链断言体系与任何
IR 修改时机变化强耦合」，**前置 0（lower 链驱动解耦）确认为最深层根前置**，
三重前置与第四前置全部以其为前提。片 1 的 IR dump 验证被此阻断，按纪律还原。
**J1 专项工程序更新**：前置 0（lower 线性化改纯物理序+显式边界块，放弃 expected
断言链）必须先行——它本身是编译器骨架级改动（lower_impl 线性化核心重写），
其后三重前置才有落地面。J1 专项总预算上调至周级以上，维持「待专项预算」封存。

### J1 片 1 实现轮 v2（2026-10-05 深夜）——两遍架构落地，matmul 零收益（性能中性合并）
修正实现（区别于被阻断的首版）：**全部修改通道单点拦截**——substitute_or_record
1 处 + kill 16 处 + replace_ir_function 家族 18 处调用统一经 `state.collect_only`
守卫（无遗漏，26 处分布式 replace 调用的教训吸收）；外层两遍（collect 遍 IR 零
修改 → apply 遍就地生效），apply 遍沿 collect 遍记录的链走（链判定与 lower
线性化的相对关系保持）。**实测：matmul +0.9% 噪声内零收益**（11.5→11.5，
0/7 胜），全 jit 组 geomean +0.0% 无回退；6615+112 全绿、clippy/fmt/wasm 过。
**判定**：两遍架构本身（消链判定漂移）为性能中性合并——它是片 2（回边
versioned 传播）的架构基座：collect 遍的零修改保证使回边快照的 version 值
首次跨遍可靠。片 2（propagate 遍环头恢复环尾快照）为下一实现片。

### 片 2 前置归因否决（2026-10-05 深夜）——当前 IR 无新增可消除项，片 2 封存
片 2 启动前的重新归因（fork opt-j1-slice2，当前 dev IR dump）：j 环折叠块
bb_linear_25 的跨块折叠**正确**（bk[j]/ci[j] 的 index 正确共享 %137、k 环不变
量 R12/R13 正确复用），剩余指令全部为语义必要守卫（CHECK_TAG/CHECK_ARRAY_
SIZE/CHECK_NO_METATABLE/CHECK_READONLY）与真实计算（MUL/ADD/寻址/递增）——
**无错误重复执行冗余**。matmul 11.5ms vs LuaJIT 4.6ms 的差距确认为 trace 级
寄存器驻留（环内全值驻留 + 链调度），method JIT 结构性不可达（J1 立项归档
结论再次确认）。**判定**：片 2（回边 tag/versioned 传播）无新增可消除靶——
propagate_tags_from_predecessors 的 tag 跨回边传播已在现管线生效；versioned
value 传播的安全实现（回边 merge + 悬空防护）预研归档的数天级成本不变，而
IR 形态已无对应可消除项。**片 2 否决封存**。

### J1 寄存器驻留收益上限实测（2026-10-05 深夜）——终局证伪，专项关闭
**源级驻留上限实验**（决定性终审）：把 matmul 内层的 `ci[j] = ci[j] + aik *
bk[j]` 手工改写为局部驻留版（`local cij/bkj` 显式驻留——等价于寄存器驻留的
极限效果），runner 实测 ulua-jit：**收益仅 ~0.5-1%**（3123-3165 vs 3110-3133ms，
3 轮方向一致）——远低于 -60% 预期。结论：a64 jit 的内存指令已被超标量窗口
高效流水（L1 命中 4c × ~6 条/迭代被 MUL/ADD 链与寻址完全遮蔽），**栈槽往返
不是 matmul 差距的构成**——此前「8-10 cycles 栈槽往返」的量化归因高估（双计
入了与寻址/守卫共享的 load unit 带宽）。**J1 寄存器驻留专项终局关闭**：
收益上限 <1%，propagate 遍/回边传播/栈槽提升的任何实现均不值得投入。
matmul 2.88x 的残余差距重新定性：**FP 链延迟 + 寻址发射的逐迭代串行**（每迭代
ci[j] 读改写 + bk[j] 读的地址依赖），trace JIT 凭跨环驻留与链调度取胜——
结构性差距，归档维持。探索路径全部关闭：FORN 整数化/守卫链吸收/tag 外提/
主位直取/E5 扩展/寄存器驻留（源级上限实测）六案互证。

### 前置 0 工程量量化（2026-10-05 深夜）——改动面清单与失败根因假设
量化前置 0（lower 线性化解耦）的改动面（侦察 get_sorted_block_order/lower_impl/
const_prop_in_block_chain 三文件）：
- **物理序即链序的函数**：`get_sorted_block_order` 按 `(kind_priority, sortkey,
  chainkey)` 排序，sortkey/chainkey 由 const-prop 链遍历写入——lower 的线性化
  与 expected 链**同源**（都源自链遍历的赋值）。前置 0 的「解耦」本质 = 让
  两遍重走后的 **(sortkey, chainkey, expected) 三元组自洽**（物理序与 expected
  链不再互相矛盾）。
- **改动面清单**（约 3 文件）：① get_sorted_block_order 排序键调整（fallback
  priority 兜底已存在）；② const_prop_in_block_chain 两遍的字段维护（第二遍
  重走时照常写三字段，链外块复位）；③ lower 断言改 `if expected != MAX` 跳过
  （已存在该跳过形态）。
- **上轮失败根因假设**：fallback 块不被链遍历覆盖（target_candidate 排除
  Fallback kind）→ pass 切换复位后 **fallback 块保持复位值、非 fallback 块被
  第二遍重赋** → 物理序混入两代 (sortkey, chainkey) → 排序与 expected 链错位
  → lower 断言炸。修复实验需逐 proto 对比两遍的 (sortkey, chainkey, expected)
  三元组（deep_arithmetic_chain 场景），且 fallback 块的复位值需与 kind_priority
  排序兜底协调。
**判定**：改动面收敛（3 文件）但根因验证需逐 proto 调试（半天~一天），随后
J1 三重前置才能落地（数天）。维持「周级以上专项」封存；启动时以本清单为
工程蓝图。

### JIT 轮双靶归因否决（2026-10-06）——call_fallback 切口预算否决 + 吸收论证第三例（首次覆盖真依赖链），J1 剩余价值形态终局关闭
JIT 模式轮按候选清单执行 samply 归因（fork opt-jit-ccall，runs=12 基线与路线图
逐位吻合；runs=6 短窗曾现 2-3x 假漂移，重负载用例 runs≥12 定为纪律）：

- **coroutines jit call_fallback（18.4% 自时）切口否决（未实现即弃）**：
  call_fallback_export 叶子地址级拆解（2365 样本）——C 调用返回点单条指令
  `tbz`（blr x22 后）37%、is_function 分派点 9.9%、checkstack 区 11.6%、
  返回尾 14.5%——**自时主体是分支/返回重定向泡而非可删指令**；建帧六写/
  checkstack/copy 循环 ~30% 为 cpp callFallback 同构语义钉死。native 侧内联
  C 调用框架（LuaJIT 形态）需在生成码里复刻 ci 写与栈扩检查，预估削 ≤6%
  （18.4% 的 1/3）低于门且工程风险高，**归因否决**。同轮对照：cpp native
  coroutines 109.6ms vs ulua-jit 83.7ms（同窗同进程）——我们已快 cpp，
  差距只在 LuaJIT 的边界机器（~60%，E3 已封存）。
- **micro_arraywrite 5.0x 归因 + FORN 整数化重估轮（实测零收益，还原）**：
  IR dump 定谳 j 环每迭代 14 条 IR 串行链（LOAD_DOUBLE→TRY_NUM_TO_INDEX→
  SUB_INT→GET_ARR_ADDR→STORE），转换链在 store 地址**真依赖**上——吸收论证
  边界（「非依赖链才被吸收」）不适用，属 J1「剩余价值形态重估」的范围。
  重放归档 FORN 整数化设计（LoopInfo.int_capable + LOADN 回扫判定 + GET/SET
  命中环变量槽走 NumToInt 直路），IR 验证热块验证分支边消失。**配对评测
  （7 轮 × runs400 CPU 时间）：A med 0.23-0.24s / B med 0.24s，零收益方向
  偏负**——预测恒命中的验证分支即使真依赖链上也被超标量窗口完全吸收；
  剩余 LOAD_DOUBLE+fcvtzs 本体的消除需跨回边 int 寄存器驻留 = J1 寄存器
  驻留终局已封存领域。**按纪律完整还原**。
- **终局定性**：吸收论证第三例（tag 外提/FORN 整数化/本轮验证分支），且
  首次覆盖真依赖链场景——method JIT 对短 store 环的 5x 差距 = trace 级
  int 寄存器驻留（LuaJIT 环变量驻硬件寄存器），method JIT 线性 IR 无
  phi/无回边活值传递，结构性不可达。**J1「剩余价值形态（类型化指令选择/
  索引直达）重估」就此终局关闭**；JIT 组重灾榜全部条目（inherit3/oop/
  spectralnorm/micro_call/micro_arraywrite/nbody/fib/matmul/coroutines）
  均有归档结论，单用例与微整形杠杆全部穷尽，剩余压缩需架构级变更立项
  （trace 层或 pattern→automata 编译类专项）。

## 全面超越 LuaJIT 总规划（2026-10-06，论文调研后定稿）

目标：解释器与 JIT 两模式对 LuaJIT 2.1 双第一（24 用例 geomean 均 <1.0）。
现状：解释器 1.024（结构性余量 ~3-5%）、JIT 2.69（结构性差距 2.7 倍）。

论文与工程参照（2022-2026 全部读毕）：
- **LuaJIT Remake（LJR，OOPSLA 2024，Deegen 元编译）**：解释器比 LuaJIT asm
  解释器快 28%（34 用例赢 31 个）。三个可移植技法：①尾调用+GHC 调用约定派发
  （每 opcode 独立 handler，无 callee-saved 寄存器浪费）；②**hidden-class
  内联缓存**（表查找命中=2 分支，替代哈希遍历——数据结构层变革）；③quickening
  （effect-lambda 序号熔进 opcode，消间接分支）。LJR 证明：asm 解释器可被
  C++/Rust 级工程超越，差距不在汇编手工而在执行模型。
- **CPython 3.13/3.14 copy-and-patch JIT + tail-call 解释器**：stencil 编译
  路线已 production 化；但 3.14 实测 JIT 常跑不赢自家解释器（Python 字节码
  分派占比低+CPython 对象模型重）——教训：stencil 层的收益取决于分派占比，
  Lua 字节码粒度小、分派占比高，模型优于 CPython。
- **ZJIT（Ruby 4.0，Rust）**：SSA 方法 JIT + phi + SSA 寄存器分配——
  证明「带 phi 的方法 JIT 可跨回边驻留」，是我们 JIT 侧无 phi 困局的业界解。
- **V8 Maglev**：中层 SSA JIT + loop peeling，production 化的 block-args phi。
- **Rust 基建**：patchouly/copypatch 等 crate 证明 copy-and-patch 在 Rust
  可行且 build-time stencil 提取无需 nightly。

### 阶段一：解释器第一——copy-and-patch 基线层（opt-copatch，预期 1.024 → 0.8-0.9）

**核心洞察：我们不需要 LLVM stencil——已有手写 a64/x64 汇编器**。每 opcode
一个 build 时汇编函数，产出带「补洞记录」的 stencil；load 期逐字节码
copy-and-patch 成原生码（~5μs/proto，比全 JIT 快三个数量级）。

设计要点：
1. **ABI 全复用**：寄存器约定（R_STATE/R_BASE/R_CODE/R_CONSTANTS）、
   NativeContext helper、TValue 布局与现 JIT 完全一致——runtime 零新增。
2. **万能逃生门**：未覆盖 opcode 的 stencil 是「跳回 tier_cold 解释器」的
   trampoline（带当前 pcpos），部分覆盖从第一天起就正确。
3. **位一致白送**：stencil 语义 = 解释器逐位同构（无 FMA、无双舍入差），
   精度解锁不需要动用。
4. **覆盖顺序由 vm-opcount 直方图驱动**：top-30 opcode 覆盖 >90% 动态频次。
5. **Phase 2 附产品**：copatch 站点是 IC/quickening 的宿主（LJR 技法②③），
   hidden-class 轻量版可先复用现有 shape/epoch 基建。
6. 片划分：片0 stencil 基建+3 opcode+逃生门+位一致冒烟；片1 top-30 覆盖；
   片2 call/return/FORN+全组评测；片3 引擎集成（引擎模式开关）+双第一校验。

**片0 进度（fork opt-copatch，本会话）**：stencil 内核已绿——`copatch.rs` 的
[`Stencil`/`PatchSite`/`patch_slot`] 抽象（imm12 字段规格 shift=10/缩放 8/4，
VM 栈槽寻址 `reg*16+off` 折算）+ 首个算术模板 `stencil_add_vv`（读 b/c 载荷 →
fadd → 写 a 值 + tag 双写 tnumber，语义与解释器逐位同构）。对账测试以
「同一汇编器直发具体寄存器形态」为 ground truth 逐字校验通过（R5/R7/R2 形态）。
下一片：imm12 值域断言（reg≤255 天然满足）+ branch 类补洞（JUMP/FORN）+
proto 编译走查 + CodeAllocator 执行通道。

**片1 进度（本会话续）**：①入口 ABI 侦察闭环——LOP_NATIVECALL→ecb.enter
（on_enter 读 `exectarget + execdata[pc_offset]`）→gate_entry（prologue 建
X19=l/X20=ctx/X21=global/X22=k/X23=closure/X24=code/X25=base 不变量+256B 帧，
`br X2` 进目标码）→出口 `X0=continue_in_vm + br gate_exit`；fallback 惯用法
（X0=state/X1=&code[pcpos]/X2=base/X3=k→helper→reload base）；绑定 =
`bind_native_protos` 三字段（execdata/exectarget/codeentry=&LOP_NATIVECALL 字）。
②翻译器内核落地（copatch_translate.rs，零警告）：LOADN/LOADK/ADD/SUB/MUL
（K+VV 双形态）/MODK（fdiv→frintm→fmul→fsub，Lua floor 取模）/FORNPREP/
FORNLOOP/RETURN 直译，bail-to-interpreter 逃生门（base/savedpc 同步 +
continue_in_vm=1 + gate_exit），回边 interrupt 检查（命中即 bail，解释器处理
GC/钩子——首轮粒度换正确性）。待续：冒烟测试（Default-Proto 构造微字节码）、
get_jump_target 编码对齐、execdata 头组装 + bind 接线、旗标门控。

**片2 落地（2026-10-06，合并 969617f8..0fdec651+接线 96034aa4）——旗标门控
基线层 rt 位一致绿 + 首个实测收益**：
- **接线**：`create_native_function_a_64` 头部插直译分支（旗标
  `LuauJitCopatch`，排除表门控保 --fflag 双态）——`translate_proto` 成功即
  词流整块 append 进共享 builder（块内 a64 分支相对寻址，连续拷贝保距）+
  `bc_mapping` 按块基址平移 + `entry/end_location` 手工置位，
  `create_native_proto_exec_data`/绑定机具与 JIT 全复用；资格不符整 proto
  落回 JIT（混合模块成立）。
- **调试战果（rt 级运行时测试连环抓四 bug）**：①FORNPREP 分支方向反
  （`ble→exit` 写成进环+fall-through 落 body = 无限环）；②
  `b_cond(Always)` 的 cond 槽 0x3F 低 4 位 = 0xF(NV 恒不跳)，无条件跳必须
  `b_label`；③**前向分支缺 `finalize()`**——pending label 未回填，imm26=0
  = `b` 跳自身（lldb 附着 `b 0xself` 一击定位）；④LOADN 立即数是 16 位 Bx
  （b|c<<8），只读 B 把 limit 2000 截成 208（值一致性测试 164264 vs
  14126776 定位）。另：浮点常量通用物化（4×movz/movk+fmov_rr，绕开
  is_fmov_supported_fp_64 窄门）、PREPVARARGS 0 noop、越界 target 防御
  拒译、Translation 按 get_code_size 截断尾随零字。
- **三方实测（micro_arith，5/5 轮 ABBA）**：解释器 33.1ms / copatch
  29.5ms / JIT 13.0ms——**copatch vs 解释器 med -10.9%（5/5 胜，超 5% 门）**，
  JIT 路径旗标关零回退；rt 级三方逐位一致（解释器=JIT=copatch），
  test.sh 6618+112 全绿，clippy 门禁 0。
- **当前定位**：旗标门控的基线层与 JIT 互斥（合格 proto 直译优先），
  默认关。下一片：独立解释器加速层集成（enable_jit 关也可用：独立 ecb
  注册 + 编译入口分流），对比口径切换为「copatch-on 解释器 vs LuaJIT
  解释器」——阶段一「解释器第一」的主战场；其后 opcode 覆盖扩展
  （vm-opcount 直方图驱动）与 IC/quickening（LJR 技法）。

预期：分派占比 ~20-30%（fib 实测派发头 22%）→ 解释器列 1.15-1.3x 提升，
对 LuaJIT interp 反超 11-21%，**解释器第一达成**。

**片2+片3 落地（2026-10-06，fork opt-copatch）——加速解释器形态接通**：
①片2 VM 接线：`create_native_function_a_64` 插直译分支（词流整块 append +
bc_mapping 基址平移，execdata/绑定机具与 JIT 全共用），`LuauJitCopatch`
旗标三处注册；rt 级三方逐位一致（解释器=JIT=copatch）绿。②片3 编译策略
分流：`CompilationOptions.copatch_only`（仅直译合格 proto 产生原生码，资格
不符整 proto 保持解释——不绑 NATIVECALL、不记失败）+ `try_create_copatch_
function_a_64` 抽出（JIT 形态落 JIT / copatch-only 形态跳过的共用入口）+
`luau_codegen_create_copatch_only`（**safeenv 保持 0**：直译码不消费
CHECK_SAFE_ENV，纯解释既有行为零扰动，对比口径纯净）+ rt load 门扩展
（jit 关 + 旗标开 → chunk 装载期懒装配上下文并走 copatch-only 编译，
`LuaInner.copatch_ctx` 幂等护栏）。顺手修一例潜在 UB：空常量表 proto 的
`k` 为空指针，`from_raw_parts(null, 0)` 违反 non-null 前置（debug UB 检查
当场 abort），sizek=0 改空切片代位。runner 实测（ulua 引擎 jit 关，
micro_arith 最小耗时）：旗标关 30.9ms → 旗标开 26.4ms（-14.6%）；fib/
micro_call/micro_gettable 旗标开关差 <1ms 零回归；三方排序 JIT 11.7 <
copatch 26.4 < 解释器 30.9——「解释器第一」对比口径（ulua 引擎 jit 关 +
`--fflag LuauJitCopatch=true` vs LuaJIT interp）就此打通。测试：
copatch_only_strategy（code-gen 层：合格绑定/不合格保持解释/跳过不记失败/
safeenv=0 下原生环+bail 语义）+ rt copatch 组两组逐位一致。

**片4 落地（2026-10-06，fork opt-copatch）——opcode 覆盖扩展（动态直方图驱动）**：

直方图读数（`ulua-vm` 的 `--features vm-opcount` luau-run，动态频次 top）：
micro_gettable/microbig_gettable＝FORNLOOP 28.8%/GETTABLE 28.7%/JUMPIFNOT 28.1%/ADDK 11.2%；
life＝GETTABLE 23.6%/JUMPIFNOT 23.4%/ADDK 16.0%/MODK 15.7%/SUBK 7.8%/SETTABLE 2.7%；
nsieve＝FORNLOOP 31.7%/LOADB 21.1%/SETTABLE 21.1%/GETTABLE 10.6%/JUMPIF 10.6%；
micro_call＝LOADN 15%/RETURN·GETUPVAL·CALL·JUMPIFNOTLE 各 13.75%/MUL·SUBK 12.5%；
patterns＝LOADK·ADD 13.6%/MOVE·GETIMPORT·CALL·LENGTH·FORGLOOP 各 11.4%；
oop＝GETTABLEKS 18.8%/CALL 12.9%/MOVE 9.4%/SETTABLEKS 7.1%/GETUPVAL 5.9%/GETIMPORT 4.7%/NAMECALL 3.5%。
族序按合计动态权重定：跳转 → 表访问 → 数据移动 → GETIMPORT。

逐族落地（每族一提交，rt 三方逐位一致 + strategy 资格门 + clippy 全绿）：
①跳转族：JUMP/JUMPIF/JUMPIFNOT（truthiness 三态）/JUMPIF(NOT)LE/LT（双 number
tag 守卫 + fcmp 有序双分支，NaN 语义逐位同解释器：LE/LT 先 b.vs 防无序误跳，
NOT 变体先 b.vs 落跳转）/JUMPXEQKN（有序判等 + aux 取反位）。
②表访问族：GET/SETTABLE（number 键数组快路：fcvtzs 饱和转换=Rust `as` 语义 +
(index-1)<sizearray 无符号回绕界检查 + 元表缺席 + scvtf 回验精确整键 + 写侧
readonly）、GET/SETTABLEN（0 基数组）、GET/SETTABLEKS（指令字 C 字节 IC 槽原生
直读——解释器 vm_patch_c 的 IC 热身对原生执行持续生效——+ gslot_hit 三守卫），
SET 命中路径带内联写屏障谓词（iscollectable && isblack && iswhite，JIT
BarrierTableForward 同式）+ 条件调用 lua_c_barriertable；守卫失败全量 bail。
资格前置顺带落地：NOP/BREAK（语句边界占位，VM 语义即 NOP）/MOVE/LOADNIL。
③数据移动族：LOADB（含 C 跳槽）、LOADK 泛化 16B 全型拷贝、GETUPVAL（open/
closed 双路）、SETUPVAL（屏障谓词命中即 bail）。NEWTABLE bail 桩尝试后回退：
48×48 规模表构造脚本（micro_gettable）触发未定位执行态腐化（三方一致门抓到
MULK 读 nil 循环变量），按宁慢勿错改回资格拒绝——含表构造 proto 片4 不直译。
④GETIMPORT：k[D] 已缓存非 nil + cl.env.safeenv 开启双守卫，命中 16B 拷贝；
未缓存链/safeenv 关 bail 交解释器。

**两处片1 正确性缺陷修复（基准三方一致门抓到）**：①`insn_len(PREPVARARGS)`
误记 2 字（实际 1 字），pc 从此错位一字吞掉后继指令；②LOADB 的 C 跳槽只作用
顺序流，被跳过指令仍是条件分支的跳转目标（`x = cmp` 形态 JUMPIF* 真值路径正
落在被跳槽的 LOADB 1 上），不得吞并翻译——修正为顺序路无条件 b 跳过跳槽区间、
被跳槽指令照常翻译。

**热信号资格门（A/B 抓到的真回退）**：直线小函数（OOP 访问器 GETTABLEKS+ADDK+
RETURN）全格直译时，门进出往返（六 callee-saved 存取 + base/savedpc 同步 +
解释器重入）比解释两三条指令更贵——inherit3 200k 次方法调用实测 +21%。修正：
仅含回边（FORNPREP/FORNLOOP/JUMPBACK 或 d<0 跳转）的 proto 直译，直线函数拒
译保解释零扰动。修正后 micro_arith ABBA med **-14.6%**（26.4 vs 30.9ms，6 轮
交替），inherit3 回零资格（其残余 ±5-20% 读数为双峰机况 + 上下文装配常量偏置，
零资格对照组 micro_call +1.0%）。

A/B 全组（ulua interp，flag on/off ABBA 6 轮取 med）：micro_arith -13.3%，
其余 15 用例 |Δ|≤1%（coroutines -2.2%/fannkuch -3.4% 为噪声），无回退。

遗留（片5 候选，附直方图证据）：CALL 12.9-13.75%（micro_call/oop/patterns 三
用例主热）+ NAMECALL 3.5% 延后——需完整帧构建/参数布局机具，独立成片；
NEWTABLE 桩腐化根因；FORGLOOP/LENGTH（patterns 22.8%）；FASTCALL2（oop 4.7%）。

**片5 落地（2026-10-06，fork opt-copatch）——CALL/NAMECALL 真译出 + NEWTABLE 根因闭环**：

①CALL/CALLFB 直译（12.9-13.75% 动态占比）：原生被调快路（七重守卫：函数 tag/
非 C 闭包/exectarget/实参数恰等/非 vararg/CallInfo 槽未满/栈余量严格充足 →
建帧六写 + savedpc/NATIVE 旗标 + JIT 环境寄存器装载 + `br exectarget`，JIT a64
CALL 同式；被调原生 RETURN 经 RETURN 臂 `br` 折回本帧 savedpc）+ `call_fallback`
通用慢路三分派（C 闭包直调收尾 / 解释被调建帧后 `emit_exit(true)` 交还解释器、
被调 RETURN 臂重入本帧 / 让出哨兵 `tbnz X0,0` → `emit_exit(false)` 使 luau_execute
直接 return，让出沿 resume 边界向上传播）。**设计修正**：任务骨架建议的
`luaD_call` 慢路被否——其 n_ccalls 抬升窗口会把「纯 Lua 被调内的
coroutine.yield」误判为 C 边界让出（解释器对 Lua→Lua 调用不增 n_ccalls），
call_fallback 两被调路径都不碰该计数，让出/pcall/multret/nil 填充语义与解释器
逐位一致。NAMECALL/NAMECALLUDATA 主位快路：表 tag 守卫 → 主位节点探测（hash
编译期取自 k[aux]，`hash & ((1<<lsizenode)-1)` 掩码，GetHashNodeAddr 同式）→
gslot_hit 三连 → self/方法 16B 落位后顺序落入后随 CALL；miss/碰撞链/非表被调
全量 bail（解释器重做整条，__index 链与 IC 回填在慢路）。

②NEWTABLE「桩腐化」根因闭环（片4 遗留）——**三处潜伏缺陷 + 一处越界，桩无罪**：
⑴`insn_len` 把 NEWTABLE 误记单字——实为双字（aux=哈希槽数，解释器
`aux=*pc; pc++`），NEWTABLE 之后所有 pc 错位一字，标签与 bc_mapping 整体漂移，
即「MULK 读 nil 循环变量」的直接来源；⑵PREPVARARGS 误当 noop——解释器臂执行
`base=l->top` 重排 + `l->top=base+stacksize`（寄存器窗口在此扩张），直译跳过它
使 bail 重入后的解释器以未扩张窗口取寄存器（VM_REG 断言炸）；改为真语义快路
（base==l->top 无实参守卫 + 栈余量守卫 → ci->top/l->top 落 base+stacksize，
其余 bail），资格检查同时修正读数字段（numparams 在 A 不在 B）；⑶copatch 块
直拷循环绕过 commit/extend——共享 builder 容量边界裸 put 越界 panic（life 规模
踩中，NEWTABLE 拒译时该路径到不了，潜伏至今）。桩复陆：建表后热环保持原生，
mat() 形态根因回归钉 + micro_gettable/nsieve/life 原负载三方逐位一致。

③NEWCLOSURE/CLOSEUPVALS 直译（A/B 驱动的资格补全）：`local function` 形态主
chunk（micro_call 等基准的热环宿主）被 NEWCLOSURE（+nups 条 CAPTURE 序列）与
块出口 CLOSEUPVALS 阻塞。NEWCLOSURE = `copatch_newclosure` helper 直调（分配/
三型捕获 VAL·REF·UPVAL/GC 步进，解释器臂逐句同构；capture 词无独立原生码，
pc 直接跨序）；CLOSEUPVALS = open upvalue 链守卫 + 条件调 luaF_close。

rt 验证门：调用族（快路/交接/C 被调/MULTRET 双向/nil 填充/pcall/协程跨 CALL
yield-resume）+ NAMECALL 族 + NEWTABLE 根因钉 + NEWCLOSURE 三型捕获 +
CLOSEUPVALS 跨轮收口，13 用例三方（解释器=JIT=copatch）逐位一致 + 加速解释器
形态（jit 关旗标开）一致；clippy 三 crate 全 feature 零错。

A/B（ulua 引擎 jit 关，flag on/off 交错 8 轮取 med）：**spectralnorm -20%**
（56.3→45.2ms，NEWCLOSURE 解锁其函数定义型主 chunk）、queens -2.3%/tablesort
-0.7%/patterns -0.5%/matmul -0.7%；micro_call 主 chunk 资格解锁但每迭代 CALL 的
call_fallback 交接 + RETURN 臂重入往返 ≈ 解释器 3 条派发代价，净 +2%（吸收论证：
直线被调受热门拒译、原生快路需被调已原生——解锁依赖 FASTCALL 族或 a64
call-inlining，均后续片）；patterns/oop 主 chunk 仍被 FASTCALL 族阻塞
（string.* 内建/setmetatable→FASTCALL2/2K/3），维持片4 遗留口径；全组无 >2%
实回退（micro_arraywrite 0.7ms 量级噪声例外）。

全组对比（同会话交错 4×4 轮取 med）：加速解释器 vs LuaJIT interp geomean
**0.978**（片4 记录 1.018 属机况漂移——同会话旗标关纯解释口径 0.977，两口径
持平；分组看点：life 2.17/micro_gettable 2.55/microbig_gettable 2.49/tablesort
1.98/spectralnorm 1.52/micro_arith 1.50 反超，nsieve 0.34/fib 0.48/coroutines
0.52/micro_call 0.61 落后）。加速 vs 纯解释（同会话）：geomean 持平，构成重排
（spectralnorm -20% vs micro_call +2%）。

遗留（片6 候选）：FASTCALL1/2/2K/3 直译（patterns/oop 主 chunk 资格 + oop 4.7%
动态占比）；FORGLOOP/LENGTH（patterns 22.8%）；call-dense 环的常驻原生（a64
call-inlining 或热直线被调的门控放宽——需双面条件：调用方原生 + 被调直线，
片4 单面热信号门不足以判益）；IC/quickening（LJR 技法）。

### 阶段二：JIT 第一——FORN trace 层（方案三蓝图深化，预期 2.69 → 1.3-1.6）

已归档蓝图（见 J1/FORN 各条）启动次序修正：
1. 片1 热环检测+录制器（FORNPREP..FORNLOOP 闭体线性 IR）；
2. 片2 回边 phi（ZJIT/Maglev 的 block-args 形态）+ 类型特化（feedback 复用）；
3. 片3 寄存器驻留 codegen（loop-carried 分配）+ trace 入口守卫 + 出口快照；
4. **FMA 折叠在 trace 内自由合法**（重启契约只约束解释器回退点，trace 体内
   无检查点）——method JIT 的 FMA 死局在 trace 层自动解锁；
5. 片4 allocation sinking（oop/inherit3 的逐迭代表构造）；
6. 验收：matmul/spectralnorm/nbody/fasta/micro_* 逼近或反超 LuaJIT。

### 明确的新不做（本轮决策）
- Deegen 式 LLVM 元编译整体引入（工程量 LJR 级，我们只要其技法不要其机器）；
- hidden-class 全量表结构替换（E1 语义红线：# border 与 cpp oracle 钉死；
  IC 层用 epoch-guarded 缓存替代，兼容现布局）。

### J5 展开反向探针否决 + fannkuch 封存确认（2026-10-06）
FMA 轮收口后的两个收尾探针：
- **J5 循环展开反向探针（未动手即否决）**：`LuauBackedgeHeapCheck=true`（回边插
  CheckGc）开关差值实测 matmul 11.4→11.6ms（+1.8%）——回边成本 ≈0.2c/迭代，
  展开的回边摊销奖品 ≈1%，远低于 5% 门；块克隆基建（克隆块域+操作数重映射+
  fallback 共享）成本不成比例。调度窗口论证亦不成立（a64 OoO 窗口本就跨迭代）。
  归因成本 < 预期收益，未动手即否决（SKILL 先例）。
- **fannkuch interp 1.40x 封存确认**（路线图首次覆盖该用例）：samply 120 runs
  （7369 样本）99.2% 自时集中于 `luau_execute::tier_cold` 派发单函数——fib 同构
  的实现受限形态（派发+帧建立结构差距），热边为整数键表访问对，属已彻底封存的
  E5/派发域。无新靶，归档确认。

至此连续小步迭代可及的全部杠杆均已实测封口；剩余结构性差距（trace 级寄存器
驻留、allocation sinking）的唯一已知路线 = FORN trace 层（周级专项，方案三），
或译码期 FMA 融合（上轮蓝图，checkpoint 语义手术）。两者均待专项预算会话。

### FMA 折叠轮（2026-10-06，精度解锁后）——被 fallback 重启契约阻塞，简单折叠不可行，译码期融合蓝图归档
用户解锁 FP ULP 级精度差异（fmadd 单舍入 ≠ mul+add 双舍入的红线解除）后立项
（fork opt-fma-fold，已按纪律完整还原）。侦察发现 `MuladdNum`（`op0*op1+op2`）
IR cmd 双架构 lowering 已存在（a64 fmla 特性分支 + x64 vfmadd FMA3 分支，仅
math.lerp 使用）——实现面收敛为折叠 pass。两轮实现与探针：

1. **折叠 pass**（AddNum 单用 MulNum → MuladdNum，`replace_ir_function_ir_block_
   u32_ir_inst` 全自动记账）+ 旗标 `LuauCodegenFmaFold`（is_default_enabled_flag
   排除表门控，--fflag 双态可控——非排除旗标会被 set_luau_bool_flags 的 Once
   全量点亮覆盖显式 false，排除表是 --fflag A/B 的前提）。
2. **VmReg 操作数扩展**：matmul 形态的算术操作数经 const-prop T_VALUE 重定向
   呈 VmReg 形态（dump 渲染与 lower_fa_bin 探针双实测确认），a64 MuladdNum 补
   temp_double_or_vmreg（temp_addr 栈槽装载，lower_load_scalar D 通道同式）、
   x64 FMA3 臂 b_reg 扩 mem_reg_double_op 双态。

**终局阻塞（反编译级定位）**：真实 matmul 内层 `ci[j] = ci[j] + aik*bk[j]` 的
IR 形态——`%101 = MUL_NUM` → `STORE_DOUBLE R6`（字节码临时寄存器物化）→
`CHECK_TAG R5`（ADD 操作数守卫=检查点）→ `%111 = ADD_NUM %109, %101`。MUL
结果 %101 有两个使用方（STORE 物化 + ADD），且 CHECK_TAG 检查点横在 MUL 与
ADD 之间——**检查点处 fallback 重启到 ADD 字节码并读取 R6**，融合后 R6 无人物
化，重启路径读脏值。乘法结果在每个检查点前必须物化进 VM 状态 = 主累加形态
（matmul/spectralnorm/nbody 的 `x = x + a*b` 全是此形状）不可融合；
use_count==1 守卫是对的（exit-sync 与物化 store 都是语义必需的第二使用方）。

**唯一健全解（蓝图归档，待专项）**：译码期融合——translate_inst_binary 前瞻
识别「MUL 后紧随 ADD 且 ADD 操作数即 MUL 目标寄存器」对，跳过 MUL 独立翻译，
改译 [MUL+ADD 双方操作数守卫（守卫 pcpos 回卷到 MUL 字节码，fallback 重启时
解释器从 MUL 重算并自然物化 R6，MUL 纯函数重执行幂等）] + [MULADD] + [ADD 目标
store]。附带效应需专项评估：MUL 字节码的 INTERRUPT 检查点随融合消失（中断粒
度语义，cpp-parity 钉死面）、golden IR 重录、错误行号归因（同语句内行号不变）。
预期收益再评估：融合仅消 mul 的独立发射与一次栈往返，matmul 13.2c/迭代的
主瓶颈（FP 链+寻址+回边）不动，预估 -5~10% 单用例 ≈ geomean +0.2~0.4%——
性价比低于 J5 循环展开（方案一，无语义手术），优先级下调至 J5 之后。

### J4-real NAMECALL IC 侦察立项轮（2026-10-06）——oop 归因修正（堆侧主导），inherit3 单靶蓝图定稿，立项待专项
旗标审计先行：LuauJitSettableHashInline 已有历史（nsieve 全负载证伪零收益 + x64
SIGSEGV 未根除，73845990 默认关、代码保留），四枚本仓 JIT 旗标无未测杠杆。

**oop jit 归因修正（samply 300 runs，9391 样本，推翻路线图旧定性）**：NAMECALL
helper 份额已消失（<2%，lua_t_gettm 残余 1.8% 系 GETTABLEKS 侧）——被
setmetatable-fastcall（LuauJitSetmetatableFastcall）与 J4b 等先前合并吃掉。当前
oop jit 15.6ms 的成本结构：JIT 本体 34.5% + **表构造机具 ~21%**（newkey 5.3 +
resize 4.8 + mainposition 3.4 + rehash 2.9 + adjustasize 2.5 + lua_h_clone 1.9）
+ **分配 ~7.4%**（lua_m_new 4.6 + newgco 2.8）+ intern 6.5% + GC ~4.8% +
setmetatable 3.5%——逐迭代 `{x=x,y=y}` 构造 + setmetatable + 访问全部走通用
堆路径。LuaJIT 1.1ms 靠 trace 级 allocation sinking（newtable+插入在 trace 内
展开为预分配+直 store，构造机具整体消失）——**method JIT 结构性不可达**，
与 matmul/spectralnorm 同级归档。

**inherit3 jit 重剖（500 runs，16934 样本）**：JIT 本体 40.4% + **helper 集群
~32%**（index_chain_probe 16.9 + lua_t_gettm 5.5 + lua_v_gettable 5.1 +
execute_namecall 4.5）+ lua_v_concat 9.8%（describe 内 `self.name .. ":a"`，
200K 次/轮，双引擎同付）+ intern 5.2%。helper 集群为 J4-real IC 的精确靶。

**深跳落 helper 的真因（推翻提示槽定性）**：get_slot_node_addr 的 C 提示 = 解释器
patch_c 回写的**表内节点索引**（table->node + (C & nodemask8)）——各站点独立
训练、相对各自 owner 表；深跳直列 absent 证明（CheckNodeNoNext + CheckSlotMatch
主位键比）在 2-4 键小类表上因主位被其它键占据（碰撞链 CheckNodeNoNext 失败/
主位键不等）**恒失败**落 helper，与提示槽无关。play/speak（1-2 跳）直列命中，
describe/breathe（3-4 跳）恒落 helper→链缓存一次守卫走查即回。

**平铺 site-IC 蓝图（立项级，六工程件）**：NAMECALL 站点首守卫直列化——
1. proto 侧槽阵列 `Vec<NamecallSiteSlot>{t0, mt0, node, epoch}`（pcpos 索引，
   生成码经 R_CLOSURE→proto 两跳装载；proto 释放路径同步释放）；
2. 训练点：index_chain_probe 命中记录 TLS last-resolve，execute_namecall 回读
   （t0/键身份校验）写站点槽；
3. 直列守卫 `receiver==t0 && mt==mt0 && epoch==epoch` → LoadTvalue node →
   StoreTvalue ra（~15-20 instr，替换 helper 重入 + 守卫走查 ~100+ cycles）；
4. **健全性三钩**：epoch 从 thread_local 升共享静态（AtomicU64 Relaxed，跨线程
   bump 只致 miss 不致假命中）；freeobj 对受监视表 bump（补地址复用洞——链缓存
   走查天然免疫而 IC 免疫不了）；setmetatable 写咽喉 bump（mt 换回原值的
   mt-cmp 漏洞——t0/mt0 身份守卫覆盖不了类表元链中途换装）；
5. 新 IrCmd + a64/x64 双 lowering + golden IR 重录；
6. 旗标隔离 A/B（oop 预期中性、inherit3 预期 -20~27%）。
**立项判定**：oop 退出后单靶 inherit3，geomean 回报 ~1%（24 用例权重 1/24 ×
25%），六工程件 + 三健全性钩 ≈ 数天级——**依 matmul 先例（单用例专项的
geomean 回报天花板）立项封存待专项预算**，蓝图以此为准。

**论文对照**：helper 集群 ≈ 经典 IC/megamorphic 派发税（Hölzle-Chambers-Ungar
ECOOP'91 PIC；我们的链缓存=单态 IC 的 chain 形态，站点槽 IC=其 inline 化）；
oop 堆侧主导 = trace JIT allocation sinking 文献（Chang et al. PLDI'09
trace 类型特化 + LuaJIT buffer/sinking 实践）——两者皆 method JIT 结构受限，
与 J1/J6 终局定性互证。

### patterns interp 对 LuaJIT 残余差距归因 + ItemClass 预分类否决（2026-10-06）——差距定性均匀分布，微整形第三处穷尽点确认
patterns 内核去移植税后对 LuaJIT interp 仍 1.28x（19.4 vs 15.2ms，路线图旧读数
1.5 为去税前），本轮首次归因该残余。samply 双侧同刻剖证（160 runs，8377/7238
样本）：
- **ulua interp**：match_item 39.6% + max_expand 9.7% + 派发 9.0% +
  lua_s_newlstr 8.7% + format 3.8% + gsub/gmatch/find 驱动 3.2/2.5/2.4%；
- **LuaJIT interp**：match 45.3% + matchbracketclass 10.9% + lj_str_new 5.2% +
  GC 4.2% + 派发 ~7.0% + 驱动 2.6/1.1/1.3%。

绝对差距 4.2ms 均匀摊在 intern（0.9ms）/派发（0.7ms）/驱动（0.8ms）/matcher
核心（~1.1ms）——无单点 >5% 的可移除税，matcher 核心全砍平也只 ~5.5%。
**实现轮**（fork opt-patterns-luajit，分支 1a77efbc 保留未合并）：汇编核对发现
max_expand 的 Esc 扫描环内 LLVM 未外提环不变量链（pat 界检查+装载+CLASS_TABLE
查询+id 范围检查 ≈13 uops/字符）——ItemClass 预分类（Esc 解析成谓词行指针 +
Bracket 折 256 位位图，oracle 全值域对账 3 测试）+ match_item 字面直连推进，
汇编门确认扫描环收紧到 8 uops/字符。**配对评测三系列：med -2.1% / -2.0% /
-1.0%（19/22 轮胜）**——方向一致但低于 5% 门、用例噪音地板（~3-4%）、E3/E5
一致性先例下限（-2.6%），跨窗口漂移内不稳定，**按纪律否决还原**。
**机制教训两条**：① dflt 单次求值换 classify 形态实测 **+9.3% 回退**——
ItemClass（Bracket 变体 32B 位图，枚举 ~40B）按 sret 内存往返返回，逐元素
（含全部失败位置）支付；位图/预分类收益只在扫描环内分类一次摊销成立，单次
求值场景 singlematch 寄存器内联不可替换。② 失败位置是数量主体（gsub/find
逐位扫描 ~35 次失败/迭代 vs ~14 次成功），其固定成本 = matchdepth + interrupt
链 + 分派 + classend + singlematch，interrupt 检查点与 matchdepth 为 cpp
oracle 同点位钉死，不可动。patterns interp 1.28x 记**第三处穷尽点**（与 fib
帧建立、coroutines 恢复边界并列的诚实上限），对 LuaJIT 残余为 C 级 matcher
代码生成质量差距。 literals 密集负载（find 长字面前缀）若成热点，可从分支
1a77efbc 复用字面直连推进。

### LICM 蓝图指令级细化否决（2026-10-06）——可外提项崩塌至零，归档不实现
LICM 蓝图细化到 matmul j 环指令级后**收益崩塌**：逐条审查可外提项——
- **可外提（环不变）**：CHECK_TAG R13（bk 表 tag）、LOAD_POINTER R13（bk 表
  指针）、CHECK_NO_METATABLE——共 **3 条守卫**，全为非依赖链指令；
- **不可外提（依赖环变量 j）**：CHECK_ARRAY_SIZE（sizearray vs j-1）、
  GET_ARR_ADDR（基址+j×16）、LOAD_TVALUE（bk[j] 读）——数据本身随 j 变化；
- **已优化**：aik = ai[k] 在 k 环头仅加载一次（跨 j 环驻留已天然成立）。
3 条非依赖链守卫指令 × **吸收论**（E5 归档实测：守卫链被超标量窗口吸收，
FORN 整数化/tag 外提/守卫链三轮互证）≈ **零收益**；CHECK_ARRAY_SIZE/GET_ARR_
ADDR 的外提需 sizearray 不变量假设（resize 时失效——需额外守卫，反增指令）。
**判定**：LICM pass 数天级成本对应近零收益，**归档不实现**。matmul 2.88x 的
残余差距终局定性维持：FP 链延迟 + 寻址发射 + trace 级寄存器驻留——结构性差距。
J1 全领域（BBV/寄存器驻留/LICM/两遍架构/前置 0）探索关闭，全部路径实测穷尽。
系统性调试的理论闭环：versioned load 跨回边命中在**标准数据流框架下不该发生**
——环头入口态 = merge(FORNPREP 前段出口, 环尾出口)，非支配定义保守失效（j 的
递增使 R12/R16 值跨迭代变化）；**唯一能命中的是环不变量（R13 bk 表指针、其
tag/sizearray/metatable）**——而它们的命中需要 **LICM（loop-invariant code
motion：preheader 显式建块 + 环不变量外提）**，独立于 const-prop 回边传播。
终局定性：
1. const-prop 回边传播路线**关闭**——标准数据流下无收益（非支配定义失效），
   且非保守实现有悬空引用风险（前轮实证）；
2. **正确实现形态 = LICM pass**：preheader 构建（CFG 变换，环不变量识别 +
   外提合法性：无别名写、无抛错穿越）→ ci/bk/aik 的 LOAD_POINTER/LOAD_DOUBLE/
   CHECK_TAG/CHECK_ARRAY_SIZE/CHECK_NO_METATABLE 全部外提到 preheader →
   j 迭代只剩 GET_ARR_ADDR/LOAD/MUL/ADD/STORE ≈ 5 条核心指令；
3. 工程量：preheader 构建（CFG 变换）+ LICM 合法性分析 + golden IR 全量重录
   + 全测试回归 = **数天级专项**（较此前估计再加 preheader 构建项）。
J1 专项维持「待专项预算」封存，启动蓝图以此终局定性为准。

### 前置 0 实现轮续（专项调试会话）——第四前置完整机制破案，重构面修正
最小两遍复现 + 断言改打印（deep_arithmetic_chain 场景）：块 0 的 expected=141
vs lower 实际后继=2——两遍的链结构分叉实锤。追因：**collect_only 只拦截了
substitute_or_record 与 kill 两类，而 const_prop_in_inst 另有 26 处分布式
`replace_ir_function` 家族 IR 修改调用**（LoadTvalue 合并/TryNumToIndex 折叠/
TruncateUint 改写等）——单点拦截遗漏这些通道 → 第一遍「collect」实际修改了
IR → 第二遍的链/替换决策基于已变 IR → expected 链分叉。**修正认知**：
前置 0 的真实重构面不是「3 文件改动面」而是 **const_prop_in_inst（2069 行）
的 analyze/transform 结构性拆分**（全部 26+ 处 IR 修改调用按指令类别归入
transform 遍，analyze 遍纯状态登记）——工程量与风险的认知修正，维持周级
以上专项封存。三遍架构设计本身不受影响（collect/propagate/apply 三遍正好
承载 analyze 拆分后的 transform 时机）。
按蓝图实现（fork opt-depatch0）：collect_only 标志（substitute 1 + kill 16 调用
点守卫）+ 两遍外层 + 出口快照/恢复 API（VersionedExitSnapshot）。三层递进调试：
① 外层两遍 + 全局尾态 restore——炸（exit_snapshots 语义错误：循环里对每块存
的都是全局尾态而非块出口）；
② 修正为链内逐块出口快照 + restore——仍炸（同一断言）；
③ 保留策略微调（删复位）——仍炸。
③层「restore 只增已知 + 链上字段值与第一遍一致」的推演与实测矛盾——存在
未定位的 state 缓存生命周期交互（collect_only 跳过 kill 使 invalidate_ir_op
漏跑、或 restore 注入与 StoreTvalue 失效链的次序耦合），调试树已三层，超出
碎片轮收敛范围。
**按纪律完整还原**。终审结论：前置 0 的两遍 + 快照形态需**专项会话**（系统性
IR/状态调试），连续碎片轮无法收敛。E5/J1/prequisite-0 领域全部封存，待专项
预算启动。

### J1 实现预研（2026-10-05 深夜）——三重前置确认，单轮不可闭
侦察 const-prop 链遍历与 state 生命周期，回边活值传播的实现面上发现**三重
前置**，确认超出单轮收敛范围：
1. **version 轨迹跨遍稳定性**：versioned CSE 键含 `regs[].version` 计数（每次
   invalidate/save 递增）——第二遍遍历的 version 轨迹受第一遍替换/kill 的指令
   形态变化影响而偏移，回边快照的 version 值跨遍不可靠；
2. **替换幂等性证明**：回边传播需两遍遍历，第二遍在第一遍已 substitute/kill
   过的 IR 上重走——替换幂等性需逐一证明（substitute_at/kill_ir_function_ir_
   inst_at 与 value_map 的一致性窗口）；
3. **回边合并不动点收敛**：环头入口态 = merge(首入态, 环尾出口态) 的不动点
   迭代需环体 killed-by 写的精确枚举（versioned load 的 killed-by = 任意
   StoreTvalue/StoreSplitTvalue 到未知地址，保守全失效的现有语义需细分）。
基建现状：tag 回边传播框架已存在（propagate_tags_from_predecessors +
block_exit_tags）但**只覆盖 tag，不覆盖 versioned value/value_map**——扩展面
清晰但触及 const-prop 核心不变式（替换与状态更新耦合在单遍遍历中，拆分 =
pass 幂等化重构）。归档为 J1 实现预研的三重前置清单，实现轮需以两遍架构
立项并按此清单逐项突破。

### J1 两遍架构第一项实现轮（2026-10-05 深夜）——第四前置浮现，还原
按清单实现第一项（fork opt-j1-twopass）：ConstPropState 加 `collect_only` 标志
（substitute/kill 共 113 个调用点经 state 单点拦截）、外层两遍遍历（collect→
apply，visited/链痕迹字段复位）。两层新发现：
1. **链痕迹残留**：expected_next_block/chainkey/sortkey 是链遍历持久副作用，
   第二遍未到达的块残留第一遍值 → lower 的 expected 一致性断言撞残留 → pass
   切换处复位（可解）；
2. **第四前置（阻断）**：apply 遍历中替换改写 IR → 后续链判定的 live_out
   （`get_live_out_value_count` 基于现 IR）随之变化 → 新链的 expected_next_block
   与 lower 线性化序列错位（deep_arithmetic_chain 测试复现）——即「链结构与
   lower expected 断言耦合」：即使第二遍沿第一遍记录链走，apply 的替换仍使
   **后续新链**的判定与 lower 期望漂移。根修 = 链判定与 lower 线性化的解耦
   （lower 改用记录链驱动）或替换延迟到所有链完成后统一应用（又一个 pass
   幂等化子问题）。
按纪律完整还原。**J1 两遍架构的前置清单更新为四项**（原三项 + 链判定/lower
解耦），实现预算进一步上调（数天→周级）；E5 扩展与 J1 微整形切片的既有否决
结论均维持。

### J1 前置第一项 B 方向实测否决（2026-10-05 深夜）——deferred 单遍存在级联弱化，位级移植约束根本冲突
B 方向（替换延迟到全部链完成后统一应用）实现（fork opt-j1-defer）：ConstPropState
加 `collect_only` 标志 + `deferred_edits/deferred_kills` 计划表 + `apply_deferred`
统一应用（传递链解析 + 保守跳过被值引用的 kill）。结果双重实证：
① golden IR 测试（double_contraction_deduplication）失败——deferred 模式下同遍历
内后续 CSE 的**级联传播丢失**（就地模式的替换立即改写 IR，后续指令基于替换后
形态级联 CSE；deferred 模式收集时值未传播，级联链断裂）；
② matmul A/B +0.9% 零收益（级联丢失抵消解耦收益），互相印证。
**深层结论（架构级发现）**：const-prop 的「替换即时改写 IR → 后续指令基于替换后
形态级联」是**位级移植的核心语义**（与 cpp OptimizeConstProp 逐位对齐的根基）；
任何解耦形态（两遍/deferred）都会断裂级联 → J1 前置第一项在位级移植约束下
**无可行解**。唯一出路 = 放弃位级移植改为行为移植的重写级专项（cpp 上游新版
IR 的做法），与 J1 本体重估合并立项。前置清单第一项标记**不可行**，J1 专项
预算需求升级为重写级。

### interpreter 派发头 256 表切片——汇编门不通过，还原（2026-10-05）
最后一条结构性预案实测：派发 match 改 u8 穷尽（91 个 `_U8` 关联常量 + `_` 臂
复刻钳-Nop 语义），意图消边界检查。**汇编门不通过**：a64 侧 LLVM 把非零基值域
的范围检查优化成 `sub #26 + cmp #89 + b.hi` 的**等价形态**——边界检查未消除
（91 变体的判别值域起点的非对齐性使 LLVM 仍需范围判定），试探性形态变化不构成
净赚（`sub+cmp` vs `cmp` 至多省 1 条且不在依赖链）。两处附带发现：
① 派发 match 尾部原有 `_ => unreachable!()` 死兜底（from 已钳合法域，永不触达），
与 wildcard 臂冲突后已辨析两者语义等价性（wildcard 复刻钳-Nop 行为）；
② CLI `--mode=binary` 的输出走 stdout（位置参数非输出路径）。
按纪律还原。interpreter 派发头专项与 E5/J4r 一并封存——**M2 Max a64 上
interp/native 两侧的微整形候选已全部实测穷尽**，剩余差距均为结构性（trace 级
特化、架构布局、语义红线），后续压缩需 J1 重估或架构级变更立项。

### JUMPIFNOT 尾接 GETTABLE 复审否决（2026-10-05 深夜）——E5 链后残余零头
最后一条未融合边实测（fork opt-jif-gettable）：h_jumpifnot 顺序流尾在
addk/fornloop 试探后补 `fuse_succ_gettable`（life 的 `if X[j] then` 形态
2 次/迭代）。**基线时效教训再现**：首测以 E5 合并前的旧基线得 -5.1% 混合值
（混入 E5 SUBK 融合收益），重建当前 dev 基线后净效果 med **-1.3%**（22.7→22.4，
7/7 配对胜）——真实但低于 5% 门与 E3/E5 先例下限（-2.6%）。定性：E5 链已把
SUBK→MODK→ADDK→GETTABLE 收成单派发，JUMPIFNOT 尾融合只剩零头。按纪律还原。
**E5 链融合领域二次收束**：连剩余零头一起，life interp 的融合面已全覆盖。

### E5 扩展复审否决（2026-10-05）——试探税抵论证，JUMPIFNOT 后继扩展封存
life 字节码全形态在手（codegenasm 意外完整输出）后复核 E5 扩展空间：j 环
JUMPIFNOT 后继分布 = GETTABLE 2 / MODK 3 / SUBK 2 / ADDK 1（每迭代 8 次）。
扩展「逐个试探 SUBK/MODK/GETTABLE」的期望探测数 = 2.13/次 vs 现态 [ADDK,
FORNLOOP] 的 1.9/次（miss 线性叠加试探税）；省 7 次派发 ≈ 90 cycles 抵消试探税
后净赚 ~2-4%——与 FORN 整数化同级的「税抵/吸收」风险区，且 matmul 权重教训
（单用例 10% ≈ geomean 0.4%）适用。**按纪律否决收束，未实现**。至此解释器组
interp 侧单用例微整形杠杆全部封存：剩余大杠杆（E5 更长链、J1 重估、DSE 修复
复用）均需专项预算与重估依据。

### 片5 落地（2026-10-06，合并 df1f2614..e3cd43fe）——CALL/NAMECALL 真译出 + NEWTABLE 四缺陷根因，geomean 1.015
子代理开发 + 主流程独立复核（ Spectralnorm -21% 与 micro_arith 回退双独立确认）：
- **CALL/CALLFB 直译**：原生被调七重守卫快路（tag/非C/exectarget/实参数恰等/
  非 vararg/ci 槽/栈余量）→ savedpc 落 CALL 后 + 建帧六写 + NATIVE 旗标 +
  `br exectarget`——原生→原生链共享一次 gate 进入零往返；通用慢路调
  `call_fallback`（JIT 同源，luaD_call 被否：n_ccalls 窗口会把 Lua→Lua 被调内
  yield 误判 C 边界），返回三分派（C 收尾重载 R_BASE 续原生 / 让出哨兵
  emit_exit(false) / 解释被调 emit_exit(true) vm_reentry）。
- **NAMECALL 主位快路**（hash 编译期取 k[aux]，gslot_hit 三连，miss 全量 bail）
  顺序落入 CALL 段。**NEWCLOSURE/CLOSEUPVALS**（三型捕获/luaF_close 槽复用）
  ——`local function` 主 chunk 资格解锁。
- **NEWTABLE 桩腐化四缺陷根因**（子代理定位）：NEWTABLE 双字 insn_len 错记、
  PREPVARARGS 非 noop（base=l->top 重排+窗口扩张）、copatch 块裸 put 绕过
  commit/extend（life 规模越界）、桩复陆后回归钉。
- **独立复核抓出并修复第五缺陷**：片5 的 PREPVARARGS「base==top 快路守卫」
  在 runner harness（入口 base≠top 常态）每次进函数即 bail，丢掉 micro_arith
  -11% 收益——改为忠实形态（base=L->top 无条件移动 + R_BASE 跟随 + ci/L
  base·top 重排 + 容量守卫 bail），ON 恒 26.1ms 恢复（4/4）。
- **独立实测**：spectralnorm -21%（3/3）、micro_call 中性（交接往返 ≈ 解释器
  3 派发，call-dense 环常驻原生留片6）、micro_arith -11%（修复后 4/4 恒
  26.1）；6627+112 全绿、三 crate 门禁 0 错、13 rt 位一致用例全绿。
- **全组 geomean（独立口径）**：加速解释器 vs LuaJIT interp = **1.015**
  （子代理同会话 0.978 属机况漂移，两口径入档）。领先：microbig_gettable
  0.41/micro_gettable 0.43/life 0.47/tablesort 0.50/micro_arith 0.59/
  spectralnorm 0.64；**落后集中在 call/recursion 系**：nsieve 2.99/fib
  2.09/coroutines 1.92/micro_call 1.62/fannkuch 1.38/binarytrees 1.35。
- **片6 候选**：FASTCALL1/2/2K/3（patterns/oop 主 chunk 资格）、FORGLOOP/
  LENGTH（patterns 22.8%）、call-dense 环常驻原生、IC/quickening。

### 片6 落地（2026-10-06，fork opt-copatch）——FASTCALL 全形 + FORG 族 + LENGTH + DUPCLOSURE/DUPTABLE/NEWTABLE 真译，geomean 1.004
按族独立验证（rt 三方位一致 + 策略资格门 + ABBA 配对），三处发射根因现场修复：
- **FASTCALL/1/2/2K/3 直译**：A 域 = bfid（LUAU_F_TABLE 下标，编译期已知），
  尾部第 skip 字为回退 CALL（被调加载指令夹在中间，命中时整段跳过）。命中
  语义 = 解释器 dispatch_fastcall 逐位：safeenv 守卫 → `(l, ra, arg0,
  nresults, args, nparams)` 六参 C ABI 直调内置项（movz/movk×4 物化指针）；
  `n>=0` 命中（MULTRET 收 top = ra+n、0 参形态恢复 top = ci->top、1/2/3 参
  不动 top）跳过加载段与 CALL；**`n<0` 拒答 = 顺序落入加载段**（解释器
  continue 'dispatch 顺序语义——拒答后被调槽由加载指令填好再走原生 CALL；
  bail 到 CALL 位会跳过 GETIMPORT，rt 抓到「call a string」即此）。
  FASTCALL3 的 arg2/arg3 压栈在 bfid/safeenv 判定之前（解释器臂层级，缺席项
  照压）；bfid 缺席（本仓 LUAU_F_TABLE 仅移植 modf/extract/byte/rawequal/
  buffer 簇/vectormin，余槽为 luau_f_missing 恒 -1）→ 零发射（除 FASTCALL3
  压栈），与解释器观测等价。FORGPREP_NEXT/INEXT（pairs/ipairs 特化布防）：
  控制槽判据（NEXT 要 nil、INEXT 要数值 0）+ safeenv + ra+1 表三守卫 →
  二写布防（ra=nil、ra+2=done，ra+1 本就是表）；迭代器为 function 原样跳出；
  其余 bail（typeerror 路交解释器全量重做）。
- **FORGLOOP 双路**：回边 interrupt 门（FORNLOOP 同款 + BackedgeHeapCheck
  编译期门）；内建表迭代（ra=nil ∧ ra+1=table）= aux>2 动态环清额外变量 +
  aux<0 数组段早停 + `forg_loop_table_iter` 直调（数组+哈希段一体，JIT 回调
  同源）；泛型路 = savedpc 落 aux 字位（解释器 VM_PROTECT 同位点——让出续延
  luau_finishop 读 savedpc-1 还原主字的协议前提）+ `forg_loop_non_table_
  fallback` 三态回调（performcally 的 n_ccalls/OPYIELD/GC 步进全在内）：
  -1 让出 emit_exit(false)、1 跳环体、0 落出口。协程让出穿 FORGLOOP
  （coroutine.wrap 迭代器 rt 用例）逐位一致。
- **LENGTH**：table 且 fastnotm(TmLen)（无元表或 tmcache 负缓存位已置）→
  lua_h_getn 直调；string → len 字段 scvtf；元方法与其余类型 bail。
- **DUPCLOSURE/DUPTABLE 真译**（JIT 回调 execute_dupclosure / lua_h_clone
  直调）+ **NEWTABLE 由 bail 桩改 lua_h_new 真译**（B 域哈希 log2 编码 + aux
  数组容量 + CheckGc 同款）。动因：帧内任一 bail 即整帧回解释器**不再返场**
  ——主 chunk 顶部的建表桩/加载段把热环一并拖回解释器（A/B 实证 spectralnorm
  -22%：主 chunk 4 处 DUPCLOSURE 解锁后驱动环 + 归约环全原生）。
- **发射根因三处（现场修复）**：①adr_u64 数据池在直译装配路径缺席（JIT 镜像
  = code+data 拼接，直译只拷 code 字）→ SIGBUS，改 movz/movk 纯指令物化；
  ②lua_h_clone 两参 ABI（l, tt）误作单参 → GC 头损坏 newgcoblock 空引用
  （lldb bt 定位）；③FASTCALL 拒答路 bail 到 CALL 位跳过加载段（见上）。
- **rt/资格门**：rt 加 4 族共 17 用例全绿（FASTCALL 全形含缺席项 / FORG 全
  形态含协程让出 / LENGTH 元方法 / DUPCLOSURE oop 形态）；策略资格门加 5
  断言（fastcall / forg+length / dupclosure 族 + patterns/oop 基准整源
  compiled=1）；clippy 三 crate -D warnings -W absolute_paths 零错。
- **A/B 配对（同 runner 二进制旗标开关，ABBA×5 med×3 runs）**：spectralnorm
  **-18.2%**、micro_arith -11.4%、queens -2.8%；全组其余无 >2% 回退（最大
  coroutines +1.5% 噪音域）。**patterns/oop 吸收入档**：加速解释器形态
  safeenv≡0（片3 不变量：copatch-only 装配不得扰动未译 proto 的解释语义），
  GETIMPORT 双守卫（k 缓存 + env.safeenv）语义必 bail——patterns 外环体 5 处
  string.* 导入 / oop setup 段 setmetatable 导入一次即整帧回解释器；该守卫与
  JIT 同源非实现缺口。JIT 形态（safeenv=1，copatch 优先）三方同域：patterns
  18.1≈18.1、oop 14.3≈14.4。
- **call-dense 环常驻（片6 任务③）实测证伪入档**：临时放宽热信号门（含 CALL
  即译）四例实测——micro_call +6.6%（较基线 +1.1% 恶化）、fib +0.6%、
  inherit3 +2.5%、queens -3.6%：递归小函数译出后门进出 + 七重守卫 + 建帧
  ≈ 解释器派发，净损耗。残余 micro_call +1.1% = 主环 native 化后
  CALL→call_fallback→解释被调→RETURN 折返链 vs 纯解释派发差额（片5 记录
  +2%，两片累计压至 +1.1%）。翻转需 CALLFB 频率反馈驱动的选择性译出
  （观测槽已在，机制留后片），实验探针已回滚。
- **全组 geomean（独立口径同片5）**：加速解释器 vs LuaJIT interp =
  **1.004**（片5 1.015 → -1.1%），反超 10/24 例。领先面：microbig_gettable
  0.39 / micro_gettable 0.39 / life 0.45 / tablesort 0.51 / spectralnorm
  0.66 / micro_arith 0.60 / micro_arraywrite 0.78 / micro_tablegrow 0.77 /
  mandel 0.98 / strings 0.99 / inherit3 1.00；**落后集中 call/recursion 系
  与导入链**：nsieve 2.98 / fib 2.06 / coroutines 1.93 / micro_call 1.62 /
  fannkuch 1.41 / binarytrees 1.35 / patterns 1.31 / nbody 1.31 / matmul
  1.22 / queens 1.21 / oop 1.20。
- **片7 候选**：CALLFB 观测驱动的 call-dense 选择性译出；GETIMPORT 的
  safeenv 解耦（加速形态单独放行 k 缓存命中，需独立语义论证）；IC/quickening。

### 片7 落地（2026-10-06，fork opt-copatch）——call/recursion 系缺口收口：JUMPBACK 臂 + GETIMPORT 解耦论证 + 空体分支崩溃修复，geomean 1.003
四项按任务书独立验证（rt 三方位一致 + 策略资格门 + ABBA 配对），两定谳推翻任务书前提、两硬伤顺手收口：

- **A·GETIMPORT safeenv 解耦（patterns 1.31）**：语义论证落地（写进臂注），
  结论比任务书预期更深——k[D] 导入缓存的**唯一写入者**是装载期
  `resolve_import_safe`（safeenv=0 分支写 nil，cpp lvmload.cpp:214 同构），
  `luaV_getimport`（解释器慢臂与 JIT helper 同函数）**从不回写 k**；加速形态
  （片3 不变量 safeenv≡0）下 k[D] 恒 nil ⇒ 「k 缓存命中」分支结构性死亡，
  safeenv 守卫删之零观测。已落地：拷贝快路去 safeenv 检查（a64 JIT
  IrCmd::GetCachedImport 同形——上游 JIT 本就不查运行期 safeenv），rt 四态
  位一致用例（sandbox 装载命中 / 加速未命中 / 三段未缓存链 / setfenv 后）。
  **未缓存臂 helper 直调形态实测后回退入档**：JIT 回调 `get_import` 直调
  （savedpc 落指令起始 + luaV_getimport 全语义，位一致构造性成立，rt 四态
  全绿）patterns -3.3%，但 oop 主环（5 次解释被调 CALL/迭代）帧常驻后暴露
  「call_fallback C 往返 + 解释器重入 + VM_HAS_NATIVE 门」三段交接税
  A/B +5.2%（bail 形态 oop 开关中性——片6 的 setup 段 bail 恰好屏罩
  call-dense 主环），踩 >2% 红线回退为 bail（2d60b592）。helper 形态与
  CALLFB 观测驱动的 call-dense 选择性译出同批入片8（两者只有合取才有净收益）。
- **B·micro_call 1.63 / fib 2.16 剖面定谳（samply，2000/300 runs）**：任务书
  「fib 递归自调用的被调已是原生（七重守卫快路应命中）」前提**证伪**——
  fib/f/micro_call 的被调 proto 无回边，热信号门拒译，剖面 fib 解释器叶
  99.8%（原生 0.0%）、micro_call 97%（主环原生 2.5%）。比值差距 = 纯解释器
  call/return 派发效率差（fib ≈85 cycles/call vs LuaJIT interp ≈40），
  copatch 不可达；片6 已证伪的 call-dense 放宽门（micro_call +6.6%）与本片
  oop 交接税互为印证。**无可动面，入档封存**；杠杆在解释器派发（IC/quickening/
  LJR）或片8 选择性译出。
- **C·nsieve 3.19 剖面**：拒译点**不存在**（探针 copatch_compile_counts
  compiled=1）；真因 = 首个内层 GETTABLE 数组快路未命中（空表 sizearray=0）
  → 整帧一次性 bail 永久落解释 + 表写机制本体主导（lua_v_settable/newkey/
  rehash/mainposition ≈67% 样本，解释器派发仅 ≈33%）。修复面 = GET/SETTABLE
  未命中臂改 lua_v_gettable/lua_v_settable helper 直调（GETIMPORT 同式），
  收益天花板 ≈ 派发份额，入档片8 候选。**顺手抓到真硬伤**：`if cond then end`
  空体形态（JUMPIF/JUMPIF(LE/LT)/JUMPXEQKN 跳转目标与顺序路同址）下
  emit_jump_truthy 系的 target/next 双 Label 副本各自分配 id + 回写互踩，
  留下永不 bind 的 pending fixup，finalize 的 LABEL_UNBOUND 断言**即进程
  崩溃**（dda77d02）——修正 = 同址分支语义真空零发射；策略资格门 +
  rt 位一致双钉住。
- **D·coroutines 1.91**：任务书评估项升级为实作——「resume 后返回原 copatch
  帧」机制**已在树上**（luau_execute 入口 VM_HAS_NATIVE 门 → ecb.enter
  （on_enter 按 savedpc 位重入 bc_mapping 偏移）→ gate；剖面证实 resume 后
  解释器叶 0%），真正缺口 = **LOP_JUMPBACK 无翻译臂**（仅入热信号预扫，
  while/repeat 宿主 proto 整体拒译——coroutines 两个协程 body 即此形态）。
  补臂（回边 interrupt 门与 FORNLOOP 同口径，bail 交 h_jumpback 慢路全量；
  GC 欠债由分配位 CheckGc 承担）后 coroutines 整源 3/3 全译，A/B **-22.4%**
  （热批 67.2→52.1ms），剖面解释器叶 0%。
- **全组 A/B（同二进制旗标开关 ABBA×3 med）**：coroutines -13.3%~-22.4%、
  micro_arith -14.7%、spectralnorm -22.9%、matmul -42.4%（片6 既有）；其余
  持平。**oop +4.9% 定性为构建/机况彩票**：同语义（片7 前翻译器源码）新构建
  复测 +6%，跨批次 0%~+6% 波动（判例：片4 nbody +4.9% 布局彩票、片6 双峰
  机况）；sub-ms 用例（micro_arraywrite/tablegrow）与 fannkuch 专项 9-run
  ABBA 均噪声域。
- **全组 geomean（独立口径，同 runner 分后端 ulua 旗标开 vs mlua/luajit-interp，
  各 5 runs）**：**1.003**（片6 1.004）。反超 11/24：microbig_gettable 0.38 /
  micro_gettable 0.41 / life 0.44 / tablesort 0.50 / micro_arith 0.60 /
  spectralnorm 0.66 / matmul 0.71 / micro_tablegrow 0.77 / mandel 0.99 /
  strings 1.00 / inherit3 1.00；落后集中 call/recursion 系：nsieve 3.09 /
  fib 2.16 / coroutines 1.68 / micro_call 1.61 / fannkuch 1.43 / binarytrees
  1.38。
- **新入档缺陷（既有，非本片引入）**：加速解释器形态间歇性「attempt to
  compare nil <= number」（micro_call chunk:5，f 收到 nil）——旗标开约
  0.14%/eval（2900 evals 4 次），旗标关 0/360+；oop 亦观察到一次。疑似
  CALL→call_fallback→解释被调→RETURN 折返交接面低概率腐化（片5/6 引入窗口），
  rt 单发评测未命中（每次进程仅 1-6 evals）。复现命令：
  `ulua-bench-runner --cases micro_call --runs 500 --fflag LuauJitCopatch=true`
  （多次必现）。片8 首位。
- **片8 候选**：间歇 nil 腐化根因（首位）；GET/SETTABLE 未命中臂 helper 化
  （nsieve，天花板 ≈33% 派发份额）；GETIMPORT helper + CALLFB 观测驱动
  call-dense 选择性（合取，patterns -3.3% 且救 oop 交接税）；CALL 交接微成本
  （解释被调建帧内联，micro_call +2.1% 残余）；IC/quickening。

### 片7 落地（2026-10-06，合并 fd366c67）——里程碑：加速解释器 vs LuaJIT interp geomean 首次 <1.0
子代理开发 + 主流程独立复核（coroutines 独立 A/B -14.5% 3/3、geomean 独立复测）：
- **GETIMPORT safeenv 解耦**（patterns 缺口）：加速形态（safeenv≡0）下 k[D] 恒
  nil（装载期 resolve_import_safe 写 nil，运行期无第二写者）⇒ 拷贝快路结构性
  不可达，safeenv 守卫删之零观测；缓存非空仅存在于 safeenv=1 装载形态（JIT
  共存域），上游 JIT 拷贝快路本就无运行期 safeenv 检查——同形论证 + setenv
  四态 rt 钉。未缓存臂 helper 化实测 oop +5.2%（暴露 call_fallback 交接税）
  踩红线回退 bail，与 CALLFB 观测驱动合取留片8。
- **JUMPBACK 回边臂**（while/repeat 资格解锁）：coroutines 独立复测
  **-14.5%**（3/3，67.3→57.3）。
- **退化同址分支零发射**：`if cond then end` 空体 target==next 双 Label 副本
  id 双分配回写互踩 → finalize LABEL_UNBOUND 崩进程（子代理顺手抓到并修）。
- **三剖面定谳（入档封存）**：fib/micro_call 残余 = 纯解释器 call/return 派发
  效率（fib ≈85 vs ≈40 cycles/call），被调无回边热信号门拒译、call-dense 放宽
  已证伪（片6 +6.6%）——copatch 不可达，**结构性入档**；nsieve 真因 =
  GETTABLE 空表未命中 → 整帧一次性 bail（样本 67% 表写机制），GET/SETTABLE
  未命中 helper 化天花板 ≈ 派发份额留片8。
- **全组 geomean（独立口径，同构建同时段）= 0.959 < 1.0**（11/24 反超；
  落后残余：nsieve 2.91/fib 1.84/coroutines 1.64/micro_call 1.57/fannkuch
  1.36/binarytrees 1.32/nbody 1.26）。漂移带说明：片6 同法 1.004 → 片7
  0.959，Δ-4.5% 远超漂移 ±1-2%，方向可信；绝对值有 ±2% 漂移带。
- **验收**：rt 20 + 策略 14 + 单测 5 全绿；三 crate 门禁 0；6634+112 全绿。
- **首位遗留（正确性，下片头项）**：加速形态间歇「attempt to compare nil <=
  number」（~0.14%/eval，micro_call/oop 均见，旗标关 0/360+；疑似 CALL→
  call_fallback→解释被调→RETURN 折返交接面，片5/6 窗口引入）。主流程独立
  复跑 500 连跑未复现（子代理称必现）——未定位，下片以重现器+llldb 位点
  攻坚。**旗标默认关，主路径不受影响**。

### 纯解释器第一轮剖面复核（层撤销后，2026-10-06）——全部热点确认封存，零动面
双模式定形后首轮回访（samply 5 用例 ≥2000 样本/例）：microbig_gettable tier_cold
99.3%（融合链摊派）、life 86.5% 同构、patterns matcher 内核 51%（第三穷尽点）、
nsieve 表写机具 66.7%（cpp ltable.cpp 逐位同构，E1 语义红线维持）、oop tier_cold
50.4%+堆侧 16%。三候选逐一复核：KS IC 双层已接线无未接槽位（站点级链 IC=J4-real
立项封存）、表写算法面被 oracle 钉死、派发头/中断粒度/savedpc 写回三处 cpp 同构。
**与 cpp oracle 同刻配对：life/microbig/micro_gettable 快 1.5-1.9x、oop 1.34x、
patterns 1.26x、nsieve 仅 1.13x（最薄边）**。结论：纯解释器可移植优化面已穷尽
（与层撤销前档案互证），诚实零产出。资产留存：samply 聚合器 /tmp/agg_*.py、
剖面 JSON /tmp/prof-*.json。

### 加速解释器层撤销（2026-10-06，用户裁决）——双模式形态定形
- **裁决理由（硬约束，入 `.agents/skills/speedup/SKILL.md`「0. 目标形态」）**：
  ①不设第三执行层——执行形态收敛为双模式（JIT + 纯解释器），禁止字节码直译
  之类的第三层；②解释器必须 iOS/wasm 全平台可跑——纯解释路径不得依赖运行时
  代码生成，copatch 层（a64 直译）按此撤销（片8 后 WIP 探针一并清除）。
- **实测成果入档不删**：加速形态 vs LuaJIT interp geomean 0.95（片8 任务 C
  独立口径）、micro_arith -40%、spectralnorm -22.9%、coroutines -22.4% 等
  全部数据已在片2-8 节，作为历史档案保留——后续优化路线的证据基线。
- **腐化随层撤销**：间歇 nil 腐化（~0.14%/eval，片8 任务 A 建成重现器、钉死
  tag-only 指纹，机制未竟）未根治，随层一并消失——这是撤销的实际收益之一
  （正确性风险清零，不再需要金丝雀与 A/B 双态旗标）。
- **配套撤销**：`LuauJitCopatch` 旗标、copatch-only 编译策略、copatch 上下文
  懒装配、NEWCLOSURE 直译 helper 槽、rt load 门全链拆线；外围 ulua-accel
  基准引擎与网站图表条目不在本线（独立分支），本线双模式基准清单不变。
- **纯解释器后续优化手段（继续迭代）**：IC 数据结构、派发机制、超级指令等
  可移植形式——目标不变（解释器逼近/反超 LuaJIT interp），手段限定为
  全平台可跑的解释器内部改造，不再走生成码路径。

### 片8 任务 A（2026-10-06，腐化攻坚：重现器建成 + 指纹钉死 + 机制未竟）
- **确定性重现器建成（crates/ulua-rt/tests/copatch_call_gc_stress.rs）**：
  四负载（micro_call 同源/GC 压力/变深递归/协程小栈）加速形态 vs 解释器
  逐位对照，默认 100 轮 `COPATCH_STRESS_ITERS` 可调；**数秒内必复现**
  （此前 2900 evals 4 次、500 连跑 0 次的间歇性终结）。注意：rt 默认环境
  无 `collectgarbage` 全局（首次负载误用已修）。
- **腐化指纹（lldb 硬件观察点 + 现场转储钉死）**：活栈槽 **value 完好、
  仅 tag 被翻 0**（setnilvalue 形态；`p1.tt=0, p1.n=11.0`，邻槽与 ci 链完好，
  L->base==ci->base 无视图分叉）。写者 = 主 chunk 直译块自身的 LOADN tag
  store（`str w18,[x25,#0x6c]`）以 **W18=0** 执行（movz 未生效，即控制流
  楔入双指令 store 序列中段）；每次 eval 恰一次、迭代位随机。
- **机器级排除项（全部实测验证，非推理）**：门折返靶点（exectarget/
  execdata offsets[12]=580→FORNLOOP 臂）✓；FORNLOOP 回边 b.le→MOVE ✓；
  代码字未被改写（码字观察点阴性）✓；return_/continue_call/gate 三折返链
  base 重载齐全（含上游对照 emitContinueCall/emitReturn）✓；copatch_only
  策略正确（拒译 proto 不落 JIT fallback，f 保持解释）✓；W18 平台寄存器
  干扰假说（换 W9 实验）阴性；本仓 GC 特有 propagatemark→shrinkstack
  （GC 步进可收缩搬栈）已确认存在，但全部直译侧步进点位（NEWTABLE/
  DUPTABLE/NEWCLOSURE/DUPCLOSURE/FORGLOOP/call_fallback done 臂）均带
  base 重载，且 micro_call 无 GC 步进仍复现——**非搬栈类**。
- **未竟**：异常 pass 的进入边（谁把控制流送上 store 指令）未钉死；
  watchpoint 报告 pc 与存活性在 macOS arm64 上归因存疑。下片头项 =
  以金丝雀（`--ignored` 一键复现）+ 单步级 trace（gate br 靶点记录环）续攻。
  **旗标默认关，主路径不受影响；金丝雀常绿由控制组保证。**
- **主流程复核迭代修正（同日）**：①双重绑定假说证伪——copatch_translate.rs
  全部 bind_label 枚举（主循环每 pc 恰一次/捕获字与双字 aux 槽各一次且
  pc 推进正确跳过/退火同址分支 dda77d02 已防）无双重绑定；②watchpoint
  归因修正——Apple Silicon 用户态硬件观察点按 8 字节粒度且可能延迟报告、
  异常交付会污染 X18（平台寄存器），故「W18=0 中段进入」指纹解读不可靠，
  真实事实收缩为「R6.tt 发生一次 3→0 的 tag-only 写」；③探针假阳性两处
  （rt 环境无 collectgarbage 全局；0 参调用 reg0 垃圾残留 + 栈内存复用
  残值），修正后金丝雀呈**随机缓解/复发**（全 harness 轮次 ~10% 失败率，
  20 连绿不可稳定达成）；④GC_PRESSURE 形态实证 main↔probe 门交替
  （probe 已直译、外层 CALL 走快路 br），折返链全部实测正确。
- **下一步（片9 头项）**：ulua-reduce-cli 对 micro_call 负载做字节码级
  约简求确定性单发复现 → 单步级定位；或全帧槽 tag 不变量巡检（gate 折返
  点全 ci 链扫描）捕获首次越界写。
- **顺带修正**：直译器 tag store 暂存寄存器 W18→W9 实验已还原（保留 W18
  与 JIT 约定一致）。

### 片8 任务 B/C（2026-10-06，GET/SETTABLE 未命中 helper 化 + 全组 geomean）
- **GET/SETTABLE 未命中臂 helper 化（nsieve 缺口）**：数组快路守卫失败从
  整帧 bail 改为 `lua_v_gettable`/`lua_v_settable` 直调（NativeContext 既有，
  解释器慢路同源全语义；X0=l、X1/X2/X3=t/key/val 槽，返回后 R_BASE 防御性
  重载）。**实测 nsieve ABBA×5 med 中性（34.3 vs 34.2）**：天花板分析兑现
  ——收益≈派发份额，写机制本体（rehash/newkey）与 helper 共享；且新加的
  step 守卫让内层标记循环（工作量主体）整循环走解释器。
- **连带挖出并修正潜伏错码：FORNPREP step≠1 无守卫**——FORNLOOP 臂回边
  增量硬编码 +1.0，`for j = i*i, n, i`（step=i）形态自片4 起若帧保持原生即
  错步进（此前被首写 bail 恰好掩盖，任务 B fallback 使帧常驻原生后暴露，
  nsieve 最小复现 on=2 vs off=15）。修正 = FORNPREP 臂 step==1 运行时守卫，
  不满足整循环 bail 落解释器（宁慢勿错）；step≠1 泛化 FORNLOOP 入档后续。
- **rt**：新增 copatch_table_miss_family 位一致（空表首写/逐次扩容/界外读/
  非整键/负键/__index/__newindex），21/21 绿；mini sieve 双旗标 15=15。
- **任务 C 全组 geomean（独立口径，engine-luajit，engines ulua+mlua/
  luajit-interp，5 runs）= 26.8 vs 28.2 → 0.95 < 1.0**（片7 0.959，漂移带
  内）；旗标关对照 28.7，加速形态 -6.6% 无回退。**警示：oop 用例本轮
  ERR**——片8 任务 A 病灶在 geomean 途中发作（加速形态 opt-in，旗标默认
  关，主路径不受影响；金丝雀见 copatch_call_gc_stress.rs）。

### FORN trace 层 T2（2026-10-07，fork opt-trace，3c2ea3c5 + c3b58ad7 + be13c9ef）——寄存器驻留实证反超 method JIT、同元素证明传播、回边计数装机面
T1 交付（3c2ea3c5）之上的第二片。三项按序，逐项独立验证。

- **方法学勘误（T1 遗留②的真相）**：「trace 与 method JIT 持平 11.7ms」是
  A/B 两侧都套在 jit 引擎上的伪中性——trace 问询挂解释器 `h_fornprep`，
  JIT 引擎下原生 proto 根本不经它，旗标结构性无效。正确口径（interp 引擎
  旗标 on/off）：matmul **interpreter 27.6ms / trace 6.6ms（-76%）/
  method JIT 11.3ms**——寄存器驻留（T1 片3 已兑现：idx/acc/limit 入口一次
  装载、回边零栈槽往返、出口快照才写回）实测反超 method JIT **1.7x**。
  教训入档：trace 层 A/B 必须钉在 interp 引擎。
- **T2-1 同元素证明传播（c3b58ad7）**：ArrayLoad/ArrayStore 下标恒 phi——
  同一 trace 内全部此类访问落在同一 (table, idx-1) 元素；首次全守卫通过
  即证得「界内 + tnumber」，trace 体直线无调用无分配（表不容变更、写值全
  number、单线程），证明跨指令存活至 trace 末——后续同表访问免界/tag
  守卫、读改写 store 免 tag 重写（matmul j 环真实 IR
  `ci[j]→bk[j]→fmul→fadd→ci[j]` 的 store 端 7 指令 → 3 指令，环体
  30 → 25 条）。立即数下标（GET/SETTABLEN）不进证明面；证明面按 table
  槽索引，异槽同对象保守不共享。此面 method JIT 结构不可达（其守卫绑定
  单 IR 指令，无跨指令证明生存期）。
- **T2-2 回边计数装机面（be13c9ef）**：FORNLOOP 位点新增热度面——ecb 单槽
  IC（counter/proto/pc/target 四字段 + 慢路槽），解释器 `h_fornloop` 臂
  旗标开时内联递增、精确达阈回调慢路（录制装配 + 解除武装 + 回收计数
  单元）。武装挂 FORNPREP 问询侧：资格预扫（环体全指令落在支持集）缓存于
  注册项，**首入口即武装**（单/双入口环的第一次进入必须被覆盖——迟一问询
  会整段漏计，首版即败于此）；含 CALL 体永不武装，回边零新增税。与 T1
  入口计数并存互补：入口面覆盖高频进入（含零跳环），回边面覆盖低入口
  频次 × 高迭代量的单层长环。rt 判别：`for r=1,2 × j=1,1500` 双入口形态
  （入口面 2 < 1000 永不装机，executed 增量只能来自回边面）位一致绿。
- **端到端 A/B（双二进制 base=3c2ea3c5 vs new，各 min-of-3，7 轮配对）**：
  matmul trace 形态 **6.8 vs 6.6ms——中性**。吸收论证：①j 环已内存/分支
  吞吐饱和（5 次存访 + 6 分支/迭代），-17% 指令被 OoO 窗口吸收；②早装
  窗口（入口 1000 → 迭代 1000）只占 j 环总工作量 ~5%，低于噪音地板。
  T2 的兑现面在能力不在该用例墙钟：matmul 的 sum 环（140 入口 × 140 迭代，
  入口面永不达阈）在 T2 经回边面真实装机。
- **旗标开税负（诚实档案，opt-in 实验层的已知成本）**：IC 检查
  （旗标读 + counter 空检 + 键比对）骑在回边取指链上，~1 周期/回边——
  回边密集型解释执行用例旗标开较 T1：micro_gettable +5.6%、
  microbig_gettable +5.2%（4.7M 回边 × 0.9 cyc）、fasta ~+2.4%、
  nbody +0.8~2.1%；binarytrees/strings 复测为扫表噪音（+1.0%/+0.4%）；
  其余用例噪音内；matmul -5.9%。**旗标默认关 = 现有形态零扰动**
  （旗标读是内联块首道闸，关态实测与基线持平）。压缩路径留档：
  per-proto 热度门字段（1 载荷快速闸）或 FORNLOOP 操作码补丁式 trampoline
  （只有被装环付费），均未动手。
- **T2-3 录制面扩宽（ts[r][j] / JUMPIF(NOT)）——按剖面否决，未动手**：
  四个 A/B 目标用例的可测内环全部含 CALL（spectralnorm 的 eval_a、nbody
  的 sqrt、fasta 的 string.sub/math.floor），两级表载与条件跳转形态不解锁
  任何一个目标用例；无 5% 门候选靶点，按纪律不攒「以后可能有用」。

**验收门**：rt 位一致 9/9 绿（新增单层环回边装机 + 同表双读两判别）；
clippy 三 crate `-D warnings -W clippy::absolute_paths` 零错；matmul 形态
rt 粗测 ≥20% 对 interpreter 保持绿；全组旗标关（默认形态）与基线持平；
./test.sh 6615+112 全绿。

### FORN trace 层 T3（2026-10-07，fork opt-trace）——MOD 族录制面 + 融合回边盲区修复 + 回边税归因定谳
T2 之上的第三片。A（回边税压缩）实测不达标如实入档；B（录制面扩宽）落
MOD/MODK 族并连带修出 T2 潜伏盲区。

- **T3-B MOD/MODK 族（录制面扩宽，落地）**：`TArith::Mod` 进支持集——发射
  `a - floor(a/b)*b` 四操作序列（fdiv→frintm→fmul→fsub），与解释器
  `luai_nummod` 的 Rust 形态同序同操作（a64 `.floor()` 落 frintm，正确舍入，
  rustc 无 FP 缩合契约），构造性逐位一致、零守卫零 bail（NaN/inf/零除面
  同构）。D3 寄存器规划为 MOD 暂存（固定图唯一空闲 d 槽）。
- **T3-B 连带 ①：临时槽覆写放宽**——编译器跨指令复用临时寄存器是常态
  （`sum + i%48 + (i+7)%48 + i*7+13` 的 ADD 复用 MODK 目标寄存器），T1 的
  「已写槽复用即拒录」把一切链式算术挡在门外；放宽为覆写（SSA 语义 =
  后读见后写，与解释器逐迭代覆写同构），累加器单写纪律保留。
- **T3-B 连带 ②：常量池按 k 下标去重**——链式形态让同 k 下标多次入池
  （48、7 各两次），4 槽池被虚耗撑爆触发容量拒录；去重后 3 槽收纳。
- **T3-B 连带 ③（T2 潜伏盲区修复）：融合回边计数 tick**——`FORNLOOP` 被
  前驱臂尾融合吃掉（`ADDK → FORNLOOP` 是 micro_arith 正中热线）时
  `h_fornloop` 不派发，T2 的回边 IC 对这类位点整支失明。修复 = IC 抽成
  `forn_backedge_heat_tick`（单载荷门共享体），`fuse_succ_fornloop` 在
  `fuse_ok + backedge_idle` 门后同调——rt 新判别 `for r=1,2 × i=1,1M`
  （ADDK 尾融合边 + 入口面 2 < 1000 永不装机）钉死此面。
- **T3-A 回边税压缩（实测不达标，如实入档）**：落地形态 = 进程级单载荷
  武装槽 `FORN_HEAT_ARMED`（静态原子指针，链深 0；ecb 去 counter 字段，
  身份键/靶值留 per-state ecb；慢路补旗标翻转复核）。双二进制 6 轮 ABBA：
  gettable 系旗标开税 **+5.7% → +5.7%（零压缩）**，但旗标关与基线逐项
  持平、matmul -76% 保持。**归因定谳（三刀消融，各 ≥5 轮）**：enter() 体
  短路 +5.5%、无调用（仅旗标读+槽检查）+5.5%、完整问询 +5.4~6.8%——
  税源不在回边 IC（其成本 ~0，静态读被 ILP 吸收）而在 **h_fornprep 入口
  问询块本身**（旗标读 + global→ecb 槽链 + 参数计算 ≈ 6 指令，骑在
  FORNPREP 臂直线流上，192k 入口 × ~7ns ≈ 全部观测税）。任何在带内逐入口
  过滤（per-proto 门 / 单槽门 / 调用移除）都 ≥ 此块成本，≤2% 门在
  「旗标开 = 逐入口问询」架构下不可达；唯一出路是 FORNPREP/FORNLOOP 操作码
  补丁式 trampoline（未动手，留档）。旗标语义不变（默认关 = 零扰动）。
- **micro_arith 用例中性（吸收论证）**：MOD 族装机路径全部打通，但该用例
  是单入口 × 2M 迭代——回边面在回边 #1000 装机，本次进入剩余 1.999M 迭代
  仍走解释器，runner 新状态单发协议下没有下一次入口可收割。rt 同态协议
  （`for r=1,60 × i=1,60000`，matmul 粗测同款）实测 **≥20% 提速门绿**
  ——MOD 环体的原生收益为真，case 级中性是测量协议 × 环形态的结构性边界。
- **life/micro_gettable 不解锁论证（按剖面）**：micro_gettable 内环 =
  `if row[j] then` → JUMPIFNOT + 布尔 GETTABLE；fall-through 线性录制
  （守卫 bail-if-taken）会坠 bail 风暴（row 值伪随机，首次 falsy 即 bail，
  余下整入口回解释器），真解锁需菱形双径录制 + merge phi（单片不解）；
  life 在此之上还要 temp 下标 GETTABLE、MOD（已具备）、AND/OR、EQ——
  多族叠加，留后续片。

**验收门**：rt 位一致 13/13 绿（新增 MOD 族三形 + 融合尾回边装机 + MOD
粗测提速三判别）；clippy 三 crate `-D warnings -W clippy::absolute_paths`
零错；双二进制 6 轮 ABBA：旗标关与基线逐项持平、matmul -76% 保持；
./test.sh 全绿。遗留：A 项 ≤2% 门未达（trampoline 留档）、菱形录制、
SSA 重命名全面化（当前覆写放宽已覆盖链式算术，多赋值分支仍保守拒录）。

### FORN trace 层 T4（2026-10-07，fork opt-trace）——录制面扩宽：step≠1 泛化 + 条件跳转菱形直译 + 泛化下标（派生表装载如实回退）
T3 之上的第四片。三族按独立验证门推进；族 1 的嵌套装载子面实测不达标如实回退。

- **族 3 FORNLOOP step≠1 泛化（落地）**：装机期按录制现场 step 锁 flavor——
  step==1 走既有快回边（fadd 1.0 + 整数表示同步 +1），step≠1 走泛回边
  （fadd step + fcmpz 选向：`step > 0 ? idx <= limit : limit <= idx`，与
  fornloop_step 逐位同式；NaN step 入口拒承同观测）。PhiIdx 寻址面逐迭代
  fcvtzs+scvtf 往返精确性校验，不精确 bail 回环体头（槽已提交，savedpc=
  环体头，解释器重跑环体而非 FORNLOOP——无双重递增）；非寻址形态放行
  分数 idx 起点。MOD 体与泛 flavor 复用 d3，装机期互斥拒录。
- **族 2 直线体条件跳转·菱形双径直译（落地）**：选型论证——arm 内守卫面
  （界/tag bail）不可 CSEL 承载；acc phi 用「arm 内直写 d_acc」（AccArith，
  跳转未走自然保持旧值，回边 fmov 对其豁免）= 零选择指令零新寄存器的区域
  phi 物理形态；循环偏侧采样下分支预测优于双份算术 + fcsel。收录面：
  JUMPIF/JUMPIFNOT（number 已知值常量折叠：JUMPIF 折无条件跳、JUMPIFNOT
  零发射；generic 临时走真值面）、JUMPIFLT/LE/EQ 及 NOT 变体（fcmp + VS
  门控，NaN 无序语义逐位对齐解释器比较臂）、JUMPXEQKN（K 常量比较，aux
  低 24 位 k 下标进身份 k_consts 面、not 位 bit31；两字指令 width 步进贯穿
  预扫/资格/走查）。布尔载（CondLoad）：界守卫 only 免 number tag 特化
  （boolean 值不坠 bail 风暴），tag 驻 W9 紧邻分支 adjacency，槽值逐迭代
  回写保持解释器可见面、写回面排除。区域规则：跳转目标限环内前向位点
  （== 环尾即 continue 语义）；区域内写槽限 acc（直写面）或「先无条件写
  后区域写」的环内临时（读点全在区域终前，写回面排除）；部分交叠拒录。
- **族 1 GETTABLE 下标泛化（部分落地 + 如实回退）**：落地面 = 下标从
  PhiIdx 泛化为任意 number 操作数（InvNum/Temp/Acc/Const；非 PhiIdx 走
  fcvtzs 往返精确性校验，类别 2 bail——A/B -92.5%）。回退面 = 派生表装载
  （嵌套表载 ts[r][j] 的外层 TableLoad）：首版 mt 守卫从元素 TValue 地址
  读 LuaTable 字段（应从 value.gc 表指针读）——误 bail 风暴使 rt 假绿；
  修复后真实原生执行暴露更深问题——其写回的行表 TValue 炸后续解释器
  set_obj 的 checkliveness 断言（gc dead/tt 失配），静态生成码审计与
  llvm-mc 独立反汇编交叉验证均无果，深因在 GC/liveness 交互层。按纪律
  整环拒录回解释器（TableRef/DERIVED 寄存器面随撤，完整实现留存 git
  历史 8d2fd4c1），嵌套布尔载组合形态随之回退。连带修复 T1 起潜伏盲区：
  累加器 bail 快照原写 final_v 承载寄存器（ADD 结果临时），bail 落于定义
  点之前时该寄存器是垃圾——快照写垃圾进槽污染解释器重做路径；改写 phi
  寄存器 d_acc（任意 bail 点恒持当前累计值，出口零差异）。
- **A/B 配对（rt 级，≥5 轮 med，AB 交替，位一致前提断言）**：
  micro_gettable 形态布尔载条件环 **-80.0%**（15.7→3.2ms）；负步数组环泛
  flavor **-90.8%**（12.1→1.1ms）；泛化下标环 **-92.5%**（14.7→1.1ms）。
  嵌套读形态回退前 +6.1~6.2%（TableLoad 逐迭代重读全守卫 ≈ 解释器快路
  成本 + 槽回写税，且同根因即 liveness 未定位面——回退后无暴露）。
- **旗标语义不变**：默认关 = 零扰动（结构未动任何解释器热路）。

**验收门**：rt 位一致 20/20 绿（新增 step 泛化三判别 + 条件跳转三判别 +
泛化下标/嵌套回退改造判别）；clippy 三 crate `-D warnings -W
clippy::absolute_paths` 零错；./test.sh 6616+112 全绿；A/B 三形 ≥5% 门全
达（-80%/-90.8%/-92.5%）。遗留：派生表装载 liveness 崩深因（GC 交互层，
完整实现留存 8d2fd4c1 供后续片恢复）；嵌套布尔载组合（micro_gettable 的
grid[i][j] 变体）随之待解；else-双臂区域（双区域写槽）与值产生型布尔
（LOP_LT/LE）未录。

### FORN trace 层 T5（2026-10-07，fork opt-trace）——派生表装载崩深因定谳翻案 + unlock；nbody/spectralnorm 剖面与多槽体解锁
T4 之上的第五片。A（派生表装载 unlock）达成且 **T4 的 GC/liveness 归因被
探针取证推翻**；B（剖面驱动推广）按判据如实入档。

- **T5-A 深因定谳（翻案）：崩因 = 派生驻留寄存器语义错位，非 GC/liveness
  架构问题**。重放 8d2fd4c1（TableRef/TableLoad 录制面 + 临时探针链）复现
  checkliveness 崩后逐层取证：①崩点 = 解释器 h_gettable 快路 set_obj，
  拷贝源 = `ts.array[r-1]`（探针：src−ts.array=+0x10，r=2）；②被拷值
  tt=7(Table) 但 gc 头全零（tt=0/marked=0）且 sweep 零 free 记录——对象被
  **原地砸烂**非回收；③R9（TableLoad 目标槽）的行表对象前 8 字节与
  `j*2+r=3.0` 的 f64 低半节逐字节吻合（tt/marked/memcat 全零）。根因 =
  TableLoad 驻 x7 的是行表**对象指针**，而派生 ArrayLoad/Store 寻址把 x7
  当 **array 指针**（少一次 `LuaTable.array` 间接）——读写整体偏移进对象
  自身：j=1 store 的值字覆盖 tt/marked/memcat 旗标区，j=2+ 覆 metatable/
  array/node；读侧吃垃圾数值即 T4 所记「嵌套布尔载读值漂移」的同根因。
  T4 的 GC 可达性/栈扫描漏扫/interrupt 三假设均不成立（trace 体无分配，
  GC 无法步进；槽在 [stack, L→top) 窗口内；行表经 ts 强可达）。
  - **修复**：驻留语义改 **array 指针**（寻址基与 Inv 槽同构），表对象
    指针仅 TableLoad 位点内经 X_SCRATCH（元素地址）瞬时转取——sizearray/
    metatable/readonly 读 + 槽回写（回写必须对象指针，解释器可见面是
    TValue）。守卫序：源界 → 元素 tag==ttable → 装载新表 sizearray/array/
    metatable → 元表缺席 →（存储穿透时）readonly → 槽回写。
  - **解锁验证**：嵌套读/写/临时下标 rt 判别恢复绿 + 新增 GC churn 判别
    （行表逐外轮重建驱动自然 GC 步进——环体无分配，GC 步进只落环间）+
    派生界外/元表 bail 判别（bailed 增量），rt 23/23 绿，compiled/executed
    增量实钉真实装机。
  - **A/B 如实入档（release 口径，med-of-7 AB 交替，位一致前提断言）**：
    嵌套读环原生 **~9.2ns/iter**（TableLoad 逐迭代重读全守卫：精确性往返
    +源界+tag+3 级依赖装载+槽回写 ≈ 19 指令 3 依赖 load）vs 融合解释器
    **~1.9ns/iter**（cache-resident 轻体）→ 端到端 **-479%**，unlock 未达
    +5% 预期收益。问询成本经 400k 入口微测排除（~0）。T4 所记回退前
    +6.1% 为 rt/debug 口径（解释器基线 3~4x 慢）。收益域 = 重体/超 L2 负载
    （bench matmul n=140 memory-bound 10.7ns/iter 口径下原生追平）与
    debug 基线。**杠杆留档**：TableLoad 入口提升（src 槽非环体存储目标时
    界/tag/驻留整体提至入口，环体归零）可压平至 flat 形态成本，但轻体
    微形状仍不敌融合解释器，未动手。
- **T5-B 剖面驱动（spectralnorm/nbody）**：两用例可测环逐源判定——
  - spectralnorm：内环 `sum += eval_a(i,j)*u[j+1]` 每迭代 1 CALL（441k
    iters、67ns/iter vs 纯算术 ~2ns → CALL 机器主导）；尾环（v_bv/vv 归
    化）形态全支持但 1 入口 × 210 回边双计永不达阈且工作量占比 ~0——
    **不支持族 = CALL 直译**，阈值下调论证拒绝（税面扩大换零收益）。
  - nbody：advance 内环 sqrt CALL（同不支持族）；位置环
    （`bi[k]=bi[k]+dt*bi[k+3]` ×3）**原被 K_MAX_TEMPS=8 容量拒录**（临时
    号按指令递增不复用，3 存多槽体需 12）→ 扩 d28..d31 解锁（探针链：
    elig=true → record 失败 → 环体转储 → 临时计数定谳）→ 装机实钉
    （compiled+1 / executed+59800）位一致绿；但 **5 迭代短环端到端
    -175%**（入口问询+调用 ~117ns/entry > 环体收益）——形态解锁达成、
    提速面如实为负。
  - **不支持族清单与扩宽成本**：①内建 CALL 特化（sqrt→fsqrt，callee
    身份判定 + 发射 + bail 面，中成本单片候选）；②通用 CALL 直译/叶子
    函数内联（跨 proto 录制基础设施，大）；③GETIMPORT 环体全局读（探针
    实测 hot 块 `ts[r]` 未局部化即整环拒录——低成本低成本扩宽候选）。

**验收门**：rt 位一致 23/23 绿（嵌套读恢复 + GC churn + 派生 bail +
multi_store_body 四判别）；clippy 三 crate `-D warnings -W
clippy::absolute_paths` 零错；./test.sh 6616+112 全绿。A/B ≥5 轮 med 按
门如实报告：A 项 unlock 未达 +5%（-479%，release 轻体口径 + 根因链入
档），B 项 nbody 位置环解锁但 -175%、spectralnorm 无 ≥5% 候选靶点。

### FORN trace 层 T6（2026-10-07，fork opt-trace）——GETIMPORT 环体读 + sqrt 内建特化 + TableLoad 提升杠杆评估（不做）
T5 之上的第五片。三族按独立验证门推进；族 1 的 patterns 解锁目标剖面修正，族 3 评估不做。

- **族 1 GETIMPORT 环体全局读（落地 + 剖面修正）**：支持集收录 GETIMPORT
  （双字）与 JUMPXEQKNIL（nil 常量比较跳转，tag 面独立判定）。GETIMPORT
  语义 = 安全双守卫提至入口（`safeenv != 0` 经 `ci.func→Closure.env` +
  k[D] 非 nil 缓存——环体无调用无赋值，两者跨环不变，入口一次判定覆盖
  全环；缓存未解析即拒承，解释器慢路解析写 k[D] 后续入口自然命中）+
  prologue 全 TValue 常量拷贝（k[D]→槽，位对齐解释器 fast-path
  `setobj(ra, kv)`，环内零代码）+ k[D] 位型（tt+value 合并 u128）进身份
  面（import 缓存改写即重录）。dst 槽不进 walk 已知值，表位/数值/真值
  各读面按槽自然路径解析。落点教训：拷贝段须置于 `fcvtzs w1` 之前（
  W_IDX_I=R_PROTO 同号，x1 在 fcvtzs 后即变 idx 整数表示——曾致
  `ldr [x1,#8]` 段错误，lldb pc 定位）。
- **族 1 剖面修正（诚实档案）**：patterns 主 chunk 的 FORN 环拒录链实
  测——GETIMPORT 收录后拒录点前移至 **LopCall**（环体 `string.format/
  find/match/gsub` 全 CALL）：**GETIMPORT 仅是环内首个不支持 op，真阻塞
  为 CALL**，patterns 解锁待「string.* 内建 CALL 面」（find/match/gsub
  为带捕获状态的重型内建，非单参单返回形态，成本远超本片）。GETIMPORT
  收录保留（正确扩宽，判别：GETIMPORT+NIL+AccArith 组合环，沙箱 safeenv
  + 嵌套驱动，位一致 + compiled/executed 双增量 ✓）。
- **族 2 sqrt 内建 CALL 特化（落地，双形态）**：① CALL 形态——func 槽
  经 **SqrtFn TVal 面**（录制期按帧实测「is_c 闭包且 inner.c.f ==
  math_sqrt」标定槽，MOVE 转发链自然传递，入口守卫按槽复检防跨入口重赋
  值）；② **FASTCALL1 形态**（nbody 实际形态——`sqrt(dist2)` 编译为
  FASTCALL1，A=bfid 静态比对 `LbfMathSqrt`，尾随 CALL 字载返回槽/返回
  数，走查跳过其执行已被 fast call 消化）。发射 = fsqrt 直译（IEEE 正确
  舍入逐位对齐 f64::sqrt），参数/返回皆 number 特化面（非 number 参数在
  解释器同执行路径先抛 check_number，特化环无守卫面）。fn 地址 usize
  对账（全仓单一定义，绕 fn_address_comparisons lint 面）。
- **A/B 配对（rt 级，≥5 轮 med，AB 交替，位一致前提断言）**：sqrt 环
  （nbody 内环形态）**-94.7%**（25.2→1.3ms）✓；GETIMPORT+NIL 环
  **+2.2%**（10.2→10.5ms）——无现网靶点的新能力形态（patterns 待 CALL
  面），环体仅 3 op、原生 dispatch 收益薄，+2.2% 在 rt 噪声带内（T3
  口径 gettable 系旗标开税 +5.5% 背景），如实入档。
- **族 3 TableLoad 入口提升（评估后不做）**：T5 定谳嵌套读原生
  9.2ns/iter vs dev 融合解释器 1.9ns（整型键直通轻体快路）。提升的可达
  终态 = 入口段守卫链 + proven 面扩至派生表后，稳态内层 ≈ 寻址+load 2
  指令 + fadd + 回边 ≈ 2ns——**与融合解释器打平，无端到端正收益空间**
  （原生无 dispatch 优势可兑现，解释器已 cache-resident）。实现成本
  （入口段拆发射/无 store 判据/回写时点）> 收益，按纪律不做、入档封存。
- **旗标语义不变**：默认关 = 零扰动。

**验收门**：rt 位一致 25/25 绿（新增 GETIMPORT+NIL 组合判别 + sqrt CALL
环判别）；clippy 三 crate `-D warnings -W clippy::absolute_paths` 零错；
./test.sh 6616+112 全绿；A/B：sqrt -94.7% 达门，GETIMPORT +2.2% 无靶点
吸收论证。遗留：patterns 解锁待 string.* 内建 CALL 面（重型）；TableLoad
提升封存（打平论证在案，若 dev 快路面退化可重启）。

### FORN trace 层 T7（2026-10-07，fork opt-trace）——math 单参内建族泛化（abs/floor/ceil/round）+ 派生表写形态判别封口
T6 之上的第七片。任务 1 = sqrt 先例按族复制推广；任务 2 = T5-A 遗留写半边的语义复核与判别封口。

- **任务 1 族清单核账**：LBF_MATH_* 扫描定选型——abs/floor/ceil/round
  四族落地（`LbfMathRound` 存在）；**exp 存在但无单指令直译面**（a64 无
  exp 指令，直译需 libm call 破叶函数无调用约束，如实排除）；**neg 不在
  LBF_MATH_***（LOP_NEG 是算术 op 族非内建，另一条扩宽线，不混本族）。
  round 的发射指令**选型纠偏**：任务猜的 frintn 是 ties-even，与
  `f64::round` 的 ties-away 规格不符——核对 Rust std 语义后定 **frinta**
  （最近邻 ties-away，汇编器 r1_table 方法全数已存在，fabs/frintm/frintp/
  frinta 复用零新增）。
- **实现形态（sqrt 先例直接泛化）**：`TMathUnary` 种（Sqrt/Abs/Floor/
  Ceil/Round）替代 TInst::Sqrt 单形；`TVal::Cfn{slot,kind}` 替代
  SqrtFn（录制期按帧实测 `inner.c.f` 地址对账标定槽→种，
  trace_forn_math_addr 扩为五函数对账表，math_ceil 提 pub 与同族一致）；
  FASTCALL1 bfid 静态比对扩为五内建映射（经 `from_id` 校验转换）；入口
  守卫 `sqrt_guards: Vec<u8>` → `math_fn_guards: Vec<(u8, kind)>`（按
  (槽, 种) 复检防跨内建重赋值错特化）。位一致构造性：IEEE roundToIntegral
  族/abs 与 f64 方法语义规格逐一对应（有限输入结果唯一，NaN/inf/-0.0 逐位
  传递）；非 number 参数在解释器同执行路径先抛 check_number（math_map1
  骨架 `check_number(1)`），特化环参数面皆 number 已知值无守卫面——
  exp 同此论证但无发射面，待后续 libm-call 形态单片评估。
- **任务 2 派生表写形态封口**：T5-A 驻留语义修复（x7=array 指针）后写
  形态（TableStore{Derived}）已随嵌套读写判别恢复绿——本片补语义复核与
  定向判别收口：①屏障面——`luaC_barriert` 谓词 `iscollectable(v)` 对
  number 恒假 → 屏障零动作，与解释器 `fuse_succ_settable` 直写快路同款
  iscollectable 门控（无需发射屏障，**无屏障参与构造性入档**）；②
  `invalidate_tmcache` 只缓存元方法判定（派生表元表缺席被 TableLoad 守卫
  逐迭代钉死，缓存无意义）；③`index_chain_write` 只对字符串键存在性写
  敏感（number 特化键 + 无元表不入链缓存）；④界守卫限制 array 段不触
  hash/rehash。新增判别：行表元素先持 `{}`（gc 指针）再由派生写以 number
  覆盖 + churn 驱动 GC 步进 + 读/写/复读环间混合——T4 时代炸
  checkliveness 的病灶面两态不分化。**单环内同派生表「值读+写」混合仍
  结构性拒录**（派生槽值读 = number 特化 bail 风暴红线，环间混合已覆盖）。
- **A/B 配对（rt 级，≥5 轮 med，AB 交替，位一致前提断言）**：floor 环
  微负载（`floor(i*0.75) + floor(i-13)` 双调用形态）**-97.7%**
  （1473.3→34.0ms）✓ 远超 ≥5% 门（CALL/FASTCALL 消灭 + dispatch 免除，
  debug 解释器基线口径）；**release 复测 -89.8%**（101.4→10.3ms）——
  融合解释器基线下仍大幅达门，与 T5 嵌套读反转面（-479%）不同类：内建
  CALL 消灭是解释器不可融合的真杠杆（每迭代 C 调用帧 + check_number
  往返为原生单指令所代）。
- **旗标语义不变**：默认关 = 零扰动。

**验收门**：rt 位一致 30/30 绿（新增四族 FASTCALL1 环 + CALL 参数槽环
（含跨内建种入口拒承）+ 非 number bail 错误一致面 + 派生写覆盖 gc 对象
churn 环四判别）；clippy 三 crate `-D warnings -W clippy::absolute_paths`
零错；./test.sh 全绿；A/B：floor 环 -97.7% 达门。遗留：exp/二参内建
（fmod/pow/max/min）待 libm-call/双参形态扩宽；rad/deg（fmul 常数对）
与 LOP_NEG（算术 op）不在本族范围；patterns 仍待 string.* CALL 面。

### FORN trace 层 T8（2026-10-07，fork opt-trace）——GETUPVAL/SETUPVAL upvalue 读写 + 推广验证定谳（nbody/spectralnorm 资格探针）
T7 之上的第八片。A（upvalue 面）落地；B（nbody/spectralnorm）资格探针定谳，
「若现可录制」条件不成立如实入档。

- **A 项 upvalue 读写（落地）**：GETUPVAL/SETUPVAL 进支持集，upref 三形态
  分态（upref = `Closure.uprefs[B]`，柔性数组）——
  - **变更载体**（体含 SETUPVAL，LCT_REF 强制捕获）：cell 指针
    （`(*UpVal).v`）驻 x14/x15（X_PC/X_CI 同号——二者仅出口/bail 块的
    savedpc 落位使用，环体内空闲），逐访问穿 cell 读（tag==tnumber 守卫
    bail，类别 6）与写（值 + tnumber tag 双写）。跨迭代累加经 cell 传递
    （SETUPVAL 写 → 下一迭代 GETUPVAL 读），同迭代读后写见新值——upvalue
    本身即跨回边载体，槽面为临时链，无 phi 依赖。
  - **只读 Num 载体**：prologue 内联保留临时，环内零代码。LCT_VAL 值内联
    upref（捕获后不再变，克隆时常量）直载 upref 本体；真 UpVal 经
    `(*uv).v` 一级间接。
  - **只读 Cfn 载体**（nbody `sqrt(dist2)` 的 GETUPVAL+CALL 形态）：环内
    零代码，值经 fn_slots 身份被 CALL 特化消费；GETUPVAL 槽写为虚写
    （prescan 剔除——防区域分析按「双无条件写」拒录；CALL 臂写面承载，
    同解释器退出态）。
  - **语义对齐点**：①open/closed 双态——`luaF_close` 只发生于调用/返回/
    CLOSEUPVAL（均不在环体内），环内态不可变；态迁移在入口间由「逐入口
    重解 cell」自然消费，open（cell→栈槽）/closed（cell→storage）统一同
    一指针语义；双态守卫 = upref 形态复核（`is_upval` 失配拒承，prologue
    生成码形态按录制现场锁定）。②SETUPVAL 屏障——`luaC_barriert` 谓词
    iscollectable(v) 对 number 恒假 → 构造性零动作（解释器 h_setupval
    同值面同零动作，T7 派生写同款论证）。③bail 前向单调——bail 点之前的
    SETUPVAL 已穿 cell 生效，与解释器从 bail 位点续延的可见面逐位一致
    （upvalue 写不可快照亦无需快照——外部效果无部分副作用）。④只读内联
    的槽写走快照写回面（入口守卫钉 cell tnumber 后环内不变）。⑤Cfn 载体
    真值源是 cell 不是槽（槽值是上次 GETUPVAL 的脏快照）——入口守卫按
    cell 复检 (槽, 种) 之外加按 cell 源复检种，防环外重绑定错特化。⑥
    算术操作数 Cfn 拒录一并封口（函数值进算术位在解释器是抛错面，T8 起
    fn 槽暴露面变宽）。
  - **连带硬化**：FMA 折叠供体 Mul 的槽写回在供体寄存器无定义写时落垃圾
    （T2 起潜伏；槽为编译器临时、环外死值、tnumber tag 恶性有限）——
    写回面跳过 folded 供体，死槽保持入口值，消除未定义寄存器读。
- **B 项推广验证（资格探针定谳，如实入档）**：临时探针链（eprintln，提交
  前清零）钉死三环拒录点——
  - nbody advance j 环（sqrt 内环）：T8-A 解除 GETUPVAL 拒录点后，实测
    拒录点 = **环体长度上限**（45 条 > K_MAX_TRACE_INSTS+2 = 34；T6 的
    sqrt 特化与 T8 的 GETUPVAL 均已具备）；energy j 环 62 条同因。「若
    现可录制」条件不成立 → 无端到端 A/B。剩余阻塞结构性：临时容量 ~19
    > 12（d8..d15 为 callee-saved 规避域，无 prologue 不可用），解锁待
    SSA 临时重命名（T3 已留档）或 prologue 化，单片不解。
  - nbody advance 位置环（T5 rt 判别形态可录）：真 nbody 字节码中 `b` 为
    表载体 upvalue → GETUPVAL 落槽后作表源被 intern_table 拒录（upvalue
    值面 number 特化；表载体 upvalue 是另一族扩宽）。且该环 5 迭代短环
    T5 已实测端到端 -175%，解锁无 A/B 价值，按纪律不扩。
  - spectralnorm 内环：`eval_a(i, j)` 为 Lua 闭包 CALL（非 math 内建）→
    CALL 臂 Cfn 判定拒录（预期，CALL 面留档不变）。
- **rt 判别（4 新增，34/34 绿）**：①变更载体累加 **open/closed 双态同
  trace**（开态段装机原生——mk 帧存活 cell 指向栈槽；闭态段 mk 返回
  luaF_close 升级 storage 后同 trace 承接）+ bump 解释器写同 cell（trace
  内外读写混合）+ 每轮 churn 分配驱动 GC 步进，手算期望 21418100.0 钉死
  累加真发生（防读捕获陈值的两态一致假绿）；②只读载体两形态（LCT_VAL
  直载 / LCT_REF 一级间接）位一致 + compiled/executed 双增量；③变更载体
  tag bail（热段装机后环外重赋字符串 → UpvalLoad 守卫类别 6 bail → 解释
  器 ADD 抛字符串算术错）两态错误一致 + bailed 增量；④A/B。
- **A/B 配对（rt 级，≥5 轮 med，AB 交替，位一致前提断言）**：upvalue 累加
  环（回边面装机，单入口 × 60000 迭代 × 60 轮）**-95.1%**（585.6→28.8ms）
  ✓ 远超 ≥5% 门——cell 双载（逐迭代读+写穿 cell，共 5 指令）仍大幅快于
  解释器 GETUPVAL/SETUPVAL 臂全路径（dispatch 免除主导）。
- **旗标语义不变**：默认关 = 零扰动。

**验收门**：rt 位一致 34/34 绿（新增 upvalue 四判别）；clippy 三 crate
`-D warnings -W clippy::absolute_paths` 零错；./test.sh 6616+112 全绿；
A/B upvalue 累加环 -95.1% 达门；B 项「若现可录制」条件不成立如实入档
（无 A/B 硬凑）。遗留：环体长度/临时容量结构性上限（nbody sqrt 内环解锁
待 SSA 临时重命名或 prologue 化）；表载体 upvalue（GETUPVAL→表源）未录；
exp/二参内建与 string.* CALL 面（T6/T7 遗留）不变。

### FORN trace 层 T8b（2026-10-07，合并修复）——Cfn 载体死轨修复：FASTCALL1 影子分类

T8 与超环融合（04360ffe）语义合并后审查揪出 P1：Cfn 载体分态零端到端覆盖。
插桩定谳，字节码实况推翻审查建议的修法（首现消费 fn_slots 不足）——

- **形态真相**：`local sqrt = math.sqrt` 后环内 `sqrt(x)`，Luau 编译器对
  upvalue 载体内建同样发 **FASTCALL1**（bfid 静态）——字节码序
  `FASTCALL1(8) → GETUPVAL(9) → CALL(10)`：GETUPVAL 是 fastcall 失败
  回退面的函数装载，热路被 FASTCALL1 跳转吞掉**永不执行**，尾随 CALL 字
  同被消化（「永不执行」仅 Sqrt 成立——本移植 F 表只装 LbfMathSqrt，
  Abs/Floor/Ceil/Round 落 luau_f_missing 恒 -1，解释器每迭代实走回退面
  执行影子装载；fn_slots 失明的真因是 CALL 结果逐迭代覆写 func 槽，
  两路皆然，影子分类按 bfid 静态给出，两路同样正确）。func 槽录制期快照恒为上一轮结果数字 → fn_slots 对此形态
  **结构性失明**（T8 原分类依赖 fn_slots，Cfn 轨全形态不可达：影子
  GETUPVAL 落 Num → 装机即被只读 Num 入口守卫 tnumber 永拒承 = 死轨；
  T8 语义注 ⑤「真值源是 cell 不是槽」已自证矛盾）。
- **修复**：prescan 录 FASTCALL1 影子区 `(fc_pc, call_pc, bfid 种)`；
  影子内 GETUPVAL 分类 Cfn 按 bfid 静态种（编译器内建追踪构造性成立），
  环内零代码 + prologue 零装载 + GETUPVAL 槽写照旧虚写剔除；语义锚不变
  ——入口守卫按真值源（cell/upref 本体）复检种，防环外重绑定后解释器走
  CALL 回退而 trace 仍算旧内建的分叉。fn_slots 槽快照分类保留为影子外
  回退（GETIMPORT 局部形态）。
- **连带**：codegen 只读 Num `inv_temp=None` 不可达臂补 `debug_assert!`
  ——正是该 tripwire 当场抓获本死轨（静默空发射转 debug 态显炸）。
- **rt 判别（1 新增，35/35 绿）**：Cfn 载体单次 GETUPVAL 端到端——位一致
  + compiled/executed 双增量（钉死轨不复现：装机即执行，非假绿）。

**验收门**：rt 35/35 绿；clippy `-D warnings -W clippy::absolute_paths`
净（含 nightly lint 面变化连带：2 处既有 `core::ptr::addr_of!` 绝对路径
按 `./sh/clippy.sh` 自修导入 `addr_of`）；fmt 净。复审（独立子代理）六
核对面全过、无 P0/P1。

## 明确不做

- corosensei 切栈协程（模型不等价且对本基准更慢）
- cranelift/inkwell 后端、dynasm-rs 迁移（现有手写 assembler 已工作）
- patchouly/copy-and-patch（nightly 依赖 + 定位是 baseline tier 不是优化 tier）

## 实测记录

- 2026-10-07：T8b 后全组配对 A/B 量化（bench.sh 双后端全组，engine-luajit
  exec 组 24 用例 × 5 引擎，每项交替采样 5 轮取最小，M2 Max；`--fflag
  LuauJitFornTrace` on/off 两轮同参，off 轮 LuaJIT 两列与 on 轮逐用例
  ±1% 稳定——测量系可信）——
  - **纯解释器（trace off）vs LuaJIT interp geomean = 1.032**：慢 3.2%；
    反超 6 例（life 0.52 / tablesort 0.50 / micro_gettable 0.42 /
    microbig_gettable 0.41 / spectralnorm 0.82 / micro_arith 0.68）；
    拖后 nsieve 3.0 / coroutines 1.9 / fib 1.5 / nbody 1.37。
  - **method JIT（trace off）vs LuaJIT JIT geomean = 2.670**：与上轮
    2.658 一致无回退。结构性大差 = inherit3 38.7 / oop 13.8 /
    spectralnorm 7.1 / micro_call 5.3 / nsieve 5.4（LuaJIT 全函数
    trace 编译 vs 本方环级 method JIT 形态差）；life 0.74 / tablesort
    0.50 反超。
  - **trace 层（旗标开 jit 关）vs LuaJIT interp geomean = 0.837**：
    trace 执行面快 16%——matmul 0.24 / microbig_gettable 0.15 /
    micro_gettable 0.17 / micro_arraywrite 0.22。注意口径：此为
    FORN trace 独立形态（jit 开关无关），非「解释模式」。
  - **配对 A/B（jit 列 on/off）geomean = 1.019 净负**：FORN trace 叠在
    method JIT 之上净付税 1.9%——bench 数值环多已被 method JIT 覆盖，
    问询/守卫税无收益可摊（nbody 不可录环纯税 +3.3%）。trace 层目标
    姿态 = 独立形态（上项 0.837），非 jit 列叠加项。
  - T8b 修复（Cfn 影子环）对 bench 全组零影响（预期：nbody sqrt 内环
    45>34 拒录，bench 无 upvalue 载体内建环负载），A/B 配对差在噪音内。

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
