-- OOP method dispatch through metatable __index inheritance chains.
-- Two-level class hierarchy: field access + inherited/virtual method calls.
local Base = {}
Base.__index = Base
function Base.new(x, y)
  return setmetatable({ x = x, y = y }, Base)
end
function Base:len2()
  return self.x * self.x + self.y * self.y
end
function Base:describe()
  return self.x + self.y
end

local Vec = setmetatable({}, { __index = Base })
Vec.__index = Vec
function Vec.new(x, y, z)
  local self = Base.new(x, y)
  self.z = z
  return setmetatable(self, Vec)
end
function Vec:dot(other)
  return self.x * other.x + self.y * other.y + self.z * other.z
end

local acc = 0
local iters = 60000
for i = 1, iters do
  local a = Vec.new(i, i + 1, i + 2)
  local b = Vec.new(i + 3, i + 4, i + 5)
  acc = acc + a:dot(b) + a:len2() + b:describe()
end
return acc % 1000000007
