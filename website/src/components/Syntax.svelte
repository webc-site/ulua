<script>
  import { t } from "../lib/i18n.svelte.js";
  import chevron_svg from "../svg/chevron.svg";

  const LUA_GROUP_LI = [
      [
        "lua_basics",
        [
          [
            "lua_variables",
            `-- 全局变量存储于 _G 环境表中，未声明直接读取返回 nil，极易引发拼写静默错误：
global_counter = 100

-- local 显式声明局部变量，分配在虚拟机寄存器栈中，执行速度显著更快：
local limit = 50

local function process(val)
    local factor = 2
    if val > limit then
        -- do ... end 显式划分独立词法作用域块：
        do
            local factor = 10  -- 变量遮蔽 (Shadowing)，仅在此代码块生效
            val = val + factor
        end
        -- 此处 factor 自动恢复为外层局部变量 2
        return val * factor
    end
    return val
end

print(process(60))  -- 输出: 140`,
          ],
          [
            "lua_types",
            `-- Lua 5.1 具备 8 大动态基础类型：
-- nil, boolean, number, string, table, function, thread, userdata

local val_nil = nil
local val_bool = false
local val_num = 0
local val_str = ""

-- 条件判定真假值法则：
-- ! 在 Lua 中，只有 false 与 nil 判定为假（falsy）
-- ! 数值 0、空字符串 "" 以及空表 {} 均为真（truthy）！
if val_num and val_str then
    print('数值 0 与空字符串 "" 均严格判定为真 (truthy)')
end

-- 运行时动态类型检测：
print(type(val_nil))   -- 输出: "nil"
print(type(val_num))   -- 输出: "number"
print(type(print))     -- 输出: "function"`,
          ],
          [
            "lua_strings",
            `-- 1. 单引号与双引号等价，支持十进制 ASCII 与常见转义：
local msg = "Line 1\\nLine 2\\tIndent"
local char_a = "\\065"  -- "A"（十进制 ASCII 码点转义）

-- 2. Lua 字符串是 8 位清洁（8-bit clean）的，可安全包含空字符 '\\0'：
local binary_data = "a\\0b\\0c"
print(#binary_data)  -- 输出: 5（以字节计算精确长度）

-- 3. [=[ ... ]=] 多行长括号原始文本块：
-- 支持在两个括号之间加入任意数量的 '='，杜绝内部包含 ']]' 引发的解析冲突：
local raw_code = [==[
    local sample = [[嵌套的长括号文本无需转义]]
    local query = "SELECT * FROM users WHERE status = 'OK';"
]==]

-- 4. 字符串拼接 '..'（触发数字自动隐式转换）与取字节长度 '#'：
local welcome = "Port: " .. 8080
print(welcome)   -- "Port: 8080"
print(#welcome)  -- 10（字节长度）`,
          ],
        ],
      ],
      [
        "lua_control",
        [
          [
            "lua_conditionals",
            `local user_role = "editor"
local permissions = nil

-- 标准 if-then-elseif-else 结构
if user_role == "admin" then
    permissions = "all"
elseif user_role == "editor" then
    permissions = "edit"
else
    permissions = "view"
end

-- 逻辑短路求值（Short-circuit）：
-- and / or 运算符返回决定表达式结果的操作数本身：
local port = custom_port or 8080

-- 传统模拟三元表达式 (cond and a or b)：
local is_logged_in = true
local display_name = is_logged_in and "Alice" or "Guest"

-- 经典假值陷阱警示：
local is_feature_enabled = false
-- 若待返回值为 false，表达式将错误跳向 fallback：
local status = true and is_feature_enabled or "fallback"
print(status)  -- 输出: fallback（非预期！建议迁移至 Luau if-then-else 表达式）`,
          ],
          [
            "lua_loops",
            `-- 1. while 循环（前置判定）：
local n = 3
while n > 0 do
    n = n - 1
end

-- 2. repeat ... until 循环（后置判定）：
-- 条件表达式处于循环体内层词法作用域，可合法访问循环体内声明的 local 变量
local attempts = 0
repeat
    local success = (math.random() > 0.5)
    attempts = attempts + 1
until success or attempts >= 3

-- 3. 数值型 for 循环 (起始, 终止 [, 步长])：
-- 三个边界表达式在进入循环前严格求值一次；i 是只读的局部循环控制变量
for i = 1, 5, 2 do
    print("Numeric for step:", i)  -- 输出 1, 3, 5
end`,
          ],
          [
            "lua_pairs",
            `local inventory = {
    sword = "Excalibur",
    shield = "Aegis",
    potion = "Elixir",
}

-- 1. pairs：遍历哈希表的全部键值对（遍历顺序未指定）：
for key, val in pairs(inventory) do
    print(key .. " -> " .. val)
end

local quest_steps = { "Start", "Find Clue", "Defeat Boss" }

-- 2. ipairs：从索引 1 开始顺序遍历连续正整数序列，遇到第一个 nil 终止：
for idx, step_name in ipairs(quest_steps) do
    print(idx .. ": " .. step_name)
end`,
          ],
          [
            "lua_break",
            `local items = { 10, 20, 99, 40, 50 }
local target_idx = nil

for idx = 1, #items do
    if items[idx] == 99 then
        target_idx = idx
        break  -- 合法：跳出当前最内层循环
    end
end

-- 语法规则：break 必须是所属块（block）的最后一条语句：
for i = 1, 3 do
    if i == 2 then
        do break end  -- 若在代码块中间强行跳出，需用 do ... end 包裹以符合语法规范
    end
    print(i)
end`,
          ],
        ],
      ],
      [
        "lua_functions",
        [
          [
            "lua_functions",
            `-- 1. 函数是一等公民：可存入变量、作为形参传递或作为返回值：
local function div_mod(a, b)
    local q = math.floor(a / b)
    local r = a % b
    return q, r  -- 原生支持返回任意数量的值
end

-- 逗号多变量解构接收：
local quot, rem = div_mod(17, 5)  -- quot = 3, rem = 2

-- 2. 多返回值展开与截断规则：
local function get_pair() return "A", "B" end

-- 处于表达式末尾：全部展开
print(get_pair())                -- 输出: A    B
local full_pack = { get_pair() } -- full_pack = { "A", "B" }

-- 处于表达式非末尾：严格截断仅保留首个值
local truncated = { get_pair(), "C" } -- truncated = { "A", "C" }（"B" 丢弃）

-- 3. 核心绝技：外层加括号无条件强制单值截断 (f())：
-- 无论是否在末尾，只要用圆括号包裹，严格只保留第一个返回值！
print((get_pair()))              -- 输出: A（"B" 强制截断丢弃）
local single = { (get_pair()) }  -- single = { "A" }`,
          ],
          [
            "lua_varargs",
            `-- 使用 ... 接收任意数量的可变长实参
local function custom_log(prefix, ...)
    -- 1. select("#", ...)：以 O(1) 效率精确统计传入实参个数（包含显式传入的 nil）：
    -- 绝不构造临时 table，零垃圾回收堆内存分配负担！
    local count = select("#", ...)
    print(prefix .. " 实参个数: " .. count)

    -- 2. select(i, ...)：获取从第 i 个位置开始的全部后续参数序列：
    for i = 1, count do
        local val = select(i, ...)
        print("Param " .. i .. ":", val)
    end
end

custom_log("[DEBUG]", "alpha", nil, 42)

-- 3. 参数完美转发代理：直接将 ... 原样透传，无额外损耗：
local function proxy_call(target_fn, ...)
    return target_fn(...)
end`,
          ],
          [
            "lua_closures",
            `-- 词法闭包：内部函数捕获并持久化持有外层局部变量（Upvalue）
local function create_shared_counters(initial_val)
    local shared_count = initial_val  -- Upvalue（函数返回后从栈槽逃逸至堆内存）

    local function increment(step)
        shared_count = shared_count + (step or 1)
        return shared_count
    end

    local function get_current()
        return shared_count
    end

    return increment, get_current
end

local inc, get = create_shared_counters(10)

-- 核心特性：多闭包共享同一 Upvalue，任一闭包的修改对外完全实时同步！
inc(5)
print("当前计数:", get())  -- 输出: 15（两函数共享并同步同一底层变量）`,
          ],
          [
            "lua_errors",
            `-- Lua 5.1 不提供 try-catch 语法，通过 pcall 与 xpcall 实现安全异常防护：

local function risky_division(a, b)
    if b == 0 then
        error("除数不可为 0！", 2)  -- level 2 将错误归因于调用方代码帧
    end
    return a / b
end

-- 1. pcall (Protected Call)：保护模式执行
-- 返回格式：成功返回 (true, 结果...)；异常返回 (false, 错误对象)
local ok, res = pcall(risky_division, 10, 0)
if not ok then
    print("捕获运行时错误:", res)  -- res 为 error 抛出的错误信息
end

-- 2. xpcall：在调用栈因展开而损毁前，通过错误处理函数获取完整堆栈追踪：
local function safe_run(f, ...)
    return xpcall(f, function(err)
        return debug.traceback("堆栈详情: " .. tostring(err), 2)
    end, ...)
end

local status, output = safe_run(risky_division, 20, 0)`,
          ],
        ],
      ],
      [
        "lua_tables",
        [
          [
            "lua_tables",
            `-- 表是 Lua 唯一的复合数据结构，内部由【数组部分】与【哈希字典部分】双核驱动：
local inventory = {
    name = "PlayerOne",
    score = 999,
    items = { "Sword", "Shield", "Potion" }, -- 连续正整数索引 1..n 分配在原生数组区
}

-- 连续数组操作标准库 table：
table.insert(inventory.items, "Bow")          -- 尾部追加
table.insert(inventory.items, 1, "Dagger")     -- 指定索引插入
local removed = table.remove(inventory.items) -- 弹出末尾元素

-- 取长度运算符 '#' 仅对【连续正整数无空洞序列（Sequence）】具有确定性行为：
print("道具总数:", #inventory.items)           -- 输出: 4
print(table.concat(inventory.items, ", "))     -- "Dagger, Sword, Shield, Potion"

-- ! 空洞陷阱（Array Hole）：若向连续数组中间注入 nil，
-- '#' 运算符将退化为二分查找边界，其返回值未定义且不可信赖！`,
          ],
          [
            "lua_metatables",
            `-- 元表（Metatable）：重定义表在面临特定事件（如访问空键、加法、调用）时的行为

local default_props = { host = "localhost", timeout = 30 }
local meta = {}

-- 1. __index：访问不存在字段时的回退代理（可为表，亦可为动态计算函数）：
meta.__index = function(t, key)
    if default_props[key] then return default_props[key] end
    return "UNKNOWN_KEY"
end

-- 2. __newindex：拦截新字段的写入（用于构建只读表或字段白名单）：
meta.__newindex = function(t, key, val)
    error("禁止动态注入未定义字段: " .. tostring(key))
end

local config = setmetatable({}, meta)
print(config.host)     -- 自身不存在，触发 __index 函数返回: "localhost"

-- 3. 穿透直读与直写：rawget 与 rawset 绕过元方法，直击底层哈希槽：
rawset(config, "raw_id", 101)  -- 绕过 __newindex，成功写入
print(rawget(config, "raw_id")) -- 绕过 __index，输出: 101`,
          ],
          [
            "lua_weak_tables",
            `-- __mode 元方法：让表具备弱引用能力，常用于构建防内存泄漏的对象池与记忆化缓存：
-- __mode = "v"（弱值）：表中引用的对象若无外部强引用，GC 运行时将自动回收并移除条目
-- __mode = "k"（弱键）：键为弱引用；__mode = "kv"（全弱）：键与值均为弱引用

local object_cache = setmetatable({}, { __mode = "v" })

local function get_expensive_object(id)
    if object_cache[id] then
        return object_cache[id]
    end
    local obj = { data = "Heavy Data for " .. id }
    object_cache[id] = obj  -- 缓存对象
    return obj
end

local ref = get_expensive_object("session_1")
print(object_cache["session_1"] ~= nil)  -- true（外部 ref 变量强引用持有）

ref = nil  -- 释放外部强引用
collectgarbage("collect")  -- 强制触发垃圾回收

-- 弱值生效：由于没有任何外部强引用，该对象被垃圾回收器彻底清除，缓存项自动消失！
print(object_cache["session_1"] == nil)  -- true（杜绝了常驻字典导致的内存缓慢泄漏）`,
          ],
          [
            "lua_op_overload",
            `local Vector2D = {}
Vector2D.__index = Vector2D

function Vector2D.new(x, y)
    return setmetatable({ x = x, y = y }, Vector2D)
end

-- 运算符重载元方法：
-- 算术：__add (+), __sub (-), __mul (*), __div (/), __mod (%), __pow (^), __unm (一元负号 -)
-- 关系：__eq (==), __lt (<), __le (<=)；注意：a > b 会自动等价转换为 b < a
-- 字符串与转换：__concat (..), __tostring
function Vector2D.__add(a, b)
    return Vector2D.new(a.x + b.x, a.y + b.y)
end

function Vector2D.__eq(a, b)
    return a.x == b.x and a.y == b.y
end

function Vector2D.__tostring(self)
    return string.format("Vector2D(%.1f, %.1f)", self.x, self.y)
end

local v1 = Vector2D.new(1, 2)
local v2 = Vector2D.new(3, 4)
local v3 = v1 + v2
print(tostring(v3))  -- 输出: Vector2D(4.0, 6.0)
print(v3 == Vector2D.new(4, 6)) -- 输出: true`,
          ],
          [
            "lua_oop",
            `-- 基于原型委托（Prototype Delegation）的面向对象实现：
local BaseUnit = {}
BaseUnit.__index = BaseUnit

function BaseUnit.new(name, hp)
    local self = setmetatable({}, BaseUnit)
    self.name = name
    self.hp = hp
    return self
end

-- 冒号语法糖：obj:method(arg) 等价于 obj.method(obj, arg)
function BaseUnit:take_damage(dmg)
    self.hp = self.hp - dmg
    print(self.name .. " 受到伤害，剩余生命: " .. self.hp)
end

-- 原型链继承：创建派生类元表，并将其 __index 回退指向父类
local Hero = setmetatable({}, { __index = BaseUnit })
Hero.__index = Hero

function Hero.new(name, hp, mana)
    local self = setmetatable(BaseUnit.new(name, hp), Hero)
    self.mana = mana
    return self
end

local paladin = Hero.new("Uther", 200, 100)
paladin:take_damage(50)  -- 沿原型链检索到父类 BaseUnit 的方法并执行`,
          ],
        ],
      ],
      [
        "lua_coroutines",
        [
          [
            "lua_coroutines",
            `-- 协作式协程（用户态微纤程，无操作系统抢占与内核上下文切换损耗）：
-- 四大运行状态：suspended（挂起）、running（运行中）、normal（恢复者）、dead（已终止）

local co = coroutine.create(function(init_val)
    print("1. 协程启动，接收首次 resume 参数:", init_val)

    -- 挂起当前协程并出让执行权；yield 参数将原样返回给外部的 resume：
    local resume_val = coroutine.yield("WAITING_STEP2")
    print("2. 协程唤醒，接收后续 resume 注入的参数:", resume_val)

    return "WORK_FINISHED"
end)

print(coroutine.status(co))  -- "suspended"

-- 首次启动：参数通过 resume 直达协程函数入参
local ok1, ret1 = coroutine.resume(co, "Step1_Data")
print("外部接收 yield 产出值:", ret1)  -- 输出: WAITING_STEP2

-- 再次恢复：此时 resume 的参数将直接作为内层 coroutine.yield() 的返回值！
local ok2, ret2 = coroutine.resume(co, "Step2_Data")
print("外部接收 return 终结值:", ret2) -- 输出: WORK_FINISHED

print(coroutine.status(co))  -- "dead"`,
          ],
          [
            "lua_generator",
            `-- coroutine.wrap：将协程封装为标准 Lua 迭代器闭包函数：
-- 对比 coroutine.create：wrap 返回普通调用函数，省去 resume 的布尔状态解构，遇到错误直接抛出
local function fibonacci_stream(limit)
    return coroutine.wrap(function()
        local a, b = 0, 1
        while a <= limit do
            coroutine.yield(a)  -- 将当前项流式产出
            a, b = b, a + b
        end
    end)
end

-- 以同步语法编写，享受流式惰性求值的轻量迭代：
for num in fibonacci_stream(30) do
    print("Fib:", num)
end
-- 依次输出: 0, 1, 1, 2, 3, 5, 8, 13, 21`,
          ],
        ],
      ],
    ],
    LUAU_GROUP_LI = [
      [
        "luau_syntax",
        [
          [
            "compound",
            `local total = 100
total += 25    -- 125
total -= 5     -- 120
total *= 2     -- 240
total /= 4     -- 60
total //= 7    -- 8（向下整除复合赋值，底层调用 __idiv）
total %= 5     -- 3
total ^= 2     -- 9
local title = "Luau"
title ..= " v0.600" -- "Luau v0.600"

-- 关键单次求值安全保障：复杂左侧表达式中的函数调用严格仅求值一次
local eval_times = 0
local function get_slot()
    eval_times += 1
    return 1
end

local arr = { [1] = 10 }
arr[get_slot()] += 5
print(arr[1])       -- 输出: 15
print(eval_times)   -- 输出: 1（杜绝展开为 x = x + 1 带来的重复副作用！）`,
          ],
          [
            "number_lit",
            `local decimal = 1048576
local with_separator = 1_048_576       -- 支持下划线千分位分隔符
local hex_val = 0xFFFF_00AA           -- 十六进制字面量
local bin_val = 0b1010_0101           -- 二进制字面量
local float_val = 3.14159e-2          -- 科学计数法
-- Luau 数值均为 64 位 IEEE754 双精度浮点（最高精确表示 2^53 整数）`,
          ],
          [
            "string_escape",
            `local single = '单引号文本'
local double = "双引号文本"
local raw_block = [[保留原始换行
  与空格排版，无需转义]]

local hex_char = "\\x48\\x65\\x6C\\x6C\\x6F"   -- "Hello"（十六进制字符编码）
local unicode_fire = "\\u{1F525}"           -- "🔥"（UTF-8 Unicode 码点编码）
local folded = "行尾反斜杠 z 可以\\z
                消除跨行产生的全部缩进和空白换行"`,
          ],
          [
            "interp",
            `local user = "Elena"
local score = 98.5

-- 基础插值：反引号内嵌 {expr}，支持任意运行时表达式
print(\`用户: {user}, 成绩: {score + 1.5}\`)

-- 多行插值与转义规则：
-- 1. 字符串字面量部分换行需使用行末反斜杠 \\ 或 \\n 或 \\z：
local query = \`SELECT * FROM users \\
WHERE name = '{user}' \\
ORDER BY score DESC\`

-- 2. { ... } 表达式内部则可自由换行编写复杂逻辑：
print(\`状态: {
    if score >= 60 then "及格" else "不及格"
}!\`)

-- 3. 特殊字符转义：\\{ 表达字面量左括号，\\\` 表达反引号，\\\\ 表达反斜杠
print(\`转义展示: \\{user\\} -> \\\`{user}\\\`\`)

-- 语法约束警示：
-- ! 禁止双花括号 {{ 作为转义（Luau 显式抛出语法错误 BrokenInterpDoubleBrace）
-- ! 反引号字符串传参必须加括号：print(\`hello\`) 合法，print\`hello\` 为非法语法
-- ! 反引号不可用于类型注解：type T = \`id\` 为非法语法（字面量类型仅支持 'id' / "id"）`,
          ],
          [
            "const_binding",
            `-- const 局部变量绑定：声明后禁止被重新赋值（绑定不可变性）
const max_retries = 3
-- max_retries = 5     -- 静态类型检查报错: cannot assign to a const variable
-- max_retries += 1    -- 同样禁止复合赋值！

-- const 函数声明：
const function calculate_hash(data: string)
    return #data * 31
end

-- 上下文关键字（Contextual Keyword）：
-- const 仅在 local 语句有效位置被识别为关键字，完全向后兼容旧代码：
local const = 42  -- 依然完全合法

-- 核心技术辨析：绑定不可变 (Binding) vs 值不可变 (Value)
-- const 锁死的是变量名引用的指向，但并不自动阻止内部数据结构的篡改：
const options = { timeout = 30 }
options.timeout = 60  -- 合法：允许修改表内部字段
-- options = {}       -- 报错：禁止重定向 const 变量绑定！

-- 真正彻底的值只读不可变，需配合 table.freeze 联合使用：
const frozen_options = table.freeze({ timeout = 30 })
-- frozen_options.timeout = 60 -- 运行时抛错: attempt to modify a readonly table`,
          ],
          [
            "control_flow",
            `local count = 25
if count > 50 then
    print("high")
elseif count > 20 then
    print("medium")
else
    print("low")
end

for i = 1, 10 do
    if i % 2 == 0 then
        continue  -- 跳至循环步进点，进入下一轮迭代
    end
    print(i)
end

-- 注意：repeat..until 中，若 until 条件依赖循环体内部 local 变量，
-- continue 禁止跳过该局部变量的声明处：
local n = 0
repeat
    n += 1
    local done = (n >= 3)
until done`,
          ],
          [
            "ifexpr",
            `local score = 85

-- 表达式级三元条件分支：else 子句为强制必选
local grade = if score >= 90 then "A"
              elseif score >= 80 then "B"
              else "C"

-- 解决传统 Lua 5.1 中 "cond and a or b" 当 a 为 false/nil 时的静默求值缺陷：
local flag = false
local res = if true then flag else "default"  -- 正确求值为 false，绝不误退回`,
          ],
          [
            "iter",
            `local fruits = { "apple", "banana", "cherry", special = "durian" }

-- 通用迭代：无需调用 pairs 或 ipairs，直接迭代任意表
-- 遍历顺序：先按 1..#t 顺序遍历连续整数索引，再遍历哈希键
for key, item in fruits do
    print(\`[{key}]: {item}\`)
end

-- 自定义结构迭代：通过 __iter 元方法定制迭代器生成器
local Range = { from = 1, to = 3 }
setmetatable(Range, {
    __iter = function(self)
        local cur = self.from - 1
        return function()
            cur += 1
            if cur <= self.to then return cur end
            return nil
        end
    end
})

for val in Range do print(val) end  -- 输出 1, 2, 3`,
          ],
        ],
      ],
      [
        "luau_types",
        [
          [
            "type_modes",
            `-- 在 Luau 文件第一行通过脚本注释指令指定类型检查器模式：

--!strict
-- 严格模式（推荐）：对所有未标注变量执行全局双向类型推断，拦截所有类型不匹配

--!nonstrict
-- 非严格模式（默认）：对未显式标注变量宽容推断为 any，适合旧脚本兼容

--!nocheck
-- 禁用模式：彻底关闭静态类型分析引擎，仅做语法解析

--!native
-- 原生编译指令：指导 Luau AOT/JIT 编译器将整份脚本直接生成为原生机器码`,
          ],
          [
            "type_annot",
            `-- 基础标量类型：
local n: number = 3.14
local s: string = "hello"
local b: boolean = true
local t: thread = coroutine.running()
local buf: buffer = buffer.create(16)
local vec: vector = vector.create(1, 2, 3)

-- 顶级类型与底层类型：
local dyn: any = "any 放弃静态类型推断，允许任意动态操作"
local unk: unknown = 42
-- local x: number = unk  -- 报错：unknown 必须经断言或类型收窄才可赋值
local safe: number = unk :: number

local function fail(): never
    error("永不正常返回的底层类型")
end`,
          ],
          [
            "func_types",
            `-- 函数签名定义：支持形参文档提示名与多返回值声明
type MathFn = (x: number, y: number) -> (boolean, number)
type RequestCallback = (statusCode: number, payload: string) -> ()

-- 无返回值以 () 显式标注：
local function run_task(cb: (step: number) -> ()): ()
    cb(1)
end`,
          ],
          [
            "union_inter",
            `-- 联合类型 (|)：表示可容纳其中任一分支
type Status = "pending" | "running" | "done"
type Config = { timeout: number? }  -- 可空语法等价于 number | nil

-- 交叉类型 (&)：组合多个表结构或定义重载函数签名
type HasId = { id: string }
type HasName = { name: string }
type Entity = HasId & HasName  -- 拥有两个表的全部字段

-- 函数重载签名：
type MultiResolver = ((id: number) -> string) & ((tag: string) -> number)`,
          ],
          [
            "casts",
            `-- :: 类型断言转换
local raw_data: any = "Luau 0.600"
local title = (raw_data :: string):upper()

-- typeof() 编译期类型提取：在静态分析期反射表达式的推导结构
local default_settings = {
    debug = false,
    retry_limit = 3,
    endpoint = "https://api.domain.internal",
}
type Settings = typeof(default_settings)

local custom: Settings = {
    debug = true,
    retry_limit = 5,
    endpoint = "https://staging.domain.internal",
}`,
          ],
        ],
      ],
      [
        "luau_tables",
        [
          [
            "tables_types",
            `type UserProfile = {
    read id: string,         -- 只读属性：禁止向该字段写入新值
    write password: string,   -- 只写属性：允许写入但禁止外部读取
    name: string,
    tags: { string },        -- 数组简写语法，等价于 {[number]: string}
    read [string]: any,      -- 只读索引签名
}

local user: UserProfile = {
    id = "usr_001",
    password = "secret",
    name = "Alice",
    tags = { "dev", "sys" },
}
-- user.id = "usr_002"       -- 编译错误：只读属性不可修改
-- print(user.password)      -- 编译错误：只写属性不可读取`,
          ],
          [
            "table_states",
            `-- 1. 未密封表（Unsealed Table）：表字面量创建，允许动态扩展新属性
local t = { x = 1 } -- 初始推导为未密封表 { x: number }
t.y = 2             -- 合法：动态扩展字段，类型收敛为 { x: number, y: number }

-- 2. 作用域退出即密封：离开创建函数/作用域后，未密封表自动封口
local function make_point()
    local p = {}
    p.x, p.y = 10, 20
    return p        -- 离开作用域，sealed 密封
end
local pt = make_point()
-- pt.z = 30        -- 静态类型检查报错：无法向密封表添加新字段 'z'

-- 3. 显式注解立即密封与宽度子类型（Width Subtyping）：
type Named = { name: string }
type Employee = { name: string, id: number }

local emp: Employee = { name = "Bob", id = 101 }
local person: Named = emp -- 合法：密封表支持宽度子类型（字段超集兼容子集）`,
          ],
          [
            "refinements",
            `-- 标签联合（Tagged Unions）与类型细化：
type Success<T> = { status: "ok", data: T }
type Failure<E> = { status: "err", error: E }
type Result<T, E> = Success<T> | Failure<E>

local function handle(res: Result<number, string>)
    -- 1. 字面量标签分支细化：
    if res.status == "ok" then
        print(\`Success: {res.data * 2}\`)  -- 自动细化为 Success<number>
    else
        print(\`Error: {res.error}\`)      -- 自动细化为 Failure<string>
    end
end

-- 2. 内置守卫与 assert 细化：
local function parse(val: string?)
    assert(val ~= nil, "val 必须非空")
    -- 经 assert 断言后，val 自动收窄为 string
    print(val:upper())
end`,
          ],
          [
            "tables_freeze",
            `-- 深度只读冻结：防止运行时意外修改或向表注入新键
const config = table.freeze({ version = 1, port = 8080 })

-- config.port = 9000         -- 运行时抛错：attempt to modify a readonly table
print(table.isfrozen(config)) -- true

-- const 约束绑定本身，配合 table.freeze 实现值不可变：
-- config = {}                -- 编译错误：无法重定向 const 常量引用`,
          ],
        ],
      ],
      [
        "luau_generics",
        [
          [
            "generics",
            `-- 泛型类型别名与默认类型实参
export type Pair<T, U = T> = { first: T, second: U }

-- 泛型函数：参数化类型约束
local function swap<T, U>(p: Pair<T, U>): Pair<U, T>
    return { first = p.second, second = p.first }
end

local p1: Pair<number, string> = { first = 1, second = "one" }
local p2: Pair<boolean> = { first = true, second = false } -- 使用默认类型 U = boolean`,
          ],
          [
            "instantiate",
            `-- 显式泛型实例化 (AstExprInstantiate / <<...>>)：
-- 在函数或方法调用时通过 <<Type>> 显式传递类型实参，解决类型推断歧义

local function make_list<T>(): { T }
    return {}
end

-- 若不显式实例化，编译器因缺乏入参无法推断 T 的具体类型：
local num_list = make_list<<number>>()   -- 明确实例化为 { number }
local str_list = make_list<<string>>()   -- 明确实例化为 { string }

local function identity<T>(x: T): T return x end
local admin = identity<<"admin">>("admin") -- 强制指定为字面量单例类型 "admin"

-- 运行时语义：实例化语法在 ulua-compiler 中直接做类型擦除，零性能负担`,
          ],
          [
            "type_packs",
            `-- T... 表示任意长度的泛型类型包（如 (string, number, boolean)）
type Handler<T...> = (T...) -> ()

local function run_forward<T...>(handler: Handler<T...>, ...: T...)
    print("Forwarding arguments...")
    handler(...)
end

run_forward(function(tag: string, id: number)
    print(tag, id)
end, "event_start", 42)

-- 对比：...number 表示同构可变长参数（全部元素皆为 number）
type SumFn = (...number) -> number`,
          ],
        ],
      ],
      [
        "luau_host",
        [
          [
            "oop",
            `local Account = {}
Account.__index = Account

type AccountData = { balance: number }
export type Account = typeof(setmetatable({} :: AccountData, Account))

function Account.new(initial: number): Account
    return setmetatable({ balance = initial }, Account)
end

-- 显式注解 self 接收者类型：
function Account:deposit(amount: number)
    self.balance += amount
end

function Account:withdraw(amount: number): boolean
    if self.balance >= amount then
        self.balance -= amount
        return true
    end
    return false
end

local acc = Account.new(100)
acc:deposit(50)`,
          ],
          [
            "modules_export",
            `-- 模块默认局部隔离：未加 export 的类型和变量仅在当前文件可见
type InternalHelper = { secret_key: string }

-- export type：将类型暴露给外部模块消费者
export type Account = { id: string, balance: number }

-- 消费者引用方式：
-- local M = require("./account")
-- local user: M.Account = { id = "001", balance = 500 }

-- export local / export function（新特性语法）：
-- 在支持值导出模式下，直接导出顶层符号，与顶层 return 互斥`,
          ],
          [
            "declare",
            `-- 外部声明契约（常用于 .d.luau 类型声明文件或嵌入式宿主绑定）：
-- 在类型定义环境中，描述宿主（如 ulua Rust 运行时）注入的全局 API

-- 1. 全局 API 与接口定义：
type HostGlobal = { [string]: any }
type MathAPI = { abs: (number) -> number, floor: (number) -> number }

-- 2. 宿主类型与类方法契约：
type EngineNode = {
    read id: string,
    write is_active: boolean,
    destroy: (self: EngineNode) -> (),
}

-- 3. 在声明文件 (.d.luau) 中的典型宿主绑定写法：
-- declare _G: HostGlobal
-- declare function print<T...>(...: T...): ()
-- declare math: MathAPI`,
          ],
        ],
      ],
      [
        "luau_native",
        [
          [
            "vector",
            `-- vector 原生高阶类型：3D/4D 浮点向量，直接映射 CPU SIMD 寄存器加速
local v1 = vector.create(1.0, 2.0, 3.0)
local v2 = vector.create(4.0, 5.0, 6.0)

local v_sum = v1 + v2       -- (5.0, 7.0, 9.0)
local v_scaled = v1 * 3.0   -- (3.0, 6.0, 9.0)
local v_div = v1 // 2.0     -- (0.0, 1.0, 1.0) 逐分量向下整除

-- 点积运算与分量访问：
local dot = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
print(\`x: {v_sum.x}, dot: {dot}\`)
-- 无垃圾回收（零 GC）压力，性能远胜普通 Lua 表模拟的向量`,
          ],
          [
            "buffer",
            `-- buffer：原生底层连续内存块，用于极速二进制序列化与网络解包
local b = buffer.create(16)

-- 整型与浮点精准读写：writeu8, writei16, writeu32, writef32, writef64
buffer.writeu8(b, 0, 0xEF)
buffer.writeu16(b, 2, 0x1234)
buffer.writef32(b, 4, 3.14159)

-- 字符串内存读写与拷贝：
buffer.writestring(b, 8, "ulua")
local tag = buffer.readstring(b, 8, 4)

-- 内存快速复制与填充：
local b2 = buffer.create(16)
buffer.copy(b2, 0, b, 0, 16)
buffer.fill(b, 0, 0, 16) -- 清零

print(\`read string: {tag}, read f32: {buffer.readf32(b2, 4)}\`)`,
          ],
          [
            "attr",
            `-- @native：强制 JIT/AOT 编译器为该函数生成本地机器码
-- 注意：@native 不递归作用于内部嵌套的闭包函数，内层闭包必须显式标注
@native
local function mandelbrot(x: number, y: number): number
    local cr, ci = y - 0.5, x - 0.5
    local zr, zi = cr, ci
    for i = 1, 100 do
        local r2, i2 = zr * zr, zi * zi
        if r2 + i2 > 4.0 then return i end
        zi = 2.0 * zr * zi + ci
        zr = r2 - i2 + cr
    end
    return 0
end

-- @deprecated：标记废弃，支持参数化提示 use 与 reason
@[deprecated { use = "new_encrypt_pipeline()", reason = "MD5 已不再安全" }]
local function legacy_md5(data: string) end

-- 属性组合与表达式级函数属性：
local fast_op = @native function(a: number) return a * 2 end`,
          ],
          [
            "type_func",
            `-- type function：在编译器静态分析阶段执行的元编程类型函数
-- 运行在类型检查器嵌入环境中，入参与返回值皆为 type 实例

type function keyof(ty)
    if not ty:is("table") then error("必须为表类型") end
    local union = nil
    for prop in ty:properties() do
        union = if union then types.unionof(union, prop) else prop
    end
    return union or types.singleton(nil)
end

type function make_readonly(ty)
    if not ty:is("table") then return ty end
    local result = types.newtable()
    for prop, spec in ty:properties() do
        result:setreadproperty(prop, spec.read or spec.write)
    end
    return result
end

type Config = { timeout: number, retries: number }
type ConfigKeys = keyof<Config>       -- "timeout" | "retries"
type Immutable = make_readonly<Config> -- { read timeout: number, read retries: number }`,
          ],
        ],
      ],
    ];

  let open_map = $state({});

  const LUA_KEY_LI = LUA_GROUP_LI.flatMap(([, item_li]) => item_li.map(([k]) => k)),
    LUAU_KEY_LI = LUAU_GROUP_LI.flatMap(([, item_li]) => item_li.map(([k]) => k)),
    keyToggle = (key) => {
      open_map[key] = !open_map[key];
    },
    luaAllOpenCheck = () => LUA_KEY_LI.every((key) => !!open_map[key]),
    luaAllToggle = () => {
      const next_state = !luaAllOpenCheck();
      LUA_KEY_LI.forEach((key) => {
        open_map[key] = next_state;
      });
    },
    luauAllOpenCheck = () => LUAU_KEY_LI.every((key) => !!open_map[key]),
    luauAllToggle = () => {
      const next_state = !luauAllOpenCheck();
      LUAU_KEY_LI.forEach((key) => {
        open_map[key] = next_state;
      });
    };
