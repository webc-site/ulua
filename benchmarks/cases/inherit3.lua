local Life = {}
Life.__index = Life
function Life:breathe()
  return self.o2 + 1
end
local Animal = setmetatable({}, { __index = Life })
Animal.__index = Animal
function Animal:describe()
  return self.name .. ":a"
end
local Dog = setmetatable({}, { __index = Animal })
Dog.__index = Dog
function Dog:speak()
  return self.voice + 2
end
local Puppy = setmetatable({}, { __index = Dog })
Puppy.__index = Puppy
function Puppy:play()
  return self.toy + 3
end
function Puppy.new(name, toy, o2, voice)
  return setmetatable({ name = name, toy = toy, o2 = o2, voice = voice }, Puppy)
end
local acc = 0
local pup = Puppy.new("rex", 1, 2, 3)
for i = 1, 200000 do
  acc = acc + pup:play() + pup:speak() + #pup:describe() + pup:breathe()
end
return acc
