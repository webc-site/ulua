-- N-Queens backtracking search (PiL / upstream Luau shootout classic).
-- Recursive backtracking with table mutation and early pruning.
local function solve(n)
  local a = {}
  local count
  local function isplaceok(pos, row, col)
    for i = 1, row - 1 do
      local c = pos[i]
      if c == col or c - i == col - row or c + i == col + row then
        return false
      end
    end
    return true
  end
  local function place(row)
    if row > n then
      count = count + 1
    else
      for col = 1, n do
        if isplaceok(a, row, col) then
          a[row] = col
          place(row + 1)
        end
      end
    end
  end
  count = 0
  place(1)
  return count
end
local total = 0
for _ = 1, 15 do
  total = total + solve(8)
end
return total
