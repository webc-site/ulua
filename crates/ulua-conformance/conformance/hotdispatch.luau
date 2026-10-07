-- 派发环 × 臂尾融合定向 conformance：本仓新增，cpp/tests/conformance 无对应源。
--
-- 覆盖对象是 ulua-vm 派发环里被 fuse_succ_* 融合的热边所在的臂（见 crates/ulua-vm/
-- src/functions/luau_execute.rs）：LOP_JUMP / JUMPIF / JUMPIFNOT / JUMPBACK / FORNPREP /
-- FORNLOOP / GETTABLE / SETTABLE / GETTABLEN / SETTABLEN / NEWTABLE / ADD / SUB / MUL /
-- ADDK / SUBK / MULK / MODK / LOAD* / MOVE / GETUPVAL / SETUPVAL。
--
-- 四件事逐条钉住：
--   1. 融合臂内部抛错的展开：本仓用 panic 模拟 longjmp（install_lua_exception_panic_hook），
--      它必须能穿过内联的 handler 与融合链被外层 pcall 接住；
--   2. 抛错后状态完好：同一 state 继续跑热循环，结果不受污染（savedpc/base 回写纪律）；
--   3. 协程在融合臂内 yield/resume，跨臂的栈帧交接与错误传播正确；
--   4. 条件跳转的「真分支双尾」不改变语义：回边/退出、零次循环、负步长、NaN 边界都要对。
--
-- 断言在解释器与原生（LUAU_CODEGEN=1）两种模式下必须一致成立；钩子相关的段落拆到
-- hotdispatch_hook.luau（该 fixture 以 skipCodegen 运行，只走解释器派发环）。

--- 热循环基准：FORNPREP/FORNLOOP/ADDK/LOADN/JUMPBACK 一条链，返回值用于「抛错后状态自检」。
local function hot_sum(n)
  local s = 0
  for i = 1, n do
    s += i
  end
  return s
end

do
  -- 1. FORNLOOP 回边内 error：必须由外层 pcall 接住，且循环只跑到抛错那一圈
  local hits = 0
  local function boom(i)
    hits += 1
    error("boom@" .. tostring(i))
  end
  local ok, msg = pcall(function()
    for i = 1, 50 do
      boom(i)
    end
  end)
  assert(not ok, "error inside a numeric for must propagate")
  assert(hits == 1, "loop must stop at the first raise, hits=" .. tostring(hits))
  assert(type(msg) == "string" and msg:find("boom@1", 1, true), tostring(msg))
  -- 2. 抛错后同 state 的热循环仍正确
  assert(hot_sum(10) == 55, "state must stay consistent after a FORNLOOP error")
end

do
  -- 1'. while（JUMPIFNOT + JUMPBACK）内 assert(false)
  local n = 0
  local ok, msg = pcall(function()
    while true do
      n += 1
      if n == 7 then
        assert(false, "inner assert")
      end
    end
  end)
  assert(not ok, "assert(false) inside a while loop must propagate")
  assert(n == 7, "loop counter at raise should be 7, got " .. tostring(n))
  assert(msg:find("inner", 1, true), tostring(msg))
  assert(hot_sum(5) == 15)
end

do
  -- 1''. 元表 __index / __newindex 抛错，键在寄存器里 —— 强制 LOP_GETTABLE / LOP_SETTABLE
  -- 的通用臂（常量键会降级为 GETTABLEKS/GETTABLEN，不走这条路径）
  local keys = { "a", "b", "c" }
  local proxy = setmetatable({}, {
    __index = function(_, k)
      error("no field " .. tostring(k))
    end,
    __newindex = function(_, k)
      error("no write " .. tostring(k))
    end,
  })

  local read
  local ok, msg = pcall(function()
    for i = 1, 3 do
      read = proxy[keys[i]]
    end
  end)
  assert(not ok, "__index error inside a hot loop must propagate")
  assert(read == nil, "read must stay nil, got " .. tostring(read))
  assert(msg:find("no field a", 1, true), tostring(msg))

  local ok2, msg2 = pcall(function()
    for i = 1, 3 do
      proxy[keys[i]] = i
    end
  end)
  assert(not ok2, "__newindex error inside a hot loop must propagate")
  assert(msg2:find("no write a", 1, true), tostring(msg2))
  assert(next(proxy) == nil, "failed writes must not land in the table")
  assert(hot_sum(3) == 6)
end

