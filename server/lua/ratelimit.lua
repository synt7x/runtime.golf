local limit_req = require("resty.limit.req")

local lim, err = limit_req.new("ratelimit", 2, 5)
if not lim then
	ngx.log(ngx.ERR, "failed to instantiate a limiter: ", err)
	return ngx.exit(500)
end

-- Use IP address as key
local key = ngx.var.binary_remote_addr
local client_ip = ngx.var.remote_addr
local uri = ngx.var.uri
local method = ngx.var.request_method

-- Debug: Log request details
ngx.log(
	ngx.INFO,
	"Rate limiter: Processing request - IP: ",
	client_ip,
	" Method: ",
	method,
	" URI: ",
	uri,
	" Key: ",
	ngx.encode_base64(key)
)

local delay, err = lim:incoming(key, true)

if not delay then
	if err == "rejected" then
		ngx.status = 429
		ngx.header["Retry-After"] = "1"
		ngx.say('{"error": "Rate limit exceeded. Please try again later."}')
		return ngx.exit(429)
	else
		ngx.log(ngx.ERR, "failed to limit request: ", err)
		return ngx.exit(500)
	end
end

if delay > 0 then
	ngx.sleep(delay)
end
