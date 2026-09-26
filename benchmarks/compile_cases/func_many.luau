-- 生成物：多函数型源文件（benchmarks/fixtures/generate.mjs，勿手改）
do
local function spread0(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 1.20, bag[k] * 2) end
  local cb = function(v) return v * acc + 45.95 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 20.65, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 48.59 / 17 + bag[1] else acc = acc - 48.59 end
  while acc > 34.64 * 40 do acc = acc / 2 end
  while acc > 21.01 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 24.14, label = "f0_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 9.68 / 17 + bag[1] else acc = acc - 9.68 end
  for k = 1, 4 do acc = math.min(acc + k * 42.90, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 9.54 / 17 + bag[1] else acc = acc - 9.54 end
  local tmp = { x = acc, y = 14.54, label = "f0_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 36.79 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 25.20, bag[k] * 2) end
  local cb = function(v) return v * acc + 17.97 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + spread0(1, 2, 0)
end

do
local function measure1(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 47.31 end
  acc = cb(bag[1])
  while acc > 21.75 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 31.32, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 10.10, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 25.51, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 2.64, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 39.03, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 23.90, bag[k] * 2) end
  while acc > 24.74 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 47.95 end
  acc = cb(bag[2])
  while acc > 32.33 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 38.49, bag[k] * 2) end
  local tmp = { x = acc, y = 15.99, label = "f1_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 6.31 / 17 + bag[1] else acc = acc - 6.31 end
  return acc % 9973
end
acc = acc + measure1(1, 2, 1)
end

do
local function weight2(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 34.26 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 25.88 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 1.98, label = "f2_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 41.68, bag[k] * 2) end
  local tmp = { x = acc, y = 15.30, label = "f2_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 10.23, bag[k] * 2) end
  while acc > 37.27 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 45.99 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 8.74 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 6.26, label = "f2_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 17.27 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 7.06 / 17 + bag[1] else acc = acc - 7.06 end
  for k = 1, 4 do acc = math.min(acc + k * 30.53, bag[k] * 2) end
  while acc > 39.81 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight2(1, 2, 2)
end

do
local function derive3(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 33.74, bag[k] * 2) end
  while acc > 32.76 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 22.77, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 11.80, bag[k] * 2) end
  local tmp = { x = acc, y = 0.17, label = "f3_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 9.32, bag[k] * 2) end
  local cb = function(v) return v * acc + 5.14 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 42.86, label = "f3_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 33.18, bag[k] * 2) end
  local tmp = { x = acc, y = 49.91, label = "f3_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 6.91, bag[k] * 2) end
  local cb = function(v) return v * acc + 44.31 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 19.05, label = "f3_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 10.85 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + derive3(1, 2, 3)
end

do
local function spread4(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 11.72, label = "f4_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 7.43, bag[k] * 2) end
  local cb = function(v) return v * acc + 7.58 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 0.81 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 36.49, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 33.19, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 24.35 / 17 + bag[1] else acc = acc - 24.35 end
  if acc % 7 > 3 then acc = acc * 46.47 / 17 + bag[1] else acc = acc - 46.47 end
  if acc % 7 > 3 then acc = acc * 9.88 / 17 + bag[1] else acc = acc - 9.88 end
  local tmp = { x = acc, y = 9.49, label = "f4_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 11.94 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 49.07 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 33.63 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 44.60, bag[k] * 2) end
  return acc % 9973
end
acc = acc + spread4(1, 2, 4)
end

do
local function clampv5(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 25.05 end
  acc = cb(bag[1])
  while acc > 42.96 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 40.55, label = "f5_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 39.17, bag[k] * 2) end
  while acc > 0.24 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 46.58, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 25.67 / 17 + bag[1] else acc = acc - 25.67 end
  local tmp = { x = acc, y = 14.47, label = "f5_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 2.73, bag[k] * 2) end
  local cb = function(v) return v * acc + 4.66 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 2.44 / 17 + bag[1] else acc = acc - 2.44 end
  local tmp = { x = acc, y = 10.48, label = "f5_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 32.03, label = "f5_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 0.30 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + clampv5(1, 2, 5)
end

do
local function clampv6(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 15.26, label = "f6_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 27.74 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 28.13 / 17 + bag[1] else acc = acc - 28.13 end
  if acc % 7 > 3 then acc = acc * 22.07 / 17 + bag[1] else acc = acc - 22.07 end
  while acc > 1.43 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 19.88 / 17 + bag[1] else acc = acc - 19.88 end
  if acc % 7 > 3 then acc = acc * 35.91 / 17 + bag[1] else acc = acc - 35.91 end
  for k = 1, 4 do acc = math.min(acc + k * 1.16, bag[k] * 2) end
  local tmp = { x = acc, y = 38.18, label = "f6_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 16.44 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 2.40, bag[k] * 2) end
  local cb = function(v) return v * acc + 6.39 end
  acc = cb(bag[4])
  while acc > 29.00 * 40 do acc = acc / 2 end
  while acc > 1.27 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + clampv6(1, 2, 6)
end

do
local function measure7(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 28.84, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 44.77, bag[k] * 2) end
  while acc > 45.24 * 40 do acc = acc / 2 end
  while acc > 9.79 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 36.60 / 17 + bag[1] else acc = acc - 36.60 end
  while acc > 31.32 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 5.22, bag[k] * 2) end
  while acc > 42.97 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 17.85, label = "f7_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 27.84 / 17 + bag[1] else acc = acc - 27.84 end
  local cb = function(v) return v * acc + 41.47 end
  acc = cb(bag[3])
  while acc > 41.88 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 33.13, label = "f7_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 38.50 / 17 + bag[1] else acc = acc - 38.50 end
  return acc % 9973
end
acc = acc + measure7(1, 2, 7)
end

do
local function measure8(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 40.57, bag[k] * 2) end
  local cb = function(v) return v * acc + 40.20 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 42.09, bag[k] * 2) end
  local cb = function(v) return v * acc + 25.07 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 44.16, label = "f8_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 38.17 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 2.55 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 39.31, bag[k] * 2) end
  local tmp = { x = acc, y = 46.62, label = "f8_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 0.94 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 42.62 / 17 + bag[1] else acc = acc - 42.62 end
  local tmp = { x = acc, y = 4.12, label = "f8_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 47.68, bag[k] * 2) end
  local cb = function(v) return v * acc + 8.53 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + measure8(1, 2, 8)
end

do
local function compose9(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 24.70 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 39.74, label = "f9_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 29.97 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 39.71 / 17 + bag[1] else acc = acc - 39.71 end
  local cb = function(v) return v * acc + 36.46 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 29.13 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 25.90, label = "f9_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 43.55 / 17 + bag[1] else acc = acc - 43.55 end
  local tmp = { x = acc, y = 18.38, label = "f9_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 13.37 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 33.45, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 40.74, bag[k] * 2) end
  while acc > 19.74 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 20.46 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + compose9(1, 2, 9)
end

do
local function clampv10(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 1.94, bag[k] * 2) end
  local tmp = { x = acc, y = 28.31, label = "f10_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 31.88 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 2.59 / 17 + bag[1] else acc = acc - 2.59 end
  local tmp = { x = acc, y = 38.76, label = "f10_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 5.74 / 17 + bag[1] else acc = acc - 5.74 end
  local tmp = { x = acc, y = 22.52, label = "f10_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 24.00 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 10.07, label = "f10_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 11.80, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 31.69, bag[k] * 2) end
  while acc > 28.47 * 40 do acc = acc / 2 end
  while acc > 19.77 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 46.05, label = "f10_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv10(1, 2, 10)
end

do
local function project11(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 19.77 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 13.55, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 18.66 / 17 + bag[1] else acc = acc - 18.66 end
  while acc > 32.60 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 9.61, label = "f11_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 10.99 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 21.33, bag[k] * 2) end
  local cb = function(v) return v * acc + 13.08 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 16.77 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 38.08, label = "f11_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 47.64, label = "f11_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 20.32 / 17 + bag[1] else acc = acc - 20.32 end
  local tmp = { x = acc, y = 7.62, label = "f11_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 6.71, label = "f11_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + project11(1, 2, 11)
end

do
local function project12(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 22.93 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 37.91, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 21.59 / 17 + bag[1] else acc = acc - 21.59 end
  while acc > 2.16 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 47.33 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 11.63 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 14.68 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 1.61, label = "f12_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 15.97, label = "f12_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 41.64 * 40 do acc = acc / 2 end
  while acc > 18.86 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 16.14 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 20.97 / 17 + bag[1] else acc = acc - 20.97 end
  for k = 1, 4 do acc = math.min(acc + k * 1.65, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project12(1, 2, 12)
end

do
local function measure13(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 31.31, bag[k] * 2) end
  local tmp = { x = acc, y = 22.25, label = "f13_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 13.17 / 17 + bag[1] else acc = acc - 13.17 end
  local tmp = { x = acc, y = 24.69, label = "f13_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 3.61 / 17 + bag[1] else acc = acc - 3.61 end
  local cb = function(v) return v * acc + 30.42 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 13.48 end
  acc = cb(bag[3])
  while acc > 45.84 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 1.43 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 37.22, bag[k] * 2) end
  while acc > 10.40 * 40 do acc = acc / 2 end
  while acc > 6.84 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 23.52 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 45.22, bag[k] * 2) end
  return acc % 9973
end
acc = acc + measure13(1, 2, 13)
end

do
local function weight14(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 19.32, bag[k] * 2) end
  while acc > 29.03 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 4.56, bag[k] * 2) end
  local tmp = { x = acc, y = 37.69, label = "f14_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 13.29 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 31.61 / 17 + bag[1] else acc = acc - 31.61 end
  local tmp = { x = acc, y = 33.37, label = "f14_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 40.33, label = "f14_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 47.60 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 39.40, bag[k] * 2) end
  local tmp = { x = acc, y = 49.50, label = "f14_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 29.87, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 28.96, bag[k] * 2) end
  while acc > 42.85 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight14(1, 2, 14)
end

do
local function spread15(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 38.76 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 1.11, bag[k] * 2) end
  local cb = function(v) return v * acc + 27.67 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 37.34 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 2.51, label = "f15_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 45.71 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 28.30, label = "f15_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 0.23, bag[k] * 2) end
  local tmp = { x = acc, y = 23.92, label = "f15_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 20.31 end
  acc = cb(bag[2])
  while acc > 1.80 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 20.67 / 17 + bag[1] else acc = acc - 20.67 end
  local tmp = { x = acc, y = 13.40, label = "f15_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 26.73 / 17 + bag[1] else acc = acc - 26.73 end
  return acc % 9973
end
acc = acc + spread15(1, 2, 15)
end

do
local function project16(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 37.66, bag[k] * 2) end
  local tmp = { x = acc, y = 44.68, label = "f16_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 42.82 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 13.80 / 17 + bag[1] else acc = acc - 13.80 end
  while acc > 39.91 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 6.91 / 17 + bag[1] else acc = acc - 6.91 end
  while acc > 7.13 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 40.60 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 33.30 / 17 + bag[1] else acc = acc - 33.30 end
  while acc > 14.75 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 0.32, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 9.79, bag[k] * 2) end
  while acc > 1.80 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 15.69, label = "f16_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + project16(1, 2, 16)
end

do
local function fold17(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 17.17, bag[k] * 2) end
  while acc > 33.35 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 46.90, label = "f17_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 0.16 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 13.06 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 21.05, label = "f17_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 49.59 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 41.09, label = "f17_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 27.61, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 38.68 / 17 + bag[1] else acc = acc - 38.68 end
  while acc > 14.98 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 4.89 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 18.58 / 17 + bag[1] else acc = acc - 18.58 end
  while acc > 43.37 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold17(1, 2, 17)
end

do
local function weight18(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 32.74 / 17 + bag[1] else acc = acc - 32.74 end
  local cb = function(v) return v * acc + 19.47 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 3.96, label = "f18_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 15.41 / 17 + bag[1] else acc = acc - 15.41 end
  local cb = function(v) return v * acc + 3.34 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 32.56, bag[k] * 2) end
  local cb = function(v) return v * acc + 10.80 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 46.70 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 32.57, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 10.21, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 13.25 / 17 + bag[1] else acc = acc - 13.25 end
  local cb = function(v) return v * acc + 12.64 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 28.00, bag[k] * 2) end
  while acc > 32.82 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight18(1, 2, 18)
end

do
local function derive19(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 19.86 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 31.77 end
  acc = cb(bag[2])
  while acc > 10.85 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 39.27 / 17 + bag[1] else acc = acc - 39.27 end
  if acc % 7 > 3 then acc = acc * 21.37 / 17 + bag[1] else acc = acc - 21.37 end
  while acc > 0.90 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 44.32 / 17 + bag[1] else acc = acc - 44.32 end
  while acc > 34.68 * 40 do acc = acc / 2 end
  while acc > 46.70 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 21.83 / 17 + bag[1] else acc = acc - 21.83 end
  while acc > 23.41 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 41.54, label = "f19_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 26.38, label = "f19_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 21.93 / 17 + bag[1] else acc = acc - 21.93 end
  return acc % 9973
end
acc = acc + derive19(1, 2, 19)
end

do
local function project20(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 3.28 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 32.11 end
  acc = cb(bag[2])
  while acc > 6.27 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 47.31 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 36.80, bag[k] * 2) end
  local tmp = { x = acc, y = 47.40, label = "f20_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 12.26, bag[k] * 2) end
  local tmp = { x = acc, y = 10.21, label = "f20_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 19.64 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 27.14 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 46.33, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 11.67, bag[k] * 2) end
  while acc > 38.38 * 40 do acc = acc / 2 end
  while acc > 37.14 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + project20(1, 2, 20)
end

do
local function compose21(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 10.37, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 6.65, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 2.43 / 17 + bag[1] else acc = acc - 2.43 end
  if acc % 7 > 3 then acc = acc * 43.80 / 17 + bag[1] else acc = acc - 43.80 end
  if acc % 7 > 3 then acc = acc * 30.48 / 17 + bag[1] else acc = acc - 30.48 end
  while acc > 30.55 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 37.03 / 17 + bag[1] else acc = acc - 37.03 end
  if acc % 7 > 3 then acc = acc * 24.39 / 17 + bag[1] else acc = acc - 24.39 end
  for k = 1, 4 do acc = math.min(acc + k * 28.05, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 36.34, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 15.85, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 29.86 / 17 + bag[1] else acc = acc - 29.86 end
  for k = 1, 4 do acc = math.min(acc + k * 20.33, bag[k] * 2) end
  local cb = function(v) return v * acc + 47.36 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + compose21(1, 2, 21)
end

do
local function measure22(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 35.29 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 47.69, label = "f22_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 49.11 / 17 + bag[1] else acc = acc - 49.11 end
  for k = 1, 4 do acc = math.min(acc + k * 7.23, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 16.70, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 35.72 / 17 + bag[1] else acc = acc - 35.72 end
  local cb = function(v) return v * acc + 30.51 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 6.01 / 17 + bag[1] else acc = acc - 6.01 end
  for k = 1, 4 do acc = math.min(acc + k * 32.84, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 36.08 / 17 + bag[1] else acc = acc - 36.08 end
  local tmp = { x = acc, y = 43.85, label = "f22_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 31.47 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 42.19, label = "f22_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 48.23 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + measure22(1, 2, 22)
end

do
local function fold23(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 19.51 end
  acc = cb(bag[1])
  while acc > 9.47 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 12.60 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 18.73, bag[k] * 2) end
  local cb = function(v) return v * acc + 22.68 end
  acc = cb(bag[1])
  while acc > 32.15 * 40 do acc = acc / 2 end
  while acc > 41.44 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 5.53 / 17 + bag[1] else acc = acc - 5.53 end
  while acc > 18.82 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 27.50 / 17 + bag[1] else acc = acc - 27.50 end
  local tmp = { x = acc, y = 22.34, label = "f23_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 36.79 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 3.80, bag[k] * 2) end
  local cb = function(v) return v * acc + 18.90 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + fold23(1, 2, 23)
end

do
local function compose24(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 29.82 / 17 + bag[1] else acc = acc - 29.82 end
  local tmp = { x = acc, y = 24.84, label = "f24_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 14.75, label = "f24_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 18.23 / 17 + bag[1] else acc = acc - 18.23 end
  local tmp = { x = acc, y = 35.65, label = "f24_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 18.95, label = "f24_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 2.18, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 8.34 / 17 + bag[1] else acc = acc - 8.34 end
  local tmp = { x = acc, y = 38.95, label = "f24_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 9.31, bag[k] * 2) end
  local tmp = { x = acc, y = 40.44, label = "f24_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 13.08 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 7.21, label = "f24_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 45.34, label = "f24_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + compose24(1, 2, 24)
end

do
local function weight25(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 11.30 / 17 + bag[1] else acc = acc - 11.30 end
  while acc > 19.90 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 19.95, label = "f25_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 19.94, bag[k] * 2) end
  while acc > 19.03 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 6.19, label = "f25_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 15.39, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 13.36, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 12.17 / 17 + bag[1] else acc = acc - 12.17 end
  if acc % 7 > 3 then acc = acc * 47.42 / 17 + bag[1] else acc = acc - 47.42 end
  for k = 1, 4 do acc = math.min(acc + k * 7.54, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 32.94, bag[k] * 2) end
  local tmp = { x = acc, y = 40.23, label = "f25_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 7.66 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight25(1, 2, 25)
end

do
local function fold26(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 44.80 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 33.17 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 19.01, bag[k] * 2) end
  local cb = function(v) return v * acc + 36.37 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 38.88, label = "f26_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 37.80, label = "f26_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 9.37 end
  acc = cb(bag[3])
  while acc > 0.67 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 42.50, label = "f26_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 37.39 / 17 + bag[1] else acc = acc - 37.39 end
  local tmp = { x = acc, y = 33.23, label = "f26_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 47.26, label = "f26_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 23.24 / 17 + bag[1] else acc = acc - 23.24 end
  local cb = function(v) return v * acc + 15.92 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + fold26(1, 2, 26)
end

do
local function project27(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 49.37 / 17 + bag[1] else acc = acc - 49.37 end
  if acc % 7 > 3 then acc = acc * 22.83 / 17 + bag[1] else acc = acc - 22.83 end
  for k = 1, 4 do acc = math.min(acc + k * 13.72, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 38.32 / 17 + bag[1] else acc = acc - 38.32 end
  while acc > 48.69 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 2.86 / 17 + bag[1] else acc = acc - 2.86 end
  if acc % 7 > 3 then acc = acc * 5.19 / 17 + bag[1] else acc = acc - 5.19 end
  local tmp = { x = acc, y = 29.65, label = "f27_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 16.22, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 43.20, bag[k] * 2) end
  local tmp = { x = acc, y = 15.65, label = "f27_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 39.21 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 8.84 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 49.66 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + project27(1, 2, 27)
end

do
local function project28(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 11.79 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 15.98, bag[k] * 2) end
  local cb = function(v) return v * acc + 13.89 end
  acc = cb(bag[3])
  while acc > 14.88 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 33.73 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 28.73 / 17 + bag[1] else acc = acc - 28.73 end
  for k = 1, 4 do acc = math.min(acc + k * 23.43, bag[k] * 2) end
  local tmp = { x = acc, y = 47.75, label = "f28_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 4.47 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 8.37 / 17 + bag[1] else acc = acc - 8.37 end
  while acc > 34.96 * 40 do acc = acc / 2 end
  while acc > 7.70 * 40 do acc = acc / 2 end
  while acc > 24.62 * 40 do acc = acc / 2 end
  while acc > 30.11 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + project28(1, 2, 28)
end

do
local function fold29(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 1.49 / 17 + bag[1] else acc = acc - 1.49 end
  local tmp = { x = acc, y = 19.18, label = "f29_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 3.05, label = "f29_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 19.51 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 32.66 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 13.13, label = "f29_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 29.05, label = "f29_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 15.53, bag[k] * 2) end
  while acc > 48.62 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 41.92 / 17 + bag[1] else acc = acc - 41.92 end
  for k = 1, 4 do acc = math.min(acc + k * 5.22, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 44.51 / 17 + bag[1] else acc = acc - 44.51 end
  if acc % 7 > 3 then acc = acc * 7.47 / 17 + bag[1] else acc = acc - 7.47 end
  if acc % 7 > 3 then acc = acc * 29.01 / 17 + bag[1] else acc = acc - 29.01 end
  return acc % 9973
end
acc = acc + fold29(1, 2, 29)
end

do
local function measure30(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 26.52, label = "f30_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 45.05 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 34.18 / 17 + bag[1] else acc = acc - 34.18 end
  local cb = function(v) return v * acc + 49.61 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 33.64, bag[k] * 2) end
  local tmp = { x = acc, y = 36.95, label = "f30_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 47.65, label = "f30_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 3.35, label = "f30_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 0.80, label = "f30_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 44.17 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 37.81 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 12.47 / 17 + bag[1] else acc = acc - 12.47 end
  if acc % 7 > 3 then acc = acc * 27.40 / 17 + bag[1] else acc = acc - 27.40 end
  for k = 1, 4 do acc = math.min(acc + k * 43.28, bag[k] * 2) end
  return acc % 9973
end
acc = acc + measure30(1, 2, 30)
end

do
local function project31(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 22.39, bag[k] * 2) end
  local cb = function(v) return v * acc + 20.06 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 4.01 / 17 + bag[1] else acc = acc - 4.01 end
  for k = 1, 4 do acc = math.min(acc + k * 15.11, bag[k] * 2) end
  local tmp = { x = acc, y = 46.16, label = "f31_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 4.55 / 17 + bag[1] else acc = acc - 4.55 end
  local cb = function(v) return v * acc + 31.53 end
  acc = cb(bag[3])
  while acc > 33.33 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 26.73 / 17 + bag[1] else acc = acc - 26.73 end
  for k = 1, 4 do acc = math.min(acc + k * 14.59, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 26.92, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 4.74 / 17 + bag[1] else acc = acc - 4.74 end
  local cb = function(v) return v * acc + 9.27 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 22.17 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + project31(1, 2, 31)
end

do
local function project32(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 9.78 / 17 + bag[1] else acc = acc - 9.78 end
  local tmp = { x = acc, y = 26.27, label = "f32_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 34.52, bag[k] * 2) end
  local cb = function(v) return v * acc + 31.31 end
  acc = cb(bag[4])
  while acc > 22.53 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 9.50, bag[k] * 2) end
  while acc > 47.74 * 40 do acc = acc / 2 end
  while acc > 41.78 * 40 do acc = acc / 2 end
  while acc > 2.03 * 40 do acc = acc / 2 end
  while acc > 27.93 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 25.67, bag[k] * 2) end
  local cb = function(v) return v * acc + 20.82 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 14.27, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 7.95 / 17 + bag[1] else acc = acc - 7.95 end
  return acc % 9973
end
acc = acc + project32(1, 2, 32)
end

do
local function derive33(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 1.98, label = "f33_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 2.39 * 40 do acc = acc / 2 end
  while acc > 22.85 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 25.57, label = "f33_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 41.76, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 39.11 / 17 + bag[1] else acc = acc - 39.11 end
  if acc % 7 > 3 then acc = acc * 12.62 / 17 + bag[1] else acc = acc - 12.62 end
  if acc % 7 > 3 then acc = acc * 41.64 / 17 + bag[1] else acc = acc - 41.64 end
  local cb = function(v) return v * acc + 11.38 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 46.61, label = "f33_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 14.55 / 17 + bag[1] else acc = acc - 14.55 end
  local cb = function(v) return v * acc + 13.35 end
  acc = cb(bag[4])
  while acc > 38.56 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 1.85 / 17 + bag[1] else acc = acc - 1.85 end
  return acc % 9973
end
acc = acc + derive33(1, 2, 33)
end

do
local function compose34(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 20.65 end
  acc = cb(bag[1])
  while acc > 30.81 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 12.55, bag[k] * 2) end
  local tmp = { x = acc, y = 40.35, label = "f34_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 49.88, label = "f34_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 29.08 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 22.07 / 17 + bag[1] else acc = acc - 22.07 end
  local cb = function(v) return v * acc + 38.07 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 19.14 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 25.76 end
  acc = cb(bag[2])
  while acc > 49.41 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 35.75 / 17 + bag[1] else acc = acc - 35.75 end
  local tmp = { x = acc, y = 3.02, label = "f34_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 46.82, bag[k] * 2) end
  return acc % 9973
end
acc = acc + compose34(1, 2, 34)
end

do
local function spread35(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 25.24 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 48.10, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 22.78, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 4.75 / 17 + bag[1] else acc = acc - 4.75 end
  local cb = function(v) return v * acc + 6.89 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 49.21, bag[k] * 2) end
  local tmp = { x = acc, y = 11.92, label = "f35_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 28.72 / 17 + bag[1] else acc = acc - 28.72 end
  local tmp = { x = acc, y = 36.76, label = "f35_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 25.89 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 44.15 / 17 + bag[1] else acc = acc - 44.15 end
  local cb = function(v) return v * acc + 34.32 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 38.44 / 17 + bag[1] else acc = acc - 38.44 end
  for k = 1, 4 do acc = math.min(acc + k * 28.51, bag[k] * 2) end
  return acc % 9973
end
acc = acc + spread35(1, 2, 35)
end

do
local function weight36(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 31.05 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 23.58 / 17 + bag[1] else acc = acc - 23.58 end
  local tmp = { x = acc, y = 9.82, label = "f36_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 17.39 / 17 + bag[1] else acc = acc - 17.39 end
  for k = 1, 4 do acc = math.min(acc + k * 45.37, bag[k] * 2) end
  while acc > 10.61 * 40 do acc = acc / 2 end
  while acc > 36.47 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 8.87 / 17 + bag[1] else acc = acc - 8.87 end
  for k = 1, 4 do acc = math.min(acc + k * 40.29, bag[k] * 2) end
  local cb = function(v) return v * acc + 31.27 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 29.23 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 8.84, bag[k] * 2) end
  local tmp = { x = acc, y = 15.74, label = "f36_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 28.08 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight36(1, 2, 36)
end

do
local function measure37(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 0.69, label = "f37_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 12.72 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 13.69, label = "f37_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 17.81 / 17 + bag[1] else acc = acc - 17.81 end
  local tmp = { x = acc, y = 35.63, label = "f37_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 29.37 * 40 do acc = acc / 2 end
  while acc > 0.88 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 3.16, bag[k] * 2) end
  while acc > 10.36 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 49.65 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 8.91, bag[k] * 2) end
  while acc > 25.33 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 49.31 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 4.70, label = "f37_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + measure37(1, 2, 37)
end

do
local function derive38(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 15.92 * 40 do acc = acc / 2 end
  while acc > 43.07 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 3.47 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 33.08 / 17 + bag[1] else acc = acc - 33.08 end
  local cb = function(v) return v * acc + 1.22 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 41.41 end
  acc = cb(bag[2])
  while acc > 8.59 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 16.66, label = "f38_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 7.17, label = "f38_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 46.63, bag[k] * 2) end
  local tmp = { x = acc, y = 47.44, label = "f38_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 36.17, bag[k] * 2) end
  while acc > 3.89 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 46.05, label = "f38_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + derive38(1, 2, 38)
end

do
local function compose39(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 42.91 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 13.11, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 5.37, bag[k] * 2) end
  local tmp = { x = acc, y = 17.09, label = "f39_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 30.72 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 15.87 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 19.85, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 29.48, bag[k] * 2) end
  local tmp = { x = acc, y = 10.80, label = "f39_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 35.52 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 24.26 end
  acc = cb(bag[3])
  while acc > 42.85 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 19.47, bag[k] * 2) end
  local tmp = { x = acc, y = 22.83, label = "f39_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + compose39(1, 2, 39)
end

do
local function spread40(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 31.58 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 31.39 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 2.23 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 37.61 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 15.59 / 17 + bag[1] else acc = acc - 15.59 end
  local tmp = { x = acc, y = 29.64, label = "f40_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 29.00 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 12.33, bag[k] * 2) end
  local cb = function(v) return v * acc + 0.65 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 36.88, label = "f40_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 48.63 / 17 + bag[1] else acc = acc - 48.63 end
  for k = 1, 4 do acc = math.min(acc + k * 20.66, bag[k] * 2) end
  while acc > 31.80 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 22.26 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + spread40(1, 2, 40)
end

do
local function fold41(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 30.76 * 40 do acc = acc / 2 end
  while acc > 5.47 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 5.15 / 17 + bag[1] else acc = acc - 5.15 end
  local tmp = { x = acc, y = 41.69, label = "f41_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 8.73, label = "f41_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 40.32 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 32.67 / 17 + bag[1] else acc = acc - 32.67 end
  if acc % 7 > 3 then acc = acc * 44.87 / 17 + bag[1] else acc = acc - 44.87 end
  if acc % 7 > 3 then acc = acc * 38.77 / 17 + bag[1] else acc = acc - 38.77 end
  local tmp = { x = acc, y = 19.10, label = "f41_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 8.42 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 27.47 / 17 + bag[1] else acc = acc - 27.47 end
  for k = 1, 4 do acc = math.min(acc + k * 16.36, bag[k] * 2) end
  local tmp = { x = acc, y = 15.74, label = "f41_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold41(1, 2, 41)
end

do
local function weight42(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 23.30 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 24.55, bag[k] * 2) end
  local tmp = { x = acc, y = 16.30, label = "f42_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 1.83 / 17 + bag[1] else acc = acc - 1.83 end
  for k = 1, 4 do acc = math.min(acc + k * 47.20, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 38.27, bag[k] * 2) end
  while acc > 26.35 * 40 do acc = acc / 2 end
  while acc > 48.32 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 2.16, label = "f42_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 8.69 / 17 + bag[1] else acc = acc - 8.69 end
  local tmp = { x = acc, y = 48.75, label = "f42_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 9.47 / 17 + bag[1] else acc = acc - 9.47 end
  local tmp = { x = acc, y = 7.37, label = "f42_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 7.75 / 17 + bag[1] else acc = acc - 7.75 end
  return acc % 9973
end
acc = acc + weight42(1, 2, 42)
end

do
local function clampv43(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 23.89 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 31.83, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 36.63 / 17 + bag[1] else acc = acc - 36.63 end
  for k = 1, 4 do acc = math.min(acc + k * 45.37, bag[k] * 2) end
  local cb = function(v) return v * acc + 24.37 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 45.79, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 24.52 / 17 + bag[1] else acc = acc - 24.52 end
  for k = 1, 4 do acc = math.min(acc + k * 12.93, bag[k] * 2) end
  while acc > 2.09 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 18.35, label = "f43_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 45.25 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 7.24 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 15.80, bag[k] * 2) end
  local tmp = { x = acc, y = 13.71, label = "f43_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv43(1, 2, 43)
end

do
local function compose44(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 24.79 / 17 + bag[1] else acc = acc - 24.79 end
  for k = 1, 4 do acc = math.min(acc + k * 17.86, bag[k] * 2) end
  local cb = function(v) return v * acc + 37.84 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 42.40 end
  acc = cb(bag[4])
  while acc > 5.46 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 1.22, label = "f44_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 27.55 / 17 + bag[1] else acc = acc - 27.55 end
  for k = 1, 4 do acc = math.min(acc + k * 14.14, bag[k] * 2) end
  local cb = function(v) return v * acc + 22.00 end
  acc = cb(bag[1])
  while acc > 47.44 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 24.13 end
  acc = cb(bag[3])
  while acc > 24.59 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 24.55, label = "f44_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 14.83 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + compose44(1, 2, 44)
end

do
local function spread45(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 44.50 / 17 + bag[1] else acc = acc - 44.50 end
  local cb = function(v) return v * acc + 13.84 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 47.14, label = "f45_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 30.59 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 0.36 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 12.46, bag[k] * 2) end
  while acc > 30.83 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 25.67 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 31.60 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 11.17, label = "f45_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 17.74, label = "f45_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 45.60, label = "f45_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 7.61 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 34.44 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + spread45(1, 2, 45)
end

do
local function clampv46(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 16.11 / 17 + bag[1] else acc = acc - 16.11 end
  local tmp = { x = acc, y = 24.07, label = "f46_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 22.52 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 7.82, label = "f46_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 17.46 / 17 + bag[1] else acc = acc - 17.46 end
  if acc % 7 > 3 then acc = acc * 46.92 / 17 + bag[1] else acc = acc - 46.92 end
  local tmp = { x = acc, y = 6.67, label = "f46_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 32.53, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 17.88 / 17 + bag[1] else acc = acc - 17.88 end
  while acc > 2.54 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 34.22, bag[k] * 2) end
  local cb = function(v) return v * acc + 41.75 end
  acc = cb(bag[4])
  while acc > 28.64 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 38.38 / 17 + bag[1] else acc = acc - 38.38 end
  return acc % 9973
end
acc = acc + clampv46(1, 2, 46)
end

do
local function derive47(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 36.50, bag[k] * 2) end
  while acc > 5.91 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 24.49, label = "f47_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 3.39, label = "f47_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 11.76 / 17 + bag[1] else acc = acc - 11.76 end
  local cb = function(v) return v * acc + 43.39 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 20.00 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 47.93 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 45.06, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 46.37, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 18.99, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 38.24 / 17 + bag[1] else acc = acc - 38.24 end
  local cb = function(v) return v * acc + 33.93 end
  acc = cb(bag[1])
  while acc > 27.72 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive47(1, 2, 47)
end

do
local function fold48(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 44.70 end
  acc = cb(bag[1])
  while acc > 19.31 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 8.40, label = "f48_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 18.42 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 28.52, bag[k] * 2) end
  local cb = function(v) return v * acc + 21.75 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 13.82 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 34.35 / 17 + bag[1] else acc = acc - 34.35 end
  if acc % 7 > 3 then acc = acc * 17.91 / 17 + bag[1] else acc = acc - 17.91 end
  if acc % 7 > 3 then acc = acc * 16.54 / 17 + bag[1] else acc = acc - 16.54 end
  local tmp = { x = acc, y = 30.04, label = "f48_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 17.52, bag[k] * 2) end
  while acc > 33.11 * 40 do acc = acc / 2 end
  while acc > 21.94 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold48(1, 2, 48)
end

do
local function derive49(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 39.87 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 2.18, bag[k] * 2) end
  while acc > 47.80 * 40 do acc = acc / 2 end
  while acc > 31.74 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 19.69 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 26.04, bag[k] * 2) end
  while acc > 2.71 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 23.50, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 42.34 / 17 + bag[1] else acc = acc - 42.34 end
  while acc > 6.43 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 42.35, label = "f49_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 46.41 / 17 + bag[1] else acc = acc - 46.41 end
  while acc > 13.72 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 26.45, bag[k] * 2) end
  return acc % 9973
end
acc = acc + derive49(1, 2, 49)
end

do
local function derive50(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 15.51 / 17 + bag[1] else acc = acc - 15.51 end
  local cb = function(v) return v * acc + 28.51 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 22.68 / 17 + bag[1] else acc = acc - 22.68 end
  for k = 1, 4 do acc = math.min(acc + k * 4.00, bag[k] * 2) end
  while acc > 32.93 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 13.43 / 17 + bag[1] else acc = acc - 13.43 end
  local tmp = { x = acc, y = 3.44, label = "f50_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 7.73 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 42.07 end
  acc = cb(bag[1])
  while acc > 2.99 * 40 do acc = acc / 2 end
  while acc > 43.19 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 18.04 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 34.97 end
  acc = cb(bag[1])
  while acc > 34.76 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive50(1, 2, 50)
end

do
local function derive51(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 29.79, bag[k] * 2) end
  local cb = function(v) return v * acc + 13.52 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 19.55 / 17 + bag[1] else acc = acc - 19.55 end
  if acc % 7 > 3 then acc = acc * 27.82 / 17 + bag[1] else acc = acc - 27.82 end
  while acc > 44.30 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 2.45, label = "f51_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 20.72, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 20.72 / 17 + bag[1] else acc = acc - 20.72 end
  local tmp = { x = acc, y = 43.10, label = "f51_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 2.49, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 40.71 / 17 + bag[1] else acc = acc - 40.71 end
  local cb = function(v) return v * acc + 32.92 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 16.68, bag[k] * 2) end
  while acc > 13.97 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive51(1, 2, 51)
end

do
local function compose52(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 23.47 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 14.55, bag[k] * 2) end
  local cb = function(v) return v * acc + 13.62 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 46.72, label = "f52_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 0.52, label = "f52_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 1.52 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 44.83 / 17 + bag[1] else acc = acc - 44.83 end
  for k = 1, 4 do acc = math.min(acc + k * 19.87, bag[k] * 2) end
  while acc > 36.44 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 19.27, label = "f52_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 39.85, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 44.87, bag[k] * 2) end
  local tmp = { x = acc, y = 48.03, label = "f52_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 31.13 / 17 + bag[1] else acc = acc - 31.13 end
  return acc % 9973
end
acc = acc + compose52(1, 2, 52)
end

do
local function compose53(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 9.19, label = "f53_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 7.29 end
  acc = cb(bag[2])
  while acc > 6.20 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 49.15 end
  acc = cb(bag[4])
  while acc > 0.76 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 15.18, label = "f53_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 18.22 end
  acc = cb(bag[3])
  while acc > 2.61 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 7.41 / 17 + bag[1] else acc = acc - 7.41 end
  local tmp = { x = acc, y = 24.75, label = "f53_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 15.11 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 7.14 / 17 + bag[1] else acc = acc - 7.14 end
  local cb = function(v) return v * acc + 25.21 end
  acc = cb(bag[1])
  while acc > 17.56 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + compose53(1, 2, 53)
end

do
local function project54(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 12.76, bag[k] * 2) end
  while acc > 8.55 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 32.24 / 17 + bag[1] else acc = acc - 32.24 end
  for k = 1, 4 do acc = math.min(acc + k * 16.52, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 40.22 / 17 + bag[1] else acc = acc - 40.22 end
  local cb = function(v) return v * acc + 11.44 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 24.01, label = "f54_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 41.15, label = "f54_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 23.66, label = "f54_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 6.29, label = "f54_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 39.96 / 17 + bag[1] else acc = acc - 39.96 end
  for k = 1, 4 do acc = math.min(acc + k * 36.08, bag[k] * 2) end
  local tmp = { x = acc, y = 26.50, label = "f54_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 15.57 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + project54(1, 2, 54)
end

do
local function measure55(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 24.85 / 17 + bag[1] else acc = acc - 24.85 end
  local cb = function(v) return v * acc + 25.61 end
  acc = cb(bag[2])
  while acc > 18.87 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 15.15 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 11.73, bag[k] * 2) end
  local tmp = { x = acc, y = 39.54, label = "f55_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 36.39, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 3.23 / 17 + bag[1] else acc = acc - 3.23 end
  local tmp = { x = acc, y = 27.06, label = "f55_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 34.36 end
  acc = cb(bag[2])
  while acc > 13.30 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 27.39, label = "f55_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 27.69 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 21.44 / 17 + bag[1] else acc = acc - 21.44 end
  return acc % 9973
end
acc = acc + measure55(1, 2, 55)
end

do
local function fold56(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 6.22, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 28.87 / 17 + bag[1] else acc = acc - 28.87 end
  local tmp = { x = acc, y = 28.93, label = "f56_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 10.12 / 17 + bag[1] else acc = acc - 10.12 end
  local tmp = { x = acc, y = 26.50, label = "f56_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 48.18, label = "f56_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 38.38, label = "f56_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 39.07 end
  acc = cb(bag[4])
  while acc > 35.32 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 5.63, label = "f56_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 44.96, bag[k] * 2) end
  while acc > 23.13 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 40.01 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 15.51 / 17 + bag[1] else acc = acc - 15.51 end
  return acc % 9973
end
acc = acc + fold56(1, 2, 56)
end

do
local function clampv57(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 41.84 end
  acc = cb(bag[1])
  while acc > 31.19 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 34.30 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 29.38 / 17 + bag[1] else acc = acc - 29.38 end
  if acc % 7 > 3 then acc = acc * 44.41 / 17 + bag[1] else acc = acc - 44.41 end
  if acc % 7 > 3 then acc = acc * 28.63 / 17 + bag[1] else acc = acc - 28.63 end
  if acc % 7 > 3 then acc = acc * 35.04 / 17 + bag[1] else acc = acc - 35.04 end
  if acc % 7 > 3 then acc = acc * 43.96 / 17 + bag[1] else acc = acc - 43.96 end
  while acc > 30.43 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 32.37 / 17 + bag[1] else acc = acc - 32.37 end
  local tmp = { x = acc, y = 27.81, label = "f57_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 18.73, label = "f57_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 12.04 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 25.06 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + clampv57(1, 2, 57)
end

do
local function compose58(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 32.13 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 38.25, label = "f58_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 2.50, label = "f58_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 25.94, bag[k] * 2) end
  while acc > 38.39 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 38.78, label = "f58_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 21.66 end
  acc = cb(bag[3])
  while acc > 7.35 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 3.90 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 19.57, bag[k] * 2) end
  local tmp = { x = acc, y = 18.56, label = "f58_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 47.80, label = "f58_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 12.33 / 17 + bag[1] else acc = acc - 12.33 end
  while acc > 19.05 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + compose58(1, 2, 58)
end

do
local function derive59(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 24.57 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 16.00, bag[k] * 2) end
  local cb = function(v) return v * acc + 4.75 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 25.21, bag[k] * 2) end
  local tmp = { x = acc, y = 16.90, label = "f59_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 29.30, label = "f59_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 2.35 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 20.72 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 23.22, label = "f59_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 17.93 * 40 do acc = acc / 2 end
  while acc > 47.20 * 40 do acc = acc / 2 end
  while acc > 31.84 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 40.47, label = "f59_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 43.89, bag[k] * 2) end
  return acc % 9973
end
acc = acc + derive59(1, 2, 59)
end

do
local function project60(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 38.07, bag[k] * 2) end
  while acc > 9.27 * 40 do acc = acc / 2 end
  while acc > 46.28 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 3.81 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 24.29, label = "f60_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 47.87 / 17 + bag[1] else acc = acc - 47.87 end
  local cb = function(v) return v * acc + 20.63 end
  acc = cb(bag[3])
  while acc > 31.14 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 7.42 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 28.30, label = "f60_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 24.61, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 18.58, bag[k] * 2) end
  local tmp = { x = acc, y = 3.10, label = "f60_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 48.67, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project60(1, 2, 60)
end

do
local function clampv61(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 25.79, label = "f61_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 11.08, label = "f61_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 25.24 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 28.41 / 17 + bag[1] else acc = acc - 28.41 end
  while acc > 16.73 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 38.14 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 2.97, bag[k] * 2) end
  while acc > 2.01 * 40 do acc = acc / 2 end
  while acc > 30.72 * 40 do acc = acc / 2 end
  while acc > 16.67 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 24.72, label = "f61_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 10.70 / 17 + bag[1] else acc = acc - 10.70 end
  local cb = function(v) return v * acc + 25.39 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 27.17 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + clampv61(1, 2, 61)
end

do
local function clampv62(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 0.96 / 17 + bag[1] else acc = acc - 0.96 end
  local tmp = { x = acc, y = 11.17, label = "f62_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 5.74 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 33.18 / 17 + bag[1] else acc = acc - 33.18 end
  if acc % 7 > 3 then acc = acc * 39.25 / 17 + bag[1] else acc = acc - 39.25 end
  if acc % 7 > 3 then acc = acc * 49.71 / 17 + bag[1] else acc = acc - 49.71 end
  for k = 1, 4 do acc = math.min(acc + k * 3.92, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 15.64 / 17 + bag[1] else acc = acc - 15.64 end
  local cb = function(v) return v * acc + 7.67 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 24.70 / 17 + bag[1] else acc = acc - 24.70 end
  local cb = function(v) return v * acc + 24.74 end
  acc = cb(bag[3])
  while acc > 13.80 * 40 do acc = acc / 2 end
  while acc > 23.45 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 14.25 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + clampv62(1, 2, 62)
end

do
local function fold63(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 42.92, label = "f63_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 47.61 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 31.81, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 45.93 / 17 + bag[1] else acc = acc - 45.93 end
  while acc > 40.27 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 48.48, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 3.45, bag[k] * 2) end
  local tmp = { x = acc, y = 38.59, label = "f63_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 31.06 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 12.49, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 49.28, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 42.33 / 17 + bag[1] else acc = acc - 42.33 end
  for k = 1, 4 do acc = math.min(acc + k * 7.72, bag[k] * 2) end
  local cb = function(v) return v * acc + 39.06 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + fold63(1, 2, 63)
end

do
local function weight64(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 21.19, bag[k] * 2) end
  local cb = function(v) return v * acc + 24.68 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 48.27 / 17 + bag[1] else acc = acc - 48.27 end
  while acc > 15.54 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 7.09, bag[k] * 2) end
  while acc > 22.97 * 40 do acc = acc / 2 end
  while acc > 20.94 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 1.62 / 17 + bag[1] else acc = acc - 1.62 end
  while acc > 4.46 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 48.54 end
  acc = cb(bag[2])
  while acc > 27.19 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 48.73, label = "f64_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 35.09 / 17 + bag[1] else acc = acc - 35.09 end
  while acc > 49.33 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight64(1, 2, 64)
end

do
local function fold65(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 4.75 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 4.73, bag[k] * 2) end
  local tmp = { x = acc, y = 37.56, label = "f65_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 10.40, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 36.03, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 7.58 / 17 + bag[1] else acc = acc - 7.58 end
  local tmp = { x = acc, y = 41.97, label = "f65_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 13.77, label = "f65_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 13.22, label = "f65_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 17.12, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 27.55, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 48.58 / 17 + bag[1] else acc = acc - 48.58 end
  for k = 1, 4 do acc = math.min(acc + k * 3.61, bag[k] * 2) end
  local tmp = { x = acc, y = 33.93, label = "f65_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold65(1, 2, 65)
end

do
local function compose66(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 5.60 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 38.23, label = "f66_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 25.79 / 17 + bag[1] else acc = acc - 25.79 end
  if acc % 7 > 3 then acc = acc * 7.98 / 17 + bag[1] else acc = acc - 7.98 end
  local tmp = { x = acc, y = 15.26, label = "f66_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 32.89 / 17 + bag[1] else acc = acc - 32.89 end
  for k = 1, 4 do acc = math.min(acc + k * 22.85, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 8.97 / 17 + bag[1] else acc = acc - 8.97 end
  local tmp = { x = acc, y = 23.86, label = "f66_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 2.20 * 40 do acc = acc / 2 end
  while acc > 36.66 * 40 do acc = acc / 2 end
  while acc > 18.40 * 40 do acc = acc / 2 end
  while acc > 16.21 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 36.34 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + compose66(1, 2, 66)
end

do
local function project67(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 16.83 / 17 + bag[1] else acc = acc - 16.83 end
  for k = 1, 4 do acc = math.min(acc + k * 29.17, bag[k] * 2) end
  while acc > 38.18 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 26.34, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 32.35, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 48.49, bag[k] * 2) end
  while acc > 42.32 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 0.08 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 24.45, label = "f67_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 2.03, label = "f67_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 26.35 * 40 do acc = acc / 2 end
  while acc > 36.48 * 40 do acc = acc / 2 end
  while acc > 49.36 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 42.04, label = "f67_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + project67(1, 2, 67)
end

do
local function clampv68(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 1.99, bag[k] * 2) end
  local cb = function(v) return v * acc + 7.66 end
  acc = cb(bag[2])
  while acc > 38.11 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 4.23, label = "f68_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 10.16, label = "f68_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 7.88 * 40 do acc = acc / 2 end
  while acc > 11.80 * 40 do acc = acc / 2 end
  while acc > 17.91 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 18.86, label = "f68_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 36.61 end
  acc = cb(bag[2])
  while acc > 35.60 * 40 do acc = acc / 2 end
  while acc > 35.41 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 40.27, label = "f68_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 47.16 / 17 + bag[1] else acc = acc - 47.16 end
  return acc % 9973
end
acc = acc + clampv68(1, 2, 68)
end

do
local function clampv69(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 10.62, bag[k] * 2) end
  while acc > 42.20 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 27.27 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 41.63 / 17 + bag[1] else acc = acc - 41.63 end
  local tmp = { x = acc, y = 42.85, label = "f69_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 48.62, label = "f69_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 0.87 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 48.25, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 29.18, bag[k] * 2) end
  local cb = function(v) return v * acc + 13.14 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 23.99 / 17 + bag[1] else acc = acc - 23.99 end
  for k = 1, 4 do acc = math.min(acc + k * 17.03, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 26.77 / 17 + bag[1] else acc = acc - 26.77 end
  local tmp = { x = acc, y = 35.29, label = "f69_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv69(1, 2, 69)
end

do
local function fold70(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 24.83 * 40 do acc = acc / 2 end
  while acc > 24.81 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 8.12 / 17 + bag[1] else acc = acc - 8.12 end
  local cb = function(v) return v * acc + 29.98 end
  acc = cb(bag[4])
  while acc > 1.39 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 7.86, label = "f70_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 26.35, bag[k] * 2) end
  local cb = function(v) return v * acc + 19.62 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 16.34 / 17 + bag[1] else acc = acc - 16.34 end
  local cb = function(v) return v * acc + 12.40 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 8.06, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 5.44, bag[k] * 2) end
  local tmp = { x = acc, y = 33.05, label = "f70_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 18.30 / 17 + bag[1] else acc = acc - 18.30 end
  return acc % 9973
end
acc = acc + fold70(1, 2, 70)
end

do
local function measure71(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 26.51 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 49.54, bag[k] * 2) end
  while acc > 19.00 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 47.22, bag[k] * 2) end
  while acc > 34.92 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 38.18, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 15.37, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 9.57, bag[k] * 2) end
  while acc > 1.88 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 29.77 end
  acc = cb(bag[2])
  while acc > 41.74 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 22.46 / 17 + bag[1] else acc = acc - 22.46 end
  local tmp = { x = acc, y = 9.83, label = "f71_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 4.50 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + measure71(1, 2, 71)
end

do
local function weight72(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 11.05 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 16.44, label = "f72_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 10.71, bag[k] * 2) end
  local cb = function(v) return v * acc + 18.56 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 9.40 / 17 + bag[1] else acc = acc - 9.40 end
  while acc > 46.71 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 3.74 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 21.27 / 17 + bag[1] else acc = acc - 21.27 end
  if acc % 7 > 3 then acc = acc * 46.60 / 17 + bag[1] else acc = acc - 46.60 end
  if acc % 7 > 3 then acc = acc * 45.83 / 17 + bag[1] else acc = acc - 45.83 end
  local cb = function(v) return v * acc + 1.79 end
  acc = cb(bag[3])
  while acc > 0.83 * 40 do acc = acc / 2 end
  while acc > 2.01 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 23.65, bag[k] * 2) end
  return acc % 9973
end
acc = acc + weight72(1, 2, 72)
end

do
local function measure73(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 3.14 / 17 + bag[1] else acc = acc - 3.14 end
  if acc % 7 > 3 then acc = acc * 7.08 / 17 + bag[1] else acc = acc - 7.08 end
  if acc % 7 > 3 then acc = acc * 22.34 / 17 + bag[1] else acc = acc - 22.34 end
  for k = 1, 4 do acc = math.min(acc + k * 5.34, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 10.09 / 17 + bag[1] else acc = acc - 10.09 end
  for k = 1, 4 do acc = math.min(acc + k * 1.25, bag[k] * 2) end
  local cb = function(v) return v * acc + 16.41 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 47.24 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 9.86, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 32.05 / 17 + bag[1] else acc = acc - 32.05 end
  for k = 1, 4 do acc = math.min(acc + k * 14.32, bag[k] * 2) end
  while acc > 14.63 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 26.95 / 17 + bag[1] else acc = acc - 26.95 end
  local tmp = { x = acc, y = 14.62, label = "f73_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + measure73(1, 2, 73)
end

do
local function weight74(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 44.55 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 37.48 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 35.68 / 17 + bag[1] else acc = acc - 35.68 end
  local cb = function(v) return v * acc + 45.11 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 29.79 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 24.92 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 11.90, bag[k] * 2) end
  while acc > 29.13 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 19.27, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 44.41 / 17 + bag[1] else acc = acc - 44.41 end
  for k = 1, 4 do acc = math.min(acc + k * 40.43, bag[k] * 2) end
  local cb = function(v) return v * acc + 7.49 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 49.05 / 17 + bag[1] else acc = acc - 49.05 end
  local tmp = { x = acc, y = 4.40, label = "f74_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + weight74(1, 2, 74)
end

do
local function weight75(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 12.55, label = "f75_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 9.23 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 13.04, bag[k] * 2) end
  local cb = function(v) return v * acc + 11.63 end
  acc = cb(bag[4])
  while acc > 44.31 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 21.68 end
  acc = cb(bag[2])
  while acc > 22.16 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 18.50, bag[k] * 2) end
  while acc > 24.02 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 45.40, bag[k] * 2) end
  local tmp = { x = acc, y = 25.51, label = "f75_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 23.44 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 48.15 end
  acc = cb(bag[1])
  while acc > 4.63 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight75(1, 2, 75)
end

do
local function fold76(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 41.06, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 8.72, bag[k] * 2) end
  local cb = function(v) return v * acc + 12.41 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 9.29 end
  acc = cb(bag[4])
  while acc > 41.91 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 29.30 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 18.17 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 34.23 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 16.05 / 17 + bag[1] else acc = acc - 16.05 end
  while acc > 1.37 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 48.11 / 17 + bag[1] else acc = acc - 48.11 end
  for k = 1, 4 do acc = math.min(acc + k * 10.63, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 35.63 / 17 + bag[1] else acc = acc - 35.63 end
  if acc % 7 > 3 then acc = acc * 44.44 / 17 + bag[1] else acc = acc - 44.44 end
  return acc % 9973
end
acc = acc + fold76(1, 2, 76)
end

do
local function clampv77(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 11.47 / 17 + bag[1] else acc = acc - 11.47 end
  local cb = function(v) return v * acc + 23.00 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 46.73 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 25.87 / 17 + bag[1] else acc = acc - 25.87 end
  local tmp = { x = acc, y = 12.72, label = "f77_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 40.02 * 40 do acc = acc / 2 end
  while acc > 4.47 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 20.60, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 17.58 / 17 + bag[1] else acc = acc - 17.58 end
  for k = 1, 4 do acc = math.min(acc + k * 2.98, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 23.05, bag[k] * 2) end
  local cb = function(v) return v * acc + 48.49 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 23.87, label = "f77_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 28.56, label = "f77_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv77(1, 2, 77)
end

do
local function clampv78(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 20.15, label = "f78_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 32.77 end
  acc = cb(bag[2])
  while acc > 9.51 * 40 do acc = acc / 2 end
  while acc > 22.35 * 40 do acc = acc / 2 end
  while acc > 38.81 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 16.36, bag[k] * 2) end
  while acc > 36.51 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 42.82, bag[k] * 2) end
  while acc > 33.30 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 5.26, label = "f78_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 13.03, label = "f78_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 3.67, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 46.00 / 17 + bag[1] else acc = acc - 46.00 end
  for k = 1, 4 do acc = math.min(acc + k * 42.96, bag[k] * 2) end
  return acc % 9973
end
acc = acc + clampv78(1, 2, 78)
end

do
local function fold79(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 40.47, bag[k] * 2) end
  local tmp = { x = acc, y = 30.90, label = "f79_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 15.83 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 49.65, label = "f79_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 42.45, bag[k] * 2) end
  local tmp = { x = acc, y = 48.15, label = "f79_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 46.54 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 33.36, bag[k] * 2) end
  local cb = function(v) return v * acc + 39.43 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 47.14 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 1.82 / 17 + bag[1] else acc = acc - 1.82 end
  local tmp = { x = acc, y = 41.95, label = "f79_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 36.34, label = "f79_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 34.55, label = "f79_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold79(1, 2, 79)
end

do
local function measure80(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 10.25 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 31.22 / 17 + bag[1] else acc = acc - 31.22 end
  local tmp = { x = acc, y = 49.71, label = "f80_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 1.78, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 1.23, bag[k] * 2) end
  while acc > 44.51 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 23.01, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 4.67 / 17 + bag[1] else acc = acc - 4.67 end
  for k = 1, 4 do acc = math.min(acc + k * 30.59, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 7.90, bag[k] * 2) end
  local cb = function(v) return v * acc + 45.74 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 30.16, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 43.94 / 17 + bag[1] else acc = acc - 43.94 end
  for k = 1, 4 do acc = math.min(acc + k * 11.94, bag[k] * 2) end
  return acc % 9973
end
acc = acc + measure80(1, 2, 80)
end

do
local function measure81(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 34.46, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 33.82, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 13.29 / 17 + bag[1] else acc = acc - 13.29 end
  if acc % 7 > 3 then acc = acc * 2.73 / 17 + bag[1] else acc = acc - 2.73 end
  for k = 1, 4 do acc = math.min(acc + k * 39.16, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 19.08, bag[k] * 2) end
  while acc > 28.11 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 21.84, bag[k] * 2) end
  while acc > 21.26 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 45.12, label = "f81_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 45.22 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 29.47 end
  acc = cb(bag[4])
  while acc > 17.62 * 40 do acc = acc / 2 end
  while acc > 16.35 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + measure81(1, 2, 81)
end

do
local function derive82(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 15.55 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 33.36, label = "f82_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 25.50, label = "f82_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 8.63 / 17 + bag[1] else acc = acc - 8.63 end
  if acc % 7 > 3 then acc = acc * 10.14 / 17 + bag[1] else acc = acc - 10.14 end
  if acc % 7 > 3 then acc = acc * 9.02 / 17 + bag[1] else acc = acc - 9.02 end
  for k = 1, 4 do acc = math.min(acc + k * 49.45, bag[k] * 2) end
  while acc > 1.39 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 1.51 / 17 + bag[1] else acc = acc - 1.51 end
  local cb = function(v) return v * acc + 34.51 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 12.21, bag[k] * 2) end
  local tmp = { x = acc, y = 3.58, label = "f82_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 17.65 / 17 + bag[1] else acc = acc - 17.65 end
  if acc % 7 > 3 then acc = acc * 18.67 / 17 + bag[1] else acc = acc - 18.67 end
  return acc % 9973
end
acc = acc + derive82(1, 2, 82)
end

do
local function fold83(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 26.13, bag[k] * 2) end
  local cb = function(v) return v * acc + 43.70 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 44.84 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 44.57 / 17 + bag[1] else acc = acc - 44.57 end
  while acc > 34.10 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 43.06, label = "f83_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 10.30 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 27.87 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 11.68 / 17 + bag[1] else acc = acc - 11.68 end
  local tmp = { x = acc, y = 13.09, label = "f83_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 9.08, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 11.20, bag[k] * 2) end
  local tmp = { x = acc, y = 4.65, label = "f83_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 34.88 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + fold83(1, 2, 83)
end

do
local function weight84(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 21.70 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 41.18 / 17 + bag[1] else acc = acc - 41.18 end
  if acc % 7 > 3 then acc = acc * 19.41 / 17 + bag[1] else acc = acc - 19.41 end
  for k = 1, 4 do acc = math.min(acc + k * 7.73, bag[k] * 2) end
  local tmp = { x = acc, y = 42.39, label = "f84_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 42.09 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 29.09 end
  acc = cb(bag[3])
  while acc > 40.65 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 35.06, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 42.50 / 17 + bag[1] else acc = acc - 42.50 end
  local tmp = { x = acc, y = 42.68, label = "f84_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 23.12 / 17 + bag[1] else acc = acc - 23.12 end
  local tmp = { x = acc, y = 10.80, label = "f84_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 29.83 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight84(1, 2, 84)
end

do
local function project85(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 19.43 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 28.13, label = "f85_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 17.84 * 40 do acc = acc / 2 end
  while acc > 18.03 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 38.87 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 31.99, bag[k] * 2) end
  local tmp = { x = acc, y = 49.55, label = "f85_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 42.39 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 15.94 end
  acc = cb(bag[1])
  while acc > 40.78 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 9.36, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 31.37 / 17 + bag[1] else acc = acc - 31.37 end
  local cb = function(v) return v * acc + 33.35 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 14.55, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project85(1, 2, 85)
end

do
local function fold86(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 10.18, label = "f86_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 49.13, label = "f86_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 1.15 / 17 + bag[1] else acc = acc - 1.15 end
  for k = 1, 4 do acc = math.min(acc + k * 42.04, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 0.52, bag[k] * 2) end
  local tmp = { x = acc, y = 20.88, label = "f86_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 26.59 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 14.89, label = "f86_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 6.41 / 17 + bag[1] else acc = acc - 6.41 end
  local tmp = { x = acc, y = 42.00, label = "f86_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 13.80, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 17.74 / 17 + bag[1] else acc = acc - 17.74 end
  local cb = function(v) return v * acc + 22.78 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 47.03, label = "f86_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold86(1, 2, 86)
end

do
local function clampv87(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 47.60, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 48.28, bag[k] * 2) end
  local cb = function(v) return v * acc + 45.31 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 49.83 end
  acc = cb(bag[4])
  while acc > 40.86 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 2.91 / 17 + bag[1] else acc = acc - 2.91 end
  while acc > 12.59 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 3.79 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 23.70, label = "f87_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 28.92 * 40 do acc = acc / 2 end
  while acc > 1.14 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 2.89 / 17 + bag[1] else acc = acc - 2.89 end
  while acc > 34.24 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 8.49 / 17 + bag[1] else acc = acc - 8.49 end
  return acc % 9973
end
acc = acc + clampv87(1, 2, 87)
end

do
local function project88(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 40.92, label = "f88_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 39.00 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 45.51, bag[k] * 2) end
  local cb = function(v) return v * acc + 17.29 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 44.43 / 17 + bag[1] else acc = acc - 44.43 end
  if acc % 7 > 3 then acc = acc * 7.16 / 17 + bag[1] else acc = acc - 7.16 end
  for k = 1, 4 do acc = math.min(acc + k * 7.13, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 2.29, bag[k] * 2) end
  local cb = function(v) return v * acc + 28.80 end
  acc = cb(bag[1])
  while acc > 11.62 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 26.68 / 17 + bag[1] else acc = acc - 26.68 end
  if acc % 7 > 3 then acc = acc * 33.73 / 17 + bag[1] else acc = acc - 33.73 end
  for k = 1, 4 do acc = math.min(acc + k * 30.20, bag[k] * 2) end
  local tmp = { x = acc, y = 33.16, label = "f88_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + project88(1, 2, 88)
end

do
local function compose89(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 1.25 / 17 + bag[1] else acc = acc - 1.25 end
  while acc > 21.51 * 40 do acc = acc / 2 end
  while acc > 28.65 * 40 do acc = acc / 2 end
  while acc > 42.91 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 13.09 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 24.52 end
  acc = cb(bag[2])
  while acc > 35.38 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 25.15 / 17 + bag[1] else acc = acc - 25.15 end
  local tmp = { x = acc, y = 18.40, label = "f89_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 6.94 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 1.58 / 17 + bag[1] else acc = acc - 1.58 end
  for k = 1, 4 do acc = math.min(acc + k * 39.75, bag[k] * 2) end
  while acc > 24.72 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 15.26 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + compose89(1, 2, 89)
end

do
local function project90(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 27.98 end
  acc = cb(bag[1])
  while acc > 3.45 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 18.89, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 45.80, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 46.95 / 17 + bag[1] else acc = acc - 46.95 end
  local cb = function(v) return v * acc + 26.85 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 47.69, label = "f90_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 16.63, bag[k] * 2) end
  local tmp = { x = acc, y = 42.13, label = "f90_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 28.74 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 35.22, label = "f90_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 36.96, bag[k] * 2) end
  local cb = function(v) return v * acc + 14.83 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 16.08, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project90(1, 2, 90)
end

do
local function compose91(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 32.04 / 17 + bag[1] else acc = acc - 32.04 end
  local cb = function(v) return v * acc + 31.96 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 37.17, bag[k] * 2) end
  local tmp = { x = acc, y = 28.91, label = "f91_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 15.47, label = "f91_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 48.10 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 30.91 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 7.51 / 17 + bag[1] else acc = acc - 7.51 end
  for k = 1, 4 do acc = math.min(acc + k * 47.58, bag[k] * 2) end
  while acc > 31.42 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 30.49, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 23.54 / 17 + bag[1] else acc = acc - 23.54 end
  for k = 1, 4 do acc = math.min(acc + k * 10.69, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 7.89 / 17 + bag[1] else acc = acc - 7.89 end
  return acc % 9973
end
acc = acc + compose91(1, 2, 91)
end

do
local function weight92(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 27.16, bag[k] * 2) end
  while acc > 3.12 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 20.41 / 17 + bag[1] else acc = acc - 20.41 end
  for k = 1, 4 do acc = math.min(acc + k * 25.68, bag[k] * 2) end
  local cb = function(v) return v * acc + 40.51 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 13.60, label = "f92_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 48.22, bag[k] * 2) end
  local cb = function(v) return v * acc + 29.65 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 10.27 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 13.54 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 20.47 / 17 + bag[1] else acc = acc - 20.47 end
  while acc > 6.50 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 31.14 / 17 + bag[1] else acc = acc - 31.14 end
  while acc > 0.54 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight92(1, 2, 92)
end

do
local function project93(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 46.37 / 17 + bag[1] else acc = acc - 46.37 end
  if acc % 7 > 3 then acc = acc * 22.14 / 17 + bag[1] else acc = acc - 22.14 end
  local cb = function(v) return v * acc + 28.63 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 31.45 / 17 + bag[1] else acc = acc - 31.45 end
  local tmp = { x = acc, y = 18.36, label = "f93_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 11.14 end
  acc = cb(bag[2])
  while acc > 47.29 * 40 do acc = acc / 2 end
  while acc > 10.81 * 40 do acc = acc / 2 end
  while acc > 33.18 * 40 do acc = acc / 2 end
  while acc > 40.74 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 7.95 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 28.79 / 17 + bag[1] else acc = acc - 28.79 end
  local tmp = { x = acc, y = 5.77, label = "f93_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 18.81, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project93(1, 2, 93)
end

do
local function derive94(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 4.61 * 40 do acc = acc / 2 end
  while acc > 37.17 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 14.64, bag[k] * 2) end
  local cb = function(v) return v * acc + 38.45 end
  acc = cb(bag[4])
  while acc > 36.84 * 40 do acc = acc / 2 end
  while acc > 41.42 * 40 do acc = acc / 2 end
  while acc > 20.68 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 47.25 end
  acc = cb(bag[4])
  while acc > 27.92 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 13.87, label = "f94_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 35.99, label = "f94_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 5.96, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 39.50, bag[k] * 2) end
  while acc > 23.19 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive94(1, 2, 94)
end

do
local function measure95(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 18.05, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 16.31 / 17 + bag[1] else acc = acc - 16.31 end
  for k = 1, 4 do acc = math.min(acc + k * 16.61, bag[k] * 2) end
  while acc > 12.73 * 40 do acc = acc / 2 end
  while acc > 1.95 * 40 do acc = acc / 2 end
  while acc > 40.87 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 22.01 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 21.83, bag[k] * 2) end
  while acc > 30.60 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 49.98 / 17 + bag[1] else acc = acc - 49.98 end
  while acc > 26.52 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 43.90, label = "f95_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 14.06, bag[k] * 2) end
  local tmp = { x = acc, y = 3.34, label = "f95_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + measure95(1, 2, 95)
end

do
local function derive96(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 15.74 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 31.30 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 10.68, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 10.63 / 17 + bag[1] else acc = acc - 10.63 end
  while acc > 38.36 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 35.32, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 2.88, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 47.99, bag[k] * 2) end
  local tmp = { x = acc, y = 1.18, label = "f96_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 10.31 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 19.83, label = "f96_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 9.97, bag[k] * 2) end
  local cb = function(v) return v * acc + 20.91 end
  acc = cb(bag[1])
  while acc > 27.33 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive96(1, 2, 96)
end

do
local function measure97(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 8.86 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 18.19, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 29.57 / 17 + bag[1] else acc = acc - 29.57 end
  local cb = function(v) return v * acc + 2.53 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 14.86 end
  acc = cb(bag[1])
  while acc > 8.61 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 12.63 end
  acc = cb(bag[3])
  while acc > 42.49 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 46.38, label = "f97_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 27.40, bag[k] * 2) end
  local cb = function(v) return v * acc + 44.56 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 16.89 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 43.10 / 17 + bag[1] else acc = acc - 43.10 end
  local tmp = { x = acc, y = 44.11, label = "f97_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + measure97(1, 2, 97)
end

do
local function project98(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 22.01 / 17 + bag[1] else acc = acc - 22.01 end
  if acc % 7 > 3 then acc = acc * 25.60 / 17 + bag[1] else acc = acc - 25.60 end
  while acc > 3.19 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 7.68 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 46.87, label = "f98_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 28.25 / 17 + bag[1] else acc = acc - 28.25 end
  if acc % 7 > 3 then acc = acc * 19.26 / 17 + bag[1] else acc = acc - 19.26 end
  for k = 1, 4 do acc = math.min(acc + k * 41.52, bag[k] * 2) end
  local tmp = { x = acc, y = 42.13, label = "f98_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 2.52 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 8.10 / 17 + bag[1] else acc = acc - 8.10 end
  for k = 1, 4 do acc = math.min(acc + k * 38.81, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 4.37 / 17 + bag[1] else acc = acc - 4.37 end
  local tmp = { x = acc, y = 16.88, label = "f98_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + project98(1, 2, 98)
end

do
local function fold99(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 7.10, label = "f99_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 31.42, bag[k] * 2) end
  local tmp = { x = acc, y = 37.98, label = "f99_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 31.35 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 30.02 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 37.82 / 17 + bag[1] else acc = acc - 37.82 end
  for k = 1, 4 do acc = math.min(acc + k * 23.34, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 15.05 / 17 + bag[1] else acc = acc - 15.05 end
  while acc > 10.87 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 40.45 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 44.80 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 18.46 / 17 + bag[1] else acc = acc - 18.46 end
  local tmp = { x = acc, y = 33.80, label = "f99_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 1.25, label = "f99_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold99(1, 2, 99)
end

do
local function spread100(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 11.96 / 17 + bag[1] else acc = acc - 11.96 end
  while acc > 4.58 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 49.93 / 17 + bag[1] else acc = acc - 49.93 end
  while acc > 39.54 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 16.15 end
  acc = cb(bag[1])
  while acc > 6.84 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 46.27 / 17 + bag[1] else acc = acc - 46.27 end
  for k = 1, 4 do acc = math.min(acc + k * 4.31, bag[k] * 2) end
  local cb = function(v) return v * acc + 9.97 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 25.49, bag[k] * 2) end
  while acc > 46.71 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 43.30, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 31.94 / 17 + bag[1] else acc = acc - 31.94 end
  while acc > 1.21 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + spread100(1, 2, 100)
end

do
local function fold101(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 17.91 / 17 + bag[1] else acc = acc - 17.91 end
  local tmp = { x = acc, y = 37.71, label = "f101_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 40.56 / 17 + bag[1] else acc = acc - 40.56 end
  local cb = function(v) return v * acc + 23.27 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 3.94 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 12.38, label = "f101_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 32.01 / 17 + bag[1] else acc = acc - 32.01 end
  for k = 1, 4 do acc = math.min(acc + k * 17.28, bag[k] * 2) end
  local cb = function(v) return v * acc + 16.44 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 11.21 / 17 + bag[1] else acc = acc - 11.21 end
  local tmp = { x = acc, y = 7.35, label = "f101_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 14.25 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 3.08 end
  acc = cb(bag[1])
  while acc > 22.39 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold101(1, 2, 101)
end

do
local function compose102(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 1.54 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 6.01 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 44.30, bag[k] * 2) end
  local cb = function(v) return v * acc + 8.02 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 17.13 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 9.51, label = "f102_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 12.70, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 3.14, bag[k] * 2) end
  local cb = function(v) return v * acc + 17.83 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 42.50 / 17 + bag[1] else acc = acc - 42.50 end
  local cb = function(v) return v * acc + 10.32 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 36.38, label = "f102_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 36.78 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 16.48 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + compose102(1, 2, 102)
end

do
local function derive103(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 29.49 / 17 + bag[1] else acc = acc - 29.49 end
  while acc > 24.10 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 14.19, label = "f103_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 39.82, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 44.56 / 17 + bag[1] else acc = acc - 44.56 end
  local cb = function(v) return v * acc + 9.08 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 18.48 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 41.83 / 17 + bag[1] else acc = acc - 41.83 end
  for k = 1, 4 do acc = math.min(acc + k * 38.23, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 18.50 / 17 + bag[1] else acc = acc - 18.50 end
  local tmp = { x = acc, y = 25.50, label = "f103_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 14.50 / 17 + bag[1] else acc = acc - 14.50 end
  if acc % 7 > 3 then acc = acc * 9.34 / 17 + bag[1] else acc = acc - 9.34 end
  local tmp = { x = acc, y = 44.75, label = "f103_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + derive103(1, 2, 103)
end

do
local function clampv104(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 18.86, label = "f104_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 2.85, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 3.98 / 17 + bag[1] else acc = acc - 3.98 end
  for k = 1, 4 do acc = math.min(acc + k * 47.80, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 41.45 / 17 + bag[1] else acc = acc - 41.45 end
  local cb = function(v) return v * acc + 36.92 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 29.85, label = "f104_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 1.67, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 7.53, bag[k] * 2) end
  local cb = function(v) return v * acc + 46.64 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 3.01, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 17.08 / 17 + bag[1] else acc = acc - 17.08 end
  local cb = function(v) return v * acc + 19.86 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 34.08 / 17 + bag[1] else acc = acc - 34.08 end
  return acc % 9973
end
acc = acc + clampv104(1, 2, 104)
end

do
local function compose105(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 28.85 / 17 + bag[1] else acc = acc - 28.85 end
  local tmp = { x = acc, y = 44.34, label = "f105_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 35.07, label = "f105_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 19.60, label = "f105_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 12.13, label = "f105_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 4.07 end
  acc = cb(bag[2])
  while acc > 43.36 * 40 do acc = acc / 2 end
  while acc > 35.10 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 44.07, label = "f105_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 42.98, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 36.33, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 38.42 / 17 + bag[1] else acc = acc - 38.42 end
  if acc % 7 > 3 then acc = acc * 11.00 / 17 + bag[1] else acc = acc - 11.00 end
  if acc % 7 > 3 then acc = acc * 12.75 / 17 + bag[1] else acc = acc - 12.75 end
  return acc % 9973
end
acc = acc + compose105(1, 2, 105)
end

do
local function compose106(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 9.69 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 26.02, label = "f106_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 15.78 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 5.12 / 17 + bag[1] else acc = acc - 5.12 end
  local tmp = { x = acc, y = 39.34, label = "f106_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 9.95, label = "f106_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 29.33 / 17 + bag[1] else acc = acc - 29.33 end
  if acc % 7 > 3 then acc = acc * 15.08 / 17 + bag[1] else acc = acc - 15.08 end
  if acc % 7 > 3 then acc = acc * 4.27 / 17 + bag[1] else acc = acc - 4.27 end
  while acc > 29.85 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 12.55, label = "f106_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 28.42 / 17 + bag[1] else acc = acc - 28.42 end
  if acc % 7 > 3 then acc = acc * 8.96 / 17 + bag[1] else acc = acc - 8.96 end
  for k = 1, 4 do acc = math.min(acc + k * 44.72, bag[k] * 2) end
  return acc % 9973
end
acc = acc + compose106(1, 2, 106)
end

do
local function derive107(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 13.42 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 19.37, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 36.63 / 17 + bag[1] else acc = acc - 36.63 end
  if acc % 7 > 3 then acc = acc * 19.95 / 17 + bag[1] else acc = acc - 19.95 end
  for k = 1, 4 do acc = math.min(acc + k * 33.25, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 18.75, bag[k] * 2) end
  local cb = function(v) return v * acc + 42.24 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 2.93 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 38.79 / 17 + bag[1] else acc = acc - 38.79 end
  local cb = function(v) return v * acc + 11.99 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 11.92 / 17 + bag[1] else acc = acc - 11.92 end
  for k = 1, 4 do acc = math.min(acc + k * 32.89, bag[k] * 2) end
  while acc > 9.51 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 41.22 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + derive107(1, 2, 107)
end

do
local function weight108(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 34.35 * 40 do acc = acc / 2 end
  while acc > 13.41 * 40 do acc = acc / 2 end
  while acc > 6.07 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 42.16, label = "f108_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 23.42, label = "f108_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 41.94, label = "f108_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 42.02, label = "f108_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 10.01 / 17 + bag[1] else acc = acc - 10.01 end
  local tmp = { x = acc, y = 0.19, label = "f108_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 23.92, label = "f108_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 22.27 end
  acc = cb(bag[3])
  while acc > 12.40 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 22.90, label = "f108_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 16.61 / 17 + bag[1] else acc = acc - 16.61 end
  return acc % 9973
end
acc = acc + weight108(1, 2, 108)
end

do
local function weight109(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 32.23, bag[k] * 2) end
  local tmp = { x = acc, y = 44.52, label = "f109_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 23.77, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 2.29, bag[k] * 2) end
  local tmp = { x = acc, y = 40.89, label = "f109_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 40.89, label = "f109_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 24.52, label = "f109_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 8.52, bag[k] * 2) end
  local tmp = { x = acc, y = 16.01, label = "f109_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 29.70 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 1.04, label = "f109_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 35.77, label = "f109_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 41.65 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 40.11 / 17 + bag[1] else acc = acc - 40.11 end
  return acc % 9973
end
acc = acc + weight109(1, 2, 109)
end

do
local function compose110(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 42.74, label = "f110_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 35.25 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 22.66, label = "f110_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 1.71, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 0.57 / 17 + bag[1] else acc = acc - 0.57 end
  for k = 1, 4 do acc = math.min(acc + k * 33.53, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 7.54 / 17 + bag[1] else acc = acc - 7.54 end
  local cb = function(v) return v * acc + 45.74 end
  acc = cb(bag[4])
  while acc > 10.25 * 40 do acc = acc / 2 end
  while acc > 35.86 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 29.38, label = "f110_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 25.51 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 21.83, bag[k] * 2) end
  local tmp = { x = acc, y = 20.87, label = "f110_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + compose110(1, 2, 110)
end

do
local function weight111(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 7.77 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 17.40 end
  acc = cb(bag[2])
  while acc > 27.62 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 2.71 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 28.43 / 17 + bag[1] else acc = acc - 28.43 end
  local cb = function(v) return v * acc + 23.40 end
  acc = cb(bag[2])
  while acc > 13.75 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 33.30, bag[k] * 2) end
  while acc > 23.86 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 14.31 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 42.27, bag[k] * 2) end
  local tmp = { x = acc, y = 15.01, label = "f111_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 26.75, label = "f111_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 23.50, bag[k] * 2) end
  return acc % 9973
end
acc = acc + weight111(1, 2, 111)
end

do
local function clampv112(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 15.01, label = "f112_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 36.52 * 40 do acc = acc / 2 end
  while acc > 22.17 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 45.51 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 20.95 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 27.62 / 17 + bag[1] else acc = acc - 27.62 end
  if acc % 7 > 3 then acc = acc * 12.97 / 17 + bag[1] else acc = acc - 12.97 end
  for k = 1, 4 do acc = math.min(acc + k * 45.10, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 47.74, bag[k] * 2) end
  local tmp = { x = acc, y = 18.78, label = "f112_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 29.76 / 17 + bag[1] else acc = acc - 29.76 end
  while acc > 34.42 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 5.43 / 17 + bag[1] else acc = acc - 5.43 end
  if acc % 7 > 3 then acc = acc * 24.44 / 17 + bag[1] else acc = acc - 24.44 end
  return acc % 9973
end
acc = acc + clampv112(1, 2, 112)
end

do
local function derive113(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 27.99 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 44.62, label = "f113_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 2.57 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 9.27, label = "f113_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 14.35, bag[k] * 2) end
  local cb = function(v) return v * acc + 40.15 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 28.90, bag[k] * 2) end
  local cb = function(v) return v * acc + 9.03 end
  acc = cb(bag[4])
  while acc > 29.31 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 24.63 / 17 + bag[1] else acc = acc - 24.63 end
  while acc > 26.92 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 9.99, label = "f113_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 23.53 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 19.10 / 17 + bag[1] else acc = acc - 19.10 end
  return acc % 9973
end
acc = acc + derive113(1, 2, 113)
end

do
local function compose114(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 19.25, bag[k] * 2) end
  while acc > 5.62 * 40 do acc = acc / 2 end
  while acc > 33.68 * 40 do acc = acc / 2 end
  while acc > 42.82 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 36.44 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 32.54, label = "f114_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 0.76 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 23.43, label = "f114_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 8.60 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 40.86, label = "f114_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 34.16 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 28.46 / 17 + bag[1] else acc = acc - 28.46 end
  local cb = function(v) return v * acc + 1.31 end
  acc = cb(bag[1])
  while acc > 24.73 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + compose114(1, 2, 114)
end

do
local function weight115(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 21.91 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 20.38 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 1.43 end
  acc = cb(bag[3])
  while acc > 18.14 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 21.18 / 17 + bag[1] else acc = acc - 21.18 end
  local cb = function(v) return v * acc + 8.49 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 48.30, label = "f115_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 0.55 * 40 do acc = acc / 2 end
  while acc > 6.50 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 33.33, bag[k] * 2) end
  local cb = function(v) return v * acc + 40.46 end
  acc = cb(bag[3])
  while acc > 42.90 * 40 do acc = acc / 2 end
  while acc > 24.23 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 14.57, label = "f115_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + weight115(1, 2, 115)
end

do
local function fold116(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 17.02 end
  acc = cb(bag[1])
  while acc > 4.18 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 18.99 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 12.07 end
  acc = cb(bag[4])
  while acc > 41.83 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 10.70, label = "f116_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 11.38, bag[k] * 2) end
  while acc > 19.31 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 36.53, label = "f116_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 19.10, bag[k] * 2) end
  local cb = function(v) return v * acc + 33.37 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 39.48, label = "f116_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 20.58 end
  acc = cb(bag[1])
  while acc > 17.41 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold116(1, 2, 116)
end

do
local function derive117(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 34.28, bag[k] * 2) end
  while acc > 8.68 * 40 do acc = acc / 2 end
  while acc > 7.95 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 27.52, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 19.44 / 17 + bag[1] else acc = acc - 19.44 end
  for k = 1, 4 do acc = math.min(acc + k * 34.75, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 11.18 / 17 + bag[1] else acc = acc - 11.18 end
  local tmp = { x = acc, y = 38.21, label = "f117_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 15.96 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 8.69, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 44.60, bag[k] * 2) end
  local cb = function(v) return v * acc + 20.67 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 0.99 / 17 + bag[1] else acc = acc - 0.99 end
  local cb = function(v) return v * acc + 39.38 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + derive117(1, 2, 117)
end

do
local function spread118(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 21.89, label = "f118_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 27.85, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 49.10 / 17 + bag[1] else acc = acc - 49.10 end
  if acc % 7 > 3 then acc = acc * 10.66 / 17 + bag[1] else acc = acc - 10.66 end
  while acc > 10.08 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 42.18 / 17 + bag[1] else acc = acc - 42.18 end
  while acc > 15.29 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 14.47, bag[k] * 2) end
  while acc > 42.99 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 22.32 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 7.44, label = "f118_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 5.28 / 17 + bag[1] else acc = acc - 5.28 end
  for k = 1, 4 do acc = math.min(acc + k * 23.61, bag[k] * 2) end
  while acc > 3.94 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + spread118(1, 2, 118)
end

do
local function clampv119(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 8.32 / 17 + bag[1] else acc = acc - 8.32 end
  while acc > 46.33 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 21.95 / 17 + bag[1] else acc = acc - 21.95 end
  local tmp = { x = acc, y = 42.48, label = "f119_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 38.64 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 44.41 / 17 + bag[1] else acc = acc - 44.41 end
  for k = 1, 4 do acc = math.min(acc + k * 11.21, bag[k] * 2) end
  while acc > 8.38 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 1.57, label = "f119_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 42.91 end
  acc = cb(bag[2])
  while acc > 44.39 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 2.16, label = "f119_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 36.78 / 17 + bag[1] else acc = acc - 36.78 end
  local cb = function(v) return v * acc + 27.61 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + clampv119(1, 2, 119)
end

do
local function compose120(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 36.69 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 41.26, label = "f120_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 20.09 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 4.38, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 30.69 / 17 + bag[1] else acc = acc - 30.69 end
  if acc % 7 > 3 then acc = acc * 34.44 / 17 + bag[1] else acc = acc - 34.44 end
  for k = 1, 4 do acc = math.min(acc + k * 43.12, bag[k] * 2) end
  while acc > 30.81 * 40 do acc = acc / 2 end
  while acc > 30.34 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 49.64, label = "f120_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 30.63 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 27.07 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 39.73 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 35.66 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + compose120(1, 2, 120)
end

do
local function measure121(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 12.87, label = "f121_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 30.07 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 24.10 / 17 + bag[1] else acc = acc - 24.10 end
  if acc % 7 > 3 then acc = acc * 20.16 / 17 + bag[1] else acc = acc - 20.16 end
  local cb = function(v) return v * acc + 45.49 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 37.97, label = "f121_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 12.71 / 17 + bag[1] else acc = acc - 12.71 end
  for k = 1, 4 do acc = math.min(acc + k * 24.58, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 49.80 / 17 + bag[1] else acc = acc - 49.80 end
  if acc % 7 > 3 then acc = acc * 26.35 / 17 + bag[1] else acc = acc - 26.35 end
  if acc % 7 > 3 then acc = acc * 6.60 / 17 + bag[1] else acc = acc - 6.60 end
  local cb = function(v) return v * acc + 27.89 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 11.60 end
  acc = cb(bag[1])
  while acc > 0.89 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + measure121(1, 2, 121)
end

do
local function weight122(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 42.38 / 17 + bag[1] else acc = acc - 42.38 end
  while acc > 44.33 * 40 do acc = acc / 2 end
  while acc > 16.21 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 31.52 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 45.89, label = "f122_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 15.42, bag[k] * 2) end
  local cb = function(v) return v * acc + 22.40 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 2.56 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 41.05 / 17 + bag[1] else acc = acc - 41.05 end
  for k = 1, 4 do acc = math.min(acc + k * 7.42, bag[k] * 2) end
  local tmp = { x = acc, y = 36.33, label = "f122_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 8.51 end
  acc = cb(bag[4])
  while acc > 48.38 * 40 do acc = acc / 2 end
  while acc > 9.88 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight122(1, 2, 122)
end

do
local function spread123(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 26.41, label = "f123_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 49.42 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 7.90 / 17 + bag[1] else acc = acc - 7.90 end
  local cb = function(v) return v * acc + 24.35 end
  acc = cb(bag[4])
  while acc > 47.67 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 6.93 / 17 + bag[1] else acc = acc - 6.93 end
  if acc % 7 > 3 then acc = acc * 39.56 / 17 + bag[1] else acc = acc - 39.56 end
  for k = 1, 4 do acc = math.min(acc + k * 11.98, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 47.47, bag[k] * 2) end
  while acc > 6.53 * 40 do acc = acc / 2 end
  while acc > 22.19 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 39.59 end
  acc = cb(bag[4])
  while acc > 38.81 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 45.97, label = "f123_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + spread123(1, 2, 123)
end

do
local function measure124(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 35.82 / 17 + bag[1] else acc = acc - 35.82 end
  local tmp = { x = acc, y = 10.21, label = "f124_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 47.44 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 35.84 end
  acc = cb(bag[4])
  while acc > 5.75 * 40 do acc = acc / 2 end
  while acc > 44.24 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 24.60, label = "f124_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 19.92, label = "f124_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 34.23 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 19.35 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 29.42 end
  acc = cb(bag[3])
  while acc > 0.62 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 9.07, bag[k] * 2) end
  local cb = function(v) return v * acc + 9.13 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + measure124(1, 2, 124)
end

do
local function weight125(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 5.80, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 42.84 / 17 + bag[1] else acc = acc - 42.84 end
  local cb = function(v) return v * acc + 48.87 end
  acc = cb(bag[3])
  while acc > 47.34 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 42.85, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 18.13, bag[k] * 2) end
  while acc > 38.48 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 9.25, label = "f125_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 30.22 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 1.90, label = "f125_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 28.47 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 21.00, label = "f125_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 48.02 * 40 do acc = acc / 2 end
  while acc > 49.71 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight125(1, 2, 125)
end

do
local function fold126(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 46.04 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 41.05, label = "f126_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 21.51, label = "f126_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 30.01, label = "f126_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 9.39, bag[k] * 2) end
  local tmp = { x = acc, y = 6.06, label = "f126_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 48.60 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 29.37, label = "f126_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 0.22 / 17 + bag[1] else acc = acc - 0.22 end
  local tmp = { x = acc, y = 12.87, label = "f126_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 43.49 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 17.98 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 1.13 / 17 + bag[1] else acc = acc - 1.13 end
  if acc % 7 > 3 then acc = acc * 9.78 / 17 + bag[1] else acc = acc - 9.78 end
  return acc % 9973
end
acc = acc + fold126(1, 2, 126)
end

do
local function weight127(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 38.08 / 17 + bag[1] else acc = acc - 38.08 end
  for k = 1, 4 do acc = math.min(acc + k * 28.79, bag[k] * 2) end
  while acc > 48.80 * 40 do acc = acc / 2 end
  while acc > 34.65 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 10.73 end
  acc = cb(bag[1])
  while acc > 18.93 * 40 do acc = acc / 2 end
  while acc > 19.99 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 14.89 / 17 + bag[1] else acc = acc - 14.89 end
  while acc > 0.42 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 5.11 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 40.06 / 17 + bag[1] else acc = acc - 40.06 end
  while acc > 10.23 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 5.10 / 17 + bag[1] else acc = acc - 5.10 end
  if acc % 7 > 3 then acc = acc * 40.12 / 17 + bag[1] else acc = acc - 40.12 end
  return acc % 9973
end
acc = acc + weight127(1, 2, 127)
end

do
local function project128(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 49.52 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 43.27, label = "f128_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 8.79 end
  acc = cb(bag[3])
  while acc > 45.46 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 17.14 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 2.49 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 15.93, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 34.18 / 17 + bag[1] else acc = acc - 34.18 end
  local tmp = { x = acc, y = 19.00, label = "f128_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 9.23, bag[k] * 2) end
  while acc > 42.58 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 13.50 / 17 + bag[1] else acc = acc - 13.50 end
  local tmp = { x = acc, y = 5.74, label = "f128_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 9.22 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + project128(1, 2, 128)
end

do
local function measure129(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 40.18, label = "f129_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 11.08 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 38.14 / 17 + bag[1] else acc = acc - 38.14 end
  for k = 1, 4 do acc = math.min(acc + k * 31.31, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 32.46 / 17 + bag[1] else acc = acc - 32.46 end
  while acc > 46.64 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 20.31, label = "f129_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 46.41 / 17 + bag[1] else acc = acc - 46.41 end
  if acc % 7 > 3 then acc = acc * 49.66 / 17 + bag[1] else acc = acc - 49.66 end
  while acc > 24.29 * 40 do acc = acc / 2 end
  while acc > 23.99 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 32.38 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 36.60 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 27.85, label = "f129_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + measure129(1, 2, 129)
end

do
local function measure130(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 34.79, label = "f130_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 33.67 / 17 + bag[1] else acc = acc - 33.67 end
  if acc % 7 > 3 then acc = acc * 0.80 / 17 + bag[1] else acc = acc - 0.80 end
  if acc % 7 > 3 then acc = acc * 28.36 / 17 + bag[1] else acc = acc - 28.36 end
  local cb = function(v) return v * acc + 37.16 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 10.00, label = "f130_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 10.52 / 17 + bag[1] else acc = acc - 10.52 end
  if acc % 7 > 3 then acc = acc * 19.35 / 17 + bag[1] else acc = acc - 19.35 end
  for k = 1, 4 do acc = math.min(acc + k * 25.18, bag[k] * 2) end
  local cb = function(v) return v * acc + 6.92 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 37.35 / 17 + bag[1] else acc = acc - 37.35 end
  while acc > 6.60 * 40 do acc = acc / 2 end
  while acc > 8.80 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 35.26 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + measure130(1, 2, 130)
end

do
local function derive131(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 48.56, label = "f131_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 20.50, label = "f131_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 7.58, label = "f131_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 47.23 / 17 + bag[1] else acc = acc - 47.23 end
  if acc % 7 > 3 then acc = acc * 32.30 / 17 + bag[1] else acc = acc - 32.30 end
  local cb = function(v) return v * acc + 33.02 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 23.40 / 17 + bag[1] else acc = acc - 23.40 end
  local cb = function(v) return v * acc + 29.40 end
  acc = cb(bag[4])
  while acc > 14.52 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 13.41 / 17 + bag[1] else acc = acc - 13.41 end
  local cb = function(v) return v * acc + 7.16 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 20.60 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 2.67, label = "f131_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 41.10 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive131(1, 2, 131)
end

do
local function spread132(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 36.67 / 17 + bag[1] else acc = acc - 36.67 end
  if acc % 7 > 3 then acc = acc * 28.77 / 17 + bag[1] else acc = acc - 28.77 end
  local tmp = { x = acc, y = 3.49, label = "f132_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 33.54, bag[k] * 2) end
  local cb = function(v) return v * acc + 11.90 end
  acc = cb(bag[1])
  while acc > 33.82 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 11.59, bag[k] * 2) end
  local cb = function(v) return v * acc + 5.86 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 16.30 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 17.43 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 28.99, bag[k] * 2) end
  while acc > 29.08 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 28.79 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 34.18, label = "f132_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + spread132(1, 2, 132)
end

do
local function measure133(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 20.83 / 17 + bag[1] else acc = acc - 20.83 end
  if acc % 7 > 3 then acc = acc * 46.96 / 17 + bag[1] else acc = acc - 46.96 end
  if acc % 7 > 3 then acc = acc * 10.50 / 17 + bag[1] else acc = acc - 10.50 end
  while acc > 29.30 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 11.11 / 17 + bag[1] else acc = acc - 11.11 end
  if acc % 7 > 3 then acc = acc * 45.12 / 17 + bag[1] else acc = acc - 45.12 end
  for k = 1, 4 do acc = math.min(acc + k * 23.29, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 6.68 / 17 + bag[1] else acc = acc - 6.68 end
  while acc > 14.51 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 1.32 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 16.33 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 49.58 / 17 + bag[1] else acc = acc - 49.58 end
  if acc % 7 > 3 then acc = acc * 27.09 / 17 + bag[1] else acc = acc - 27.09 end
  while acc > 12.75 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + measure133(1, 2, 133)
end

do
local function fold134(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 0.13, label = "f134_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 45.49, bag[k] * 2) end
  while acc > 21.26 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 0.72, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 33.20 / 17 + bag[1] else acc = acc - 33.20 end
  local cb = function(v) return v * acc + 37.38 end
  acc = cb(bag[2])
  while acc > 23.20 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 23.60, label = "f134_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 8.30 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 41.45 / 17 + bag[1] else acc = acc - 41.45 end
  if acc % 7 > 3 then acc = acc * 37.31 / 17 + bag[1] else acc = acc - 37.31 end
  for k = 1, 4 do acc = math.min(acc + k * 39.48, bag[k] * 2) end
  local tmp = { x = acc, y = 4.91, label = "f134_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 39.86 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold134(1, 2, 134)
end

do
local function measure135(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 18.13, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 29.46, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 47.96 / 17 + bag[1] else acc = acc - 47.96 end
  while acc > 19.69 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 46.42 / 17 + bag[1] else acc = acc - 46.42 end
  local cb = function(v) return v * acc + 37.09 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 9.81, label = "f135_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 45.02 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 33.36, label = "f135_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 32.29, bag[k] * 2) end
  local cb = function(v) return v * acc + 36.05 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 37.93, label = "f135_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 14.53, label = "f135_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 2.47 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + measure135(1, 2, 135)
end

do
local function derive136(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 45.66, bag[k] * 2) end
  local cb = function(v) return v * acc + 30.16 end
  acc = cb(bag[2])
  while acc > 31.78 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 49.72, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 3.37, bag[k] * 2) end
  while acc > 4.85 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 48.67 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 8.58 end
  acc = cb(bag[4])
  while acc > 7.30 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 40.15, bag[k] * 2) end
  local tmp = { x = acc, y = 16.47, label = "f136_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 36.39, label = "f136_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 22.68 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 31.95 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + derive136(1, 2, 136)
end

do
local function compose137(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 42.57 / 17 + bag[1] else acc = acc - 42.57 end
  local tmp = { x = acc, y = 20.91, label = "f137_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 7.00 / 17 + bag[1] else acc = acc - 7.00 end
  if acc % 7 > 3 then acc = acc * 39.78 / 17 + bag[1] else acc = acc - 39.78 end
  local cb = function(v) return v * acc + 9.10 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 19.84 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 14.87 / 17 + bag[1] else acc = acc - 14.87 end
  local cb = function(v) return v * acc + 34.28 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 1.47, label = "f137_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 46.59, bag[k] * 2) end
  while acc > 19.77 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 1.44 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 0.63 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 34.69, bag[k] * 2) end
  return acc % 9973
end
acc = acc + compose137(1, 2, 137)
end

do
local function weight138(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 19.80, label = "f138_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 37.38, label = "f138_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 1.90 / 17 + bag[1] else acc = acc - 1.90 end
  local tmp = { x = acc, y = 5.33, label = "f138_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 10.69 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 0.92 / 17 + bag[1] else acc = acc - 0.92 end
  if acc % 7 > 3 then acc = acc * 23.36 / 17 + bag[1] else acc = acc - 23.36 end
  while acc > 30.62 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 7.03 end
  acc = cb(bag[1])
  while acc > 14.15 * 40 do acc = acc / 2 end
  while acc > 40.49 * 40 do acc = acc / 2 end
  while acc > 28.92 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 33.27 / 17 + bag[1] else acc = acc - 33.27 end
  local cb = function(v) return v * acc + 0.71 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + weight138(1, 2, 138)
end

do
local function weight139(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 33.12 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 21.04, label = "f139_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 37.53 / 17 + bag[1] else acc = acc - 37.53 end
  while acc > 6.58 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 8.94 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 27.23 / 17 + bag[1] else acc = acc - 27.23 end
  for k = 1, 4 do acc = math.min(acc + k * 29.16, bag[k] * 2) end
  local tmp = { x = acc, y = 12.04, label = "f139_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 31.50, label = "f139_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 46.06 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 19.39 / 17 + bag[1] else acc = acc - 19.39 end
  local cb = function(v) return v * acc + 47.23 end
  acc = cb(bag[4])
  while acc > 8.62 * 40 do acc = acc / 2 end
  while acc > 45.35 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight139(1, 2, 139)
end

do
local function weight140(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 30.58 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 29.38, bag[k] * 2) end
  local tmp = { x = acc, y = 5.39, label = "f140_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 37.39 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 5.00 / 17 + bag[1] else acc = acc - 5.00 end
  local tmp = { x = acc, y = 4.75, label = "f140_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 5.41, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 21.63, bag[k] * 2) end
  local cb = function(v) return v * acc + 39.78 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 30.10 end
  acc = cb(bag[2])
  while acc > 3.86 * 40 do acc = acc / 2 end
  while acc > 25.12 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 43.33, label = "f140_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 27.04, label = "f140_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + weight140(1, 2, 140)
end

do
local function fold141(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 16.95 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 39.58, bag[k] * 2) end
  while acc > 12.73 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 30.49, bag[k] * 2) end
  local tmp = { x = acc, y = 20.23, label = "f141_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 34.28, label = "f141_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 28.81, bag[k] * 2) end
  local tmp = { x = acc, y = 21.23, label = "f141_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 43.70 / 17 + bag[1] else acc = acc - 43.70 end
  local cb = function(v) return v * acc + 24.78 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 49.07, label = "f141_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 6.62 / 17 + bag[1] else acc = acc - 6.62 end
  if acc % 7 > 3 then acc = acc * 15.46 / 17 + bag[1] else acc = acc - 15.46 end
  if acc % 7 > 3 then acc = acc * 23.07 / 17 + bag[1] else acc = acc - 23.07 end
  return acc % 9973
end
acc = acc + fold141(1, 2, 141)
end

do
local function project142(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 14.72 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 12.40, label = "f142_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 1.11 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 29.62 / 17 + bag[1] else acc = acc - 29.62 end
  local cb = function(v) return v * acc + 20.49 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 17.78 / 17 + bag[1] else acc = acc - 17.78 end
  if acc % 7 > 3 then acc = acc * 11.45 / 17 + bag[1] else acc = acc - 11.45 end
  while acc > 4.11 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 34.64 end
  acc = cb(bag[1])
  while acc > 33.44 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 8.73 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 30.46, bag[k] * 2) end
  local cb = function(v) return v * acc + 41.47 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 12.18, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project142(1, 2, 142)
end

do
local function measure143(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 36.89, bag[k] * 2) end
  while acc > 4.38 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 39.71, label = "f143_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 49.27 / 17 + bag[1] else acc = acc - 49.27 end
  if acc % 7 > 3 then acc = acc * 19.95 / 17 + bag[1] else acc = acc - 19.95 end
  while acc > 5.17 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 6.35 / 17 + bag[1] else acc = acc - 6.35 end
  if acc % 7 > 3 then acc = acc * 9.67 / 17 + bag[1] else acc = acc - 9.67 end
  local cb = function(v) return v * acc + 6.62 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 37.25, label = "f143_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 36.17, label = "f143_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 0.78 / 17 + bag[1] else acc = acc - 0.78 end
  local tmp = { x = acc, y = 27.66, label = "f143_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 43.49 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + measure143(1, 2, 143)
end

do
local function weight144(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 15.46 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 48.39 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 19.45, label = "f144_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 0.74, bag[k] * 2) end
  while acc > 1.37 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 5.53 / 17 + bag[1] else acc = acc - 5.53 end
  for k = 1, 4 do acc = math.min(acc + k * 25.23, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 43.69, bag[k] * 2) end
  local tmp = { x = acc, y = 21.54, label = "f144_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 36.18 * 40 do acc = acc / 2 end
  while acc > 2.74 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 36.53, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 39.48 / 17 + bag[1] else acc = acc - 39.48 end
  local cb = function(v) return v * acc + 30.24 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + weight144(1, 2, 144)
end

do
local function clampv145(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 20.18, bag[k] * 2) end
  while acc > 29.85 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 9.96 / 17 + bag[1] else acc = acc - 9.96 end
  for k = 1, 4 do acc = math.min(acc + k * 38.92, bag[k] * 2) end
  local cb = function(v) return v * acc + 35.78 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 33.16, bag[k] * 2) end
  while acc > 18.38 * 40 do acc = acc / 2 end
  while acc > 24.37 * 40 do acc = acc / 2 end
  while acc > 4.09 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 43.76, bag[k] * 2) end
  while acc > 25.92 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 35.28 end
  acc = cb(bag[4])
  while acc > 22.38 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 44.62 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + clampv145(1, 2, 145)
end

do
local function spread146(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 45.05 / 17 + bag[1] else acc = acc - 45.05 end
  if acc % 7 > 3 then acc = acc * 30.20 / 17 + bag[1] else acc = acc - 30.20 end
  local cb = function(v) return v * acc + 13.04 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 45.85 / 17 + bag[1] else acc = acc - 45.85 end
  while acc > 20.29 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 35.48 end
  acc = cb(bag[2])
  while acc > 28.83 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 20.34, label = "f146_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 43.12, bag[k] * 2) end
  while acc > 21.37 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 17.42 end
  acc = cb(bag[3])
  while acc > 27.68 * 40 do acc = acc / 2 end
  while acc > 18.26 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 31.39, bag[k] * 2) end
  return acc % 9973
end
acc = acc + spread146(1, 2, 146)
end

do
local function spread147(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 34.08, bag[k] * 2) end
  local cb = function(v) return v * acc + 1.75 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 20.43 / 17 + bag[1] else acc = acc - 20.43 end
  if acc % 7 > 3 then acc = acc * 49.80 / 17 + bag[1] else acc = acc - 49.80 end
  local tmp = { x = acc, y = 14.89, label = "f147_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 19.65 / 17 + bag[1] else acc = acc - 19.65 end
  local cb = function(v) return v * acc + 11.32 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 30.86, bag[k] * 2) end
  local cb = function(v) return v * acc + 35.53 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 3.06 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 13.25 end
  acc = cb(bag[3])
  while acc > 17.18 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 38.27 end
  acc = cb(bag[1])
  while acc > 18.07 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + spread147(1, 2, 147)
end

do
local function derive148(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 16.81, bag[k] * 2) end
  local cb = function(v) return v * acc + 15.24 end
  acc = cb(bag[2])
  while acc > 8.48 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 23.49, label = "f148_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 41.54, label = "f148_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 43.47, bag[k] * 2) end
  local cb = function(v) return v * acc + 13.80 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 9.63, bag[k] * 2) end
  while acc > 0.14 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 40.82 / 17 + bag[1] else acc = acc - 40.82 end
  local tmp = { x = acc, y = 41.23, label = "f148_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 19.02 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 34.51 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 22.56 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + derive148(1, 2, 148)
end

do
local function derive149(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 32.05 / 17 + bag[1] else acc = acc - 32.05 end
  local tmp = { x = acc, y = 29.84, label = "f149_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 44.81 end
  acc = cb(bag[3])
  while acc > 8.64 * 40 do acc = acc / 2 end
  while acc > 27.22 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 43.64, label = "f149_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 5.97, label = "f149_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 13.47 / 17 + bag[1] else acc = acc - 13.47 end
  local tmp = { x = acc, y = 4.92, label = "f149_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 44.16 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 14.49 / 17 + bag[1] else acc = acc - 14.49 end
  for k = 1, 4 do acc = math.min(acc + k * 47.07, bag[k] * 2) end
  local cb = function(v) return v * acc + 44.55 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 9.49, label = "f149_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + derive149(1, 2, 149)
end

do
local function spread150(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 36.72, bag[k] * 2) end
  while acc > 28.92 * 40 do acc = acc / 2 end
  while acc > 26.34 * 40 do acc = acc / 2 end
  while acc > 37.57 * 40 do acc = acc / 2 end
  while acc > 0.92 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 4.28, bag[k] * 2) end
  while acc > 49.16 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 40.42, label = "f150_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 39.83 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 2.17 / 17 + bag[1] else acc = acc - 2.17 end
  if acc % 7 > 3 then acc = acc * 20.00 / 17 + bag[1] else acc = acc - 20.00 end
  local cb = function(v) return v * acc + 15.82 end
  acc = cb(bag[4])
  while acc > 41.07 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 48.47 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + spread150(1, 2, 150)
end

do
local function project151(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 43.98 / 17 + bag[1] else acc = acc - 43.98 end
  while acc > 11.27 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 7.75 / 17 + bag[1] else acc = acc - 7.75 end
  if acc % 7 > 3 then acc = acc * 48.01 / 17 + bag[1] else acc = acc - 48.01 end
  while acc > 25.06 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 33.90, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 27.32, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 28.34, bag[k] * 2) end
  local tmp = { x = acc, y = 19.89, label = "f151_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 30.73 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 30.52 / 17 + bag[1] else acc = acc - 30.52 end
  for k = 1, 4 do acc = math.min(acc + k * 49.22, bag[k] * 2) end
  local cb = function(v) return v * acc + 40.05 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 24.01 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + project151(1, 2, 151)
end

do
local function derive152(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 47.07 end
  acc = cb(bag[1])
  while acc > 3.70 * 40 do acc = acc / 2 end
  while acc > 21.56 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 30.54, label = "f152_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 2.31 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 25.60 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 19.34 / 17 + bag[1] else acc = acc - 19.34 end
  local cb = function(v) return v * acc + 26.94 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 1.69, label = "f152_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 24.05, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 15.15, bag[k] * 2) end
  while acc > 29.90 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 6.32 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 17.44 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + derive152(1, 2, 152)
end

do
local function derive153(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 47.71 * 40 do acc = acc / 2 end
  while acc > 49.99 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 1.91 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 37.16, bag[k] * 2) end
  while acc > 34.69 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 41.98, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 48.48 / 17 + bag[1] else acc = acc - 48.48 end
  for k = 1, 4 do acc = math.min(acc + k * 1.26, bag[k] * 2) end
  while acc > 46.05 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 49.73 / 17 + bag[1] else acc = acc - 49.73 end
  local cb = function(v) return v * acc + 30.75 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 31.46, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 28.62 / 17 + bag[1] else acc = acc - 28.62 end
  while acc > 33.46 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive153(1, 2, 153)
end

do
local function spread154(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 42.40 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 14.72, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 12.02 / 17 + bag[1] else acc = acc - 12.02 end
  for k = 1, 4 do acc = math.min(acc + k * 19.46, bag[k] * 2) end
  local tmp = { x = acc, y = 48.57, label = "f154_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 31.17, label = "f154_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 25.25 / 17 + bag[1] else acc = acc - 25.25 end
  if acc % 7 > 3 then acc = acc * 14.96 / 17 + bag[1] else acc = acc - 14.96 end
  local tmp = { x = acc, y = 41.57, label = "f154_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 36.40, bag[k] * 2) end
  local tmp = { x = acc, y = 44.53, label = "f154_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 7.02, bag[k] * 2) end
  local tmp = { x = acc, y = 12.68, label = "f154_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 16.58, bag[k] * 2) end
  return acc % 9973
end
acc = acc + spread154(1, 2, 154)
end

do
local function fold155(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 41.25 / 17 + bag[1] else acc = acc - 41.25 end
  local cb = function(v) return v * acc + 32.24 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 5.29 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 34.08, label = "f155_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 20.68 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 13.61, label = "f155_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 43.38, bag[k] * 2) end
  local tmp = { x = acc, y = 28.78, label = "f155_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 9.95, label = "f155_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 1.28 end
  acc = cb(bag[2])
  while acc > 20.43 * 40 do acc = acc / 2 end
  while acc > 2.97 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 48.01, label = "f155_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 22.58, bag[k] * 2) end
  return acc % 9973
end
acc = acc + fold155(1, 2, 155)
end

do
local function fold156(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 34.11, label = "f156_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 37.74, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 14.31 / 17 + bag[1] else acc = acc - 14.31 end
  for k = 1, 4 do acc = math.min(acc + k * 46.67, bag[k] * 2) end
  while acc > 31.68 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 27.69 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 18.94, label = "f156_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 10.63, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 13.81 / 17 + bag[1] else acc = acc - 13.81 end
  if acc % 7 > 3 then acc = acc * 45.50 / 17 + bag[1] else acc = acc - 45.50 end
  local cb = function(v) return v * acc + 34.10 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 37.18, bag[k] * 2) end
  local tmp = { x = acc, y = 25.49, label = "f156_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 17.39 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold156(1, 2, 156)
end

do
local function compose157(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 6.82 end
  acc = cb(bag[1])
  while acc > 30.09 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 44.28 / 17 + bag[1] else acc = acc - 44.28 end
  for k = 1, 4 do acc = math.min(acc + k * 23.69, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 30.61 / 17 + bag[1] else acc = acc - 30.61 end
  if acc % 7 > 3 then acc = acc * 36.49 / 17 + bag[1] else acc = acc - 36.49 end
  local cb = function(v) return v * acc + 0.66 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 18.59 / 17 + bag[1] else acc = acc - 18.59 end
  for k = 1, 4 do acc = math.min(acc + k * 6.23, bag[k] * 2) end
  while acc > 4.29 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 32.47, label = "f157_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 38.46, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 36.73 / 17 + bag[1] else acc = acc - 36.73 end
  while acc > 7.01 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + compose157(1, 2, 157)
end

do
local function fold158(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 3.35 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 41.57 / 17 + bag[1] else acc = acc - 41.57 end
  if acc % 7 > 3 then acc = acc * 19.01 / 17 + bag[1] else acc = acc - 19.01 end
  if acc % 7 > 3 then acc = acc * 35.72 / 17 + bag[1] else acc = acc - 35.72 end
  local tmp = { x = acc, y = 34.43, label = "f158_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 13.39, label = "f158_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 45.37 / 17 + bag[1] else acc = acc - 45.37 end
  for k = 1, 4 do acc = math.min(acc + k * 31.46, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 32.75, bag[k] * 2) end
  local tmp = { x = acc, y = 23.68, label = "f158_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 1.25 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 42.17, bag[k] * 2) end
  local cb = function(v) return v * acc + 28.44 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 39.47 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + fold158(1, 2, 158)
end

do
local function project159(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 34.70 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 35.37 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 22.67, label = "f159_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 41.01, label = "f159_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 12.58 / 17 + bag[1] else acc = acc - 12.58 end
  for k = 1, 4 do acc = math.min(acc + k * 42.52, bag[k] * 2) end
  local tmp = { x = acc, y = 9.05, label = "f159_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 34.77 / 17 + bag[1] else acc = acc - 34.77 end
  local cb = function(v) return v * acc + 6.98 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 3.56, label = "f159_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 25.21 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 2.27, bag[k] * 2) end
  local tmp = { x = acc, y = 24.05, label = "f159_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 47.16 / 17 + bag[1] else acc = acc - 47.16 end
  return acc % 9973
end
acc = acc + project159(1, 2, 159)
end

do
local function clampv160(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 4.38, bag[k] * 2) end
  while acc > 14.77 * 40 do acc = acc / 2 end
  while acc > 17.92 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 18.46 / 17 + bag[1] else acc = acc - 18.46 end
  local tmp = { x = acc, y = 45.02, label = "f160_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 12.45 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 45.79 end
  acc = cb(bag[3])
  while acc > 8.02 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 10.47, bag[k] * 2) end
  local tmp = { x = acc, y = 48.31, label = "f160_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 22.71 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 37.82 / 17 + bag[1] else acc = acc - 37.82 end
  local tmp = { x = acc, y = 24.36, label = "f160_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 44.31, label = "f160_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv160(1, 2, 160)
end

do
local function clampv161(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 47.26 / 17 + bag[1] else acc = acc - 47.26 end
  local tmp = { x = acc, y = 30.63, label = "f161_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 37.68, label = "f161_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 17.50 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 40.70, label = "f161_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 30.02 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 20.89, label = "f161_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 10.54 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 34.46, label = "f161_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 33.51, label = "f161_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 9.41, label = "f161_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 0.59 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 11.38, bag[k] * 2) end
  while acc > 34.53 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + clampv161(1, 2, 161)
end

do
local function project162(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 40.92 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 16.48, bag[k] * 2) end
  local cb = function(v) return v * acc + 19.18 end
  acc = cb(bag[3])
  while acc > 34.69 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 35.63 end
  acc = cb(bag[1])
  while acc > 11.27 * 40 do acc = acc / 2 end
  while acc > 15.34 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 26.87 / 17 + bag[1] else acc = acc - 26.87 end
  if acc % 7 > 3 then acc = acc * 6.31 / 17 + bag[1] else acc = acc - 6.31 end
  for k = 1, 4 do acc = math.min(acc + k * 8.89, bag[k] * 2) end
  local tmp = { x = acc, y = 42.54, label = "f162_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 7.13 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 27.80 / 17 + bag[1] else acc = acc - 27.80 end
  local cb = function(v) return v * acc + 26.57 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + project162(1, 2, 162)
end

do
local function derive163(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 47.60, bag[k] * 2) end
  local tmp = { x = acc, y = 22.20, label = "f163_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 48.79, label = "f163_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 24.09 end
  acc = cb(bag[4])
  while acc > 0.45 * 40 do acc = acc / 2 end
  while acc > 35.08 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 2.32, bag[k] * 2) end
  local cb = function(v) return v * acc + 46.55 end
  acc = cb(bag[4])
  while acc > 7.83 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 3.35, label = "f163_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 37.51 end
  acc = cb(bag[3])
  while acc > 5.91 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 45.75 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 31.73, label = "f163_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + derive163(1, 2, 163)
end

do
local function compose164(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 40.12, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 14.19 / 17 + bag[1] else acc = acc - 14.19 end
  if acc % 7 > 3 then acc = acc * 7.82 / 17 + bag[1] else acc = acc - 7.82 end
  if acc % 7 > 3 then acc = acc * 28.50 / 17 + bag[1] else acc = acc - 28.50 end
  for k = 1, 4 do acc = math.min(acc + k * 9.26, bag[k] * 2) end
  local tmp = { x = acc, y = 9.71, label = "f164_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 5.26 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 17.36, label = "f164_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 28.79 / 17 + bag[1] else acc = acc - 28.79 end
  if acc % 7 > 3 then acc = acc * 1.46 / 17 + bag[1] else acc = acc - 1.46 end
  local tmp = { x = acc, y = 49.66, label = "f164_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 21.35 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 32.10, label = "f164_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 34.52 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + compose164(1, 2, 164)
end

do
local function spread165(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 2.77, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 9.06, bag[k] * 2) end
  local tmp = { x = acc, y = 41.69, label = "f165_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 30.18 / 17 + bag[1] else acc = acc - 30.18 end
  local cb = function(v) return v * acc + 16.68 end
  acc = cb(bag[1])
  while acc > 37.72 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 26.40, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 41.81, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 0.95 / 17 + bag[1] else acc = acc - 0.95 end
  for k = 1, 4 do acc = math.min(acc + k * 32.36, bag[k] * 2) end
  local tmp = { x = acc, y = 5.90, label = "f165_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 9.48, label = "f165_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 7.78 * 40 do acc = acc / 2 end
  while acc > 19.50 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + spread165(1, 2, 165)
end

do
local function clampv166(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 19.23 / 17 + bag[1] else acc = acc - 19.23 end
  if acc % 7 > 3 then acc = acc * 27.85 / 17 + bag[1] else acc = acc - 27.85 end
  while acc > 5.31 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 29.88 / 17 + bag[1] else acc = acc - 29.88 end
  local tmp = { x = acc, y = 3.88, label = "f166_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 33.32 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 45.41, label = "f166_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 0.66, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 28.93 / 17 + bag[1] else acc = acc - 28.93 end
  local cb = function(v) return v * acc + 39.56 end
  acc = cb(bag[2])
  while acc > 29.15 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 13.50 / 17 + bag[1] else acc = acc - 13.50 end
  while acc > 46.79 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 38.45, label = "f166_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv166(1, 2, 166)
end

do
local function weight167(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 25.10 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 30.96 end
  acc = cb(bag[2])
  while acc > 6.18 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 46.82, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 18.67, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 28.36, bag[k] * 2) end
  while acc > 23.88 * 40 do acc = acc / 2 end
  while acc > 48.06 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 17.22, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 24.36 / 17 + bag[1] else acc = acc - 24.36 end
  for k = 1, 4 do acc = math.min(acc + k * 12.79, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 46.86 / 17 + bag[1] else acc = acc - 46.86 end
  local cb = function(v) return v * acc + 13.35 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 34.22, bag[k] * 2) end
  return acc % 9973
end
acc = acc + weight167(1, 2, 167)
end

do
local function fold168(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 32.90 / 17 + bag[1] else acc = acc - 32.90 end
  for k = 1, 4 do acc = math.min(acc + k * 19.93, bag[k] * 2) end
  while acc > 7.14 * 40 do acc = acc / 2 end
  while acc > 43.24 * 40 do acc = acc / 2 end
  while acc > 12.58 * 40 do acc = acc / 2 end
  while acc > 12.10 * 40 do acc = acc / 2 end
  while acc > 39.80 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 16.59 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 20.56 / 17 + bag[1] else acc = acc - 20.56 end
  if acc % 7 > 3 then acc = acc * 9.26 / 17 + bag[1] else acc = acc - 9.26 end
  for k = 1, 4 do acc = math.min(acc + k * 19.65, bag[k] * 2) end
  local cb = function(v) return v * acc + 20.25 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 23.60 / 17 + bag[1] else acc = acc - 23.60 end
  while acc > 2.68 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold168(1, 2, 168)
end

do
local function weight169(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 26.63, label = "f169_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 44.37 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 11.98 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 37.64, bag[k] * 2) end
  while acc > 17.89 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 48.50 end
  acc = cb(bag[2])
  while acc > 17.49 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 12.03 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 39.76, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 41.45 / 17 + bag[1] else acc = acc - 41.45 end
  local cb = function(v) return v * acc + 20.10 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 22.67 / 17 + bag[1] else acc = acc - 22.67 end
  local cb = function(v) return v * acc + 0.64 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 33.94 / 17 + bag[1] else acc = acc - 33.94 end
  return acc % 9973
end
acc = acc + weight169(1, 2, 169)
end

do
local function clampv170(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 32.95 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 7.03, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 8.85 / 17 + bag[1] else acc = acc - 8.85 end
  for k = 1, 4 do acc = math.min(acc + k * 19.98, bag[k] * 2) end
  local tmp = { x = acc, y = 10.73, label = "f170_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 18.64, bag[k] * 2) end
  while acc > 33.23 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 25.98 end
  acc = cb(bag[4])
  while acc > 34.48 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 34.76, label = "f170_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 19.16 / 17 + bag[1] else acc = acc - 19.16 end
  for k = 1, 4 do acc = math.min(acc + k * 16.46, bag[k] * 2) end
  while acc > 0.18 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 7.67, label = "f170_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv170(1, 2, 170)
end

do
local function project171(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 7.15 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 8.21 / 17 + bag[1] else acc = acc - 8.21 end
  local cb = function(v) return v * acc + 37.28 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 5.40, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 43.00 / 17 + bag[1] else acc = acc - 43.00 end
  local tmp = { x = acc, y = 47.54, label = "f171_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 43.70 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 14.29, label = "f171_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 16.17 / 17 + bag[1] else acc = acc - 16.17 end
  if acc % 7 > 3 then acc = acc * 10.71 / 17 + bag[1] else acc = acc - 10.71 end
  while acc > 4.21 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 25.73, label = "f171_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 8.14 / 17 + bag[1] else acc = acc - 8.14 end
  for k = 1, 4 do acc = math.min(acc + k * 49.05, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project171(1, 2, 171)
end

do
local function project172(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 18.94 / 17 + bag[1] else acc = acc - 18.94 end
  if acc % 7 > 3 then acc = acc * 48.60 / 17 + bag[1] else acc = acc - 48.60 end
  local cb = function(v) return v * acc + 22.68 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 48.37 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 29.95, bag[k] * 2) end
  local tmp = { x = acc, y = 2.86, label = "f172_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 15.38, label = "f172_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 31.29 end
  acc = cb(bag[4])
  while acc > 13.48 * 40 do acc = acc / 2 end
  while acc > 29.49 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 25.92 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 1.01, label = "f172_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 42.16, label = "f172_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 40.94 / 17 + bag[1] else acc = acc - 40.94 end
  return acc % 9973
end
acc = acc + project172(1, 2, 172)
end

do
local function weight173(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 35.94 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 35.77, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 20.76, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 10.06 / 17 + bag[1] else acc = acc - 10.06 end
  while acc > 21.26 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 30.77, label = "f173_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 2.77 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 44.43, label = "f173_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 27.60 end
  acc = cb(bag[1])
  while acc > 38.07 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 37.18 / 17 + bag[1] else acc = acc - 37.18 end
  while acc > 39.13 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 34.94, bag[k] * 2) end
  local tmp = { x = acc, y = 22.80, label = "f173_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + weight173(1, 2, 173)
end

do
local function compose174(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 36.47 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 45.88, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 16.03, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 3.09, bag[k] * 2) end
  local cb = function(v) return v * acc + 10.85 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 14.26, label = "f174_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 46.95, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 19.05 / 17 + bag[1] else acc = acc - 19.05 end
  local tmp = { x = acc, y = 11.92, label = "f174_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 37.87 / 17 + bag[1] else acc = acc - 37.87 end
  local tmp = { x = acc, y = 8.59, label = "f174_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 4.98 / 17 + bag[1] else acc = acc - 4.98 end
  local cb = function(v) return v * acc + 34.79 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 32.75 / 17 + bag[1] else acc = acc - 32.75 end
  return acc % 9973
end
acc = acc + compose174(1, 2, 174)
end

do
local function weight175(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 17.72 / 17 + bag[1] else acc = acc - 17.72 end
  if acc % 7 > 3 then acc = acc * 28.47 / 17 + bag[1] else acc = acc - 28.47 end
  local tmp = { x = acc, y = 35.85, label = "f175_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 36.22 end
  acc = cb(bag[4])
  while acc > 33.66 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 2.15, bag[k] * 2) end
  while acc > 37.97 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 9.54, label = "f175_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 37.38 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 2.98, label = "f175_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 33.30 / 17 + bag[1] else acc = acc - 33.30 end
  while acc > 2.79 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 28.01, bag[k] * 2) end
  local tmp = { x = acc, y = 16.11, label = "f175_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + weight175(1, 2, 175)
end

do
local function project176(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 43.84 / 17 + bag[1] else acc = acc - 43.84 end
  for k = 1, 4 do acc = math.min(acc + k * 37.02, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 23.51 / 17 + bag[1] else acc = acc - 23.51 end
  local cb = function(v) return v * acc + 38.19 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 7.00 / 17 + bag[1] else acc = acc - 7.00 end
  if acc % 7 > 3 then acc = acc * 44.72 / 17 + bag[1] else acc = acc - 44.72 end
  local cb = function(v) return v * acc + 14.60 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 33.07, bag[k] * 2) end
  while acc > 29.96 * 40 do acc = acc / 2 end
  while acc > 4.12 * 40 do acc = acc / 2 end
  while acc > 23.60 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 12.25 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 18.77, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 32.14, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project176(1, 2, 176)
end

do
local function derive177(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 33.38, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 13.69, bag[k] * 2) end
  local tmp = { x = acc, y = 14.98, label = "f177_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 20.64 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 41.64, bag[k] * 2) end
  local tmp = { x = acc, y = 45.91, label = "f177_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 20.00, bag[k] * 2) end
  local tmp = { x = acc, y = 23.59, label = "f177_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 5.88 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 30.19, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 15.87, bag[k] * 2) end
  local cb = function(v) return v * acc + 44.08 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 2.75, label = "f177_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 6.71 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + derive177(1, 2, 177)
end

do
local function spread178(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 43.43 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 5.26 end
  acc = cb(bag[2])
  while acc > 23.90 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 32.79 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 27.20, label = "f178_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 7.63, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 24.66, bag[k] * 2) end
  while acc > 39.44 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 16.88, label = "f178_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 38.68 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 14.90, label = "f178_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 12.01, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 7.84 / 17 + bag[1] else acc = acc - 7.84 end
  if acc % 7 > 3 then acc = acc * 5.37 / 17 + bag[1] else acc = acc - 5.37 end
  return acc % 9973
end
acc = acc + spread178(1, 2, 178)
end

do
local function clampv179(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 33.50 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 6.19 / 17 + bag[1] else acc = acc - 6.19 end
  if acc % 7 > 3 then acc = acc * 49.35 / 17 + bag[1] else acc = acc - 49.35 end
  while acc > 13.48 * 40 do acc = acc / 2 end
  while acc > 3.47 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 32.59, bag[k] * 2) end
  local cb = function(v) return v * acc + 17.38 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 8.57 / 17 + bag[1] else acc = acc - 8.57 end
  if acc % 7 > 3 then acc = acc * 10.63 / 17 + bag[1] else acc = acc - 10.63 end
  for k = 1, 4 do acc = math.min(acc + k * 22.28, bag[k] * 2) end
  local tmp = { x = acc, y = 22.22, label = "f179_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 45.86, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 38.51 / 17 + bag[1] else acc = acc - 38.51 end
  local tmp = { x = acc, y = 45.58, label = "f179_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv179(1, 2, 179)
end

do
local function clampv180(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 16.69 / 17 + bag[1] else acc = acc - 16.69 end
  for k = 1, 4 do acc = math.min(acc + k * 5.59, bag[k] * 2) end
  local tmp = { x = acc, y = 34.78, label = "f180_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 37.77 / 17 + bag[1] else acc = acc - 37.77 end
  local cb = function(v) return v * acc + 47.51 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 21.03, label = "f180_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 34.22, label = "f180_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 22.45, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 23.50 / 17 + bag[1] else acc = acc - 23.50 end
  while acc > 18.99 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 46.39 / 17 + bag[1] else acc = acc - 46.39 end
  while acc > 18.86 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 23.03 / 17 + bag[1] else acc = acc - 23.03 end
  local tmp = { x = acc, y = 18.52, label = "f180_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv180(1, 2, 180)
end

do
local function weight181(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 26.24, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 13.95 / 17 + bag[1] else acc = acc - 13.95 end
  local cb = function(v) return v * acc + 44.33 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 46.31 / 17 + bag[1] else acc = acc - 46.31 end
  while acc > 7.16 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 45.21 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 9.41 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 20.47, label = "f181_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 15.45 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 4.29, bag[k] * 2) end
  local cb = function(v) return v * acc + 10.34 end
  acc = cb(bag[3])
  while acc > 36.51 * 40 do acc = acc / 2 end
  while acc > 40.60 * 40 do acc = acc / 2 end
  while acc > 8.70 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight181(1, 2, 181)
end

do
local function project182(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 38.05 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 12.34, label = "f182_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 40.20, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 26.21, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 24.58 / 17 + bag[1] else acc = acc - 24.58 end
  if acc % 7 > 3 then acc = acc * 36.89 / 17 + bag[1] else acc = acc - 36.89 end
  while acc > 19.30 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 6.52 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 47.88, label = "f182_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 12.24, bag[k] * 2) end
  while acc > 7.59 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 4.77 / 17 + bag[1] else acc = acc - 4.77 end
  while acc > 6.78 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 43.52 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + project182(1, 2, 182)
end

do
local function derive183(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 7.96 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 7.26, label = "f183_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 40.53, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 6.46 / 17 + bag[1] else acc = acc - 6.46 end
  for k = 1, 4 do acc = math.min(acc + k * 11.49, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 38.85, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 9.90, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 35.16, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 4.31, bag[k] * 2) end
  while acc > 19.46 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 25.89 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 3.27, label = "f183_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 40.19, label = "f183_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 40.58 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive183(1, 2, 183)
end

do
local function derive184(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 26.40 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 26.39 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 32.06, label = "f184_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 49.48 / 17 + bag[1] else acc = acc - 49.48 end
  local tmp = { x = acc, y = 2.09, label = "f184_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 33.28 / 17 + bag[1] else acc = acc - 33.28 end
  local tmp = { x = acc, y = 47.05, label = "f184_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 12.55 / 17 + bag[1] else acc = acc - 12.55 end
  while acc > 5.39 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 21.27 / 17 + bag[1] else acc = acc - 21.27 end
  local tmp = { x = acc, y = 1.86, label = "f184_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 13.49, label = "f184_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 43.47 / 17 + bag[1] else acc = acc - 43.47 end
  while acc > 29.93 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive184(1, 2, 184)
end

do
local function measure185(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 24.43 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 28.29 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 3.49 / 17 + bag[1] else acc = acc - 3.49 end
  local tmp = { x = acc, y = 36.34, label = "f185_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 46.41, label = "f185_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 38.45 / 17 + bag[1] else acc = acc - 38.45 end
  for k = 1, 4 do acc = math.min(acc + k * 9.30, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 5.66, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 26.17, bag[k] * 2) end
  local cb = function(v) return v * acc + 40.15 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 22.10 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 15.16, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 4.42 / 17 + bag[1] else acc = acc - 4.42 end
  for k = 1, 4 do acc = math.min(acc + k * 43.41, bag[k] * 2) end
  return acc % 9973
end
acc = acc + measure185(1, 2, 185)
end

do
local function project186(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 2.16 * 40 do acc = acc / 2 end
  while acc > 6.76 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 43.40 / 17 + bag[1] else acc = acc - 43.40 end
  for k = 1, 4 do acc = math.min(acc + k * 46.77, bag[k] * 2) end
  local tmp = { x = acc, y = 0.39, label = "f186_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 14.09, label = "f186_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 30.26 / 17 + bag[1] else acc = acc - 30.26 end
  for k = 1, 4 do acc = math.min(acc + k * 29.46, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 41.61, bag[k] * 2) end
  while acc > 34.74 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 1.97, bag[k] * 2) end
  local tmp = { x = acc, y = 27.83, label = "f186_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 39.45 / 17 + bag[1] else acc = acc - 39.45 end
  while acc > 15.62 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + project186(1, 2, 186)
end

do
local function project187(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 39.00 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 29.47, bag[k] * 2) end
  while acc > 9.07 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 28.32, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 45.60, bag[k] * 2) end
  local cb = function(v) return v * acc + 36.12 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 23.88 / 17 + bag[1] else acc = acc - 23.88 end
  if acc % 7 > 3 then acc = acc * 25.20 / 17 + bag[1] else acc = acc - 25.20 end
  for k = 1, 4 do acc = math.min(acc + k * 0.98, bag[k] * 2) end
  local cb = function(v) return v * acc + 8.83 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 14.70 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 34.27, label = "f187_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 15.42, label = "f187_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 12.35, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project187(1, 2, 187)
end

do
local function measure188(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 5.34, label = "f188_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 37.45 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 33.75 / 17 + bag[1] else acc = acc - 33.75 end
  for k = 1, 4 do acc = math.min(acc + k * 27.60, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 18.22 / 17 + bag[1] else acc = acc - 18.22 end
  local tmp = { x = acc, y = 21.32, label = "f188_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 1.19 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 42.47 / 17 + bag[1] else acc = acc - 42.47 end
  local tmp = { x = acc, y = 46.79, label = "f188_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 36.32 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 42.94 / 17 + bag[1] else acc = acc - 42.94 end
  while acc > 9.41 * 40 do acc = acc / 2 end
  while acc > 7.30 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 37.81 / 17 + bag[1] else acc = acc - 37.81 end
  return acc % 9973
end
acc = acc + measure188(1, 2, 188)
end

do
local function fold189(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 12.34 / 17 + bag[1] else acc = acc - 12.34 end
  local cb = function(v) return v * acc + 7.17 end
  acc = cb(bag[2])
  while acc > 45.82 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 37.03 / 17 + bag[1] else acc = acc - 37.03 end
  local cb = function(v) return v * acc + 7.25 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 35.05 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 5.62, bag[k] * 2) end
  local tmp = { x = acc, y = 13.98, label = "f189_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 5.74, label = "f189_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 18.48 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 38.80 / 17 + bag[1] else acc = acc - 38.80 end
  local cb = function(v) return v * acc + 11.20 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 41.62, label = "f189_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 19.30, label = "f189_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold189(1, 2, 189)
end

do
local function project190(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 24.20 / 17 + bag[1] else acc = acc - 24.20 end
  local tmp = { x = acc, y = 8.32, label = "f190_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 31.83, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 46.72 / 17 + bag[1] else acc = acc - 46.72 end
  if acc % 7 > 3 then acc = acc * 34.01 / 17 + bag[1] else acc = acc - 34.01 end
  local tmp = { x = acc, y = 8.95, label = "f190_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 25.72 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 11.27, bag[k] * 2) end
  local tmp = { x = acc, y = 3.23, label = "f190_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 49.08, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 45.27, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 34.12 / 17 + bag[1] else acc = acc - 34.12 end
  if acc % 7 > 3 then acc = acc * 36.05 / 17 + bag[1] else acc = acc - 36.05 end
  for k = 1, 4 do acc = math.min(acc + k * 3.51, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project190(1, 2, 190)
end

do
local function measure191(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 31.01 * 40 do acc = acc / 2 end
  while acc > 33.99 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 36.42, label = "f191_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 21.25 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 19.85, bag[k] * 2) end
  local cb = function(v) return v * acc + 44.17 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 29.79, label = "f191_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 33.63, bag[k] * 2) end
  local cb = function(v) return v * acc + 8.14 end
  acc = cb(bag[1])
  while acc > 18.68 * 40 do acc = acc / 2 end
  while acc > 21.17 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 41.11 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 30.55, label = "f191_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 44.87 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + measure191(1, 2, 191)
end

do
local function fold192(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 37.99, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 7.02 / 17 + bag[1] else acc = acc - 7.02 end
  for k = 1, 4 do acc = math.min(acc + k * 1.67, bag[k] * 2) end
  local cb = function(v) return v * acc + 47.58 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 24.39, label = "f192_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 35.20 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 41.39 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 11.89 / 17 + bag[1] else acc = acc - 11.89 end
  while acc > 3.94 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 37.58 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 25.79 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 30.95 / 17 + bag[1] else acc = acc - 30.95 end
  local cb = function(v) return v * acc + 12.37 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 13.92, bag[k] * 2) end
  return acc % 9973
end
acc = acc + fold192(1, 2, 192)
end

do
local function measure193(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 1.29 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 2.98, bag[k] * 2) end
  local cb = function(v) return v * acc + 37.78 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 23.27, bag[k] * 2) end
  while acc > 39.09 * 40 do acc = acc / 2 end
  while acc > 21.02 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 30.38 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 36.32 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 34.89, label = "f193_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 9.73 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 29.53, label = "f193_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 18.45, label = "f193_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 42.34 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 17.41, bag[k] * 2) end
  return acc % 9973
end
acc = acc + measure193(1, 2, 193)
end

do
local function measure194(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 19.36 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 41.35 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 3.42, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 17.14, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 48.81, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 3.49 / 17 + bag[1] else acc = acc - 3.49 end
  if acc % 7 > 3 then acc = acc * 22.00 / 17 + bag[1] else acc = acc - 22.00 end
  local cb = function(v) return v * acc + 49.26 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 0.83 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 15.20 end
  acc = cb(bag[2])
  while acc > 36.36 * 40 do acc = acc / 2 end
  while acc > 1.58 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 16.73 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 14.17 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + measure194(1, 2, 194)
end

do
local function spread195(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 14.53 / 17 + bag[1] else acc = acc - 14.53 end
  local tmp = { x = acc, y = 22.45, label = "f195_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 29.83, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 34.68 / 17 + bag[1] else acc = acc - 34.68 end
  if acc % 7 > 3 then acc = acc * 24.67 / 17 + bag[1] else acc = acc - 24.67 end
  local tmp = { x = acc, y = 30.14, label = "f195_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 11.33 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 17.59 / 17 + bag[1] else acc = acc - 17.59 end
  for k = 1, 4 do acc = math.min(acc + k * 17.12, bag[k] * 2) end
  while acc > 2.85 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 26.33, label = "f195_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 10.02 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 31.76, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 28.41 / 17 + bag[1] else acc = acc - 28.41 end
  return acc % 9973
end
acc = acc + spread195(1, 2, 195)
end

do
local function clampv196(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 24.60, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 14.35, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 22.70, bag[k] * 2) end
  local cb = function(v) return v * acc + 48.73 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 33.35, bag[k] * 2) end
  local tmp = { x = acc, y = 49.92, label = "f196_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 0.53 * 40 do acc = acc / 2 end
  while acc > 40.65 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 26.67 / 17 + bag[1] else acc = acc - 26.67 end
  local tmp = { x = acc, y = 36.16, label = "f196_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 26.72, label = "f196_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 30.73 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 6.46, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 35.21 / 17 + bag[1] else acc = acc - 35.21 end
  return acc % 9973
end
acc = acc + clampv196(1, 2, 196)
end

do
local function fold197(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 32.29 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 4.55, label = "f197_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 36.38 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 0.28 / 17 + bag[1] else acc = acc - 0.28 end
  local cb = function(v) return v * acc + 33.18 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 47.46 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 15.64 / 17 + bag[1] else acc = acc - 15.64 end
  for k = 1, 4 do acc = math.min(acc + k * 26.92, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 38.46, bag[k] * 2) end
  while acc > 44.63 * 40 do acc = acc / 2 end
  while acc > 28.71 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 34.35 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 14.30 end
  acc = cb(bag[1])
  while acc > 22.18 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold197(1, 2, 197)
end

do
local function project198(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 1.97, label = "f198_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 13.46, bag[k] * 2) end
  local tmp = { x = acc, y = 19.75, label = "f198_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 5.61, label = "f198_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 2.98 end
  acc = cb(bag[1])
  while acc > 39.93 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 48.94, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 17.26, bag[k] * 2) end
  local cb = function(v) return v * acc + 15.46 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 42.91, label = "f198_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 32.98, label = "f198_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 45.49 * 40 do acc = acc / 2 end
  while acc > 41.31 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 13.07 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + project198(1, 2, 198)
end

do
local function derive199(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 49.47 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 16.59, bag[k] * 2) end
  while acc > 3.66 * 40 do acc = acc / 2 end
  while acc > 16.32 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 30.91 / 17 + bag[1] else acc = acc - 30.91 end
  local cb = function(v) return v * acc + 36.71 end
  acc = cb(bag[2])
  while acc > 49.13 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 40.02 / 17 + bag[1] else acc = acc - 40.02 end
  if acc % 7 > 3 then acc = acc * 36.15 / 17 + bag[1] else acc = acc - 36.15 end
  while acc > 20.60 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 24.05, label = "f199_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 30.55, label = "f199_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 35.15, bag[k] * 2) end
  while acc > 44.18 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive199(1, 2, 199)
end

do
local function project200(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 22.77 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 25.75 / 17 + bag[1] else acc = acc - 25.75 end
  for k = 1, 4 do acc = math.min(acc + k * 6.34, bag[k] * 2) end
  while acc > 41.83 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 12.26 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 27.29 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 21.15, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 19.83, bag[k] * 2) end
  local cb = function(v) return v * acc + 22.72 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 18.89 / 17 + bag[1] else acc = acc - 18.89 end
  while acc > 3.30 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 24.71 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 11.50 / 17 + bag[1] else acc = acc - 11.50 end
  if acc % 7 > 3 then acc = acc * 38.74 / 17 + bag[1] else acc = acc - 38.74 end
  return acc % 9973
end
acc = acc + project200(1, 2, 200)
end

do
local function clampv201(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 11.02 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 26.20 / 17 + bag[1] else acc = acc - 26.20 end
  local tmp = { x = acc, y = 34.00, label = "f201_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 1.29 / 17 + bag[1] else acc = acc - 1.29 end
  if acc % 7 > 3 then acc = acc * 28.07 / 17 + bag[1] else acc = acc - 28.07 end
  local tmp = { x = acc, y = 18.92, label = "f201_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 22.89, label = "f201_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 26.98 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 14.33 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 30.08, label = "f201_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 2.96 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 37.54 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 38.47 / 17 + bag[1] else acc = acc - 38.47 end
  if acc % 7 > 3 then acc = acc * 42.85 / 17 + bag[1] else acc = acc - 42.85 end
  return acc % 9973
end
acc = acc + clampv201(1, 2, 201)
end

do
local function spread202(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 14.27 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 8.74 / 17 + bag[1] else acc = acc - 8.74 end
  while acc > 21.00 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 24.11 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 38.37, bag[k] * 2) end
  local tmp = { x = acc, y = 47.24, label = "f202_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 42.71 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 27.16 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 0.55, bag[k] * 2) end
  local tmp = { x = acc, y = 17.37, label = "f202_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 12.99, label = "f202_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 49.26 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 19.93 end
  acc = cb(bag[1])
  while acc > 14.15 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + spread202(1, 2, 202)
end

do
local function measure203(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 49.91, bag[k] * 2) end
  local cb = function(v) return v * acc + 9.98 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 48.82 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 47.97, label = "f203_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 10.97 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 40.22 / 17 + bag[1] else acc = acc - 40.22 end
  for k = 1, 4 do acc = math.min(acc + k * 13.07, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 0.79 / 17 + bag[1] else acc = acc - 0.79 end
  while acc > 35.74 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 4.18, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 14.09, bag[k] * 2) end
  while acc > 25.34 * 40 do acc = acc / 2 end
  while acc > 24.62 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 44.52 / 17 + bag[1] else acc = acc - 44.52 end
  return acc % 9973
end
acc = acc + measure203(1, 2, 203)
end

do
local function compose204(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 1.90 / 17 + bag[1] else acc = acc - 1.90 end
  local tmp = { x = acc, y = 7.57, label = "f204_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 46.08, label = "f204_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 35.54 * 40 do acc = acc / 2 end
  while acc > 29.83 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 40.95 end
  acc = cb(bag[2])
  while acc > 29.77 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 33.19, label = "f204_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 13.20 / 17 + bag[1] else acc = acc - 13.20 end
  local cb = function(v) return v * acc + 26.14 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 47.27, label = "f204_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 49.33, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 2.91 / 17 + bag[1] else acc = acc - 2.91 end
  for k = 1, 4 do acc = math.min(acc + k * 43.07, bag[k] * 2) end
  return acc % 9973
end
acc = acc + compose204(1, 2, 204)
end

do
local function fold205(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 19.68, bag[k] * 2) end
  local tmp = { x = acc, y = 27.10, label = "f205_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 27.64 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 19.46 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 49.64 / 17 + bag[1] else acc = acc - 49.64 end
  if acc % 7 > 3 then acc = acc * 32.37 / 17 + bag[1] else acc = acc - 32.37 end
  if acc % 7 > 3 then acc = acc * 29.53 / 17 + bag[1] else acc = acc - 29.53 end
  if acc % 7 > 3 then acc = acc * 17.72 / 17 + bag[1] else acc = acc - 17.72 end
  local tmp = { x = acc, y = 4.83, label = "f205_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 35.84, label = "f205_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 8.05 / 17 + bag[1] else acc = acc - 8.05 end
  for k = 1, 4 do acc = math.min(acc + k * 40.28, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 24.90 / 17 + bag[1] else acc = acc - 24.90 end
  local tmp = { x = acc, y = 26.08, label = "f205_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold205(1, 2, 205)
end

do
local function spread206(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 27.89 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 18.16, label = "f206_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 20.62 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 32.41 / 17 + bag[1] else acc = acc - 32.41 end
  local cb = function(v) return v * acc + 28.48 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 43.91, bag[k] * 2) end
  local tmp = { x = acc, y = 19.78, label = "f206_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 29.92 / 17 + bag[1] else acc = acc - 29.92 end
  local tmp = { x = acc, y = 17.09, label = "f206_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 39.81 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 18.48 / 17 + bag[1] else acc = acc - 18.48 end
  while acc > 48.79 * 40 do acc = acc / 2 end
  while acc > 26.17 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 24.20, label = "f206_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + spread206(1, 2, 206)
end

do
local function fold207(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 31.57 * 40 do acc = acc / 2 end
  while acc > 23.95 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 3.67 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 20.60 / 17 + bag[1] else acc = acc - 20.60 end
  local tmp = { x = acc, y = 2.62, label = "f207_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 45.71, bag[k] * 2) end
  while acc > 1.14 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 15.14 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 7.20 / 17 + bag[1] else acc = acc - 7.20 end
  while acc > 46.17 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 37.31 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 34.72, bag[k] * 2) end
  local cb = function(v) return v * acc + 33.11 end
  acc = cb(bag[1])
  while acc > 35.62 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold207(1, 2, 207)
end

do
local function derive208(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 8.21 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 29.64, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 4.61 / 17 + bag[1] else acc = acc - 4.61 end
  local cb = function(v) return v * acc + 48.04 end
  acc = cb(bag[4])
  while acc > 3.09 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 43.72, bag[k] * 2) end
  local tmp = { x = acc, y = 49.18, label = "f208_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 40.34 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 26.72 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 23.67 end
  acc = cb(bag[2])
  while acc > 39.54 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 6.88, label = "f208_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 16.77 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 14.99, label = "f208_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + derive208(1, 2, 208)
end

do
local function derive209(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 12.64, bag[k] * 2) end
  local cb = function(v) return v * acc + 49.23 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 32.28, label = "f209_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 24.79, label = "f209_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 17.79 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 30.13, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 17.34, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 28.81, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 17.58, bag[k] * 2) end
  local tmp = { x = acc, y = 40.51, label = "f209_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 5.56 end
  acc = cb(bag[3])
  while acc > 16.51 * 40 do acc = acc / 2 end
  while acc > 0.88 * 40 do acc = acc / 2 end
  while acc > 23.12 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive209(1, 2, 209)
end

do
local function spread210(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 46.88 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 42.27 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 21.13 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 44.49, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 29.72 / 17 + bag[1] else acc = acc - 29.72 end
  local tmp = { x = acc, y = 27.04, label = "f210_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 48.78 / 17 + bag[1] else acc = acc - 48.78 end
  while acc > 28.80 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 23.68 / 17 + bag[1] else acc = acc - 23.68 end
  while acc > 18.69 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 24.05 / 17 + bag[1] else acc = acc - 24.05 end
  if acc % 7 > 3 then acc = acc * 18.00 / 17 + bag[1] else acc = acc - 18.00 end
  while acc > 16.41 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 1.69 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + spread210(1, 2, 210)
end

do
local function spread211(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 44.93 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 12.22 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 2.11 / 17 + bag[1] else acc = acc - 2.11 end
  for k = 1, 4 do acc = math.min(acc + k * 5.78, bag[k] * 2) end
  local tmp = { x = acc, y = 4.05, label = "f211_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 38.53, bag[k] * 2) end
  local cb = function(v) return v * acc + 25.93 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 21.07, bag[k] * 2) end
  while acc > 47.60 * 40 do acc = acc / 2 end
  while acc > 1.17 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 28.56 / 17 + bag[1] else acc = acc - 28.56 end
  while acc > 13.41 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 45.63, bag[k] * 2) end
  local tmp = { x = acc, y = 32.47, label = "f211_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + spread211(1, 2, 211)
end

do
local function compose212(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 0.58, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 25.99 / 17 + bag[1] else acc = acc - 25.99 end
  while acc > 45.58 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 32.71 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 29.01, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 18.56 / 17 + bag[1] else acc = acc - 18.56 end
  local tmp = { x = acc, y = 46.30, label = "f212_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 25.61 / 17 + bag[1] else acc = acc - 25.61 end
  local cb = function(v) return v * acc + 42.47 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 34.45, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 29.73 / 17 + bag[1] else acc = acc - 29.73 end
  local cb = function(v) return v * acc + 28.13 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 19.88, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 23.02 / 17 + bag[1] else acc = acc - 23.02 end
  return acc % 9973
end
acc = acc + compose212(1, 2, 212)
end

do
local function clampv213(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 15.47 / 17 + bag[1] else acc = acc - 15.47 end
  local tmp = { x = acc, y = 24.73, label = "f213_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 20.10, label = "f213_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 29.35, bag[k] * 2) end
  local cb = function(v) return v * acc + 14.32 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 42.47 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 19.64 / 17 + bag[1] else acc = acc - 19.64 end
  if acc % 7 > 3 then acc = acc * 40.42 / 17 + bag[1] else acc = acc - 40.42 end
  while acc > 15.34 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 37.56 end
  acc = cb(bag[2])
  while acc > 21.13 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 18.81 end
  acc = cb(bag[4])
  while acc > 25.86 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 37.28, bag[k] * 2) end
  return acc % 9973
end
acc = acc + clampv213(1, 2, 213)
end

do
local function project214(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 3.80, bag[k] * 2) end
  local cb = function(v) return v * acc + 13.87 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 5.01, bag[k] * 2) end
  local tmp = { x = acc, y = 18.30, label = "f214_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 2.14 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 20.53, bag[k] * 2) end
  local tmp = { x = acc, y = 48.83, label = "f214_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 39.86 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 12.64 end
  acc = cb(bag[1])
  while acc > 9.01 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 33.39 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 15.81 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 21.89, bag[k] * 2) end
  local tmp = { x = acc, y = 2.79, label = "f214_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + project214(1, 2, 214)
end

do
local function compose215(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 20.03 end
  acc = cb(bag[1])
  while acc > 21.13 * 40 do acc = acc / 2 end
  while acc > 37.07 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 43.02 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 0.72 / 17 + bag[1] else acc = acc - 0.72 end
  for k = 1, 4 do acc = math.min(acc + k * 24.40, bag[k] * 2) end
  local tmp = { x = acc, y = 11.84, label = "f215_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 10.49, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 46.87, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 8.79, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 25.78, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 8.64, bag[k] * 2) end
  while acc > 24.78 * 40 do acc = acc / 2 end
  while acc > 17.85 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + compose215(1, 2, 215)
end

do
local function derive216(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 29.77, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 41.44 / 17 + bag[1] else acc = acc - 41.44 end
  if acc % 7 > 3 then acc = acc * 47.05 / 17 + bag[1] else acc = acc - 47.05 end
  local tmp = { x = acc, y = 9.40, label = "f216_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 48.61 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 17.10 / 17 + bag[1] else acc = acc - 17.10 end
  local tmp = { x = acc, y = 25.79, label = "f216_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 29.02, bag[k] * 2) end
  while acc > 4.76 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 9.74 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 25.65, label = "f216_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 43.65 / 17 + bag[1] else acc = acc - 43.65 end
  local tmp = { x = acc, y = 40.48, label = "f216_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 31.61 / 17 + bag[1] else acc = acc - 31.61 end
  return acc % 9973
end
acc = acc + derive216(1, 2, 216)
end

do
local function fold217(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 9.33, label = "f217_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 15.92 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 30.41 / 17 + bag[1] else acc = acc - 30.41 end
  while acc > 26.55 * 40 do acc = acc / 2 end
  while acc > 19.03 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 8.20 / 17 + bag[1] else acc = acc - 8.20 end
  for k = 1, 4 do acc = math.min(acc + k * 46.92, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 29.63 / 17 + bag[1] else acc = acc - 29.63 end
  local tmp = { x = acc, y = 6.88, label = "f217_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 14.53 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 38.86 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 10.08, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 40.25 / 17 + bag[1] else acc = acc - 40.25 end
  while acc > 40.05 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold217(1, 2, 217)
end

do
local function derive218(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 2.77, label = "f218_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 0.83, bag[k] * 2) end
  local tmp = { x = acc, y = 16.98, label = "f218_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 1.36 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 28.76, label = "f218_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 17.41 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 5.96 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 47.46, label = "f218_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 46.15 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 9.81 / 17 + bag[1] else acc = acc - 9.81 end
  while acc > 20.37 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 17.06 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 42.43, label = "f218_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 49.69 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + derive218(1, 2, 218)
end

do
local function measure219(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 11.04, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 25.28, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 40.39, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 24.96 / 17 + bag[1] else acc = acc - 24.96 end
  local cb = function(v) return v * acc + 37.43 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 2.58, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 28.97 / 17 + bag[1] else acc = acc - 28.97 end
  while acc > 41.17 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 15.47 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 7.51, label = "f219_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 35.92 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 1.77, label = "f219_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 22.03 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 0.05, bag[k] * 2) end
  return acc % 9973
end
acc = acc + measure219(1, 2, 219)
end

do
local function fold220(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 46.15 end
  acc = cb(bag[1])
  while acc > 5.62 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 44.26, label = "f220_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 5.07 * 40 do acc = acc / 2 end
  while acc > 3.59 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 2.10, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 42.08, bag[k] * 2) end
  local tmp = { x = acc, y = 38.19, label = "f220_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 44.24 / 17 + bag[1] else acc = acc - 44.24 end
  local tmp = { x = acc, y = 30.65, label = "f220_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 40.14, label = "f220_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 17.58, label = "f220_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 19.64, bag[k] * 2) end
  local tmp = { x = acc, y = 2.39, label = "f220_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold220(1, 2, 220)
end

do
local function derive221(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 45.40 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 13.79 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 7.10 end
  acc = cb(bag[3])
  while acc > 42.77 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 33.52 / 17 + bag[1] else acc = acc - 33.52 end
  local tmp = { x = acc, y = 24.00, label = "f221_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 22.80, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 24.48 / 17 + bag[1] else acc = acc - 24.48 end
  for k = 1, 4 do acc = math.min(acc + k * 33.44, bag[k] * 2) end
  local cb = function(v) return v * acc + 31.44 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 38.97 / 17 + bag[1] else acc = acc - 38.97 end
  local tmp = { x = acc, y = 48.17, label = "f221_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 17.59 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 6.55 / 17 + bag[1] else acc = acc - 6.55 end
  return acc % 9973
end
acc = acc + derive221(1, 2, 221)
end

do
local function clampv222(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 25.74 / 17 + bag[1] else acc = acc - 25.74 end
  local cb = function(v) return v * acc + 11.61 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 16.69 / 17 + bag[1] else acc = acc - 16.69 end
  if acc % 7 > 3 then acc = acc * 17.43 / 17 + bag[1] else acc = acc - 17.43 end
  for k = 1, 4 do acc = math.min(acc + k * 24.72, bag[k] * 2) end
  local cb = function(v) return v * acc + 14.06 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 11.41 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 14.82, label = "f222_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 8.89, label = "f222_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 42.61 end
  acc = cb(bag[2])
  while acc > 43.61 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 12.73 / 17 + bag[1] else acc = acc - 12.73 end
  local cb = function(v) return v * acc + 33.13 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 18.91, bag[k] * 2) end
  return acc % 9973
end
acc = acc + clampv222(1, 2, 222)
end

do
local function compose223(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 41.51, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 20.26, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 42.15, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 25.00, bag[k] * 2) end
  while acc > 11.32 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 29.07, label = "f223_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 1.70 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 48.77, bag[k] * 2) end
  while acc > 3.18 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 21.46, label = "f223_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 21.07, bag[k] * 2) end
  while acc > 8.01 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 0.63 / 17 + bag[1] else acc = acc - 0.63 end
  local cb = function(v) return v * acc + 26.33 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + compose223(1, 2, 223)
end

do
local function derive224(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 33.76 / 17 + bag[1] else acc = acc - 33.76 end
  local cb = function(v) return v * acc + 38.27 end
  acc = cb(bag[2])
  while acc > 11.93 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 20.30, label = "f224_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 8.40 / 17 + bag[1] else acc = acc - 8.40 end
  local cb = function(v) return v * acc + 22.77 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 14.98 end
  acc = cb(bag[3])
  while acc > 32.67 * 40 do acc = acc / 2 end
  while acc > 3.56 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 44.11 / 17 + bag[1] else acc = acc - 44.11 end
  for k = 1, 4 do acc = math.min(acc + k * 5.64, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 10.46 / 17 + bag[1] else acc = acc - 10.46 end
  if acc % 7 > 3 then acc = acc * 6.11 / 17 + bag[1] else acc = acc - 6.11 end
  for k = 1, 4 do acc = math.min(acc + k * 31.41, bag[k] * 2) end
  return acc % 9973
end
acc = acc + derive224(1, 2, 224)
end

do
local function clampv225(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 46.14, label = "f225_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 33.14, label = "f225_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 33.16 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 2.19 / 17 + bag[1] else acc = acc - 2.19 end
  local tmp = { x = acc, y = 24.81, label = "f225_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 35.80 / 17 + bag[1] else acc = acc - 35.80 end
  while acc > 24.95 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 41.55, label = "f225_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 7.69, label = "f225_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 36.48 end
  acc = cb(bag[2])
  while acc > 48.22 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 24.28 end
  acc = cb(bag[4])
  while acc > 2.05 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 33.69 / 17 + bag[1] else acc = acc - 33.69 end
  return acc % 9973
end
acc = acc + clampv225(1, 2, 225)
end

do
local function fold226(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 31.14, label = "f226_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 38.87, bag[k] * 2) end
  local tmp = { x = acc, y = 43.17, label = "f226_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 41.94 / 17 + bag[1] else acc = acc - 41.94 end
  local tmp = { x = acc, y = 40.25, label = "f226_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 46.36 / 17 + bag[1] else acc = acc - 46.36 end
  while acc > 38.28 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 22.49 / 17 + bag[1] else acc = acc - 22.49 end
  local tmp = { x = acc, y = 6.08, label = "f226_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 45.89 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 26.01, bag[k] * 2) end
  local tmp = { x = acc, y = 26.89, label = "f226_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 37.99 / 17 + bag[1] else acc = acc - 37.99 end
  while acc > 9.72 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold226(1, 2, 226)
end

do
local function spread227(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 13.57 * 40 do acc = acc / 2 end
  while acc > 17.36 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 35.33 / 17 + bag[1] else acc = acc - 35.33 end
  local tmp = { x = acc, y = 24.42, label = "f227_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 24.06 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 39.49, bag[k] * 2) end
  local tmp = { x = acc, y = 43.05, label = "f227_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 41.26 end
  acc = cb(bag[4])
  while acc > 22.62 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 14.64, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 30.52, bag[k] * 2) end
  local cb = function(v) return v * acc + 41.24 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 8.31 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 21.58, label = "f227_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + spread227(1, 2, 227)
end

do
local function measure228(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 30.65 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 2.26 / 17 + bag[1] else acc = acc - 2.26 end
  local cb = function(v) return v * acc + 8.73 end
  acc = cb(bag[3])
  while acc > 0.55 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 22.59, bag[k] * 2) end
  local cb = function(v) return v * acc + 14.40 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 46.48, label = "f228_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 34.59, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 27.46, bag[k] * 2) end
  local tmp = { x = acc, y = 49.65, label = "f228_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 28.94 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 49.88, label = "f228_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 29.74 / 17 + bag[1] else acc = acc - 29.74 end
  if acc % 7 > 3 then acc = acc * 13.78 / 17 + bag[1] else acc = acc - 13.78 end
  return acc % 9973
end
acc = acc + measure228(1, 2, 228)
end

do
local function fold229(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 13.12, bag[k] * 2) end
  while acc > 5.34 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 44.48, label = "f229_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 17.88 / 17 + bag[1] else acc = acc - 17.88 end
  while acc > 33.27 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 48.03, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 38.01 / 17 + bag[1] else acc = acc - 38.01 end
  while acc > 45.17 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 24.89 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 19.82, label = "f229_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 35.07 end
  acc = cb(bag[3])
  while acc > 18.47 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 25.96 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 16.76 / 17 + bag[1] else acc = acc - 16.76 end
  return acc % 9973
end
acc = acc + fold229(1, 2, 229)
end

do
local function weight230(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 36.03 * 40 do acc = acc / 2 end
  while acc > 38.49 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 44.95, label = "f230_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 26.26 * 40 do acc = acc / 2 end
  while acc > 23.83 * 40 do acc = acc / 2 end
  while acc > 0.60 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 35.45 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 19.63, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 48.15 / 17 + bag[1] else acc = acc - 48.15 end
  if acc % 7 > 3 then acc = acc * 33.36 / 17 + bag[1] else acc = acc - 33.36 end
  while acc > 2.98 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 10.76 / 17 + bag[1] else acc = acc - 10.76 end
  local cb = function(v) return v * acc + 36.88 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 15.31, bag[k] * 2) end
  return acc % 9973
end
acc = acc + weight230(1, 2, 230)
end

do
local function compose231(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 6.72 * 40 do acc = acc / 2 end
  while acc > 47.01 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 34.10, bag[k] * 2) end
  while acc > 8.47 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 44.13, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 13.61 / 17 + bag[1] else acc = acc - 13.61 end
  if acc % 7 > 3 then acc = acc * 27.90 / 17 + bag[1] else acc = acc - 27.90 end
  while acc > 20.11 * 40 do acc = acc / 2 end
  while acc > 2.90 * 40 do acc = acc / 2 end
  while acc > 19.55 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 17.04 / 17 + bag[1] else acc = acc - 17.04 end
  local tmp = { x = acc, y = 13.42, label = "f231_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 0.70, label = "f231_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 34.24 / 17 + bag[1] else acc = acc - 34.24 end
  return acc % 9973
end
acc = acc + compose231(1, 2, 231)
end

do
local function fold232(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 8.30 / 17 + bag[1] else acc = acc - 8.30 end
  if acc % 7 > 3 then acc = acc * 33.26 / 17 + bag[1] else acc = acc - 33.26 end
  if acc % 7 > 3 then acc = acc * 18.98 / 17 + bag[1] else acc = acc - 18.98 end
  local tmp = { x = acc, y = 3.46, label = "f232_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 21.40, label = "f232_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 26.11, bag[k] * 2) end
  local cb = function(v) return v * acc + 48.91 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 31.95 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 39.99, label = "f232_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 15.99 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 21.45, bag[k] * 2) end
  local cb = function(v) return v * acc + 0.33 end
  acc = cb(bag[4])
  while acc > 1.08 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 39.62 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + fold232(1, 2, 232)
end

do
local function derive233(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 23.38, bag[k] * 2) end
  while acc > 31.78 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 17.19 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 35.30, bag[k] * 2) end
  while acc > 1.85 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 38.88, label = "f233_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 37.91, label = "f233_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 29.21, label = "f233_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 28.21, label = "f233_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 18.87, bag[k] * 2) end
  while acc > 44.13 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 47.84, label = "f233_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 14.23 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 22.72, label = "f233_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + derive233(1, 2, 233)
end

do
local function project234(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 35.91 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 4.54, bag[k] * 2) end
  local tmp = { x = acc, y = 42.23, label = "f234_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 43.53, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 24.53 / 17 + bag[1] else acc = acc - 24.53 end
  while acc > 34.11 * 40 do acc = acc / 2 end
  while acc > 30.09 * 40 do acc = acc / 2 end
  while acc > 35.95 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 26.22, bag[k] * 2) end
  while acc > 25.99 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 5.95 / 17 + bag[1] else acc = acc - 5.95 end
  for k = 1, 4 do acc = math.min(acc + k * 36.34, bag[k] * 2) end
  local cb = function(v) return v * acc + 34.78 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 32.01 / 17 + bag[1] else acc = acc - 32.01 end
  return acc % 9973
end
acc = acc + project234(1, 2, 234)
end

do
local function spread235(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 4.72 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 40.60, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 26.51 / 17 + bag[1] else acc = acc - 26.51 end
  for k = 1, 4 do acc = math.min(acc + k * 13.49, bag[k] * 2) end
  local cb = function(v) return v * acc + 42.31 end
  acc = cb(bag[1])
  while acc > 25.97 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 30.89, bag[k] * 2) end
  local cb = function(v) return v * acc + 35.33 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 12.02, label = "f235_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 9.88, label = "f235_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 38.16 / 17 + bag[1] else acc = acc - 38.16 end
  for k = 1, 4 do acc = math.min(acc + k * 0.64, bag[k] * 2) end
  local tmp = { x = acc, y = 28.53, label = "f235_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 34.11 / 17 + bag[1] else acc = acc - 34.11 end
  return acc % 9973
end
acc = acc + spread235(1, 2, 235)
end

do
local function measure236(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 45.21 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 37.33, label = "f236_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 39.47 / 17 + bag[1] else acc = acc - 39.47 end
  if acc % 7 > 3 then acc = acc * 16.36 / 17 + bag[1] else acc = acc - 16.36 end
  for k = 1, 4 do acc = math.min(acc + k * 24.50, bag[k] * 2) end
  local tmp = { x = acc, y = 43.21, label = "f236_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 7.23 / 17 + bag[1] else acc = acc - 7.23 end
  while acc > 3.60 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 44.15 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 18.69, label = "f236_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 2.52, label = "f236_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 36.37 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 47.40 / 17 + bag[1] else acc = acc - 47.40 end
  while acc > 37.69 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + measure236(1, 2, 236)
end

do
local function derive237(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 8.81, bag[k] * 2) end
  while acc > 23.97 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 35.83, bag[k] * 2) end
  while acc > 16.07 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 46.20 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 32.51 / 17 + bag[1] else acc = acc - 32.51 end
  for k = 1, 4 do acc = math.min(acc + k * 39.84, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 34.61, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 22.66, bag[k] * 2) end
  local cb = function(v) return v * acc + 0.77 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 27.00, label = "f237_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 45.96, label = "f237_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 45.32, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 11.08, bag[k] * 2) end
  return acc % 9973
end
acc = acc + derive237(1, 2, 237)
end

do
local function clampv238(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 28.42 / 17 + bag[1] else acc = acc - 28.42 end
  if acc % 7 > 3 then acc = acc * 12.45 / 17 + bag[1] else acc = acc - 12.45 end
  local cb = function(v) return v * acc + 0.11 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 41.13, bag[k] * 2) end
  local cb = function(v) return v * acc + 24.72 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 3.43 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 42.71, label = "f238_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 14.41 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 44.64 end
  acc = cb(bag[1])
  while acc > 30.69 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 9.97, bag[k] * 2) end
  while acc > 24.91 * 40 do acc = acc / 2 end
  while acc > 34.55 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 3.95 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + clampv238(1, 2, 238)
end

do
local function project239(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 1.15, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 22.77, bag[k] * 2) end
  local cb = function(v) return v * acc + 34.76 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 19.60 / 17 + bag[1] else acc = acc - 19.60 end
  local tmp = { x = acc, y = 26.31, label = "f239_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 39.84 / 17 + bag[1] else acc = acc - 39.84 end
  for k = 1, 4 do acc = math.min(acc + k * 17.31, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 43.88, bag[k] * 2) end
  local cb = function(v) return v * acc + 48.56 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 49.16, bag[k] * 2) end
  local cb = function(v) return v * acc + 5.79 end
  acc = cb(bag[3])
  while acc > 20.39 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 14.70, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 18.90 / 17 + bag[1] else acc = acc - 18.90 end
  return acc % 9973
end
acc = acc + project239(1, 2, 239)
end

do
local function project240(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 42.38 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 10.51, label = "f240_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 21.27 / 17 + bag[1] else acc = acc - 21.27 end
  local cb = function(v) return v * acc + 40.07 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 5.68, bag[k] * 2) end
  local cb = function(v) return v * acc + 33.07 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 10.36 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 37.18, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 41.36, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 2.90, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 36.03, bag[k] * 2) end
  local cb = function(v) return v * acc + 35.23 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 7.73, bag[k] * 2) end
  while acc > 45.66 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + project240(1, 2, 240)
end

do
local function clampv241(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 25.64, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 18.45 / 17 + bag[1] else acc = acc - 18.45 end
  local cb = function(v) return v * acc + 11.66 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 13.15, bag[k] * 2) end
  while acc > 47.72 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 41.44 / 17 + bag[1] else acc = acc - 41.44 end
  while acc > 4.17 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 28.56 end
  acc = cb(bag[4])
  while acc > 44.76 * 40 do acc = acc / 2 end
  while acc > 10.25 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 38.95 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 24.12 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 0.66, label = "f241_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 15.48, label = "f241_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv241(1, 2, 241)
end

do
local function weight242(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 19.94 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 15.96 / 17 + bag[1] else acc = acc - 15.96 end
  for k = 1, 4 do acc = math.min(acc + k * 4.12, bag[k] * 2) end
  local tmp = { x = acc, y = 47.58, label = "f242_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 7.41 / 17 + bag[1] else acc = acc - 7.41 end
  local tmp = { x = acc, y = 10.05, label = "f242_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 31.25, label = "f242_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 37.28 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 39.41, label = "f242_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 0.60, label = "f242_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 36.83, bag[k] * 2) end
  local cb = function(v) return v * acc + 7.99 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 47.38, label = "f242_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 13.64, label = "f242_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + weight242(1, 2, 242)
end

do
local function fold243(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 47.05, bag[k] * 2) end
  while acc > 8.51 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 9.04, label = "f243_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 45.93, bag[k] * 2) end
  local tmp = { x = acc, y = 12.69, label = "f243_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 13.84 / 17 + bag[1] else acc = acc - 13.84 end
  local tmp = { x = acc, y = 37.78, label = "f243_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 3.42, bag[k] * 2) end
  local cb = function(v) return v * acc + 16.10 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 10.04, bag[k] * 2) end
  local cb = function(v) return v * acc + 9.87 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 26.99, label = "f243_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 9.18, bag[k] * 2) end
  local tmp = { x = acc, y = 49.25, label = "f243_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold243(1, 2, 243)
end

do
local function fold244(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 40.69, label = "f244_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 46.75, bag[k] * 2) end
  local cb = function(v) return v * acc + 39.36 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 18.18 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 18.60, bag[k] * 2) end
  local cb = function(v) return v * acc + 5.78 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 49.50, label = "f244_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 44.00, label = "f244_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 31.21 / 17 + bag[1] else acc = acc - 31.21 end
  if acc % 7 > 3 then acc = acc * 37.56 / 17 + bag[1] else acc = acc - 37.56 end
  while acc > 22.92 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 18.01, bag[k] * 2) end
  local tmp = { x = acc, y = 22.06, label = "f244_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 48.36 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + fold244(1, 2, 244)
end

do
local function derive245(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 33.54, label = "f245_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 17.31 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 33.36 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 0.50 / 17 + bag[1] else acc = acc - 0.50 end
  for k = 1, 4 do acc = math.min(acc + k * 13.97, bag[k] * 2) end
  local tmp = { x = acc, y = 4.81, label = "f245_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 7.48 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 33.43 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 7.51 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 33.87 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 33.48, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 27.74, bag[k] * 2) end
  local tmp = { x = acc, y = 47.21, label = "f245_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 11.88, bag[k] * 2) end
  return acc % 9973
end
acc = acc + derive245(1, 2, 245)
end

do
local function clampv246(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 6.42, label = "f246_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 45.08 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 42.13 / 17 + bag[1] else acc = acc - 42.13 end
  for k = 1, 4 do acc = math.min(acc + k * 43.86, bag[k] * 2) end
  local tmp = { x = acc, y = 24.20, label = "f246_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 36.46 / 17 + bag[1] else acc = acc - 36.46 end
  local cb = function(v) return v * acc + 42.92 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 30.81 / 17 + bag[1] else acc = acc - 30.81 end
  if acc % 7 > 3 then acc = acc * 24.13 / 17 + bag[1] else acc = acc - 24.13 end
  local tmp = { x = acc, y = 24.74, label = "f246_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 17.59, label = "f246_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 30.90 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 7.10 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 40.78 / 17 + bag[1] else acc = acc - 40.78 end
  return acc % 9973
end
acc = acc + clampv246(1, 2, 246)
end

do
local function measure247(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 16.39 end
  acc = cb(bag[1])
  while acc > 12.36 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 43.02 / 17 + bag[1] else acc = acc - 43.02 end
  if acc % 7 > 3 then acc = acc * 18.06 / 17 + bag[1] else acc = acc - 18.06 end
  if acc % 7 > 3 then acc = acc * 36.95 / 17 + bag[1] else acc = acc - 36.95 end
  while acc > 3.36 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 39.94, bag[k] * 2) end
  local cb = function(v) return v * acc + 46.13 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 6.04 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 21.10 / 17 + bag[1] else acc = acc - 21.10 end
  for k = 1, 4 do acc = math.min(acc + k * 4.35, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 44.84 / 17 + bag[1] else acc = acc - 44.84 end
  while acc > 22.76 * 40 do acc = acc / 2 end
  while acc > 44.00 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + measure247(1, 2, 247)
end

do
local function measure248(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 13.08 / 17 + bag[1] else acc = acc - 13.08 end
  local cb = function(v) return v * acc + 13.00 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 25.24 end
  acc = cb(bag[3])
  while acc > 3.07 * 40 do acc = acc / 2 end
  while acc > 36.24 * 40 do acc = acc / 2 end
  while acc > 43.77 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 43.37 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 37.27 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 13.87 / 17 + bag[1] else acc = acc - 13.87 end
  local tmp = { x = acc, y = 39.92, label = "f248_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 34.69 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 30.69, bag[k] * 2) end
  local cb = function(v) return v * acc + 12.42 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 38.00 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + measure248(1, 2, 248)
end

do
local function fold249(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 16.40, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 17.72 / 17 + bag[1] else acc = acc - 17.72 end
  local cb = function(v) return v * acc + 44.40 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 33.03 / 17 + bag[1] else acc = acc - 33.03 end
  for k = 1, 4 do acc = math.min(acc + k * 13.61, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 10.96, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 48.28, bag[k] * 2) end
  local tmp = { x = acc, y = 6.41, label = "f249_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 2.55, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 43.74 / 17 + bag[1] else acc = acc - 43.74 end
  for k = 1, 4 do acc = math.min(acc + k * 49.38, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 13.76 / 17 + bag[1] else acc = acc - 13.76 end
  for k = 1, 4 do acc = math.min(acc + k * 10.07, bag[k] * 2) end
  local cb = function(v) return v * acc + 11.63 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + fold249(1, 2, 249)
end

do
local function project250(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 32.41 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 15.62 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 8.91 / 17 + bag[1] else acc = acc - 8.91 end
  if acc % 7 > 3 then acc = acc * 21.93 / 17 + bag[1] else acc = acc - 21.93 end
  for k = 1, 4 do acc = math.min(acc + k * 43.14, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 7.68 / 17 + bag[1] else acc = acc - 7.68 end
  local cb = function(v) return v * acc + 15.42 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 46.12, label = "f250_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 17.16 end
  acc = cb(bag[1])
  while acc > 7.24 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 30.95 / 17 + bag[1] else acc = acc - 30.95 end
  for k = 1, 4 do acc = math.min(acc + k * 21.62, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 17.42 / 17 + bag[1] else acc = acc - 17.42 end
  for k = 1, 4 do acc = math.min(acc + k * 24.80, bag[k] * 2) end
  return acc % 9973
end
acc = acc + project250(1, 2, 250)
end

do
local function clampv251(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 27.62 / 17 + bag[1] else acc = acc - 27.62 end
  for k = 1, 4 do acc = math.min(acc + k * 39.02, bag[k] * 2) end
  local tmp = { x = acc, y = 11.82, label = "f251_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 39.52, bag[k] * 2) end
  while acc > 15.06 * 40 do acc = acc / 2 end
  while acc > 1.28 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 22.95, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 39.32, bag[k] * 2) end
  local cb = function(v) return v * acc + 27.17 end
  acc = cb(bag[1])
  while acc > 10.80 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 6.41, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 21.39 / 17 + bag[1] else acc = acc - 21.39 end
  if acc % 7 > 3 then acc = acc * 2.77 / 17 + bag[1] else acc = acc - 2.77 end
  if acc % 7 > 3 then acc = acc * 30.60 / 17 + bag[1] else acc = acc - 30.60 end
  return acc % 9973
end
acc = acc + clampv251(1, 2, 251)
end

do
local function project252(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 16.42 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 33.05, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 10.14 / 17 + bag[1] else acc = acc - 10.14 end
  local tmp = { x = acc, y = 12.55, label = "f252_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 14.29, label = "f252_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 16.47 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 13.13, bag[k] * 2) end
  local cb = function(v) return v * acc + 2.54 end
  acc = cb(bag[4])
  while acc > 30.50 * 40 do acc = acc / 2 end
  while acc > 11.89 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 35.71 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 19.00 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 15.98 / 17 + bag[1] else acc = acc - 15.98 end
  local cb = function(v) return v * acc + 12.69 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + project252(1, 2, 252)
end

do
local function measure253(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 39.22, label = "f253_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 6.06 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 4.40 / 17 + bag[1] else acc = acc - 4.40 end
  if acc % 7 > 3 then acc = acc * 25.26 / 17 + bag[1] else acc = acc - 25.26 end
  while acc > 5.31 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 4.96, label = "f253_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 19.30 / 17 + bag[1] else acc = acc - 19.30 end
  for k = 1, 4 do acc = math.min(acc + k * 33.06, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 4.19 / 17 + bag[1] else acc = acc - 4.19 end
  local tmp = { x = acc, y = 46.55, label = "f253_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 47.15, bag[k] * 2) end
  while acc > 37.21 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 4.67, bag[k] * 2) end
  local tmp = { x = acc, y = 30.14, label = "f253_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + measure253(1, 2, 253)
end

do
local function fold254(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 44.69, bag[k] * 2) end
  local tmp = { x = acc, y = 22.60, label = "f254_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 12.33 / 17 + bag[1] else acc = acc - 12.33 end
  for k = 1, 4 do acc = math.min(acc + k * 15.56, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 17.66, bag[k] * 2) end
  while acc > 25.81 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 31.23, label = "f254_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 26.42 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 16.53 / 17 + bag[1] else acc = acc - 16.53 end
  for k = 1, 4 do acc = math.min(acc + k * 8.59, bag[k] * 2) end
  while acc > 46.35 * 40 do acc = acc / 2 end
  while acc > 10.14 * 40 do acc = acc / 2 end
  while acc > 23.36 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 28.50, bag[k] * 2) end
  return acc % 9973
end
acc = acc + fold254(1, 2, 254)
end

do
local function clampv255(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 22.13, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 37.45, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 45.89 / 17 + bag[1] else acc = acc - 45.89 end
  while acc > 12.02 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 32.14 / 17 + bag[1] else acc = acc - 32.14 end
  if acc % 7 > 3 then acc = acc * 25.05 / 17 + bag[1] else acc = acc - 25.05 end
  local cb = function(v) return v * acc + 23.15 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 2.66, label = "f255_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 32.78, bag[k] * 2) end
  local tmp = { x = acc, y = 30.58, label = "f255_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 21.94, bag[k] * 2) end
  local cb = function(v) return v * acc + 44.38 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 26.71 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 7.27, label = "f255_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv255(1, 2, 255)
end

do
local function spread256(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 0.91 * 40 do acc = acc / 2 end
  while acc > 33.77 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 38.88 end
  acc = cb(bag[3])
  while acc > 43.64 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 0.97, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 9.22, bag[k] * 2) end
  local tmp = { x = acc, y = 42.08, label = "f256_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 1.80 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 39.23 / 17 + bag[1] else acc = acc - 39.23 end
  for k = 1, 4 do acc = math.min(acc + k * 42.01, bag[k] * 2) end
  local tmp = { x = acc, y = 4.41, label = "f256_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 13.44 / 17 + bag[1] else acc = acc - 13.44 end
  local cb = function(v) return v * acc + 42.74 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 42.18, label = "f256_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + spread256(1, 2, 256)
end

do
local function fold257(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 48.30, label = "f257_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 5.07 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 3.72, bag[k] * 2) end
  while acc > 49.23 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 42.68 / 17 + bag[1] else acc = acc - 42.68 end
  local cb = function(v) return v * acc + 44.44 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 40.95, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 10.49 / 17 + bag[1] else acc = acc - 10.49 end
  while acc > 32.25 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 26.23 / 17 + bag[1] else acc = acc - 26.23 end
  if acc % 7 > 3 then acc = acc * 14.52 / 17 + bag[1] else acc = acc - 14.52 end
  local tmp = { x = acc, y = 47.85, label = "f257_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 10.73 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 44.46, bag[k] * 2) end
  return acc % 9973
end
acc = acc + fold257(1, 2, 257)
end

do
local function spread258(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 19.31, label = "f258_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 2.92 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 22.58 / 17 + bag[1] else acc = acc - 22.58 end
  local cb = function(v) return v * acc + 1.12 end
  acc = cb(bag[4])
  while acc > 30.53 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 37.85 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 41.52 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 33.31 / 17 + bag[1] else acc = acc - 33.31 end
  while acc > 1.44 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 34.31, bag[k] * 2) end
  local tmp = { x = acc, y = 13.24, label = "f258_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 45.78, bag[k] * 2) end
  local cb = function(v) return v * acc + 10.97 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 13.14 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + spread258(1, 2, 258)
end

do
local function derive259(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 12.84, label = "f259_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 13.18, bag[k] * 2) end
  local tmp = { x = acc, y = 16.32, label = "f259_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 11.39, bag[k] * 2) end
  local tmp = { x = acc, y = 46.62, label = "f259_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 19.59 end
  acc = cb(bag[2])
  while acc > 31.53 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 13.93 / 17 + bag[1] else acc = acc - 13.93 end
  while acc > 8.46 * 40 do acc = acc / 2 end
  while acc > 1.63 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 23.38 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 30.55, label = "f259_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 1.93, bag[k] * 2) end
  local cb = function(v) return v * acc + 36.82 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + derive259(1, 2, 259)
end

do
local function project260(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 37.11, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 20.50 / 17 + bag[1] else acc = acc - 20.50 end
  local tmp = { x = acc, y = 27.25, label = "f260_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 22.32, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 28.66, bag[k] * 2) end
  local tmp = { x = acc, y = 7.61, label = "f260_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 17.92, label = "f260_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 15.08 / 17 + bag[1] else acc = acc - 15.08 end
  for k = 1, 4 do acc = math.min(acc + k * 36.94, bag[k] * 2) end
  while acc > 39.56 * 40 do acc = acc / 2 end
  while acc > 25.90 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 3.48, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 43.07, bag[k] * 2) end
  local tmp = { x = acc, y = 42.28, label = "f260_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + project260(1, 2, 260)
end

do
local function weight261(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 34.12 / 17 + bag[1] else acc = acc - 34.12 end
  for k = 1, 4 do acc = math.min(acc + k * 24.78, bag[k] * 2) end
  local cb = function(v) return v * acc + 13.55 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 15.17, label = "f261_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 49.76, label = "f261_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 29.52, label = "f261_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 13.12 * 40 do acc = acc / 2 end
  while acc > 8.63 * 40 do acc = acc / 2 end
  while acc > 43.26 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 31.02 end
  acc = cb(bag[2])
  while acc > 22.93 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 10.06 / 17 + bag[1] else acc = acc - 10.06 end
  while acc > 32.21 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 15.59, label = "f261_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + weight261(1, 2, 261)
end

do
local function measure262(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 3.62 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 14.90 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 18.28, label = "f262_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 28.49, bag[k] * 2) end
  local cb = function(v) return v * acc + 29.32 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 9.50 / 17 + bag[1] else acc = acc - 9.50 end
  local cb = function(v) return v * acc + 38.61 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 17.26, label = "f262_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 1.76 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 18.57, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 0.38, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 22.92, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 1.47 / 17 + bag[1] else acc = acc - 1.47 end
  local cb = function(v) return v * acc + 47.55 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + measure262(1, 2, 262)
end

do
local function spread263(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 28.76 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 5.52 end
  acc = cb(bag[2])
  while acc > 20.13 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 38.71 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 10.72 / 17 + bag[1] else acc = acc - 10.72 end
  while acc > 8.18 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 6.44, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 47.72 / 17 + bag[1] else acc = acc - 47.72 end
  if acc % 7 > 3 then acc = acc * 46.63 / 17 + bag[1] else acc = acc - 46.63 end
  for k = 1, 4 do acc = math.min(acc + k * 42.41, bag[k] * 2) end
  local tmp = { x = acc, y = 17.90, label = "f263_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 5.05 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 22.14, bag[k] * 2) end
  local tmp = { x = acc, y = 3.89, label = "f263_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + spread263(1, 2, 263)
end

do
local function clampv264(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 33.25 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 10.11 end
  acc = cb(bag[2])
  while acc > 47.92 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 30.68 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 34.75 / 17 + bag[1] else acc = acc - 34.75 end
  if acc % 7 > 3 then acc = acc * 41.23 / 17 + bag[1] else acc = acc - 41.23 end
  local tmp = { x = acc, y = 17.54, label = "f264_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 36.13, label = "f264_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 49.36, label = "f264_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 29.56 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 41.42 / 17 + bag[1] else acc = acc - 41.42 end
  for k = 1, 4 do acc = math.min(acc + k * 42.21, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 32.78, bag[k] * 2) end
  local cb = function(v) return v * acc + 11.84 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + clampv264(1, 2, 264)
end

do
local function derive265(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 39.00 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 30.41 / 17 + bag[1] else acc = acc - 30.41 end
  local tmp = { x = acc, y = 15.87, label = "f265_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 10.62 / 17 + bag[1] else acc = acc - 10.62 end
  local tmp = { x = acc, y = 24.09, label = "f265_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 45.04, label = "f265_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 4.56, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 10.16, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 32.10, bag[k] * 2) end
  while acc > 18.13 * 40 do acc = acc / 2 end
  while acc > 29.57 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 21.81 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 0.68, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 13.74, bag[k] * 2) end
  return acc % 9973
end
acc = acc + derive265(1, 2, 265)
end

do
local function weight266(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 27.04 * 40 do acc = acc / 2 end
  while acc > 44.75 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 45.97 / 17 + bag[1] else acc = acc - 45.97 end
  local tmp = { x = acc, y = 4.82, label = "f266_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 32.19, bag[k] * 2) end
  local tmp = { x = acc, y = 39.93, label = "f266_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 18.21 / 17 + bag[1] else acc = acc - 18.21 end
  local tmp = { x = acc, y = 29.92, label = "f266_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 39.52, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 12.90, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 1.53 / 17 + bag[1] else acc = acc - 1.53 end
  for k = 1, 4 do acc = math.min(acc + k * 37.50, bag[k] * 2) end
  local tmp = { x = acc, y = 19.04, label = "f266_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 36.36 / 17 + bag[1] else acc = acc - 36.36 end
  return acc % 9973
end
acc = acc + weight266(1, 2, 266)
end

do
local function spread267(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 43.81, label = "f267_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 49.51 / 17 + bag[1] else acc = acc - 49.51 end
  local cb = function(v) return v * acc + 16.10 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 28.83, bag[k] * 2) end
  local cb = function(v) return v * acc + 20.38 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 35.67, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 7.16 / 17 + bag[1] else acc = acc - 7.16 end
  for k = 1, 4 do acc = math.min(acc + k * 4.33, bag[k] * 2) end
  local tmp = { x = acc, y = 45.76, label = "f267_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 46.36 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 6.94, label = "f267_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 3.91 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 44.24, label = "f267_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 30.32, label = "f267_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + spread267(1, 2, 267)
end

do
local function derive268(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 27.91, label = "f268_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 13.44 / 17 + bag[1] else acc = acc - 13.44 end
  while acc > 42.55 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 15.32, label = "f268_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 22.10 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 3.38 end
  acc = cb(bag[2])
  while acc > 7.14 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 0.79, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 24.81 / 17 + bag[1] else acc = acc - 24.81 end
  for k = 1, 4 do acc = math.min(acc + k * 0.27, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 2.04, bag[k] * 2) end
  local cb = function(v) return v * acc + 35.45 end
  acc = cb(bag[4])
  while acc > 35.71 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 48.62, label = "f268_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + derive268(1, 2, 268)
end

do
local function weight269(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 4.10, label = "f269_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 3.20 * 40 do acc = acc / 2 end
  while acc > 20.42 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 36.18, label = "f269_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 36.93 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 16.00, label = "f269_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 12.14 / 17 + bag[1] else acc = acc - 12.14 end
  while acc > 42.52 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 38.51, bag[k] * 2) end
  local cb = function(v) return v * acc + 31.93 end
  acc = cb(bag[2])
  while acc > 32.05 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 20.98, label = "f269_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 41.31 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 47.40, bag[k] * 2) end
  return acc % 9973
end
acc = acc + weight269(1, 2, 269)
end

do
local function fold270(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 47.02 / 17 + bag[1] else acc = acc - 47.02 end
  while acc > 12.78 * 40 do acc = acc / 2 end
  while acc > 21.78 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 16.11, label = "f270_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 45.46, label = "f270_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 15.32 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 35.56 / 17 + bag[1] else acc = acc - 35.56 end
  local tmp = { x = acc, y = 38.46, label = "f270_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 8.63 / 17 + bag[1] else acc = acc - 8.63 end
  local tmp = { x = acc, y = 2.74, label = "f270_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 36.66 / 17 + bag[1] else acc = acc - 36.66 end
  for k = 1, 4 do acc = math.min(acc + k * 9.66, bag[k] * 2) end
  local tmp = { x = acc, y = 23.39, label = "f270_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 15.20 / 17 + bag[1] else acc = acc - 15.20 end
  return acc % 9973
end
acc = acc + fold270(1, 2, 270)
end

do
local function derive271(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 40.58, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 4.51, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 24.56, bag[k] * 2) end
  local cb = function(v) return v * acc + 10.57 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 20.60 / 17 + bag[1] else acc = acc - 20.60 end
  while acc > 11.23 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 36.82 / 17 + bag[1] else acc = acc - 36.82 end
  while acc > 48.28 * 40 do acc = acc / 2 end
  while acc > 35.35 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 41.92 / 17 + bag[1] else acc = acc - 41.92 end
  while acc > 0.22 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 43.80 end
  acc = cb(bag[4])
  while acc > 10.93 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 5.53 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + derive271(1, 2, 271)
end

do
local function weight272(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 5.94 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 20.92, label = "f272_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 20.29 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 43.53, label = "f272_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 31.61, label = "f272_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 25.34 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 42.77, label = "f272_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 39.77 / 17 + bag[1] else acc = acc - 39.77 end
  if acc % 7 > 3 then acc = acc * 29.17 / 17 + bag[1] else acc = acc - 29.17 end
  local tmp = { x = acc, y = 6.95, label = "f272_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 36.85, label = "f272_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 31.68, bag[k] * 2) end
  local cb = function(v) return v * acc + 16.68 end
  acc = cb(bag[1])
  while acc > 23.06 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + weight272(1, 2, 272)
end

do
local function project273(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 12.70, bag[k] * 2) end
  while acc > 32.24 * 40 do acc = acc / 2 end
  while acc > 38.10 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 38.90, label = "f273_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 12.59, bag[k] * 2) end
  while acc > 39.35 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 34.40 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 27.78 / 17 + bag[1] else acc = acc - 27.78 end
  local cb = function(v) return v * acc + 15.65 end
  acc = cb(bag[1])
  while acc > 3.79 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 45.14 / 17 + bag[1] else acc = acc - 45.14 end
  for k = 1, 4 do acc = math.min(acc + k * 29.86, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 33.64, bag[k] * 2) end
  local tmp = { x = acc, y = 9.34, label = "f273_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + project273(1, 2, 273)
end

do
local function spread274(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 9.17 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 14.34 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 16.81 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 34.48 / 17 + bag[1] else acc = acc - 34.48 end
  for k = 1, 4 do acc = math.min(acc + k * 46.88, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 11.27 / 17 + bag[1] else acc = acc - 11.27 end
  local cb = function(v) return v * acc + 21.66 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 14.51 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 21.94 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 36.87, label = "f274_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 39.76 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 20.10 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 11.18, label = "f274_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 22.79 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + spread274(1, 2, 274)
end

do
local function project275(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 22.42 / 17 + bag[1] else acc = acc - 22.42 end
  local cb = function(v) return v * acc + 43.90 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 16.88 / 17 + bag[1] else acc = acc - 16.88 end
  local tmp = { x = acc, y = 25.83, label = "f275_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 4.34 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 20.06 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 45.19, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 38.36, bag[k] * 2) end
  while acc > 39.73 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 29.35, label = "f275_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 10.13, label = "f275_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 16.37, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 7.65, bag[k] * 2) end
  local tmp = { x = acc, y = 1.58, label = "f275_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + project275(1, 2, 275)
end

do
local function clampv276(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 47.48, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 7.68, bag[k] * 2) end
  while acc > 15.35 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 36.04 / 17 + bag[1] else acc = acc - 36.04 end
  while acc > 12.09 * 40 do acc = acc / 2 end
  while acc > 46.92 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 19.04, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 9.53 / 17 + bag[1] else acc = acc - 9.53 end
  local cb = function(v) return v * acc + 2.14 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 48.24, label = "f276_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 8.42, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 41.71, bag[k] * 2) end
  local cb = function(v) return v * acc + 39.84 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 1.36 / 17 + bag[1] else acc = acc - 1.36 end
  return acc % 9973
end
acc = acc + clampv276(1, 2, 276)
end

do
local function derive277(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 49.41 end
  acc = cb(bag[1])
  while acc > 38.23 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 42.99, bag[k] * 2) end
  local tmp = { x = acc, y = 6.39, label = "f277_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 31.48, bag[k] * 2) end
  while acc > 18.01 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 16.39, label = "f277_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 38.52 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 30.06, label = "f277_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 24.53, label = "f277_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 31.68, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 0.95 / 17 + bag[1] else acc = acc - 0.95 end
  if acc % 7 > 3 then acc = acc * 11.69 / 17 + bag[1] else acc = acc - 11.69 end
  if acc % 7 > 3 then acc = acc * 15.57 / 17 + bag[1] else acc = acc - 15.57 end
  return acc % 9973
end
acc = acc + derive277(1, 2, 277)
end

do
local function clampv278(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 12.86 / 17 + bag[1] else acc = acc - 12.86 end
  local cb = function(v) return v * acc + 43.30 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 29.97 / 17 + bag[1] else acc = acc - 29.97 end
  local tmp = { x = acc, y = 21.19, label = "f278_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 0.90, label = "f278_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 16.75 / 17 + bag[1] else acc = acc - 16.75 end
  if acc % 7 > 3 then acc = acc * 38.76 / 17 + bag[1] else acc = acc - 38.76 end
  local tmp = { x = acc, y = 33.01, label = "f278_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 16.89, label = "f278_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 17.47, label = "f278_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 37.43 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 20.29, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 2.62 / 17 + bag[1] else acc = acc - 2.62 end
  if acc % 7 > 3 then acc = acc * 22.75 / 17 + bag[1] else acc = acc - 22.75 end
  return acc % 9973
end
acc = acc + clampv278(1, 2, 278)
end

do
local function fold279(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 49.62, label = "f279_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 13.09 / 17 + bag[1] else acc = acc - 13.09 end
  while acc > 32.94 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 27.08 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 5.78 / 17 + bag[1] else acc = acc - 5.78 end
  while acc > 36.18 * 40 do acc = acc / 2 end
  while acc > 4.89 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 44.03 / 17 + bag[1] else acc = acc - 44.03 end
  if acc % 7 > 3 then acc = acc * 6.66 / 17 + bag[1] else acc = acc - 6.66 end
  local tmp = { x = acc, y = 1.41, label = "f279_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 14.69 * 40 do acc = acc / 2 end
  while acc > 26.52 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 4.50 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 4.89, label = "f279_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold279(1, 2, 279)
end

do
local function derive280(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 33.00, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 37.24, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 38.64 / 17 + bag[1] else acc = acc - 38.64 end
  while acc > 17.77 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 35.13, label = "f280_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 36.98 / 17 + bag[1] else acc = acc - 36.98 end
  if acc % 7 > 3 then acc = acc * 25.92 / 17 + bag[1] else acc = acc - 25.92 end
  local tmp = { x = acc, y = 17.75, label = "f280_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 47.61 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 44.31, bag[k] * 2) end
  local tmp = { x = acc, y = 40.43, label = "f280_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 29.50, bag[k] * 2) end
  while acc > 0.14 * 40 do acc = acc / 2 end
  while acc > 21.64 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive280(1, 2, 280)
end

do
local function measure281(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 7.26 / 17 + bag[1] else acc = acc - 7.26 end
  local tmp = { x = acc, y = 42.42, label = "f281_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 17.33, label = "f281_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 47.46 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 13.83 / 17 + bag[1] else acc = acc - 13.83 end
  while acc > 35.36 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 35.45 / 17 + bag[1] else acc = acc - 35.45 end
  local tmp = { x = acc, y = 10.68, label = "f281_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 25.18 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 2.54 end
  acc = cb(bag[2])
  while acc > 32.19 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 40.11, label = "f281_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 10.44 / 17 + bag[1] else acc = acc - 10.44 end
  if acc % 7 > 3 then acc = acc * 25.30 / 17 + bag[1] else acc = acc - 25.30 end
  return acc % 9973
end
acc = acc + measure281(1, 2, 281)
end

do
local function fold282(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 36.27 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 8.10, label = "f282_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 14.88 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 47.98, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 35.02, bag[k] * 2) end
  while acc > 14.72 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 16.24, bag[k] * 2) end
  local cb = function(v) return v * acc + 28.42 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 23.68 / 17 + bag[1] else acc = acc - 23.68 end
  for k = 1, 4 do acc = math.min(acc + k * 10.95, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 34.47, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 39.21, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 42.22, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 10.20 / 17 + bag[1] else acc = acc - 10.20 end
  return acc % 9973
end
acc = acc + fold282(1, 2, 282)
end

do
local function spread283(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 37.94 / 17 + bag[1] else acc = acc - 37.94 end
  while acc > 25.18 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 27.31, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 35.20, bag[k] * 2) end
  local tmp = { x = acc, y = 1.31, label = "f283_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 30.67, bag[k] * 2) end
  local tmp = { x = acc, y = 36.55, label = "f283_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 4.89 / 17 + bag[1] else acc = acc - 4.89 end
  if acc % 7 > 3 then acc = acc * 38.80 / 17 + bag[1] else acc = acc - 38.80 end
  for k = 1, 4 do acc = math.min(acc + k * 17.34, bag[k] * 2) end
  local tmp = { x = acc, y = 39.89, label = "f283_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 6.71 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 46.46 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 9.08, bag[k] * 2) end
  return acc % 9973
end
acc = acc + spread283(1, 2, 283)
end

do
local function measure284(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 39.32 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 5.06 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 21.46, label = "f284_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 40.55, label = "f284_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 15.08 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 30.44 / 17 + bag[1] else acc = acc - 30.44 end
  local cb = function(v) return v * acc + 36.63 end
  acc = cb(bag[3])
  local cb = function(v) return v * acc + 22.34 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 43.20 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 7.93, label = "f284_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 15.41 * 40 do acc = acc / 2 end
  while acc > 28.01 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 38.05 / 17 + bag[1] else acc = acc - 38.05 end
  local cb = function(v) return v * acc + 39.77 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + measure284(1, 2, 284)
end

do
local function project285(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 37.16 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 6.49 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 36.65 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 47.73 / 17 + bag[1] else acc = acc - 47.73 end
  for k = 1, 4 do acc = math.min(acc + k * 3.69, bag[k] * 2) end
  local cb = function(v) return v * acc + 20.97 end
  acc = cb(bag[2])
  while acc > 9.86 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 26.58, bag[k] * 2) end
  local tmp = { x = acc, y = 42.99, label = "f285_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 33.57 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 0.16 / 17 + bag[1] else acc = acc - 0.16 end
  for k = 1, 4 do acc = math.min(acc + k * 45.07, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 28.08, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 42.86 / 17 + bag[1] else acc = acc - 42.86 end
  return acc % 9973
end
acc = acc + project285(1, 2, 285)
end

do
local function weight286(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 37.26 end
  acc = cb(bag[1])
  while acc > 13.17 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 39.54 / 17 + bag[1] else acc = acc - 39.54 end
  local cb = function(v) return v * acc + 22.11 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 18.49 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 17.08 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 20.99 / 17 + bag[1] else acc = acc - 20.99 end
  for k = 1, 4 do acc = math.min(acc + k * 18.66, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 12.45, bag[k] * 2) end
  while acc > 2.14 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 35.59, label = "f286_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 21.80, bag[k] * 2) end
  local cb = function(v) return v * acc + 4.98 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 0.84, label = "f286_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + weight286(1, 2, 286)
end

do
local function compose287(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 20.19 / 17 + bag[1] else acc = acc - 20.19 end
  local cb = function(v) return v * acc + 26.49 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 7.54 end
  acc = cb(bag[3])
  while acc > 2.25 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 43.90 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 32.60 / 17 + bag[1] else acc = acc - 32.60 end
  local cb = function(v) return v * acc + 21.47 end
  acc = cb(bag[3])
  while acc > 2.94 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 29.02, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 12.34 / 17 + bag[1] else acc = acc - 12.34 end
  local tmp = { x = acc, y = 24.71, label = "f287_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 1.80, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 17.66, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 26.07, bag[k] * 2) end
  return acc % 9973
end
acc = acc + compose287(1, 2, 287)
end

do
local function fold288(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 24.42, bag[k] * 2) end
  local cb = function(v) return v * acc + 25.85 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 10.95 end
  acc = cb(bag[3])
  while acc > 44.12 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 6.84, label = "f288_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 10.66 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 11.49, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 45.23, bag[k] * 2) end
  local tmp = { x = acc, y = 16.65, label = "f288_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 12.42 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 2.47, label = "f288_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 46.51 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 10.16, label = "f288_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 23.64, label = "f288_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + fold288(1, 2, 288)
end

do
local function derive289(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 46.95, bag[k] * 2) end
  local cb = function(v) return v * acc + 31.61 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 14.50 / 17 + bag[1] else acc = acc - 14.50 end
  if acc % 7 > 3 then acc = acc * 0.77 / 17 + bag[1] else acc = acc - 0.77 end
  local tmp = { x = acc, y = 35.20, label = "f289_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 20.93 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 24.87, label = "f289_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 43.77, label = "f289_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 10.64 end
  acc = cb(bag[1])
  while acc > 20.34 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 12.60, bag[k] * 2) end
  local cb = function(v) return v * acc + 10.28 end
  acc = cb(bag[4])
  if acc % 7 > 3 then acc = acc * 36.25 / 17 + bag[1] else acc = acc - 36.25 end
  while acc > 14.71 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + derive289(1, 2, 289)
end

do
local function measure290(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 8.94, bag[k] * 2) end
  local cb = function(v) return v * acc + 40.66 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 20.34, label = "f290_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 12.48 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 47.28, bag[k] * 2) end
  local cb = function(v) return v * acc + 14.22 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 28.57, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 19.25, bag[k] * 2) end
  while acc > 48.32 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 27.06 / 17 + bag[1] else acc = acc - 27.06 end
  for k = 1, 4 do acc = math.min(acc + k * 44.32, bag[k] * 2) end
  while acc > 46.77 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 12.13, label = "f290_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 12.67 / 17 + bag[1] else acc = acc - 12.67 end
  return acc % 9973
end
acc = acc + measure290(1, 2, 290)
end

do
local function clampv291(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 34.36 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 26.33 end
  acc = cb(bag[2])
  while acc > 40.23 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 36.31 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 19.27, label = "f291_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 12.04 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 2.22, bag[k] * 2) end
  local tmp = { x = acc, y = 29.67, label = "f291_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 12.70, bag[k] * 2) end
  local tmp = { x = acc, y = 36.87, label = "f291_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 44.60 end
  acc = cb(bag[3])
  for k = 1, 4 do acc = math.min(acc + k * 31.55, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 18.02 / 17 + bag[1] else acc = acc - 18.02 end
  for k = 1, 4 do acc = math.min(acc + k * 34.89, bag[k] * 2) end
  return acc % 9973
end
acc = acc + clampv291(1, 2, 291)
end

do
local function compose292(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local cb = function(v) return v * acc + 11.12 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 31.15 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 40.40, label = "f292_l2" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 34.44, bag[k] * 2) end
  local tmp = { x = acc, y = 16.54, label = "f292_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 28.56 end
  acc = cb(bag[2])
  local cb = function(v) return v * acc + 15.49 end
  acc = cb(bag[3])
  local tmp = { x = acc, y = 5.07, label = "f292_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 49.19, label = "f292_l8" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 23.05 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 37.93, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 48.26 / 17 + bag[1] else acc = acc - 48.26 end
  if acc % 7 > 3 then acc = acc * 0.88 / 17 + bag[1] else acc = acc - 0.88 end
  local tmp = { x = acc, y = 37.30, label = "f292_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + compose292(1, 2, 292)
end

do
local function measure293(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 19.32 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 17.60, label = "f293_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 38.82 / 17 + bag[1] else acc = acc - 38.82 end
  local tmp = { x = acc, y = 28.24, label = "f293_l3" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 25.63, label = "f293_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 48.43 end
  acc = cb(bag[2])
  local tmp = { x = acc, y = 33.28, label = "f293_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 8.27 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 11.74 end
  acc = cb(bag[1])
  for k = 1, 4 do acc = math.min(acc + k * 42.75, bag[k] * 2) end
  local cb = function(v) return v * acc + 37.89 end
  acc = cb(bag[3])
  if acc % 7 > 3 then acc = acc * 11.20 / 17 + bag[1] else acc = acc - 11.20 end
  local tmp = { x = acc, y = 32.66, label = "f293_l12" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 47.43, label = "f293_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + measure293(1, 2, 293)
end

do
local function spread294(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 31.49, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 30.39 / 17 + bag[1] else acc = acc - 30.39 end
  local cb = function(v) return v * acc + 34.17 end
  acc = cb(bag[3])
  while acc > 37.24 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 1.84 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 11.58 / 17 + bag[1] else acc = acc - 11.58 end
  for k = 1, 4 do acc = math.min(acc + k * 24.11, bag[k] * 2) end
  local tmp = { x = acc, y = 32.94, label = "f294_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 45.63 end
  acc = cb(bag[1])
  local tmp = { x = acc, y = 31.96, label = "f294_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 43.40 / 17 + bag[1] else acc = acc - 43.40 end
  for k = 1, 4 do acc = math.min(acc + k * 33.86, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 22.54, bag[k] * 2) end
  local cb = function(v) return v * acc + 12.94 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + spread294(1, 2, 294)
end

do
local function project295(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  for k = 1, 4 do acc = math.min(acc + k * 13.88, bag[k] * 2) end
  local tmp = { x = acc, y = 48.09, label = "f295_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 13.26 / 17 + bag[1] else acc = acc - 13.26 end
  for k = 1, 4 do acc = math.min(acc + k * 7.63, bag[k] * 2) end
  local cb = function(v) return v * acc + 24.48 end
  acc = cb(bag[1])
  if acc % 7 > 3 then acc = acc * 17.21 / 17 + bag[1] else acc = acc - 17.21 end
  if acc % 7 > 3 then acc = acc * 34.89 / 17 + bag[1] else acc = acc - 34.89 end
  local cb = function(v) return v * acc + 42.38 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 12.90, bag[k] * 2) end
  local tmp = { x = acc, y = 33.04, label = "f295_l9" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 31.66, label = "f295_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 0.17 / 17 + bag[1] else acc = acc - 0.17 end
  for k = 1, 4 do acc = math.min(acc + k * 3.06, bag[k] * 2) end
  while acc > 0.33 * 40 do acc = acc / 2 end
  return acc % 9973
end
acc = acc + project295(1, 2, 295)
end

do
local function clampv296(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  if acc % 7 > 3 then acc = acc * 42.19 / 17 + bag[1] else acc = acc - 42.19 end
  for k = 1, 4 do acc = math.min(acc + k * 5.98, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 9.29, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 20.51 / 17 + bag[1] else acc = acc - 20.51 end
  for k = 1, 4 do acc = math.min(acc + k * 22.84, bag[k] * 2) end
  local cb = function(v) return v * acc + 12.40 end
  acc = cb(bag[2])
  for k = 1, 4 do acc = math.min(acc + k * 9.79, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 35.09, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 28.57, bag[k] * 2) end
  local cb = function(v) return v * acc + 20.60 end
  acc = cb(bag[2])
  while acc > 43.82 * 40 do acc = acc / 2 end
  if acc % 7 > 3 then acc = acc * 39.98 / 17 + bag[1] else acc = acc - 39.98 end
  if acc % 7 > 3 then acc = acc * 22.86 / 17 + bag[1] else acc = acc - 22.86 end
  local tmp = { x = acc, y = 18.97, label = "f296_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + clampv296(1, 2, 296)
end

do
local function spread297(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 30.85 * 40 do acc = acc / 2 end
  local cb = function(v) return v * acc + 45.29 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 44.50 / 17 + bag[1] else acc = acc - 44.50 end
  local cb = function(v) return v * acc + 14.60 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 2.32, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 2.99 / 17 + bag[1] else acc = acc - 2.99 end
  local tmp = { x = acc, y = 49.99, label = "f297_l6" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 2.70, label = "f297_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  if acc % 7 > 3 then acc = acc * 24.33 / 17 + bag[1] else acc = acc - 24.33 end
  if acc % 7 > 3 then acc = acc * 42.08 / 17 + bag[1] else acc = acc - 42.08 end
  for k = 1, 4 do acc = math.min(acc + k * 5.36, bag[k] * 2) end
  local cb = function(v) return v * acc + 44.06 end
  acc = cb(bag[4])
  for k = 1, 4 do acc = math.min(acc + k * 43.24, bag[k] * 2) end
  local tmp = { x = acc, y = 35.01, label = "f297_l13" }
  acc = tmp.x + tmp.y * (seed % 5)
  return acc % 9973
end
acc = acc + spread297(1, 2, 297)
end

do
local function compose298(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  local tmp = { x = acc, y = 23.88, label = "f298_l0" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 17.99, label = "f298_l1" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 25.16, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 35.37 / 17 + bag[1] else acc = acc - 35.37 end
  for k = 1, 4 do acc = math.min(acc + k * 6.35, bag[k] * 2) end
  local tmp = { x = acc, y = 38.64, label = "f298_l5" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 24.45 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 12.42, label = "f298_l7" }
  acc = tmp.x + tmp.y * (seed % 5)
  local cb = function(v) return v * acc + 25.46 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 18.65 end
  acc = cb(bag[2])
  if acc % 7 > 3 then acc = acc * 22.76 / 17 + bag[1] else acc = acc - 22.76 end
  for k = 1, 4 do acc = math.min(acc + k * 13.31, bag[k] * 2) end
  local cb = function(v) return v * acc + 29.03 end
  acc = cb(bag[1])
  local cb = function(v) return v * acc + 15.63 end
  acc = cb(bag[2])
  return acc % 9973
end
acc = acc + compose298(1, 2, 298)
end

do
local function fold299(a: number, b: number, seed: number): number
  local acc = a + b * seed
  local bag = { a, b, seed, acc }
  while acc > 18.31 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 21.28, bag[k] * 2) end
  if acc % 7 > 3 then acc = acc * 4.76 / 17 + bag[1] else acc = acc - 4.76 end
  local cb = function(v) return v * acc + 18.50 end
  acc = cb(bag[4])
  local tmp = { x = acc, y = 32.42, label = "f299_l4" }
  acc = tmp.x + tmp.y * (seed % 5)
  while acc > 10.33 * 40 do acc = acc / 2 end
  for k = 1, 4 do acc = math.min(acc + k * 18.31, bag[k] * 2) end
  local cb = function(v) return v * acc + 16.89 end
  acc = cb(bag[4])
  local cb = function(v) return v * acc + 16.61 end
  acc = cb(bag[1])
  while acc > 17.35 * 40 do acc = acc / 2 end
  local tmp = { x = acc, y = 42.35, label = "f299_l10" }
  acc = tmp.x + tmp.y * (seed % 5)
  local tmp = { x = acc, y = 35.03, label = "f299_l11" }
  acc = tmp.x + tmp.y * (seed % 5)
  for k = 1, 4 do acc = math.min(acc + k * 0.03, bag[k] * 2) end
  for k = 1, 4 do acc = math.min(acc + k * 5.82, bag[k] * 2) end
  return acc % 9973
end
acc = acc + fold299(1, 2, 299)
end