</script>

<template lang="pug">
section#lua-syntax.syntax
  .wrap
    .syntax-head
      .syntax-head-row
        h2.syntax-main-title {t("syntax.lua_title")}
        button.btn.btn-secondary.syntax-toggle-btn(type="button" onclick={luaAllToggle})
          span {luaAllOpenCheck() ? t("syntax.collapse_all") : t("syntax.expand_all")}

    .syntax-intro-box
      p.syntax-intro-desc {t("syntax.lua_intro")}

    .syntax-groups
      +each('LUA_GROUP_LI as [group_id, item_li]')
        .syntax-group
          h3.syntax-group-title {t("syntax." + group_id)}
          .syntax-grid
            +each('item_li as [key, code]')
              .syntax-card(class:open={open_map[key]})
                button.syntax-header(
                  type="button"
                  onclick={() => keyToggle(key)}
                  aria-expanded={open_map[key] ? 'true' : 'false'}
                )
                  h4.syntax-title {t("syntax." + key + "_title")}
                  img.syntax-arrow(src={chevron_svg} width="14" height="14" alt="")

                +if('open_map[key]')
                  .syntax-body
                    p.syntax-desc {t("syntax." + key + "_desc")}
                    pre.code-pane
                      code {code}

section#luau-syntax.syntax.syntax-alt
  .wrap
    .syntax-head
      .syntax-head-row
        h2.syntax-main-title {t("syntax.luau_title")}
        button.btn.btn-secondary.syntax-toggle-btn(type="button" onclick={luauAllToggle})
          span {luauAllOpenCheck() ? t("syntax.collapse_all") : t("syntax.expand_all")}

    .syntax-intro-box
      p.syntax-intro-desc {t("syntax.luau_intro")}

    .syntax-groups
      +each('LUAU_GROUP_LI as [group_id, item_li]')
        .syntax-group
          h3.syntax-group-title {t("syntax." + group_id)}
          .syntax-grid
            +each('item_li as [key, code]')
              .syntax-card(class:open={open_map[key]})
                button.syntax-header(
                  type="button"
                  onclick={() => keyToggle(key)}
                  aria-expanded={open_map[key] ? 'true' : 'false'}
                )
                  h4.syntax-title {t("syntax." + key + "_title")}
                  img.syntax-arrow(src={chevron_svg} width="14" height="14" alt="")

                +if('open_map[key]')
                  .syntax-body
                    p.syntax-desc {t("syntax." + key + "_desc")}
                    pre.code-pane
                      code {code}
