-- call_fallback 慢路径定向 conformance。
-- 对照 ./cpp oracle 行为（cpp/CodeGen/src/CodeGenUtils.cpp:301 callFallback）：
-- __call 元方法就地前移参数区（luaV_tryfuncTM 后 argtop++）、ci->top = argtop +
-- stacksize、vararg 帧的 L->top 收敛（不得被 argend 截断）、C 闭包返回值按
-- nresults 收敛 / MULTRET 全量转发。所有经"值中转"的调用（call 参数、__call 表、
-- loadstring 产物）在字节码里都是普通 CALL：LUAU_CODEGEN=1 下原生代码的每条
-- CALL 都经 call_fallback 建帧，由此保证慢路径被真实执行；同一用例在解释器与
-- 原生两种模式下断言一致。

-- 经参数中转调用：被调者在编译期未知，不会降级为 builtin FASTCALL，
-- 保证走通用 CALL 序列。
local function call(f, ...)
	return f(...)
end

-- __call 元方法 + vararg 被调函数：实参少于/恰等于/多于 numparams 三种组合。
do
	local function self_and_vararg(self_, a, ...)
		local n = select('#', ...)
		return n, a, ...
	end

	local t = setmetatable({}, { __call = self_and_vararg })

	-- 少于 numparams：缺省参数补 nil，vararg 区为空
	local n, a, b = t()
	assert(n == 0 and a == nil and b == nil)

	-- 恰好填满 numparams：vararg 区为空
	local n, a, b = t(10)
	assert(n == 0 and a == 10 and b == nil)

	-- 多于 numparams（回归点：vararg 区曾被 argend 截断丢失多余实参）
	local n, a, b, c, d = t(10, 20, 30)
	assert(n == 2 and a == 10 and b == 20 and c == 30 and d == nil)

	-- MULTRET 位置转发 __call 的全部返回值
	local packed = table.pack(t(1, 2, 3, 4))
	assert(packed.n == 5)
	assert(packed[1] == 3 and packed[2] == 1 and packed[3] == 2 and packed[4] == 3 and packed[5] == 4)
	assert(packed[6] == nil)
end

-- __call 元方法为 C 闭包：tryfuncTM 前移后直接进入 call_fallback 的 C 分支。
do
	-- __call 表自身兼作被操作数组：handler 收到 (self, ...) 后恰为 unpack(arr, ...)
	local arr = setmetatable({ 1, 2, 3 }, { __call = table.unpack })

	local a, b, c = arr()
	assert(a == 1 and b == 2 and c == 3)

	local p, q = arr(2, 3)
	assert(p == 2 and q == 3)

	-- MULTRET：C 闭包返回值全量转发
	assert(select('#', arr()) == 3)
	local packed = table.pack(arr())
	assert(packed.n == 3 and packed[3] == 3)

	-- 0 个返回值的 C 闭包
	local quiet = setmetatable({}, { __call = table.sort })
	assert(select('#', quiet()) == 0)
	assert(table.pack(quiet()).n == 0)
end

-- 非函数值触发 __tryfuncTM：参数区前移后必须完整（回归点：
-- ci->top 少加 stacksize / argtop 漏移）。
do
	local args = table.create(40, 7)
	local t = setmetatable({}, { __call = function(s, ...)
		return select('#', ...), ...
	end })

	local n, first, last = t(table.unpack(args))
	assert(n == 40 and first == 7 and last == 7)

	local packed = table.pack(t(table.unpack(args)))
	assert(packed.n == 41 and packed[1] == 40 and packed[41] == 7)
end

-- __tryfuncTM 错误路径：无 __call / __call 非函数时报 type error。
do
	local t = {}

	local ok, err = pcall(function()
		t()
	end)
	assert(not ok)
	assert(string.find(err, "attempt to call a table value", 1, true) ~= nil)

	local ok, err = pcall(function()
		local x
		x()
	end)
	assert(not ok)
	assert(string.find(err, "attempt to call a nil value", 1, true) ~= nil)

	local bad = setmetatable({}, { __call = 42 })

	local ok, err = pcall(function()
		bad()
	end)
	assert(not ok)
	assert(string.find(err, "attempt to call a table value", 1, true) ~= nil)
end

