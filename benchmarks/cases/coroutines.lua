-- Ping-pong coroutine switching (LuaJIT "cheap concurrency" style).
-- Coroutine create/resume/yield round-trips between two suspended frames.
local rounds = 700000
local count = 0
local pong
local ping = coroutine.create(function()
  while true do
    count = count + 1
    coroutine.yield()
  end
end)
pong = coroutine.create(function()
  while true do
    count = count + 1
    coroutine.yield()
  end
end)
for _ = 1, rounds do
  coroutine.resume(ping)
  coroutine.resume(pong)
end
return count