</template>

<style lang="stylus">
.syntax
  padding 56px 0 64px
  background #ffffff
  border-top 1px solid #eaeef2

.syntax-alt
  background #fafbfc

.syntax-head
  width 100%
  text-align left
  margin-bottom 20px

.syntax-head-row
  display flex
  align-items center
  justify-content space-between
  flex-wrap wrap
  gap 16px
  width 100%

.syntax-main-title
  font-size 28px
  font-weight 800
  color #1f2328
  letter-spacing -0.02em
  line-height 1.2
  margin 0

@media (max-width 480px)
  .syntax-main-title
    font-size 22px

.syntax-toggle-btn
  height 32px
  align-self center
  flex-shrink 0
  color #0969da !important

.syntax-intro-box
  background #f8fafc
  border 1px solid #e2e8f0
  border-left 4px solid #0969da
  border-radius 8px
  padding 14px 18px
  margin-bottom 28px
  display flex
  flex-direction column
  gap 6px

.syntax-intro-desc
  font-size 13.5px
  color #475569
  line-height 1.65
  margin 0

.syntax-groups
  display flex
  flex-direction column
  gap 36px
  width 100%

.syntax-group
  display flex
  flex-direction column
  width 100%

.syntax-group-title
  font-size 16px
  font-weight 700
  color #1f2328
  margin 0 0 14px
  letter-spacing -0.01em
  display flex
  align-items center
  gap 8px
  &::before
    content ""
    display inline-block
    width 6px
    height 6px
    border-radius 50%
    background #0969da

