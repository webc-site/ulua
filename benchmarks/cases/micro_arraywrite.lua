-- TEMP micro: writes into pre-sized array part (SETTABLE fast path only).
local t = {}
for i = 1, 48 do
  t[i] = 0
end
for _ = 1, 3000 do
  for j = 1, 48 do
    t[j] = j
  end
end
return t[24]
