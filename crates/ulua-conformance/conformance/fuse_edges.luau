-- 派发环臂尾融合（fuse_succ_*）定向 conformance：本仓新增，cpp/tests/conformance 无对应源。
--
-- 被测对象是被融进前驱臂尾巴的那几条热边的**未命中回退**：融合只吃数字快路，操作数一旦
-- 不是数字（字符串、表、越界索引）就必须原样回环头重走，副作用一次不能多、一次不能少。
-- 覆盖的热边：LOADN→JUMPIFNOTLT、GETUPVAL→SUBK、GETTABLE→MUL、MUL→ADD、MULK→ADD、
-- LOADB→SETTABLE、ADD→SETTABLE、FORNLOOP/ADDK→GETTABLE、JUMPIFNOT→ADDK。
--
-- 另钉两件事：融合链上抛错时 panic 模拟的 longjmp 能被外层 pcall 接住；协程在融合臂中间
-- yield/resume 后状态自洽。解释器与原生（LUAU_CODEGEN=1）两种模式跑同一批断言。

local function numEq(got: number, want: number, tag: string)
  assert(got == want, tag .. ", got " .. tostring(got))
end

local function errEq(ok: boolean, tag: string)
  assert(not ok, tag .. ", expected an error")
end

-- 1) LOADN → JUMPIFNOTLT 的未命中回退：字符串比较走环里的原臂
local function cmpStr(x)
  local y = 0
  for i = 1, 6 do
    if 3 < x then
      y = y + 2
    else
      y = y + i
    end
  end
  return y
end

-- 数与字符串 tag 不同，luaV_lessthan 直接 order error；融合臂必须原样回环重走
errEq(pcall(cmpStr, "z"), "number < string is an order error")
errEq(pcall(cmpStr, setmetatable({}, {})), "number < table is an order error")
-- 同一函数走数字操作数：3 < 4 恒真，6 圈各 +2
numEq(cmpStr(4), 12, "fused edge on a number operand")

-- 2) 同一条边的元方法路：两侧同为 table 时按 luaV_lessthan 调 __lt，一圈恰好一次
local ltCount = 0
local ltTrue = setmetatable({}, {
  __lt = function(_, _)
    ltCount = ltCount + 1
    return true
  end,
})
local function cmpTwo(a, b)
  local y = 0
  for _ = 1, 5 do
    if a < b then
      y = y + 1
    end
  end
  return y
end
ltCount = 0
numEq(cmpTwo(ltTrue, ltTrue), 5, "table < table through __lt")
numEq(ltCount, 5, "__lt must run exactly once per iteration")

-- 3) __lt 里抛错：必须由外层 pcall 接住，且只在第一圈调用
local raiseCount = 0
local ltBoom = setmetatable({}, {
  __lt = function(_, _)
    raiseCount = raiseCount + 1
    error("lt-boom")
  end,
})
local okBoom, msgBoom = pcall(cmpTwo, ltBoom, ltBoom)
assert(not okBoom, "__lt raising on the fused edge must propagate")
assert(type(msgBoom) == "string" and string.find(msgBoom, "lt%-boom", 1) ~= nil, tostring(msgBoom))
numEq(raiseCount, 1, "the loop must stop at the first raise")

-- 4) GETUPVAL → SUBK：upvalue 是字符串时快路必须让位
local strUp = "hello"
local function subStr()
  local t = 0
  for i = 1, 4 do
    t = t + (strUp - 1)
  end
  return t
end
errEq(pcall(subStr), "string upvalue minus a number")

-- 5) GETUPVAL → SUBK 走 __sub：每圈调一次元方法，融合不得把它算两遍
local subCount = 0
local subObj = setmetatable({}, {
  __sub = function(_, b)
    subCount = subCount + 1
    return b * 3
  end,
})
local function subMt()
  local t = 0
  for i = 1, 4 do
    t = t + (subObj - 2)
  end
  return t
end
subCount = 0
numEq(subMt(), 24, "__sub on the fused SUBK edge")
numEq(subCount, 4, "__sub must run once per iteration")

