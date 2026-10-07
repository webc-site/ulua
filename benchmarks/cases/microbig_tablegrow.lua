-- TEMP micro big: fresh-table sequential integer appends ~50ms per engine.
local sum = 0
for g = 1, 300000 do
  local row = {}
  for j = 1, 48 do
    row[j] = 1
  end
  sum = sum + row[24]
end
return sum
