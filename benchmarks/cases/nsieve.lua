-- Sieve of Eratosthenes on boolean tables (Benchmarks Game nsieve).
-- Hash-table insert/clear cycles and integer stride loops.
local n = 200000
local total = 0
for _ = 1, 3 do
  local sieve = {}
  local count = 0
  for i = 2, n do
    if not sieve[i] then
      count = count + 1
      for j = i * i, n, i do
        sieve[j] = true
      end
    end
  end
  total = total + count
end
return total
