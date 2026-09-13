-- N-body (Computer Language Benchmarks Game), portable Lua. Float-heavy.
local sqrt = math.sqrt
local PI = 3.141592653589793
local SOLAR_MASS = 4 * PI * PI
local DPY = 365.24
-- body = {x,y,z, vx,vy,vz, mass}
local b = {
  {0,0,0, 0,0,0, SOLAR_MASS},
  {4.84143144246472090e+00,-1.16032004402742839e+00,-1.03622044471123109e-01,
   1.66007664274403694e-03*DPY,7.69901118419740425e-03*DPY,-6.90460016972063023e-05*DPY,
   9.54791938424326609e-04*SOLAR_MASS},
  {8.34336671824457987e+00,4.12479856412430479e+00,-4.03523417114321381e-01,
   -2.76742510726862411e-03*DPY,4.99852801234917238e-03*DPY,2.30417297573763929e-05*DPY,
   2.85885980666130812e-04*SOLAR_MASS},
  {1.28943695621391310e+01,-1.51111514016986312e+01,-2.23307578892655734e-01,
   2.96460137564761618e-03*DPY,2.37847173959480950e-03*DPY,-2.96589568540237556e-05*DPY,
   4.36624404335156298e-05*SOLAR_MASS},
  {1.53796971148509165e+01,-2.59193146099879641e+01,1.79258772950371181e-01,
   2.68067772490389322e-03*DPY,1.62824170038242295e-03*DPY,-9.51592254519715870e-05*DPY,
   5.15138902046765740e-05*SOLAR_MASS},
}
local n = #b
local function advance(dt)
  for i = 1, n do
    local bi = b[i]
    local bix, biy, biz, bimass = bi[1], bi[2], bi[3], bi[7]
    local bivx, bivy, bivz = bi[4], bi[5], bi[6]
    for j = i + 1, n do
      local bj = b[j]
      local dx, dy, dz = bix - bj[1], biy - bj[2], biz - bj[3]
      local dist2 = dx*dx + dy*dy + dz*dz
      local mag = dt / (dist2 * sqrt(dist2))
      local bjmass = bj[7]
      bivx = bivx - dx * bjmass * mag
      bivy = bivy - dy * bjmass * mag
      bivz = bivz - dz * bjmass * mag
      bj[4] = bj[4] + dx * bimass * mag
      bj[5] = bj[5] + dy * bimass * mag
      bj[6] = bj[6] + dz * bimass * mag
    end
    bi[4], bi[5], bi[6] = bivx, bivy, bivz
  end
  for i = 1, n do
    local bi = b[i]
    bi[1] = bi[1] + dt * bi[4]
    bi[2] = bi[2] + dt * bi[5]
    bi[3] = bi[3] + dt * bi[6]
  end
end
local function energy()
  local e = 0.0
  for i = 1, n do
    local bi = b[i]
    e = e + 0.5 * bi[7] * (bi[4]*bi[4] + bi[5]*bi[5] + bi[6]*bi[6])
    for j = i + 1, n do
      local bj = b[j]
      local dx, dy, dz = bi[1]-bj[1], bi[2]-bj[2], bi[3]-bj[3]
      e = e - (bi[7]*bj[7]) / sqrt(dx*dx + dy*dy + dz*dz)
    end
  end
  return e
end
local function offset()
  local px, py, pz = 0.0, 0.0, 0.0
  for i = 1, n do local bi = b[i]; px = px + bi[4]*bi[7]; py = py + bi[5]*bi[7]; pz = pz + bi[6]*bi[7] end
  b[1][4], b[1][5], b[1][6] = -px/SOLAR_MASS, -py/SOLAR_MASS, -pz/SOLAR_MASS
end
offset()
for _ = 1, 500000 do advance(0.01) end
print(string.format("%.9f", energy()))
