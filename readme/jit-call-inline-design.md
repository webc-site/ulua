# JIT 收敛 LuaJIT 差距：两主攻方向选型（案头归因）

官方口径（2026-10-03T02:16:59Z，253b6200 后）：JIT geomean 2.244（excl inherit3）。
差距按负载形态分两簇，各自对应一个主攻方向。

## 差距量化与归因

| 簇 | 代表负载 | 比值 | 差距本质 |
|---|---|---|---|
| call-heavy | inherit3 59.5x / oop 14.3x / fib(JIT) 3.1x | 极大 | per-proto 编译无跨函数内联；每虚调用付全帧建立/返回。LuaJIT trace 把循环体+全部被调体录成常量偏移单 trace 回放（inherit3 全程 0.4ms） |
| float-heavy | spectralnorm 6.45x / nbody 3.6x | 大 | 热循环内数值全程 TValue 装箱（16B tag+value 过内存）；LuaJIT trace 内浮点驻 XMM 寄存器不装箱 |

## 现有基建（案头核实）

- `apply_builtin_call.rs`：builtin 内联覆盖已广（math/bit32/string/table/vector/buffer 全家）——builtin 覆盖不是缺口。
- J2 CALL 快路（8ce356de 合入）：CALL 指令本身的派发快路，不内联被调体。
- J4-real 两跳 __index 内联 + 多点链缓存（14a52a3f 合入）：fallback 咽喉 `luaV_gettable` 双引擎受益。
- `const_prop_in_{inst,block,block_chain,block_chains,fallback}.rs`：常量传播家族，是 float 提升的现成载体。
- TSFB 站点观测（type_feedback.rs）：可为内联决策提供站点级类型/热度证据。

## 方向 A：用户函数 call inlining（主攻 call-heavy）

callee proto 体级内联进调用方 IR。设计要点：

1. **内联判据**：callee proto 常量可辨（NAMECALL/CALL 的 callee 槽在该站点恒定，TSFB 可证）、指令数上限（起步 ~100 insn）、非变参、非 `...` 传递、返回值数 ≤ caller 期望。
2. **递归防线**：内联深度栈（proto 链上已现即拒），互递归天然被深度上限挡住。
3. **参数直通**：实参 TValue 从 caller 栈位重映射为 callee 形参槽；常数实参可被 const_prop 直接吃掉（类型特化副产品）。
4. **返回直通**：单返回点 callee 可跳过帧；多返回点（return a,b 形态）按 CALL nresults 折叠。
5. **deopt/溢出路径**：callee 内任何慢路 fallback 必须能落到全帧语义——起步策略：callee 内出现任何 Fallback*/EXIT 需求即整体内联失败回退现路径（保守零风险），后续再优化为局部退出。
6. **GC safepoint**：内联体不新增分配点（TValue 搬运不分配），分配仍发生在 callee 未内联时的相同原语内，风险低；需在 review 中逐点核对 barrier。

预期：inherit3 的 4 个 3~5 行方法全可内联，循环体变纯算术+表读，收一轮 trace 常量偏移级别的收益（量级 2~5x 而非 59x——LuaJIT 还叠加循环整体 trace 化）。

## 方向 B：float 去装箱提升（主攻 float-heavy）

热点块内已证 number 的槽读写提升为未装箱 double：

1. **float 范围分析**：TSFB/字节码类型（LBC_TYPE_NUMBER）已证 number 的寄存器，块内 LoadTag+CheckTag 链外提出块。
2. **表槽直取**：`t[i]` 数字键读在 CheckSlotMatch 后已是节点 TValue——提升为 LoadDouble 直取 value 字段，tag 静态断言（节点值被写侧 barrier 保护为 number 的证据链须闭环，否则加运行时 tag 守卫）。
3. **算术链去装箱**：ADD/SUB/MUL 现为解箱→算→装箱三段；块内连续浮点算术折叠为 XMM 链，只在跨块边界装箱。
4. 载体：const_prop 家族扩展 `FloatRange`/`NumberProven` 格子。

预期：spectralnorm/nbody 收 20~40% 量级（装箱消除不等价 trace 化，但可收敛一大部分）。

## 排序与风险

先 B 后 A：B 增量小、风险低、载体现成；A 是架构级增量（IR 体积、deopt 复杂度），
建议 A 拆「判据+直通（单返回点零分支 callee）」最小可证子集起步。

红线照旧：语义逐格一致（conformance 全绿）、CPU 配对 AB/BA ≥15 对、全 16 用例回归 ≤2%、
wasm 双 target 0 错、证伪如实还原。
