math.randomseed(os.time())
generateProbeId = function(length)
	local res = ""
	for i = 1, length do
		res = res .. string.char(math.random(97, 122))
	end
	return res
end
generateEventId = function()
  local template = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'
  return string.gsub(template, '[xy]', function (c)
      local v = (c == 'x') and math.random(0, 0xf) or math.random(8, 0xb)
      return string.format('%x', v)
  end)
end
probeCount = 450000
onceEvery = 10
probeIds = {}
rps = probeCount / onceEvery
for i=1,probeCount do
  probeIds[i] = generateProbeId(math.random(3, 100))
end
request = function()
  eventTime = os.time(os.date("!*t"))
  second = eventTime % onceEvery
  probeId = probeIds[(second * rps) + math.random(1, rps)]
  eventId = generateEventId()
  method = "POST"
  path = "/probe/" .. probeId .. "/event/" .. eventId
  wrk.body = "{\"probeId\":\"" .. probeId .. "\",\"eventId\":\"" .. eventId .. "\",\"messageType\":\"spaceCartography\",\"eventReceivedTime\":1640780076046,\"eventTransmissionTime\":" .. eventTime .. ",\"messageData\":[{\"type\":\"Measure\",\"measureName\":\"Spherical coordinate system - euclidean distance\",\"measureCode\":\"SCSED\",\"measureUnit\":\"parsecs\",\"measureValue\":539900000.0,\"measureValueDescription\":\"Euclidean distance from earth\",\"measureType\":\"Positioning\",\"componentReading\":4.3e24},{\"type\":\"Measure\",\"measureName\":\"Spherical coordinate system - azimuth angle\",\"measureCode\":\"SCSEAA\",\"measureUnit\":\"degrees\",\"measureValue\":170.42,\"measureValueDescription\":\"Azimuth angle from earth\",\"measureType\":\"Positioning\",\"componentReading\":4600.0},{\"type\":\"Measure\",\"measureName\":\"Spherical coordinate system - polar angle\",\"measureCode\":\"SCSEPA\",\"measureUnit\":\"degrees\",\"measureValue\":30.23,\"measureValueDescription\":\"Polar/Inclination angle from earth\",\"measureType\":\"Positioning\",\"componentReading\":5.6e43},{\"type\":\"Measure\",\"measureName\":\"Localized electromagnetic frequency reading\",\"measureCode\":\"LER\",\"measureUnit\":\"hz\",\"measureValue\":300000.0,\"measureValueDescription\":\"Electromagnetic frequency reading\",\"measureType\":\"Composition\",\"componentReading\":3000000000000000.0},{\"type\":\"Measure\",\"measureName\":\"Probe lifespan estimate\",\"measureCode\":\"PLSE\",\"measureUnit\":\"Years\",\"measureValue\":239000.0,\"measureValueDescription\":\"Number of years left in probe lifespan\",\"measureType\":\"Probe\",\"componentReading\":6524000.0}]}"
  wrk.headers["Content-Type"] = "application/json"
  return wrk.format(method, path)
end