-- 6) GETTABLE → MUL：__index 返回非数字 / 数字两条路
local idxCount = 0
local idx = setmetatable({}, {
  __index = function(_, _)
    idxCount = idxCount + 1
    return "notnum"
  end,
})
local function gtmul()
  local s = 0
  for i = 1, 4 do
    s = s + idx["k"] * 2
  end
  return s
end
idxCount = 0
errEq(pcall(gtmul), "non-number __index result times a number")
numEq(idxCount, 1, "__index must not be re-evaluated by the fused MUL")

local numIdx = setmetatable({}, {
  __index = function(_, k)
    if k == "k" then
      return 2.5
    else
      return 0
    end
  end,
})
local function gtmul3()
  local s = 0
  for i = 1, 4 do
    s = s + numIdx["k"] * 2
  end
  return s
end
numEq(gtmul3(), 20, "number __index through the fused GETTABLE→MUL")

local plain = { 1.5, 2.5, 3.5 }
local function gtmul2()
  local s = 0
  for i = 1, 3 do
    s = s + plain[i] * 2
  end
  return s
end
numEq(gtmul2(), 15, "array hits through the fused GETTABLE→MUL")

-- 7) MUL → ADD：加法操作数是字符串，错误在链中段抛出
local badAdd = "x"
local function muladd()
  local s = 0
  for i = 1, 3 do
    s = s + (i * 2) + badAdd
  end
  return s
end
errEq(pcall(muladd), "string operand inside the fused MUL→ADD")

-- 8) MULK → ADD 走 __add：只保留最后一次赋值，链上不得多跑一圈
local addCount = 0
local big = setmetatable({}, {
  __add = function(_, _)
    addCount = addCount + 1
    return 100
  end,
})
local function mulkadd()
  local s = nil
  for i = 1, 3 do
    s = (i * 10) + big
  end
  return s
end
addCount = 0
numEq(mulkadd(), 100, "__add through the fused MULK→ADD")
numEq(addCount, 3, "__add must run once per iteration")

-- 9) 协程跨融合臂 yield/resume：每次让出都要能按原状态续上
local co = coroutine.create(function(n)
  local a = 0
  for _ = 1, 6 do
    if 2 < n then
      a = a + (n - 1)
    else
      a = a + 1
    end
    coroutine.yield(a)
  end
  return a
end)
local vals = {}
while coroutine.status(co) ~= "dead" do
  local ok, v = coroutine.resume(co, 3)
  assert(ok, tostring(v))
  vals[#vals + 1] = v
end
numEq(#vals, 7, "six yields plus the return value")
numEq(vals[1], 2, "first resume across the fused edge")
numEq(vals[6], 12, "sixth yield across the fused edge")
numEq(vals[7], 12, "coroutine state must not drift across yields")

-- 9b) LOADN → JUMPIFNOTLE → GETUPVAL → SUBK/ADDK 串起来的复合链
local accUp = 10
local function leChain(n)
  local a = 0
  for i = 1, n do
    if i <= 3 then
      a = a + (accUp - i)
    else
      a = a + (accUp + i)
    end
  end
  return a
end
numEq(leChain(6), 69, "JUMPIFNOTLE plus SUBK/ADDK chain")
errEq(pcall(leChain, "x"), "string loop bound on the fused chain")

-- 10) 融合链尾巴里抛错：展开路径与 pc 回写
local function deep(n)
  if 1 < n then
    error("deep")
  end
  return n
end
local okDeep, msgDeep = pcall(deep, 2)
assert(not okDeep, "error inside the fused LOADN→JUMPIFNOTLT must propagate")
assert(string.find(msgDeep, "deep", 1) ~= nil, tostring(msgDeep))
numEq(deep(0), 0, "same function must stay usable after the raise")

-- 11) LOADB → SETTABLE / ADD → SETTABLE：写回与再散列路径
local t = { 1, 2, 3 }
local function writeThrough(n)
  for i = 1, n do
    t[i] = i * 2
  end
  t[1] = true
  if t[1] == true then
    return t[2]
  else
    return -1
  end
end
numEq(writeThrough(3), 4, "LOADB→SETTABLE plus ADD→SETTABLE")
local g = {}
local function grow(n)
  for i = 1, n do
    g[i] = i + 0.5
  end
  return g[n]
end
numEq(grow(9), 9.5, "rehashing writes through the fused SETTABLE edge")

return "OK"
