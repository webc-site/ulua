# Build a 200k-element array via a Park-Miller RNG, sort it, checksum.
# The RNG stays under 2^53 so it is exact in every engine, keeping the
# checksum identical to the Lua version.
seed = 42


def rnd():
    global seed
    seed = (seed * 16807) % 2147483647
    return seed


n = 200000
t = [rnd() for _ in range(n)]
t.sort()
total = 0
for i in range(0, n, 997):
    total += t[i]
print("%d" % (total % 1000000007))
