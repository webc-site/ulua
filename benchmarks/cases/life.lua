-- Conway's Game of Life on nested 2D tables (upstream Luau bench/tests classic).
-- 2D table indexing, wrap-around neighbor counting, branchy state updates.
local w, h, gens = 48, 48, 150
local grid = {}
for i = 1, h do
  local row = {}
  for j = 1, w do
    row[j] = ((i * 7 + j * 13) % 5) < 2
  end
  grid[i] = row
end
for _ = 1, gens do
  local next_grid = {}
  for i = 1, h do
    local up = grid[(i - 2) % h + 1]
    local mid = grid[i]
    local down = grid[i % h + 1]
    local row = {}
    for j = 1, w do
      local alive = mid[j]
      local neighbors = 0
      if up[(j - 2) % w + 1] then neighbors = neighbors + 1 end
      if up[j] then neighbors = neighbors + 1 end
      if up[j % w + 1] then neighbors = neighbors + 1 end
      if mid[(j - 2) % w + 1] then neighbors = neighbors + 1 end
      if mid[j % w + 1] then neighbors = neighbors + 1 end
      if down[(j - 2) % w + 1] then neighbors = neighbors + 1 end
      if down[j] then neighbors = neighbors + 1 end
      if down[j % w + 1] then neighbors = neighbors + 1 end
      row[j] = (alive and neighbors == 2) or neighbors == 3
    end
    next_grid[i] = row
  end
  grid = next_grid
end
local alive = 0
for i = 1, h do
  for j = 1, w do
    if grid[i][j] then alive = alive + 1 end
  end
end
return alive
