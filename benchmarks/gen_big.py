#!/usr/bin/env python3
"""Generate a large Luau source file (big.luau) for the compilation-speed
benchmark. Functions are global (not file-scope locals) to stay under Luau's
200-local-register limit. Run, then:

    ulua-compile null big.luau     # this repo's compiler
    luau-compile   null big.luau    # reference C++ Luau compiler
"""
import os
import random
random.seed(7)
out, N = [], 5000
for f in range(N):
    out.append(f"function f{f}(a, b, c)")
    out.append(f"  local t = {{x = a + b, y = b * c, z = a - c, name = 'fn{f}'}}")
    out.append( "  local s = 0")
    out.append( "  for i = 1, 24 do")
    out.append( "    if i % 2 == 0 then")
    out.append( "      s = s + t.x * i - t.z")
    out.append( "    elseif i % 3 == 0 then")
    out.append( "      s = s - t.y + i * 2")
    out.append( "    else")
    out.append( "      s = s + (a * b - c) / (i + 1)")
    out.append( "    end")
    out.append( "  end")
    out.append(f"  local str = string.format('%d:%d:%s', s, {f}, t.name)")
    out.append( "  local arr = {s, t.x, t.y, t.z, #str}")
    out.append( "  table.sort(arr)")
    out.append( "  return s, str, arr")
    out.append( "end")
out.append("local acc = 0")
for f in range(0, N, 7):
    out.append(f"acc = acc + (f{f}(acc + {f}, {f} * 2, {f} - 1))")
out.append("print(acc)")
dst = os.path.join(os.path.dirname(os.path.abspath(__file__)), "big.luau")
open(dst, "w").write("\n".join(out) + "\n")
print("generated", dst, "-", os.path.getsize(dst), "bytes,", len(out), "lines")
