-- Function to generate a JSON request body with a dynamic value
function generate_request_body()
    local value = math.random(1, 100) -- Change this to generate the desired values
    local body = '{"data":{}, "id":' .. value .. '}'
    return body
end

-- Set the HTTP method and headers
wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

-- Initialize the request counter
counter = 0

-- Request function that updates the request body with a dynamic value
request = function()
    counter = counter + 1
    local body = generate_request_body()
    wrk.body = body
    return wrk.format(nil, nil, nil, body)
end
