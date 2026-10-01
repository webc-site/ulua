-- TEMP micro: reads from array part (GETTABLE) with nested tables like life.
local h, w = 48, 48
local grid = {}
for i = 1, h do
  local row = {}
  for j = 1, w do
    row[j] = ((i * 7 + j * 13) % 5) < 2
  end
  grid[i] = row
end
local alive = 0
for _ = 1, 300 do
  for i = 1, h do
    local row = grid[i]
    for j = 1, w do
      if row[j] then
        alive = alive + 1
      end
    end
  end
end
return alive
