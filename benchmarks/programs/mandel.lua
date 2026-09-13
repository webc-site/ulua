-- Mandelbrot escape-iteration sum over an 800x800 grid. Float loops + branches.
local function mandel(w, h, maxiter)
  local sum = 0
  for py = 0, h - 1 do
    local y0 = (py / h) * 2.0 - 1.0
    for px = 0, w - 1 do
      local x0 = (px / w) * 3.0 - 2.0
      local x, y, iter = 0.0, 0.0, 0
      while iter < maxiter do
        local x2 = x * x
        local y2 = y * y
        if x2 + y2 > 4.0 then break end
        y = 2.0 * x * y + y0
        x = x2 - y2 + x0
        iter = iter + 1
      end
      sum = sum + iter
    end
  end
  return sum
end
print(string.format("%d", mandel(800, 800, 256)))
