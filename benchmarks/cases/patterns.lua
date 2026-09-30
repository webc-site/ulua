-- String pattern matching churn: find / match / gsub / gmatch.
-- Pattern compiler cache + capture walk + backtracking matcher.
local acc = 0
local iters = 18000
for i = 1, iters do
  local s = string.format("user%d@host%d.example.org/path/%d", i, i % 97, i * 3)
  if string.find(s, "@host%d+") then
    acc = acc + 1
  end
  local dom = string.match(s, "@host(%d+)%.example")
  if dom then
    acc = acc + #dom
  end
  local clean, count = string.gsub(s, "%d+", "#")
  acc = acc + count
  for part in string.gmatch(clean, "[^/#]+") do
    acc = acc + #part
  end
end
return acc % 1000000007
