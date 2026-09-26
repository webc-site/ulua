-- Build a 140k-element array via a Park-Miller RNG, table.sort it, checksum.
-- The RNG stays under 2^53 so it is exact in both double (Luau/LuaJIT) and
-- 64-bit-integer (Lua 5.4) engines, keeping the checksum identical everywhere.
local seed = 42
local function rnd()
  seed = (seed * 16807) % 2147483647
  return seed
end
local n = 140000
local t = {}
for i = 1, n do t[i] = rnd() end
table.sort(t)
local sum = 0
for i = 1, n, 997 do sum = sum + t[i] end
return sum % 1000000007
