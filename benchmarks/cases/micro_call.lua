-- TEMP micro: recursion + closure call overhead (queens shape).
local n = 11
local res = 0
local function f(k)
  if k <= 1 then
    res = res + 1
    return k
  end
  return k * f(k - 1)
end
for _ = 1, 200000 do
  f(n)
end
return res
