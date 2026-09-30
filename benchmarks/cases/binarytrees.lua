-- Binary trees benchmark: measures garbage collection and table allocation throughput
local function bottom_up_tree(depth)
  if depth > 0 then
    local d = depth - 1
    local left = bottom_up_tree(d)
    local right = bottom_up_tree(d)
    return { left, right }
  else
    return { false, false }
  end
end

local function item_check(tree)
  if tree[1] then
    return 1 + item_check(tree[1]) + item_check(tree[2])
  else
    return 1
  end
end

local max_depth = 13
local stretch_depth = max_depth + 1

local stretch_tree = bottom_up_tree(stretch_depth)
local stretch_check = item_check(stretch_tree)

local long_lived_tree = bottom_up_tree(max_depth)

local check_sum = stretch_check
local depth = 4
while depth <= max_depth do
  local iterations = 2 ^ (max_depth - depth + 4)
  for i = 1, iterations do
    local temp_tree = bottom_up_tree(depth)
    check_sum = (check_sum + item_check(temp_tree)) % 1000000007
  end
  depth = depth + 2
end

check_sum = (check_sum + item_check(long_lived_tree)) % 1000000007
return check_sum
