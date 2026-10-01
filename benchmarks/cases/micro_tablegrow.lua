-- TEMP micro: fresh table grown by sequential integer writes (life inner row).
local sum = 0
for _ = 1, 3000 do
  local row = {}
  for j = 1, 48 do
    row[j] = j % 3 == 0
  end
  if row[24] then
    sum = sum + 1
  end
end
return sum
