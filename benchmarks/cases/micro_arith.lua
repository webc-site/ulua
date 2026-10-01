-- TEMP micro: pure integer arithmetic incl. modulo (life modulo cost).
local sum = 0
for i = 1, 2000000 do
  sum = sum + i % 48 + (i + 7) % 48 + i * 7 + 13
end
return sum
