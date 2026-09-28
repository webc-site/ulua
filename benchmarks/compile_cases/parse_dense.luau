-- 生成物：表达式密集型源文件（benchmarks/fixtures/generate.mjs，勿手改）
local acc = 0.0
local bag, cb, m, q, r = nil, nil, nil, nil, nil
local names = { "alpha", "beta", "gamma", "delta" }
local cfg = { depth = 3, scale = 1.5, tags = { "a", "b" } }

local function blend(a: number, b: number, k: number): number
  return a * (1 - k) + b * k
end

cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 28.49) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 98.20) end
bag = { id = 4432, name = "warm-beta", pos = { 31.26, 4432.5, 62.52 }, ok = true }
q = "warm {math.floor(4.05)} {string.rep('=', 6 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 89.21) end
r = (90.47 > 33) and (acc ~= 0 and "warm" or "beta") or blend(90.47, 8433, 0.25)
acc = acc + blend(79.11, 95, math.sin(79.11 / 7)) * (1 + 1 / 31)
q = "blue {math.floor(43.59)} {string.rep('=', 6 + 1)} beta done"
m = { ["k7801"] = { nested = { deeper = { value = "green", list = { 1, 2, 7801 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 55.09) end
q = "fast {math.floor(78.67)} {string.rep('=', 4 + 1)} gamma done"
r = (21.95 > 16) and (acc ~= 0 and "green" or "alpha") or blend(21.95, 5616, 0.25)
acc = acc + blend(8.05, 38, math.sin(8.05 / 7)) * (1 + 5 / 31)
m = { ["k2681"] = { nested = { deeper = { value = "warm", list = { 1, 2, 2681 } } } } }
m = { ["k3929"] = { nested = { deeper = { value = "fast", list = { 1, 2, 3929 } } } } }
bag = { id = 6996, name = "red-delta", pos = { 70.95, 6996.5, 141.9 }, ok = true }
m = { ["k152"] = { nested = { deeper = { value = "cold", list = { 1, 2, 152 } } } } }
r = (3.57 > 22) and (acc ~= 0 and "cold" or "delta") or blend(3.57, 622, 0.25)
if acc > 61.12 then acc = acc / 19 elseif acc < -61.12 then acc = -acc else acc = acc + 1887 end
if acc > 37.12 then acc = acc / 74 elseif acc < -37.12 then acc = -acc else acc = acc + 4167 end
if acc > 58.56 then acc = acc / 20 elseif acc < -58.56 then acc = -acc else acc = acc + 375 end
acc = acc + blend(25.86, 60, math.sin(25.86 / 7)) * (1 + 2 / 31)
bag = { id = 4335, name = "warm-delta", pos = { 18.50, 4335.5, 37 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 82.78) end
bag = { id = 3395, name = "warm-beta", pos = { 43.61, 3395.5, 87.22 }, ok = false }
for j = 1, 35 do acc = acc + (j * 84.28) % 97 / (j + 1) end
bag = { id = 4912, name = "fast-alpha", pos = { 85.70, 4912.5, 171.4 }, ok = true }
r = (60.31 > 33) and (acc ~= 0 and "blue" or "beta") or blend(60.31, 4283, 0.25)
q = "fast {math.floor(74.25)} {string.rep('=', 4 + 1)} gamma done"
for j = 1, 15 do acc = acc + (j * 95.89) % 97 / (j + 1) end
q = "green {math.floor(43.18)} {string.rep('=', 6 + 1)} alpha done"
if acc > 48.53 then acc = acc / 19 elseif acc < -48.53 then acc = -acc else acc = acc + 6337 end
bag = { id = 6639, name = "fast-alpha", pos = { 4.28, 6639.5, 8.56 }, ok = false }
bag = { id = 1722, name = "fast-beta", pos = { 12.44, 1722.5, 24.88 }, ok = true }
m = { ["k8106"] = { nested = { deeper = { value = "green", list = { 1, 2, 8106 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 65.43) end
m = { ["k724"] = { nested = { deeper = { value = "blue", list = { 1, 2, 724 } } } } }
if acc > 99.94 then acc = acc / 85 elseif acc < -99.94 then acc = -acc else acc = acc + 7026 end
acc = acc + blend(99.06, 26, math.sin(99.06 / 7)) * (1 + 5 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 96.19) end
q = "blue {math.floor(64.72)} {string.rep('=', 5 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 24.94) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 64.46) end
r = (0.62 > 42) and (acc ~= 0 and "cold" or "delta") or blend(0.62, 4142, 0.25)
bag = { id = 4899, name = "blue-gamma", pos = { 32.28, 4899.5, 64.56 }, ok = false }
if acc > 24.09 then acc = acc / 26 elseif acc < -24.09 then acc = -acc else acc = acc + 114 end
bag = { id = 7394, name = "fast-delta", pos = { 75.19, 7394.5, 150.38 }, ok = true }
bag = { id = 5394, name = "blue-delta", pos = { 89.02, 5394.5, 178.04 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 46.59) end
acc = acc + blend(58.63, 25, math.sin(58.63 / 7)) * (1 + 4 / 31)
if acc > 46.56 then acc = acc / 12 elseif acc < -46.56 then acc = -acc else acc = acc + 6330 end
if acc > 80.64 then acc = acc / 34 elseif acc < -80.64 then acc = -acc else acc = acc + 6975 end
acc = acc + blend(0.95, 34, math.sin(0.95 / 7)) * (1 + 4 / 31)
if acc > 80.25 then acc = acc / 53 elseif acc < -80.25 then acc = -acc else acc = acc + 9308 end
bag = { id = 2169, name = "fast-gamma", pos = { 28.13, 2169.5, 56.26 }, ok = false }
m = { ["k5949"] = { nested = { deeper = { value = "red", list = { 1, 2, 5949 } } } } }
r = (10.21 > 39) and (acc ~= 0 and "blue" or "gamma") or blend(10.21, 6539, 0.25)
r = (59.93 > 28) and (acc ~= 0 and "blue" or "delta") or blend(59.93, 2528, 0.25)
if acc > 0.46 then acc = acc / 9 elseif acc < -0.46 then acc = -acc else acc = acc + 5259 end
if acc > 60.52 then acc = acc / 84 elseif acc < -60.52 then acc = -acc else acc = acc + 3376 end
q = "blue {math.floor(26.96)} {string.rep('=', 5 + 1)} alpha done"
r = (56.40 > 36) and (acc ~= 0 and "fast" or "gamma") or blend(56.40, 3786, 0.25)
for j = 1, 24 do acc = acc + (j * 50.16) % 97 / (j + 1) end
bag = { id = 7683, name = "cold-delta", pos = { 68.98, 7683.5, 137.96 }, ok = false }
r = (5.55 > 24) and (acc ~= 0 and "blue" or "alpha") or blend(5.55, 1024, 0.25)
bag = { id = 7176, name = "warm-gamma", pos = { 25.63, 7176.5, 51.26 }, ok = true }
if acc > 68.47 then acc = acc / 22 elseif acc < -68.47 then acc = -acc else acc = acc + 4382 end
bag = { id = 8454, name = "red-gamma", pos = { 90.46, 8454.5, 180.92 }, ok = true }
m = { ["k7553"] = { nested = { deeper = { value = "green", list = { 1, 2, 7553 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 98.05) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 43.65) end
m = { ["k1379"] = { nested = { deeper = { value = "fast", list = { 1, 2, 1379 } } } } }
acc = acc + blend(79.47, 72, math.sin(79.47 / 7)) * (1 + 12 / 31)
r = (74.97 > 22) and (acc ~= 0 and "fast" or "gamma") or blend(74.97, 9872, 0.25)
r = (42.57 > 12) and (acc ~= 0 and "red" or "gamma") or blend(42.57, 8112, 0.25)
m = { ["k7676"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7676 } } } } }
bag = { id = 2728, name = "warm-gamma", pos = { 67.32, 2728.5, 134.64 }, ok = true }
m = { ["k1484"] = { nested = { deeper = { value = "warm", list = { 1, 2, 1484 } } } } }
if acc > 1.76 then acc = acc / 66 elseif acc < -1.76 then acc = -acc else acc = acc + 7185 end
for j = 1, 7 do acc = acc + (j * 35.77) % 97 / (j + 1) end
if acc > 39.20 then acc = acc / 86 elseif acc < -39.20 then acc = -acc else acc = acc + 6137 end
bag = { id = 8120, name = "blue-gamma", pos = { 52.93, 8120.5, 105.86 }, ok = true }
for j = 1, 11 do acc = acc + (j * 73.21) % 97 / (j + 1) end
q = "green {math.floor(75.02)} {string.rep('=', 3 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 35.41) end
q = "red {math.floor(54.66)} {string.rep('=', 1 + 1)} beta done"
bag = { id = 5107, name = "blue-delta", pos = { 95.34, 5107.5, 190.68 }, ok = false }
m = { ["k5433"] = { nested = { deeper = { value = "cold", list = { 1, 2, 5433 } } } } }
r = (72.02 > 26) and (acc ~= 0 and "green" or "beta") or blend(72.02, 7276, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 64.84) end
bag = { id = 3732, name = "fast-delta", pos = { 84.31, 3732.5, 168.62 }, ok = true }
bag = { id = 3372, name = "fast-delta", pos = { 37.18, 3372.5, 74.36 }, ok = true }
r = (57.77 > 19) and (acc ~= 0 and "green" or "beta") or blend(57.77, 569, 0.25)
acc = acc + blend(18.62, 36, math.sin(18.62 / 7)) * (1 + 1 / 31)
r = (66.10 > 42) and (acc ~= 0 and "green" or "gamma") or blend(66.10, 4042, 0.25)
bag = { id = 4364, name = "cold-delta", pos = { 63.82, 4364.5, 127.64 }, ok = true }
for j = 1, 17 do acc = acc + (j * 21.31) % 97 / (j + 1) end
q = "fast {math.floor(42.19)} {string.rep('=', 3 + 1)} beta done"
acc = acc + blend(94.59, 53, math.sin(94.59 / 7)) * (1 + 6 / 31)
for j = 1, 39 do acc = acc + (j * 43.02) % 97 / (j + 1) end
q = "cold {math.floor(56.54)} {string.rep('=', 0 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 27.66) end
acc = acc + blend(26.02, 9, math.sin(26.02 / 7)) * (1 + 2 / 31)
for j = 1, 15 do acc = acc + (j * 64.83) % 97 / (j + 1) end
if acc > 18.72 then acc = acc / 82 elseif acc < -18.72 then acc = -acc else acc = acc + 4086 end
m = { ["k4566"] = { nested = { deeper = { value = "warm", list = { 1, 2, 4566 } } } } }
acc = acc + blend(45.08, 94, math.sin(45.08 / 7)) * (1 + 5 / 31)
if acc > 85.44 then acc = acc / 18 elseif acc < -85.44 then acc = -acc else acc = acc + 551 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 29.37) end
q = "warm {math.floor(30.23)} {string.rep('=', 6 + 1)} delta done"
r = (63.00 > 35) and (acc ~= 0 and "red" or "gamma") or blend(63.00, 6835, 0.25)
acc = acc + blend(10.83, 3, math.sin(10.83 / 7)) * (1 + 11 / 31)
bag = { id = 5100, name = "blue-beta", pos = { 72.65, 5100.5, 145.3 }, ok = true }
r = (63.54 > 39) and (acc ~= 0 and "blue" or "delta") or blend(63.54, 9739, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 32.92) end
q = "cold {math.floor(88.05)} {string.rep('=', 3 + 1)} gamma done"
q = "cold {math.floor(91.00)} {string.rep('=', 3 + 1)} beta done"
for j = 1, 17 do acc = acc + (j * 77.15) % 97 / (j + 1) end
q = "cold {math.floor(52.81)} {string.rep('=', 1 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 42.98) end
bag = { id = 9111, name = "blue-delta", pos = { 13.01, 9111.5, 26.02 }, ok = false }
q = "red {math.floor(31.15)} {string.rep('=', 5 + 1)} beta done"
m = { ["k8065"] = { nested = { deeper = { value = "red", list = { 1, 2, 8065 } } } } }
acc = acc + blend(28.79, 73, math.sin(28.79 / 7)) * (1 + 9 / 31)
q = "blue {math.floor(71.12)} {string.rep('=', 1 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 21.90) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 77.21) end
m = { ["k1591"] = { nested = { deeper = { value = "blue", list = { 1, 2, 1591 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 92.17) end
bag = { id = 6594, name = "green-gamma", pos = { 61.82, 6594.5, 123.64 }, ok = true }
if acc > 36.24 then acc = acc / 35 elseif acc < -36.24 then acc = -acc else acc = acc + 6620 end
q = "red {math.floor(44.80)} {string.rep('=', 2 + 1)} alpha done"
if acc > 68.46 then acc = acc / 42 elseif acc < -68.46 then acc = -acc else acc = acc + 7250 end
q = "green {math.floor(51.99)} {string.rep('=', 3 + 1)} alpha done"
q = "warm {math.floor(48.39)} {string.rep('=', 2 + 1)} delta done"
for j = 1, 22 do acc = acc + (j * 59.02) % 97 / (j + 1) end
m = { ["k3890"] = { nested = { deeper = { value = "fast", list = { 1, 2, 3890 } } } } }
bag = { id = 6122, name = "cold-delta", pos = { 98.83, 6122.5, 197.66 }, ok = true }
m = { ["k472"] = { nested = { deeper = { value = "fast", list = { 1, 2, 472 } } } } }
if acc > 18.57 then acc = acc / 46 elseif acc < -18.57 then acc = -acc else acc = acc + 7254 end
acc = acc + blend(61.15, 23, math.sin(61.15 / 7)) * (1 + 10 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 51.71) end
q = "fast {math.floor(39.65)} {string.rep('=', 0 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 46.62) end
acc = acc + blend(88.32, 12, math.sin(88.32 / 7)) * (1 + 0 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 62.83) end
bag = { id = 5201, name = "cold-alpha", pos = { 81.61, 5201.5, 163.22 }, ok = false }
m = { ["k8562"] = { nested = { deeper = { value = "green", list = { 1, 2, 8562 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 73.35) end
acc = acc + blend(96.06, 88, math.sin(96.06 / 7)) * (1 + 2 / 31)
acc = acc + blend(22.28, 93, math.sin(22.28 / 7)) * (1 + 12 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 12.53) end
r = (18.67 > 34) and (acc ~= 0 and "cold" or "gamma") or blend(18.67, 7584, 0.25)
q = "warm {math.floor(36.32)} {string.rep('=', 6 + 1)} gamma done"
m = { ["k4277"] = { nested = { deeper = { value = "red", list = { 1, 2, 4277 } } } } }
for j = 1, 38 do acc = acc + (j * 81.63) % 97 / (j + 1) end
acc = acc + blend(98.03, 96, math.sin(98.03 / 7)) * (1 + 1 / 31)
acc = acc + blend(75.92, 43, math.sin(75.92 / 7)) * (1 + 4 / 31)
r = (8.72 > 12) and (acc ~= 0 and "fast" or "gamma") or blend(8.72, 5162, 0.25)
for j = 1, 30 do acc = acc + (j * 73.58) % 97 / (j + 1) end
acc = acc + blend(10.26, 76, math.sin(10.26 / 7)) * (1 + 0 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 79.56) end
for j = 1, 7 do acc = acc + (j * 56.34) % 97 / (j + 1) end
acc = acc + blend(75.30, 26, math.sin(75.30 / 7)) * (1 + 12 / 31)
if acc > 76.16 then acc = acc / 45 elseif acc < -76.16 then acc = -acc else acc = acc + 8499 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 54.53) end
bag = { id = 7209, name = "fast-delta", pos = { 2.03, 7209.5, 4.06 }, ok = false }
q = "blue {math.floor(83.24)} {string.rep('=', 5 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 11.19) end
bag = { id = 4058, name = "cold-gamma", pos = { 77.57, 4058.5, 155.14 }, ok = true }
r = (21.99 > 34) and (acc ~= 0 and "blue" or "gamma") or blend(21.99, 2934, 0.25)
for j = 1, 32 do acc = acc + (j * 76.53) % 97 / (j + 1) end
bag = { id = 1541, name = "fast-delta", pos = { 31.97, 1541.5, 63.94 }, ok = false }
if acc > 55.15 then acc = acc / 43 elseif acc < -55.15 then acc = -acc else acc = acc + 4759 end
if acc > 51.49 then acc = acc / 34 elseif acc < -51.49 then acc = -acc else acc = acc + 3771 end
if acc > 8.39 then acc = acc / 57 elseif acc < -8.39 then acc = -acc else acc = acc + 5841 end
r = (79.14 > 44) and (acc ~= 0 and "fast" or "delta") or blend(79.14, 5744, 0.25)
acc = acc + blend(18.49, 43, math.sin(18.49 / 7)) * (1 + 12 / 31)
bag = { id = 3577, name = "green-delta", pos = { 39.26, 3577.5, 78.52 }, ok = false }
if acc > 30.26 then acc = acc / 33 elseif acc < -30.26 then acc = -acc else acc = acc + 655 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 64.64) end
if acc > 80.06 then acc = acc / 11 elseif acc < -80.06 then acc = -acc else acc = acc + 4727 end
if acc > 14.55 then acc = acc / 61 elseif acc < -14.55 then acc = -acc else acc = acc + 2018 end
for j = 1, 24 do acc = acc + (j * 87.32) % 97 / (j + 1) end
if acc > 76.09 then acc = acc / 88 elseif acc < -76.09 then acc = -acc else acc = acc + 265 end
if acc > 3.11 then acc = acc / 61 elseif acc < -3.11 then acc = -acc else acc = acc + 4421 end
if acc > 62.03 then acc = acc / 78 elseif acc < -62.03 then acc = -acc else acc = acc + 6663 end
r = (49.82 > 25) and (acc ~= 0 and "fast" or "alpha") or blend(49.82, 8675, 0.25)
acc = acc + blend(96.09, 63, math.sin(96.09 / 7)) * (1 + 9 / 31)
bag = { id = 634, name = "warm-beta", pos = { 60.69, 634.5, 121.38 }, ok = true }
q = "blue {math.floor(24.97)} {string.rep('=', 5 + 1)} gamma done"
q = "warm {math.floor(69.01)} {string.rep('=', 5 + 1)} alpha done"
q = "warm {math.floor(19.31)} {string.rep('=', 4 + 1)} delta done"
if acc > 85.76 then acc = acc / 12 elseif acc < -85.76 then acc = -acc else acc = acc + 6953 end
m = { ["k7718"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7718 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 71.84) end
m = { ["k8821"] = { nested = { deeper = { value = "blue", list = { 1, 2, 8821 } } } } }
r = (39.10 > 48) and (acc ~= 0 and "green" or "gamma") or blend(39.10, 4148, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 67.27) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 33.60) end
m = { ["k1338"] = { nested = { deeper = { value = "green", list = { 1, 2, 1338 } } } } }
r = (46.31 > 46) and (acc ~= 0 and "green" or "alpha") or blend(46.31, 6796, 0.25)
acc = acc + blend(30.89, 26, math.sin(30.89 / 7)) * (1 + 4 / 31)
r = (92.32 > 32) and (acc ~= 0 and "warm" or "alpha") or blend(92.32, 882, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 37.68) end
acc = acc + blend(80.69, 81, math.sin(80.69 / 7)) * (1 + 0 / 31)
q = "red {math.floor(57.37)} {string.rep('=', 4 + 1)} delta done"
q = "green {math.floor(94.20)} {string.rep('=', 4 + 1)} delta done"
bag = { id = 4829, name = "green-delta", pos = { 89.61, 4829.5, 179.22 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 93.51) end
q = "green {math.floor(89.92)} {string.rep('=', 4 + 1)} delta done"
for j = 1, 21 do acc = acc + (j * 84.07) % 97 / (j + 1) end
acc = acc + blend(23.63, 38, math.sin(23.63 / 7)) * (1 + 3 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 18.12) end
acc = acc + blend(29.48, 5, math.sin(29.48 / 7)) * (1 + 9 / 31)
for j = 1, 2 do acc = acc + (j * 79.85) % 97 / (j + 1) end
r = (73.90 > 3) and (acc ~= 0 and "cold" or "beta") or blend(73.90, 6753, 0.25)
bag = { id = 4722, name = "cold-gamma", pos = { 99.60, 4722.5, 199.2 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 4.14) end
bag = { id = 2957, name = "red-delta", pos = { 95.30, 2957.5, 190.6 }, ok = false }
if acc > 49.44 then acc = acc / 8 elseif acc < -49.44 then acc = -acc else acc = acc + 8195 end
q = "blue {math.floor(26.49)} {string.rep('=', 1 + 1)} beta done"
if acc > 29.33 then acc = acc / 9 elseif acc < -29.33 then acc = -acc else acc = acc + 542 end
q = "cold {math.floor(99.80)} {string.rep('=', 4 + 1)} beta done"
q = "red {math.floor(14.58)} {string.rep('=', 5 + 1)} delta done"
bag = { id = 3233, name = "cold-gamma", pos = { 66.07, 3233.5, 132.14 }, ok = false }
q = "warm {math.floor(4.07)} {string.rep('=', 1 + 1)} beta done"
for j = 1, 25 do acc = acc + (j * 29.55) % 97 / (j + 1) end
for j = 1, 36 do acc = acc + (j * 37.35) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 29.85) end
for j = 1, 29 do acc = acc + (j * 13.52) % 97 / (j + 1) end
bag = { id = 7118, name = "warm-alpha", pos = { 89.02, 7118.5, 178.04 }, ok = true }
m = { ["k3184"] = { nested = { deeper = { value = "cold", list = { 1, 2, 3184 } } } } }
bag = { id = 1669, name = "fast-gamma", pos = { 20.77, 1669.5, 41.54 }, ok = false }
q = "green {math.floor(33.19)} {string.rep('=', 3 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 6.79) end
for j = 1, 40 do acc = acc + (j * 0.72) % 97 / (j + 1) end
acc = acc + blend(38.80, 78, math.sin(38.80 / 7)) * (1 + 0 / 31)
bag = { id = 2116, name = "red-gamma", pos = { 55.20, 2116.5, 110.4 }, ok = true }
r = (34.93 > 20) and (acc ~= 0 and "blue" or "beta") or blend(34.93, 6620, 0.25)
q = "cold {math.floor(15.77)} {string.rep('=', 0 + 1)} beta done"
bag = { id = 67, name = "cold-delta", pos = { 46.23, 67.5, 92.46 }, ok = false }
if acc > 81.71 then acc = acc / 89 elseif acc < -81.71 then acc = -acc else acc = acc + 5339 end
acc = acc + blend(5.79, 61, math.sin(5.79 / 7)) * (1 + 11 / 31)
for j = 1, 20 do acc = acc + (j * 85.64) % 97 / (j + 1) end
for j = 1, 22 do acc = acc + (j * 19.53) % 97 / (j + 1) end
bag = { id = 3683, name = "red-delta", pos = { 56.02, 3683.5, 112.04 }, ok = false }
acc = acc + blend(85.46, 42, math.sin(85.46 / 7)) * (1 + 12 / 31)
bag = { id = 8680, name = "blue-delta", pos = { 2.29, 8680.5, 4.58 }, ok = true }
q = "fast {math.floor(39.18)} {string.rep('=', 6 + 1)} beta done"
if acc > 26.60 then acc = acc / 20 elseif acc < -26.60 then acc = -acc else acc = acc + 8563 end
q = "blue {math.floor(87.80)} {string.rep('=', 3 + 1)} delta done"
q = "cold {math.floor(68.78)} {string.rep('=', 0 + 1)} gamma done"
r = (93.97 > 16) and (acc ~= 0 and "red" or "beta") or blend(93.97, 5916, 0.25)
acc = acc + blend(64.25, 24, math.sin(64.25 / 7)) * (1 + 6 / 31)
r = (38.25 > 40) and (acc ~= 0 and "red" or "gamma") or blend(38.25, 3140, 0.25)
acc = acc + blend(42.93, 10, math.sin(42.93 / 7)) * (1 + 0 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 1.47) end
m = { ["k6748"] = { nested = { deeper = { value = "red", list = { 1, 2, 6748 } } } } }
bag = { id = 9138, name = "red-beta", pos = { 1.38, 9138.5, 2.76 }, ok = true }
if acc > 11.47 then acc = acc / 15 elseif acc < -11.47 then acc = -acc else acc = acc + 8291 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 71.21) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 99.22) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 11.22) end
acc = acc + blend(27.66, 96, math.sin(27.66 / 7)) * (1 + 8 / 31)
r = (62.02 > 12) and (acc ~= 0 and "cold" or "alpha") or blend(62.02, 4912, 0.25)
m = { ["k1567"] = { nested = { deeper = { value = "warm", list = { 1, 2, 1567 } } } } }
if acc > 68.58 then acc = acc / 30 elseif acc < -68.58 then acc = -acc else acc = acc + 7594 end
m = { ["k9670"] = { nested = { deeper = { value = "warm", list = { 1, 2, 9670 } } } } }
acc = acc + blend(10.24, 28, math.sin(10.24 / 7)) * (1 + 9 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 81.96) end
m = { ["k6853"] = { nested = { deeper = { value = "warm", list = { 1, 2, 6853 } } } } }
acc = acc + blend(5.26, 81, math.sin(5.26 / 7)) * (1 + 1 / 31)
r = (30.25 > 4) and (acc ~= 0 and "blue" or "delta") or blend(30.25, 2254, 0.25)
m = { ["k5838"] = { nested = { deeper = { value = "green", list = { 1, 2, 5838 } } } } }
for j = 1, 11 do acc = acc + (j * 52.28) % 97 / (j + 1) end
for j = 1, 35 do acc = acc + (j * 62.54) % 97 / (j + 1) end
bag = { id = 8898, name = "blue-delta", pos = { 13.90, 8898.5, 27.8 }, ok = true }
r = (72.14 > 29) and (acc ~= 0 and "warm" or "gamma") or blend(72.14, 8579, 0.25)
bag = { id = 7067, name = "warm-delta", pos = { 82.27, 7067.5, 164.54 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 53.03) end
r = (80.56 > 32) and (acc ~= 0 and "blue" or "delta") or blend(80.56, 932, 0.25)
acc = acc + blend(13.83, 28, math.sin(13.83 / 7)) * (1 + 6 / 31)
r = (4.87 > 20) and (acc ~= 0 and "cold" or "delta") or blend(4.87, 270, 0.25)
bag = { id = 8053, name = "green-delta", pos = { 91.38, 8053.5, 182.76 }, ok = false }
for j = 1, 16 do acc = acc + (j * 54.03) % 97 / (j + 1) end
if acc > 11.23 then acc = acc / 2 elseif acc < -11.23 then acc = -acc else acc = acc + 3472 end
if acc > 89.45 then acc = acc / 46 elseif acc < -89.45 then acc = -acc else acc = acc + 4228 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 2.73) end
r = (68.22 > 19) and (acc ~= 0 and "red" or "delta") or blend(68.22, 8669, 0.25)
for j = 1, 19 do acc = acc + (j * 22.65) % 97 / (j + 1) end
q = "warm {math.floor(98.07)} {string.rep('=', 4 + 1)} delta done"
m = { ["k2071"] = { nested = { deeper = { value = "red", list = { 1, 2, 2071 } } } } }
r = (27.94 > 47) and (acc ~= 0 and "fast" or "delta") or blend(27.94, 8747, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 71.90) end
m = { ["k3199"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3199 } } } } }
r = (22.91 > 7) and (acc ~= 0 and "red" or "delta") or blend(22.91, 3857, 0.25)
m = { ["k1668"] = { nested = { deeper = { value = "blue", list = { 1, 2, 1668 } } } } }
q = "cold {math.floor(41.66)} {string.rep('=', 4 + 1)} beta done"
bag = { id = 6238, name = "blue-gamma", pos = { 34.14, 6238.5, 68.28 }, ok = true }
bag = { id = 9258, name = "fast-beta", pos = { 21.25, 9258.5, 42.5 }, ok = true }
m = { ["k5941"] = { nested = { deeper = { value = "green", list = { 1, 2, 5941 } } } } }
bag = { id = 8287, name = "green-gamma", pos = { 13.68, 8287.5, 27.36 }, ok = false }
acc = acc + blend(68.67, 1, math.sin(68.67 / 7)) * (1 + 1 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 57.26) end
m = { ["k6413"] = { nested = { deeper = { value = "cold", list = { 1, 2, 6413 } } } } }
q = "warm {math.floor(1.37)} {string.rep('=', 6 + 1)} alpha done"
m = { ["k6947"] = { nested = { deeper = { value = "cold", list = { 1, 2, 6947 } } } } }
for j = 1, 19 do acc = acc + (j * 42.77) % 97 / (j + 1) end
for j = 1, 13 do acc = acc + (j * 51.14) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 44.91) end
bag = { id = 9838, name = "blue-alpha", pos = { 31.24, 9838.5, 62.48 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 19.43) end
for j = 1, 6 do acc = acc + (j * 27.75) % 97 / (j + 1) end
m = { ["k3776"] = { nested = { deeper = { value = "warm", list = { 1, 2, 3776 } } } } }
for j = 1, 40 do acc = acc + (j * 31.87) % 97 / (j + 1) end
if acc > 81.08 then acc = acc / 79 elseif acc < -81.08 then acc = -acc else acc = acc + 7910 end
m = { ["k5004"] = { nested = { deeper = { value = "cold", list = { 1, 2, 5004 } } } } }
if acc > 90.85 then acc = acc / 86 elseif acc < -90.85 then acc = -acc else acc = acc + 4090 end
acc = acc + blend(91.29, 1, math.sin(91.29 / 7)) * (1 + 12 / 31)
acc = acc + blend(43.97, 96, math.sin(43.97 / 7)) * (1 + 10 / 31)
m = { ["k3815"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3815 } } } } }
bag = { id = 5282, name = "cold-alpha", pos = { 21.54, 5282.5, 43.08 }, ok = true }
acc = acc + blend(88.20, 40, math.sin(88.20 / 7)) * (1 + 12 / 31)
q = "green {math.floor(72.55)} {string.rep('=', 0 + 1)} delta done"
m = { ["k1262"] = { nested = { deeper = { value = "green", list = { 1, 2, 1262 } } } } }
for j = 1, 9 do acc = acc + (j * 68.15) % 97 / (j + 1) end
if acc > 16.97 then acc = acc / 63 elseif acc < -16.97 then acc = -acc else acc = acc + 6292 end
bag = { id = 9074, name = "blue-alpha", pos = { 93.57, 9074.5, 187.14 }, ok = true }
m = { ["k5986"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5986 } } } } }
r = (94.83 > 23) and (acc ~= 0 and "red" or "delta") or blend(94.83, 4073, 0.25)
q = "red {math.floor(67.75)} {string.rep('=', 5 + 1)} gamma done"
q = "cold {math.floor(56.42)} {string.rep('=', 1 + 1)} alpha done"
m = { ["k1542"] = { nested = { deeper = { value = "blue", list = { 1, 2, 1542 } } } } }
q = "green {math.floor(65.47)} {string.rep('=', 4 + 1)} delta done"
bag = { id = 3607, name = "fast-delta", pos = { 87.49, 3607.5, 174.98 }, ok = false }
m = { ["k4110"] = { nested = { deeper = { value = "warm", list = { 1, 2, 4110 } } } } }
bag = { id = 6945, name = "warm-delta", pos = { 83.28, 6945.5, 166.56 }, ok = false }
m = { ["k8459"] = { nested = { deeper = { value = "warm", list = { 1, 2, 8459 } } } } }
r = (36.01 > 30) and (acc ~= 0 and "warm" or "gamma") or blend(36.01, 9780, 0.25)
r = (91.34 > 9) and (acc ~= 0 and "blue" or "beta") or blend(91.34, 2809, 0.25)
for j = 1, 14 do acc = acc + (j * 1.48) % 97 / (j + 1) end
for j = 1, 17 do acc = acc + (j * 30.43) % 97 / (j + 1) end
for j = 1, 26 do acc = acc + (j * 76.13) % 97 / (j + 1) end
for j = 1, 15 do acc = acc + (j * 96.39) % 97 / (j + 1) end
bag = { id = 3744, name = "green-beta", pos = { 30.07, 3744.5, 60.14 }, ok = true }
acc = acc + blend(90.69, 96, math.sin(90.69 / 7)) * (1 + 5 / 31)
q = "fast {math.floor(61.98)} {string.rep('=', 6 + 1)} alpha done"
acc = acc + blend(28.26, 91, math.sin(28.26 / 7)) * (1 + 1 / 31)
if acc > 67.22 then acc = acc / 62 elseif acc < -67.22 then acc = -acc else acc = acc + 5579 end
bag = { id = 9743, name = "cold-alpha", pos = { 62.73, 9743.5, 125.46 }, ok = false }
if acc > 14.50 then acc = acc / 56 elseif acc < -14.50 then acc = -acc else acc = acc + 2992 end
m = { ["k9963"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9963 } } } } }
r = (77.52 > 47) and (acc ~= 0 and "cold" or "delta") or blend(77.52, 7747, 0.25)
q = "warm {math.floor(21.93)} {string.rep('=', 1 + 1)} alpha done"
r = (98.14 > 13) and (acc ~= 0 and "cold" or "gamma") or blend(98.14, 313, 0.25)
m = { ["k782"] = { nested = { deeper = { value = "blue", list = { 1, 2, 782 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 48.12) end
for j = 1, 27 do acc = acc + (j * 13.61) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 38.05) end
m = { ["k5190"] = { nested = { deeper = { value = "warm", list = { 1, 2, 5190 } } } } }
q = "blue {math.floor(76.59)} {string.rep('=', 3 + 1)} delta done"
acc = acc + blend(35.39, 87, math.sin(35.39 / 7)) * (1 + 8 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 41.49) end
bag = { id = 4677, name = "red-beta", pos = { 4.23, 4677.5, 8.46 }, ok = false }
if acc > 67.03 then acc = acc / 15 elseif acc < -67.03 then acc = -acc else acc = acc + 5354 end
m = { ["k1203"] = { nested = { deeper = { value = "blue", list = { 1, 2, 1203 } } } } }
r = (12.84 > 38) and (acc ~= 0 and "blue" or "alpha") or blend(12.84, 2738, 0.25)
q = "fast {math.floor(3.30)} {string.rep('=', 3 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 36.10) end
m = { ["k7380"] = { nested = { deeper = { value = "red", list = { 1, 2, 7380 } } } } }
if acc > 76.00 then acc = acc / 45 elseif acc < -76.00 then acc = -acc else acc = acc + 7876 end
m = { ["k9894"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9894 } } } } }
bag = { id = 6557, name = "blue-delta", pos = { 55.45, 6557.5, 110.9 }, ok = false }
for j = 1, 31 do acc = acc + (j * 73.88) % 97 / (j + 1) end
bag = { id = 368, name = "blue-delta", pos = { 43.51, 368.5, 87.02 }, ok = true }
bag = { id = 4528, name = "fast-beta", pos = { 80.35, 4528.5, 160.7 }, ok = true }
bag = { id = 5699, name = "cold-delta", pos = { 82.18, 5699.5, 164.36 }, ok = false }
r = (19.50 > 9) and (acc ~= 0 and "green" or "beta") or blend(19.50, 4809, 0.25)
bag = { id = 830, name = "fast-beta", pos = { 96.20, 830.5, 192.4 }, ok = true }
m = { ["k6911"] = { nested = { deeper = { value = "fast", list = { 1, 2, 6911 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 53.76) end
if acc > 25.56 then acc = acc / 12 elseif acc < -25.56 then acc = -acc else acc = acc + 8110 end
for j = 1, 14 do acc = acc + (j * 62.59) % 97 / (j + 1) end
if acc > 28.41 then acc = acc / 87 elseif acc < -28.41 then acc = -acc else acc = acc + 1866 end
if acc > 49.18 then acc = acc / 40 elseif acc < -49.18 then acc = -acc else acc = acc + 8227 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 35.19) end
r = (12.09 > 12) and (acc ~= 0 and "red" or "gamma") or blend(12.09, 462, 0.25)
acc = acc + blend(78.39, 62, math.sin(78.39 / 7)) * (1 + 4 / 31)
bag = { id = 6394, name = "red-gamma", pos = { 42.65, 6394.5, 85.3 }, ok = true }
acc = acc + blend(22.99, 34, math.sin(22.99 / 7)) * (1 + 5 / 31)
r = (0.96 > 4) and (acc ~= 0 and "cold" or "delta") or blend(0.96, 8054, 0.25)
bag = { id = 477, name = "fast-delta", pos = { 45.37, 477.5, 90.74 }, ok = false }
acc = acc + blend(1.68, 83, math.sin(1.68 / 7)) * (1 + 3 / 31)
bag = { id = 1231, name = "green-delta", pos = { 22.00, 1231.5, 44 }, ok = false }
m = { ["k8902"] = { nested = { deeper = { value = "blue", list = { 1, 2, 8902 } } } } }
if acc > 19.92 then acc = acc / 66 elseif acc < -19.92 then acc = -acc else acc = acc + 8342 end
for j = 1, 7 do acc = acc + (j * 5.82) % 97 / (j + 1) end
acc = acc + blend(84.96, 76, math.sin(84.96 / 7)) * (1 + 11 / 31)
acc = acc + blend(39.64, 64, math.sin(39.64 / 7)) * (1 + 5 / 31)
for j = 1, 27 do acc = acc + (j * 53.52) % 97 / (j + 1) end
q = "blue {math.floor(87.86)} {string.rep('=', 4 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 25.53) end
acc = acc + blend(46.27, 70, math.sin(46.27 / 7)) * (1 + 5 / 31)
if acc > 36.29 then acc = acc / 89 elseif acc < -36.29 then acc = -acc else acc = acc + 7297 end
acc = acc + blend(23.92, 24, math.sin(23.92 / 7)) * (1 + 7 / 31)
q = "blue {math.floor(9.51)} {string.rep('=', 1 + 1)} beta done"
q = "green {math.floor(42.77)} {string.rep('=', 0 + 1)} gamma done"
m = { ["k9230"] = { nested = { deeper = { value = "warm", list = { 1, 2, 9230 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 24.02) end
acc = acc + blend(91.46, 93, math.sin(91.46 / 7)) * (1 + 2 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 43.31) end
bag = { id = 3421, name = "cold-beta", pos = { 89.46, 3421.5, 178.92 }, ok = false }
for j = 1, 29 do acc = acc + (j * 99.75) % 97 / (j + 1) end
q = "green {math.floor(17.12)} {string.rep('=', 3 + 1)} alpha done"
r = (71.91 > 0) and (acc ~= 0 and "cold" or "alpha") or blend(71.91, 5850, 0.25)
bag = { id = 2820, name = "warm-delta", pos = { 29.35, 2820.5, 58.7 }, ok = true }
q = "cold {math.floor(10.45)} {string.rep('=', 5 + 1)} delta done"
r = (36.78 > 43) and (acc ~= 0 and "blue" or "alpha") or blend(36.78, 1043, 0.25)
q = "fast {math.floor(72.04)} {string.rep('=', 0 + 1)} alpha done"
for j = 1, 3 do acc = acc + (j * 20.56) % 97 / (j + 1) end
r = (14.71 > 26) and (acc ~= 0 and "blue" or "alpha") or blend(14.71, 5976, 0.25)
r = (0.07 > 14) and (acc ~= 0 and "green" or "delta") or blend(0.07, 14, 0.25)
q = "red {math.floor(81.50)} {string.rep('=', 2 + 1)} gamma done"
if acc > 72.53 then acc = acc / 77 elseif acc < -72.53 then acc = -acc else acc = acc + 5238 end
acc = acc + blend(78.47, 57, math.sin(78.47 / 7)) * (1 + 1 / 31)
if acc > 27.37 then acc = acc / 15 elseif acc < -27.37 then acc = -acc else acc = acc + 7045 end
q = "green {math.floor(36.75)} {string.rep('=', 4 + 1)} alpha done"
acc = acc + blend(85.28, 23, math.sin(85.28 / 7)) * (1 + 4 / 31)
m = { ["k9048"] = { nested = { deeper = { value = "red", list = { 1, 2, 9048 } } } } }
for j = 1, 36 do acc = acc + (j * 78.17) % 97 / (j + 1) end
acc = acc + blend(95.65, 30, math.sin(95.65 / 7)) * (1 + 10 / 31)
bag = { id = 2261, name = "warm-beta", pos = { 78.60, 2261.5, 157.2 }, ok = false }
if acc > 68.74 then acc = acc / 31 elseif acc < -68.74 then acc = -acc else acc = acc + 9375 end
q = "warm {math.floor(61.79)} {string.rep('=', 2 + 1)} gamma done"
bag = { id = 620, name = "blue-gamma", pos = { 92.74, 620.5, 185.48 }, ok = true }
if acc > 45.50 then acc = acc / 87 elseif acc < -45.50 then acc = -acc else acc = acc + 2489 end
q = "red {math.floor(38.03)} {string.rep('=', 0 + 1)} gamma done"
acc = acc + blend(60.62, 61, math.sin(60.62 / 7)) * (1 + 9 / 31)
q = "fast {math.floor(66.12)} {string.rep('=', 5 + 1)} alpha done"
for j = 1, 23 do acc = acc + (j * 62.88) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 40.87) end
acc = acc + blend(61.31, 77, math.sin(61.31 / 7)) * (1 + 4 / 31)
r = (19.94 > 35) and (acc ~= 0 and "cold" or "beta") or blend(19.94, 1135, 0.25)
q = "cold {math.floor(17.08)} {string.rep('=', 1 + 1)} delta done"
acc = acc + blend(47.66, 11, math.sin(47.66 / 7)) * (1 + 6 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 14.20) end
m = { ["k9519"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9519 } } } } }
for j = 1, 40 do acc = acc + (j * 27.20) % 97 / (j + 1) end
acc = acc + blend(14.19, 89, math.sin(14.19 / 7)) * (1 + 2 / 31)
m = { ["k9652"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9652 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 77.82) end
bag = { id = 1852, name = "green-delta", pos = { 73.99, 1852.5, 147.98 }, ok = true }
if acc > 30.63 then acc = acc / 81 elseif acc < -30.63 then acc = -acc else acc = acc + 8891 end
for j = 1, 1 do acc = acc + (j * 37.68) % 97 / (j + 1) end
acc = acc + blend(38.28, 49, math.sin(38.28 / 7)) * (1 + 1 / 31)
r = (67.14 > 10) and (acc ~= 0 and "fast" or "beta") or blend(67.14, 1810, 0.25)
bag = { id = 1583, name = "blue-gamma", pos = { 39.88, 1583.5, 79.76 }, ok = false }
if acc > 38.28 then acc = acc / 20 elseif acc < -38.28 then acc = -acc else acc = acc + 1176 end
q = "cold {math.floor(41.50)} {string.rep('=', 5 + 1)} beta done"
m = { ["k1156"] = { nested = { deeper = { value = "cold", list = { 1, 2, 1156 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 1.73) end
for j = 1, 32 do acc = acc + (j * 42.47) % 97 / (j + 1) end
acc = acc + blend(41.42, 29, math.sin(41.42 / 7)) * (1 + 12 / 31)
r = (35.53 > 34) and (acc ~= 0 and "green" or "gamma") or blend(35.53, 934, 0.25)
bag = { id = 7527, name = "cold-gamma", pos = { 4.25, 7527.5, 8.5 }, ok = false }
if acc > 69.67 then acc = acc / 20 elseif acc < -69.67 then acc = -acc else acc = acc + 4291 end
r = (19.12 > 42) and (acc ~= 0 and "green" or "gamma") or blend(19.12, 3542, 0.25)
for j = 1, 33 do acc = acc + (j * 48.68) % 97 / (j + 1) end
bag = { id = 1269, name = "warm-beta", pos = { 49.28, 1269.5, 98.56 }, ok = false }
acc = acc + blend(39.57, 72, math.sin(39.57 / 7)) * (1 + 7 / 31)
m = { ["k9799"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9799 } } } } }
for j = 1, 34 do acc = acc + (j * 62.16) % 97 / (j + 1) end
if acc > 63.29 then acc = acc / 24 elseif acc < -63.29 then acc = -acc else acc = acc + 6253 end
bag = { id = 4463, name = "red-beta", pos = { 97.88, 4463.5, 195.76 }, ok = false }
m = { ["k5685"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5685 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 29.38) end
r = (10.32 > 1) and (acc ~= 0 and "cold" or "beta") or blend(10.32, 4901, 0.25)
m = { ["k7975"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7975 } } } } }
if acc > 83.67 then acc = acc / 51 elseif acc < -83.67 then acc = -acc else acc = acc + 6369 end
acc = acc + blend(29.73, 93, math.sin(29.73 / 7)) * (1 + 3 / 31)
r = (8.55 > 47) and (acc ~= 0 and "green" or "delta") or blend(8.55, 5147, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 27.06) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 87.06) end
m = { ["k8429"] = { nested = { deeper = { value = "green", list = { 1, 2, 8429 } } } } }
m = { ["k4635"] = { nested = { deeper = { value = "cold", list = { 1, 2, 4635 } } } } }
acc = acc + blend(17.71, 47, math.sin(17.71 / 7)) * (1 + 9 / 31)
bag = { id = 5735, name = "cold-gamma", pos = { 12.71, 5735.5, 25.42 }, ok = false }
if acc > 6.13 then acc = acc / 20 elseif acc < -6.13 then acc = -acc else acc = acc + 4558 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 64.16) end
m = { ["k4091"] = { nested = { deeper = { value = "blue", list = { 1, 2, 4091 } } } } }
for j = 1, 17 do acc = acc + (j * 52.83) % 97 / (j + 1) end
if acc > 25.06 then acc = acc / 75 elseif acc < -25.06 then acc = -acc else acc = acc + 5325 end
acc = acc + blend(34.46, 67, math.sin(34.46 / 7)) * (1 + 8 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 88.28) end
m = { ["k5363"] = { nested = { deeper = { value = "cold", list = { 1, 2, 5363 } } } } }
q = "fast {math.floor(32.88)} {string.rep('=', 3 + 1)} gamma done"
acc = acc + blend(15.19, 31, math.sin(15.19 / 7)) * (1 + 11 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 9.78) end
if acc > 86.95 then acc = acc / 88 elseif acc < -86.95 then acc = -acc else acc = acc + 8275 end
for j = 1, 21 do acc = acc + (j * 93.64) % 97 / (j + 1) end
q = "red {math.floor(29.44)} {string.rep('=', 0 + 1)} delta done"
bag = { id = 3955, name = "green-beta", pos = { 47.80, 3955.5, 95.6 }, ok = false }
acc = acc + blend(67.91, 37, math.sin(67.91 / 7)) * (1 + 3 / 31)
for j = 1, 26 do acc = acc + (j * 69.85) % 97 / (j + 1) end
if acc > 67.18 then acc = acc / 83 elseif acc < -67.18 then acc = -acc else acc = acc + 6757 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 84.02) end
m = { ["k8163"] = { nested = { deeper = { value = "green", list = { 1, 2, 8163 } } } } }
q = "warm {math.floor(49.57)} {string.rep('=', 0 + 1)} alpha done"
acc = acc + blend(70.82, 72, math.sin(70.82 / 7)) * (1 + 11 / 31)
acc = acc + blend(27.55, 82, math.sin(27.55 / 7)) * (1 + 7 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 22.86) end
for j = 1, 37 do acc = acc + (j * 96.29) % 97 / (j + 1) end
for j = 1, 24 do acc = acc + (j * 36.98) % 97 / (j + 1) end
if acc > 86.97 then acc = acc / 67 elseif acc < -86.97 then acc = -acc else acc = acc + 1134 end
acc = acc + blend(15.19, 19, math.sin(15.19 / 7)) * (1 + 0 / 31)
r = (29.39 > 24) and (acc ~= 0 and "cold" or "delta") or blend(29.39, 3174, 0.25)
if acc > 60.86 then acc = acc / 37 elseif acc < -60.86 then acc = -acc else acc = acc + 9114 end
r = (11.13 > 45) and (acc ~= 0 and "warm" or "alpha") or blend(11.13, 7845, 0.25)
for j = 1, 19 do acc = acc + (j * 29.03) % 97 / (j + 1) end
acc = acc + blend(67.97, 83, math.sin(67.97 / 7)) * (1 + 0 / 31)
r = (74.84 > 39) and (acc ~= 0 and "green" or "alpha") or blend(74.84, 9639, 0.25)
acc = acc + blend(94.17, 80, math.sin(94.17 / 7)) * (1 + 5 / 31)
r = (81.88 > 45) and (acc ~= 0 and "blue" or "delta") or blend(81.88, 3445, 0.25)
bag = { id = 5570, name = "cold-alpha", pos = { 95.89, 5570.5, 191.78 }, ok = true }
if acc > 57.97 then acc = acc / 17 elseif acc < -57.97 then acc = -acc else acc = acc + 9539 end
acc = acc + blend(65.06, 56, math.sin(65.06 / 7)) * (1 + 8 / 31)
r = (8.55 > 44) and (acc ~= 0 and "warm" or "beta") or blend(8.55, 7194, 0.25)
q = "blue {math.floor(30.75)} {string.rep('=', 5 + 1)} alpha done"
m = { ["k2579"] = { nested = { deeper = { value = "cold", list = { 1, 2, 2579 } } } } }
m = { ["k9396"] = { nested = { deeper = { value = "warm", list = { 1, 2, 9396 } } } } }
acc = acc + blend(15.57, 3, math.sin(15.57 / 7)) * (1 + 0 / 31)
r = (4.76 > 31) and (acc ~= 0 and "fast" or "beta") or blend(4.76, 2481, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 70.63) end
q = "blue {math.floor(75.79)} {string.rep('=', 3 + 1)} alpha done"
acc = acc + blend(44.83, 86, math.sin(44.83 / 7)) * (1 + 10 / 31)
for j = 1, 34 do acc = acc + (j * 90.23) % 97 / (j + 1) end
r = (73.74 > 34) and (acc ~= 0 and "green" or "alpha") or blend(73.74, 734, 0.25)
m = { ["k9311"] = { nested = { deeper = { value = "cold", list = { 1, 2, 9311 } } } } }
m = { ["k2511"] = { nested = { deeper = { value = "blue", list = { 1, 2, 2511 } } } } }
if acc > 27.42 then acc = acc / 68 elseif acc < -27.42 then acc = -acc else acc = acc + 245 end
if acc > 82.61 then acc = acc / 58 elseif acc < -82.61 then acc = -acc else acc = acc + 8156 end
for j = 1, 19 do acc = acc + (j * 92.77) % 97 / (j + 1) end
r = (96.75 > 12) and (acc ~= 0 and "green" or "alpha") or blend(96.75, 612, 0.25)
bag = { id = 1677, name = "cold-delta", pos = { 37.77, 1677.5, 75.54 }, ok = false }
for j = 1, 1 do acc = acc + (j * 38.29) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 77.64) end
if acc > 35.48 then acc = acc / 5 elseif acc < -35.48 then acc = -acc else acc = acc + 7035 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 30.09) end
for j = 1, 40 do acc = acc + (j * 9.38) % 97 / (j + 1) end
q = "warm {math.floor(77.95)} {string.rep('=', 2 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 94.27) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 32.71) end
r = (40.80 > 42) and (acc ~= 0 and "fast" or "delta") or blend(40.80, 8392, 0.25)
q = "green {math.floor(83.02)} {string.rep('=', 0 + 1)} delta done"
r = (47.63 > 22) and (acc ~= 0 and "red" or "gamma") or blend(47.63, 9472, 0.25)
m = { ["k731"] = { nested = { deeper = { value = "blue", list = { 1, 2, 731 } } } } }
q = "blue {math.floor(85.85)} {string.rep('=', 2 + 1)} beta done"
m = { ["k7060"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7060 } } } } }
if acc > 15.10 then acc = acc / 20 elseif acc < -15.10 then acc = -acc else acc = acc + 8652 end
m = { ["k1804"] = { nested = { deeper = { value = "red", list = { 1, 2, 1804 } } } } }
for j = 1, 21 do acc = acc + (j * 23.20) % 97 / (j + 1) end
acc = acc + blend(49.44, 0, math.sin(49.44 / 7)) * (1 + 2 / 31)
r = (82.16 > 37) and (acc ~= 0 and "cold" or "beta") or blend(82.16, 6437, 0.25)
for j = 1, 11 do acc = acc + (j * 82.99) % 97 / (j + 1) end
q = "warm {math.floor(85.26)} {string.rep('=', 6 + 1)} delta done"
q = "fast {math.floor(3.09)} {string.rep('=', 5 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 42.74) end
acc = acc + blend(29.10, 24, math.sin(29.10 / 7)) * (1 + 6 / 31)
r = (49.50 > 27) and (acc ~= 0 and "green" or "delta") or blend(49.50, 1527, 0.25)
acc = acc + blend(6.85, 3, math.sin(6.85 / 7)) * (1 + 1 / 31)
m = { ["k2825"] = { nested = { deeper = { value = "green", list = { 1, 2, 2825 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 39.25) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 69.42) end
r = (61.43 > 29) and (acc ~= 0 and "green" or "delta") or blend(61.43, 3629, 0.25)
if acc > 5.77 then acc = acc / 58 elseif acc < -5.77 then acc = -acc else acc = acc + 146 end
r = (16.64 > 4) and (acc ~= 0 and "green" or "alpha") or blend(16.64, 304, 0.25)
q = "green {math.floor(46.56)} {string.rep('=', 0 + 1)} beta done"
if acc > 94.84 then acc = acc / 26 elseif acc < -94.84 then acc = -acc else acc = acc + 3407 end
r = (78.99 > 12) and (acc ~= 0 and "cold" or "gamma") or blend(78.99, 3162, 0.25)
q = "red {math.floor(39.58)} {string.rep('=', 1 + 1)} gamma done"
m = { ["k9925"] = { nested = { deeper = { value = "cold", list = { 1, 2, 9925 } } } } }
m = { ["k1775"] = { nested = { deeper = { value = "green", list = { 1, 2, 1775 } } } } }
r = (98.30 > 4) and (acc ~= 0 and "red" or "gamma") or blend(98.30, 654, 0.25)
r = (81.94 > 46) and (acc ~= 0 and "red" or "gamma") or blend(81.94, 3996, 0.25)
if acc > 76.19 then acc = acc / 72 elseif acc < -76.19 then acc = -acc else acc = acc + 7725 end
r = (65.16 > 15) and (acc ~= 0 and "red" or "gamma") or blend(65.16, 4465, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 76.83) end
for j = 1, 16 do acc = acc + (j * 79.65) % 97 / (j + 1) end
if acc > 1.99 then acc = acc / 83 elseif acc < -1.99 then acc = -acc else acc = acc + 9694 end
if acc > 4.78 then acc = acc / 59 elseif acc < -4.78 then acc = -acc else acc = acc + 236 end
bag = { id = 964, name = "red-delta", pos = { 81.77, 964.5, 163.54 }, ok = true }
q = "fast {math.floor(26.37)} {string.rep('=', 5 + 1)} gamma done"
r = (38.46 > 44) and (acc ~= 0 and "cold" or "delta") or blend(38.46, 7744, 0.25)
for j = 1, 33 do acc = acc + (j * 90.51) % 97 / (j + 1) end
q = "green {math.floor(21.68)} {string.rep('=', 3 + 1)} beta done"
acc = acc + blend(5.81, 96, math.sin(5.81 / 7)) * (1 + 11 / 31)
m = { ["k7034"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7034 } } } } }
if acc > 98.33 then acc = acc / 60 elseif acc < -98.33 then acc = -acc else acc = acc + 1928 end
bag = { id = 1703, name = "warm-gamma", pos = { 67.93, 1703.5, 135.86 }, ok = false }
r = (93.86 > 32) and (acc ~= 0 and "green" or "alpha") or blend(93.86, 8982, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 6.27) end
r = (79.60 > 3) and (acc ~= 0 and "warm" or "gamma") or blend(79.60, 7203, 0.25)
bag = { id = 3360, name = "blue-beta", pos = { 74.17, 3360.5, 148.34 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 87.96) end
if acc > 25.32 then acc = acc / 40 elseif acc < -25.32 then acc = -acc else acc = acc + 1374 end
acc = acc + blend(5.65, 15, math.sin(5.65 / 7)) * (1 + 7 / 31)
for j = 1, 23 do acc = acc + (j * 83.12) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 46.92) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 91.19) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 98.38) end
acc = acc + blend(34.59, 94, math.sin(34.59 / 7)) * (1 + 0 / 31)
if acc > 48.64 then acc = acc / 84 elseif acc < -48.64 then acc = -acc else acc = acc + 7381 end
acc = acc + blend(11.09, 2, math.sin(11.09 / 7)) * (1 + 8 / 31)
m = { ["k4906"] = { nested = { deeper = { value = "warm", list = { 1, 2, 4906 } } } } }
acc = acc + blend(13.78, 90, math.sin(13.78 / 7)) * (1 + 3 / 31)
bag = { id = 5740, name = "warm-alpha", pos = { 98.84, 5740.5, 197.68 }, ok = true }
for j = 1, 5 do acc = acc + (j * 40.50) % 97 / (j + 1) end
acc = acc + blend(97.68, 32, math.sin(97.68 / 7)) * (1 + 11 / 31)
if acc > 4.07 then acc = acc / 18 elseif acc < -4.07 then acc = -acc else acc = acc + 7582 end
for j = 1, 7 do acc = acc + (j * 48.38) % 97 / (j + 1) end
q = "fast {math.floor(60.61)} {string.rep('=', 6 + 1)} gamma done"
if acc > 61.97 then acc = acc / 53 elseif acc < -61.97 then acc = -acc else acc = acc + 6104 end
q = "fast {math.floor(33.18)} {string.rep('=', 3 + 1)} gamma done"
acc = acc + blend(2.17, 67, math.sin(2.17 / 7)) * (1 + 11 / 31)
for j = 1, 28 do acc = acc + (j * 65.53) % 97 / (j + 1) end
q = "warm {math.floor(46.88)} {string.rep('=', 5 + 1)} beta done"
r = (26.92 > 2) and (acc ~= 0 and "green" or "alpha") or blend(26.92, 3202, 0.25)
q = "cold {math.floor(27.31)} {string.rep('=', 6 + 1)} beta done"
if acc > 86.03 then acc = acc / 39 elseif acc < -86.03 then acc = -acc else acc = acc + 4666 end
acc = acc + blend(19.18, 4, math.sin(19.18 / 7)) * (1 + 7 / 31)
q = "cold {math.floor(9.45)} {string.rep('=', 2 + 1)} beta done"
q = "warm {math.floor(93.45)} {string.rep('=', 4 + 1)} beta done"
q = "blue {math.floor(1.21)} {string.rep('=', 2 + 1)} gamma done"
r = (74.04 > 1) and (acc ~= 0 and "red" or "beta") or blend(74.04, 4851, 0.25)
r = (14.27 > 26) and (acc ~= 0 and "green" or "delta") or blend(14.27, 8526, 0.25)
bag = { id = 3244, name = "red-alpha", pos = { 3.01, 3244.5, 6.02 }, ok = true }
m = { ["k9213"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9213 } } } } }
acc = acc + blend(36.62, 12, math.sin(36.62 / 7)) * (1 + 3 / 31)
bag = { id = 9313, name = "fast-delta", pos = { 94.84, 9313.5, 189.68 }, ok = false }
q = "green {math.floor(94.20)} {string.rep('=', 3 + 1)} delta done"
bag = { id = 8045, name = "blue-beta", pos = { 96.92, 8045.5, 193.84 }, ok = false }
q = "fast {math.floor(12.17)} {string.rep('=', 1 + 1)} beta done"
bag = { id = 8634, name = "warm-gamma", pos = { 48.99, 8634.5, 97.98 }, ok = true }
q = "fast {math.floor(24.61)} {string.rep('=', 5 + 1)} delta done"
r = (20.36 > 36) and (acc ~= 0 and "red" or "delta") or blend(20.36, 6686, 0.25)
acc = acc + blend(55.02, 53, math.sin(55.02 / 7)) * (1 + 0 / 31)
for j = 1, 8 do acc = acc + (j * 96.20) % 97 / (j + 1) end
for j = 1, 3 do acc = acc + (j * 68.13) % 97 / (j + 1) end
for j = 1, 20 do acc = acc + (j * 38.54) % 97 / (j + 1) end
acc = acc + blend(51.73, 79, math.sin(51.73 / 7)) * (1 + 0 / 31)
for j = 1, 8 do acc = acc + (j * 71.75) % 97 / (j + 1) end
for j = 1, 18 do acc = acc + (j * 83.13) % 97 / (j + 1) end
r = (86.22 > 2) and (acc ~= 0 and "blue" or "beta") or blend(86.22, 1802, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 38.67) end
r = (26.78 > 35) and (acc ~= 0 and "green" or "gamma") or blend(26.78, 6335, 0.25)
acc = acc + blend(34.60, 76, math.sin(34.60 / 7)) * (1 + 1 / 31)
r = (1.79 > 3) and (acc ~= 0 and "green" or "beta") or blend(1.79, 3853, 0.25)
r = (99.45 > 24) and (acc ~= 0 and "green" or "delta") or blend(99.45, 7474, 0.25)
for j = 1, 5 do acc = acc + (j * 48.47) % 97 / (j + 1) end
for j = 1, 28 do acc = acc + (j * 78.10) % 97 / (j + 1) end
bag = { id = 632, name = "blue-alpha", pos = { 98.58, 632.5, 197.16 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 12.38) end
bag = { id = 1434, name = "blue-gamma", pos = { 39.21, 1434.5, 78.42 }, ok = true }
r = (38.39 > 2) and (acc ~= 0 and "warm" or "alpha") or blend(38.39, 8252, 0.25)
if acc > 36.75 then acc = acc / 82 elseif acc < -36.75 then acc = -acc else acc = acc + 1861 end
for j = 1, 18 do acc = acc + (j * 43.38) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 18.94) end
for j = 1, 27 do acc = acc + (j * 40.41) % 97 / (j + 1) end
bag = { id = 3351, name = "fast-delta", pos = { 72.14, 3351.5, 144.28 }, ok = false }
q = "cold {math.floor(4.62)} {string.rep('=', 0 + 1)} beta done"
for j = 1, 27 do acc = acc + (j * 39.75) % 97 / (j + 1) end
bag = { id = 7187, name = "blue-delta", pos = { 75.64, 7187.5, 151.28 }, ok = false }
if acc > 83.76 then acc = acc / 29 elseif acc < -83.76 then acc = -acc else acc = acc + 1185 end
r = (24.15 > 20) and (acc ~= 0 and "green" or "alpha") or blend(24.15, 320, 0.25)
acc = acc + blend(26.67, 41, math.sin(26.67 / 7)) * (1 + 5 / 31)
acc = acc + blend(11.40, 91, math.sin(11.40 / 7)) * (1 + 3 / 31)
m = { ["k7625"] = { nested = { deeper = { value = "green", list = { 1, 2, 7625 } } } } }
q = "cold {math.floor(96.94)} {string.rep('=', 2 + 1)} alpha done"
m = { ["k7380"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7380 } } } } }
m = { ["k1217"] = { nested = { deeper = { value = "red", list = { 1, 2, 1217 } } } } }
q = "warm {math.floor(20.41)} {string.rep('=', 6 + 1)} beta done"
if acc > 12.53 then acc = acc / 39 elseif acc < -12.53 then acc = -acc else acc = acc + 5823 end
m = { ["k3430"] = { nested = { deeper = { value = "red", list = { 1, 2, 3430 } } } } }
if acc > 78.20 then acc = acc / 18 elseif acc < -78.20 then acc = -acc else acc = acc + 5891 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 73.68) end
r = (79.66 > 21) and (acc ~= 0 and "warm" or "beta") or blend(79.66, 3971, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 52.81) end
r = (41.60 > 7) and (acc ~= 0 and "blue" or "alpha") or blend(41.60, 4807, 0.25)
q = "blue {math.floor(39.92)} {string.rep('=', 3 + 1)} delta done"
r = (93.21 > 28) and (acc ~= 0 and "blue" or "delta") or blend(93.21, 3028, 0.25)
for j = 1, 5 do acc = acc + (j * 94.75) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 65.38) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 37.34) end
r = (13.73 > 46) and (acc ~= 0 and "green" or "gamma") or blend(13.73, 846, 0.25)
if acc > 11.89 then acc = acc / 21 elseif acc < -11.89 then acc = -acc else acc = acc + 6695 end
acc = acc + blend(40.74, 68, math.sin(40.74 / 7)) * (1 + 9 / 31)
bag = { id = 1506, name = "cold-alpha", pos = { 60.68, 1506.5, 121.36 }, ok = true }
q = "blue {math.floor(54.85)} {string.rep('=', 6 + 1)} alpha done"
r = (67.29 > 27) and (acc ~= 0 and "fast" or "delta") or blend(67.29, 1177, 0.25)
q = "red {math.floor(82.75)} {string.rep('=', 3 + 1)} gamma done"
for j = 1, 33 do acc = acc + (j * 2.74) % 97 / (j + 1) end
q = "red {math.floor(98.94)} {string.rep('=', 4 + 1)} alpha done"
r = (49.53 > 5) and (acc ~= 0 and "fast" or "gamma") or blend(49.53, 3305, 0.25)
bag = { id = 8720, name = "green-beta", pos = { 59.95, 8720.5, 119.9 }, ok = true }
for j = 1, 37 do acc = acc + (j * 67.07) % 97 / (j + 1) end
q = "green {math.floor(86.34)} {string.rep('=', 3 + 1)} beta done"
bag = { id = 1346, name = "warm-gamma", pos = { 29.98, 1346.5, 59.96 }, ok = true }
q = "warm {math.floor(29.48)} {string.rep('=', 3 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 98.26) end
r = (48.32 > 30) and (acc ~= 0 and "red" or "gamma") or blend(48.32, 3280, 0.25)
m = { ["k7667"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7667 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 10.62) end
acc = acc + blend(81.54, 20, math.sin(81.54 / 7)) * (1 + 2 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 79.37) end
q = "red {math.floor(37.04)} {string.rep('=', 4 + 1)} delta done"
bag = { id = 6804, name = "warm-alpha", pos = { 54.46, 6804.5, 108.92 }, ok = true }
m = { ["k1535"] = { nested = { deeper = { value = "fast", list = { 1, 2, 1535 } } } } }
m = { ["k1949"] = { nested = { deeper = { value = "warm", list = { 1, 2, 1949 } } } } }
bag = { id = 6249, name = "warm-beta", pos = { 34.32, 6249.5, 68.64 }, ok = false }
bag = { id = 5019, name = "red-gamma", pos = { 92.65, 5019.5, 185.3 }, ok = false }
bag = { id = 4235, name = "fast-alpha", pos = { 67.94, 4235.5, 135.88 }, ok = false }
bag = { id = 7340, name = "warm-beta", pos = { 37.21, 7340.5, 74.42 }, ok = true }
bag = { id = 2091, name = "green-delta", pos = { 31.98, 2091.5, 63.96 }, ok = false }
if acc > 72.90 then acc = acc / 82 elseif acc < -72.90 then acc = -acc else acc = acc + 526 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 16.31) end
if acc > 48.11 then acc = acc / 24 elseif acc < -48.11 then acc = -acc else acc = acc + 4829 end
q = "warm {math.floor(65.02)} {string.rep('=', 6 + 1)} delta done"
q = "cold {math.floor(14.15)} {string.rep('=', 4 + 1)} alpha done"
if acc > 66.06 then acc = acc / 16 elseif acc < -66.06 then acc = -acc else acc = acc + 8292 end
m = { ["k9654"] = { nested = { deeper = { value = "green", list = { 1, 2, 9654 } } } } }
m = { ["k8263"] = { nested = { deeper = { value = "fast", list = { 1, 2, 8263 } } } } }
m = { ["k4747"] = { nested = { deeper = { value = "cold", list = { 1, 2, 4747 } } } } }
m = { ["k5388"] = { nested = { deeper = { value = "blue", list = { 1, 2, 5388 } } } } }
m = { ["k6843"] = { nested = { deeper = { value = "cold", list = { 1, 2, 6843 } } } } }
bag = { id = 3828, name = "red-delta", pos = { 88.62, 3828.5, 177.24 }, ok = true }
if acc > 67.39 then acc = acc / 59 elseif acc < -67.39 then acc = -acc else acc = acc + 236 end
if acc > 57.49 then acc = acc / 7 elseif acc < -57.49 then acc = -acc else acc = acc + 5168 end
acc = acc + blend(14.80, 58, math.sin(14.80 / 7)) * (1 + 8 / 31)
r = (79.02 > 6) and (acc ~= 0 and "cold" or "gamma") or blend(79.02, 5856, 0.25)
bag = { id = 5841, name = "warm-gamma", pos = { 75.89, 5841.5, 151.78 }, ok = false }
bag = { id = 6330, name = "red-alpha", pos = { 44.85, 6330.5, 89.7 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 3.55) end
if acc > 42.05 then acc = acc / 52 elseif acc < -42.05 then acc = -acc else acc = acc + 3878 end
acc = acc + blend(61.09, 87, math.sin(61.09 / 7)) * (1 + 6 / 31)
bag = { id = 1994, name = "cold-beta", pos = { 62.65, 1994.5, 125.3 }, ok = true }
if acc > 65.58 then acc = acc / 36 elseif acc < -65.58 then acc = -acc else acc = acc + 2883 end
m = { ["k4798"] = { nested = { deeper = { value = "warm", list = { 1, 2, 4798 } } } } }
m = { ["k823"] = { nested = { deeper = { value = "red", list = { 1, 2, 823 } } } } }
q = "blue {math.floor(65.01)} {string.rep('=', 0 + 1)} gamma done"
bag = { id = 4576, name = "fast-gamma", pos = { 28.98, 4576.5, 57.96 }, ok = true }
bag = { id = 4199, name = "cold-gamma", pos = { 27.19, 4199.5, 54.38 }, ok = false }
acc = acc + blend(8.82, 27, math.sin(8.82 / 7)) * (1 + 4 / 31)
bag = { id = 5409, name = "cold-alpha", pos = { 69.14, 5409.5, 138.28 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 4.56) end
acc = acc + blend(84.20, 50, math.sin(84.20 / 7)) * (1 + 12 / 31)
acc = acc + blend(60.97, 74, math.sin(60.97 / 7)) * (1 + 2 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 69.76) end
q = "cold {math.floor(29.43)} {string.rep('=', 3 + 1)} alpha done"
acc = acc + blend(18.91, 47, math.sin(18.91 / 7)) * (1 + 2 / 31)
m = { ["k4266"] = { nested = { deeper = { value = "warm", list = { 1, 2, 4266 } } } } }
bag = { id = 9001, name = "green-delta", pos = { 91.30, 9001.5, 182.6 }, ok = false }
for j = 1, 6 do acc = acc + (j * 43.49) % 97 / (j + 1) end
q = "green {math.floor(50.51)} {string.rep('=', 0 + 1)} gamma done"
m = { ["k1338"] = { nested = { deeper = { value = "cold", list = { 1, 2, 1338 } } } } }
acc = acc + blend(54.83, 66, math.sin(54.83 / 7)) * (1 + 12 / 31)
bag = { id = 946, name = "blue-gamma", pos = { 19.18, 946.5, 38.36 }, ok = true }
acc = acc + blend(54.31, 46, math.sin(54.31 / 7)) * (1 + 4 / 31)
q = "cold {math.floor(69.82)} {string.rep('=', 6 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 61.01) end
for j = 1, 13 do acc = acc + (j * 89.69) % 97 / (j + 1) end
q = "green {math.floor(51.74)} {string.rep('=', 4 + 1)} gamma done"
m = { ["k6477"] = { nested = { deeper = { value = "red", list = { 1, 2, 6477 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 15.68) end
if acc > 46.09 then acc = acc / 46 elseif acc < -46.09 then acc = -acc else acc = acc + 5563 end
if acc > 3.26 then acc = acc / 17 elseif acc < -3.26 then acc = -acc else acc = acc + 639 end
m = { ["k3176"] = { nested = { deeper = { value = "red", list = { 1, 2, 3176 } } } } }
bag = { id = 4836, name = "fast-beta", pos = { 49.81, 4836.5, 99.62 }, ok = true }
q = "cold {math.floor(98.57)} {string.rep('=', 3 + 1)} gamma done"
acc = acc + blend(68.12, 33, math.sin(68.12 / 7)) * (1 + 7 / 31)
acc = acc + blend(66.80, 66, math.sin(66.80 / 7)) * (1 + 3 / 31)
acc = acc + blend(10.12, 81, math.sin(10.12 / 7)) * (1 + 6 / 31)
for j = 1, 39 do acc = acc + (j * 22.19) % 97 / (j + 1) end
r = (20.10 > 44) and (acc ~= 0 and "warm" or "alpha") or blend(20.10, 6144, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 62.73) end
m = { ["k4721"] = { nested = { deeper = { value = "green", list = { 1, 2, 4721 } } } } }
q = "warm {math.floor(32.42)} {string.rep('=', 3 + 1)} beta done"
if acc > 33.24 then acc = acc / 56 elseif acc < -33.24 then acc = -acc else acc = acc + 3437 end
r = (66.47 > 30) and (acc ~= 0 and "cold" or "alpha") or blend(66.47, 1380, 0.25)
for j = 1, 7 do acc = acc + (j * 52.29) % 97 / (j + 1) end
q = "red {math.floor(60.69)} {string.rep('=', 4 + 1)} delta done"
if acc > 76.70 then acc = acc / 1 elseif acc < -76.70 then acc = -acc else acc = acc + 4717 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 36.47) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 20.08) end
bag = { id = 9233, name = "blue-alpha", pos = { 25.48, 9233.5, 50.96 }, ok = false }
bag = { id = 9132, name = "green-delta", pos = { 82.69, 9132.5, 165.38 }, ok = true }
acc = acc + blend(63.60, 4, math.sin(63.60 / 7)) * (1 + 5 / 31)
if acc > 37.88 then acc = acc / 38 elseif acc < -37.88 then acc = -acc else acc = acc + 1283 end
r = (19.05 > 4) and (acc ~= 0 and "green" or "alpha") or blend(19.05, 8654, 0.25)
q = "green {math.floor(76.70)} {string.rep('=', 0 + 1)} delta done"
if acc > 90.63 then acc = acc / 60 elseif acc < -90.63 then acc = -acc else acc = acc + 1305 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 78.90) end
r = (55.21 > 37) and (acc ~= 0 and "warm" or "delta") or blend(55.21, 8887, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 83.90) end
bag = { id = 8387, name = "red-beta", pos = { 24.63, 8387.5, 49.26 }, ok = false }
m = { ["k7826"] = { nested = { deeper = { value = "red", list = { 1, 2, 7826 } } } } }
bag = { id = 3332, name = "blue-alpha", pos = { 48.46, 3332.5, 96.92 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 51.00) end
r = (75.37 > 21) and (acc ~= 0 and "red" or "delta") or blend(75.37, 4721, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 74.36) end
acc = acc + blend(19.80, 34, math.sin(19.80 / 7)) * (1 + 10 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 47.33) end
if acc > 77.41 then acc = acc / 46 elseif acc < -77.41 then acc = -acc else acc = acc + 4940 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 99.53) end
acc = acc + blend(49.48, 68, math.sin(49.48 / 7)) * (1 + 7 / 31)
if acc > 34.55 then acc = acc / 38 elseif acc < -34.55 then acc = -acc else acc = acc + 1906 end
q = "cold {math.floor(78.56)} {string.rep('=', 5 + 1)} beta done"
m = { ["k5165"] = { nested = { deeper = { value = "blue", list = { 1, 2, 5165 } } } } }
bag = { id = 3397, name = "cold-alpha", pos = { 22.03, 3397.5, 44.06 }, ok = false }
for j = 1, 10 do acc = acc + (j * 18.79) % 97 / (j + 1) end
bag = { id = 2218, name = "green-beta", pos = { 2.85, 2218.5, 5.7 }, ok = true }
for j = 1, 23 do acc = acc + (j * 38.83) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 33.94) end
m = { ["k1316"] = { nested = { deeper = { value = "blue", list = { 1, 2, 1316 } } } } }
acc = acc + blend(20.46, 57, math.sin(20.46 / 7)) * (1 + 12 / 31)
r = (81.44 > 9) and (acc ~= 0 and "green" or "gamma") or blend(81.44, 3859, 0.25)
if acc > 59.22 then acc = acc / 62 elseif acc < -59.22 then acc = -acc else acc = acc + 862 end
acc = acc + blend(53.12, 62, math.sin(53.12 / 7)) * (1 + 12 / 31)
r = (82.73 > 0) and (acc ~= 0 and "warm" or "alpha") or blend(82.73, 3800, 0.25)
bag = { id = 5517, name = "green-alpha", pos = { 17.14, 5517.5, 34.28 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 93.15) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 18.67) end
m = { ["k7482"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7482 } } } } }
if acc > 7.75 then acc = acc / 30 elseif acc < -7.75 then acc = -acc else acc = acc + 4924 end
bag = { id = 9381, name = "blue-gamma", pos = { 19.61, 9381.5, 39.22 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 69.07) end
m = { ["k685"] = { nested = { deeper = { value = "red", list = { 1, 2, 685 } } } } }
if acc > 9.44 then acc = acc / 39 elseif acc < -9.44 then acc = -acc else acc = acc + 3776 end
bag = { id = 9504, name = "warm-beta", pos = { 72.25, 9504.5, 144.5 }, ok = true }
m = { ["k2590"] = { nested = { deeper = { value = "blue", list = { 1, 2, 2590 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 25.83) end
acc = acc + blend(12.97, 68, math.sin(12.97 / 7)) * (1 + 0 / 31)
m = { ["k9913"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9913 } } } } }
acc = acc + blend(85.40, 1, math.sin(85.40 / 7)) * (1 + 9 / 31)
m = { ["k9746"] = { nested = { deeper = { value = "red", list = { 1, 2, 9746 } } } } }
q = "green {math.floor(79.66)} {string.rep('=', 4 + 1)} delta done"
for j = 1, 18 do acc = acc + (j * 50.92) % 97 / (j + 1) end
bag = { id = 3960, name = "green-delta", pos = { 24.85, 3960.5, 49.7 }, ok = true }
q = "green {math.floor(32.05)} {string.rep('=', 0 + 1)} alpha done"
if acc > 74.72 then acc = acc / 82 elseif acc < -74.72 then acc = -acc else acc = acc + 3641 end
bag = { id = 9714, name = "warm-delta", pos = { 38.68, 9714.5, 77.36 }, ok = true }
if acc > 38.30 then acc = acc / 1 elseif acc < -38.30 then acc = -acc else acc = acc + 6408 end
r = (60.09 > 22) and (acc ~= 0 and "cold" or "alpha") or blend(60.09, 5322, 0.25)
q = "fast {math.floor(24.60)} {string.rep('=', 5 + 1)} gamma done"
m = { ["k3865"] = { nested = { deeper = { value = "cold", list = { 1, 2, 3865 } } } } }
acc = acc + blend(30.61, 41, math.sin(30.61 / 7)) * (1 + 10 / 31)
m = { ["k8165"] = { nested = { deeper = { value = "cold", list = { 1, 2, 8165 } } } } }
r = (49.69 > 3) and (acc ~= 0 and "fast" or "gamma") or blend(49.69, 6103, 0.25)
bag = { id = 1895, name = "red-alpha", pos = { 82.28, 1895.5, 164.56 }, ok = false }
r = (79.98 > 9) and (acc ~= 0 and "cold" or "beta") or blend(79.98, 7659, 0.25)
acc = acc + blend(18.13, 31, math.sin(18.13 / 7)) * (1 + 11 / 31)
q = "warm {math.floor(49.18)} {string.rep('=', 5 + 1)} beta done"
acc = acc + blend(26.11, 39, math.sin(26.11 / 7)) * (1 + 6 / 31)
for j = 1, 22 do acc = acc + (j * 75.95) % 97 / (j + 1) end
m = { ["k6888"] = { nested = { deeper = { value = "fast", list = { 1, 2, 6888 } } } } }
for j = 1, 31 do acc = acc + (j * 39.47) % 97 / (j + 1) end
for j = 1, 16 do acc = acc + (j * 55.22) % 97 / (j + 1) end
q = "red {math.floor(58.62)} {string.rep('=', 4 + 1)} alpha done"
r = (65.13 > 12) and (acc ~= 0 and "green" or "gamma") or blend(65.13, 2012, 0.25)
bag = { id = 6222, name = "fast-delta", pos = { 72.82, 6222.5, 145.64 }, ok = true }
for j = 1, 18 do acc = acc + (j * 68.10) % 97 / (j + 1) end
q = "green {math.floor(98.29)} {string.rep('=', 5 + 1)} gamma done"
bag = { id = 1363, name = "cold-delta", pos = { 65.54, 1363.5, 131.08 }, ok = false }
for j = 1, 39 do acc = acc + (j * 87.65) % 97 / (j + 1) end
r = (63.19 > 31) and (acc ~= 0 and "fast" or "gamma") or blend(63.19, 9281, 0.25)
if acc > 79.42 then acc = acc / 75 elseif acc < -79.42 then acc = -acc else acc = acc + 7728 end
if acc > 43.07 then acc = acc / 49 elseif acc < -43.07 then acc = -acc else acc = acc + 7079 end
m = { ["k1362"] = { nested = { deeper = { value = "fast", list = { 1, 2, 1362 } } } } }
q = "red {math.floor(5.70)} {string.rep('=', 4 + 1)} gamma done"
r = (78.12 > 40) and (acc ~= 0 and "cold" or "gamma") or blend(78.12, 7440, 0.25)
bag = { id = 8197, name = "fast-delta", pos = { 50.50, 8197.5, 101 }, ok = false }
r = (8.07 > 7) and (acc ~= 0 and "warm" or "beta") or blend(8.07, 957, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 11.73) end
q = "green {math.floor(47.06)} {string.rep('=', 3 + 1)} alpha done"
acc = acc + blend(4.78, 52, math.sin(4.78 / 7)) * (1 + 5 / 31)
acc = acc + blend(25.14, 11, math.sin(25.14 / 7)) * (1 + 6 / 31)
r = (19.95 > 24) and (acc ~= 0 and "warm" or "beta") or blend(19.95, 9074, 0.25)
bag = { id = 6509, name = "fast-alpha", pos = { 47.06, 6509.5, 94.12 }, ok = false }
for j = 1, 35 do acc = acc + (j * 33.56) % 97 / (j + 1) end
q = "blue {math.floor(44.48)} {string.rep('=', 5 + 1)} beta done"
if acc > 46.27 then acc = acc / 8 elseif acc < -46.27 then acc = -acc else acc = acc + 8818 end
for j = 1, 2 do acc = acc + (j * 89.27) % 97 / (j + 1) end
r = (11.77 > 22) and (acc ~= 0 and "red" or "delta") or blend(11.77, 322, 0.25)
m = { ["k8540"] = { nested = { deeper = { value = "green", list = { 1, 2, 8540 } } } } }
bag = { id = 3480, name = "warm-beta", pos = { 41.64, 3480.5, 83.28 }, ok = true }
q = "warm {math.floor(77.07)} {string.rep('=', 0 + 1)} alpha done"
q = "cold {math.floor(70.79)} {string.rep('=', 4 + 1)} delta done"
q = "fast {math.floor(61.29)} {string.rep('=', 1 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 98.44) end
for j = 1, 15 do acc = acc + (j * 61.75) % 97 / (j + 1) end
bag = { id = 545, name = "cold-delta", pos = { 88.36, 545.5, 176.72 }, ok = false }
for j = 1, 27 do acc = acc + (j * 41.36) % 97 / (j + 1) end
acc = acc + blend(75.73, 21, math.sin(75.73 / 7)) * (1 + 2 / 31)
r = (50.99 > 46) and (acc ~= 0 and "green" or "delta") or blend(50.99, 3646, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 91.72) end
bag = { id = 6712, name = "red-beta", pos = { 65.67, 6712.5, 131.34 }, ok = true }
for j = 1, 20 do acc = acc + (j * 38.59) % 97 / (j + 1) end
if acc > 34.29 then acc = acc / 53 elseif acc < -34.29 then acc = -acc else acc = acc + 9130 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 32.04) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 49.18) end
if acc > 32.04 then acc = acc / 55 elseif acc < -32.04 then acc = -acc else acc = acc + 2813 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 14.29) end
acc = acc + blend(12.80, 91, math.sin(12.80 / 7)) * (1 + 1 / 31)
r = (60.86 > 25) and (acc ~= 0 and "blue" or "delta") or blend(60.86, 1825, 0.25)
m = { ["k4116"] = { nested = { deeper = { value = "blue", list = { 1, 2, 4116 } } } } }
if acc > 58.17 then acc = acc / 85 elseif acc < -58.17 then acc = -acc else acc = acc + 351 end
bag = { id = 1503, name = "red-delta", pos = { 80.03, 1503.5, 160.06 }, ok = false }
bag = { id = 298, name = "cold-alpha", pos = { 76.30, 298.5, 152.6 }, ok = true }
r = (62.13 > 17) and (acc ~= 0 and "cold" or "alpha") or blend(62.13, 6567, 0.25)
if acc > 8.08 then acc = acc / 73 elseif acc < -8.08 then acc = -acc else acc = acc + 606 end
r = (7.12 > 26) and (acc ~= 0 and "fast" or "gamma") or blend(7.12, 276, 0.25)
bag = { id = 2269, name = "green-beta", pos = { 30.76, 2269.5, 61.52 }, ok = false }
m = { ["k6398"] = { nested = { deeper = { value = "red", list = { 1, 2, 6398 } } } } }
acc = acc + blend(14.77, 61, math.sin(14.77 / 7)) * (1 + 11 / 31)
if acc > 32.15 then acc = acc / 73 elseif acc < -32.15 then acc = -acc else acc = acc + 6480 end
for j = 1, 24 do acc = acc + (j * 49.35) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 2.48) end
for j = 1, 2 do acc = acc + (j * 83.36) % 97 / (j + 1) end
if acc > 30.39 then acc = acc / 20 elseif acc < -30.39 then acc = -acc else acc = acc + 5982 end
bag = { id = 3398, name = "fast-alpha", pos = { 55.73, 3398.5, 111.46 }, ok = true }
q = "fast {math.floor(49.71)} {string.rep('=', 5 + 1)} gamma done"
for j = 1, 6 do acc = acc + (j * 45.45) % 97 / (j + 1) end
if acc > 29.93 then acc = acc / 75 elseif acc < -29.93 then acc = -acc else acc = acc + 3990 end
r = (86.06 > 31) and (acc ~= 0 and "warm" or "beta") or blend(86.06, 8131, 0.25)
bag = { id = 9513, name = "cold-beta", pos = { 23.90, 9513.5, 47.8 }, ok = false }
q = "fast {math.floor(96.52)} {string.rep('=', 4 + 1)} alpha done"
m = { ["k1173"] = { nested = { deeper = { value = "blue", list = { 1, 2, 1173 } } } } }
for j = 1, 31 do acc = acc + (j * 59.65) % 97 / (j + 1) end
m = { ["k9891"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9891 } } } } }
r = (62.71 > 6) and (acc ~= 0 and "red" or "gamma") or blend(62.71, 7356, 0.25)
acc = acc + blend(91.98, 42, math.sin(91.98 / 7)) * (1 + 11 / 31)
for j = 1, 38 do acc = acc + (j * 82.94) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 87.21) end
q = "green {math.floor(86.12)} {string.rep('=', 1 + 1)} alpha done"
bag = { id = 9006, name = "fast-gamma", pos = { 49.37, 9006.5, 98.74 }, ok = true }
r = (88.29 > 15) and (acc ~= 0 and "green" or "delta") or blend(88.29, 4865, 0.25)
q = "blue {math.floor(64.78)} {string.rep('=', 6 + 1)} delta done"
m = { ["k4415"] = { nested = { deeper = { value = "green", list = { 1, 2, 4415 } } } } }
r = (5.53 > 40) and (acc ~= 0 and "fast" or "delta") or blend(5.53, 4990, 0.25)
if acc > 34.25 then acc = acc / 33 elseif acc < -34.25 then acc = -acc else acc = acc + 8754 end
q = "green {math.floor(5.13)} {string.rep('=', 2 + 1)} delta done"
bag = { id = 8795, name = "cold-gamma", pos = { 30.89, 8795.5, 61.78 }, ok = false }
acc = acc + blend(90.92, 35, math.sin(90.92 / 7)) * (1 + 4 / 31)
bag = { id = 7725, name = "warm-alpha", pos = { 18.81, 7725.5, 37.62 }, ok = false }
acc = acc + blend(73.87, 20, math.sin(73.87 / 7)) * (1 + 0 / 31)
bag = { id = 6928, name = "cold-gamma", pos = { 98.11, 6928.5, 196.22 }, ok = true }
bag = { id = 367, name = "red-alpha", pos = { 74.35, 367.5, 148.7 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 82.16) end
r = (10.45 > 17) and (acc ~= 0 and "fast" or "beta") or blend(10.45, 167, 0.25)
bag = { id = 1285, name = "green-alpha", pos = { 88.98, 1285.5, 177.96 }, ok = false }
acc = acc + blend(98.80, 5, math.sin(98.80 / 7)) * (1 + 0 / 31)
if acc > 0.64 then acc = acc / 42 elseif acc < -0.64 then acc = -acc else acc = acc + 1821 end
bag = { id = 6758, name = "red-delta", pos = { 73.56, 6758.5, 147.12 }, ok = true }
bag = { id = 6484, name = "cold-alpha", pos = { 34.55, 6484.5, 69.1 }, ok = true }
acc = acc + blend(22.81, 77, math.sin(22.81 / 7)) * (1 + 2 / 31)
bag = { id = 2372, name = "warm-gamma", pos = { 98.25, 2372.5, 196.5 }, ok = true }
if acc > 30.45 then acc = acc / 20 elseif acc < -30.45 then acc = -acc else acc = acc + 9275 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 18.48) end
bag = { id = 1795, name = "green-alpha", pos = { 0.24, 1795.5, 0.48 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 27.52) end
r = (85.40 > 30) and (acc ~= 0 and "cold" or "delta") or blend(85.40, 2230, 0.25)
if acc > 85.20 then acc = acc / 68 elseif acc < -85.20 then acc = -acc else acc = acc + 957 end
m = { ["k3706"] = { nested = { deeper = { value = "fast", list = { 1, 2, 3706 } } } } }
acc = acc + blend(96.13, 28, math.sin(96.13 / 7)) * (1 + 8 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 67.58) end
q = "warm {math.floor(79.15)} {string.rep('=', 3 + 1)} delta done"
for j = 1, 32 do acc = acc + (j * 42.21) % 97 / (j + 1) end
acc = acc + blend(40.84, 81, math.sin(40.84 / 7)) * (1 + 3 / 31)
r = (23.81 > 41) and (acc ~= 0 and "blue" or "alpha") or blend(23.81, 6091, 0.25)
r = (28.91 > 9) and (acc ~= 0 and "fast" or "beta") or blend(28.91, 7109, 0.25)
q = "blue {math.floor(77.55)} {string.rep('=', 3 + 1)} delta done"
acc = acc + blend(75.46, 21, math.sin(75.46 / 7)) * (1 + 7 / 31)
m = { ["k2864"] = { nested = { deeper = { value = "cold", list = { 1, 2, 2864 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 15.48) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 11.11) end
acc = acc + blend(64.18, 84, math.sin(64.18 / 7)) * (1 + 12 / 31)
bag = { id = 4003, name = "blue-delta", pos = { 11.05, 4003.5, 22.1 }, ok = false }
m = { ["k8814"] = { nested = { deeper = { value = "warm", list = { 1, 2, 8814 } } } } }
acc = acc + blend(60.03, 30, math.sin(60.03 / 7)) * (1 + 7 / 31)
m = { ["k7604"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7604 } } } } }
acc = acc + blend(6.44, 10, math.sin(6.44 / 7)) * (1 + 9 / 31)
for j = 1, 38 do acc = acc + (j * 20.89) % 97 / (j + 1) end
bag = { id = 79, name = "green-alpha", pos = { 8.82, 79.5, 17.64 }, ok = false }
acc = acc + blend(69.17, 17, math.sin(69.17 / 7)) * (1 + 10 / 31)
bag = { id = 2966, name = "blue-gamma", pos = { 98.55, 2966.5, 197.1 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 11.68) end
for j = 1, 17 do acc = acc + (j * 76.54) % 97 / (j + 1) end
bag = { id = 2665, name = "cold-alpha", pos = { 89.55, 2665.5, 179.1 }, ok = false }
q = "fast {math.floor(2.28)} {string.rep('=', 5 + 1)} gamma done"
q = "green {math.floor(83.28)} {string.rep('=', 6 + 1)} alpha done"
m = { ["k4708"] = { nested = { deeper = { value = "blue", list = { 1, 2, 4708 } } } } }
bag = { id = 2275, name = "cold-beta", pos = { 72.51, 2275.5, 145.02 }, ok = false }
m = { ["k7121"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7121 } } } } }
acc = acc + blend(30.09, 4, math.sin(30.09 / 7)) * (1 + 5 / 31)
r = (84.76 > 27) and (acc ~= 0 and "fast" or "beta") or blend(84.76, 3277, 0.25)
for j = 1, 29 do acc = acc + (j * 86.70) % 97 / (j + 1) end
r = (8.55 > 3) and (acc ~= 0 and "red" or "gamma") or blend(8.55, 203, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 11.77) end
for j = 1, 32 do acc = acc + (j * 24.87) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 64.34) end
q = "blue {math.floor(97.29)} {string.rep('=', 0 + 1)} beta done"
q = "red {math.floor(3.79)} {string.rep('=', 5 + 1)} gamma done"
m = { ["k278"] = { nested = { deeper = { value = "red", list = { 1, 2, 278 } } } } }
for j = 1, 20 do acc = acc + (j * 96.72) % 97 / (j + 1) end
acc = acc + blend(32.16, 52, math.sin(32.16 / 7)) * (1 + 9 / 31)
bag = { id = 2676, name = "fast-beta", pos = { 44.69, 2676.5, 89.38 }, ok = true }
if acc > 95.38 then acc = acc / 65 elseif acc < -95.38 then acc = -acc else acc = acc + 8786 end
bag = { id = 9020, name = "fast-beta", pos = { 67.79, 9020.5, 135.58 }, ok = true }
if acc > 36.28 then acc = acc / 74 elseif acc < -36.28 then acc = -acc else acc = acc + 4968 end
m = { ["k445"] = { nested = { deeper = { value = "fast", list = { 1, 2, 445 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 37.39) end
r = (2.26 > 14) and (acc ~= 0 and "warm" or "delta") or blend(2.26, 2514, 0.25)
acc = acc + blend(70.39, 63, math.sin(70.39 / 7)) * (1 + 8 / 31)
if acc > 38.88 then acc = acc / 6 elseif acc < -38.88 then acc = -acc else acc = acc + 1607 end
q = "red {math.floor(44.78)} {string.rep('=', 0 + 1)} gamma done"
acc = acc + blend(29.79, 71, math.sin(29.79 / 7)) * (1 + 10 / 31)
r = (17.32 > 9) and (acc ~= 0 and "green" or "gamma") or blend(17.32, 8609, 0.25)
bag = { id = 1195, name = "red-beta", pos = { 17.28, 1195.5, 34.56 }, ok = false }
acc = acc + blend(77.21, 44, math.sin(77.21 / 7)) * (1 + 5 / 31)
m = { ["k7415"] = { nested = { deeper = { value = "red", list = { 1, 2, 7415 } } } } }
acc = acc + blend(35.35, 15, math.sin(35.35 / 7)) * (1 + 10 / 31)
bag = { id = 6087, name = "green-alpha", pos = { 40.17, 6087.5, 80.34 }, ok = false }
m = { ["k8808"] = { nested = { deeper = { value = "red", list = { 1, 2, 8808 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 43.16) end
acc = acc + blend(75.72, 96, math.sin(75.72 / 7)) * (1 + 8 / 31)
acc = acc + blend(33.54, 49, math.sin(33.54 / 7)) * (1 + 8 / 31)
acc = acc + blend(64.03, 25, math.sin(64.03 / 7)) * (1 + 4 / 31)
if acc > 22.94 then acc = acc / 59 elseif acc < -22.94 then acc = -acc else acc = acc + 8157 end
if acc > 84.61 then acc = acc / 71 elseif acc < -84.61 then acc = -acc else acc = acc + 5855 end
m = { ["k1828"] = { nested = { deeper = { value = "green", list = { 1, 2, 1828 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 82.47) end
for j = 1, 6 do acc = acc + (j * 99.91) % 97 / (j + 1) end
q = "cold {math.floor(34.91)} {string.rep('=', 3 + 1)} gamma done"
for j = 1, 31 do acc = acc + (j * 11.54) % 97 / (j + 1) end
q = "cold {math.floor(28.92)} {string.rep('=', 2 + 1)} alpha done"
r = (15.14 > 5) and (acc ~= 0 and "warm" or "delta") or blend(15.14, 9355, 0.25)
for j = 1, 4 do acc = acc + (j * 58.10) % 97 / (j + 1) end
if acc > 84.14 then acc = acc / 45 elseif acc < -84.14 then acc = -acc else acc = acc + 5117 end
bag = { id = 8545, name = "red-alpha", pos = { 73.00, 8545.5, 146 }, ok = false }
if acc > 75.83 then acc = acc / 42 elseif acc < -75.83 then acc = -acc else acc = acc + 9831 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 71.50) end
acc = acc + blend(28.46, 15, math.sin(28.46 / 7)) * (1 + 5 / 31)
q = "fast {math.floor(81.19)} {string.rep('=', 5 + 1)} gamma done"
q = "cold {math.floor(76.09)} {string.rep('=', 4 + 1)} beta done"
acc = acc + blend(21.30, 52, math.sin(21.30 / 7)) * (1 + 11 / 31)
m = { ["k1354"] = { nested = { deeper = { value = "green", list = { 1, 2, 1354 } } } } }
acc = acc + blend(42.57, 68, math.sin(42.57 / 7)) * (1 + 0 / 31)
q = "fast {math.floor(46.29)} {string.rep('=', 6 + 1)} beta done"
q = "blue {math.floor(86.58)} {string.rep('=', 5 + 1)} delta done"
m = { ["k6962"] = { nested = { deeper = { value = "green", list = { 1, 2, 6962 } } } } }
r = (87.47 > 7) and (acc ~= 0 and "fast" or "beta") or blend(87.47, 4607, 0.25)
acc = acc + blend(57.61, 76, math.sin(57.61 / 7)) * (1 + 4 / 31)
q = "blue {math.floor(14.68)} {string.rep('=', 0 + 1)} alpha done"
for j = 1, 8 do acc = acc + (j * 21.18) % 97 / (j + 1) end
q = "red {math.floor(32.16)} {string.rep('=', 6 + 1)} alpha done"
r = (33.34 > 47) and (acc ~= 0 and "blue" or "alpha") or blend(33.34, 3797, 0.25)
if acc > 83.25 then acc = acc / 24 elseif acc < -83.25 then acc = -acc else acc = acc + 7855 end
for j = 1, 17 do acc = acc + (j * 39.49) % 97 / (j + 1) end
m = { ["k7511"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7511 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 43.43) end
bag = { id = 742, name = "green-delta", pos = { 9.28, 742.5, 18.56 }, ok = true }
if acc > 33.29 then acc = acc / 54 elseif acc < -33.29 then acc = -acc else acc = acc + 409 end
r = (51.69 > 9) and (acc ~= 0 and "red" or "gamma") or blend(51.69, 4109, 0.25)
m = { ["k7852"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7852 } } } } }
m = { ["k1791"] = { nested = { deeper = { value = "red", list = { 1, 2, 1791 } } } } }
if acc > 54.32 then acc = acc / 61 elseif acc < -54.32 then acc = -acc else acc = acc + 6468 end
for j = 1, 27 do acc = acc + (j * 83.60) % 97 / (j + 1) end
r = (68.12 > 8) and (acc ~= 0 and "cold" or "delta") or blend(68.12, 1658, 0.25)
if acc > 68.70 then acc = acc / 64 elseif acc < -68.70 then acc = -acc else acc = acc + 6560 end
acc = acc + blend(82.30, 11, math.sin(82.30 / 7)) * (1 + 4 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 74.77) end
q = "cold {math.floor(79.81)} {string.rep('=', 6 + 1)} delta done"
acc = acc + blend(52.71, 3, math.sin(52.71 / 7)) * (1 + 6 / 31)
m = { ["k8858"] = { nested = { deeper = { value = "fast", list = { 1, 2, 8858 } } } } }
for j = 1, 19 do acc = acc + (j * 92.77) % 97 / (j + 1) end
q = "fast {math.floor(67.33)} {string.rep('=', 5 + 1)} beta done"
bag = { id = 9586, name = "blue-delta", pos = { 17.26, 9586.5, 34.52 }, ok = true }
m = { ["k3137"] = { nested = { deeper = { value = "green", list = { 1, 2, 3137 } } } } }
if acc > 89.67 then acc = acc / 27 elseif acc < -89.67 then acc = -acc else acc = acc + 3853 end
acc = acc + blend(77.76, 20, math.sin(77.76 / 7)) * (1 + 4 / 31)
if acc > 27.09 then acc = acc / 5 elseif acc < -27.09 then acc = -acc else acc = acc + 3831 end
q = "blue {math.floor(64.70)} {string.rep('=', 1 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 46.56) end
m = { ["k5937"] = { nested = { deeper = { value = "red", list = { 1, 2, 5937 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 14.35) end
q = "red {math.floor(41.17)} {string.rep('=', 0 + 1)} gamma done"
for j = 1, 40 do acc = acc + (j * 6.50) % 97 / (j + 1) end
m = { ["k582"] = { nested = { deeper = { value = "red", list = { 1, 2, 582 } } } } }
bag = { id = 1044, name = "green-beta", pos = { 85.04, 1044.5, 170.08 }, ok = true }
r = (99.72 > 38) and (acc ~= 0 and "blue" or "delta") or blend(99.72, 5588, 0.25)
acc = acc + blend(53.53, 72, math.sin(53.53 / 7)) * (1 + 11 / 31)
if acc > 57.06 then acc = acc / 33 elseif acc < -57.06 then acc = -acc else acc = acc + 2791 end
q = "green {math.floor(35.16)} {string.rep('=', 4 + 1)} delta done"
m = { ["k722"] = { nested = { deeper = { value = "green", list = { 1, 2, 722 } } } } }
for j = 1, 24 do acc = acc + (j * 20.79) % 97 / (j + 1) end
bag = { id = 7761, name = "fast-gamma", pos = { 48.32, 7761.5, 96.64 }, ok = false }
if acc > 21.51 then acc = acc / 46 elseif acc < -21.51 then acc = -acc else acc = acc + 4050 end
acc = acc + blend(92.10, 34, math.sin(92.10 / 7)) * (1 + 1 / 31)
q = "fast {math.floor(35.42)} {string.rep('=', 2 + 1)} delta done"
bag = { id = 6205, name = "red-delta", pos = { 13.40, 6205.5, 26.8 }, ok = false }
for j = 1, 15 do acc = acc + (j * 57.59) % 97 / (j + 1) end
for j = 1, 19 do acc = acc + (j * 70.42) % 97 / (j + 1) end
for j = 1, 29 do acc = acc + (j * 33.10) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 16.58) end
q = "red {math.floor(29.98)} {string.rep('=', 2 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 85.68) end
for j = 1, 34 do acc = acc + (j * 0.02) % 97 / (j + 1) end
if acc > 95.04 then acc = acc / 2 elseif acc < -95.04 then acc = -acc else acc = acc + 6587 end
q = "fast {math.floor(55.37)} {string.rep('=', 0 + 1)} beta done"
q = "blue {math.floor(99.23)} {string.rep('=', 5 + 1)} alpha done"
acc = acc + blend(25.09, 12, math.sin(25.09 / 7)) * (1 + 3 / 31)
bag = { id = 2858, name = "blue-delta", pos = { 78.60, 2858.5, 157.2 }, ok = true }
m = { ["k9428"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9428 } } } } }
r = (76.48 > 30) and (acc ~= 0 and "fast" or "delta") or blend(76.48, 9130, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 70.26) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 89.56) end
bag = { id = 1704, name = "cold-alpha", pos = { 97.50, 1704.5, 195 }, ok = true }
for j = 1, 20 do acc = acc + (j * 97.81) % 97 / (j + 1) end
m = { ["k7680"] = { nested = { deeper = { value = "green", list = { 1, 2, 7680 } } } } }
m = { ["k687"] = { nested = { deeper = { value = "warm", list = { 1, 2, 687 } } } } }
acc = acc + blend(63.19, 24, math.sin(63.19 / 7)) * (1 + 10 / 31)
m = { ["k9793"] = { nested = { deeper = { value = "cold", list = { 1, 2, 9793 } } } } }
r = (69.56 > 21) and (acc ~= 0 and "green" or "delta") or blend(69.56, 121, 0.25)
for j = 1, 29 do acc = acc + (j * 43.06) % 97 / (j + 1) end
if acc > 53.93 then acc = acc / 16 elseif acc < -53.93 then acc = -acc else acc = acc + 15 end
q = "blue {math.floor(53.90)} {string.rep('=', 1 + 1)} delta done"
bag = { id = 5141, name = "red-beta", pos = { 29.56, 5141.5, 59.12 }, ok = false }
m = { ["k3590"] = { nested = { deeper = { value = "warm", list = { 1, 2, 3590 } } } } }
if acc > 90.16 then acc = acc / 40 elseif acc < -90.16 then acc = -acc else acc = acc + 8405 end
acc = acc + blend(42.67, 7, math.sin(42.67 / 7)) * (1 + 2 / 31)
for j = 1, 17 do acc = acc + (j * 63.45) % 97 / (j + 1) end
if acc > 29.02 then acc = acc / 81 elseif acc < -29.02 then acc = -acc else acc = acc + 1326 end
r = (7.45 > 24) and (acc ~= 0 and "red" or "alpha") or blend(7.45, 3624, 0.25)
acc = acc + blend(62.57, 3, math.sin(62.57 / 7)) * (1 + 6 / 31)
m = { ["k815"] = { nested = { deeper = { value = "red", list = { 1, 2, 815 } } } } }
if acc > 68.26 then acc = acc / 6 elseif acc < -68.26 then acc = -acc else acc = acc + 9795 end
if acc > 67.42 then acc = acc / 36 elseif acc < -67.42 then acc = -acc else acc = acc + 7333 end
r = (27.42 > 29) and (acc ~= 0 and "blue" or "delta") or blend(27.42, 5579, 0.25)
m = { ["k3770"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3770 } } } } }
bag = { id = 6549, name = "fast-alpha", pos = { 48.58, 6549.5, 97.16 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 71.85) end
r = (15.53 > 42) and (acc ~= 0 and "red" or "delta") or blend(15.53, 5692, 0.25)
m = { ["k9025"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9025 } } } } }
q = "fast {math.floor(98.55)} {string.rep('=', 0 + 1)} beta done"
m = { ["k6497"] = { nested = { deeper = { value = "red", list = { 1, 2, 6497 } } } } }
acc = acc + blend(56.62, 40, math.sin(56.62 / 7)) * (1 + 5 / 31)
q = "green {math.floor(50.90)} {string.rep('=', 5 + 1)} delta done"
for j = 1, 19 do acc = acc + (j * 61.90) % 97 / (j + 1) end
m = { ["k2707"] = { nested = { deeper = { value = "green", list = { 1, 2, 2707 } } } } }
m = { ["k8975"] = { nested = { deeper = { value = "red", list = { 1, 2, 8975 } } } } }
acc = acc + blend(97.41, 22, math.sin(97.41 / 7)) * (1 + 10 / 31)
bag = { id = 9723, name = "green-delta", pos = { 39.31, 9723.5, 78.62 }, ok = false }
bag = { id = 3538, name = "warm-delta", pos = { 13.85, 3538.5, 27.7 }, ok = true }
bag = { id = 3960, name = "fast-delta", pos = { 29.14, 3960.5, 58.28 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 47.90) end
if acc > 50.94 then acc = acc / 51 elseif acc < -50.94 then acc = -acc else acc = acc + 4767 end
bag = { id = 2255, name = "warm-alpha", pos = { 96.31, 2255.5, 192.62 }, ok = false }
r = (8.43 > 34) and (acc ~= 0 and "warm" or "beta") or blend(8.43, 4734, 0.25)
r = (69.38 > 9) and (acc ~= 0 and "green" or "beta") or blend(69.38, 8059, 0.25)
m = { ["k1302"] = { nested = { deeper = { value = "green", list = { 1, 2, 1302 } } } } }
for j = 1, 9 do acc = acc + (j * 62.24) % 97 / (j + 1) end
q = "blue {math.floor(81.63)} {string.rep('=', 4 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 23.31) end
for j = 1, 33 do acc = acc + (j * 13.33) % 97 / (j + 1) end
for j = 1, 26 do acc = acc + (j * 90.00) % 97 / (j + 1) end
if acc > 55.65 then acc = acc / 84 elseif acc < -55.65 then acc = -acc else acc = acc + 4266 end
for j = 1, 7 do acc = acc + (j * 56.76) % 97 / (j + 1) end
for j = 1, 36 do acc = acc + (j * 62.31) % 97 / (j + 1) end
for j = 1, 7 do acc = acc + (j * 43.23) % 97 / (j + 1) end
m = { ["k5677"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5677 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 45.90) end
q = "blue {math.floor(87.63)} {string.rep('=', 2 + 1)} gamma done"
m = { ["k271"] = { nested = { deeper = { value = "green", list = { 1, 2, 271 } } } } }
bag = { id = 9541, name = "warm-delta", pos = { 8.98, 9541.5, 17.96 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 74.83) end
acc = acc + blend(82.61, 90, math.sin(82.61 / 7)) * (1 + 4 / 31)
m = { ["k1250"] = { nested = { deeper = { value = "warm", list = { 1, 2, 1250 } } } } }
acc = acc + blend(34.62, 8, math.sin(34.62 / 7)) * (1 + 9 / 31)
m = { ["k8443"] = { nested = { deeper = { value = "green", list = { 1, 2, 8443 } } } } }
bag = { id = 622, name = "cold-alpha", pos = { 9.72, 622.5, 19.44 }, ok = true }
r = (12.19 > 9) and (acc ~= 0 and "cold" or "beta") or blend(12.19, 7459, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 25.77) end
if acc > 34.99 then acc = acc / 21 elseif acc < -34.99 then acc = -acc else acc = acc + 732 end
if acc > 52.44 then acc = acc / 75 elseif acc < -52.44 then acc = -acc else acc = acc + 2210 end
if acc > 47.23 then acc = acc / 87 elseif acc < -47.23 then acc = -acc else acc = acc + 9876 end
for j = 1, 14 do acc = acc + (j * 40.95) % 97 / (j + 1) end
if acc > 7.86 then acc = acc / 39 elseif acc < -7.86 then acc = -acc else acc = acc + 2441 end
m = { ["k27"] = { nested = { deeper = { value = "fast", list = { 1, 2, 27 } } } } }
q = "blue {math.floor(32.07)} {string.rep('=', 1 + 1)} beta done"
acc = acc + blend(1.83, 50, math.sin(1.83 / 7)) * (1 + 3 / 31)
bag = { id = 485, name = "green-gamma", pos = { 83.18, 485.5, 166.36 }, ok = false }
r = (70.08 > 16) and (acc ~= 0 and "cold" or "beta") or blend(70.08, 766, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 7.74) end
q = "green {math.floor(42.38)} {string.rep('=', 0 + 1)} alpha done"
for j = 1, 9 do acc = acc + (j * 47.20) % 97 / (j + 1) end
bag = { id = 8620, name = "green-gamma", pos = { 84.99, 8620.5, 169.98 }, ok = true }
for j = 1, 40 do acc = acc + (j * 66.07) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 29.36) end
acc = acc + blend(94.68, 75, math.sin(94.68 / 7)) * (1 + 11 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 71.49) end
m = { ["k9933"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9933 } } } } }
bag = { id = 6513, name = "fast-beta", pos = { 18.66, 6513.5, 37.32 }, ok = false }
bag = { id = 2536, name = "green-delta", pos = { 73.14, 2536.5, 146.28 }, ok = true }
for j = 1, 25 do acc = acc + (j * 42.18) % 97 / (j + 1) end
r = (79.97 > 23) and (acc ~= 0 and "blue" or "alpha") or blend(79.97, 3173, 0.25)
for j = 1, 26 do acc = acc + (j * 36.23) % 97 / (j + 1) end
if acc > 25.02 then acc = acc / 29 elseif acc < -25.02 then acc = -acc else acc = acc + 5368 end
m = { ["k8041"] = { nested = { deeper = { value = "red", list = { 1, 2, 8041 } } } } }
r = (59.12 > 29) and (acc ~= 0 and "blue" or "beta") or blend(59.12, 3179, 0.25)
m = { ["k5084"] = { nested = { deeper = { value = "cold", list = { 1, 2, 5084 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 35.24) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 86.84) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 15.33) end
for j = 1, 8 do acc = acc + (j * 6.50) % 97 / (j + 1) end
if acc > 17.27 then acc = acc / 68 elseif acc < -17.27 then acc = -acc else acc = acc + 2559 end
r = (34.08 > 20) and (acc ~= 0 and "fast" or "beta") or blend(34.08, 1570, 0.25)
bag = { id = 1435, name = "cold-delta", pos = { 87.63, 1435.5, 175.26 }, ok = false }
q = "fast {math.floor(92.23)} {string.rep('=', 4 + 1)} alpha done"
for j = 1, 7 do acc = acc + (j * 95.29) % 97 / (j + 1) end
for j = 1, 38 do acc = acc + (j * 82.00) % 97 / (j + 1) end
r = (70.08 > 41) and (acc ~= 0 and "fast" or "alpha") or blend(70.08, 2441, 0.25)
m = { ["k6804"] = { nested = { deeper = { value = "cold", list = { 1, 2, 6804 } } } } }
acc = acc + blend(65.45, 31, math.sin(65.45 / 7)) * (1 + 1 / 31)
r = (79.20 > 7) and (acc ~= 0 and "cold" or "delta") or blend(79.20, 5757, 0.25)
bag = { id = 1169, name = "warm-alpha", pos = { 60.70, 1169.5, 121.4 }, ok = false }
m = { ["k6723"] = { nested = { deeper = { value = "fast", list = { 1, 2, 6723 } } } } }
r = (43.87 > 32) and (acc ~= 0 and "blue" or "gamma") or blend(43.87, 1232, 0.25)
for j = 1, 40 do acc = acc + (j * 82.98) % 97 / (j + 1) end
acc = acc + blend(85.50, 50, math.sin(85.50 / 7)) * (1 + 6 / 31)
if acc > 46.20 then acc = acc / 22 elseif acc < -46.20 then acc = -acc else acc = acc + 5272 end
bag = { id = 2548, name = "warm-gamma", pos = { 34.31, 2548.5, 68.62 }, ok = true }
bag = { id = 6395, name = "warm-alpha", pos = { 13.90, 6395.5, 27.8 }, ok = false }
q = "blue {math.floor(92.98)} {string.rep('=', 1 + 1)} beta done"
for j = 1, 27 do acc = acc + (j * 84.20) % 97 / (j + 1) end
q = "cold {math.floor(73.25)} {string.rep('=', 0 + 1)} gamma done"
q = "fast {math.floor(12.55)} {string.rep('=', 6 + 1)} beta done"
m = { ["k1381"] = { nested = { deeper = { value = "blue", list = { 1, 2, 1381 } } } } }
m = { ["k3"] = { nested = { deeper = { value = "fast", list = { 1, 2, 3 } } } } }
r = (35.27 > 29) and (acc ~= 0 and "green" or "beta") or blend(35.27, 3879, 0.25)
bag = { id = 2027, name = "red-delta", pos = { 99.82, 2027.5, 199.64 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 9.54) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 91.30) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 6.45) end
if acc > 67.39 then acc = acc / 1 elseif acc < -67.39 then acc = -acc else acc = acc + 6764 end
bag = { id = 9232, name = "warm-alpha", pos = { 45.17, 9232.5, 90.34 }, ok = true }
q = "cold {math.floor(42.66)} {string.rep('=', 0 + 1)} gamma done"
bag = { id = 4734, name = "fast-alpha", pos = { 58.67, 4734.5, 117.34 }, ok = true }
bag = { id = 134, name = "red-delta", pos = { 49.97, 134.5, 99.94 }, ok = true }
r = (73.38 > 33) and (acc ~= 0 and "warm" or "beta") or blend(73.38, 7433, 0.25)
m = { ["k3460"] = { nested = { deeper = { value = "green", list = { 1, 2, 3460 } } } } }
bag = { id = 8312, name = "red-gamma", pos = { 82.62, 8312.5, 165.24 }, ok = true }
bag = { id = 9435, name = "cold-delta", pos = { 55.79, 9435.5, 111.58 }, ok = false }
acc = acc + blend(58.25, 18, math.sin(58.25 / 7)) * (1 + 3 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 30.31) end
r = (62.25 > 9) and (acc ~= 0 and "cold" or "gamma") or blend(62.25, 9759, 0.25)
q = "red {math.floor(57.03)} {string.rep('=', 3 + 1)} beta done"
r = (19.23 > 35) and (acc ~= 0 and "warm" or "delta") or blend(19.23, 7435, 0.25)
m = { ["k2549"] = { nested = { deeper = { value = "warm", list = { 1, 2, 2549 } } } } }
m = { ["k5943"] = { nested = { deeper = { value = "warm", list = { 1, 2, 5943 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 66.58) end
acc = acc + blend(87.23, 37, math.sin(87.23 / 7)) * (1 + 5 / 31)
for j = 1, 38 do acc = acc + (j * 51.50) % 97 / (j + 1) end
if acc > 15.85 then acc = acc / 43 elseif acc < -15.85 then acc = -acc else acc = acc + 5293 end
r = (52.91 > 37) and (acc ~= 0 and "blue" or "gamma") or blend(52.91, 9937, 0.25)
if acc > 93.62 then acc = acc / 30 elseif acc < -93.62 then acc = -acc else acc = acc + 5992 end
if acc > 53.39 then acc = acc / 72 elseif acc < -53.39 then acc = -acc else acc = acc + 71 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 6.92) end
if acc > 69.34 then acc = acc / 59 elseif acc < -69.34 then acc = -acc else acc = acc + 1037 end
if acc > 12.42 then acc = acc / 84 elseif acc < -12.42 then acc = -acc else acc = acc + 4444 end
bag = { id = 5631, name = "green-delta", pos = { 82.80, 5631.5, 165.6 }, ok = false }
r = (2.40 > 26) and (acc ~= 0 and "fast" or "alpha") or blend(2.40, 626, 0.25)
m = { ["k5777"] = { nested = { deeper = { value = "warm", list = { 1, 2, 5777 } } } } }
r = (1.11 > 0) and (acc ~= 0 and "green" or "delta") or blend(1.11, 2950, 0.25)
r = (18.05 > 18) and (acc ~= 0 and "red" or "delta") or blend(18.05, 368, 0.25)
m = { ["k848"] = { nested = { deeper = { value = "warm", list = { 1, 2, 848 } } } } }
q = "green {math.floor(45.83)} {string.rep('=', 1 + 1)} gamma done"
q = "cold {math.floor(10.17)} {string.rep('=', 0 + 1)} delta done"
q = "fast {math.floor(46.01)} {string.rep('=', 3 + 1)} beta done"
bag = { id = 2403, name = "cold-alpha", pos = { 75.17, 2403.5, 150.34 }, ok = false }
q = "green {math.floor(11.15)} {string.rep('=', 3 + 1)} beta done"
q = "blue {math.floor(56.88)} {string.rep('=', 2 + 1)} beta done"
if acc > 84.39 then acc = acc / 55 elseif acc < -84.39 then acc = -acc else acc = acc + 5483 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 68.83) end
for j = 1, 20 do acc = acc + (j * 52.69) % 97 / (j + 1) end
if acc > 83.55 then acc = acc / 48 elseif acc < -83.55 then acc = -acc else acc = acc + 3340 end
if acc > 59.94 then acc = acc / 58 elseif acc < -59.94 then acc = -acc else acc = acc + 2371 end
for j = 1, 21 do acc = acc + (j * 71.20) % 97 / (j + 1) end
acc = acc + blend(7.18, 69, math.sin(7.18 / 7)) * (1 + 1 / 31)
r = (45.51 > 18) and (acc ~= 0 and "red" or "beta") or blend(45.51, 3968, 0.25)
r = (56.71 > 0) and (acc ~= 0 and "green" or "delta") or blend(56.71, 2750, 0.25)
if acc > 73.36 then acc = acc / 61 elseif acc < -73.36 then acc = -acc else acc = acc + 2997 end
q = "blue {math.floor(15.41)} {string.rep('=', 6 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 99.26) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 73.40) end
if acc > 81.07 then acc = acc / 15 elseif acc < -81.07 then acc = -acc else acc = acc + 6867 end
m = { ["k5378"] = { nested = { deeper = { value = "green", list = { 1, 2, 5378 } } } } }
r = (85.98 > 45) and (acc ~= 0 and "cold" or "delta") or blend(85.98, 7645, 0.25)
q = "green {math.floor(53.80)} {string.rep('=', 3 + 1)} delta done"
r = (61.44 > 32) and (acc ~= 0 and "cold" or "gamma") or blend(61.44, 1132, 0.25)
acc = acc + blend(53.49, 44, math.sin(53.49 / 7)) * (1 + 2 / 31)
for j = 1, 19 do acc = acc + (j * 30.79) % 97 / (j + 1) end
acc = acc + blend(79.49, 22, math.sin(79.49 / 7)) * (1 + 12 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 74.84) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 96.72) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 3.02) end
for j = 1, 24 do acc = acc + (j * 38.80) % 97 / (j + 1) end
q = "warm {math.floor(99.89)} {string.rep('=', 5 + 1)} alpha done"
if acc > 36.90 then acc = acc / 43 elseif acc < -36.90 then acc = -acc else acc = acc + 8586 end
q = "blue {math.floor(7.85)} {string.rep('=', 5 + 1)} beta done"
r = (11.08 > 44) and (acc ~= 0 and "cold" or "gamma") or blend(11.08, 5994, 0.25)
q = "green {math.floor(84.31)} {string.rep('=', 1 + 1)} delta done"
bag = { id = 8727, name = "blue-gamma", pos = { 73.99, 8727.5, 147.98 }, ok = false }
bag = { id = 1033, name = "red-delta", pos = { 4.61, 1033.5, 9.22 }, ok = false }
m = { ["k7255"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7255 } } } } }
if acc > 79.48 then acc = acc / 67 elseif acc < -79.48 then acc = -acc else acc = acc + 4071 end
q = "green {math.floor(73.78)} {string.rep('=', 4 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 72.01) end
q = "blue {math.floor(17.55)} {string.rep('=', 5 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 41.48) end
for j = 1, 8 do acc = acc + (j * 28.59) % 97 / (j + 1) end
q = "cold {math.floor(65.76)} {string.rep('=', 3 + 1)} alpha done"
for j = 1, 20 do acc = acc + (j * 95.01) % 97 / (j + 1) end
for j = 1, 21 do acc = acc + (j * 6.50) % 97 / (j + 1) end
for j = 1, 15 do acc = acc + (j * 29.38) % 97 / (j + 1) end
for j = 1, 18 do acc = acc + (j * 38.63) % 97 / (j + 1) end
bag = { id = 8555, name = "green-alpha", pos = { 22.85, 8555.5, 45.7 }, ok = false }
if acc > 73.28 then acc = acc / 47 elseif acc < -73.28 then acc = -acc else acc = acc + 7967 end
acc = acc + blend(0.88, 16, math.sin(0.88 / 7)) * (1 + 11 / 31)
q = "cold {math.floor(18.58)} {string.rep('=', 3 + 1)} beta done"
bag = { id = 7157, name = "warm-beta", pos = { 20.78, 7157.5, 41.56 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 50.62) end
for j = 1, 34 do acc = acc + (j * 76.24) % 97 / (j + 1) end
acc = acc + blend(65.00, 77, math.sin(65.00 / 7)) * (1 + 1 / 31)
acc = acc + blend(64.72, 3, math.sin(64.72 / 7)) * (1 + 9 / 31)
r = (52.53 > 41) and (acc ~= 0 and "fast" or "beta") or blend(52.53, 5041, 0.25)
r = (12.84 > 46) and (acc ~= 0 and "red" or "delta") or blend(12.84, 646, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 48.55) end
bag = { id = 2597, name = "blue-beta", pos = { 72.13, 2597.5, 144.26 }, ok = false }
if acc > 49.01 then acc = acc / 3 elseif acc < -49.01 then acc = -acc else acc = acc + 8991 end
acc = acc + blend(75.69, 28, math.sin(75.69 / 7)) * (1 + 5 / 31)
m = { ["k8151"] = { nested = { deeper = { value = "blue", list = { 1, 2, 8151 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 15.22) end
r = (5.20 > 20) and (acc ~= 0 and "fast" or "delta") or blend(5.20, 4970, 0.25)
for j = 1, 29 do acc = acc + (j * 40.16) % 97 / (j + 1) end
m = { ["k8481"] = { nested = { deeper = { value = "red", list = { 1, 2, 8481 } } } } }
if acc > 26.02 then acc = acc / 34 elseif acc < -26.02 then acc = -acc else acc = acc + 9200 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 66.68) end
bag = { id = 3984, name = "red-beta", pos = { 90.46, 3984.5, 180.92 }, ok = true }
m = { ["k7615"] = { nested = { deeper = { value = "fast", list = { 1, 2, 7615 } } } } }
m = { ["k9374"] = { nested = { deeper = { value = "warm", list = { 1, 2, 9374 } } } } }
r = (73.84 > 42) and (acc ~= 0 and "green" or "beta") or blend(73.84, 3942, 0.25)
bag = { id = 3230, name = "fast-alpha", pos = { 15.51, 3230.5, 31.02 }, ok = true }
q = "warm {math.floor(70.27)} {string.rep('=', 5 + 1)} beta done"
if acc > 89.34 then acc = acc / 6 elseif acc < -89.34 then acc = -acc else acc = acc + 5167 end
if acc > 13.05 then acc = acc / 74 elseif acc < -13.05 then acc = -acc else acc = acc + 1052 end
q = "blue {math.floor(28.70)} {string.rep('=', 2 + 1)} delta done"
r = (11.45 > 20) and (acc ~= 0 and "blue" or "alpha") or blend(11.45, 7920, 0.25)
q = "green {math.floor(93.61)} {string.rep('=', 2 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 37.38) end
acc = acc + blend(63.18, 31, math.sin(63.18 / 7)) * (1 + 3 / 31)
q = "blue {math.floor(53.96)} {string.rep('=', 1 + 1)} delta done"
r = (97.52 > 43) and (acc ~= 0 and "warm" or "beta") or blend(97.52, 5493, 0.25)
r = (14.87 > 17) and (acc ~= 0 and "red" or "beta") or blend(14.87, 1767, 0.25)
r = (19.15 > 7) and (acc ~= 0 and "blue" or "beta") or blend(19.15, 2257, 0.25)
q = "red {math.floor(24.68)} {string.rep('=', 6 + 1)} delta done"
q = "fast {math.floor(59.21)} {string.rep('=', 0 + 1)} alpha done"
r = (87.01 > 46) and (acc ~= 0 and "fast" or "gamma") or blend(87.01, 8396, 0.25)
bag = { id = 5565, name = "red-beta", pos = { 64.84, 5565.5, 129.68 }, ok = false }
bag = { id = 2441, name = "blue-gamma", pos = { 35.03, 2441.5, 70.06 }, ok = false }
if acc > 94.42 then acc = acc / 34 elseif acc < -94.42 then acc = -acc else acc = acc + 5818 end
acc = acc + blend(45.26, 6, math.sin(45.26 / 7)) * (1 + 11 / 31)
for j = 1, 3 do acc = acc + (j * 17.57) % 97 / (j + 1) end
acc = acc + blend(36.68, 35, math.sin(36.68 / 7)) * (1 + 8 / 31)
q = "cold {math.floor(18.61)} {string.rep('=', 5 + 1)} delta done"
m = { ["k321"] = { nested = { deeper = { value = "blue", list = { 1, 2, 321 } } } } }
q = "fast {math.floor(56.63)} {string.rep('=', 4 + 1)} beta done"
acc = acc + blend(28.91, 62, math.sin(28.91 / 7)) * (1 + 10 / 31)
m = { ["k1903"] = { nested = { deeper = { value = "red", list = { 1, 2, 1903 } } } } }
m = { ["k8555"] = { nested = { deeper = { value = "cold", list = { 1, 2, 8555 } } } } }
bag = { id = 6317, name = "blue-delta", pos = { 44.44, 6317.5, 88.88 }, ok = false }
q = "cold {math.floor(37.91)} {string.rep('=', 6 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 66.77) end
bag = { id = 1082, name = "blue-beta", pos = { 20.59, 1082.5, 41.18 }, ok = true }
m = { ["k9212"] = { nested = { deeper = { value = "green", list = { 1, 2, 9212 } } } } }
for j = 1, 28 do acc = acc + (j * 13.42) % 97 / (j + 1) end
bag = { id = 4720, name = "blue-gamma", pos = { 20.00, 4720.5, 40 }, ok = true }
r = (84.58 > 40) and (acc ~= 0 and "red" or "gamma") or blend(84.58, 9340, 0.25)
r = (53.93 > 26) and (acc ~= 0 and "blue" or "delta") or blend(53.93, 6126, 0.25)
for j = 1, 38 do acc = acc + (j * 12.92) % 97 / (j + 1) end
m = { ["k6541"] = { nested = { deeper = { value = "fast", list = { 1, 2, 6541 } } } } }
acc = acc + blend(19.80, 25, math.sin(19.80 / 7)) * (1 + 10 / 31)
acc = acc + blend(28.78, 48, math.sin(28.78 / 7)) * (1 + 4 / 31)
q = "warm {math.floor(68.76)} {string.rep('=', 1 + 1)} alpha done"
for j = 1, 25 do acc = acc + (j * 35.00) % 97 / (j + 1) end
if acc > 21.14 then acc = acc / 46 elseif acc < -21.14 then acc = -acc else acc = acc + 9212 end
if acc > 34.08 then acc = acc / 47 elseif acc < -34.08 then acc = -acc else acc = acc + 6365 end
if acc > 58.51 then acc = acc / 88 elseif acc < -58.51 then acc = -acc else acc = acc + 3647 end
bag = { id = 9939, name = "cold-alpha", pos = { 16.47, 9939.5, 32.94 }, ok = false }
acc = acc + blend(81.36, 2, math.sin(81.36 / 7)) * (1 + 8 / 31)
r = (64.93 > 2) and (acc ~= 0 and "green" or "delta") or blend(64.93, 7852, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 23.83) end
if acc > 39.85 then acc = acc / 48 elseif acc < -39.85 then acc = -acc else acc = acc + 492 end
bag = { id = 992, name = "red-alpha", pos = { 93.48, 992.5, 186.96 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 38.23) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 8.18) end
acc = acc + blend(64.23, 63, math.sin(64.23 / 7)) * (1 + 4 / 31)
r = (66.43 > 19) and (acc ~= 0 and "green" or "alpha") or blend(66.43, 2069, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 15.72) end
m = { ["k6399"] = { nested = { deeper = { value = "blue", list = { 1, 2, 6399 } } } } }
acc = acc + blend(62.55, 85, math.sin(62.55 / 7)) * (1 + 2 / 31)
q = "warm {math.floor(11.96)} {string.rep('=', 0 + 1)} alpha done"
if acc > 47.74 then acc = acc / 35 elseif acc < -47.74 then acc = -acc else acc = acc + 7065 end
for j = 1, 38 do acc = acc + (j * 62.73) % 97 / (j + 1) end
m = { ["k8253"] = { nested = { deeper = { value = "red", list = { 1, 2, 8253 } } } } }
m = { ["k5910"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5910 } } } } }
q = "cold {math.floor(39.46)} {string.rep('=', 4 + 1)} beta done"
acc = acc + blend(13.61, 80, math.sin(13.61 / 7)) * (1 + 1 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 25.08) end
m = { ["k4548"] = { nested = { deeper = { value = "blue", list = { 1, 2, 4548 } } } } }
acc = acc + blend(65.92, 67, math.sin(65.92 / 7)) * (1 + 6 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 27.26) end
m = { ["k539"] = { nested = { deeper = { value = "warm", list = { 1, 2, 539 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 68.57) end
bag = { id = 1643, name = "green-gamma", pos = { 87.63, 1643.5, 175.26 }, ok = false }
r = (53.66 > 6) and (acc ~= 0 and "warm" or "beta") or blend(53.66, 4906, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 92.01) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 62.94) end
m = { ["k5668"] = { nested = { deeper = { value = "green", list = { 1, 2, 5668 } } } } }
m = { ["k1616"] = { nested = { deeper = { value = "red", list = { 1, 2, 1616 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 22.86) end
for j = 1, 32 do acc = acc + (j * 90.24) % 97 / (j + 1) end
for j = 1, 15 do acc = acc + (j * 20.66) % 97 / (j + 1) end
acc = acc + blend(40.41, 80, math.sin(40.41 / 7)) * (1 + 7 / 31)
bag = { id = 8097, name = "warm-alpha", pos = { 0.43, 8097.5, 0.86 }, ok = false }
for j = 1, 24 do acc = acc + (j * 75.08) % 97 / (j + 1) end
for j = 1, 20 do acc = acc + (j * 33.70) % 97 / (j + 1) end
if acc > 93.07 then acc = acc / 12 elseif acc < -93.07 then acc = -acc else acc = acc + 1702 end
r = (77.87 > 4) and (acc ~= 0 and "blue" or "beta") or blend(77.87, 6104, 0.25)
for j = 1, 39 do acc = acc + (j * 52.79) % 97 / (j + 1) end
acc = acc + blend(55.02, 22, math.sin(55.02 / 7)) * (1 + 3 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 59.27) end
for j = 1, 7 do acc = acc + (j * 71.02) % 97 / (j + 1) end
bag = { id = 3104, name = "green-gamma", pos = { 7.10, 3104.5, 14.2 }, ok = true }
if acc > 82.71 then acc = acc / 52 elseif acc < -82.71 then acc = -acc else acc = acc + 140 end
m = { ["k4458"] = { nested = { deeper = { value = "fast", list = { 1, 2, 4458 } } } } }
acc = acc + blend(6.20, 79, math.sin(6.20 / 7)) * (1 + 4 / 31)
r = (24.27 > 16) and (acc ~= 0 and "fast" or "gamma") or blend(24.27, 4866, 0.25)
m = { ["k2936"] = { nested = { deeper = { value = "fast", list = { 1, 2, 2936 } } } } }
for j = 1, 31 do acc = acc + (j * 53.34) % 97 / (j + 1) end
m = { ["k7528"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7528 } } } } }
acc = acc + blend(71.64, 26, math.sin(71.64 / 7)) * (1 + 5 / 31)
q = "fast {math.floor(45.82)} {string.rep('=', 4 + 1)} gamma done"
r = (27.79 > 34) and (acc ~= 0 and "red" or "beta") or blend(27.79, 3484, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 13.55) end
r = (33.25 > 44) and (acc ~= 0 and "green" or "gamma") or blend(33.25, 2594, 0.25)
m = { ["k1142"] = { nested = { deeper = { value = "green", list = { 1, 2, 1142 } } } } }
q = "red {math.floor(13.41)} {string.rep('=', 3 + 1)} gamma done"
acc = acc + blend(35.19, 6, math.sin(35.19 / 7)) * (1 + 10 / 31)
q = "cold {math.floor(60.36)} {string.rep('=', 1 + 1)} gamma done"
m = { ["k7280"] = { nested = { deeper = { value = "red", list = { 1, 2, 7280 } } } } }
bag = { id = 8160, name = "red-delta", pos = { 38.76, 8160.5, 77.52 }, ok = true }
for j = 1, 11 do acc = acc + (j * 97.52) % 97 / (j + 1) end
bag = { id = 4825, name = "fast-alpha", pos = { 19.85, 4825.5, 39.7 }, ok = false }
if acc > 34.86 then acc = acc / 30 elseif acc < -34.86 then acc = -acc else acc = acc + 5102 end
if acc > 74.29 then acc = acc / 24 elseif acc < -74.29 then acc = -acc else acc = acc + 9012 end
for j = 1, 33 do acc = acc + (j * 25.98) % 97 / (j + 1) end
for j = 1, 16 do acc = acc + (j * 48.02) % 97 / (j + 1) end
for j = 1, 29 do acc = acc + (j * 20.28) % 97 / (j + 1) end
r = (34.19 > 41) and (acc ~= 0 and "red" or "beta") or blend(34.19, 391, 0.25)
bag = { id = 1385, name = "warm-delta", pos = { 68.13, 1385.5, 136.26 }, ok = false }
for j = 1, 5 do acc = acc + (j * 42.94) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 57.47) end
r = (17.31 > 10) and (acc ~= 0 and "warm" or "beta") or blend(17.31, 7260, 0.25)
if acc > 64.38 then acc = acc / 44 elseif acc < -64.38 then acc = -acc else acc = acc + 2713 end
for j = 1, 10 do acc = acc + (j * 52.85) % 97 / (j + 1) end
q = "red {math.floor(1.98)} {string.rep('=', 4 + 1)} delta done"
for j = 1, 28 do acc = acc + (j * 7.56) % 97 / (j + 1) end
if acc > 49.62 then acc = acc / 81 elseif acc < -49.62 then acc = -acc else acc = acc + 1949 end
for j = 1, 17 do acc = acc + (j * 43.63) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 39.28) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 79.14) end
bag = { id = 9057, name = "fast-gamma", pos = { 99.96, 9057.5, 199.92 }, ok = false }
acc = acc + blend(53.12, 91, math.sin(53.12 / 7)) * (1 + 0 / 31)
m = { ["k9437"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9437 } } } } }
q = "warm {math.floor(76.86)} {string.rep('=', 6 + 1)} alpha done"
if acc > 63.42 then acc = acc / 82 elseif acc < -63.42 then acc = -acc else acc = acc + 2217 end
bag = { id = 2828, name = "cold-alpha", pos = { 64.97, 2828.5, 129.94 }, ok = true }
for j = 1, 30 do acc = acc + (j * 71.45) % 97 / (j + 1) end
q = "green {math.floor(66.82)} {string.rep('=', 3 + 1)} gamma done"
q = "green {math.floor(38.94)} {string.rep('=', 6 + 1)} alpha done"
for j = 1, 6 do acc = acc + (j * 60.05) % 97 / (j + 1) end
for j = 1, 5 do acc = acc + (j * 84.38) % 97 / (j + 1) end
q = "blue {math.floor(77.86)} {string.rep('=', 4 + 1)} alpha done"
m = { ["k4969"] = { nested = { deeper = { value = "blue", list = { 1, 2, 4969 } } } } }
q = "red {math.floor(27.98)} {string.rep('=', 4 + 1)} gamma done"
if acc > 5.69 then acc = acc / 43 elseif acc < -5.69 then acc = -acc else acc = acc + 5115 end
bag = { id = 4146, name = "green-alpha", pos = { 79.04, 4146.5, 158.08 }, ok = true }
r = (24.12 > 10) and (acc ~= 0 and "cold" or "gamma") or blend(24.12, 3810, 0.25)
acc = acc + blend(46.86, 8, math.sin(46.86 / 7)) * (1 + 5 / 31)
if acc > 34.62 then acc = acc / 62 elseif acc < -34.62 then acc = -acc else acc = acc + 1752 end
q = "red {math.floor(63.45)} {string.rep('=', 0 + 1)} beta done"
m = { ["k6720"] = { nested = { deeper = { value = "green", list = { 1, 2, 6720 } } } } }
for j = 1, 14 do acc = acc + (j * 63.79) % 97 / (j + 1) end
bag = { id = 8697, name = "red-delta", pos = { 51.29, 8697.5, 102.58 }, ok = false }
acc = acc + blend(33.69, 77, math.sin(33.69 / 7)) * (1 + 9 / 31)
if acc > 95.96 then acc = acc / 2 elseif acc < -95.96 then acc = -acc else acc = acc + 7388 end
q = "fast {math.floor(34.14)} {string.rep('=', 6 + 1)} delta done"
m = { ["k5946"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5946 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 9.32) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 89.07) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 2.23) end
q = "blue {math.floor(3.46)} {string.rep('=', 4 + 1)} delta done"
q = "blue {math.floor(1.63)} {string.rep('=', 4 + 1)} alpha done"
r = (35.01 > 29) and (acc ~= 0 and "warm" or "delta") or blend(35.01, 4779, 0.25)
r = (31.46 > 3) and (acc ~= 0 and "red" or "gamma") or blend(31.46, 4053, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 35.60) end
acc = acc + blend(25.91, 81, math.sin(25.91 / 7)) * (1 + 4 / 31)
if acc > 45.60 then acc = acc / 18 elseif acc < -45.60 then acc = -acc else acc = acc + 6336 end
m = { ["k9409"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9409 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 2.90) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 46.76) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 85.52) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 35.20) end
bag = { id = 2928, name = "fast-alpha", pos = { 12.33, 2928.5, 24.66 }, ok = true }
if acc > 76.53 then acc = acc / 5 elseif acc < -76.53 then acc = -acc else acc = acc + 7836 end
if acc > 92.33 then acc = acc / 13 elseif acc < -92.33 then acc = -acc else acc = acc + 7132 end
m = { ["k3181"] = { nested = { deeper = { value = "fast", list = { 1, 2, 3181 } } } } }
for j = 1, 40 do acc = acc + (j * 26.63) % 97 / (j + 1) end
m = { ["k1859"] = { nested = { deeper = { value = "blue", list = { 1, 2, 1859 } } } } }
if acc > 60.30 then acc = acc / 38 elseif acc < -60.30 then acc = -acc else acc = acc + 660 end
bag = { id = 3164, name = "warm-gamma", pos = { 82.67, 3164.5, 165.34 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 33.65) end
for j = 1, 6 do acc = acc + (j * 1.79) % 97 / (j + 1) end
acc = acc + blend(27.02, 1, math.sin(27.02 / 7)) * (1 + 5 / 31)
m = { ["k9583"] = { nested = { deeper = { value = "warm", list = { 1, 2, 9583 } } } } }
q = "fast {math.floor(10.04)} {string.rep('=', 0 + 1)} gamma done"
bag = { id = 1699, name = "fast-delta", pos = { 73.30, 1699.5, 146.6 }, ok = false }
acc = acc + blend(26.92, 23, math.sin(26.92 / 7)) * (1 + 1 / 31)
acc = acc + blend(62.46, 42, math.sin(62.46 / 7)) * (1 + 6 / 31)
if acc > 55.62 then acc = acc / 66 elseif acc < -55.62 then acc = -acc else acc = acc + 3536 end
acc = acc + blend(50.27, 24, math.sin(50.27 / 7)) * (1 + 12 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 0.72) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 10.19) end
bag = { id = 3299, name = "cold-gamma", pos = { 0.22, 3299.5, 0.44 }, ok = false }
bag = { id = 2207, name = "blue-gamma", pos = { 95.47, 2207.5, 190.94 }, ok = false }
r = (75.01 > 21) and (acc ~= 0 and "red" or "beta") or blend(75.01, 6721, 0.25)
r = (81.03 > 13) and (acc ~= 0 and "blue" or "alpha") or blend(81.03, 6963, 0.25)
for j = 1, 31 do acc = acc + (j * 9.37) % 97 / (j + 1) end
if acc > 15.43 then acc = acc / 74 elseif acc < -15.43 then acc = -acc else acc = acc + 6481 end
for j = 1, 14 do acc = acc + (j * 50.73) % 97 / (j + 1) end
for j = 1, 14 do acc = acc + (j * 41.92) % 97 / (j + 1) end
q = "cold {math.floor(40.53)} {string.rep('=', 3 + 1)} delta done"
bag = { id = 7386, name = "fast-beta", pos = { 0.18, 7386.5, 0.36 }, ok = true }
bag = { id = 6531, name = "warm-gamma", pos = { 64.96, 6531.5, 129.92 }, ok = false }
bag = { id = 1616, name = "warm-delta", pos = { 85.58, 1616.5, 171.16 }, ok = true }
if acc > 26.33 then acc = acc / 38 elseif acc < -26.33 then acc = -acc else acc = acc + 8225 end
for j = 1, 35 do acc = acc + (j * 78.77) % 97 / (j + 1) end
m = { ["k3562"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3562 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 68.68) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 86.45) end
for j = 1, 19 do acc = acc + (j * 35.78) % 97 / (j + 1) end
m = { ["k7927"] = { nested = { deeper = { value = "fast", list = { 1, 2, 7927 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 43.12) end
r = (94.64 > 10) and (acc ~= 0 and "fast" or "gamma") or blend(94.64, 2410, 0.25)
q = "green {math.floor(59.24)} {string.rep('=', 6 + 1)} beta done"
r = (26.93 > 13) and (acc ~= 0 and "cold" or "delta") or blend(26.93, 8713, 0.25)
r = (13.08 > 4) and (acc ~= 0 and "cold" or "alpha") or blend(13.08, 3504, 0.25)
acc = acc + blend(52.56, 23, math.sin(52.56 / 7)) * (1 + 3 / 31)
r = (15.14 > 14) and (acc ~= 0 and "green" or "alpha") or blend(15.14, 1064, 0.25)
acc = acc + blend(26.20, 84, math.sin(26.20 / 7)) * (1 + 0 / 31)
q = "blue {math.floor(75.86)} {string.rep('=', 5 + 1)} alpha done"
r = (46.84 > 13) and (acc ~= 0 and "warm" or "beta") or blend(46.84, 3163, 0.25)
if acc > 68.94 then acc = acc / 58 elseif acc < -68.94 then acc = -acc else acc = acc + 3439 end
for j = 1, 25 do acc = acc + (j * 93.59) % 97 / (j + 1) end
acc = acc + blend(29.18, 85, math.sin(29.18 / 7)) * (1 + 5 / 31)
r = (90.99 > 24) and (acc ~= 0 and "green" or "delta") or blend(90.99, 5474, 0.25)
bag = { id = 1385, name = "cold-gamma", pos = { 99.40, 1385.5, 198.8 }, ok = false }
q = "warm {math.floor(47.10)} {string.rep('=', 3 + 1)} gamma done"
if acc > 52.79 then acc = acc / 16 elseif acc < -52.79 then acc = -acc else acc = acc + 4465 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 78.85) end
m = { ["k6147"] = { nested = { deeper = { value = "warm", list = { 1, 2, 6147 } } } } }
bag = { id = 9650, name = "green-gamma", pos = { 52.25, 9650.5, 104.5 }, ok = true }
q = "red {math.floor(97.23)} {string.rep('=', 6 + 1)} alpha done"
for j = 1, 8 do acc = acc + (j * 0.88) % 97 / (j + 1) end
for j = 1, 40 do acc = acc + (j * 88.06) % 97 / (j + 1) end
m = { ["k6477"] = { nested = { deeper = { value = "fast", list = { 1, 2, 6477 } } } } }
m = { ["k3926"] = { nested = { deeper = { value = "cold", list = { 1, 2, 3926 } } } } }
bag = { id = 8356, name = "cold-alpha", pos = { 0.13, 8356.5, 0.26 }, ok = true }
if acc > 67.64 then acc = acc / 82 elseif acc < -67.64 then acc = -acc else acc = acc + 2217 end
if acc > 10.38 then acc = acc / 58 elseif acc < -10.38 then acc = -acc else acc = acc + 1659 end
q = "warm {math.floor(74.63)} {string.rep('=', 1 + 1)} beta done"
q = "blue {math.floor(69.24)} {string.rep('=', 4 + 1)} gamma done"
r = (15.23 > 5) and (acc ~= 0 and "red" or "beta") or blend(15.23, 655, 0.25)
r = (97.61 > 26) and (acc ~= 0 and "warm" or "alpha") or blend(97.61, 2876, 0.25)
if acc > 92.43 then acc = acc / 54 elseif acc < -92.43 then acc = -acc else acc = acc + 7885 end
for j = 1, 13 do acc = acc + (j * 15.15) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 34.89) end
if acc > 66.01 then acc = acc / 18 elseif acc < -66.01 then acc = -acc else acc = acc + 5090 end
acc = acc + blend(3.16, 3, math.sin(3.16 / 7)) * (1 + 4 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 43.05) end
acc = acc + blend(52.21, 37, math.sin(52.21 / 7)) * (1 + 3 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 70.57) end
q = "fast {math.floor(38.96)} {string.rep('=', 4 + 1)} alpha done"
acc = acc + blend(36.09, 49, math.sin(36.09 / 7)) * (1 + 0 / 31)
acc = acc + blend(42.92, 11, math.sin(42.92 / 7)) * (1 + 2 / 31)
m = { ["k8041"] = { nested = { deeper = { value = "warm", list = { 1, 2, 8041 } } } } }
bag = { id = 9009, name = "blue-gamma", pos = { 72.46, 9009.5, 144.92 }, ok = false }
if acc > 32.53 then acc = acc / 58 elseif acc < -32.53 then acc = -acc else acc = acc + 3172 end
for j = 1, 24 do acc = acc + (j * 16.21) % 97 / (j + 1) end
bag = { id = 3159, name = "blue-gamma", pos = { 36.27, 3159.5, 72.54 }, ok = false }
r = (74.23 > 34) and (acc ~= 0 and "green" or "beta") or blend(74.23, 6684, 0.25)
bag = { id = 1903, name = "cold-beta", pos = { 26.75, 1903.5, 53.5 }, ok = false }
bag = { id = 1289, name = "fast-beta", pos = { 23.13, 1289.5, 46.26 }, ok = false }
acc = acc + blend(6.43, 57, math.sin(6.43 / 7)) * (1 + 12 / 31)
for j = 1, 15 do acc = acc + (j * 39.75) % 97 / (j + 1) end
if acc > 45.75 then acc = acc / 9 elseif acc < -45.75 then acc = -acc else acc = acc + 3924 end
r = (85.47 > 45) and (acc ~= 0 and "red" or "alpha") or blend(85.47, 5295, 0.25)
m = { ["k5277"] = { nested = { deeper = { value = "red", list = { 1, 2, 5277 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 43.93) end
if acc > 77.77 then acc = acc / 82 elseif acc < -77.77 then acc = -acc else acc = acc + 7468 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 65.28) end
bag = { id = 1476, name = "cold-gamma", pos = { 62.14, 1476.5, 124.28 }, ok = true }
bag = { id = 2432, name = "green-delta", pos = { 5.82, 2432.5, 11.64 }, ok = true }
q = "warm {math.floor(54.42)} {string.rep('=', 6 + 1)} gamma done"
acc = acc + blend(20.49, 39, math.sin(20.49 / 7)) * (1 + 12 / 31)
r = (18.68 > 21) and (acc ~= 0 and "blue" or "beta") or blend(18.68, 7921, 0.25)
for j = 1, 16 do acc = acc + (j * 71.39) % 97 / (j + 1) end
acc = acc + blend(13.08, 38, math.sin(13.08 / 7)) * (1 + 3 / 31)
bag = { id = 3064, name = "cold-delta", pos = { 16.88, 3064.5, 33.76 }, ok = true }
q = "red {math.floor(66.93)} {string.rep('=', 1 + 1)} gamma done"
acc = acc + blend(47.24, 56, math.sin(47.24 / 7)) * (1 + 5 / 31)
acc = acc + blend(90.98, 80, math.sin(90.98 / 7)) * (1 + 10 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 91.61) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 44.63) end
if acc > 42.91 then acc = acc / 88 elseif acc < -42.91 then acc = -acc else acc = acc + 8008 end
m = { ["k3713"] = { nested = { deeper = { value = "green", list = { 1, 2, 3713 } } } } }
m = { ["k3565"] = { nested = { deeper = { value = "green", list = { 1, 2, 3565 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 22.98) end
bag = { id = 1644, name = "green-gamma", pos = { 4.83, 1644.5, 9.66 }, ok = true }
q = "green {math.floor(76.90)} {string.rep('=', 5 + 1)} beta done"
m = { ["k4131"] = { nested = { deeper = { value = "blue", list = { 1, 2, 4131 } } } } }
m = { ["k7490"] = { nested = { deeper = { value = "fast", list = { 1, 2, 7490 } } } } }
m = { ["k3282"] = { nested = { deeper = { value = "cold", list = { 1, 2, 3282 } } } } }
m = { ["k6074"] = { nested = { deeper = { value = "green", list = { 1, 2, 6074 } } } } }
acc = acc + blend(67.62, 1, math.sin(67.62 / 7)) * (1 + 1 / 31)
bag = { id = 7077, name = "cold-beta", pos = { 52.34, 7077.5, 104.68 }, ok = false }
if acc > 28.44 then acc = acc / 34 elseif acc < -28.44 then acc = -acc else acc = acc + 7954 end
r = (82.30 > 9) and (acc ~= 0 and "fast" or "delta") or blend(82.30, 7959, 0.25)
r = (81.53 > 12) and (acc ~= 0 and "blue" or "gamma") or blend(81.53, 512, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 18.23) end
bag = { id = 2277, name = "red-delta", pos = { 84.25, 2277.5, 168.5 }, ok = false }
for j = 1, 35 do acc = acc + (j * 73.62) % 97 / (j + 1) end
bag = { id = 3342, name = "green-beta", pos = { 32.88, 3342.5, 65.76 }, ok = true }
q = "green {math.floor(86.15)} {string.rep('=', 6 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 77.84) end
if acc > 16.70 then acc = acc / 56 elseif acc < -16.70 then acc = -acc else acc = acc + 1657 end
q = "red {math.floor(25.45)} {string.rep('=', 0 + 1)} delta done"
q = "cold {math.floor(17.69)} {string.rep('=', 3 + 1)} delta done"
r = (97.34 > 16) and (acc ~= 0 and "blue" or "gamma") or blend(97.34, 5566, 0.25)
q = "warm {math.floor(26.61)} {string.rep('=', 1 + 1)} beta done"
r = (82.42 > 36) and (acc ~= 0 and "warm" or "gamma") or blend(82.42, 8736, 0.25)
m = { ["k5323"] = { nested = { deeper = { value = "warm", list = { 1, 2, 5323 } } } } }
acc = acc + blend(80.03, 90, math.sin(80.03 / 7)) * (1 + 3 / 31)
for j = 1, 35 do acc = acc + (j * 51.14) % 97 / (j + 1) end
m = { ["k6349"] = { nested = { deeper = { value = "cold", list = { 1, 2, 6349 } } } } }
r = (40.76 > 24) and (acc ~= 0 and "warm" or "alpha") or blend(40.76, 3674, 0.25)
if acc > 5.48 then acc = acc / 31 elseif acc < -5.48 then acc = -acc else acc = acc + 1632 end
bag = { id = 742, name = "fast-delta", pos = { 95.85, 742.5, 191.7 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 62.56) end
for j = 1, 20 do acc = acc + (j * 41.57) % 97 / (j + 1) end
acc = acc + blend(42.66, 47, math.sin(42.66 / 7)) * (1 + 0 / 31)
acc = acc + blend(71.48, 91, math.sin(71.48 / 7)) * (1 + 2 / 31)
acc = acc + blend(19.98, 70, math.sin(19.98 / 7)) * (1 + 0 / 31)
q = "warm {math.floor(94.78)} {string.rep('=', 1 + 1)} delta done"
q = "warm {math.floor(58.98)} {string.rep('=', 4 + 1)} alpha done"
m = { ["k2522"] = { nested = { deeper = { value = "cold", list = { 1, 2, 2522 } } } } }
m = { ["k1204"] = { nested = { deeper = { value = "cold", list = { 1, 2, 1204 } } } } }
if acc > 11.87 then acc = acc / 74 elseif acc < -11.87 then acc = -acc else acc = acc + 7549 end
acc = acc + blend(23.79, 21, math.sin(23.79 / 7)) * (1 + 5 / 31)
r = (62.98 > 8) and (acc ~= 0 and "blue" or "delta") or blend(62.98, 6858, 0.25)
for j = 1, 4 do acc = acc + (j * 79.43) % 97 / (j + 1) end
r = (85.93 > 35) and (acc ~= 0 and "blue" or "delta") or blend(85.93, 7235, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 64.51) end
bag = { id = 6253, name = "warm-gamma", pos = { 72.37, 6253.5, 144.74 }, ok = false }
acc = acc + blend(70.70, 68, math.sin(70.70 / 7)) * (1 + 1 / 31)
q = "red {math.floor(31.25)} {string.rep('=', 1 + 1)} gamma done"
if acc > 57.84 then acc = acc / 16 elseif acc < -57.84 then acc = -acc else acc = acc + 5177 end
if acc > 26.12 then acc = acc / 87 elseif acc < -26.12 then acc = -acc else acc = acc + 8986 end
r = (68.04 > 16) and (acc ~= 0 and "green" or "delta") or blend(68.04, 9516, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 49.13) end
bag = { id = 35, name = "cold-delta", pos = { 64.18, 35.5, 128.36 }, ok = false }
for j = 1, 11 do acc = acc + (j * 14.25) % 97 / (j + 1) end
acc = acc + blend(97.78, 53, math.sin(97.78 / 7)) * (1 + 6 / 31)
m = { ["k4837"] = { nested = { deeper = { value = "cold", list = { 1, 2, 4837 } } } } }
bag = { id = 1157, name = "blue-delta", pos = { 37.84, 1157.5, 75.68 }, ok = false }
q = "green {math.floor(23.65)} {string.rep('=', 4 + 1)} beta done"
if acc > 76.10 then acc = acc / 7 elseif acc < -76.10 then acc = -acc else acc = acc + 95 end
acc = acc + blend(59.18, 49, math.sin(59.18 / 7)) * (1 + 2 / 31)
q = "fast {math.floor(70.94)} {string.rep('=', 2 + 1)} gamma done"
r = (85.71 > 20) and (acc ~= 0 and "blue" or "gamma") or blend(85.71, 2020, 0.25)
if acc > 58.73 then acc = acc / 9 elseif acc < -58.73 then acc = -acc else acc = acc + 5170 end
for j = 1, 35 do acc = acc + (j * 5.48) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 53.89) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 58.95) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 32.83) end
if acc > 23.13 then acc = acc / 42 elseif acc < -23.13 then acc = -acc else acc = acc + 1643 end
for j = 1, 25 do acc = acc + (j * 60.10) % 97 / (j + 1) end
for j = 1, 8 do acc = acc + (j * 72.99) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 21.55) end
m = { ["k621"] = { nested = { deeper = { value = "green", list = { 1, 2, 621 } } } } }
r = (87.89 > 47) and (acc ~= 0 and "green" or "alpha") or blend(87.89, 9547, 0.25)
m = { ["k4590"] = { nested = { deeper = { value = "green", list = { 1, 2, 4590 } } } } }
bag = { id = 1050, name = "fast-gamma", pos = { 96.42, 1050.5, 192.84 }, ok = true }
bag = { id = 9684, name = "warm-delta", pos = { 53.94, 9684.5, 107.88 }, ok = true }
r = (17.27 > 5) and (acc ~= 0 and "red" or "delta") or blend(17.27, 2705, 0.25)
m = { ["k1251"] = { nested = { deeper = { value = "warm", list = { 1, 2, 1251 } } } } }
bag = { id = 5752, name = "fast-gamma", pos = { 18.05, 5752.5, 36.1 }, ok = true }
for j = 1, 6 do acc = acc + (j * 79.10) % 97 / (j + 1) end
m = { ["k2537"] = { nested = { deeper = { value = "green", list = { 1, 2, 2537 } } } } }
bag = { id = 6128, name = "red-gamma", pos = { 48.01, 6128.5, 96.02 }, ok = true }
for j = 1, 4 do acc = acc + (j * 50.30) % 97 / (j + 1) end
acc = acc + blend(25.32, 31, math.sin(25.32 / 7)) * (1 + 11 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 42.40) end
if acc > 54.83 then acc = acc / 25 elseif acc < -54.83 then acc = -acc else acc = acc + 9458 end
bag = { id = 6761, name = "red-beta", pos = { 18.30, 6761.5, 36.6 }, ok = false }
q = "warm {math.floor(9.63)} {string.rep('=', 5 + 1)} beta done"
r = (58.51 > 5) and (acc ~= 0 and "warm" or "delta") or blend(58.51, 3755, 0.25)
for j = 1, 36 do acc = acc + (j * 53.93) % 97 / (j + 1) end
q = "red {math.floor(5.69)} {string.rep('=', 1 + 1)} alpha done"
if acc > 7.89 then acc = acc / 30 elseif acc < -7.89 then acc = -acc else acc = acc + 4479 end
for j = 1, 20 do acc = acc + (j * 10.16) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 12.32) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 11.25) end
acc = acc + blend(87.76, 7, math.sin(87.76 / 7)) * (1 + 1 / 31)
if acc > 58.00 then acc = acc / 29 elseif acc < -58.00 then acc = -acc else acc = acc + 3677 end
if acc > 77.31 then acc = acc / 22 elseif acc < -77.31 then acc = -acc else acc = acc + 8921 end
if acc > 10.38 then acc = acc / 43 elseif acc < -10.38 then acc = -acc else acc = acc + 2801 end
if acc > 86.58 then acc = acc / 13 elseif acc < -86.58 then acc = -acc else acc = acc + 2237 end
bag = { id = 8903, name = "green-delta", pos = { 86.54, 8903.5, 173.08 }, ok = false }
bag = { id = 3214, name = "red-gamma", pos = { 68.13, 3214.5, 136.26 }, ok = true }
r = (91.67 > 6) and (acc ~= 0 and "green" or "delta") or blend(91.67, 456, 0.25)
acc = acc + blend(73.40, 89, math.sin(73.40 / 7)) * (1 + 8 / 31)
bag = { id = 1890, name = "cold-alpha", pos = { 84.37, 1890.5, 168.74 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 93.10) end
q = "cold {math.floor(63.81)} {string.rep('=', 4 + 1)} delta done"
m = { ["k9565"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9565 } } } } }
bag = { id = 7104, name = "fast-gamma", pos = { 1.69, 7104.5, 3.38 }, ok = true }
m = { ["k5878"] = { nested = { deeper = { value = "red", list = { 1, 2, 5878 } } } } }
for j = 1, 33 do acc = acc + (j * 53.33) % 97 / (j + 1) end
bag = { id = 2020, name = "warm-alpha", pos = { 44.16, 2020.5, 88.32 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 68.20) end
if acc > 43.04 then acc = acc / 88 elseif acc < -43.04 then acc = -acc else acc = acc + 8809 end
bag = { id = 847, name = "cold-alpha", pos = { 38.29, 847.5, 76.58 }, ok = false }
m = { ["k7917"] = { nested = { deeper = { value = "red", list = { 1, 2, 7917 } } } } }
r = (79.02 > 18) and (acc ~= 0 and "blue" or "alpha") or blend(79.02, 168, 0.25)
bag = { id = 755, name = "cold-gamma", pos = { 47.48, 755.5, 94.96 }, ok = false }
q = "green {math.floor(88.37)} {string.rep('=', 3 + 1)} gamma done"
q = "red {math.floor(40.86)} {string.rep('=', 1 + 1)} beta done"
m = { ["k2812"] = { nested = { deeper = { value = "cold", list = { 1, 2, 2812 } } } } }
for j = 1, 10 do acc = acc + (j * 32.72) % 97 / (j + 1) end
bag = { id = 8773, name = "warm-beta", pos = { 6.60, 8773.5, 13.2 }, ok = false }
q = "warm {math.floor(2.75)} {string.rep('=', 1 + 1)} delta done"
if acc > 20.58 then acc = acc / 82 elseif acc < -20.58 then acc = -acc else acc = acc + 8625 end
for j = 1, 8 do acc = acc + (j * 61.15) % 97 / (j + 1) end
if acc > 93.02 then acc = acc / 35 elseif acc < -93.02 then acc = -acc else acc = acc + 2170 end
r = (94.29 > 26) and (acc ~= 0 and "green" or "beta") or blend(94.29, 2976, 0.25)
q = "blue {math.floor(26.05)} {string.rep('=', 4 + 1)} alpha done"
bag = { id = 712, name = "cold-delta", pos = { 59.88, 712.5, 119.76 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 75.73) end
acc = acc + blend(59.14, 53, math.sin(59.14 / 7)) * (1 + 1 / 31)
for j = 1, 33 do acc = acc + (j * 48.62) % 97 / (j + 1) end
if acc > 60.02 then acc = acc / 39 elseif acc < -60.02 then acc = -acc else acc = acc + 3954 end
q = "cold {math.floor(79.44)} {string.rep('=', 1 + 1)} delta done"
acc = acc + blend(33.24, 28, math.sin(33.24 / 7)) * (1 + 10 / 31)
acc = acc + blend(78.43, 68, math.sin(78.43 / 7)) * (1 + 1 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 74.02) end
if acc > 26.97 then acc = acc / 53 elseif acc < -26.97 then acc = -acc else acc = acc + 5125 end
acc = acc + blend(46.43, 28, math.sin(46.43 / 7)) * (1 + 12 / 31)
q = "warm {math.floor(3.47)} {string.rep('=', 5 + 1)} delta done"
if acc > 87.48 then acc = acc / 70 elseif acc < -87.48 then acc = -acc else acc = acc + 1315 end
for j = 1, 11 do acc = acc + (j * 93.96) % 97 / (j + 1) end
for j = 1, 35 do acc = acc + (j * 73.71) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 90.68) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 67.18) end
bag = { id = 5961, name = "warm-gamma", pos = { 62.16, 5961.5, 124.32 }, ok = false }
q = "green {math.floor(50.29)} {string.rep('=', 0 + 1)} beta done"
r = (97.64 > 1) and (acc ~= 0 and "warm" or "alpha") or blend(97.64, 6401, 0.25)
q = "warm {math.floor(11.64)} {string.rep('=', 0 + 1)} alpha done"
r = (41.46 > 47) and (acc ~= 0 and "warm" or "gamma") or blend(41.46, 7997, 0.25)
for j = 1, 8 do acc = acc + (j * 89.46) % 97 / (j + 1) end
m = { ["k5977"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5977 } } } } }
bag = { id = 2819, name = "red-gamma", pos = { 96.06, 2819.5, 192.12 }, ok = false }
acc = acc + blend(56.24, 92, math.sin(56.24 / 7)) * (1 + 6 / 31)
m = { ["k5164"] = { nested = { deeper = { value = "blue", list = { 1, 2, 5164 } } } } }
bag = { id = 3087, name = "blue-beta", pos = { 26.16, 3087.5, 52.32 }, ok = false }
m = { ["k4073"] = { nested = { deeper = { value = "warm", list = { 1, 2, 4073 } } } } }
if acc > 16.02 then acc = acc / 18 elseif acc < -16.02 then acc = -acc else acc = acc + 1085 end
if acc > 63.76 then acc = acc / 1 elseif acc < -63.76 then acc = -acc else acc = acc + 534 end
m = { ["k4899"] = { nested = { deeper = { value = "green", list = { 1, 2, 4899 } } } } }
r = (35.28 > 9) and (acc ~= 0 and "warm" or "gamma") or blend(35.28, 3959, 0.25)
r = (16.04 > 28) and (acc ~= 0 and "blue" or "gamma") or blend(16.04, 3178, 0.25)
if acc > 72.68 then acc = acc / 2 elseif acc < -72.68 then acc = -acc else acc = acc + 1069 end
q = "fast {math.floor(33.20)} {string.rep('=', 0 + 1)} beta done"
bag = { id = 9965, name = "warm-beta", pos = { 53.26, 9965.5, 106.52 }, ok = false }
for j = 1, 13 do acc = acc + (j * 37.85) % 97 / (j + 1) end
r = (22.37 > 21) and (acc ~= 0 and "blue" or "gamma") or blend(22.37, 121, 0.25)
q = "warm {math.floor(36.82)} {string.rep('=', 0 + 1)} alpha done"
q = "green {math.floor(62.00)} {string.rep('=', 2 + 1)} alpha done"
for j = 1, 37 do acc = acc + (j * 45.34) % 97 / (j + 1) end
r = (68.20 > 26) and (acc ~= 0 and "red" or "gamma") or blend(68.20, 1926, 0.25)
if acc > 34.88 then acc = acc / 33 elseif acc < -34.88 then acc = -acc else acc = acc + 3236 end
if acc > 50.87 then acc = acc / 55 elseif acc < -50.87 then acc = -acc else acc = acc + 143 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 96.47) end
bag = { id = 4280, name = "cold-alpha", pos = { 47.84, 4280.5, 95.68 }, ok = true }
if acc > 5.36 then acc = acc / 6 elseif acc < -5.36 then acc = -acc else acc = acc + 8460 end
m = { ["k3241"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3241 } } } } }
bag = { id = 2066, name = "green-beta", pos = { 16.10, 2066.5, 32.2 }, ok = true }
bag = { id = 5362, name = "fast-delta", pos = { 12.77, 5362.5, 25.54 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 98.42) end
q = "cold {math.floor(87.07)} {string.rep('=', 2 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 0.09) end
if acc > 74.99 then acc = acc / 63 elseif acc < -74.99 then acc = -acc else acc = acc + 5491 end
q = "red {math.floor(99.72)} {string.rep('=', 3 + 1)} alpha done"
if acc > 25.31 then acc = acc / 34 elseif acc < -25.31 then acc = -acc else acc = acc + 8666 end
if acc > 46.08 then acc = acc / 30 elseif acc < -46.08 then acc = -acc else acc = acc + 8573 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 33.24) end
for j = 1, 13 do acc = acc + (j * 77.48) % 97 / (j + 1) end
r = (18.85 > 0) and (acc ~= 0 and "green" or "gamma") or blend(18.85, 2100, 0.25)
m = { ["k981"] = { nested = { deeper = { value = "fast", list = { 1, 2, 981 } } } } }
for j = 1, 27 do acc = acc + (j * 67.66) % 97 / (j + 1) end
acc = acc + blend(96.04, 58, math.sin(96.04 / 7)) * (1 + 1 / 31)
for j = 1, 20 do acc = acc + (j * 27.45) % 97 / (j + 1) end
m = { ["k3835"] = { nested = { deeper = { value = "fast", list = { 1, 2, 3835 } } } } }
if acc > 16.86 then acc = acc / 40 elseif acc < -16.86 then acc = -acc else acc = acc + 1018 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 3.73) end
m = { ["k511"] = { nested = { deeper = { value = "fast", list = { 1, 2, 511 } } } } }
acc = acc + blend(8.96, 29, math.sin(8.96 / 7)) * (1 + 10 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 76.75) end
m = { ["k780"] = { nested = { deeper = { value = "fast", list = { 1, 2, 780 } } } } }
q = "blue {math.floor(36.06)} {string.rep('=', 6 + 1)} gamma done"
acc = acc + blend(92.41, 31, math.sin(92.41 / 7)) * (1 + 4 / 31)
for j = 1, 24 do acc = acc + (j * 84.86) % 97 / (j + 1) end
if acc > 34.75 then acc = acc / 58 elseif acc < -34.75 then acc = -acc else acc = acc + 146 end
for j = 1, 35 do acc = acc + (j * 95.23) % 97 / (j + 1) end
q = "fast {math.floor(56.41)} {string.rep('=', 0 + 1)} delta done"
m = { ["k806"] = { nested = { deeper = { value = "red", list = { 1, 2, 806 } } } } }
m = { ["k4371"] = { nested = { deeper = { value = "blue", list = { 1, 2, 4371 } } } } }
acc = acc + blend(3.21, 62, math.sin(3.21 / 7)) * (1 + 12 / 31)
m = { ["k4278"] = { nested = { deeper = { value = "fast", list = { 1, 2, 4278 } } } } }
m = { ["k1929"] = { nested = { deeper = { value = "warm", list = { 1, 2, 1929 } } } } }
m = { ["k4108"] = { nested = { deeper = { value = "green", list = { 1, 2, 4108 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 33.85) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 75.83) end
m = { ["k8571"] = { nested = { deeper = { value = "fast", list = { 1, 2, 8571 } } } } }
if acc > 81.65 then acc = acc / 38 elseif acc < -81.65 then acc = -acc else acc = acc + 5377 end
bag = { id = 183, name = "fast-alpha", pos = { 47.15, 183.5, 94.3 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 81.65) end
q = "blue {math.floor(39.62)} {string.rep('=', 3 + 1)} beta done"
r = (51.62 > 40) and (acc ~= 0 and "blue" or "alpha") or blend(51.62, 8940, 0.25)
q = "blue {math.floor(50.21)} {string.rep('=', 2 + 1)} delta done"
m = { ["k8363"] = { nested = { deeper = { value = "red", list = { 1, 2, 8363 } } } } }
for j = 1, 10 do acc = acc + (j * 10.29) % 97 / (j + 1) end
q = "fast {math.floor(97.89)} {string.rep('=', 6 + 1)} delta done"
bag = { id = 712, name = "blue-gamma", pos = { 55.64, 712.5, 111.28 }, ok = true }
if acc > 90.57 then acc = acc / 45 elseif acc < -90.57 then acc = -acc else acc = acc + 9389 end
if acc > 39.14 then acc = acc / 80 elseif acc < -39.14 then acc = -acc else acc = acc + 1058 end
m = { ["k4996"] = { nested = { deeper = { value = "fast", list = { 1, 2, 4996 } } } } }
if acc > 63.79 then acc = acc / 17 elseif acc < -63.79 then acc = -acc else acc = acc + 1262 end
if acc > 23.88 then acc = acc / 65 elseif acc < -23.88 then acc = -acc else acc = acc + 6383 end
bag = { id = 367, name = "red-alpha", pos = { 15.13, 367.5, 30.26 }, ok = false }
r = (21.48 > 46) and (acc ~= 0 and "fast" or "delta") or blend(21.48, 8346, 0.25)
acc = acc + blend(20.54, 25, math.sin(20.54 / 7)) * (1 + 9 / 31)
r = (55.75 > 21) and (acc ~= 0 and "warm" or "delta") or blend(55.75, 1621, 0.25)
for j = 1, 34 do acc = acc + (j * 64.59) % 97 / (j + 1) end
if acc > 68.47 then acc = acc / 82 elseif acc < -68.47 then acc = -acc else acc = acc + 7290 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 27.14) end
m = { ["k4374"] = { nested = { deeper = { value = "red", list = { 1, 2, 4374 } } } } }
m = { ["k5117"] = { nested = { deeper = { value = "green", list = { 1, 2, 5117 } } } } }
acc = acc + blend(62.77, 78, math.sin(62.77 / 7)) * (1 + 10 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 3.60) end
acc = acc + blend(11.34, 28, math.sin(11.34 / 7)) * (1 + 10 / 31)
if acc > 55.53 then acc = acc / 6 elseif acc < -55.53 then acc = -acc else acc = acc + 3565 end
q = "red {math.floor(29.90)} {string.rep('=', 5 + 1)} gamma done"
acc = acc + blend(32.35, 10, math.sin(32.35 / 7)) * (1 + 9 / 31)
acc = acc + blend(69.35, 66, math.sin(69.35 / 7)) * (1 + 4 / 31)
acc = acc + blend(49.02, 52, math.sin(49.02 / 7)) * (1 + 7 / 31)
if acc > 39.16 then acc = acc / 34 elseif acc < -39.16 then acc = -acc else acc = acc + 7064 end
m = { ["k7822"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7822 } } } } }
r = (82.54 > 30) and (acc ~= 0 and "blue" or "beta") or blend(82.54, 8880, 0.25)
q = "red {math.floor(33.58)} {string.rep('=', 5 + 1)} gamma done"
if acc > 52.28 then acc = acc / 13 elseif acc < -52.28 then acc = -acc else acc = acc + 4106 end
q = "warm {math.floor(48.75)} {string.rep('=', 5 + 1)} gamma done"
bag = { id = 519, name = "green-beta", pos = { 38.11, 519.5, 76.22 }, ok = false }
r = (28.79 > 16) and (acc ~= 0 and "fast" or "delta") or blend(28.79, 9216, 0.25)
bag = { id = 2454, name = "cold-alpha", pos = { 63.55, 2454.5, 127.1 }, ok = true }
q = "red {math.floor(34.41)} {string.rep('=', 0 + 1)} delta done"
acc = acc + blend(19.43, 54, math.sin(19.43 / 7)) * (1 + 3 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 44.82) end
q = "blue {math.floor(39.21)} {string.rep('=', 6 + 1)} gamma done"
bag = { id = 4293, name = "fast-gamma", pos = { 91.83, 4293.5, 183.66 }, ok = false }
bag = { id = 5600, name = "cold-gamma", pos = { 2.79, 5600.5, 5.58 }, ok = true }
r = (39.09 > 11) and (acc ~= 0 and "cold" or "gamma") or blend(39.09, 5461, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 89.08) end
q = "red {math.floor(85.07)} {string.rep('=', 4 + 1)} beta done"
q = "fast {math.floor(36.98)} {string.rep('=', 4 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 83.39) end
if acc > 27.99 then acc = acc / 42 elseif acc < -27.99 then acc = -acc else acc = acc + 8585 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 23.45) end
r = (26.20 > 11) and (acc ~= 0 and "blue" or "delta") or blend(26.20, 3611, 0.25)
for j = 1, 16 do acc = acc + (j * 30.80) % 97 / (j + 1) end
q = "green {math.floor(10.12)} {string.rep('=', 2 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 10.10) end
for j = 1, 26 do acc = acc + (j * 61.35) % 97 / (j + 1) end
q = "green {math.floor(96.39)} {string.rep('=', 0 + 1)} beta done"
q = "warm {math.floor(43.73)} {string.rep('=', 2 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 83.28) end
r = (70.33 > 17) and (acc ~= 0 and "red" or "delta") or blend(70.33, 7017, 0.25)
m = { ["k2593"] = { nested = { deeper = { value = "warm", list = { 1, 2, 2593 } } } } }
if acc > 19.47 then acc = acc / 25 elseif acc < -19.47 then acc = -acc else acc = acc + 3762 end
bag = { id = 946, name = "blue-alpha", pos = { 1.83, 946.5, 3.66 }, ok = true }
q = "blue {math.floor(82.41)} {string.rep('=', 1 + 1)} delta done"
if acc > 70.03 then acc = acc / 4 elseif acc < -70.03 then acc = -acc else acc = acc + 4809 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 9.69) end
r = (51.50 > 20) and (acc ~= 0 and "blue" or "beta") or blend(51.50, 8920, 0.25)
r = (27.05 > 14) and (acc ~= 0 and "blue" or "delta") or blend(27.05, 5514, 0.25)
r = (93.07 > 7) and (acc ~= 0 and "fast" or "beta") or blend(93.07, 4557, 0.25)
m = { ["k6964"] = { nested = { deeper = { value = "cold", list = { 1, 2, 6964 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 73.65) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 12.57) end
for j = 1, 29 do acc = acc + (j * 96.99) % 97 / (j + 1) end
for j = 1, 34 do acc = acc + (j * 13.85) % 97 / (j + 1) end
if acc > 42.76 then acc = acc / 80 elseif acc < -42.76 then acc = -acc else acc = acc + 8089 end
q = "warm {math.floor(54.93)} {string.rep('=', 4 + 1)} beta done"
r = (33.02 > 23) and (acc ~= 0 and "cold" or "gamma") or blend(33.02, 3423, 0.25)
m = { ["k880"] = { nested = { deeper = { value = "red", list = { 1, 2, 880 } } } } }
r = (44.61 > 25) and (acc ~= 0 and "green" or "alpha") or blend(44.61, 9925, 0.25)
q = "cold {math.floor(51.79)} {string.rep('=', 6 + 1)} beta done"
if acc > 25.04 then acc = acc / 35 elseif acc < -25.04 then acc = -acc else acc = acc + 7866 end
if acc > 31.85 then acc = acc / 78 elseif acc < -31.85 then acc = -acc else acc = acc + 2480 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 31.35) end
q = "warm {math.floor(22.49)} {string.rep('=', 0 + 1)} delta done"
if acc > 40.41 then acc = acc / 79 elseif acc < -40.41 then acc = -acc else acc = acc + 5596 end
bag = { id = 7929, name = "green-alpha", pos = { 58.90, 7929.5, 117.8 }, ok = false }
if acc > 55.24 then acc = acc / 67 elseif acc < -55.24 then acc = -acc else acc = acc + 4249 end
q = "red {math.floor(58.86)} {string.rep('=', 4 + 1)} beta done"
bag = { id = 2497, name = "cold-gamma", pos = { 86.97, 2497.5, 173.94 }, ok = false }
for j = 1, 24 do acc = acc + (j * 5.99) % 97 / (j + 1) end
r = (7.63 > 39) and (acc ~= 0 and "fast" or "delta") or blend(7.63, 1239, 0.25)
q = "warm {math.floor(57.57)} {string.rep('=', 2 + 1)} gamma done"
for j = 1, 12 do acc = acc + (j * 75.55) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 71.38) end
bag = { id = 7822, name = "green-gamma", pos = { 14.57, 7822.5, 29.14 }, ok = true }
m = { ["k1578"] = { nested = { deeper = { value = "fast", list = { 1, 2, 1578 } } } } }
r = (11.68 > 18) and (acc ~= 0 and "green" or "gamma") or blend(11.68, 8818, 0.25)
r = (80.64 > 28) and (acc ~= 0 and "fast" or "gamma") or blend(80.64, 7328, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 65.80) end
q = "fast {math.floor(3.03)} {string.rep('=', 1 + 1)} delta done"
for j = 1, 36 do acc = acc + (j * 5.90) % 97 / (j + 1) end
q = "green {math.floor(53.29)} {string.rep('=', 2 + 1)} delta done"
m = { ["k8587"] = { nested = { deeper = { value = "fast", list = { 1, 2, 8587 } } } } }
m = { ["k7247"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7247 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 16.56) end
acc = acc + blend(93.83, 57, math.sin(93.83 / 7)) * (1 + 6 / 31)
if acc > 22.75 then acc = acc / 63 elseif acc < -22.75 then acc = -acc else acc = acc + 7538 end
acc = acc + blend(78.60, 67, math.sin(78.60 / 7)) * (1 + 9 / 31)
r = (64.11 > 36) and (acc ~= 0 and "cold" or "alpha") or blend(64.11, 3486, 0.25)
acc = acc + blend(39.85, 52, math.sin(39.85 / 7)) * (1 + 6 / 31)
acc = acc + blend(59.43, 30, math.sin(59.43 / 7)) * (1 + 4 / 31)
m = { ["k1166"] = { nested = { deeper = { value = "blue", list = { 1, 2, 1166 } } } } }
r = (81.18 > 20) and (acc ~= 0 and "red" or "gamma") or blend(81.18, 9520, 0.25)
r = (25.11 > 4) and (acc ~= 0 and "green" or "delta") or blend(25.11, 1954, 0.25)
q = "warm {math.floor(68.32)} {string.rep('=', 6 + 1)} delta done"
m = { ["k6575"] = { nested = { deeper = { value = "warm", list = { 1, 2, 6575 } } } } }
for j = 1, 16 do acc = acc + (j * 87.23) % 97 / (j + 1) end
q = "fast {math.floor(40.33)} {string.rep('=', 5 + 1)} gamma done"
if acc > 66.86 then acc = acc / 62 elseif acc < -66.86 then acc = -acc else acc = acc + 4244 end
if acc > 18.40 then acc = acc / 82 elseif acc < -18.40 then acc = -acc else acc = acc + 1416 end
q = "blue {math.floor(2.09)} {string.rep('=', 5 + 1)} delta done"
bag = { id = 9223, name = "cold-beta", pos = { 99.63, 9223.5, 199.26 }, ok = false }
m = { ["k2818"] = { nested = { deeper = { value = "green", list = { 1, 2, 2818 } } } } }
acc = acc + blend(98.90, 45, math.sin(98.90 / 7)) * (1 + 9 / 31)
q = "cold {math.floor(37.24)} {string.rep('=', 1 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 45.23) end
r = (79.72 > 25) and (acc ~= 0 and "fast" or "gamma") or blend(79.72, 5625, 0.25)
acc = acc + blend(84.01, 7, math.sin(84.01 / 7)) * (1 + 7 / 31)
bag = { id = 4335, name = "warm-beta", pos = { 29.03, 4335.5, 58.06 }, ok = false }
q = "cold {math.floor(48.49)} {string.rep('=', 1 + 1)} beta done"
r = (69.28 > 10) and (acc ~= 0 and "green" or "delta") or blend(69.28, 2660, 0.25)
m = { ["k203"] = { nested = { deeper = { value = "cold", list = { 1, 2, 203 } } } } }
if acc > 2.84 then acc = acc / 15 elseif acc < -2.84 then acc = -acc else acc = acc + 2595 end
m = { ["k2046"] = { nested = { deeper = { value = "warm", list = { 1, 2, 2046 } } } } }
m = { ["k5183"] = { nested = { deeper = { value = "blue", list = { 1, 2, 5183 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 85.67) end
acc = acc + blend(82.26, 22, math.sin(82.26 / 7)) * (1 + 6 / 31)
if acc > 25.39 then acc = acc / 66 elseif acc < -25.39 then acc = -acc else acc = acc + 8164 end
if acc > 58.26 then acc = acc / 20 elseif acc < -58.26 then acc = -acc else acc = acc + 3935 end
bag = { id = 2898, name = "cold-beta", pos = { 61.17, 2898.5, 122.34 }, ok = true }
m = { ["k2541"] = { nested = { deeper = { value = "red", list = { 1, 2, 2541 } } } } }
m = { ["k5359"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5359 } } } } }
acc = acc + blend(35.10, 56, math.sin(35.10 / 7)) * (1 + 2 / 31)
acc = acc + blend(15.20, 51, math.sin(15.20 / 7)) * (1 + 10 / 31)
q = "warm {math.floor(51.21)} {string.rep('=', 3 + 1)} alpha done"
bag = { id = 4518, name = "blue-gamma", pos = { 79.42, 4518.5, 158.84 }, ok = true }
r = (22.18 > 11) and (acc ~= 0 and "red" or "delta") or blend(22.18, 6261, 0.25)
m = { ["k4564"] = { nested = { deeper = { value = "warm", list = { 1, 2, 4564 } } } } }
for j = 1, 2 do acc = acc + (j * 48.23) % 97 / (j + 1) end
bag = { id = 5280, name = "cold-alpha", pos = { 37.19, 5280.5, 74.38 }, ok = true }
r = (68.12 > 35) and (acc ~= 0 and "cold" or "gamma") or blend(68.12, 6235, 0.25)
acc = acc + blend(18.54, 64, math.sin(18.54 / 7)) * (1 + 2 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 74.38) end
r = (84.19 > 44) and (acc ~= 0 and "red" or "beta") or blend(84.19, 8044, 0.25)
q = "blue {math.floor(62.23)} {string.rep('=', 0 + 1)} delta done"
m = { ["k9148"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9148 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 92.18) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 92.95) end
r = (50.51 > 35) and (acc ~= 0 and "cold" or "delta") or blend(50.51, 435, 0.25)
bag = { id = 1052, name = "red-gamma", pos = { 23.36, 1052.5, 46.72 }, ok = true }
acc = acc + blend(0.43, 23, math.sin(0.43 / 7)) * (1 + 10 / 31)
bag = { id = 5154, name = "cold-alpha", pos = { 23.18, 5154.5, 46.36 }, ok = true }
q = "warm {math.floor(9.92)} {string.rep('=', 5 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 40.58) end
bag = { id = 4153, name = "cold-beta", pos = { 52.24, 4153.5, 104.48 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 14.97) end
acc = acc + blend(82.54, 64, math.sin(82.54 / 7)) * (1 + 6 / 31)
q = "red {math.floor(72.78)} {string.rep('=', 3 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 30.62) end
if acc > 31.36 then acc = acc / 18 elseif acc < -31.36 then acc = -acc else acc = acc + 106 end
acc = acc + blend(96.09, 35, math.sin(96.09 / 7)) * (1 + 3 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 79.07) end
acc = acc + blend(20.67, 91, math.sin(20.67 / 7)) * (1 + 0 / 31)
r = (83.04 > 24) and (acc ~= 0 and "warm" or "delta") or blend(83.04, 2974, 0.25)
q = "cold {math.floor(22.51)} {string.rep('=', 2 + 1)} alpha done"
if acc > 65.48 then acc = acc / 35 elseif acc < -65.48 then acc = -acc else acc = acc + 4484 end
q = "green {math.floor(42.39)} {string.rep('=', 1 + 1)} delta done"
bag = { id = 2214, name = "fast-gamma", pos = { 92.99, 2214.5, 185.98 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 51.05) end
if acc > 95.98 then acc = acc / 54 elseif acc < -95.98 then acc = -acc else acc = acc + 1032 end
acc = acc + blend(33.20, 18, math.sin(33.20 / 7)) * (1 + 5 / 31)
for j = 1, 2 do acc = acc + (j * 83.82) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 77.49) end
q = "blue {math.floor(1.92)} {string.rep('=', 2 + 1)} alpha done"
bag = { id = 7099, name = "blue-alpha", pos = { 94.27, 7099.5, 188.54 }, ok = false }
for j = 1, 26 do acc = acc + (j * 95.77) % 97 / (j + 1) end
if acc > 36.05 then acc = acc / 24 elseif acc < -36.05 then acc = -acc else acc = acc + 6876 end
q = "green {math.floor(60.44)} {string.rep('=', 2 + 1)} alpha done"
bag = { id = 7055, name = "fast-beta", pos = { 39.58, 7055.5, 79.16 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 68.02) end
bag = { id = 7237, name = "cold-alpha", pos = { 25.47, 7237.5, 50.94 }, ok = false }
r = (36.93 > 30) and (acc ~= 0 and "blue" or "gamma") or blend(36.93, 7180, 0.25)
acc = acc + blend(12.02, 25, math.sin(12.02 / 7)) * (1 + 0 / 31)
for j = 1, 8 do acc = acc + (j * 41.19) % 97 / (j + 1) end
m = { ["k675"] = { nested = { deeper = { value = "blue", list = { 1, 2, 675 } } } } }
m = { ["k2597"] = { nested = { deeper = { value = "red", list = { 1, 2, 2597 } } } } }
bag = { id = 4404, name = "fast-beta", pos = { 55.29, 4404.5, 110.58 }, ok = true }
bag = { id = 3486, name = "warm-gamma", pos = { 63.91, 3486.5, 127.82 }, ok = true }
bag = { id = 2062, name = "cold-delta", pos = { 68.61, 2062.5, 137.22 }, ok = true }
q = "red {math.floor(38.48)} {string.rep('=', 3 + 1)} alpha done"
bag = { id = 3918, name = "red-beta", pos = { 55.98, 3918.5, 111.96 }, ok = true }
if acc > 11.10 then acc = acc / 85 elseif acc < -11.10 then acc = -acc else acc = acc + 9340 end
q = "red {math.floor(3.34)} {string.rep('=', 4 + 1)} beta done"
m = { ["k9759"] = { nested = { deeper = { value = "green", list = { 1, 2, 9759 } } } } }
acc = acc + blend(94.59, 78, math.sin(94.59 / 7)) * (1 + 2 / 31)
bag = { id = 6714, name = "red-delta", pos = { 47.01, 6714.5, 94.02 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 17.86) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 52.71) end
if acc > 69.75 then acc = acc / 12 elseif acc < -69.75 then acc = -acc else acc = acc + 2325 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 90.58) end
for j = 1, 32 do acc = acc + (j * 83.34) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 15.16) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 84.72) end
for j = 1, 16 do acc = acc + (j * 66.11) % 97 / (j + 1) end
m = { ["k3650"] = { nested = { deeper = { value = "green", list = { 1, 2, 3650 } } } } }
bag = { id = 9789, name = "cold-beta", pos = { 75.24, 9789.5, 150.48 }, ok = false }
r = (10.03 > 30) and (acc ~= 0 and "fast" or "gamma") or blend(10.03, 4930, 0.25)
q = "cold {math.floor(47.04)} {string.rep('=', 3 + 1)} beta done"
m = { ["k2569"] = { nested = { deeper = { value = "blue", list = { 1, 2, 2569 } } } } }
bag = { id = 5833, name = "warm-beta", pos = { 61.64, 5833.5, 123.28 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 61.50) end
q = "blue {math.floor(25.47)} {string.rep('=', 1 + 1)} delta done"
q = "green {math.floor(48.48)} {string.rep('=', 0 + 1)} gamma done"
for j = 1, 28 do acc = acc + (j * 28.69) % 97 / (j + 1) end
bag = { id = 7782, name = "cold-gamma", pos = { 57.75, 7782.5, 115.5 }, ok = true }
q = "green {math.floor(65.57)} {string.rep('=', 3 + 1)} beta done"
q = "blue {math.floor(46.07)} {string.rep('=', 2 + 1)} gamma done"
q = "red {math.floor(90.34)} {string.rep('=', 6 + 1)} beta done"
bag = { id = 9418, name = "cold-alpha", pos = { 46.00, 9418.5, 92 }, ok = true }
m = { ["k4811"] = { nested = { deeper = { value = "cold", list = { 1, 2, 4811 } } } } }
if acc > 94.21 then acc = acc / 75 elseif acc < -94.21 then acc = -acc else acc = acc + 6927 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 46.49) end
for j = 1, 37 do acc = acc + (j * 10.52) % 97 / (j + 1) end
acc = acc + blend(40.56, 42, math.sin(40.56 / 7)) * (1 + 9 / 31)
r = (67.42 > 10) and (acc ~= 0 and "green" or "alpha") or blend(67.42, 3860, 0.25)
for j = 1, 9 do acc = acc + (j * 71.87) % 97 / (j + 1) end
bag = { id = 7412, name = "cold-gamma", pos = { 2.60, 7412.5, 5.2 }, ok = true }
acc = acc + blend(46.47, 10, math.sin(46.47 / 7)) * (1 + 5 / 31)
q = "blue {math.floor(62.68)} {string.rep('=', 1 + 1)} beta done"
for j = 1, 28 do acc = acc + (j * 62.88) % 97 / (j + 1) end
if acc > 56.18 then acc = acc / 59 elseif acc < -56.18 then acc = -acc else acc = acc + 681 end
r = (23.52 > 30) and (acc ~= 0 and "red" or "delta") or blend(23.52, 3830, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 17.81) end
q = "green {math.floor(62.58)} {string.rep('=', 2 + 1)} alpha done"
acc = acc + blend(78.20, 31, math.sin(78.20 / 7)) * (1 + 8 / 31)
q = "red {math.floor(21.54)} {string.rep('=', 3 + 1)} gamma done"
acc = acc + blend(71.00, 41, math.sin(71.00 / 7)) * (1 + 2 / 31)
q = "green {math.floor(9.26)} {string.rep('=', 0 + 1)} beta done"
bag = { id = 7882, name = "cold-gamma", pos = { 27.71, 7882.5, 55.42 }, ok = true }
for j = 1, 3 do acc = acc + (j * 27.20) % 97 / (j + 1) end
r = (88.03 > 43) and (acc ~= 0 and "red" or "gamma") or blend(88.03, 193, 0.25)
bag = { id = 3466, name = "warm-alpha", pos = { 78.89, 3466.5, 157.78 }, ok = true }
m = { ["k251"] = { nested = { deeper = { value = "green", list = { 1, 2, 251 } } } } }
m = { ["k8383"] = { nested = { deeper = { value = "green", list = { 1, 2, 8383 } } } } }
bag = { id = 1301, name = "cold-alpha", pos = { 8.70, 1301.5, 17.4 }, ok = false }
bag = { id = 4884, name = "red-gamma", pos = { 16.67, 4884.5, 33.34 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 39.41) end
acc = acc + blend(85.93, 24, math.sin(85.93 / 7)) * (1 + 12 / 31)
for j = 1, 40 do acc = acc + (j * 89.07) % 97 / (j + 1) end
q = "green {math.floor(78.05)} {string.rep('=', 5 + 1)} delta done"
q = "cold {math.floor(91.18)} {string.rep('=', 5 + 1)} beta done"
if acc > 31.39 then acc = acc / 41 elseif acc < -31.39 then acc = -acc else acc = acc + 9652 end
for j = 1, 16 do acc = acc + (j * 41.08) % 97 / (j + 1) end
q = "red {math.floor(24.60)} {string.rep('=', 6 + 1)} alpha done"
q = "cold {math.floor(4.20)} {string.rep('=', 3 + 1)} beta done"
if acc > 72.24 then acc = acc / 11 elseif acc < -72.24 then acc = -acc else acc = acc + 1612 end
r = (96.07 > 26) and (acc ~= 0 and "warm" or "beta") or blend(96.07, 9126, 0.25)
r = (44.95 > 29) and (acc ~= 0 and "fast" or "delta") or blend(44.95, 3829, 0.25)
if acc > 25.96 then acc = acc / 21 elseif acc < -25.96 then acc = -acc else acc = acc + 4470 end
for j = 1, 20 do acc = acc + (j * 63.81) % 97 / (j + 1) end
bag = { id = 5118, name = "fast-alpha", pos = { 43.80, 5118.5, 87.6 }, ok = true }
if acc > 96.06 then acc = acc / 38 elseif acc < -96.06 then acc = -acc else acc = acc + 9916 end
acc = acc + blend(32.33, 28, math.sin(32.33 / 7)) * (1 + 4 / 31)
for j = 1, 11 do acc = acc + (j * 69.71) % 97 / (j + 1) end
acc = acc + blend(23.51, 91, math.sin(23.51 / 7)) * (1 + 6 / 31)
q = "warm {math.floor(85.36)} {string.rep('=', 1 + 1)} gamma done"
acc = acc + blend(21.85, 24, math.sin(21.85 / 7)) * (1 + 2 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 39.12) end
bag = { id = 449, name = "warm-gamma", pos = { 68.64, 449.5, 137.28 }, ok = false }
m = { ["k7792"] = { nested = { deeper = { value = "fast", list = { 1, 2, 7792 } } } } }
m = { ["k874"] = { nested = { deeper = { value = "cold", list = { 1, 2, 874 } } } } }
for j = 1, 15 do acc = acc + (j * 2.04) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 31.64) end
if acc > 25.34 then acc = acc / 71 elseif acc < -25.34 then acc = -acc else acc = acc + 3986 end
bag = { id = 8297, name = "cold-delta", pos = { 71.12, 8297.5, 142.24 }, ok = false }
bag = { id = 2418, name = "red-beta", pos = { 15.51, 2418.5, 31.02 }, ok = true }
acc = acc + blend(40.61, 59, math.sin(40.61 / 7)) * (1 + 11 / 31)
for j = 1, 8 do acc = acc + (j * 32.95) % 97 / (j + 1) end
m = { ["k9748"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9748 } } } } }
m = { ["k2472"] = { nested = { deeper = { value = "blue", list = { 1, 2, 2472 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 64.19) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 16.84) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 90.43) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 77.98) end
m = { ["k5263"] = { nested = { deeper = { value = "blue", list = { 1, 2, 5263 } } } } }
acc = acc + blend(64.69, 76, math.sin(64.69 / 7)) * (1 + 9 / 31)
acc = acc + blend(4.81, 78, math.sin(4.81 / 7)) * (1 + 6 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 35.21) end
m = { ["k3910"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3910 } } } } }
for j = 1, 40 do acc = acc + (j * 49.68) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 28.75) end
if acc > 82.98 then acc = acc / 31 elseif acc < -82.98 then acc = -acc else acc = acc + 9286 end
bag = { id = 970, name = "cold-delta", pos = { 93.34, 970.5, 186.68 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 66.37) end
r = (36.71 > 12) and (acc ~= 0 and "green" or "delta") or blend(36.71, 3812, 0.25)
m = { ["k3764"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3764 } } } } }
acc = acc + blend(39.81, 51, math.sin(39.81 / 7)) * (1 + 4 / 31)
for j = 1, 40 do acc = acc + (j * 69.71) % 97 / (j + 1) end
for j = 1, 14 do acc = acc + (j * 56.69) % 97 / (j + 1) end
r = (59.44 > 5) and (acc ~= 0 and "warm" or "delta") or blend(59.44, 6955, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 1.38) end
if acc > 62.78 then acc = acc / 21 elseif acc < -62.78 then acc = -acc else acc = acc + 4470 end
q = "blue {math.floor(82.20)} {string.rep('=', 3 + 1)} gamma done"
for j = 1, 20 do acc = acc + (j * 13.92) % 97 / (j + 1) end
if acc > 64.49 then acc = acc / 33 elseif acc < -64.49 then acc = -acc else acc = acc + 8131 end
q = "fast {math.floor(3.18)} {string.rep('=', 2 + 1)} beta done"
if acc > 36.97 then acc = acc / 4 elseif acc < -36.97 then acc = -acc else acc = acc + 92 end
acc = acc + blend(66.00, 91, math.sin(66.00 / 7)) * (1 + 3 / 31)
if acc > 24.03 then acc = acc / 50 elseif acc < -24.03 then acc = -acc else acc = acc + 5834 end
m = { ["k13"] = { nested = { deeper = { value = "fast", list = { 1, 2, 13 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 56.61) end
acc = acc + blend(5.84, 46, math.sin(5.84 / 7)) * (1 + 5 / 31)
acc = acc + blend(70.96, 70, math.sin(70.96 / 7)) * (1 + 7 / 31)
q = "warm {math.floor(87.44)} {string.rep('=', 0 + 1)} beta done"
acc = acc + blend(76.20, 2, math.sin(76.20 / 7)) * (1 + 3 / 31)
acc = acc + blend(4.45, 59, math.sin(4.45 / 7)) * (1 + 0 / 31)
for j = 1, 35 do acc = acc + (j * 2.21) % 97 / (j + 1) end
m = { ["k7375"] = { nested = { deeper = { value = "cold", list = { 1, 2, 7375 } } } } }
if acc > 37.95 then acc = acc / 9 elseif acc < -37.95 then acc = -acc else acc = acc + 2945 end
bag = { id = 4129, name = "warm-beta", pos = { 35.09, 4129.5, 70.18 }, ok = false }
r = (79.52 > 44) and (acc ~= 0 and "blue" or "alpha") or blend(79.52, 7944, 0.25)
if acc > 7.83 then acc = acc / 71 elseif acc < -7.83 then acc = -acc else acc = acc + 6834 end
m = { ["k6284"] = { nested = { deeper = { value = "red", list = { 1, 2, 6284 } } } } }
acc = acc + blend(89.48, 53, math.sin(89.48 / 7)) * (1 + 9 / 31)
r = (88.78 > 20) and (acc ~= 0 and "red" or "beta") or blend(88.78, 3970, 0.25)
if acc > 1.99 then acc = acc / 9 elseif acc < -1.99 then acc = -acc else acc = acc + 809 end
r = (15.60 > 16) and (acc ~= 0 and "red" or "gamma") or blend(15.60, 2566, 0.25)
bag = { id = 6350, name = "warm-gamma", pos = { 46.22, 6350.5, 92.44 }, ok = true }
bag = { id = 892, name = "red-gamma", pos = { 76.38, 892.5, 152.76 }, ok = true }
for j = 1, 19 do acc = acc + (j * 40.88) % 97 / (j + 1) end
q = "blue {math.floor(56.58)} {string.rep('=', 0 + 1)} delta done"
if acc > 74.66 then acc = acc / 30 elseif acc < -74.66 then acc = -acc else acc = acc + 3233 end
m = { ["k3086"] = { nested = { deeper = { value = "warm", list = { 1, 2, 3086 } } } } }
r = (44.13 > 20) and (acc ~= 0 and "fast" or "alpha") or blend(44.13, 120, 0.25)
q = "fast {math.floor(20.21)} {string.rep('=', 0 + 1)} delta done"
m = { ["k9373"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9373 } } } } }
acc = acc + blend(70.08, 1, math.sin(70.08 / 7)) * (1 + 12 / 31)
for j = 1, 21 do acc = acc + (j * 3.70) % 97 / (j + 1) end
for j = 1, 31 do acc = acc + (j * 33.64) % 97 / (j + 1) end
q = "cold {math.floor(34.66)} {string.rep('=', 3 + 1)} gamma done"
if acc > 84.05 then acc = acc / 44 elseif acc < -84.05 then acc = -acc else acc = acc + 9121 end
for j = 1, 39 do acc = acc + (j * 49.14) % 97 / (j + 1) end
q = "warm {math.floor(34.76)} {string.rep('=', 5 + 1)} gamma done"
r = (17.44 > 40) and (acc ~= 0 and "cold" or "alpha") or blend(17.44, 2140, 0.25)
if acc > 79.21 then acc = acc / 27 elseif acc < -79.21 then acc = -acc else acc = acc + 3586 end
acc = acc + blend(9.63, 58, math.sin(9.63 / 7)) * (1 + 7 / 31)
bag = { id = 1489, name = "green-gamma", pos = { 13.74, 1489.5, 27.48 }, ok = false }
r = (3.72 > 31) and (acc ~= 0 and "warm" or "beta") or blend(3.72, 1681, 0.25)
m = { ["k8318"] = { nested = { deeper = { value = "cold", list = { 1, 2, 8318 } } } } }
acc = acc + blend(18.50, 31, math.sin(18.50 / 7)) * (1 + 5 / 31)
bag = { id = 5609, name = "cold-gamma", pos = { 87.02, 5609.5, 174.04 }, ok = false }
acc = acc + blend(54.08, 25, math.sin(54.08 / 7)) * (1 + 9 / 31)
acc = acc + blend(51.34, 92, math.sin(51.34 / 7)) * (1 + 9 / 31)
bag = { id = 4396, name = "red-delta", pos = { 30.18, 4396.5, 60.36 }, ok = true }
bag = { id = 4355, name = "warm-gamma", pos = { 42.98, 4355.5, 85.96 }, ok = false }
if acc > 70.39 then acc = acc / 42 elseif acc < -70.39 then acc = -acc else acc = acc + 8941 end
acc = acc + blend(52.86, 7, math.sin(52.86 / 7)) * (1 + 4 / 31)
if acc > 45.18 then acc = acc / 33 elseif acc < -45.18 then acc = -acc else acc = acc + 8487 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 23.14) end
q = "fast {math.floor(65.13)} {string.rep('=', 2 + 1)} delta done"
for j = 1, 31 do acc = acc + (j * 13.23) % 97 / (j + 1) end
r = (63.19 > 4) and (acc ~= 0 and "red" or "alpha") or blend(63.19, 1254, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 28.15) end
m = { ["k5225"] = { nested = { deeper = { value = "warm", list = { 1, 2, 5225 } } } } }
if acc > 47.65 then acc = acc / 85 elseif acc < -47.65 then acc = -acc else acc = acc + 2576 end
acc = acc + blend(9.86, 21, math.sin(9.86 / 7)) * (1 + 6 / 31)
bag = { id = 6611, name = "blue-gamma", pos = { 62.53, 6611.5, 125.06 }, ok = false }
bag = { id = 4483, name = "red-alpha", pos = { 0.95, 4483.5, 1.9 }, ok = false }
m = { ["k8551"] = { nested = { deeper = { value = "warm", list = { 1, 2, 8551 } } } } }
for j = 1, 14 do acc = acc + (j * 7.23) % 97 / (j + 1) end
m = { ["k9204"] = { nested = { deeper = { value = "cold", list = { 1, 2, 9204 } } } } }
m = { ["k8673"] = { nested = { deeper = { value = "red", list = { 1, 2, 8673 } } } } }
if acc > 91.09 then acc = acc / 19 elseif acc < -91.09 then acc = -acc else acc = acc + 1353 end
m = { ["k701"] = { nested = { deeper = { value = "red", list = { 1, 2, 701 } } } } }
acc = acc + blend(34.03, 21, math.sin(34.03 / 7)) * (1 + 6 / 31)
q = "cold {math.floor(49.09)} {string.rep('=', 0 + 1)} beta done"
r = (29.10 > 11) and (acc ~= 0 and "blue" or "beta") or blend(29.10, 9611, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 42.27) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 1.09) end
m = { ["k2146"] = { nested = { deeper = { value = "red", list = { 1, 2, 2146 } } } } }
for j = 1, 10 do acc = acc + (j * 12.26) % 97 / (j + 1) end
bag = { id = 7239, name = "fast-delta", pos = { 36.26, 7239.5, 72.52 }, ok = false }
r = (93.25 > 32) and (acc ~= 0 and "cold" or "beta") or blend(93.25, 3632, 0.25)
q = "blue {math.floor(50.92)} {string.rep('=', 1 + 1)} alpha done"
for j = 1, 6 do acc = acc + (j * 21.76) % 97 / (j + 1) end
bag = { id = 3004, name = "fast-alpha", pos = { 16.61, 3004.5, 33.22 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 96.63) end
bag = { id = 8860, name = "warm-beta", pos = { 14.34, 8860.5, 28.68 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 53.68) end
q = "blue {math.floor(52.24)} {string.rep('=', 1 + 1)} alpha done"
if acc > 21.23 then acc = acc / 45 elseif acc < -21.23 then acc = -acc else acc = acc + 5562 end
m = { ["k2911"] = { nested = { deeper = { value = "fast", list = { 1, 2, 2911 } } } } }
q = "cold {math.floor(0.65)} {string.rep('=', 1 + 1)} gamma done"
q = "blue {math.floor(40.02)} {string.rep('=', 0 + 1)} gamma done"
q = "cold {math.floor(10.88)} {string.rep('=', 0 + 1)} alpha done"
r = (16.20 > 14) and (acc ~= 0 and "green" or "alpha") or blend(16.20, 1164, 0.25)
bag = { id = 517, name = "warm-gamma", pos = { 37.18, 517.5, 74.36 }, ok = false }
acc = acc + blend(51.05, 87, math.sin(51.05 / 7)) * (1 + 12 / 31)
r = (4.31 > 17) and (acc ~= 0 and "green" or "delta") or blend(4.31, 2067, 0.25)
bag = { id = 4889, name = "blue-gamma", pos = { 43.61, 4889.5, 87.22 }, ok = false }
m = { ["k3302"] = { nested = { deeper = { value = "warm", list = { 1, 2, 3302 } } } } }
bag = { id = 6609, name = "cold-gamma", pos = { 83.17, 6609.5, 166.34 }, ok = false }
acc = acc + blend(86.64, 45, math.sin(86.64 / 7)) * (1 + 5 / 31)
m = { ["k2427"] = { nested = { deeper = { value = "warm", list = { 1, 2, 2427 } } } } }
if acc > 60.90 then acc = acc / 69 elseif acc < -60.90 then acc = -acc else acc = acc + 9858 end
m = { ["k1903"] = { nested = { deeper = { value = "cold", list = { 1, 2, 1903 } } } } }
q = "green {math.floor(95.26)} {string.rep('=', 5 + 1)} beta done"
if acc > 23.21 then acc = acc / 1 elseif acc < -23.21 then acc = -acc else acc = acc + 6764 end
bag = { id = 3278, name = "red-alpha", pos = { 31.09, 3278.5, 62.18 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 83.28) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 69.15) end
for j = 1, 24 do acc = acc + (j * 50.23) % 97 / (j + 1) end
acc = acc + blend(27.69, 89, math.sin(27.69 / 7)) * (1 + 8 / 31)
bag = { id = 2001, name = "warm-alpha", pos = { 24.05, 2001.5, 48.1 }, ok = false }
if acc > 55.69 then acc = acc / 70 elseif acc < -55.69 then acc = -acc else acc = acc + 7634 end
if acc > 60.93 then acc = acc / 43 elseif acc < -60.93 then acc = -acc else acc = acc + 9298 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 0.06) end
acc = acc + blend(56.05, 74, math.sin(56.05 / 7)) * (1 + 9 / 31)
r = (14.13 > 24) and (acc ~= 0 and "cold" or "delta") or blend(14.13, 5524, 0.25)
if acc > 11.54 then acc = acc / 24 elseif acc < -11.54 then acc = -acc else acc = acc + 3672 end
acc = acc + blend(4.62, 53, math.sin(4.62 / 7)) * (1 + 1 / 31)
bag = { id = 1204, name = "red-beta", pos = { 93.55, 1204.5, 187.1 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 79.63) end
r = (85.00 > 37) and (acc ~= 0 and "green" or "delta") or blend(85.00, 8937, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 59.27) end
if acc > 13.81 then acc = acc / 38 elseif acc < -13.81 then acc = -acc else acc = acc + 2084 end
if acc > 77.05 then acc = acc / 88 elseif acc < -77.05 then acc = -acc else acc = acc + 7741 end
q = "warm {math.floor(84.64)} {string.rep('=', 1 + 1)} alpha done"
m = { ["k9043"] = { nested = { deeper = { value = "warm", list = { 1, 2, 9043 } } } } }
if acc > 22.84 then acc = acc / 23 elseif acc < -22.84 then acc = -acc else acc = acc + 2247 end
if acc > 98.48 then acc = acc / 64 elseif acc < -98.48 then acc = -acc else acc = acc + 9497 end
acc = acc + blend(58.32, 88, math.sin(58.32 / 7)) * (1 + 1 / 31)
r = (63.12 > 16) and (acc ~= 0 and "green" or "beta") or blend(63.12, 1816, 0.25)
q = "green {math.floor(89.97)} {string.rep('=', 1 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 83.63) end
bag = { id = 8900, name = "red-delta", pos = { 86.06, 8900.5, 172.12 }, ok = true }
bag = { id = 876, name = "red-gamma", pos = { 54.52, 876.5, 109.04 }, ok = true }
if acc > 46.63 then acc = acc / 20 elseif acc < -46.63 then acc = -acc else acc = acc + 7584 end
bag = { id = 5335, name = "blue-alpha", pos = { 67.09, 5335.5, 134.18 }, ok = false }
r = (88.80 > 32) and (acc ~= 0 and "blue" or "alpha") or blend(88.80, 8832, 0.25)
acc = acc + blend(65.94, 64, math.sin(65.94 / 7)) * (1 + 5 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 69.61) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 81.58) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 41.14) end
for j = 1, 12 do acc = acc + (j * 56.47) % 97 / (j + 1) end
r = (94.02 > 18) and (acc ~= 0 and "green" or "gamma") or blend(94.02, 5818, 0.25)
r = (31.16 > 45) and (acc ~= 0 and "green" or "gamma") or blend(31.16, 3395, 0.25)
bag = { id = 9000, name = "blue-gamma", pos = { 95.99, 9000.5, 191.98 }, ok = true }
r = (15.85 > 6) and (acc ~= 0 and "blue" or "delta") or blend(15.85, 5456, 0.25)
bag = { id = 9067, name = "warm-gamma", pos = { 26.47, 9067.5, 52.94 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 73.62) end
acc = acc + blend(53.74, 56, math.sin(53.74 / 7)) * (1 + 7 / 31)
m = { ["k460"] = { nested = { deeper = { value = "warm", list = { 1, 2, 460 } } } } }
m = { ["k6245"] = { nested = { deeper = { value = "green", list = { 1, 2, 6245 } } } } }
m = { ["k5855"] = { nested = { deeper = { value = "blue", list = { 1, 2, 5855 } } } } }
q = "green {math.floor(33.93)} {string.rep('=', 4 + 1)} beta done"
m = { ["k2136"] = { nested = { deeper = { value = "blue", list = { 1, 2, 2136 } } } } }
if acc > 66.08 then acc = acc / 65 elseif acc < -66.08 then acc = -acc else acc = acc + 7362 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 31.74) end
if acc > 32.20 then acc = acc / 39 elseif acc < -32.20 then acc = -acc else acc = acc + 2975 end
if acc > 53.84 then acc = acc / 15 elseif acc < -53.84 then acc = -acc else acc = acc + 4197 end
if acc > 44.44 then acc = acc / 23 elseif acc < -44.44 then acc = -acc else acc = acc + 1535 end
acc = acc + blend(55.80, 5, math.sin(55.80 / 7)) * (1 + 0 / 31)
acc = acc + blend(70.22, 56, math.sin(70.22 / 7)) * (1 + 4 / 31)
if acc > 1.04 then acc = acc / 82 elseif acc < -1.04 then acc = -acc else acc = acc + 5154 end
q = "warm {math.floor(75.16)} {string.rep('=', 6 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 92.73) end
if acc > 71.59 then acc = acc / 9 elseif acc < -71.59 then acc = -acc else acc = acc + 275 end
bag = { id = 9546, name = "cold-alpha", pos = { 78.31, 9546.5, 156.62 }, ok = true }
acc = acc + blend(94.22, 89, math.sin(94.22 / 7)) * (1 + 0 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 5.98) end
if acc > 21.40 then acc = acc / 13 elseif acc < -21.40 then acc = -acc else acc = acc + 2682 end
for j = 1, 26 do acc = acc + (j * 84.75) % 97 / (j + 1) end
bag = { id = 1578, name = "fast-gamma", pos = { 77.32, 1578.5, 154.64 }, ok = true }
bag = { id = 6044, name = "red-alpha", pos = { 34.88, 6044.5, 69.76 }, ok = true }
if acc > 92.21 then acc = acc / 88 elseif acc < -92.21 then acc = -acc else acc = acc + 2401 end
acc = acc + blend(47.53, 90, math.sin(47.53 / 7)) * (1 + 3 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 22.43) end
m = { ["k1846"] = { nested = { deeper = { value = "green", list = { 1, 2, 1846 } } } } }
if acc > 44.54 then acc = acc / 52 elseif acc < -44.54 then acc = -acc else acc = acc + 6993 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 19.04) end
m = { ["k3829"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3829 } } } } }
for j = 1, 11 do acc = acc + (j * 36.04) % 97 / (j + 1) end
if acc > 34.26 then acc = acc / 33 elseif acc < -34.26 then acc = -acc else acc = acc + 4660 end
if acc > 36.99 then acc = acc / 28 elseif acc < -36.99 then acc = -acc else acc = acc + 4388 end
r = (30.72 > 45) and (acc ~= 0 and "fast" or "gamma") or blend(30.72, 7545, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 95.34) end
if acc > 85.10 then acc = acc / 71 elseif acc < -85.10 then acc = -acc else acc = acc + 604 end
for j = 1, 40 do acc = acc + (j * 24.11) % 97 / (j + 1) end
r = (17.38 > 9) and (acc ~= 0 and "green" or "alpha") or blend(17.38, 7109, 0.25)
if acc > 12.86 then acc = acc / 63 elseif acc < -12.86 then acc = -acc else acc = acc + 9496 end
acc = acc + blend(8.84, 83, math.sin(8.84 / 7)) * (1 + 3 / 31)
if acc > 28.86 then acc = acc / 51 elseif acc < -28.86 then acc = -acc else acc = acc + 5924 end
r = (99.10 > 24) and (acc ~= 0 and "cold" or "delta") or blend(99.10, 5074, 0.25)
bag = { id = 8861, name = "fast-delta", pos = { 44.83, 8861.5, 89.66 }, ok = false }
m = { ["k1033"] = { nested = { deeper = { value = "green", list = { 1, 2, 1033 } } } } }
r = (33.93 > 35) and (acc ~= 0 and "warm" or "beta") or blend(33.93, 785, 0.25)
bag = { id = 5696, name = "fast-delta", pos = { 75.32, 5696.5, 150.64 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 41.30) end
r = (5.20 > 9) and (acc ~= 0 and "cold" or "gamma") or blend(5.20, 759, 0.25)
r = (83.95 > 19) and (acc ~= 0 and "fast" or "beta") or blend(83.95, 9919, 0.25)
for j = 1, 39 do acc = acc + (j * 88.50) % 97 / (j + 1) end
for j = 1, 22 do acc = acc + (j * 76.19) % 97 / (j + 1) end
if acc > 23.12 then acc = acc / 3 elseif acc < -23.12 then acc = -acc else acc = acc + 5431 end
m = { ["k5966"] = { nested = { deeper = { value = "warm", list = { 1, 2, 5966 } } } } }
bag = { id = 940, name = "blue-gamma", pos = { 21.50, 940.5, 43 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 58.37) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 54.19) end
r = (45.11 > 37) and (acc ~= 0 and "fast" or "delta") or blend(45.11, 1087, 0.25)
q = "red {math.floor(49.56)} {string.rep('=', 5 + 1)} alpha done"
for j = 1, 40 do acc = acc + (j * 37.70) % 97 / (j + 1) end
if acc > 71.58 then acc = acc / 21 elseif acc < -71.58 then acc = -acc else acc = acc + 2156 end
for j = 1, 23 do acc = acc + (j * 79.85) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 17.58) end
for j = 1, 16 do acc = acc + (j * 45.17) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 47.99) end
m = { ["k7883"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7883 } } } } }
m = { ["k808"] = { nested = { deeper = { value = "warm", list = { 1, 2, 808 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 31.20) end
for j = 1, 3 do acc = acc + (j * 28.92) % 97 / (j + 1) end
r = (23.14 > 1) and (acc ~= 0 and "warm" or "delta") or blend(23.14, 1601, 0.25)
if acc > 27.35 then acc = acc / 33 elseif acc < -27.35 then acc = -acc else acc = acc + 8220 end
r = (91.13 > 43) and (acc ~= 0 and "blue" or "gamma") or blend(91.13, 3343, 0.25)
m = { ["k5160"] = { nested = { deeper = { value = "red", list = { 1, 2, 5160 } } } } }
acc = acc + blend(5.77, 29, math.sin(5.77 / 7)) * (1 + 7 / 31)
acc = acc + blend(34.16, 80, math.sin(34.16 / 7)) * (1 + 9 / 31)
bag = { id = 7973, name = "green-beta", pos = { 91.96, 7973.5, 183.92 }, ok = false }
if acc > 69.06 then acc = acc / 51 elseif acc < -69.06 then acc = -acc else acc = acc + 3432 end
bag = { id = 2580, name = "cold-delta", pos = { 67.18, 2580.5, 134.36 }, ok = true }
if acc > 16.10 then acc = acc / 62 elseif acc < -16.10 then acc = -acc else acc = acc + 5846 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 39.50) end
for j = 1, 34 do acc = acc + (j * 57.55) % 97 / (j + 1) end
bag = { id = 4952, name = "red-delta", pos = { 68.41, 4952.5, 136.82 }, ok = true }
r = (74.09 > 39) and (acc ~= 0 and "green" or "beta") or blend(74.09, 739, 0.25)
bag = { id = 3286, name = "blue-beta", pos = { 49.81, 3286.5, 99.62 }, ok = true }
acc = acc + blend(22.64, 1, math.sin(22.64 / 7)) * (1 + 8 / 31)
m = { ["k3072"] = { nested = { deeper = { value = "cold", list = { 1, 2, 3072 } } } } }
q = "fast {math.floor(56.31)} {string.rep('=', 3 + 1)} beta done"
acc = acc + blend(14.16, 82, math.sin(14.16 / 7)) * (1 + 11 / 31)
r = (29.38 > 15) and (acc ~= 0 and "warm" or "delta") or blend(29.38, 7315, 0.25)
acc = acc + blend(88.78, 18, math.sin(88.78 / 7)) * (1 + 10 / 31)
r = (79.77 > 35) and (acc ~= 0 and "cold" or "delta") or blend(79.77, 2785, 0.25)
r = (71.52 > 40) and (acc ~= 0 and "cold" or "alpha") or blend(71.52, 6340, 0.25)
r = (18.67 > 48) and (acc ~= 0 and "warm" or "gamma") or blend(18.67, 2898, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 88.84) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 39.56) end
for j = 1, 32 do acc = acc + (j * 27.57) % 97 / (j + 1) end
if acc > 91.38 then acc = acc / 69 elseif acc < -91.38 then acc = -acc else acc = acc + 5586 end
bag = { id = 855, name = "red-alpha", pos = { 63.36, 855.5, 126.72 }, ok = false }
q = "fast {math.floor(0.93)} {string.rep('=', 6 + 1)} alpha done"
if acc > 84.44 then acc = acc / 22 elseif acc < -84.44 then acc = -acc else acc = acc + 377 end
if acc > 82.34 then acc = acc / 57 elseif acc < -82.34 then acc = -acc else acc = acc + 3438 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 78.95) end
m = { ["k8372"] = { nested = { deeper = { value = "red", list = { 1, 2, 8372 } } } } }
acc = acc + blend(70.06, 73, math.sin(70.06 / 7)) * (1 + 3 / 31)
bag = { id = 435, name = "cold-alpha", pos = { 7.06, 435.5, 14.12 }, ok = false }
for j = 1, 25 do acc = acc + (j * 46.80) % 97 / (j + 1) end
r = (61.00 > 41) and (acc ~= 0 and "fast" or "beta") or blend(61.00, 1791, 0.25)
q = "fast {math.floor(81.38)} {string.rep('=', 5 + 1)} gamma done"
acc = acc + blend(89.13, 18, math.sin(89.13 / 7)) * (1 + 3 / 31)
if acc > 26.49 then acc = acc / 21 elseif acc < -26.49 then acc = -acc else acc = acc + 821 end
for j = 1, 36 do acc = acc + (j * 12.21) % 97 / (j + 1) end
bag = { id = 5946, name = "green-gamma", pos = { 80.33, 5946.5, 160.66 }, ok = true }
acc = acc + blend(0.86, 39, math.sin(0.86 / 7)) * (1 + 8 / 31)
acc = acc + blend(73.99, 4, math.sin(73.99 / 7)) * (1 + 3 / 31)
for j = 1, 17 do acc = acc + (j * 9.63) % 97 / (j + 1) end
q = "green {math.floor(6.74)} {string.rep('=', 1 + 1)} gamma done"
if acc > 75.20 then acc = acc / 46 elseif acc < -75.20 then acc = -acc else acc = acc + 8767 end
q = "red {math.floor(10.47)} {string.rep('=', 5 + 1)} gamma done"
if acc > 74.98 then acc = acc / 20 elseif acc < -74.98 then acc = -acc else acc = acc + 3045 end
for j = 1, 7 do acc = acc + (j * 97.20) % 97 / (j + 1) end
q = "green {math.floor(71.65)} {string.rep('=', 2 + 1)} gamma done"
for j = 1, 31 do acc = acc + (j * 13.42) % 97 / (j + 1) end
r = (81.48 > 13) and (acc ~= 0 and "warm" or "alpha") or blend(81.48, 413, 0.25)
bag = { id = 32, name = "fast-alpha", pos = { 23.91, 32.5, 47.82 }, ok = true }
m = { ["k7159"] = { nested = { deeper = { value = "red", list = { 1, 2, 7159 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 56.84) end
r = (95.11 > 37) and (acc ~= 0 and "red" or "beta") or blend(95.11, 287, 0.25)
for j = 1, 20 do acc = acc + (j * 92.64) % 97 / (j + 1) end
for j = 1, 40 do acc = acc + (j * 62.44) % 97 / (j + 1) end
r = (51.72 > 17) and (acc ~= 0 and "cold" or "beta") or blend(51.72, 2117, 0.25)
if acc > 61.27 then acc = acc / 61 elseif acc < -61.27 then acc = -acc else acc = acc + 7447 end
acc = acc + blend(63.86, 48, math.sin(63.86 / 7)) * (1 + 6 / 31)
bag = { id = 5419, name = "warm-beta", pos = { 88.87, 5419.5, 177.74 }, ok = false }
acc = acc + blend(63.74, 31, math.sin(63.74 / 7)) * (1 + 12 / 31)
acc = acc + blend(57.14, 9, math.sin(57.14 / 7)) * (1 + 4 / 31)
r = (86.01 > 29) and (acc ~= 0 and "fast" or "beta") or blend(86.01, 9279, 0.25)
for j = 1, 35 do acc = acc + (j * 86.37) % 97 / (j + 1) end
r = (9.47 > 23) and (acc ~= 0 and "red" or "alpha") or blend(9.47, 8623, 0.25)
acc = acc + blend(55.08, 14, math.sin(55.08 / 7)) * (1 + 6 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 55.08) end
q = "blue {math.floor(72.66)} {string.rep('=', 4 + 1)} gamma done"
for j = 1, 32 do acc = acc + (j * 57.86) % 97 / (j + 1) end
r = (67.41 > 41) and (acc ~= 0 and "cold" or "beta") or blend(67.41, 4141, 0.25)
for j = 1, 21 do acc = acc + (j * 39.56) % 97 / (j + 1) end
r = (8.93 > 40) and (acc ~= 0 and "blue" or "gamma") or blend(8.93, 9440, 0.25)
for j = 1, 24 do acc = acc + (j * 88.31) % 97 / (j + 1) end
m = { ["k418"] = { nested = { deeper = { value = "fast", list = { 1, 2, 418 } } } } }
for j = 1, 36 do acc = acc + (j * 34.68) % 97 / (j + 1) end
q = "warm {math.floor(62.60)} {string.rep('=', 1 + 1)} alpha done"
q = "green {math.floor(82.21)} {string.rep('=', 0 + 1)} alpha done"
if acc > 9.80 then acc = acc / 52 elseif acc < -9.80 then acc = -acc else acc = acc + 7705 end
if acc > 35.22 then acc = acc / 15 elseif acc < -35.22 then acc = -acc else acc = acc + 6600 end
bag = { id = 3926, name = "green-alpha", pos = { 86.43, 3926.5, 172.86 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 49.22) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 87.18) end
r = (40.76 > 12) and (acc ~= 0 and "green" or "gamma") or blend(40.76, 1712, 0.25)
q = "fast {math.floor(22.10)} {string.rep('=', 4 + 1)} beta done"
q = "cold {math.floor(15.03)} {string.rep('=', 3 + 1)} delta done"
m = { ["k9455"] = { nested = { deeper = { value = "red", list = { 1, 2, 9455 } } } } }
if acc > 32.57 then acc = acc / 18 elseif acc < -32.57 then acc = -acc else acc = acc + 1352 end
q = "cold {math.floor(60.74)} {string.rep('=', 1 + 1)} alpha done"
if acc > 22.11 then acc = acc / 45 elseif acc < -22.11 then acc = -acc else acc = acc + 9389 end
r = (73.42 > 39) and (acc ~= 0 and "fast" or "beta") or blend(73.42, 3639, 0.25)
if acc > 86.08 then acc = acc / 88 elseif acc < -86.08 then acc = -acc else acc = acc + 6317 end
bag = { id = 6906, name = "red-delta", pos = { 18.59, 6906.5, 37.18 }, ok = true }
m = { ["k8691"] = { nested = { deeper = { value = "warm", list = { 1, 2, 8691 } } } } }
q = "red {math.floor(91.53)} {string.rep('=', 1 + 1)} beta done"
for j = 1, 21 do acc = acc + (j * 8.12) % 97 / (j + 1) end
acc = acc + blend(10.25, 94, math.sin(10.25 / 7)) * (1 + 9 / 31)
r = (46.64 > 49) and (acc ~= 0 and "warm" or "beta") or blend(46.64, 4249, 0.25)
for j = 1, 9 do acc = acc + (j * 55.90) % 97 / (j + 1) end
m = { ["k4399"] = { nested = { deeper = { value = "red", list = { 1, 2, 4399 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 27.62) end
bag = { id = 9783, name = "red-delta", pos = { 27.61, 9783.5, 55.22 }, ok = false }
if acc > 74.56 then acc = acc / 39 elseif acc < -74.56 then acc = -acc else acc = acc + 6624 end
m = { ["k8417"] = { nested = { deeper = { value = "fast", list = { 1, 2, 8417 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 57.12) end
bag = { id = 1144, name = "fast-delta", pos = { 44.29, 1144.5, 88.58 }, ok = true }
q = "blue {math.floor(35.50)} {string.rep('=', 6 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 29.16) end
q = "green {math.floor(93.19)} {string.rep('=', 4 + 1)} alpha done"
bag = { id = 8096, name = "warm-alpha", pos = { 48.03, 8096.5, 96.06 }, ok = true }
r = (32.11 > 18) and (acc ~= 0 and "warm" or "gamma") or blend(32.11, 4018, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 88.79) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 30.73) end
r = (21.67 > 38) and (acc ~= 0 and "fast" or "beta") or blend(21.67, 2688, 0.25)
bag = { id = 3323, name = "warm-gamma", pos = { 93.40, 3323.5, 186.8 }, ok = false }
bag = { id = 8097, name = "cold-alpha", pos = { 25.82, 8097.5, 51.64 }, ok = false }
bag = { id = 5659, name = "fast-delta", pos = { 44.37, 5659.5, 88.74 }, ok = false }
q = "green {math.floor(14.88)} {string.rep('=', 1 + 1)} gamma done"
if acc > 89.25 then acc = acc / 66 elseif acc < -89.25 then acc = -acc else acc = acc + 1400 end
for j = 1, 22 do acc = acc + (j * 22.37) % 97 / (j + 1) end
acc = acc + blend(87.42, 65, math.sin(87.42 / 7)) * (1 + 11 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 5.11) end
r = (6.18 > 22) and (acc ~= 0 and "blue" or "alpha") or blend(6.18, 7422, 0.25)
for j = 1, 26 do acc = acc + (j * 75.52) % 97 / (j + 1) end
q = "red {math.floor(68.15)} {string.rep('=', 5 + 1)} delta done"
q = "red {math.floor(60.50)} {string.rep('=', 3 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 13.75) end
acc = acc + blend(38.36, 50, math.sin(38.36 / 7)) * (1 + 8 / 31)
r = (86.47 > 30) and (acc ~= 0 and "cold" or "gamma") or blend(86.47, 9130, 0.25)
q = "fast {math.floor(6.45)} {string.rep('=', 6 + 1)} delta done"
acc = acc + blend(67.05, 77, math.sin(67.05 / 7)) * (1 + 4 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 12.45) end
q = "warm {math.floor(38.54)} {string.rep('=', 5 + 1)} beta done"
m = { ["k3097"] = { nested = { deeper = { value = "green", list = { 1, 2, 3097 } } } } }
r = (15.53 > 25) and (acc ~= 0 and "fast" or "gamma") or blend(15.53, 525, 0.25)
for j = 1, 19 do acc = acc + (j * 30.50) % 97 / (j + 1) end
r = (22.34 > 21) and (acc ~= 0 and "blue" or "gamma") or blend(22.34, 9371, 0.25)
r = (20.16 > 37) and (acc ~= 0 and "red" or "delta") or blend(20.16, 2487, 0.25)
q = "cold {math.floor(13.45)} {string.rep('=', 6 + 1)} alpha done"
for j = 1, 34 do acc = acc + (j * 93.49) % 97 / (j + 1) end
q = "cold {math.floor(1.90)} {string.rep('=', 0 + 1)} gamma done"
r = (80.89 > 36) and (acc ~= 0 and "green" or "delta") or blend(80.89, 8736, 0.25)
q = "cold {math.floor(52.34)} {string.rep('=', 6 + 1)} gamma done"
acc = acc + blend(51.27, 0, math.sin(51.27 / 7)) * (1 + 7 / 31)
acc = acc + blend(13.09, 7, math.sin(13.09 / 7)) * (1 + 4 / 31)
acc = acc + blend(51.59, 32, math.sin(51.59 / 7)) * (1 + 8 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 60.76) end
for j = 1, 19 do acc = acc + (j * 40.60) % 97 / (j + 1) end
bag = { id = 3436, name = "red-gamma", pos = { 18.59, 3436.5, 37.18 }, ok = true }
if acc > 28.18 then acc = acc / 52 elseif acc < -28.18 then acc = -acc else acc = acc + 9040 end
r = (41.62 > 0) and (acc ~= 0 and "green" or "beta") or blend(41.62, 7350, 0.25)
r = (19.87 > 24) and (acc ~= 0 and "red" or "gamma") or blend(19.87, 4074, 0.25)
acc = acc + blend(45.17, 19, math.sin(45.17 / 7)) * (1 + 9 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 76.61) end
if acc > 42.42 then acc = acc / 37 elseif acc < -42.42 then acc = -acc else acc = acc + 5910 end
for j = 1, 6 do acc = acc + (j * 64.42) % 97 / (j + 1) end
r = (60.76 > 19) and (acc ~= 0 and "green" or "alpha") or blend(60.76, 2969, 0.25)
acc = acc + blend(68.40, 96, math.sin(68.40 / 7)) * (1 + 10 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 78.73) end
acc = acc + blend(11.51, 10, math.sin(11.51 / 7)) * (1 + 8 / 31)
acc = acc + blend(74.37, 5, math.sin(74.37 / 7)) * (1 + 5 / 31)
m = { ["k3602"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3602 } } } } }
for j = 1, 1 do acc = acc + (j * 56.17) % 97 / (j + 1) end
if acc > 69.18 then acc = acc / 87 elseif acc < -69.18 then acc = -acc else acc = acc + 5515 end
m = { ["k7890"] = { nested = { deeper = { value = "fast", list = { 1, 2, 7890 } } } } }
q = "cold {math.floor(49.12)} {string.rep('=', 6 + 1)} alpha done"
if acc > 61.30 then acc = acc / 79 elseif acc < -61.30 then acc = -acc else acc = acc + 7465 end
r = (27.85 > 46) and (acc ~= 0 and "fast" or "alpha") or blend(27.85, 796, 0.25)
for j = 1, 1 do acc = acc + (j * 83.06) % 97 / (j + 1) end
bag = { id = 6858, name = "fast-beta", pos = { 39.47, 6858.5, 78.94 }, ok = true }
for j = 1, 25 do acc = acc + (j * 16.60) % 97 / (j + 1) end
r = (38.84 > 17) and (acc ~= 0 and "blue" or "delta") or blend(38.84, 4667, 0.25)
q = "green {math.floor(50.22)} {string.rep('=', 2 + 1)} alpha done"
q = "green {math.floor(24.06)} {string.rep('=', 2 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 19.95) end
bag = { id = 2346, name = "red-gamma", pos = { 48.85, 2346.5, 97.7 }, ok = true }
bag = { id = 3197, name = "fast-delta", pos = { 93.63, 3197.5, 187.26 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 55.10) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 13.93) end
r = (4.46 > 22) and (acc ~= 0 and "cold" or "alpha") or blend(4.46, 7622, 0.25)
if acc > 69.12 then acc = acc / 29 elseif acc < -69.12 then acc = -acc else acc = acc + 1808 end
r = (47.04 > 2) and (acc ~= 0 and "blue" or "beta") or blend(47.04, 5102, 0.25)
bag = { id = 4320, name = "blue-gamma", pos = { 55.17, 4320.5, 110.34 }, ok = true }
m = { ["k1453"] = { nested = { deeper = { value = "fast", list = { 1, 2, 1453 } } } } }
q = "fast {math.floor(38.25)} {string.rep('=', 6 + 1)} delta done"
r = (57.81 > 29) and (acc ~= 0 and "blue" or "beta") or blend(57.81, 8079, 0.25)
for j = 1, 15 do acc = acc + (j * 79.09) % 97 / (j + 1) end
r = (92.83 > 41) and (acc ~= 0 and "green" or "alpha") or blend(92.83, 641, 0.25)
r = (46.18 > 12) and (acc ~= 0 and "cold" or "gamma") or blend(46.18, 1162, 0.25)
m = { ["k8459"] = { nested = { deeper = { value = "red", list = { 1, 2, 8459 } } } } }
q = "warm {math.floor(80.48)} {string.rep('=', 2 + 1)} gamma done"
q = "cold {math.floor(95.15)} {string.rep('=', 2 + 1)} beta done"
q = "red {math.floor(84.86)} {string.rep('=', 5 + 1)} delta done"
m = { ["k8005"] = { nested = { deeper = { value = "green", list = { 1, 2, 8005 } } } } }
if acc > 54.95 then acc = acc / 6 elseif acc < -54.95 then acc = -acc else acc = acc + 7926 end
acc = acc + blend(56.34, 61, math.sin(56.34 / 7)) * (1 + 7 / 31)
r = (49.22 > 5) and (acc ~= 0 and "green" or "alpha") or blend(49.22, 9155, 0.25)
q = "cold {math.floor(35.07)} {string.rep('=', 1 + 1)} alpha done"
m = { ["k4850"] = { nested = { deeper = { value = "cold", list = { 1, 2, 4850 } } } } }
q = "cold {math.floor(74.91)} {string.rep('=', 2 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 19.33) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 71.07) end
q = "green {math.floor(73.91)} {string.rep('=', 5 + 1)} gamma done"
m = { ["k1325"] = { nested = { deeper = { value = "green", list = { 1, 2, 1325 } } } } }
for j = 1, 34 do acc = acc + (j * 26.20) % 97 / (j + 1) end
bag = { id = 144, name = "blue-alpha", pos = { 91.13, 144.5, 182.26 }, ok = true }
q = "red {math.floor(23.48)} {string.rep('=', 2 + 1)} beta done"
bag = { id = 6670, name = "blue-gamma", pos = { 40.54, 6670.5, 81.08 }, ok = true }
q = "green {math.floor(84.04)} {string.rep('=', 1 + 1)} alpha done"
bag = { id = 7210, name = "green-alpha", pos = { 43.65, 7210.5, 87.3 }, ok = true }
r = (41.64 > 0) and (acc ~= 0 and "green" or "delta") or blend(41.64, 9300, 0.25)
acc = acc + blend(94.57, 79, math.sin(94.57 / 7)) * (1 + 2 / 31)
bag = { id = 6371, name = "cold-delta", pos = { 96.91, 6371.5, 193.82 }, ok = false }
for j = 1, 27 do acc = acc + (j * 83.86) % 97 / (j + 1) end
q = "fast {math.floor(9.43)} {string.rep('=', 1 + 1)} gamma done"
m = { ["k3438"] = { nested = { deeper = { value = "warm", list = { 1, 2, 3438 } } } } }
bag = { id = 2313, name = "green-gamma", pos = { 72.49, 2313.5, 144.98 }, ok = false }
q = "warm {math.floor(15.24)} {string.rep('=', 0 + 1)} gamma done"
q = "fast {math.floor(27.18)} {string.rep('=', 0 + 1)} delta done"
if acc > 8.62 then acc = acc / 60 elseif acc < -8.62 then acc = -acc else acc = acc + 6467 end
m = { ["k6090"] = { nested = { deeper = { value = "blue", list = { 1, 2, 6090 } } } } }
for j = 1, 17 do acc = acc + (j * 70.38) % 97 / (j + 1) end
r = (52.07 > 43) and (acc ~= 0 and "cold" or "beta") or blend(52.07, 7493, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 89.70) end
q = "warm {math.floor(80.38)} {string.rep('=', 3 + 1)} beta done"
m = { ["k9189"] = { nested = { deeper = { value = "red", list = { 1, 2, 9189 } } } } }
r = (59.17 > 47) and (acc ~= 0 and "warm" or "beta") or blend(59.17, 3497, 0.25)
for j = 1, 31 do acc = acc + (j * 5.01) % 97 / (j + 1) end
m = { ["k2986"] = { nested = { deeper = { value = "fast", list = { 1, 2, 2986 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 27.82) end
bag = { id = 5864, name = "blue-beta", pos = { 96.36, 5864.5, 192.72 }, ok = true }
q = "blue {math.floor(61.54)} {string.rep('=', 1 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 12.76) end
if acc > 29.15 then acc = acc / 87 elseif acc < -29.15 then acc = -acc else acc = acc + 9164 end
r = (28.42 > 44) and (acc ~= 0 and "cold" or "delta") or blend(28.42, 2144, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 56.21) end
q = "blue {math.floor(44.45)} {string.rep('=', 6 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 66.00) end
if acc > 90.54 then acc = acc / 31 elseif acc < -90.54 then acc = -acc else acc = acc + 5726 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 18.12) end
for j = 1, 33 do acc = acc + (j * 47.56) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 23.97) end
q = "red {math.floor(96.69)} {string.rep('=', 3 + 1)} gamma done"
r = (45.88 > 2) and (acc ~= 0 and "green" or "alpha") or blend(45.88, 5452, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 79.46) end
r = (71.38 > 26) and (acc ~= 0 and "blue" or "gamma") or blend(71.38, 2876, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 73.03) end
acc = acc + blend(79.01, 46, math.sin(79.01 / 7)) * (1 + 7 / 31)
r = (55.28 > 3) and (acc ~= 0 and "warm" or "beta") or blend(55.28, 9603, 0.25)
for j = 1, 36 do acc = acc + (j * 60.72) % 97 / (j + 1) end
bag = { id = 4921, name = "fast-beta", pos = { 11.75, 4921.5, 23.5 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 67.46) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 27.35) end
r = (82.27 > 29) and (acc ~= 0 and "fast" or "delta") or blend(82.27, 6229, 0.25)
r = (65.63 > 14) and (acc ~= 0 and "green" or "gamma") or blend(65.63, 1514, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 28.36) end
if acc > 74.98 then acc = acc / 44 elseif acc < -74.98 then acc = -acc else acc = acc + 5027 end
m = { ["k2256"] = { nested = { deeper = { value = "warm", list = { 1, 2, 2256 } } } } }
if acc > 28.64 then acc = acc / 20 elseif acc < -28.64 then acc = -acc else acc = acc + 9542 end
if acc > 4.05 then acc = acc / 53 elseif acc < -4.05 then acc = -acc else acc = acc + 2989 end
acc = acc + blend(12.21, 29, math.sin(12.21 / 7)) * (1 + 7 / 31)
acc = acc + blend(95.24, 79, math.sin(95.24 / 7)) * (1 + 7 / 31)
for j = 1, 33 do acc = acc + (j * 72.88) % 97 / (j + 1) end
r = (60.40 > 43) and (acc ~= 0 and "cold" or "alpha") or blend(60.40, 4993, 0.25)
if acc > 46.51 then acc = acc / 54 elseif acc < -46.51 then acc = -acc else acc = acc + 5215 end
for j = 1, 4 do acc = acc + (j * 73.19) % 97 / (j + 1) end
q = "red {math.floor(37.21)} {string.rep('=', 4 + 1)} beta done"
bag = { id = 7493, name = "green-gamma", pos = { 89.59, 7493.5, 179.18 }, ok = false }
acc = acc + blend(46.14, 71, math.sin(46.14 / 7)) * (1 + 8 / 31)
for j = 1, 26 do acc = acc + (j * 2.87) % 97 / (j + 1) end
m = { ["k9590"] = { nested = { deeper = { value = "red", list = { 1, 2, 9590 } } } } }
bag = { id = 2311, name = "cold-beta", pos = { 6.24, 2311.5, 12.48 }, ok = false }
m = { ["k2617"] = { nested = { deeper = { value = "blue", list = { 1, 2, 2617 } } } } }
r = (98.14 > 34) and (acc ~= 0 and "warm" or "gamma") or blend(98.14, 1534, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 95.93) end
acc = acc + blend(51.20, 55, math.sin(51.20 / 7)) * (1 + 2 / 31)
acc = acc + blend(70.84, 43, math.sin(70.84 / 7)) * (1 + 9 / 31)
q = "warm {math.floor(62.63)} {string.rep('=', 6 + 1)} gamma done"
q = "cold {math.floor(3.65)} {string.rep('=', 4 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 41.81) end
bag = { id = 9809, name = "cold-gamma", pos = { 95.55, 9809.5, 191.1 }, ok = false }
m = { ["k3826"] = { nested = { deeper = { value = "fast", list = { 1, 2, 3826 } } } } }
bag = { id = 8954, name = "fast-gamma", pos = { 71.93, 8954.5, 143.86 }, ok = true }
r = (32.84 > 43) and (acc ~= 0 and "fast" or "delta") or blend(32.84, 4993, 0.25)
if acc > 55.44 then acc = acc / 30 elseif acc < -55.44 then acc = -acc else acc = acc + 3233 end
if acc > 97.13 then acc = acc / 63 elseif acc < -97.13 then acc = -acc else acc = acc + 1931 end
for j = 1, 36 do acc = acc + (j * 32.36) % 97 / (j + 1) end
for j = 1, 36 do acc = acc + (j * 77.16) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 34.23) end
q = "green {math.floor(2.60)} {string.rep('=', 0 + 1)} alpha done"
bag = { id = 6492, name = "green-alpha", pos = { 87.25, 6492.5, 174.5 }, ok = true }
if acc > 0.16 then acc = acc / 27 elseif acc < -0.16 then acc = -acc else acc = acc + 5544 end
for j = 1, 7 do acc = acc + (j * 50.63) % 97 / (j + 1) end
bag = { id = 6352, name = "warm-beta", pos = { 25.54, 6352.5, 51.08 }, ok = true }
acc = acc + blend(28.66, 35, math.sin(28.66 / 7)) * (1 + 7 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 26.98) end
m = { ["k5235"] = { nested = { deeper = { value = "cold", list = { 1, 2, 5235 } } } } }
acc = acc + blend(7.97, 66, math.sin(7.97 / 7)) * (1 + 9 / 31)
r = (71.44 > 27) and (acc ~= 0 and "cold" or "gamma") or blend(71.44, 9527, 0.25)
if acc > 26.68 then acc = acc / 25 elseif acc < -26.68 then acc = -acc else acc = acc + 825 end
acc = acc + blend(99.01, 26, math.sin(99.01 / 7)) * (1 + 7 / 31)
m = { ["k1722"] = { nested = { deeper = { value = "cold", list = { 1, 2, 1722 } } } } }
if acc > 20.93 then acc = acc / 22 elseif acc < -20.93 then acc = -acc else acc = acc + 4293 end
if acc > 62.32 then acc = acc / 44 elseif acc < -62.32 then acc = -acc else acc = acc + 4048 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 4.73) end
if acc > 63.54 then acc = acc / 42 elseif acc < -63.54 then acc = -acc else acc = acc + 4758 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 20.29) end
r = (61.12 > 32) and (acc ~= 0 and "fast" or "delta") or blend(61.12, 932, 0.25)
if acc > 1.67 then acc = acc / 9 elseif acc < -1.67 then acc = -acc else acc = acc + 6149 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 30.97) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 48.03) end
for j = 1, 32 do acc = acc + (j * 43.62) % 97 / (j + 1) end
m = { ["k970"] = { nested = { deeper = { value = "cold", list = { 1, 2, 970 } } } } }
m = { ["k7490"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7490 } } } } }
acc = acc + blend(60.90, 90, math.sin(60.90 / 7)) * (1 + 0 / 31)
acc = acc + blend(76.06, 71, math.sin(76.06 / 7)) * (1 + 11 / 31)
bag = { id = 5312, name = "warm-delta", pos = { 41.04, 5312.5, 82.08 }, ok = true }
acc = acc + blend(49.66, 53, math.sin(49.66 / 7)) * (1 + 5 / 31)
acc = acc + blend(2.50, 7, math.sin(2.50 / 7)) * (1 + 7 / 31)
q = "green {math.floor(34.95)} {string.rep('=', 2 + 1)} gamma done"
r = (75.29 > 47) and (acc ~= 0 and "blue" or "gamma") or blend(75.29, 3997, 0.25)
bag = { id = 3985, name = "cold-beta", pos = { 47.09, 3985.5, 94.18 }, ok = false }
q = "fast {math.floor(22.97)} {string.rep('=', 2 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 97.85) end
bag = { id = 1458, name = "blue-delta", pos = { 60.04, 1458.5, 120.08 }, ok = true }
if acc > 27.21 then acc = acc / 69 elseif acc < -27.21 then acc = -acc else acc = acc + 2204 end
q = "blue {math.floor(79.96)} {string.rep('=', 3 + 1)} beta done"
if acc > 99.57 then acc = acc / 2 elseif acc < -99.57 then acc = -acc else acc = acc + 2226 end
bag = { id = 4740, name = "fast-beta", pos = { 75.45, 4740.5, 150.9 }, ok = true }
m = { ["k9611"] = { nested = { deeper = { value = "cold", list = { 1, 2, 9611 } } } } }
if acc > 16.33 then acc = acc / 36 elseif acc < -16.33 then acc = -acc else acc = acc + 9024 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 17.69) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 97.36) end
acc = acc + blend(31.04, 24, math.sin(31.04 / 7)) * (1 + 0 / 31)
q = "red {math.floor(34.78)} {string.rep('=', 0 + 1)} beta done"
m = { ["k9772"] = { nested = { deeper = { value = "red", list = { 1, 2, 9772 } } } } }
if acc > 82.68 then acc = acc / 80 elseif acc < -82.68 then acc = -acc else acc = acc + 7199 end
r = (12.84 > 15) and (acc ~= 0 and "warm" or "delta") or blend(12.84, 7215, 0.25)
acc = acc + blend(14.74, 34, math.sin(14.74 / 7)) * (1 + 11 / 31)
for j = 1, 36 do acc = acc + (j * 84.04) % 97 / (j + 1) end
acc = acc + blend(79.14, 37, math.sin(79.14 / 7)) * (1 + 7 / 31)
bag = { id = 1061, name = "warm-beta", pos = { 70.06, 1061.5, 140.12 }, ok = false }
q = "fast {math.floor(41.97)} {string.rep('=', 2 + 1)} gamma done"
q = "fast {math.floor(39.13)} {string.rep('=', 6 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 92.17) end
m = { ["k9558"] = { nested = { deeper = { value = "cold", list = { 1, 2, 9558 } } } } }
bag = { id = 7245, name = "cold-beta", pos = { 45.91, 7245.5, 91.82 }, ok = false }
q = "warm {math.floor(76.24)} {string.rep('=', 2 + 1)} gamma done"
q = "green {math.floor(66.73)} {string.rep('=', 2 + 1)} gamma done"
for j = 1, 6 do acc = acc + (j * 44.83) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 65.39) end
if acc > 33.38 then acc = acc / 75 elseif acc < -33.38 then acc = -acc else acc = acc + 7105 end
m = { ["k8432"] = { nested = { deeper = { value = "blue", list = { 1, 2, 8432 } } } } }
bag = { id = 2387, name = "red-gamma", pos = { 20.07, 2387.5, 40.14 }, ok = false }
r = (87.80 > 38) and (acc ~= 0 and "fast" or "gamma") or blend(87.80, 1238, 0.25)
if acc > 59.44 then acc = acc / 39 elseif acc < -59.44 then acc = -acc else acc = acc + 4399 end
bag = { id = 4797, name = "warm-gamma", pos = { 60.89, 4797.5, 121.78 }, ok = false }
r = (29.08 > 38) and (acc ~= 0 and "cold" or "delta") or blend(29.08, 9638, 0.25)
for j = 1, 6 do acc = acc + (j * 32.13) % 97 / (j + 1) end
acc = acc + blend(99.27, 30, math.sin(99.27 / 7)) * (1 + 5 / 31)
r = (44.79 > 15) and (acc ~= 0 and "cold" or "gamma") or blend(44.79, 2315, 0.25)
r = (95.56 > 23) and (acc ~= 0 and "blue" or "beta") or blend(95.56, 8123, 0.25)
acc = acc + blend(71.16, 80, math.sin(71.16 / 7)) * (1 + 9 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 20.65) end
r = (62.06 > 3) and (acc ~= 0 and "warm" or "gamma") or blend(62.06, 9603, 0.25)
q = "fast {math.floor(82.33)} {string.rep('=', 2 + 1)} alpha done"
m = { ["k7392"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7392 } } } } }
r = (72.16 > 36) and (acc ~= 0 and "fast" or "alpha") or blend(72.16, 1336, 0.25)
bag = { id = 41, name = "green-gamma", pos = { 2.35, 41.5, 4.7 }, ok = false }
for j = 1, 22 do acc = acc + (j * 34.04) % 97 / (j + 1) end
acc = acc + blend(88.34, 73, math.sin(88.34 / 7)) * (1 + 5 / 31)
acc = acc + blend(86.19, 88, math.sin(86.19 / 7)) * (1 + 6 / 31)
r = (87.00 > 44) and (acc ~= 0 and "blue" or "delta") or blend(87.00, 5144, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 93.87) end
if acc > 10.33 then acc = acc / 77 elseif acc < -10.33 then acc = -acc else acc = acc + 8709 end
r = (65.87 > 33) and (acc ~= 0 and "green" or "gamma") or blend(65.87, 983, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 74.22) end
q = "green {math.floor(45.63)} {string.rep('=', 4 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 8.27) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 20.31) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 41.66) end
q = "green {math.floor(57.97)} {string.rep('=', 3 + 1)} delta done"
q = "cold {math.floor(49.40)} {string.rep('=', 4 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 36.91) end
bag = { id = 4805, name = "fast-gamma", pos = { 29.26, 4805.5, 58.52 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 32.21) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 97.81) end
bag = { id = 1630, name = "fast-gamma", pos = { 5.04, 1630.5, 10.08 }, ok = true }
q = "red {math.floor(50.22)} {string.rep('=', 1 + 1)} beta done"
q = "blue {math.floor(80.29)} {string.rep('=', 5 + 1)} gamma done"
if acc > 79.55 then acc = acc / 6 elseif acc < -79.55 then acc = -acc else acc = acc + 1518 end
acc = acc + blend(30.00, 12, math.sin(30.00 / 7)) * (1 + 2 / 31)
for j = 1, 17 do acc = acc + (j * 2.73) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 84.52) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 49.27) end
r = (1.38 > 12) and (acc ~= 0 and "warm" or "alpha") or blend(1.38, 8012, 0.25)
q = "blue {math.floor(40.51)} {string.rep('=', 5 + 1)} alpha done"
q = "red {math.floor(15.60)} {string.rep('=', 5 + 1)} beta done"
for j = 1, 10 do acc = acc + (j * 79.03) % 97 / (j + 1) end
if acc > 4.86 then acc = acc / 43 elseif acc < -4.86 then acc = -acc else acc = acc + 3246 end
r = (84.78 > 21) and (acc ~= 0 and "warm" or "beta") or blend(84.78, 6571, 0.25)
r = (10.48 > 36) and (acc ~= 0 and "fast" or "gamma") or blend(10.48, 8836, 0.25)
r = (14.18 > 11) and (acc ~= 0 and "red" or "beta") or blend(14.18, 3961, 0.25)
m = { ["k7592"] = { nested = { deeper = { value = "cold", list = { 1, 2, 7592 } } } } }
q = "red {math.floor(85.58)} {string.rep('=', 2 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 36.71) end
q = "red {math.floor(67.05)} {string.rep('=', 1 + 1)} beta done"
bag = { id = 8907, name = "blue-gamma", pos = { 44.16, 8907.5, 88.32 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 65.18) end
m = { ["k9320"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9320 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 43.36) end
r = (2.99 > 34) and (acc ~= 0 and "fast" or "beta") or blend(2.99, 8484, 0.25)
m = { ["k9932"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9932 } } } } }
bag = { id = 2308, name = "green-alpha", pos = { 34.69, 2308.5, 69.38 }, ok = true }
if acc > 21.97 then acc = acc / 85 elseif acc < -21.97 then acc = -acc else acc = acc + 6225 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 42.82) end
bag = { id = 878, name = "fast-gamma", pos = { 34.28, 878.5, 68.56 }, ok = true }
if acc > 89.47 then acc = acc / 76 elseif acc < -89.47 then acc = -acc else acc = acc + 7907 end
q = "red {math.floor(33.16)} {string.rep('=', 2 + 1)} delta done"
if acc > 12.61 then acc = acc / 72 elseif acc < -12.61 then acc = -acc else acc = acc + 3275 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 1.63) end
acc = acc + blend(50.28, 27, math.sin(50.28 / 7)) * (1 + 2 / 31)
if acc > 5.69 then acc = acc / 40 elseif acc < -5.69 then acc = -acc else acc = acc + 4934 end
r = (78.50 > 16) and (acc ~= 0 and "warm" or "gamma") or blend(78.50, 9416, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 40.27) end
acc = acc + blend(36.55, 65, math.sin(36.55 / 7)) * (1 + 3 / 31)
r = (78.07 > 44) and (acc ~= 0 and "blue" or "delta") or blend(78.07, 6844, 0.25)
q = "fast {math.floor(33.53)} {string.rep('=', 3 + 1)} alpha done"
bag = { id = 217, name = "warm-gamma", pos = { 6.73, 217.5, 13.46 }, ok = false }
m = { ["k8120"] = { nested = { deeper = { value = "blue", list = { 1, 2, 8120 } } } } }
bag = { id = 5187, name = "red-delta", pos = { 94.32, 5187.5, 188.64 }, ok = false }
acc = acc + blend(63.38, 49, math.sin(63.38 / 7)) * (1 + 3 / 31)
q = "cold {math.floor(2.61)} {string.rep('=', 0 + 1)} delta done"
r = (20.76 > 20) and (acc ~= 0 and "green" or "alpha") or blend(20.76, 2270, 0.25)
q = "cold {math.floor(29.45)} {string.rep('=', 1 + 1)} gamma done"
if acc > 82.14 then acc = acc / 10 elseif acc < -82.14 then acc = -acc else acc = acc + 4726 end
q = "red {math.floor(11.74)} {string.rep('=', 0 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 53.16) end
bag = { id = 6143, name = "blue-beta", pos = { 15.11, 6143.5, 30.22 }, ok = false }
m = { ["k2331"] = { nested = { deeper = { value = "warm", list = { 1, 2, 2331 } } } } }
r = (81.81 > 9) and (acc ~= 0 and "cold" or "beta") or blend(81.81, 7009, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 54.40) end
m = { ["k6432"] = { nested = { deeper = { value = "blue", list = { 1, 2, 6432 } } } } }
for j = 1, 7 do acc = acc + (j * 26.85) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 8.59) end
m = { ["k9169"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9169 } } } } }
r = (89.61 > 35) and (acc ~= 0 and "cold" or "delta") or blend(89.61, 7985, 0.25)
if acc > 81.20 then acc = acc / 17 elseif acc < -81.20 then acc = -acc else acc = acc + 817 end
bag = { id = 1447, name = "blue-gamma", pos = { 5.34, 1447.5, 10.68 }, ok = false }
acc = acc + blend(93.13, 54, math.sin(93.13 / 7)) * (1 + 12 / 31)
q = "red {math.floor(38.03)} {string.rep('=', 6 + 1)} beta done"
acc = acc + blend(17.90, 29, math.sin(17.90 / 7)) * (1 + 9 / 31)
acc = acc + blend(65.28, 30, math.sin(65.28 / 7)) * (1 + 5 / 31)
q = "warm {math.floor(28.80)} {string.rep('=', 3 + 1)} alpha done"
if acc > 83.06 then acc = acc / 8 elseif acc < -83.06 then acc = -acc else acc = acc + 6148 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 14.31) end
if acc > 49.94 then acc = acc / 80 elseif acc < -49.94 then acc = -acc else acc = acc + 9780 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 83.00) end
acc = acc + blend(99.02, 78, math.sin(99.02 / 7)) * (1 + 9 / 31)
r = (98.56 > 29) and (acc ~= 0 and "cold" or "delta") or blend(98.56, 429, 0.25)
if acc > 61.24 then acc = acc / 27 elseif acc < -61.24 then acc = -acc else acc = acc + 2162 end
q = "blue {math.floor(21.47)} {string.rep('=', 6 + 1)} beta done"
q = "cold {math.floor(60.08)} {string.rep('=', 1 + 1)} alpha done"
if acc > 69.22 then acc = acc / 77 elseif acc < -69.22 then acc = -acc else acc = acc + 9421 end
m = { ["k2854"] = { nested = { deeper = { value = "fast", list = { 1, 2, 2854 } } } } }
bag = { id = 3476, name = "red-alpha", pos = { 38.82, 3476.5, 77.64 }, ok = true }
q = "red {math.floor(35.37)} {string.rep('=', 4 + 1)} beta done"
if acc > 97.45 then acc = acc / 32 elseif acc < -97.45 then acc = -acc else acc = acc + 743 end
r = (78.93 > 25) and (acc ~= 0 and "warm" or "beta") or blend(78.93, 2725, 0.25)
for j = 1, 14 do acc = acc + (j * 18.15) % 97 / (j + 1) end
r = (46.58 > 27) and (acc ~= 0 and "red" or "beta") or blend(46.58, 1777, 0.25)
for j = 1, 23 do acc = acc + (j * 50.73) % 97 / (j + 1) end
r = (42.42 > 7) and (acc ~= 0 and "red" or "alpha") or blend(42.42, 4607, 0.25)
m = { ["k3007"] = { nested = { deeper = { value = "red", list = { 1, 2, 3007 } } } } }
for j = 1, 35 do acc = acc + (j * 83.51) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 2.24) end
for j = 1, 37 do acc = acc + (j * 51.70) % 97 / (j + 1) end
if acc > 12.87 then acc = acc / 50 elseif acc < -12.87 then acc = -acc else acc = acc + 3609 end
bag = { id = 8295, name = "blue-beta", pos = { 97.90, 8295.5, 195.8 }, ok = false }
for j = 1, 18 do acc = acc + (j * 14.95) % 97 / (j + 1) end
q = "red {math.floor(17.40)} {string.rep('=', 2 + 1)} gamma done"
r = (39.78 > 32) and (acc ~= 0 and "warm" or "beta") or blend(39.78, 5482, 0.25)
bag = { id = 4769, name = "red-delta", pos = { 20.70, 4769.5, 41.4 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 14.47) end
r = (67.89 > 17) and (acc ~= 0 and "red" or "alpha") or blend(67.89, 4767, 0.25)
acc = acc + blend(25.93, 69, math.sin(25.93 / 7)) * (1 + 11 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 73.27) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 89.78) end
if acc > 25.85 then acc = acc / 61 elseif acc < -25.85 then acc = -acc else acc = acc + 6112 end
if acc > 0.56 then acc = acc / 25 elseif acc < -0.56 then acc = -acc else acc = acc + 2160 end
acc = acc + blend(67.65, 62, math.sin(67.65 / 7)) * (1 + 1 / 31)
if acc > 85.73 then acc = acc / 52 elseif acc < -85.73 then acc = -acc else acc = acc + 4234 end
bag = { id = 3184, name = "fast-delta", pos = { 31.17, 3184.5, 62.34 }, ok = true }
if acc > 16.57 then acc = acc / 11 elseif acc < -16.57 then acc = -acc else acc = acc + 3036 end
r = (16.42 > 45) and (acc ~= 0 and "warm" or "gamma") or blend(16.42, 345, 0.25)
r = (76.69 > 33) and (acc ~= 0 and "red" or "gamma") or blend(76.69, 5983, 0.25)
q = "warm {math.floor(92.74)} {string.rep('=', 1 + 1)} gamma done"
r = (33.92 > 4) and (acc ~= 0 and "red" or "alpha") or blend(33.92, 9354, 0.25)
bag = { id = 5286, name = "warm-beta", pos = { 97.25, 5286.5, 194.5 }, ok = true }
if acc > 35.01 then acc = acc / 50 elseif acc < -35.01 then acc = -acc else acc = acc + 4232 end
for j = 1, 12 do acc = acc + (j * 77.15) % 97 / (j + 1) end
r = (26.90 > 10) and (acc ~= 0 and "cold" or "alpha") or blend(26.90, 5010, 0.25)
m = { ["k6527"] = { nested = { deeper = { value = "warm", list = { 1, 2, 6527 } } } } }
if acc > 33.73 then acc = acc / 26 elseif acc < -33.73 then acc = -acc else acc = acc + 7768 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 66.91) end
r = (66.88 > 28) and (acc ~= 0 and "fast" or "alpha") or blend(66.88, 3378, 0.25)
r = (49.64 > 29) and (acc ~= 0 and "blue" or "gamma") or blend(49.64, 2879, 0.25)
bag = { id = 7874, name = "warm-beta", pos = { 47.35, 7874.5, 94.7 }, ok = true }
for j = 1, 24 do acc = acc + (j * 46.24) % 97 / (j + 1) end
for j = 1, 24 do acc = acc + (j * 57.15) % 97 / (j + 1) end
if acc > 31.46 then acc = acc / 56 elseif acc < -31.46 then acc = -acc else acc = acc + 6552 end
bag = { id = 3662, name = "blue-alpha", pos = { 39.79, 3662.5, 79.58 }, ok = true }
for j = 1, 17 do acc = acc + (j * 96.99) % 97 / (j + 1) end
m = { ["k6100"] = { nested = { deeper = { value = "green", list = { 1, 2, 6100 } } } } }
for j = 1, 27 do acc = acc + (j * 53.49) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 53.71) end
bag = { id = 3148, name = "blue-delta", pos = { 12.74, 3148.5, 25.48 }, ok = true }
for j = 1, 22 do acc = acc + (j * 72.06) % 97 / (j + 1) end
if acc > 26.94 then acc = acc / 61 elseif acc < -26.94 then acc = -acc else acc = acc + 5489 end
acc = acc + blend(40.08, 82, math.sin(40.08 / 7)) * (1 + 0 / 31)
m = { ["k3908"] = { nested = { deeper = { value = "red", list = { 1, 2, 3908 } } } } }
for j = 1, 28 do acc = acc + (j * 11.20) % 97 / (j + 1) end
m = { ["k637"] = { nested = { deeper = { value = "green", list = { 1, 2, 637 } } } } }
q = "cold {math.floor(92.14)} {string.rep('=', 6 + 1)} beta done"
if acc > 70.18 then acc = acc / 46 elseif acc < -70.18 then acc = -acc else acc = acc + 7432 end
bag = { id = 4870, name = "blue-gamma", pos = { 38.61, 4870.5, 77.22 }, ok = true }
m = { ["k7435"] = { nested = { deeper = { value = "red", list = { 1, 2, 7435 } } } } }
acc = acc + blend(72.91, 17, math.sin(72.91 / 7)) * (1 + 11 / 31)
m = { ["k666"] = { nested = { deeper = { value = "green", list = { 1, 2, 666 } } } } }
r = (70.62 > 15) and (acc ~= 0 and "green" or "gamma") or blend(70.62, 715, 0.25)
for j = 1, 36 do acc = acc + (j * 44.43) % 97 / (j + 1) end
acc = acc + blend(50.10, 16, math.sin(50.10 / 7)) * (1 + 9 / 31)
acc = acc + blend(72.96, 85, math.sin(72.96 / 7)) * (1 + 8 / 31)
bag = { id = 2393, name = "green-alpha", pos = { 94.95, 2393.5, 189.9 }, ok = false }
bag = { id = 1003, name = "red-delta", pos = { 73.22, 1003.5, 146.44 }, ok = false }
m = { ["k9970"] = { nested = { deeper = { value = "green", list = { 1, 2, 9970 } } } } }
bag = { id = 7580, name = "fast-delta", pos = { 76.72, 7580.5, 153.44 }, ok = true }
acc = acc + blend(13.65, 2, math.sin(13.65 / 7)) * (1 + 6 / 31)
if acc > 10.61 then acc = acc / 49 elseif acc < -10.61 then acc = -acc else acc = acc + 6812 end
r = (88.15 > 37) and (acc ~= 0 and "blue" or "alpha") or blend(88.15, 5787, 0.25)
for j = 1, 13 do acc = acc + (j * 34.16) % 97 / (j + 1) end
m = { ["k8215"] = { nested = { deeper = { value = "cold", list = { 1, 2, 8215 } } } } }
r = (13.98 > 32) and (acc ~= 0 and "warm" or "beta") or blend(13.98, 2932, 0.25)
if acc > 84.25 then acc = acc / 64 elseif acc < -84.25 then acc = -acc else acc = acc + 2110 end
r = (22.20 > 14) and (acc ~= 0 and "cold" or "delta") or blend(22.20, 3564, 0.25)
m = { ["k316"] = { nested = { deeper = { value = "blue", list = { 1, 2, 316 } } } } }
q = "fast {math.floor(72.79)} {string.rep('=', 1 + 1)} gamma done"
for j = 1, 34 do acc = acc + (j * 44.81) % 97 / (j + 1) end
r = (79.67 > 38) and (acc ~= 0 and "warm" or "alpha") or blend(79.67, 6938, 0.25)
m = { ["k7851"] = { nested = { deeper = { value = "red", list = { 1, 2, 7851 } } } } }
r = (83.79 > 28) and (acc ~= 0 and "fast" or "gamma") or blend(83.79, 9828, 0.25)
if acc > 83.19 then acc = acc / 59 elseif acc < -83.19 then acc = -acc else acc = acc + 5220 end
q = "green {math.floor(60.60)} {string.rep('=', 1 + 1)} gamma done"
if acc > 82.22 then acc = acc / 35 elseif acc < -82.22 then acc = -acc else acc = acc + 7688 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 40.39) end
q = "red {math.floor(22.67)} {string.rep('=', 2 + 1)} gamma done"
q = "warm {math.floor(58.52)} {string.rep('=', 1 + 1)} beta done"
if acc > 74.63 then acc = acc / 83 elseif acc < -74.63 then acc = -acc else acc = acc + 9160 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 56.53) end
acc = acc + blend(77.81, 73, math.sin(77.81 / 7)) * (1 + 6 / 31)
if acc > 84.94 then acc = acc / 55 elseif acc < -84.94 then acc = -acc else acc = acc + 7975 end
acc = acc + blend(8.79, 36, math.sin(8.79 / 7)) * (1 + 5 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 70.33) end
r = (55.83 > 21) and (acc ~= 0 and "cold" or "beta") or blend(55.83, 3771, 0.25)
if acc > 9.50 then acc = acc / 51 elseif acc < -9.50 then acc = -acc else acc = acc + 6725 end
for j = 1, 6 do acc = acc + (j * 58.15) % 97 / (j + 1) end
acc = acc + blend(97.83, 38, math.sin(97.83 / 7)) * (1 + 4 / 31)
for j = 1, 7 do acc = acc + (j * 17.03) % 97 / (j + 1) end
m = { ["k9501"] = { nested = { deeper = { value = "green", list = { 1, 2, 9501 } } } } }
q = "green {math.floor(58.95)} {string.rep('=', 3 + 1)} delta done"
if acc > 32.11 then acc = acc / 58 elseif acc < -32.11 then acc = -acc else acc = acc + 1481 end
if acc > 58.46 then acc = acc / 8 elseif acc < -58.46 then acc = -acc else acc = acc + 2321 end
m = { ["k1653"] = { nested = { deeper = { value = "fast", list = { 1, 2, 1653 } } } } }
r = (90.39 > 45) and (acc ~= 0 and "blue" or "gamma") or blend(90.39, 3395, 0.25)
if acc > 8.22 then acc = acc / 27 elseif acc < -8.22 then acc = -acc else acc = acc + 8392 end
r = (9.66 > 30) and (acc ~= 0 and "cold" or "gamma") or blend(9.66, 3180, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 22.66) end
if acc > 33.36 then acc = acc / 73 elseif acc < -33.36 then acc = -acc else acc = acc + 2297 end
m = { ["k5093"] = { nested = { deeper = { value = "cold", list = { 1, 2, 5093 } } } } }
if acc > 54.76 then acc = acc / 37 elseif acc < -54.76 then acc = -acc else acc = acc + 659 end
acc = acc + blend(25.10, 60, math.sin(25.10 / 7)) * (1 + 8 / 31)
acc = acc + blend(34.38, 60, math.sin(34.38 / 7)) * (1 + 3 / 31)
m = { ["k4393"] = { nested = { deeper = { value = "cold", list = { 1, 2, 4393 } } } } }
m = { ["k2812"] = { nested = { deeper = { value = "blue", list = { 1, 2, 2812 } } } } }
q = "cold {math.floor(15.46)} {string.rep('=', 1 + 1)} beta done"
if acc > 26.23 then acc = acc / 70 elseif acc < -26.23 then acc = -acc else acc = acc + 5053 end
q = "fast {math.floor(25.87)} {string.rep('=', 5 + 1)} beta done"
for j = 1, 15 do acc = acc + (j * 90.04) % 97 / (j + 1) end
acc = acc + blend(2.98, 26, math.sin(2.98 / 7)) * (1 + 1 / 31)
m = { ["k7320"] = { nested = { deeper = { value = "green", list = { 1, 2, 7320 } } } } }
r = (15.81 > 29) and (acc ~= 0 and "blue" or "delta") or blend(15.81, 5979, 0.25)
bag = { id = 9132, name = "warm-delta", pos = { 44.15, 9132.5, 88.3 }, ok = true }
q = "warm {math.floor(73.71)} {string.rep('=', 0 + 1)} beta done"
for j = 1, 33 do acc = acc + (j * 3.63) % 97 / (j + 1) end
for j = 1, 21 do acc = acc + (j * 32.45) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 23.09) end
bag = { id = 1525, name = "red-gamma", pos = { 6.44, 1525.5, 12.88 }, ok = false }
if acc > 72.67 then acc = acc / 20 elseif acc < -72.67 then acc = -acc else acc = acc + 1176 end
acc = acc + blend(64.67, 70, math.sin(64.67 / 7)) * (1 + 3 / 31)
for j = 1, 33 do acc = acc + (j * 68.92) % 97 / (j + 1) end
m = { ["k6127"] = { nested = { deeper = { value = "green", list = { 1, 2, 6127 } } } } }
q = "blue {math.floor(13.04)} {string.rep('=', 6 + 1)} gamma done"
m = { ["k9931"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9931 } } } } }
for j = 1, 11 do acc = acc + (j * 99.89) % 97 / (j + 1) end
acc = acc + blend(90.81, 10, math.sin(90.81 / 7)) * (1 + 8 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 44.03) end
for j = 1, 35 do acc = acc + (j * 16.01) % 97 / (j + 1) end
bag = { id = 1145, name = "green-alpha", pos = { 10.18, 1145.5, 20.36 }, ok = false }
r = (79.61 > 45) and (acc ~= 0 and "green" or "delta") or blend(79.61, 1795, 0.25)
for j = 1, 36 do acc = acc + (j * 24.20) % 97 / (j + 1) end
bag = { id = 6669, name = "warm-beta", pos = { 36.79, 6669.5, 73.58 }, ok = false }
q = "warm {math.floor(8.40)} {string.rep('=', 5 + 1)} gamma done"
if acc > 82.00 then acc = acc / 34 elseif acc < -82.00 then acc = -acc else acc = acc + 1101 end
for j = 1, 4 do acc = acc + (j * 12.91) % 97 / (j + 1) end
acc = acc + blend(11.18, 92, math.sin(11.18 / 7)) * (1 + 12 / 31)
if acc > 33.84 then acc = acc / 58 elseif acc < -33.84 then acc = -acc else acc = acc + 502 end
if acc > 28.60 then acc = acc / 55 elseif acc < -28.60 then acc = -acc else acc = acc + 3970 end
q = "cold {math.floor(1.16)} {string.rep('=', 4 + 1)} gamma done"
if acc > 21.78 then acc = acc / 75 elseif acc < -21.78 then acc = -acc else acc = acc + 6215 end
m = { ["k7177"] = { nested = { deeper = { value = "green", list = { 1, 2, 7177 } } } } }
if acc > 36.93 then acc = acc / 41 elseif acc < -36.93 then acc = -acc else acc = acc + 9029 end
acc = acc + blend(88.24, 35, math.sin(88.24 / 7)) * (1 + 11 / 31)
for j = 1, 32 do acc = acc + (j * 16.70) % 97 / (j + 1) end
acc = acc + blend(57.48, 9, math.sin(57.48 / 7)) * (1 + 1 / 31)
r = (16.40 > 35) and (acc ~= 0 and "cold" or "delta") or blend(16.40, 9085, 0.25)
bag = { id = 4335, name = "red-beta", pos = { 5.63, 4335.5, 11.26 }, ok = false }
if acc > 52.32 then acc = acc / 25 elseif acc < -52.32 then acc = -acc else acc = acc + 6343 end
for j = 1, 30 do acc = acc + (j * 80.63) % 97 / (j + 1) end
acc = acc + blend(60.06, 1, math.sin(60.06 / 7)) * (1 + 7 / 31)
bag = { id = 8759, name = "blue-gamma", pos = { 14.99, 8759.5, 29.98 }, ok = false }
for j = 1, 39 do acc = acc + (j * 11.11) % 97 / (j + 1) end
if acc > 33.38 then acc = acc / 11 elseif acc < -33.38 then acc = -acc else acc = acc + 6418 end
if acc > 56.00 then acc = acc / 25 elseif acc < -56.00 then acc = -acc else acc = acc + 5809 end
if acc > 31.49 then acc = acc / 75 elseif acc < -31.49 then acc = -acc else acc = acc + 5147 end
q = "warm {math.floor(89.76)} {string.rep('=', 3 + 1)} delta done"
m = { ["k5302"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5302 } } } } }
for j = 1, 19 do acc = acc + (j * 76.25) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 49.32) end
for j = 1, 18 do acc = acc + (j * 78.21) % 97 / (j + 1) end
r = (12.78 > 35) and (acc ~= 0 and "blue" or "alpha") or blend(12.78, 2935, 0.25)
for j = 1, 14 do acc = acc + (j * 25.82) % 97 / (j + 1) end
acc = acc + blend(70.69, 26, math.sin(70.69 / 7)) * (1 + 3 / 31)
for j = 1, 15 do acc = acc + (j * 74.58) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 88.34) end
acc = acc + blend(59.91, 47, math.sin(59.91 / 7)) * (1 + 6 / 31)
m = { ["k5355"] = { nested = { deeper = { value = "green", list = { 1, 2, 5355 } } } } }
acc = acc + blend(6.46, 20, math.sin(6.46 / 7)) * (1 + 6 / 31)
bag = { id = 6508, name = "blue-gamma", pos = { 91.40, 6508.5, 182.8 }, ok = true }
r = (1.94 > 14) and (acc ~= 0 and "warm" or "delta") or blend(1.94, 5664, 0.25)
q = "cold {math.floor(10.24)} {string.rep('=', 4 + 1)} beta done"
acc = acc + blend(53.95, 7, math.sin(53.95 / 7)) * (1 + 9 / 31)
bag = { id = 7208, name = "blue-alpha", pos = { 25.16, 7208.5, 50.32 }, ok = true }
r = (20.29 > 30) and (acc ~= 0 and "cold" or "delta") or blend(20.29, 8330, 0.25)
m = { ["k7928"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7928 } } } } }
if acc > 94.59 then acc = acc / 35 elseif acc < -94.59 then acc = -acc else acc = acc + 3238 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 36.17) end
for j = 1, 15 do acc = acc + (j * 46.58) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 60.60) end
bag = { id = 9547, name = "warm-delta", pos = { 15.56, 9547.5, 31.12 }, ok = false }
m = { ["k5363"] = { nested = { deeper = { value = "red", list = { 1, 2, 5363 } } } } }
acc = acc + blend(43.15, 23, math.sin(43.15 / 7)) * (1 + 9 / 31)
q = "cold {math.floor(81.88)} {string.rep('=', 4 + 1)} delta done"
for j = 1, 24 do acc = acc + (j * 63.58) % 97 / (j + 1) end
if acc > 93.23 then acc = acc / 53 elseif acc < -93.23 then acc = -acc else acc = acc + 8151 end
if acc > 52.83 then acc = acc / 30 elseif acc < -52.83 then acc = -acc else acc = acc + 8929 end
acc = acc + blend(15.21, 55, math.sin(15.21 / 7)) * (1 + 2 / 31)
acc = acc + blend(27.59, 37, math.sin(27.59 / 7)) * (1 + 2 / 31)
bag = { id = 4164, name = "red-gamma", pos = { 1.61, 4164.5, 3.22 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 88.52) end
m = { ["k44"] = { nested = { deeper = { value = "fast", list = { 1, 2, 44 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 74.79) end
for j = 1, 18 do acc = acc + (j * 77.23) % 97 / (j + 1) end
for j = 1, 17 do acc = acc + (j * 45.62) % 97 / (j + 1) end
m = { ["k5084"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5084 } } } } }
m = { ["k6251"] = { nested = { deeper = { value = "green", list = { 1, 2, 6251 } } } } }
r = (35.03 > 5) and (acc ~= 0 and "blue" or "beta") or blend(35.03, 905, 0.25)
for j = 1, 40 do acc = acc + (j * 44.91) % 97 / (j + 1) end
acc = acc + blend(22.85, 25, math.sin(22.85 / 7)) * (1 + 3 / 31)
acc = acc + blend(54.37, 54, math.sin(54.37 / 7)) * (1 + 8 / 31)
for j = 1, 27 do acc = acc + (j * 41.83) % 97 / (j + 1) end
bag = { id = 3001, name = "fast-delta", pos = { 38.94, 3001.5, 77.88 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 2.02) end
bag = { id = 6950, name = "green-beta", pos = { 62.30, 6950.5, 124.6 }, ok = true }
r = (57.39 > 0) and (acc ~= 0 and "blue" or "gamma") or blend(57.39, 2450, 0.25)
if acc > 39.55 then acc = acc / 62 elseif acc < -39.55 then acc = -acc else acc = acc + 9584 end
if acc > 59.84 then acc = acc / 56 elseif acc < -59.84 then acc = -acc else acc = acc + 233 end
bag = { id = 4152, name = "blue-delta", pos = { 23.97, 4152.5, 47.94 }, ok = true }
acc = acc + blend(31.40, 68, math.sin(31.40 / 7)) * (1 + 5 / 31)
for j = 1, 21 do acc = acc + (j * 49.35) % 97 / (j + 1) end
bag = { id = 1390, name = "red-alpha", pos = { 9.68, 1390.5, 19.36 }, ok = true }
if acc > 71.83 then acc = acc / 9 elseif acc < -71.83 then acc = -acc else acc = acc + 898 end
m = { ["k2832"] = { nested = { deeper = { value = "fast", list = { 1, 2, 2832 } } } } }
if acc > 5.10 then acc = acc / 21 elseif acc < -5.10 then acc = -acc else acc = acc + 9365 end
if acc > 90.47 then acc = acc / 82 elseif acc < -90.47 then acc = -acc else acc = acc + 3819 end
m = { ["k5162"] = { nested = { deeper = { value = "blue", list = { 1, 2, 5162 } } } } }
q = "green {math.floor(14.11)} {string.rep('=', 3 + 1)} gamma done"
r = (65.65 > 0) and (acc ~= 0 and "warm" or "beta") or blend(65.65, 7900, 0.25)
r = (10.39 > 12) and (acc ~= 0 and "warm" or "alpha") or blend(10.39, 7812, 0.25)
m = { ["k2685"] = { nested = { deeper = { value = "blue", list = { 1, 2, 2685 } } } } }
m = { ["k6477"] = { nested = { deeper = { value = "fast", list = { 1, 2, 6477 } } } } }
acc = acc + blend(2.40, 11, math.sin(2.40 / 7)) * (1 + 10 / 31)
bag = { id = 7389, name = "cold-beta", pos = { 17.64, 7389.5, 35.28 }, ok = false }
for j = 1, 37 do acc = acc + (j * 24.73) % 97 / (j + 1) end
for j = 1, 34 do acc = acc + (j * 96.13) % 97 / (j + 1) end
m = { ["k3729"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3729 } } } } }
for j = 1, 31 do acc = acc + (j * 99.83) % 97 / (j + 1) end
for j = 1, 30 do acc = acc + (j * 24.93) % 97 / (j + 1) end
m = { ["k7048"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7048 } } } } }
q = "fast {math.floor(99.66)} {string.rep('=', 3 + 1)} beta done"
for j = 1, 35 do acc = acc + (j * 38.48) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 39.96) end
bag = { id = 4627, name = "green-alpha", pos = { 17.16, 4627.5, 34.32 }, ok = false }
for j = 1, 27 do acc = acc + (j * 44.67) % 97 / (j + 1) end
for j = 1, 16 do acc = acc + (j * 70.35) % 97 / (j + 1) end
for j = 1, 17 do acc = acc + (j * 0.79) % 97 / (j + 1) end
for j = 1, 3 do acc = acc + (j * 55.41) % 97 / (j + 1) end
q = "green {math.floor(7.76)} {string.rep('=', 3 + 1)} alpha done"
acc = acc + blend(32.02, 7, math.sin(32.02 / 7)) * (1 + 8 / 31)
for j = 1, 28 do acc = acc + (j * 18.37) % 97 / (j + 1) end
acc = acc + blend(27.18, 94, math.sin(27.18 / 7)) * (1 + 4 / 31)
acc = acc + blend(61.41, 80, math.sin(61.41 / 7)) * (1 + 12 / 31)
acc = acc + blend(89.30, 63, math.sin(89.30 / 7)) * (1 + 11 / 31)
bag = { id = 7623, name = "red-alpha", pos = { 34.87, 7623.5, 69.74 }, ok = false }
r = (49.24 > 30) and (acc ~= 0 and "cold" or "alpha") or blend(49.24, 2280, 0.25)
m = { ["k1679"] = { nested = { deeper = { value = "fast", list = { 1, 2, 1679 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 78.15) end
q = "cold {math.floor(37.64)} {string.rep('=', 0 + 1)} beta done"
for j = 1, 9 do acc = acc + (j * 44.48) % 97 / (j + 1) end
q = "fast {math.floor(25.30)} {string.rep('=', 0 + 1)} beta done"
q = "warm {math.floor(8.54)} {string.rep('=', 6 + 1)} alpha done"
for j = 1, 17 do acc = acc + (j * 30.96) % 97 / (j + 1) end
r = (82.47 > 39) and (acc ~= 0 and "cold" or "gamma") or blend(82.47, 8839, 0.25)
r = (19.21 > 6) and (acc ~= 0 and "red" or "delta") or blend(19.21, 5706, 0.25)
acc = acc + blend(7.84, 12, math.sin(7.84 / 7)) * (1 + 0 / 31)
if acc > 86.12 then acc = acc / 53 elseif acc < -86.12 then acc = -acc else acc = acc + 141 end
for j = 1, 3 do acc = acc + (j * 94.87) % 97 / (j + 1) end
bag = { id = 751, name = "warm-delta", pos = { 1.16, 751.5, 2.32 }, ok = false }
r = (5.35 > 27) and (acc ~= 0 and "blue" or "gamma") or blend(5.35, 7927, 0.25)
if acc > 14.96 then acc = acc / 83 elseif acc < -14.96 then acc = -acc else acc = acc + 9516 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 79.49) end
for j = 1, 1 do acc = acc + (j * 63.40) % 97 / (j + 1) end
q = "green {math.floor(84.53)} {string.rep('=', 0 + 1)} delta done"
for j = 1, 34 do acc = acc + (j * 37.29) % 97 / (j + 1) end
bag = { id = 1460, name = "blue-beta", pos = { 32.25, 1460.5, 64.5 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 25.71) end
m = { ["k2031"] = { nested = { deeper = { value = "fast", list = { 1, 2, 2031 } } } } }
for j = 1, 32 do acc = acc + (j * 37.45) % 97 / (j + 1) end
r = (70.99 > 0) and (acc ~= 0 and "red" or "delta") or blend(70.99, 4800, 0.25)
r = (64.52 > 45) and (acc ~= 0 and "red" or "beta") or blend(64.52, 7845, 0.25)
bag = { id = 4028, name = "warm-beta", pos = { 16.87, 4028.5, 33.74 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 30.40) end
bag = { id = 5367, name = "green-beta", pos = { 40.31, 5367.5, 80.62 }, ok = false }
acc = acc + blend(70.99, 81, math.sin(70.99 / 7)) * (1 + 1 / 31)
if acc > 31.20 then acc = acc / 72 elseif acc < -31.20 then acc = -acc else acc = acc + 1762 end
acc = acc + blend(82.48, 39, math.sin(82.48 / 7)) * (1 + 12 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 11.32) end
m = { ["k3612"] = { nested = { deeper = { value = "fast", list = { 1, 2, 3612 } } } } }
q = "blue {math.floor(2.67)} {string.rep('=', 2 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 57.39) end
r = (15.58 > 48) and (acc ~= 0 and "fast" or "alpha") or blend(15.58, 2948, 0.25)
r = (64.15 > 7) and (acc ~= 0 and "blue" or "alpha") or blend(64.15, 3207, 0.25)
q = "warm {math.floor(46.51)} {string.rep('=', 4 + 1)} alpha done"
bag = { id = 5308, name = "fast-delta", pos = { 82.55, 5308.5, 165.1 }, ok = true }
acc = acc + blend(77.12, 50, math.sin(77.12 / 7)) * (1 + 9 / 31)
for j = 1, 22 do acc = acc + (j * 66.84) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 4.03) end
for j = 1, 3 do acc = acc + (j * 56.08) % 97 / (j + 1) end
bag = { id = 9917, name = "cold-beta", pos = { 31.67, 9917.5, 63.34 }, ok = false }
acc = acc + blend(63.40, 62, math.sin(63.40 / 7)) * (1 + 7 / 31)
r = (12.28 > 37) and (acc ~= 0 and "green" or "delta") or blend(12.28, 2737, 0.25)
r = (39.97 > 3) and (acc ~= 0 and "fast" or "beta") or blend(39.97, 3003, 0.25)
q = "cold {math.floor(47.21)} {string.rep('=', 6 + 1)} delta done"
for j = 1, 18 do acc = acc + (j * 51.89) % 97 / (j + 1) end
if acc > 53.89 then acc = acc / 81 elseif acc < -53.89 then acc = -acc else acc = acc + 5420 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 9.51) end
bag = { id = 6047, name = "fast-delta", pos = { 84.77, 6047.5, 169.54 }, ok = false }
r = (90.23 > 48) and (acc ~= 0 and "fast" or "alpha") or blend(90.23, 8298, 0.25)
acc = acc + blend(38.69, 57, math.sin(38.69 / 7)) * (1 + 3 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 54.38) end
acc = acc + blend(63.71, 29, math.sin(63.71 / 7)) * (1 + 1 / 31)
bag = { id = 1123, name = "warm-beta", pos = { 76.01, 1123.5, 152.02 }, ok = false }
if acc > 93.62 then acc = acc / 75 elseif acc < -93.62 then acc = -acc else acc = acc + 3812 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 51.71) end
bag = { id = 9943, name = "green-beta", pos = { 20.42, 9943.5, 40.84 }, ok = false }
r = (17.90 > 37) and (acc ~= 0 and "blue" or "delta") or blend(17.90, 7087, 0.25)
acc = acc + blend(62.46, 6, math.sin(62.46 / 7)) * (1 + 1 / 31)
r = (56.50 > 16) and (acc ~= 0 and "green" or "alpha") or blend(56.50, 2516, 0.25)
acc = acc + blend(74.47, 75, math.sin(74.47 / 7)) * (1 + 2 / 31)
for j = 1, 7 do acc = acc + (j * 6.29) % 97 / (j + 1) end
m = { ["k8847"] = { nested = { deeper = { value = "red", list = { 1, 2, 8847 } } } } }
if acc > 79.52 then acc = acc / 15 elseif acc < -79.52 then acc = -acc else acc = acc + 7490 end
m = { ["k9465"] = { nested = { deeper = { value = "fast", list = { 1, 2, 9465 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 16.15) end
m = { ["k1529"] = { nested = { deeper = { value = "cold", list = { 1, 2, 1529 } } } } }
for j = 1, 20 do acc = acc + (j * 48.22) % 97 / (j + 1) end
for j = 1, 4 do acc = acc + (j * 56.63) % 97 / (j + 1) end
for j = 1, 3 do acc = acc + (j * 99.76) % 97 / (j + 1) end
acc = acc + blend(95.59, 34, math.sin(95.59 / 7)) * (1 + 5 / 31)
m = { ["k5447"] = { nested = { deeper = { value = "red", list = { 1, 2, 5447 } } } } }
acc = acc + blend(97.56, 89, math.sin(97.56 / 7)) * (1 + 4 / 31)
q = "red {math.floor(52.21)} {string.rep('=', 5 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 70.34) end
for j = 1, 20 do acc = acc + (j * 48.86) % 97 / (j + 1) end
if acc > 21.56 then acc = acc / 69 elseif acc < -21.56 then acc = -acc else acc = acc + 335 end
m = { ["k8288"] = { nested = { deeper = { value = "warm", list = { 1, 2, 8288 } } } } }
m = { ["k8442"] = { nested = { deeper = { value = "blue", list = { 1, 2, 8442 } } } } }
for j = 1, 40 do acc = acc + (j * 8.85) % 97 / (j + 1) end
bag = { id = 4890, name = "green-delta", pos = { 55.20, 4890.5, 110.4 }, ok = true }
if acc > 47.97 then acc = acc / 57 elseif acc < -47.97 then acc = -acc else acc = acc + 7799 end
m = { ["k4454"] = { nested = { deeper = { value = "warm", list = { 1, 2, 4454 } } } } }
r = (47.72 > 44) and (acc ~= 0 and "blue" or "alpha") or blend(47.72, 9894, 0.25)
acc = acc + blend(0.97, 15, math.sin(0.97 / 7)) * (1 + 5 / 31)
r = (34.98 > 49) and (acc ~= 0 and "warm" or "gamma") or blend(34.98, 8899, 0.25)
r = (11.78 > 27) and (acc ~= 0 and "blue" or "delta") or blend(11.78, 6727, 0.25)
acc = acc + blend(13.04, 30, math.sin(13.04 / 7)) * (1 + 5 / 31)
acc = acc + blend(49.93, 50, math.sin(49.93 / 7)) * (1 + 4 / 31)
m = { ["k9979"] = { nested = { deeper = { value = "red", list = { 1, 2, 9979 } } } } }
acc = acc + blend(5.45, 72, math.sin(5.45 / 7)) * (1 + 1 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 62.86) end
if acc > 14.29 then acc = acc / 1 elseif acc < -14.29 then acc = -acc else acc = acc + 6675 end
q = "red {math.floor(8.69)} {string.rep('=', 3 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 94.64) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 75.75) end
if acc > 19.14 then acc = acc / 22 elseif acc < -19.14 then acc = -acc else acc = acc + 110 end
for j = 1, 21 do acc = acc + (j * 16.82) % 97 / (j + 1) end
acc = acc + blend(15.74, 92, math.sin(15.74 / 7)) * (1 + 2 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 6.16) end
r = (17.31 > 42) and (acc ~= 0 and "blue" or "alpha") or blend(17.31, 4542, 0.25)
bag = { id = 67, name = "fast-delta", pos = { 98.97, 67.5, 197.94 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 42.19) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 5.55) end
bag = { id = 6773, name = "fast-delta", pos = { 16.38, 6773.5, 32.76 }, ok = false }
m = { ["k2468"] = { nested = { deeper = { value = "warm", list = { 1, 2, 2468 } } } } }
for j = 1, 4 do acc = acc + (j * 24.55) % 97 / (j + 1) end
r = (54.57 > 35) and (acc ~= 0 and "fast" or "delta") or blend(54.57, 8335, 0.25)
bag = { id = 6629, name = "fast-alpha", pos = { 32.07, 6629.5, 64.14 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 49.11) end
acc = acc + blend(35.63, 41, math.sin(35.63 / 7)) * (1 + 12 / 31)
acc = acc + blend(76.56, 21, math.sin(76.56 / 7)) * (1 + 7 / 31)
for j = 1, 26 do acc = acc + (j * 6.83) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 10.21) end
acc = acc + blend(72.39, 40, math.sin(72.39 / 7)) * (1 + 9 / 31)
if acc > 27.70 then acc = acc / 83 elseif acc < -27.70 then acc = -acc else acc = acc + 7736 end
m = { ["k7977"] = { nested = { deeper = { value = "fast", list = { 1, 2, 7977 } } } } }
for j = 1, 39 do acc = acc + (j * 60.71) % 97 / (j + 1) end
q = "fast {math.floor(91.20)} {string.rep('=', 3 + 1)} gamma done"
m = { ["k3965"] = { nested = { deeper = { value = "cold", list = { 1, 2, 3965 } } } } }
m = { ["k2805"] = { nested = { deeper = { value = "blue", list = { 1, 2, 2805 } } } } }
bag = { id = 9331, name = "warm-alpha", pos = { 40.77, 9331.5, 81.54 }, ok = false }
for j = 1, 34 do acc = acc + (j * 77.85) % 97 / (j + 1) end
r = (71.55 > 21) and (acc ~= 0 and "blue" or "alpha") or blend(71.55, 7671, 0.25)
for j = 1, 1 do acc = acc + (j * 64.18) % 97 / (j + 1) end
bag = { id = 6388, name = "blue-gamma", pos = { 85.27, 6388.5, 170.54 }, ok = true }
r = (79.13 > 18) and (acc ~= 0 and "blue" or "alpha") or blend(79.13, 968, 0.25)
if acc > 6.90 then acc = acc / 86 elseif acc < -6.90 then acc = -acc else acc = acc + 7650 end
for j = 1, 7 do acc = acc + (j * 28.18) % 97 / (j + 1) end
acc = acc + blend(3.23, 3, math.sin(3.23 / 7)) * (1 + 5 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 47.22) end
m = { ["k2324"] = { nested = { deeper = { value = "fast", list = { 1, 2, 2324 } } } } }
acc = acc + blend(63.26, 92, math.sin(63.26 / 7)) * (1 + 2 / 31)
m = { ["k4845"] = { nested = { deeper = { value = "cold", list = { 1, 2, 4845 } } } } }
bag = { id = 54, name = "warm-beta", pos = { 73.83, 54.5, 147.66 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 74.64) end
for j = 1, 26 do acc = acc + (j * 44.78) % 97 / (j + 1) end
if acc > 29.71 then acc = acc / 42 elseif acc < -29.71 then acc = -acc else acc = acc + 7161 end
m = { ["k559"] = { nested = { deeper = { value = "green", list = { 1, 2, 559 } } } } }
acc = acc + blend(34.12, 68, math.sin(34.12 / 7)) * (1 + 12 / 31)
acc = acc + blend(66.03, 46, math.sin(66.03 / 7)) * (1 + 2 / 31)
for j = 1, 30 do acc = acc + (j * 72.22) % 97 / (j + 1) end
q = "warm {math.floor(35.34)} {string.rep('=', 0 + 1)} delta done"
q = "green {math.floor(42.98)} {string.rep('=', 4 + 1)} alpha done"
q = "warm {math.floor(85.78)} {string.rep('=', 5 + 1)} alpha done"
m = { ["k9679"] = { nested = { deeper = { value = "green", list = { 1, 2, 9679 } } } } }
bag = { id = 6145, name = "green-gamma", pos = { 17.34, 6145.5, 34.68 }, ok = false }
r = (18.59 > 25) and (acc ~= 0 and "warm" or "alpha") or blend(18.59, 5875, 0.25)
r = (17.73 > 16) and (acc ~= 0 and "fast" or "alpha") or blend(17.73, 9366, 0.25)
acc = acc + blend(99.07, 43, math.sin(99.07 / 7)) * (1 + 9 / 31)
bag = { id = 4005, name = "blue-delta", pos = { 22.01, 4005.5, 44.02 }, ok = false }
bag = { id = 200, name = "green-gamma", pos = { 73.94, 200.5, 147.88 }, ok = true }
bag = { id = 7135, name = "blue-alpha", pos = { 50.97, 7135.5, 101.94 }, ok = false }
q = "green {math.floor(54.15)} {string.rep('=', 3 + 1)} delta done"
r = (92.67 > 24) and (acc ~= 0 and "warm" or "gamma") or blend(92.67, 8274, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 16.61) end
acc = acc + blend(15.38, 89, math.sin(15.38 / 7)) * (1 + 10 / 31)
m = { ["k3712"] = { nested = { deeper = { value = "fast", list = { 1, 2, 3712 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 34.47) end
acc = acc + blend(22.82, 50, math.sin(22.82 / 7)) * (1 + 11 / 31)
bag = { id = 3373, name = "warm-delta", pos = { 31.14, 3373.5, 62.28 }, ok = false }
q = "cold {math.floor(14.01)} {string.rep('=', 6 + 1)} alpha done"
for j = 1, 26 do acc = acc + (j * 18.95) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 81.90) end
m = { ["k3247"] = { nested = { deeper = { value = "green", list = { 1, 2, 3247 } } } } }
acc = acc + blend(53.18, 48, math.sin(53.18 / 7)) * (1 + 3 / 31)
for j = 1, 29 do acc = acc + (j * 90.81) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 31.45) end
acc = acc + blend(72.23, 21, math.sin(72.23 / 7)) * (1 + 5 / 31)
q = "fast {math.floor(57.39)} {string.rep('=', 1 + 1)} alpha done"
if acc > 78.15 then acc = acc / 73 elseif acc < -78.15 then acc = -acc else acc = acc + 8705 end
for j = 1, 12 do acc = acc + (j * 28.59) % 97 / (j + 1) end
m = { ["k1605"] = { nested = { deeper = { value = "warm", list = { 1, 2, 1605 } } } } }
r = (48.85 > 45) and (acc ~= 0 and "green" or "delta") or blend(48.85, 4445, 0.25)
bag = { id = 4068, name = "cold-beta", pos = { 33.01, 4068.5, 66.02 }, ok = true }
m = { ["k7238"] = { nested = { deeper = { value = "cold", list = { 1, 2, 7238 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 20.95) end
q = "warm {math.floor(1.19)} {string.rep('=', 4 + 1)} gamma done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 53.57) end
if acc > 58.74 then acc = acc / 61 elseif acc < -58.74 then acc = -acc else acc = acc + 3531 end
acc = acc + blend(71.09, 58, math.sin(71.09 / 7)) * (1 + 7 / 31)
r = (22.49 > 14) and (acc ~= 0 and "cold" or "delta") or blend(22.49, 6514, 0.25)
acc = acc + blend(64.68, 54, math.sin(64.68 / 7)) * (1 + 7 / 31)
r = (30.27 > 44) and (acc ~= 0 and "green" or "alpha") or blend(30.27, 5244, 0.25)
acc = acc + blend(51.44, 51, math.sin(51.44 / 7)) * (1 + 5 / 31)
m = { ["k3320"] = { nested = { deeper = { value = "red", list = { 1, 2, 3320 } } } } }
if acc > 58.22 then acc = acc / 30 elseif acc < -58.22 then acc = -acc else acc = acc + 207 end
r = (96.14 > 38) and (acc ~= 0 and "blue" or "alpha") or blend(96.14, 9038, 0.25)
if acc > 16.91 then acc = acc / 4 elseif acc < -16.91 then acc = -acc else acc = acc + 2495 end
if acc > 72.55 then acc = acc / 76 elseif acc < -72.55 then acc = -acc else acc = acc + 1766 end
q = "blue {math.floor(19.96)} {string.rep('=', 6 + 1)} beta done"
acc = acc + blend(62.46, 2, math.sin(62.46 / 7)) * (1 + 0 / 31)
if acc > 73.33 then acc = acc / 51 elseif acc < -73.33 then acc = -acc else acc = acc + 9929 end
acc = acc + blend(89.36, 36, math.sin(89.36 / 7)) * (1 + 10 / 31)
acc = acc + blend(94.32, 82, math.sin(94.32 / 7)) * (1 + 0 / 31)
for j = 1, 40 do acc = acc + (j * 26.39) % 97 / (j + 1) end
acc = acc + blend(66.51, 8, math.sin(66.51 / 7)) * (1 + 11 / 31)
acc = acc + blend(3.02, 18, math.sin(3.02 / 7)) * (1 + 11 / 31)
acc = acc + blend(63.74, 17, math.sin(63.74 / 7)) * (1 + 8 / 31)
if acc > 68.43 then acc = acc / 87 elseif acc < -68.43 then acc = -acc else acc = acc + 3913 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 21.01) end
if acc > 89.62 then acc = acc / 88 elseif acc < -89.62 then acc = -acc else acc = acc + 6584 end
bag = { id = 8257, name = "cold-alpha", pos = { 44.15, 8257.5, 88.3 }, ok = false }
for j = 1, 1 do acc = acc + (j * 43.11) % 97 / (j + 1) end
if acc > 42.69 then acc = acc / 32 elseif acc < -42.69 then acc = -acc else acc = acc + 6528 end
q = "fast {math.floor(77.59)} {string.rep('=', 0 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 23.10) end
if acc > 38.62 then acc = acc / 19 elseif acc < -38.62 then acc = -acc else acc = acc + 5091 end
r = (24.08 > 38) and (acc ~= 0 and "warm" or "alpha") or blend(24.08, 788, 0.25)
acc = acc + blend(60.33, 75, math.sin(60.33 / 7)) * (1 + 9 / 31)
if acc > 92.13 then acc = acc / 35 elseif acc < -92.13 then acc = -acc else acc = acc + 123 end
if acc > 0.55 then acc = acc / 19 elseif acc < -0.55 then acc = -acc else acc = acc + 8206 end
q = "blue {math.floor(16.69)} {string.rep('=', 2 + 1)} alpha done"
if acc > 29.95 then acc = acc / 14 elseif acc < -29.95 then acc = -acc else acc = acc + 2238 end
q = "blue {math.floor(49.99)} {string.rep('=', 3 + 1)} delta done"
acc = acc + blend(40.15, 88, math.sin(40.15 / 7)) * (1 + 9 / 31)
q = "warm {math.floor(59.17)} {string.rep('=', 3 + 1)} alpha done"
r = (15.09 > 36) and (acc ~= 0 and "warm" or "delta") or blend(15.09, 3486, 0.25)
acc = acc + blend(78.13, 85, math.sin(78.13 / 7)) * (1 + 2 / 31)
q = "cold {math.floor(72.33)} {string.rep('=', 4 + 1)} gamma done"
r = (75.63 > 15) and (acc ~= 0 and "blue" or "beta") or blend(75.63, 65, 0.25)
if acc > 30.44 then acc = acc / 26 elseif acc < -30.44 then acc = -acc else acc = acc + 5365 end
bag = { id = 9140, name = "warm-gamma", pos = { 15.21, 9140.5, 30.42 }, ok = true }
acc = acc + blend(53.78, 62, math.sin(53.78 / 7)) * (1 + 0 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 82.91) end
for j = 1, 33 do acc = acc + (j * 71.04) % 97 / (j + 1) end
acc = acc + blend(82.03, 65, math.sin(82.03 / 7)) * (1 + 1 / 31)
if acc > 16.51 then acc = acc / 52 elseif acc < -16.51 then acc = -acc else acc = acc + 3878 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 71.10) end
acc = acc + blend(8.86, 64, math.sin(8.86 / 7)) * (1 + 3 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 33.57) end
for j = 1, 28 do acc = acc + (j * 17.08) % 97 / (j + 1) end
if acc > 46.20 then acc = acc / 53 elseif acc < -46.20 then acc = -acc else acc = acc + 5036 end
bag = { id = 8875, name = "cold-alpha", pos = { 74.81, 8875.5, 149.62 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 39.57) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 46.89) end
q = "warm {math.floor(49.07)} {string.rep('=', 1 + 1)} beta done"
for j = 1, 40 do acc = acc + (j * 33.61) % 97 / (j + 1) end
acc = acc + blend(20.36, 11, math.sin(20.36 / 7)) * (1 + 0 / 31)
m = { ["k5447"] = { nested = { deeper = { value = "cold", list = { 1, 2, 5447 } } } } }
acc = acc + blend(89.72, 74, math.sin(89.72 / 7)) * (1 + 9 / 31)
r = (84.76 > 46) and (acc ~= 0 and "cold" or "beta") or blend(84.76, 6446, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 89.38) end
m = { ["k5682"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5682 } } } } }
m = { ["k3927"] = { nested = { deeper = { value = "green", list = { 1, 2, 3927 } } } } }
bag = { id = 5092, name = "fast-delta", pos = { 47.00, 5092.5, 94 }, ok = true }
acc = acc + blend(79.02, 82, math.sin(79.02 / 7)) * (1 + 1 / 31)
m = { ["k2319"] = { nested = { deeper = { value = "cold", list = { 1, 2, 2319 } } } } }
q = "green {math.floor(20.67)} {string.rep('=', 3 + 1)} delta done"
if acc > 9.45 then acc = acc / 16 elseif acc < -9.45 then acc = -acc else acc = acc + 1172 end
q = "cold {math.floor(51.33)} {string.rep('=', 2 + 1)} beta done"
r = (71.39 > 13) and (acc ~= 0 and "fast" or "beta") or blend(71.39, 5063, 0.25)
r = (56.19 > 9) and (acc ~= 0 and "warm" or "gamma") or blend(56.19, 2709, 0.25)
acc = acc + blend(56.90, 31, math.sin(56.90 / 7)) * (1 + 5 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 20.12) end
m = { ["k3819"] = { nested = { deeper = { value = "cold", list = { 1, 2, 3819 } } } } }
r = (70.82 > 10) and (acc ~= 0 and "warm" or "alpha") or blend(70.82, 5160, 0.25)
bag = { id = 5621, name = "fast-beta", pos = { 2.37, 5621.5, 4.74 }, ok = false }
for j = 1, 8 do acc = acc + (j * 36.10) % 97 / (j + 1) end
bag = { id = 9854, name = "cold-gamma", pos = { 0.78, 9854.5, 1.56 }, ok = true }
if acc > 55.55 then acc = acc / 49 elseif acc < -55.55 then acc = -acc else acc = acc + 2807 end
bag = { id = 1086, name = "red-alpha", pos = { 84.53, 1086.5, 169.06 }, ok = true }
for j = 1, 23 do acc = acc + (j * 11.53) % 97 / (j + 1) end
r = (47.98 > 4) and (acc ~= 0 and "fast" or "gamma") or blend(47.98, 3654, 0.25)
m = { ["k5968"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5968 } } } } }
bag = { id = 2979, name = "green-beta", pos = { 12.91, 2979.5, 25.82 }, ok = false }
m = { ["k9650"] = { nested = { deeper = { value = "red", list = { 1, 2, 9650 } } } } }
for j = 1, 4 do acc = acc + (j * 73.19) % 97 / (j + 1) end
r = (6.31 > 25) and (acc ~= 0 and "blue" or "beta") or blend(6.31, 5525, 0.25)
bag = { id = 311, name = "green-delta", pos = { 68.63, 311.5, 137.26 }, ok = false }
bag = { id = 5014, name = "blue-delta", pos = { 59.85, 5014.5, 119.7 }, ok = true }
q = "red {math.floor(19.67)} {string.rep('=', 6 + 1)} beta done"
bag = { id = 6009, name = "warm-beta", pos = { 95.21, 6009.5, 190.42 }, ok = false }
r = (93.59 > 39) and (acc ~= 0 and "red" or "delta") or blend(93.59, 6439, 0.25)
r = (46.98 > 2) and (acc ~= 0 and "red" or "alpha") or blend(46.98, 1902, 0.25)
q = "red {math.floor(69.08)} {string.rep('=', 2 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 18.25) end
acc = acc + blend(25.76, 20, math.sin(25.76 / 7)) * (1 + 9 / 31)
m = { ["k7490"] = { nested = { deeper = { value = "red", list = { 1, 2, 7490 } } } } }
if acc > 63.72 then acc = acc / 66 elseif acc < -63.72 then acc = -acc else acc = acc + 1578 end
for j = 1, 3 do acc = acc + (j * 15.24) % 97 / (j + 1) end
acc = acc + blend(63.54, 71, math.sin(63.54 / 7)) * (1 + 4 / 31)
m = { ["k516"] = { nested = { deeper = { value = "blue", list = { 1, 2, 516 } } } } }
m = { ["k6817"] = { nested = { deeper = { value = "green", list = { 1, 2, 6817 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 81.07) end
for j = 1, 3 do acc = acc + (j * 66.92) % 97 / (j + 1) end
acc = acc + blend(54.93, 88, math.sin(54.93 / 7)) * (1 + 2 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 48.80) end
bag = { id = 3709, name = "fast-beta", pos = { 30.82, 3709.5, 61.64 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 28.24) end
for j = 1, 16 do acc = acc + (j * 55.86) % 97 / (j + 1) end
m = { ["k2834"] = { nested = { deeper = { value = "green", list = { 1, 2, 2834 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 63.03) end
for j = 1, 33 do acc = acc + (j * 70.42) % 97 / (j + 1) end
m = { ["k3611"] = { nested = { deeper = { value = "cold", list = { 1, 2, 3611 } } } } }
bag = { id = 4368, name = "green-delta", pos = { 70.74, 4368.5, 141.48 }, ok = true }
for j = 1, 34 do acc = acc + (j * 10.25) % 97 / (j + 1) end
if acc > 55.45 then acc = acc / 77 elseif acc < -55.45 then acc = -acc else acc = acc + 254 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 0.32) end
m = { ["k1938"] = { nested = { deeper = { value = "green", list = { 1, 2, 1938 } } } } }
m = { ["k6779"] = { nested = { deeper = { value = "green", list = { 1, 2, 6779 } } } } }
r = (10.93 > 36) and (acc ~= 0 and "warm" or "gamma") or blend(10.93, 7486, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 96.98) end
m = { ["k7194"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7194 } } } } }
m = { ["k9770"] = { nested = { deeper = { value = "green", list = { 1, 2, 9770 } } } } }
bag = { id = 4009, name = "cold-beta", pos = { 7.48, 4009.5, 14.96 }, ok = false }
bag = { id = 1407, name = "fast-alpha", pos = { 11.92, 1407.5, 23.84 }, ok = false }
m = { ["k3574"] = { nested = { deeper = { value = "green", list = { 1, 2, 3574 } } } } }
m = { ["k5382"] = { nested = { deeper = { value = "green", list = { 1, 2, 5382 } } } } }
q = "green {math.floor(44.89)} {string.rep('=', 6 + 1)} delta done"
bag = { id = 5616, name = "red-gamma", pos = { 24.15, 5616.5, 48.3 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 46.68) end
q = "blue {math.floor(55.01)} {string.rep('=', 2 + 1)} alpha done"
for j = 1, 26 do acc = acc + (j * 29.68) % 97 / (j + 1) end
r = (29.92 > 42) and (acc ~= 0 and "warm" or "gamma") or blend(29.92, 6792, 0.25)
if acc > 63.40 then acc = acc / 19 elseif acc < -63.40 then acc = -acc else acc = acc + 6693 end
if acc > 30.56 then acc = acc / 66 elseif acc < -30.56 then acc = -acc else acc = acc + 2735 end
m = { ["k567"] = { nested = { deeper = { value = "cold", list = { 1, 2, 567 } } } } }
for j = 1, 6 do acc = acc + (j * 60.17) % 97 / (j + 1) end
q = "red {math.floor(53.79)} {string.rep('=', 1 + 1)} delta done"
q = "fast {math.floor(39.48)} {string.rep('=', 0 + 1)} beta done"
for j = 1, 28 do acc = acc + (j * 95.09) % 97 / (j + 1) end
q = "cold {math.floor(85.94)} {string.rep('=', 4 + 1)} delta done"
acc = acc + blend(16.01, 54, math.sin(16.01 / 7)) * (1 + 6 / 31)
for j = 1, 7 do acc = acc + (j * 44.53) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 33.48) end
m = { ["k4957"] = { nested = { deeper = { value = "cold", list = { 1, 2, 4957 } } } } }
if acc > 34.08 then acc = acc / 64 elseif acc < -34.08 then acc = -acc else acc = acc + 2288 end
bag = { id = 1810, name = "red-alpha", pos = { 23.36, 1810.5, 46.72 }, ok = true }
m = { ["k9088"] = { nested = { deeper = { value = "cold", list = { 1, 2, 9088 } } } } }
acc = acc + blend(94.78, 50, math.sin(94.78 / 7)) * (1 + 12 / 31)
bag = { id = 1133, name = "cold-alpha", pos = { 46.71, 1133.5, 93.42 }, ok = false }
if acc > 36.68 then acc = acc / 85 elseif acc < -36.68 then acc = -acc else acc = acc + 9251 end
q = "green {math.floor(65.35)} {string.rep('=', 2 + 1)} alpha done"
for j = 1, 12 do acc = acc + (j * 7.42) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 35.08) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 49.26) end
m = { ["k2430"] = { nested = { deeper = { value = "cold", list = { 1, 2, 2430 } } } } }
q = "fast {math.floor(76.51)} {string.rep('=', 2 + 1)} delta done"
acc = acc + blend(24.69, 74, math.sin(24.69 / 7)) * (1 + 6 / 31)
q = "red {math.floor(67.83)} {string.rep('=', 3 + 1)} alpha done"
acc = acc + blend(9.63, 13, math.sin(9.63 / 7)) * (1 + 5 / 31)
bag = { id = 7546, name = "cold-delta", pos = { 93.99, 7546.5, 187.98 }, ok = true }
for j = 1, 29 do acc = acc + (j * 21.70) % 97 / (j + 1) end
for j = 1, 24 do acc = acc + (j * 84.92) % 97 / (j + 1) end
bag = { id = 7933, name = "cold-beta", pos = { 11.72, 7933.5, 23.44 }, ok = false }
r = (9.75 > 22) and (acc ~= 0 and "cold" or "gamma") or blend(9.75, 9922, 0.25)
acc = acc + blend(29.13, 57, math.sin(29.13 / 7)) * (1 + 3 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 42.75) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 80.50) end
acc = acc + blend(96.69, 36, math.sin(96.69 / 7)) * (1 + 12 / 31)
bag = { id = 2925, name = "fast-delta", pos = { 13.06, 2925.5, 26.12 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 56.20) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 50.95) end
m = { ["k8224"] = { nested = { deeper = { value = "red", list = { 1, 2, 8224 } } } } }
bag = { id = 4022, name = "blue-gamma", pos = { 14.30, 4022.5, 28.6 }, ok = true }
if acc > 58.85 then acc = acc / 33 elseif acc < -58.85 then acc = -acc else acc = acc + 1456 end
acc = acc + blend(67.23, 68, math.sin(67.23 / 7)) * (1 + 4 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 61.31) end
r = (25.74 > 21) and (acc ~= 0 and "fast" or "beta") or blend(25.74, 871, 0.25)
m = { ["k2297"] = { nested = { deeper = { value = "warm", list = { 1, 2, 2297 } } } } }
if acc > 17.77 then acc = acc / 12 elseif acc < -17.77 then acc = -acc else acc = acc + 4906 end
for j = 1, 31 do acc = acc + (j * 76.90) % 97 / (j + 1) end
q = "red {math.floor(80.78)} {string.rep('=', 5 + 1)} delta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 36.01) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 13.31) end
acc = acc + blend(87.20, 33, math.sin(87.20 / 7)) * (1 + 12 / 31)
for j = 1, 13 do acc = acc + (j * 23.11) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 66.59) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 40.96) end
if acc > 78.34 then acc = acc / 86 elseif acc < -78.34 then acc = -acc else acc = acc + 1954 end
m = { ["k9091"] = { nested = { deeper = { value = "cold", list = { 1, 2, 9091 } } } } }
q = "warm {math.floor(27.74)} {string.rep('=', 6 + 1)} gamma done"
m = { ["k8199"] = { nested = { deeper = { value = "warm", list = { 1, 2, 8199 } } } } }
m = { ["k7193"] = { nested = { deeper = { value = "blue", list = { 1, 2, 7193 } } } } }
acc = acc + blend(71.07, 78, math.sin(71.07 / 7)) * (1 + 3 / 31)
r = (82.69 > 13) and (acc ~= 0 and "blue" or "delta") or blend(82.69, 8913, 0.25)
r = (65.11 > 40) and (acc ~= 0 and "blue" or "beta") or blend(65.11, 9440, 0.25)
acc = acc + blend(91.76, 55, math.sin(91.76 / 7)) * (1 + 11 / 31)
bag = { id = 7375, name = "fast-delta", pos = { 63.30, 7375.5, 126.6 }, ok = false }
if acc > 13.55 then acc = acc / 58 elseif acc < -13.55 then acc = -acc else acc = acc + 3172 end
bag = { id = 2291, name = "red-beta", pos = { 96.87, 2291.5, 193.74 }, ok = false }
q = "green {math.floor(45.67)} {string.rep('=', 1 + 1)} delta done"
acc = acc + blend(99.92, 14, math.sin(99.92 / 7)) * (1 + 5 / 31)
acc = acc + blend(23.17, 23, math.sin(23.17 / 7)) * (1 + 7 / 31)
r = (45.37 > 40) and (acc ~= 0 and "cold" or "beta") or blend(45.37, 1940, 0.25)
m = { ["k9521"] = { nested = { deeper = { value = "red", list = { 1, 2, 9521 } } } } }
for j = 1, 7 do acc = acc + (j * 41.34) % 97 / (j + 1) end
for j = 1, 37 do acc = acc + (j * 72.61) % 97 / (j + 1) end
bag = { id = 129, name = "warm-delta", pos = { 91.76, 129.5, 183.52 }, ok = false }
r = (90.47 > 18) and (acc ~= 0 and "green" or "delta") or blend(90.47, 1518, 0.25)
q = "cold {math.floor(72.54)} {string.rep('=', 2 + 1)} alpha done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 43.43) end
q = "cold {math.floor(98.41)} {string.rep('=', 4 + 1)} delta done"
bag = { id = 6843, name = "red-gamma", pos = { 49.53, 6843.5, 99.06 }, ok = false }
bag = { id = 9519, name = "red-alpha", pos = { 11.05, 9519.5, 22.1 }, ok = false }
q = "blue {math.floor(38.59)} {string.rep('=', 0 + 1)} gamma done"
acc = acc + blend(12.78, 53, math.sin(12.78 / 7)) * (1 + 6 / 31)
if acc > 52.62 then acc = acc / 63 elseif acc < -52.62 then acc = -acc else acc = acc + 2198 end
acc = acc + blend(58.77, 8, math.sin(58.77 / 7)) * (1 + 9 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 61.33) end
q = "fast {math.floor(9.68)} {string.rep('=', 3 + 1)} delta done"
bag = { id = 3439, name = "cold-delta", pos = { 0.35, 3439.5, 0.7 }, ok = false }
m = { ["k7154"] = { nested = { deeper = { value = "green", list = { 1, 2, 7154 } } } } }
acc = acc + blend(35.83, 55, math.sin(35.83 / 7)) * (1 + 12 / 31)
acc = acc + blend(29.73, 23, math.sin(29.73 / 7)) * (1 + 2 / 31)
r = (39.89 > 25) and (acc ~= 0 and "warm" or "alpha") or blend(39.89, 9875, 0.25)
if acc > 16.14 then acc = acc / 83 elseif acc < -16.14 then acc = -acc else acc = acc + 883 end
q = "fast {math.floor(58.91)} {string.rep('=', 1 + 1)} delta done"
m = { ["k5495"] = { nested = { deeper = { value = "fast", list = { 1, 2, 5495 } } } } }
for j = 1, 6 do acc = acc + (j * 51.43) % 97 / (j + 1) end
bag = { id = 4669, name = "green-alpha", pos = { 12.44, 4669.5, 24.88 }, ok = false }
for j = 1, 24 do acc = acc + (j * 96.69) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 22.44) end
if acc > 8.57 then acc = acc / 7 elseif acc < -8.57 then acc = -acc else acc = acc + 9084 end
r = (76.74 > 24) and (acc ~= 0 and "green" or "gamma") or blend(76.74, 374, 0.25)
for j = 1, 12 do acc = acc + (j * 65.40) % 97 / (j + 1) end
q = "warm {math.floor(29.49)} {string.rep('=', 5 + 1)} alpha done"
bag = { id = 8027, name = "cold-delta", pos = { 64.25, 8027.5, 128.5 }, ok = false }
r = (15.95 > 2) and (acc ~= 0 and "warm" or "delta") or blend(15.95, 8702, 0.25)
q = "fast {math.floor(60.23)} {string.rep('=', 4 + 1)} delta done"
bag = { id = 6445, name = "warm-beta", pos = { 53.70, 6445.5, 107.4 }, ok = false }
m = { ["k2371"] = { nested = { deeper = { value = "fast", list = { 1, 2, 2371 } } } } }
acc = acc + blend(40.36, 5, math.sin(40.36 / 7)) * (1 + 4 / 31)
r = (21.51 > 25) and (acc ~= 0 and "blue" or "delta") or blend(21.51, 2275, 0.25)
r = (5.23 > 27) and (acc ~= 0 and "blue" or "delta") or blend(5.23, 4327, 0.25)
if acc > 10.83 then acc = acc / 49 elseif acc < -10.83 then acc = -acc else acc = acc + 4142 end
for j = 1, 40 do acc = acc + (j * 91.10) % 97 / (j + 1) end
r = (52.05 > 9) and (acc ~= 0 and "warm" or "alpha") or blend(52.05, 4559, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 98.29) end
bag = { id = 8670, name = "fast-beta", pos = { 40.32, 8670.5, 80.64 }, ok = true }
m = { ["k3216"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3216 } } } } }
acc = acc + blend(24.89, 69, math.sin(24.89 / 7)) * (1 + 6 / 31)
for j = 1, 34 do acc = acc + (j * 81.76) % 97 / (j + 1) end
q = "green {math.floor(70.76)} {string.rep('=', 4 + 1)} alpha done"
q = "warm {math.floor(94.87)} {string.rep('=', 4 + 1)} beta done"
m = { ["k1672"] = { nested = { deeper = { value = "cold", list = { 1, 2, 1672 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 28.36) end
acc = acc + blend(91.56, 87, math.sin(91.56 / 7)) * (1 + 4 / 31)
bag = { id = 9973, name = "blue-gamma", pos = { 17.33, 9973.5, 34.66 }, ok = false }
bag = { id = 1305, name = "fast-alpha", pos = { 56.93, 1305.5, 113.86 }, ok = false }
if acc > 65.44 then acc = acc / 11 elseif acc < -65.44 then acc = -acc else acc = acc + 6596 end
m = { ["k1157"] = { nested = { deeper = { value = "red", list = { 1, 2, 1157 } } } } }
q = "blue {math.floor(87.18)} {string.rep('=', 6 + 1)} gamma done"
acc = acc + blend(3.70, 31, math.sin(3.70 / 7)) * (1 + 1 / 31)
m = { ["k1931"] = { nested = { deeper = { value = "blue", list = { 1, 2, 1931 } } } } }
m = { ["k2673"] = { nested = { deeper = { value = "blue", list = { 1, 2, 2673 } } } } }
for j = 1, 24 do acc = acc + (j * 94.93) % 97 / (j + 1) end
q = "fast {math.floor(90.73)} {string.rep('=', 2 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 56.02) end
if acc > 88.31 then acc = acc / 4 elseif acc < -88.31 then acc = -acc else acc = acc + 4275 end
acc = acc + blend(70.99, 20, math.sin(70.99 / 7)) * (1 + 3 / 31)
m = { ["k3743"] = { nested = { deeper = { value = "fast", list = { 1, 2, 3743 } } } } }
bag = { id = 4671, name = "cold-gamma", pos = { 26.27, 4671.5, 52.54 }, ok = false }
m = { ["k4373"] = { nested = { deeper = { value = "cold", list = { 1, 2, 4373 } } } } }
m = { ["k6900"] = { nested = { deeper = { value = "blue", list = { 1, 2, 6900 } } } } }
q = "cold {math.floor(80.39)} {string.rep('=', 6 + 1)} delta done"
m = { ["k3930"] = { nested = { deeper = { value = "warm", list = { 1, 2, 3930 } } } } }
acc = acc + blend(54.25, 68, math.sin(54.25 / 7)) * (1 + 7 / 31)
if acc > 38.57 then acc = acc / 17 elseif acc < -38.57 then acc = -acc else acc = acc + 2597 end
for j = 1, 30 do acc = acc + (j * 41.83) % 97 / (j + 1) end
acc = acc + blend(80.94, 77, math.sin(80.94 / 7)) * (1 + 7 / 31)
r = (7.26 > 13) and (acc ~= 0 and "warm" or "beta") or blend(7.26, 8213, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 89.66) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 31.51) end
acc = acc + blend(85.67, 5, math.sin(85.67 / 7)) * (1 + 12 / 31)
if acc > 63.13 then acc = acc / 58 elseif acc < -63.13 then acc = -acc else acc = acc + 6732 end
q = "warm {math.floor(24.05)} {string.rep('=', 6 + 1)} alpha done"
m = { ["k3223"] = { nested = { deeper = { value = "warm", list = { 1, 2, 3223 } } } } }
acc = acc + blend(59.18, 16, math.sin(59.18 / 7)) * (1 + 3 / 31)
q = "blue {math.floor(63.31)} {string.rep('=', 6 + 1)} delta done"
if acc > 74.10 then acc = acc / 65 elseif acc < -74.10 then acc = -acc else acc = acc + 7985 end
bag = { id = 6538, name = "red-alpha", pos = { 52.94, 6538.5, 105.88 }, ok = true }
for j = 1, 36 do acc = acc + (j * 81.07) % 97 / (j + 1) end
bag = { id = 1956, name = "warm-beta", pos = { 82.35, 1956.5, 164.7 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 62.18) end
q = "cold {math.floor(3.94)} {string.rep('=', 6 + 1)} delta done"
r = (69.02 > 13) and (acc ~= 0 and "warm" or "gamma") or blend(69.02, 7013, 0.25)
acc = acc + blend(83.63, 20, math.sin(83.63 / 7)) * (1 + 9 / 31)
if acc > 95.00 then acc = acc / 78 elseif acc < -95.00 then acc = -acc else acc = acc + 7286 end
acc = acc + blend(89.78, 18, math.sin(89.78 / 7)) * (1 + 5 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 79.21) end
q = "blue {math.floor(25.58)} {string.rep('=', 4 + 1)} beta done"
if acc > 53.41 then acc = acc / 13 elseif acc < -53.41 then acc = -acc else acc = acc + 635 end
for j = 1, 38 do acc = acc + (j * 93.84) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 9.23) end
bag = { id = 8860, name = "green-gamma", pos = { 94.94, 8860.5, 189.88 }, ok = true }
m = { ["k8343"] = { nested = { deeper = { value = "fast", list = { 1, 2, 8343 } } } } }
if acc > 37.66 then acc = acc / 41 elseif acc < -37.66 then acc = -acc else acc = acc + 7071 end
r = (86.28 > 12) and (acc ~= 0 and "blue" or "gamma") or blend(86.28, 2512, 0.25)
r = (68.59 > 14) and (acc ~= 0 and "cold" or "alpha") or blend(68.59, 8364, 0.25)
bag = { id = 2459, name = "blue-beta", pos = { 35.16, 2459.5, 70.32 }, ok = false }
for j = 1, 6 do acc = acc + (j * 31.18) % 97 / (j + 1) end
m = { ["k3005"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3005 } } } } }
if acc > 30.68 then acc = acc / 40 elseif acc < -30.68 then acc = -acc else acc = acc + 8138 end
bag = { id = 4699, name = "blue-gamma", pos = { 15.87, 4699.5, 31.74 }, ok = false }
m = { ["k9064"] = { nested = { deeper = { value = "warm", list = { 1, 2, 9064 } } } } }
acc = acc + blend(51.47, 51, math.sin(51.47 / 7)) * (1 + 11 / 31)
bag = { id = 2420, name = "green-beta", pos = { 96.32, 2420.5, 192.64 }, ok = true }
r = (59.34 > 42) and (acc ~= 0 and "green" or "beta") or blend(59.34, 3042, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 21.52) end
for j = 1, 24 do acc = acc + (j * 60.23) % 97 / (j + 1) end
q = "red {math.floor(18.73)} {string.rep('=', 2 + 1)} alpha done"
m = { ["k4389"] = { nested = { deeper = { value = "red", list = { 1, 2, 4389 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 55.94) end
if acc > 5.40 then acc = acc / 25 elseif acc < -5.40 then acc = -acc else acc = acc + 1092 end
m = { ["k4268"] = { nested = { deeper = { value = "cold", list = { 1, 2, 4268 } } } } }
q = "cold {math.floor(76.17)} {string.rep('=', 4 + 1)} beta done"
r = (24.55 > 5) and (acc ~= 0 and "fast" or "gamma") or blend(24.55, 155, 0.25)
m = { ["k9888"] = { nested = { deeper = { value = "warm", list = { 1, 2, 9888 } } } } }
m = { ["k6889"] = { nested = { deeper = { value = "blue", list = { 1, 2, 6889 } } } } }
bag = { id = 438, name = "red-alpha", pos = { 6.49, 438.5, 12.98 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 36.01) end
for j = 1, 31 do acc = acc + (j * 54.88) % 97 / (j + 1) end
for j = 1, 23 do acc = acc + (j * 92.59) % 97 / (j + 1) end
q = "red {math.floor(24.21)} {string.rep('=', 3 + 1)} delta done"
for j = 1, 37 do acc = acc + (j * 27.73) % 97 / (j + 1) end
m = { ["k7678"] = { nested = { deeper = { value = "cold", list = { 1, 2, 7678 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 80.19) end
m = { ["k9460"] = { nested = { deeper = { value = "green", list = { 1, 2, 9460 } } } } }
q = "fast {math.floor(43.72)} {string.rep('=', 0 + 1)} beta done"
q = "blue {math.floor(33.88)} {string.rep('=', 6 + 1)} delta done"
m = { ["k3777"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3777 } } } } }
if acc > 26.63 then acc = acc / 68 elseif acc < -26.63 then acc = -acc else acc = acc + 1936 end
m = { ["k3504"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3504 } } } } }
acc = acc + blend(64.83, 85, math.sin(64.83 / 7)) * (1 + 6 / 31)
if acc > 8.16 then acc = acc / 27 elseif acc < -8.16 then acc = -acc else acc = acc + 2162 end
q = "fast {math.floor(51.68)} {string.rep('=', 4 + 1)} delta done"
for j = 1, 28 do acc = acc + (j * 84.91) % 97 / (j + 1) end
for j = 1, 31 do acc = acc + (j * 10.74) % 97 / (j + 1) end
if acc > 34.49 then acc = acc / 4 elseif acc < -34.49 then acc = -acc else acc = acc + 4898 end
if acc > 93.78 then acc = acc / 33 elseif acc < -93.78 then acc = -acc else acc = acc + 2346 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 33.75) end
m = { ["k6866"] = { nested = { deeper = { value = "red", list = { 1, 2, 6866 } } } } }
m = { ["k6550"] = { nested = { deeper = { value = "fast", list = { 1, 2, 6550 } } } } }
if acc > 47.63 then acc = acc / 9 elseif acc < -47.63 then acc = -acc else acc = acc + 9442 end
q = "blue {math.floor(7.74)} {string.rep('=', 5 + 1)} gamma done"
m = { ["k641"] = { nested = { deeper = { value = "blue", list = { 1, 2, 641 } } } } }
if acc > 12.78 then acc = acc / 23 elseif acc < -12.78 then acc = -acc else acc = acc + 8744 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 31.96) end
for j = 1, 31 do acc = acc + (j * 50.52) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 21.51) end
for j = 1, 32 do acc = acc + (j * 39.93) % 97 / (j + 1) end
bag = { id = 2891, name = "warm-delta", pos = { 48.03, 2891.5, 96.06 }, ok = false }
if acc > 85.68 then acc = acc / 78 elseif acc < -85.68 then acc = -acc else acc = acc + 8176 end
bag = { id = 8190, name = "fast-delta", pos = { 40.43, 8190.5, 80.86 }, ok = true }
bag = { id = 8400, name = "green-alpha", pos = { 51.17, 8400.5, 102.34 }, ok = true }
q = "blue {math.floor(58.44)} {string.rep('=', 3 + 1)} gamma done"
m = { ["k6874"] = { nested = { deeper = { value = "fast", list = { 1, 2, 6874 } } } } }
m = { ["k4200"] = { nested = { deeper = { value = "red", list = { 1, 2, 4200 } } } } }
acc = acc + blend(38.81, 89, math.sin(38.81 / 7)) * (1 + 11 / 31)
for j = 1, 37 do acc = acc + (j * 6.70) % 97 / (j + 1) end
m = { ["k5250"] = { nested = { deeper = { value = "blue", list = { 1, 2, 5250 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 54.39) end
r = (41.71 > 42) and (acc ~= 0 and "red" or "alpha") or blend(41.71, 6892, 0.25)
acc = acc + blend(22.82, 71, math.sin(22.82 / 7)) * (1 + 3 / 31)
bag = { id = 4708, name = "warm-gamma", pos = { 90.68, 4708.5, 181.36 }, ok = true }
for j = 1, 22 do acc = acc + (j * 71.09) % 97 / (j + 1) end
r = (57.58 > 7) and (acc ~= 0 and "warm" or "gamma") or blend(57.58, 407, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 85.23) end
m = { ["k6458"] = { nested = { deeper = { value = "blue", list = { 1, 2, 6458 } } } } }
q = "red {math.floor(96.67)} {string.rep('=', 2 + 1)} beta done"
bag = { id = 3373, name = "cold-gamma", pos = { 87.13, 3373.5, 174.26 }, ok = false }
m = { ["k7745"] = { nested = { deeper = { value = "warm", list = { 1, 2, 7745 } } } } }
q = "warm {math.floor(38.94)} {string.rep('=', 4 + 1)} alpha done"
for j = 1, 31 do acc = acc + (j * 58.06) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 86.45) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 58.64) end
acc = acc + blend(82.95, 83, math.sin(82.95 / 7)) * (1 + 10 / 31)
bag = { id = 7545, name = "green-delta", pos = { 84.67, 7545.5, 169.34 }, ok = false }
m = { ["k9939"] = { nested = { deeper = { value = "blue", list = { 1, 2, 9939 } } } } }
if acc > 96.39 then acc = acc / 17 elseif acc < -96.39 then acc = -acc else acc = acc + 5712 end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 24.62) end
if acc > 91.58 then acc = acc / 75 elseif acc < -91.58 then acc = -acc else acc = acc + 9063 end
m = { ["k5949"] = { nested = { deeper = { value = "blue", list = { 1, 2, 5949 } } } } }
r = (27.29 > 5) and (acc ~= 0 and "green" or "alpha") or blend(27.29, 4005, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 77.86) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 94.79) end
m = { ["k9077"] = { nested = { deeper = { value = "cold", list = { 1, 2, 9077 } } } } }
for j = 1, 22 do acc = acc + (j * 45.24) % 97 / (j + 1) end
if acc > 58.09 then acc = acc / 76 elseif acc < -58.09 then acc = -acc else acc = acc + 6483 end
m = { ["k6113"] = { nested = { deeper = { value = "red", list = { 1, 2, 6113 } } } } }
r = (85.19 > 14) and (acc ~= 0 and "cold" or "beta") or blend(85.19, 1664, 0.25)
q = "red {math.floor(35.05)} {string.rep('=', 4 + 1)} gamma done"
bag = { id = 2405, name = "warm-alpha", pos = { 67.89, 2405.5, 135.78 }, ok = false }
r = (63.37 > 12) and (acc ~= 0 and "fast" or "alpha") or blend(63.37, 3062, 0.25)
bag = { id = 1722, name = "green-delta", pos = { 54.61, 1722.5, 109.22 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 97.60) end
acc = acc + blend(96.41, 52, math.sin(96.41 / 7)) * (1 + 9 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 84.50) end
acc = acc + blend(17.14, 90, math.sin(17.14 / 7)) * (1 + 3 / 31)
if acc > 93.75 then acc = acc / 10 elseif acc < -93.75 then acc = -acc else acc = acc + 1611 end
if acc > 71.62 then acc = acc / 25 elseif acc < -71.62 then acc = -acc else acc = acc + 4919 end
m = { ["k3461"] = { nested = { deeper = { value = "warm", list = { 1, 2, 3461 } } } } }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 9.05) end
bag = { id = 6114, name = "warm-beta", pos = { 2.54, 6114.5, 5.08 }, ok = true }
acc = acc + blend(29.49, 88, math.sin(29.49 / 7)) * (1 + 11 / 31)
acc = acc + blend(63.74, 34, math.sin(63.74 / 7)) * (1 + 10 / 31)
for j = 1, 24 do acc = acc + (j * 77.50) % 97 / (j + 1) end
bag = { id = 421, name = "red-gamma", pos = { 13.76, 421.5, 27.52 }, ok = false }
q = "fast {math.floor(89.02)} {string.rep('=', 5 + 1)} alpha done"
acc = acc + blend(20.80, 90, math.sin(20.80 / 7)) * (1 + 0 / 31)
m = { ["k6319"] = { nested = { deeper = { value = "warm", list = { 1, 2, 6319 } } } } }
r = (0.38 > 49) and (acc ~= 0 and "red" or "gamma") or blend(0.38, 1899, 0.25)
q = "red {math.floor(47.47)} {string.rep('=', 1 + 1)} alpha done"
q = "blue {math.floor(6.17)} {string.rep('=', 6 + 1)} beta done"
m = { ["k2115"] = { nested = { deeper = { value = "cold", list = { 1, 2, 2115 } } } } }
r = (41.00 > 16) and (acc ~= 0 and "fast" or "beta") or blend(41.00, 8916, 0.25)
m = { ["k5200"] = { nested = { deeper = { value = "blue", list = { 1, 2, 5200 } } } } }
m = { ["k9354"] = { nested = { deeper = { value = "red", list = { 1, 2, 9354 } } } } }
acc = acc + blend(16.90, 71, math.sin(16.90 / 7)) * (1 + 0 / 31)
r = (87.77 > 45) and (acc ~= 0 and "warm" or "delta") or blend(87.77, 3795, 0.25)
r = (89.34 > 38) and (acc ~= 0 and "red" or "beta") or blend(89.34, 1188, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 70.02) end
q = "cold {math.floor(3.99)} {string.rep('=', 2 + 1)} beta done"
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 0.03) end
q = "red {math.floor(50.86)} {string.rep('=', 6 + 1)} beta done"
r = (28.00 > 46) and (acc ~= 0 and "green" or "gamma") or blend(28.00, 3646, 0.25)
r = (46.67 > 31) and (acc ~= 0 and "cold" or "delta") or blend(46.67, 6181, 0.25)
bag = { id = 5948, name = "green-gamma", pos = { 88.93, 5948.5, 177.86 }, ok = true }
if acc > 72.85 then acc = acc / 25 elseif acc < -72.85 then acc = -acc else acc = acc + 2160 end
r = (98.96 > 18) and (acc ~= 0 and "warm" or "delta") or blend(98.96, 3868, 0.25)
r = (45.34 > 36) and (acc ~= 0 and "cold" or "delta") or blend(45.34, 2086, 0.25)
m = { ["k6373"] = { nested = { deeper = { value = "fast", list = { 1, 2, 6373 } } } } }
for j = 1, 27 do acc = acc + (j * 5.43) % 97 / (j + 1) end
for j = 1, 7 do acc = acc + (j * 88.94) % 97 / (j + 1) end
bag = { id = 9663, name = "green-gamma", pos = { 0.34, 9663.5, 0.68 }, ok = false }
if acc > 33.22 then acc = acc / 33 elseif acc < -33.22 then acc = -acc else acc = acc + 2613 end
q = "warm {math.floor(5.60)} {string.rep('=', 2 + 1)} beta done"
m = { ["k3186"] = { nested = { deeper = { value = "blue", list = { 1, 2, 3186 } } } } }
for j = 1, 40 do acc = acc + (j * 9.67) % 97 / (j + 1) end
bag = { id = 762, name = "cold-alpha", pos = { 7.79, 762.5, 15.58 }, ok = true }
q = "fast {math.floor(66.19)} {string.rep('=', 6 + 1)} beta done"
for j = 1, 39 do acc = acc + (j * 27.98) % 97 / (j + 1) end
acc = acc + blend(1.59, 58, math.sin(1.59 / 7)) * (1 + 5 / 31)
bag = { id = 7078, name = "warm-delta", pos = { 38.93, 7078.5, 77.86 }, ok = true }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 77.46) end
bag = { id = 4856, name = "blue-alpha", pos = { 22.49, 4856.5, 44.98 }, ok = true }
m = { ["k2088"] = { nested = { deeper = { value = "red", list = { 1, 2, 2088 } } } } }
q = "warm {math.floor(74.98)} {string.rep('=', 4 + 1)} alpha done"
r = (60.13 > 42) and (acc ~= 0 and "cold" or "beta") or blend(60.13, 2642, 0.25)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 53.57) end
bag = { id = 6411, name = "green-beta", pos = { 74.27, 6411.5, 148.54 }, ok = false }
for j = 1, 4 do acc = acc + (j * 3.30) % 97 / (j + 1) end
if acc > 34.19 then acc = acc / 1 elseif acc < -34.19 then acc = -acc else acc = acc + 712 end
for j = 1, 38 do acc = acc + (j * 31.14) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 50.21) end
bag = { id = 5293, name = "warm-delta", pos = { 40.95, 5293.5, 81.9 }, ok = false }
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 12.37) end
acc = acc + blend(66.98, 7, math.sin(66.98 / 7)) * (1 + 0 / 31)
q = "red {math.floor(83.24)} {string.rep('=', 0 + 1)} delta done"
r = (98.83 > 38) and (acc ~= 0 and "warm" or "beta") or blend(98.83, 8188, 0.25)
r = (32.32 > 26) and (acc ~= 0 and "cold" or "beta") or blend(32.32, 9026, 0.25)
for j = 1, 22 do acc = acc + (j * 84.94) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 21.01) end
for j = 1, 10 do acc = acc + (j * 89.80) % 97 / (j + 1) end
for j = 1, 12 do acc = acc + (j * 66.74) % 97 / (j + 1) end
q = "fast {math.floor(69.44)} {string.rep('=', 0 + 1)} alpha done"
m = { ["k421"] = { nested = { deeper = { value = "warm", list = { 1, 2, 421 } } } } }
q = "red {math.floor(75.80)} {string.rep('=', 6 + 1)} beta done"
acc = acc + blend(75.29, 66, math.sin(75.29 / 7)) * (1 + 2 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 10.14) end
acc = acc + blend(98.14, 92, math.sin(98.14 / 7)) * (1 + 6 / 31)
for j = 1, 26 do acc = acc + (j * 74.57) % 97 / (j + 1) end
q = "red {math.floor(87.52)} {string.rep('=', 4 + 1)} alpha done"
m = { ["k4109"] = { nested = { deeper = { value = "green", list = { 1, 2, 4109 } } } } }
q = "fast {math.floor(38.14)} {string.rep('=', 4 + 1)} delta done"
acc = acc + blend(71.08, 12, math.sin(71.08 / 7)) * (1 + 0 / 31)
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 30.82) end
for j = 1, 6 do acc = acc + (j * 74.28) % 97 / (j + 1) end
cb = function(a, b) return tostring(a) .. "·" .. string.format("%.4f", b * 93.55) end
for j = 1, 31 do acc = acc + (j * 4.09) % 97 / (j + 1) end
m = { ["k4340"] = { nested = { deeper = { value = "warm", list = { 1, 2, 4340 } } } } }

print("parse_dense checksum", acc % 1000, cfg.depth)
