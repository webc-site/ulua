-- Random DNA sequence generation, shootout-fasta style linear congruential PRNG.
-- Float math + string slicing + incremental table build + big concat.
local IM, IA, IC = 139968, 3877, 29573
local seed = 42.0
local nts = "ccgggcttaaccggatgcgcgttggcaagctgacgtttaagcct" -- fixed 43-char nucleotide alphabet
local n = 900000
local parts = {}
local acc = 0
local alen = #nts
for i = 1, n do
  seed = (seed * IA + IC) % IM
  local idx = math.floor((seed / IM) * alen) + 1
  local c = string.sub(nts, idx, idx)
  parts[i] = c
  acc = acc + string.byte(c)
end
local seq = table.concat(parts)
return (acc + #seq) % 1000000007
