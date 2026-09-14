# N-body (Computer Language Benchmarks Game), portable Python. Float-heavy.
# Faithful port of nbody.lua; bodies are [x,y,z, vx,vy,vz, mass] (0-based here
# vs 1-based in Lua), same arithmetic order, so the %.9f energy checksum matches.
from math import sqrt

PI = 3.141592653589793
SOLAR_MASS = 4 * PI * PI
DPY = 365.24
b = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, SOLAR_MASS],
    [4.84143144246472090e+00, -1.16032004402742839e+00, -1.03622044471123109e-01,
     1.66007664274403694e-03 * DPY, 7.69901118419740425e-03 * DPY, -6.90460016972063023e-05 * DPY,
     9.54791938424326609e-04 * SOLAR_MASS],
    [8.34336671824457987e+00, 4.12479856412430479e+00, -4.03523417114321381e-01,
     -2.76742510726862411e-03 * DPY, 4.99852801234917238e-03 * DPY, 2.30417297573763929e-05 * DPY,
     2.85885980666130812e-04 * SOLAR_MASS],
    [1.28943695621391310e+01, -1.51111514016986312e+01, -2.23307578892655734e-01,
     2.96460137564761618e-03 * DPY, 2.37847173959480950e-03 * DPY, -2.96589568540237556e-05 * DPY,
     4.36624404335156298e-05 * SOLAR_MASS],
    [1.53796971148509165e+01, -2.59193146099879641e+01, 1.79258772950371181e-01,
     2.68067772490389322e-03 * DPY, 1.62824170038242295e-03 * DPY, -9.51592254519715870e-05 * DPY,
     5.15138902046765740e-05 * SOLAR_MASS],
]
n = len(b)


def advance(dt):
    for i in range(n):
        bi = b[i]
        bix, biy, biz, bimass = bi[0], bi[1], bi[2], bi[6]
        bivx, bivy, bivz = bi[3], bi[4], bi[5]
        for j in range(i + 1, n):
            bj = b[j]
            dx, dy, dz = bix - bj[0], biy - bj[1], biz - bj[2]
            dist2 = dx * dx + dy * dy + dz * dz
            mag = dt / (dist2 * sqrt(dist2))
            bjmass = bj[6]
            bivx -= dx * bjmass * mag
            bivy -= dy * bjmass * mag
            bivz -= dz * bjmass * mag
            bj[3] += dx * bimass * mag
            bj[4] += dy * bimass * mag
            bj[5] += dz * bimass * mag
        bi[3], bi[4], bi[5] = bivx, bivy, bivz
    for i in range(n):
        bi = b[i]
        bi[0] += dt * bi[3]
        bi[1] += dt * bi[4]
        bi[2] += dt * bi[5]


def energy():
    e = 0.0
    for i in range(n):
        bi = b[i]
        e += 0.5 * bi[6] * (bi[3] * bi[3] + bi[4] * bi[4] + bi[5] * bi[5])
        for j in range(i + 1, n):
            bj = b[j]
            dx, dy, dz = bi[0] - bj[0], bi[1] - bj[1], bi[2] - bj[2]
            e -= (bi[6] * bj[6]) / sqrt(dx * dx + dy * dy + dz * dz)
    return e


def offset():
    px, py, pz = 0.0, 0.0, 0.0
    for i in range(n):
        bi = b[i]
        px += bi[3] * bi[6]
        py += bi[4] * bi[6]
        pz += bi[5] * bi[6]
    b[0][3], b[0][4], b[0][5] = -px / SOLAR_MASS, -py / SOLAR_MASS, -pz / SOLAR_MASS


offset()
for _ in range(500000):
    advance(0.01)
print("%.9f" % energy())
