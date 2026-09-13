# Mandelbrot escape-iteration sum over an 800x800 grid. Float loops + branches.
def mandel(w, h, maxiter):
    total = 0
    for py in range(h):
        y0 = (py / h) * 2.0 - 1.0
        for px in range(w):
            x0 = (px / w) * 3.0 - 2.0
            x, y, it = 0.0, 0.0, 0
            while it < maxiter:
                x2 = x * x
                y2 = y * y
                if x2 + y2 > 4.0:
                    break
                y = 2.0 * x * y + y0
                x = x2 - y2 + x0
                it += 1
            total += it
    return total


print("%d" % mandel(800, 800, 256))
