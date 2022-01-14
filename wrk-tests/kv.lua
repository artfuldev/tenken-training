payload = function()
  file = io.open("./payload.json", "r")
  return file:read("*a")
end
body = payload()

putRequest = function()
  wrk.headers["Content-Type"] = "application/json"
  probe = math.random(1, 10000000)
  time = math.random(1, 100000)
  wrk.body = string.format(body, time);
  path = string.format("/probe/%s/event/xx", probe)
  return wrk.format("PUT", path)
end

getRequest = function()
  probeId = math.random(1, 10000000)
  path = string.format("/probe/%s/latest", probeId)
  return wrk.format("GET", path)
end

request = function()
  if math.random(0, 10) < 3 then
      return getRequest()
  else
      return putRequest()
  end
end
