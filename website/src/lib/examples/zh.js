export default {
  hello: `-- hello.luau
-- 经典的初始示例：输出问候信息，演示基础语法。

-- \`print\` 支持接收多个参数，输出时以制表符分隔。
print("Hello, world!")

-- 数值均为双精度浮点数；基础算术按预期工作。
local a = 7
local b = 5
print("sum:", a + b)          -- 12
print("product:", a * b)      -- 35
print("power:", a ^ 2)        -- 49 (^ 为幂运算)
print("remainder:", a % b)    -- 2  (% 为取模运算)

-- 字符串通过 \`..\` 连接运算符拼接。
local name = "Luau"
print("Welcome to " .. name .. "!")

-- 在连接运算中，数值会自动转换为字符串。
print("a + b = " .. (a + b))

-- 单行输出多种混合类型的值。
print("mixed:", 42, true, "text", nil)
`,

  fibonacci: `-- fibonacci.luau
-- 计算斐波那契数列的两种方式：递归与迭代。

-- 递归定义：fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2)。
local function fibRecursive(n)
\tif n < 2 then
\t\treturn n
\tend
\treturn fibRecursive(n - 1) + fibRecursive(n - 2)
end

-- 迭代定义：保留最后两个数值并向前推进。
local function fibIterative(n)
\tlocal prev, curr = 0, 1
\tfor _ = 1, n do
\t\tprev, curr = curr, prev + curr
\tend
\treturn prev
end

print("Recursive:")
for i = 0, 9 do
\tprint(i, fibRecursive(i))
end

print("Iterative:")
for i = 0, 9 do
\tprint(i, fibIterative(i))
end
`,

  tables: `-- tables.luau
-- 表是 Luau 核心数据结构：兼具数组与字典两种形态。

-- 数组型表：连续的正整数键，从 1 开始。
local fruits = { "apple", "banana", "cherry" }

-- \`#t\` 获取数组长度（元素个数）。
print("number of fruits:", #fruits)

-- \`ipairs\` 按序遍历数组项（1, 2, 3, ...）。
print("fruits in order:")
for index, fruit in ipairs(fruits) do
\tprint(index, fruit)
end

-- \`table.insert\` 向数组末尾追加元素。
table.insert(fruits, "date")
print("after insert, length:", #fruits)

-- 字典型表：任意字符串键映射到对应值。
local ages = { alice = 30, bob = 25, carol = 41 }

-- \`pairs\` 遍历表中的每个键值对。
print("ages:")
for name, age in pairs(ages) do
\tprint(name, age)
end

-- 表可以自由混合与多层嵌套。
local person = { name = "Dave", hobbies = { "chess", "cycling" } }
print(person.name .. " enjoys " .. person.hobbies[1] .. " and " .. person.hobbies[2])
`,

  metatables: `-- metatables.luau
-- Luau 的面向对象编程构建于元表之上。
-- 这里构建一个轻量二维向量 Vector2 类。

local Vector2 = {}
Vector2.__index = Vector2

function Vector2.new(x, y)
\tself = setmetatable({}, Vector2)
\tself.x = x
\tself.y = y
\treturn self
end

function Vector2:magnitude()
\treturn (self.x * self.x + self.y * self.y) ^ 0.5
end

function Vector2.__add(a, b)
\treturn Vector2.new(a.x + b.x, a.y + b.y)
end

function Vector2:toString()
\treturn "(" .. self.x .. ", " .. self.y .. ")"
end

local v1 = Vector2.new(3, 4)
local v2 = Vector2.new(1, 2)

print("v1:", v1:toString())
print("v2:", v2:toString())
print("v1 magnitude:", v1:magnitude())   -- 5

local sum = v1 + v2
print("v1 + v2:", sum:toString())          -- (4, 6)
`,

  strings: `-- strings.luau
-- 标准字符串库常见操作概览。

local s = "Hello, Luau"

print("length:", #s)                         -- 字节长度
print("upper:", string.upper(s))
print("lower:", string.lower(s))
print("sub(1, 5):", string.sub(s, 1, 5))     -- "Hello"
print("sub(8):", string.sub(s, 8))            -- "Luau"
print("rep:", string.rep("ab", 3))            -- "ababab"

-- string.format 用法类似 C 语言 printf。
print(string.format("name=%s pi=%.2f count=%d", "Luau", 3.14159, 42))

local start, finish = string.find(s, "Luau")
print("found 'Luau' at:", start, finish)      -- 8  11

local replaced, count = s:gsub("l", "L")
print("gsub result:", replaced)
print("gsub count:", count)
`,

  coroutines: `-- coroutines.luau
-- 协程是协作式、可恢复的函数：通过 coroutine.yield 暂停，
-- 通过 coroutine.resume 继续执行。

local function producer()
\tcoroutine.yield("first")
\tcoroutine.yield("second")
\tcoroutine.yield("third")
\treturn "done"
end

local routine = coroutine.create(producer)

print("resume 1:", coroutine.resume(routine))   -- true  first
print("resume 2:", coroutine.resume(routine))   -- true  second
print("resume 3:", coroutine.resume(routine))   -- true  third
print("resume 4:", coroutine.resume(routine))   -- true  done
print("status:", coroutine.status(routine))      -- dead
`,

  typed: `--!strict
-- typed.luau
-- Luau 为 Lua 引入了具备局部类型推断的渐进类型系统。
-- 顶部的 \`--!strict\` 注释指令开启严格类型检查模式。

type Player = {
\tname: string,
\tscore: number,
\tisActive: boolean,
}

local function formatPlayer(p: Player): string
\treturn string.format("%s: %d pts (%s)", p.name, p.score, p.isActive and "active" or "idle")
end

local function map<T, U>(list: { T }, fn: (T) -> U): { U }
\tlocal out = {}
\tfor _, item in ipairs(list) do
\t\ttable.insert(out, fn(item))
\tend
\treturn out
end

local players: { Player } = {
\t{ name = "Alice", score = 42, isActive = true },
\t{ name = "Bob", score = 18, isActive = false },
}

for _, line in ipairs(map(players, formatPlayer)) do
\tprint(line)
end

local function scoreValue(s: number | string): number
\tif typeof(s) == "number" then
\t\treturn s
\telse
\t\treturn #s
\tend
end

local total = 0
for _, s in ipairs({ 10, "bonus", 45 }) do
\ttotal = total + scoreValue(s)
end
print("total score:", total)   -- 60
`,

  globals: `-- globals.luau
print(_G)
for i, v in _G do
\tprint(i)
\tprint(v)
end
print("Done")
`,

  type_error: `--!strict
-- type_error.luau
-- 本文件故意构造了类型错误。语法完全有效，但类型无法匹配。
-- 点击「类型检查」即可观察静态分析器的精准拦截。

local function double(n: number): number
\treturn n * 2
end

local count: number = "not a number"

print(double("ten"))
print(count)
`,
};