-- 经 JIT 调用 C 函数：返回值个数 0 / 1 / N 与 MULTRET 转发。
do
	-- 0 个返回值
	assert(select('#', call(table.sort, { 3, 1, 2 })) == 0)
	assert(table.pack(call(table.sort, { 3, 1, 2 })).n == 0)

	-- 1 个返回值
	assert(call(string.find, "hello world", "wor") == 7)
	assert(call(string.find, "hello", "xyz") == nil)
	assert(call(math.max, 1, 2, 3) == 3)

	-- N 个返回值
	local a, b = call(string.find, "hello world", "o")
	assert(a == 5 and b == 5)
	assert(table.pack(call(string.find, "hello world", "o")).n == 2)

	-- MULTRET：C 函数返回值全量展开进后续调用
	local packed = table.pack(call(table.unpack, { 1, 2, 3 }))
	assert(packed.n == 3 and packed[1] == 1 and packed[2] == 2 and packed[3] == 3)

	-- C 函数再调 C 函数（pcall 收敛 string.find 的两个返回值）
	local ok, a, b = call(pcall, string.find, "hello world", "wor")
	assert(ok == true and a == 7 and b == 9)
end

-- loadstring 产物没有原生目标：LUAU_CODEGEN=1 下从原生调用方进入 call_fallback
-- 的 Lua 闭包分支（非 vararg 补 nil / vararg 收敛 top）。
do
	local fixed = loadstring("return function(a, b) return a, b end")()

	-- 实参少于 numparams：缺省槽补 nil
	local a, b = fixed()
	assert(a == nil and b == nil)

	-- 恰等于 numparams
	local a, b = fixed(1, 2)
	assert(a == 1 and b == 2)

	-- 多于 numparams：多余实参静默丢弃
	local a, b = fixed(1, 2, 3, 4)
	assert(a == 1 and b == 2)

	local var = loadstring("return function(a, ...) return a, select('#', ...), ... end")()

	local a, n, b, c = var()
	assert(a == nil and n == 0 and b == nil and c == nil)

	local a, n, b, c = var(1)
	assert(a == 1 and n == 0 and b == nil and c == nil)

	local a, n, b, c, d = var(1, 2, 3)
	assert(a == 1 and n == 2 and b == 2 and c == 3 and d == nil)

	-- MULTRET 转发非原生 vararg 闭包的全部返回值
	local packed = table.pack(var(1, 2, 3))
	assert(packed.n == 4 and packed[1] == 1 and packed[2] == 2 and packed[3] == 2 and packed[4] == 3)
	assert(packed[5] == nil)
end

-- 大栈帧被调函数：call_fallback 建帧时 ci->top 必须完整覆盖 stacksize
-- （回归点：ci->top = argtop + stacksize 曾少加）。
do
	local wide = loadstring([[
		return function(...)
			local a, b, c, d, e, f, g, h, i, j = ...
			local t = { a, b, c, d, e, f, g, h, i, j }
			local ok, m = pcall(function()
				return math.max(a + j, b + i, c + h, d + g, e + f)
			end)
			return ok, m, #t, select('#', ...)
		end
	]])()

	local ok, m, len, n = wide(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
	assert(ok == true and m == 11 and len == 10 and n == 10)

	-- 同一大栈帧经 __call 元方法调用：tryfuncTM 前移 + 大栈帧回归点叠加
	local adapter = setmetatable({}, { __call = function(s, ...)
		return wide(...)
	end })

	local ok, m, len, n = adapter(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
	assert(ok == true and m == 11 and len == 10 and n == 10)
end

-- 原生目标的大栈帧 vararg 闭包：native→native 调用同样经 call_fallback 建帧。
do
	local function wide_native(a, b, c, d, e, f, g, h, i, j, ...)
		local t = { a, b, c, d, e, f, g, h, i, j, ... }
		local ok, m = pcall(function()
			return math.max(a + j, b + i, c + h, d + g, e + f)
		end)
		return ok, m, #t, select('#', ...)
	end

	-- 实参多于 numparams：varargs 保留（回归点：vararg 分支 top 被 argend 截断）
	local ok, m, len, n = wide_native(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
	assert(ok == true and m == 11 and len == 12 and n == 2)

	-- 实参少于 numparams：缺省槽补 nil，算术经 pcall 失败
	local ok, m, len, n = wide_native(1, 2)
	assert(ok == false and len == 2 and n == 0)
	assert(type(m) == "string")
end

return('OK')
