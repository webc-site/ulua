export default {
  hello: `-- hello.luau
-- The classic first program: print a greeting, then show a few basics.

-- \`print\` accepts multiple arguments, separated by tabs in the output.
print("Hello, world!")

-- Numbers are double-precision; arithmetic works as expected.
local a = 7
local b = 5
print("sum:", a + b)          -- 12
print("product:", a * b)      -- 35
print("power:", a ^ 2)        -- 49 (^ is exponentiation)
print("remainder:", a % b)    -- 2  (% is modulo)

-- Strings are joined with the \`..\` concatenation operator.
local name = "Luau"
print("Welcome to " .. name .. "!")

-- Numbers are coerced to strings inside concatenation.
print("a + b = " .. (a + b))

-- \`print\` with several values of mixed types on one line.
print("mixed:", 42, true, "text", nil)
`,

  fibonacci: `-- fibonacci.luau
-- Two ways to compute Fibonacci numbers: recursion and iteration.

-- Recursive definition: fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2).
local function fibRecursive(n)
\tif n < 2 then
\t\treturn n
\tend
\treturn fibRecursive(n - 1) + fibRecursive(n - 2)
end

-- Iterative definition: keep the last two values and step forward.
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
-- Tables are Luau's single data structure: they act as arrays AND dictionaries.

-- An array-like table: consecutive integer keys starting at 1.
local fruits = { "apple", "banana", "cherry" }

-- \`#t\` gives the length (number of array entries).
print("number of fruits:", #fruits)

-- \`ipairs\` walks array entries in order (1, 2, 3, ...).
print("fruits in order:")
for index, fruit in ipairs(fruits) do
\tprint(index, fruit)
end

-- \`table.insert\` appends to the end of the array part.
table.insert(fruits, "date")
print("after insert, length:", #fruits)

-- A dictionary-like table: arbitrary string keys mapping to values.
local ages = { alice = 30, bob = 25, carol = 41 }

-- \`pairs\` walks every key/value pair.
print("ages:")
for name, age in pairs(ages) do
\tprint(name, age)
end

-- Tables can mix and nest freely.
local person = { name = "Dave", hobbies = { "chess", "cycling" } }
print(person.name .. " enjoys " .. person.hobbies[1] .. " and " .. person.hobbies[2])
`,

  metatables: `-- metatables.luau
-- Object-oriented programming in Luau is built on metatables.
-- Here we make a small Vector2 "class".

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
-- A tour of the string library.

local s = "Hello, Luau"

print("length:", #s)                         -- byte length
print("upper:", string.upper(s))
print("lower:", string.lower(s))
print("sub(1, 5):", string.sub(s, 1, 5))     -- "Hello"
print("sub(8):", string.sub(s, 8))            -- "Luau"
print("rep:", string.rep("ab", 3))            -- "ababab"

-- string.format works like C printf.
print(string.format("name=%s pi=%.2f count=%d", "Luau", 3.14159, 42))

local start, finish = string.find(s, "Luau")
print("found 'Luau' at:", start, finish)      -- 8  11

local replaced, count = s:gsub("l", "L")
print("gsub result:", replaced)
print("gsub count:", count)
`,

  coroutines: `-- coroutines.luau
-- Coroutines are cooperative, resumable functions: they pause with
-- \`coroutine.yield\` and continue with \`coroutine.resume\`.

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
-- Luau adds a gradual type system with local type inference to Lua.
-- The \`--!strict\` comment at the top turns on strict type checking.

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
-- This file is DELIBERATELY wrong. The syntax is valid (it parses), but the
-- types do not line up. Press "Type-check" to see the analyzer catch it.

local function double(n: number): number
\treturn n * 2
end

local count: number = "not a number"

print(double("ten"))
print(count)
`,
};
