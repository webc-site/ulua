--!strict
-- 生成物：重类型 Luau 库（benchmarks/fixtures/generate.mjs，勿手改）
export type Vec2 = { x: number, y: number }
export type Color = "red" | "green" | "blue" | "warm" | "cold"
export type Event<T> = { kind: string, at: number, payload: T }
export type Listener<T> = (T) -> ()
export type Filter<T, U> = (T, U) -> boolean
export type Store<S> = {
  get: () -> S,
  set: (S) -> (),
  subscribe: (Listener<S>) -> () -> (),
}

export type Marker = { name: string, rank: number, tags: { string } }

local function label(m: Marker): string
  return m.name .. "#" .. tostring(m.rank)
end

-- 区块 0
export type Config0 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result0 = { ok: boolean, value: number, events: { Event<Config0> } }
local function build0(seed: number): Config0
  local rows: Config0 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce0<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score0(cfg: Config0): Result0
  local total = reduce0(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m0", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 199,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store0: Store<Result0> = (function()
  local current: Result0 = score0(build0(0))
  local listeners: { Listener<Result0> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered0 = if store0.get().ok then store0.get().value else -store0.get().value
local _lifted0 = if typeof(filtered0) == "number" then filtered0 * 2 else 0

-- 区块 1
export type Config1 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result1 = { ok: boolean, value: number, events: { Event<Config1> } }
local function build1(seed: number): Config1
  local rows: Config1 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce1<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score1(cfg: Config1): Result1
  local total = reduce1(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m1", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 278,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store1: Store<Result1> = (function()
  local current: Result1 = score1(build1(1))
  local listeners: { Listener<Result1> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered1 = if store1.get().ok then store1.get().value else -store1.get().value
local _lifted1 = if typeof(filtered1) == "number" then filtered1 * 2 else 0

-- 区块 2
export type Config2 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result2 = { ok: boolean, value: number, events: { Event<Config2> } }
local function build2(seed: number): Config2
  local rows: Config2 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce2<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score2(cfg: Config2): Result2
  local total = reduce2(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m2", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 196,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store2: Store<Result2> = (function()
  local current: Result2 = score2(build2(2))
  local listeners: { Listener<Result2> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered2 = if store2.get().ok then store2.get().value else -store2.get().value
local _lifted2 = if typeof(filtered2) == "number" then filtered2 * 2 else 0

-- 区块 3
export type Config3 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result3 = { ok: boolean, value: number, events: { Event<Config3> } }
local function build3(seed: number): Config3
  local rows: Config3 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce3<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score3(cfg: Config3): Result3
  local total = reduce3(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m3", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 351,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store3: Store<Result3> = (function()
  local current: Result3 = score3(build3(3))
  local listeners: { Listener<Result3> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered3 = if store3.get().ok then store3.get().value else -store3.get().value
local _lifted3 = if typeof(filtered3) == "number" then filtered3 * 2 else 0

-- 区块 4
export type Config4 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result4 = { ok: boolean, value: number, events: { Event<Config4> } }
local function build4(seed: number): Config4
  local rows: Config4 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce4<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score4(cfg: Config4): Result4
  local total = reduce4(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m4", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 125,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store4: Store<Result4> = (function()
  local current: Result4 = score4(build4(4))
  local listeners: { Listener<Result4> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered4 = if store4.get().ok then store4.get().value else -store4.get().value
local _lifted4 = if typeof(filtered4) == "number" then filtered4 * 2 else 0

-- 区块 5
export type Config5 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result5 = { ok: boolean, value: number, events: { Event<Config5> } }
local function build5(seed: number): Config5
  local rows: Config5 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce5<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score5(cfg: Config5): Result5
  local total = reduce5(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m5", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 359,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store5: Store<Result5> = (function()
  local current: Result5 = score5(build5(5))
  local listeners: { Listener<Result5> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered5 = if store5.get().ok then store5.get().value else -store5.get().value
local _lifted5 = if typeof(filtered5) == "number" then filtered5 * 2 else 0

-- 区块 6
export type Config6 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result6 = { ok: boolean, value: number, events: { Event<Config6> } }
local function build6(seed: number): Config6
  local rows: Config6 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce6<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score6(cfg: Config6): Result6
  local total = reduce6(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m6", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 137,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store6: Store<Result6> = (function()
  local current: Result6 = score6(build6(6))
  local listeners: { Listener<Result6> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered6 = if store6.get().ok then store6.get().value else -store6.get().value
local _lifted6 = if typeof(filtered6) == "number" then filtered6 * 2 else 0

-- 区块 7
export type Config7 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result7 = { ok: boolean, value: number, events: { Event<Config7> } }
local function build7(seed: number): Config7
  local rows: Config7 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce7<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score7(cfg: Config7): Result7
  local total = reduce7(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m7", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 424,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store7: Store<Result7> = (function()
  local current: Result7 = score7(build7(7))
  local listeners: { Listener<Result7> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered7 = if store7.get().ok then store7.get().value else -store7.get().value
local _lifted7 = if typeof(filtered7) == "number" then filtered7 * 2 else 0

-- 区块 8
export type Config8 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result8 = { ok: boolean, value: number, events: { Event<Config8> } }
local function build8(seed: number): Config8
  local rows: Config8 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce8<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score8(cfg: Config8): Result8
  local total = reduce8(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m8", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 494,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store8: Store<Result8> = (function()
  local current: Result8 = score8(build8(8))
  local listeners: { Listener<Result8> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered8 = if store8.get().ok then store8.get().value else -store8.get().value
local _lifted8 = if typeof(filtered8) == "number" then filtered8 * 2 else 0

-- 区块 9
export type Config9 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result9 = { ok: boolean, value: number, events: { Event<Config9> } }
local function build9(seed: number): Config9
  local rows: Config9 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce9<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score9(cfg: Config9): Result9
  local total = reduce9(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m9", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 497,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store9: Store<Result9> = (function()
  local current: Result9 = score9(build9(9))
  local listeners: { Listener<Result9> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered9 = if store9.get().ok then store9.get().value else -store9.get().value
local _lifted9 = if typeof(filtered9) == "number" then filtered9 * 2 else 0

-- 区块 10
export type Config10 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result10 = { ok: boolean, value: number, events: { Event<Config10> } }
local function build10(seed: number): Config10
  local rows: Config10 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce10<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score10(cfg: Config10): Result10
  local total = reduce10(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m10", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 300,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store10: Store<Result10> = (function()
  local current: Result10 = score10(build10(10))
  local listeners: { Listener<Result10> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered10 = if store10.get().ok then store10.get().value else -store10.get().value
local _lifted10 = if typeof(filtered10) == "number" then filtered10 * 2 else 0

-- 区块 11
export type Config11 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result11 = { ok: boolean, value: number, events: { Event<Config11> } }
local function build11(seed: number): Config11
  local rows: Config11 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce11<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score11(cfg: Config11): Result11
  local total = reduce11(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m11", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 183,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store11: Store<Result11> = (function()
  local current: Result11 = score11(build11(11))
  local listeners: { Listener<Result11> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered11 = if store11.get().ok then store11.get().value else -store11.get().value
local _lifted11 = if typeof(filtered11) == "number" then filtered11 * 2 else 0

-- 区块 12
export type Config12 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result12 = { ok: boolean, value: number, events: { Event<Config12> } }
local function build12(seed: number): Config12
  local rows: Config12 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce12<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score12(cfg: Config12): Result12
  local total = reduce12(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m12", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 186,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store12: Store<Result12> = (function()
  local current: Result12 = score12(build12(12))
  local listeners: { Listener<Result12> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered12 = if store12.get().ok then store12.get().value else -store12.get().value
local _lifted12 = if typeof(filtered12) == "number" then filtered12 * 2 else 0

-- 区块 13
export type Config13 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result13 = { ok: boolean, value: number, events: { Event<Config13> } }
local function build13(seed: number): Config13
  local rows: Config13 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce13<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score13(cfg: Config13): Result13
  local total = reduce13(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m13", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 181,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store13: Store<Result13> = (function()
  local current: Result13 = score13(build13(13))
  local listeners: { Listener<Result13> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered13 = if store13.get().ok then store13.get().value else -store13.get().value
local _lifted13 = if typeof(filtered13) == "number" then filtered13 * 2 else 0

-- 区块 14
export type Config14 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result14 = { ok: boolean, value: number, events: { Event<Config14> } }
local function build14(seed: number): Config14
  local rows: Config14 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce14<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score14(cfg: Config14): Result14
  local total = reduce14(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m14", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 469,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store14: Store<Result14> = (function()
  local current: Result14 = score14(build14(14))
  local listeners: { Listener<Result14> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered14 = if store14.get().ok then store14.get().value else -store14.get().value
local _lifted14 = if typeof(filtered14) == "number" then filtered14 * 2 else 0

-- 区块 15
export type Config15 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result15 = { ok: boolean, value: number, events: { Event<Config15> } }
local function build15(seed: number): Config15
  local rows: Config15 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce15<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score15(cfg: Config15): Result15
  local total = reduce15(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m15", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 54,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store15: Store<Result15> = (function()
  local current: Result15 = score15(build15(15))
  local listeners: { Listener<Result15> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered15 = if store15.get().ok then store15.get().value else -store15.get().value
local _lifted15 = if typeof(filtered15) == "number" then filtered15 * 2 else 0

-- 区块 16
export type Config16 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result16 = { ok: boolean, value: number, events: { Event<Config16> } }
local function build16(seed: number): Config16
  local rows: Config16 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce16<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score16(cfg: Config16): Result16
  local total = reduce16(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m16", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 249,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store16: Store<Result16> = (function()
  local current: Result16 = score16(build16(16))
  local listeners: { Listener<Result16> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered16 = if store16.get().ok then store16.get().value else -store16.get().value
local _lifted16 = if typeof(filtered16) == "number" then filtered16 * 2 else 0

-- 区块 17
export type Config17 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result17 = { ok: boolean, value: number, events: { Event<Config17> } }
local function build17(seed: number): Config17
  local rows: Config17 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce17<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score17(cfg: Config17): Result17
  local total = reduce17(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m17", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 182,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store17: Store<Result17> = (function()
  local current: Result17 = score17(build17(17))
  local listeners: { Listener<Result17> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered17 = if store17.get().ok then store17.get().value else -store17.get().value
local _lifted17 = if typeof(filtered17) == "number" then filtered17 * 2 else 0

-- 区块 18
export type Config18 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result18 = { ok: boolean, value: number, events: { Event<Config18> } }
local function build18(seed: number): Config18
  local rows: Config18 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce18<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score18(cfg: Config18): Result18
  local total = reduce18(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m18", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 227,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store18: Store<Result18> = (function()
  local current: Result18 = score18(build18(18))
  local listeners: { Listener<Result18> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered18 = if store18.get().ok then store18.get().value else -store18.get().value
local _lifted18 = if typeof(filtered18) == "number" then filtered18 * 2 else 0

-- 区块 19
export type Config19 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result19 = { ok: boolean, value: number, events: { Event<Config19> } }
local function build19(seed: number): Config19
  local rows: Config19 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce19<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score19(cfg: Config19): Result19
  local total = reduce19(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m19", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 420,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store19: Store<Result19> = (function()
  local current: Result19 = score19(build19(19))
  local listeners: { Listener<Result19> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered19 = if store19.get().ok then store19.get().value else -store19.get().value
local _lifted19 = if typeof(filtered19) == "number" then filtered19 * 2 else 0

-- 区块 20
export type Config20 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result20 = { ok: boolean, value: number, events: { Event<Config20> } }
local function build20(seed: number): Config20
  local rows: Config20 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce20<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score20(cfg: Config20): Result20
  local total = reduce20(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m20", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 286,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store20: Store<Result20> = (function()
  local current: Result20 = score20(build20(20))
  local listeners: { Listener<Result20> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered20 = if store20.get().ok then store20.get().value else -store20.get().value
local _lifted20 = if typeof(filtered20) == "number" then filtered20 * 2 else 0

-- 区块 21
export type Config21 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result21 = { ok: boolean, value: number, events: { Event<Config21> } }
local function build21(seed: number): Config21
  local rows: Config21 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce21<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score21(cfg: Config21): Result21
  local total = reduce21(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m21", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 290,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store21: Store<Result21> = (function()
  local current: Result21 = score21(build21(21))
  local listeners: { Listener<Result21> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered21 = if store21.get().ok then store21.get().value else -store21.get().value
local _lifted21 = if typeof(filtered21) == "number" then filtered21 * 2 else 0

-- 区块 22
export type Config22 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result22 = { ok: boolean, value: number, events: { Event<Config22> } }
local function build22(seed: number): Config22
  local rows: Config22 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce22<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score22(cfg: Config22): Result22
  local total = reduce22(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m22", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 315,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store22: Store<Result22> = (function()
  local current: Result22 = score22(build22(22))
  local listeners: { Listener<Result22> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered22 = if store22.get().ok then store22.get().value else -store22.get().value
local _lifted22 = if typeof(filtered22) == "number" then filtered22 * 2 else 0

-- 区块 23
export type Config23 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result23 = { ok: boolean, value: number, events: { Event<Config23> } }
local function build23(seed: number): Config23
  local rows: Config23 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce23<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score23(cfg: Config23): Result23
  local total = reduce23(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m23", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 430,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store23: Store<Result23> = (function()
  local current: Result23 = score23(build23(23))
  local listeners: { Listener<Result23> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered23 = if store23.get().ok then store23.get().value else -store23.get().value
local _lifted23 = if typeof(filtered23) == "number" then filtered23 * 2 else 0

-- 区块 24
export type Config24 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result24 = { ok: boolean, value: number, events: { Event<Config24> } }
local function build24(seed: number): Config24
  local rows: Config24 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce24<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score24(cfg: Config24): Result24
  local total = reduce24(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m24", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 485,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store24: Store<Result24> = (function()
  local current: Result24 = score24(build24(24))
  local listeners: { Listener<Result24> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered24 = if store24.get().ok then store24.get().value else -store24.get().value
local _lifted24 = if typeof(filtered24) == "number" then filtered24 * 2 else 0

-- 区块 25
export type Config25 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result25 = { ok: boolean, value: number, events: { Event<Config25> } }
local function build25(seed: number): Config25
  local rows: Config25 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce25<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score25(cfg: Config25): Result25
  local total = reduce25(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m25", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 141,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store25: Store<Result25> = (function()
  local current: Result25 = score25(build25(25))
  local listeners: { Listener<Result25> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered25 = if store25.get().ok then store25.get().value else -store25.get().value
local _lifted25 = if typeof(filtered25) == "number" then filtered25 * 2 else 0

-- 区块 26
export type Config26 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result26 = { ok: boolean, value: number, events: { Event<Config26> } }
local function build26(seed: number): Config26
  local rows: Config26 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce26<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score26(cfg: Config26): Result26
  local total = reduce26(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m26", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 351,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store26: Store<Result26> = (function()
  local current: Result26 = score26(build26(26))
  local listeners: { Listener<Result26> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered26 = if store26.get().ok then store26.get().value else -store26.get().value
local _lifted26 = if typeof(filtered26) == "number" then filtered26 * 2 else 0

-- 区块 27
export type Config27 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result27 = { ok: boolean, value: number, events: { Event<Config27> } }
local function build27(seed: number): Config27
  local rows: Config27 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce27<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score27(cfg: Config27): Result27
  local total = reduce27(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m27", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 72,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store27: Store<Result27> = (function()
  local current: Result27 = score27(build27(27))
  local listeners: { Listener<Result27> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered27 = if store27.get().ok then store27.get().value else -store27.get().value
local _lifted27 = if typeof(filtered27) == "number" then filtered27 * 2 else 0

-- 区块 28
export type Config28 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result28 = { ok: boolean, value: number, events: { Event<Config28> } }
local function build28(seed: number): Config28
  local rows: Config28 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce28<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score28(cfg: Config28): Result28
  local total = reduce28(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m28", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 489,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store28: Store<Result28> = (function()
  local current: Result28 = score28(build28(28))
  local listeners: { Listener<Result28> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered28 = if store28.get().ok then store28.get().value else -store28.get().value
local _lifted28 = if typeof(filtered28) == "number" then filtered28 * 2 else 0

-- 区块 29
export type Config29 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result29 = { ok: boolean, value: number, events: { Event<Config29> } }
local function build29(seed: number): Config29
  local rows: Config29 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce29<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score29(cfg: Config29): Result29
  local total = reduce29(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m29", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 165,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store29: Store<Result29> = (function()
  local current: Result29 = score29(build29(29))
  local listeners: { Listener<Result29> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered29 = if store29.get().ok then store29.get().value else -store29.get().value
local _lifted29 = if typeof(filtered29) == "number" then filtered29 * 2 else 0

-- 区块 30
export type Config30 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result30 = { ok: boolean, value: number, events: { Event<Config30> } }
local function build30(seed: number): Config30
  local rows: Config30 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce30<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score30(cfg: Config30): Result30
  local total = reduce30(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m30", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 396,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store30: Store<Result30> = (function()
  local current: Result30 = score30(build30(30))
  local listeners: { Listener<Result30> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered30 = if store30.get().ok then store30.get().value else -store30.get().value
local _lifted30 = if typeof(filtered30) == "number" then filtered30 * 2 else 0

-- 区块 31
export type Config31 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result31 = { ok: boolean, value: number, events: { Event<Config31> } }
local function build31(seed: number): Config31
  local rows: Config31 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce31<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score31(cfg: Config31): Result31
  local total = reduce31(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m31", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 128,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store31: Store<Result31> = (function()
  local current: Result31 = score31(build31(31))
  local listeners: { Listener<Result31> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered31 = if store31.get().ok then store31.get().value else -store31.get().value
local _lifted31 = if typeof(filtered31) == "number" then filtered31 * 2 else 0

-- 区块 32
export type Config32 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result32 = { ok: boolean, value: number, events: { Event<Config32> } }
local function build32(seed: number): Config32
  local rows: Config32 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce32<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score32(cfg: Config32): Result32
  local total = reduce32(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m32", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 420,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store32: Store<Result32> = (function()
  local current: Result32 = score32(build32(32))
  local listeners: { Listener<Result32> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered32 = if store32.get().ok then store32.get().value else -store32.get().value
local _lifted32 = if typeof(filtered32) == "number" then filtered32 * 2 else 0

-- 区块 33
export type Config33 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result33 = { ok: boolean, value: number, events: { Event<Config33> } }
local function build33(seed: number): Config33
  local rows: Config33 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce33<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score33(cfg: Config33): Result33
  local total = reduce33(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m33", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 323,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store33: Store<Result33> = (function()
  local current: Result33 = score33(build33(33))
  local listeners: { Listener<Result33> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered33 = if store33.get().ok then store33.get().value else -store33.get().value
local _lifted33 = if typeof(filtered33) == "number" then filtered33 * 2 else 0

-- 区块 34
export type Config34 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result34 = { ok: boolean, value: number, events: { Event<Config34> } }
local function build34(seed: number): Config34
  local rows: Config34 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce34<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score34(cfg: Config34): Result34
  local total = reduce34(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m34", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 18,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store34: Store<Result34> = (function()
  local current: Result34 = score34(build34(34))
  local listeners: { Listener<Result34> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered34 = if store34.get().ok then store34.get().value else -store34.get().value
local _lifted34 = if typeof(filtered34) == "number" then filtered34 * 2 else 0

-- 区块 35
export type Config35 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result35 = { ok: boolean, value: number, events: { Event<Config35> } }
local function build35(seed: number): Config35
  local rows: Config35 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce35<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score35(cfg: Config35): Result35
  local total = reduce35(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m35", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 235,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store35: Store<Result35> = (function()
  local current: Result35 = score35(build35(35))
  local listeners: { Listener<Result35> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered35 = if store35.get().ok then store35.get().value else -store35.get().value
local _lifted35 = if typeof(filtered35) == "number" then filtered35 * 2 else 0

-- 区块 36
export type Config36 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result36 = { ok: boolean, value: number, events: { Event<Config36> } }
local function build36(seed: number): Config36
  local rows: Config36 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce36<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score36(cfg: Config36): Result36
  local total = reduce36(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m36", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 2,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store36: Store<Result36> = (function()
  local current: Result36 = score36(build36(36))
  local listeners: { Listener<Result36> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered36 = if store36.get().ok then store36.get().value else -store36.get().value
local _lifted36 = if typeof(filtered36) == "number" then filtered36 * 2 else 0

-- 区块 37
export type Config37 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result37 = { ok: boolean, value: number, events: { Event<Config37> } }
local function build37(seed: number): Config37
  local rows: Config37 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce37<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score37(cfg: Config37): Result37
  local total = reduce37(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m37", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 423,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store37: Store<Result37> = (function()
  local current: Result37 = score37(build37(37))
  local listeners: { Listener<Result37> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered37 = if store37.get().ok then store37.get().value else -store37.get().value
local _lifted37 = if typeof(filtered37) == "number" then filtered37 * 2 else 0

-- 区块 38
export type Config38 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result38 = { ok: boolean, value: number, events: { Event<Config38> } }
local function build38(seed: number): Config38
  local rows: Config38 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce38<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score38(cfg: Config38): Result38
  local total = reduce38(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m38", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 89,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store38: Store<Result38> = (function()
  local current: Result38 = score38(build38(38))
  local listeners: { Listener<Result38> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered38 = if store38.get().ok then store38.get().value else -store38.get().value
local _lifted38 = if typeof(filtered38) == "number" then filtered38 * 2 else 0

-- 区块 39
export type Config39 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result39 = { ok: boolean, value: number, events: { Event<Config39> } }
local function build39(seed: number): Config39
  local rows: Config39 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce39<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score39(cfg: Config39): Result39
  local total = reduce39(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m39", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 401,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store39: Store<Result39> = (function()
  local current: Result39 = score39(build39(39))
  local listeners: { Listener<Result39> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered39 = if store39.get().ok then store39.get().value else -store39.get().value
local _lifted39 = if typeof(filtered39) == "number" then filtered39 * 2 else 0

-- 区块 40
export type Config40 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result40 = { ok: boolean, value: number, events: { Event<Config40> } }
local function build40(seed: number): Config40
  local rows: Config40 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce40<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score40(cfg: Config40): Result40
  local total = reduce40(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m40", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 365,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store40: Store<Result40> = (function()
  local current: Result40 = score40(build40(40))
  local listeners: { Listener<Result40> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered40 = if store40.get().ok then store40.get().value else -store40.get().value
local _lifted40 = if typeof(filtered40) == "number" then filtered40 * 2 else 0

-- 区块 41
export type Config41 = {{ id: number, color: Color, pos: Vec2, flags: { [string]: boolean } } }
export type Result41 = { ok: boolean, value: number, events: { Event<Config41> } }
local function build41(seed: number): Config41
  local rows: Config41 = {}
  for i = 1, 16 do
    local color: Color = if i % 5 == 0 then "warm" elseif i % 3 == 0 then "cold" elseif i % 2 == 0 then "green" else "blue"
    rows[i] = {
      id = seed * 97 + i,
      color = color,
      pos = { x = i * 1.5, y = i / 3 },
      flags = { active = i % 2 == 0, visible = i < 12 },
    }
  end
  return rows
end
local function reduce41<T, U>(src: { T }, f: (U, T) -> U, init: U): U
  local acc: U = init
  for _, v in src do
    acc = f(acc, v)
  end
  return acc
end
local function score41(cfg: Config41): Result41
  local total = reduce41(cfg, function(a: number, row: { id: number, color: Color, pos: Vec2, flags: { [string]: boolean } })
    return a + row.id * row.pos.x + (if row.flags.active then 10 else 1)
  end, 0)
  local marker: Marker = { name = "m41", rank = total % 13, tags = {} }
  table.insert(marker.tags, label(marker))
  return {
    ok = total > 416,
    value = total / 3,
    events = { { kind = label(marker), at = total, payload = cfg } },
  }
end
local store41: Store<Result41> = (function()
  local current: Result41 = score41(build41(41))
  local listeners: { Listener<Result41> } = {}
  return {
    get = function() return current end,
    set = function(next)
      current = next
      for _, fn in listeners do
        fn(next)
      end
    end,
    subscribe = function(fn)
      table.insert(listeners, fn)
      return function()
        for i = #listeners, 1, -1 do
          if listeners[i] == fn then table.remove(listeners, i) end
        end
      end
    end,
  }
end)()
local filtered41 = if store41.get().ok then store41.get().value else -store41.get().value
local _lifted41 = if typeof(filtered41) == "number" then filtered41 * 2 else 0
