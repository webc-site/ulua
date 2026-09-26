-- Spectral norm benchmark: dense math and array iteration
local function eval_a(i, j)
  local ij = i + j
  return 1.0 / ((ij * (ij + 1)) * 0.5 + i + 1.0)
end

local function eval_a_times_u(n, u, au)
  for i = 0, n - 1 do
    local sum = 0.0
    for j = 0, n - 1 do
      sum = sum + eval_a(i, j) * u[j + 1]
    end
    au[i + 1] = sum
  end
end

local function eval_at_times_u(n, u, au)
  for i = 0, n - 1 do
    local sum = 0.0
    for j = 0, n - 1 do
      sum = sum + eval_a(j, i) * u[j + 1]
    end
    au[i + 1] = sum
  end
end

local function eval_ata_times_u(n, u, atau, v)
  eval_a_times_u(n, u, v)
  eval_at_times_u(n, v, atau)
end

local n = 1000
local u = {}
local v = {}
local temp = {}
for i = 1, n do
  u[i] = 1.0
  v[i] = 0.0
  temp[i] = 0.0
end

for i = 1, 10 do
  eval_ata_times_u(n, u, v, temp)
  eval_ata_times_u(n, v, u, temp)
end

local v_bv = 0.0
local vv = 0.0
for i = 1, n do
  local ui = u[i]
  local vi = v[i]
  v_bv = v_bv + ui * vi
  vv = vv + vi * vi
end

local result = math.sqrt(v_bv / vv)
return result