.syntax-grid
  display flex
  flex-direction column
  gap 10px
  width 100%

.syntax-card
  background #ffffff
  border 1px solid #d8dee4
  border-radius 10px
  overflow hidden
  transition border-color 0.15s ease, box-shadow 0.15s ease
  &:hover
    border-color #b6c2cf
  &.open
    border-color #0969da
    box-shadow 0 3px 12px rgba(9, 105, 218, 0.08)
    .syntax-arrow
      transform rotate(180deg)

.syntax-header
  width 100%
  display flex
  align-items center
  justify-content space-between
  padding 13px 18px
  background #ffffff
  border none
  cursor pointer
  text-align left
  transition background 0.15s ease
  &:hover
    background #f6f8fa

.syntax-title
  font-size 14.5px
  font-weight 600
  color #1f2328
  margin 0
  line-height 1.4

.syntax-arrow
  flex-shrink 0
  margin-left 12px
  transition transform 0.2s cubic-bezier(0.16, 1, 0.3, 1)
  opacity 0.7

.syntax-body
  padding 14px 18px 18px
  background #fafbfc
  border-top 1px solid #f0f2f5

.syntax-desc
  font-size 13px
  color #57606a
  line-height 1.6
  margin 0 0 10px

.code-pane
  margin 0
  padding 14px 16px
  background #f6f8fa
  border 1px solid #e1e4e8
  border-radius 8px
  overflow-x auto
  font-family 'JetBrains Mono', monospace
  font-size 13px
  line-height 1.6
  color #1f2328
  white-space pre
</style>