do
  -- 3. 递归穿过热臂后再在深处抛错：跨多帧 become 链展开 + 展开后栈可用
  local calls = 0
  local function rec(d)
    calls += 1
    if d == 0 then
      error("deep")
    end
    local acc = 0
    for i = 1, 3 do
      acc = acc + rec(d - 1)
    end
    return acc
  end
  local ok, msg = pcall(rec, 40)
  assert(not ok and msg:find("deep", 1, true), tostring(msg))
  assert(calls == 41, "each depth raises exactly once, calls=" .. tostring(calls))
  assert(hot_sum(20) == 210)
  -- 展开后 GC / 栈仍能正常推进
  local t = {}
  for i = 1, 200 do
    t[i] = { i }
  end
  assert(#t == 200 and t[137][1] == 137)
end

do
  -- 4. FORNLOOP 的边界形状：零次循环 / 负步长 / 浮点步长 / NaN 与 ±inf 界限
  local runs = 0
  for _ = 1, 0 do
    runs += 1
  end
  assert(runs == 0, "for i=1,0 must not run")

  local desc = {}
  for i = 10, 1, -1 do
    desc[#desc + 1] = i
  end
  assert(#desc == 10 and desc[1] == 10 and desc[10] == 1)

  local half = 0
  for _ = 1, 2, 0.5 do
    half += 1
  end
  assert(half == 3, "step 0.5 over 1..2 runs 3 times, got " .. tostring(half))

  for _ = 1, 0 / 0 do
    error("NaN limit must not run")
  end
  local huge_hits = 0
  for i = 1, math.huge do
    huge_hits += 1
    if i == 5 then
      break
    end
  end
  assert(huge_hits == 5, "break out of an unbounded for must exit the backedge")

  -- 嵌套回边：外层 SETTABLEN/GETTABLEN，内层 MODK/ADD
  local sum = 0
  for i = 1, 6 do
    for j = 1, 6 do
      sum = (sum + i * j) % 1000003
    end
  end
  assert(sum == (6 * 6 * 7 * 7 / 4) % 1000003, "nested loop checksum, got " .. tostring(sum))
end

do
  -- 5. 协程在热臂内 yield：数字 for、while、以及 __index 里 yield（跨 GETTABLE 臂续跑）
  local co = coroutine.create(function()
    for i = 1, 5 do
      coroutine.yield(i * 2)
    end
    local n = 0
    while n < 3 do
      n += 1
      coroutine.yield("w" .. tostring(n))
    end
    return n
  end)
  local expect = { 2, 4, 6, 8, 10, "w1", "w2", "w3" }
  for k = 1, #expect do
    local ok, v = coroutine.resume(co)
    assert(ok, tostring(v))
    assert(v == expect[k], "yield #" .. tostring(k) .. " = " .. tostring(v))
    assert(coroutine.status(co) == "suspended")
  end
  local ok, v = coroutine.resume(co)
  assert(ok and v == 3, "final resume returns the loop counter, got " .. tostring(v))
  assert(coroutine.status(co) == "dead")

  -- 协程体内抛错：错误由 resume 返回 false，不逃逸到外层
  local bad = coroutine.create(function()
    local acc = 0
    for i = 1, 20 do
      acc = acc + i
      if i == 3 then
        error("in-coroutine " .. tostring(acc))
      end
    end
    return acc
  end)
  local ok1, err1 = coroutine.resume(bad)
  assert(not ok1 and tostring(err1):find("in-coroutine 6", 1, true), tostring(err1))
  assert(coroutine.status(bad) == "dead")
  assert(hot_sum(6) == 21)

  -- 元方法本身不可 yield（cpp `ldo.cpp:855` 同款拒绝：attempt to yield across
  -- metamethod/C-call boundary），这里只测跨 GETTABLE 臂的元表读：每次未命中都要真的
  -- 走到 __index，缓存后不再进元表。
  local reads = 0
  local lazy = setmetatable({}, {
    __index = function(t, k)
      reads += 1
      local v = "fed@" .. tostring(k)
      rawset(t, tostring(k), v)
      return v
    end,
  })
  local keys = { "x", "y" }
  local out = {}
  for pass = 1, 2 do
    for i = 1, 2 do
      out[pass * 2 + i] = lazy[keys[i]]
    end
  end
  assert(reads == 2, "cached reads must not re-enter __index, reads=" .. tostring(reads))
  assert(out[3] == "fed@x" and out[4] == "fed@y" and out[5] == "fed@x" and out[6] == "fed@y")
  assert(rawget(lazy, "x") == "fed@x" and rawget(lazy, "y") == "fed@y")
end

do
  -- 6. 一条覆盖多数热臂的「形状」基准：NEWTABLE/LOADK/LOADNIL/LOADB/MOVE/GETUPVAL/
  -- SETUPVAL/ADD/SUB/MUL/ADDK/SUBK/MULK/MODK/GETTABLEN/SETTABLEN/JUMPIF*
  local up = 0
  local function bump()
    up = up + 1
    return up
  end
  local t = {}
  local flag = false
  for i = 1, 40 do
    t[i] = i * 2 - 1
    if t[1] == 1 and (i % 4) == 0 then
      flag = true
      bump()
    end
  end
  assert(flag and up == 10, "upvalue counter, up=" .. tostring(up))
  assert(t[40] == 79 and #t == 40)
  local q = {}
  for i = 1, 20 do
    q[i] = { v = i, m = i % 7 }
  end
  local acc = 0
  for i = 1, 20 do
    local e = q[i]
    acc = acc + e.v - e.m * 0 -- SUBK/MULK 形状
  end
  assert(acc == 210, "aggregate, acc=" .. tostring(acc))
  assert(hot_sum(40) == 820)
end

return "OK"
