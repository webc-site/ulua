# Dense 200x200 matrix multiply. List indexing + multiply-accumulate.
# Matrices are filled with the same 1-based formula as the Lua version so the
# checksum matches exactly.
n = 200
a = [[((i + 1) + (j + 1)) % 100 for j in range(n)] for i in range(n)]
b = [[((i + 1) * 2 + (j + 1)) % 100 for j in range(n)] for i in range(n)]
c = [[0] * n for _ in range(n)]
for i in range(n):
    ai, ci = a[i], c[i]
    for k in range(n):
        aik, bk = ai[k], b[k]
        for j in range(n):
            ci[j] += aik * bk[j]
total = 0
for i in range(n):
    ci = c[i]
    for j in range(n):
        total += ci[j]
print("%d" % (total % 1000000007))
