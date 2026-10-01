-- 热层派发 × interrupt 钩子定向 conformance：本仓新增，cpp/tests/conformance 无对应源。
--
-- 本 fixture 里的函数由 Rust 侧驱动（tests/conformance/hot_dispatch.rs 写入
-- `callbacks.interrupt` 后逐函数 resume）。钩子在热臂回边（LOP_FORNLOOP / LOP_JUMPBACK /
-- LOP_FORNPREP）上命中，钉三件事：
--   1. 钩子激活时热臂经 `backedge_idle` 判定整臂 become 到冷续延，回边与 pc 推进不变
--      （迭代次数与求和结果即判据）；
--   2. 钩子里 yield_：线程从热臂中途挂起，续跑后按原状态跑完（savedpc / base 回写正确）；
--   3. 钩子里 luaL_error：可被循环内层的 pcall 接住，展开后同一线程继续跑热循环仍正确。
-- 本 fixture 以 skipCodegen=true 运行：被测对象是解释器的 become 尾调用派发层。

function hotforn(n)
	local s = 0
	for i = 1, n do
		s = s + i
	end
	return s
end

function hotwhile(n)
	local i = 0
	local s = 0
	while i < n do
		i = i + 1
		s = s + i
	end
	return s
end

function hotnested(n)
	local t = {}
	local acc = 0
	for i = 1, n do
		t[i] = i * 2
		acc = acc + t[1] % 3
	end
	return acc
end

-- 钩子在循环体内抛错：内层 pcall 必须接住；展开后仍在钩子下跑一段热循环，
-- 借它验证 savedpc / 栈槽状态没被展开破坏（钩子此时已不再抛错，只会计数）。
function hotpcall(n)
	local ok, err = pcall(function()
		local acc = 0
		for i = 1, n do
			acc = acc + i
		end
		return acc
	end)
	if ok then
		return "uncaught"
	end
	if not err:find("timeout", 1, true) then
		return "badmsg"
	end
	if hotforn(3) ~= 6 then
		return "broken"
	end
	return "caught"
end

return "OK"
