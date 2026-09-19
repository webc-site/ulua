-- String formatting + concatenation churn. string.format / table.concat / #.
local parts = {}
local acc = 0
for i = 1, 200000 do
  local s = string.format("item-%d-%x", i, i * 7)
  parts[#parts + 1] = s
  acc = acc + #s
end
local big = table.concat(parts, ",")
print(string.format("%d", (acc + #big) % 1000000007))
