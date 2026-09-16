-- Dense 200x200 matrix multiply. Table indexing + multiply-accumulate.
local n = 200
local function mat(fill)
  local m = {}
  for i = 1, n do
    local row = {}
    for j = 1, n do row[j] = fill(i, j) end
    m[i] = row
  end
  return m
end
local a = mat(function(i, j) return (i + j) % 100 end)
local b = mat(function(i, j) return (i * 2 + j) % 100 end)
local c = mat(function() return 0 end)
for i = 1, n do
  local ai, ci = a[i], c[i]
  for k = 1, n do
    local aik, bk = ai[k], b[k]
    for j = 1, n do
      ci[j] = ci[j] + aik * bk[j]
    end
  end
end
local sum = 0
for i = 1, n do
  local ci = c[i]
  for j = 1, n do sum = sum + ci[j] end
end
print(string.format("%d", sum % 1000000007))
