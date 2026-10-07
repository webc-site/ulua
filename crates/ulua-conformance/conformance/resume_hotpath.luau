-- 本仓新增，cpp/tests/conformance 无对应源。
--
-- 协程恢复热路定向用例：钉 `resume_continue` 的 Lua 帧直达派发路径（三旗排除后
-- 持有闭包直进 tier_cold）在反复挂起/恢复下的状态重建正确性——上值读写、常量
-- 表取值、元方法调用、yield 结果回传、pcall 展开与跨恢复错误全部逐值核对。

-- ① 热乒乓：上值 + 元方法 + 常量，逐轮精确值
local shared = 0

local mt = {
  __add = function(a, b)
    return a.v + b.v + 1
  end,
  __index = function(t, k)
    return k == "tag" and "mt" or nil
  end,
}

local ping = coroutine.create(function(n)
  local acc = 0
  for i = 1, n do
    acc = acc + i
    shared = shared + 1
    local obj = setmetatable({ v = i }, mt)
    acc = acc + (obj + obj)
    assert(obj.tag == "mt")
    local back = coroutine.yield(acc)
    acc = acc + back
  end
  return acc, shared
end)

local ok, v
for i = 1, 100 do
  -- 首轮实参 100 即循环轮数；后续实参 2 只作为 yield 回传值
  ok, v = coroutine.resume(ping, i == 1 and 100 or 2)
  assert(ok, v)
end
-- 每轮：acc += i + (2i+1) + 2（yield 回传 2）；100 轮后返回
-- Σ(3i+3) = 3·5050 + 300 = 15450
local ok2, acc2, shared2 = coroutine.resume(ping, 2)
assert(ok2, `final resume failed: {tostring(acc2)}`)
assert(acc2 == 15450, `expected acc 15450, got {acc2}`)
assert(shared2 == 100, `expected shared 100, got {shared2}`)
assert(coroutine.status(ping) == "dead")

-- ② yield 实参回传逐值：resume 的首实参窗在重入后成为 yield 返回值
local echo = coroutine.create(function()
  for i = 1, 50 do
    local a, b = coroutine.yield(i * 1000)
    assert(a == i * 3 and b == -i, `echo iter {i}: got {a},{b}`)
  end
  return "done"
end)

local ok3, v3 = coroutine.resume(echo)
assert(ok3 and v3 == 1000, `echo start: got {tostring(v3)}`)
for i = 1, 49 do
  local ok4, v4 = coroutine.resume(echo, i * 3, -i)
  assert(ok4 and v4 == (i + 1) * 1000, `echo yield {i}: got {tostring(v4)}`)
end
local ok5, v5 = coroutine.resume(echo, 150, -50)
assert(ok5 and v5 == "done", `echo end: got {tostring(v5)}`)

-- ③ pcall handler 帧内 yield 后续跑（resume_finish 找回 handler 帧的路径）
local handled = 0
local inner = coroutine.create(function()
  for i = 1, 20 do
    local okp, err = pcall(function()
      coroutine.yield(i * 10)
      error(`boom{i}`)
    end)
    assert(not okp)
    assert(err:find(`boom{i}`), `iter {i}: {err}`)
    handled = handled + 1
  end
  return handled
end)

for i = 1, 20 do
  local ok5, v5 = coroutine.resume(inner)
  assert(ok5 and v5 == i * 10, `inner yield {i}: got {v5}`)
end
local ok6, v6 = coroutine.resume(inner)
assert(ok6 and v6 == 20, `handled {v6}`)

-- ④ 跨恢复错误：抛错后协程死透、可再度 resume 报 dead
local dying = coroutine.create(function()
  coroutine.yield(1)
  error("fatal")
end)

local ok7, v7 = coroutine.resume(dying)
assert(ok7 and v7 == 1)
local ok8, v8 = coroutine.resume(dying)
assert(not ok8)
assert(v8:find("fatal"), v8)
local ok9, v9 = coroutine.resume(dying)
assert(not ok9)
assert(v9:find("dead coroutine"), v9)

return "OK"
