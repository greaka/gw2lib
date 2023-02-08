#!lua name=gw2lib

local function take(keys, args)
    local amount = args[1]
    local burst = args[2]
    local refill = args[3]
    local key = keys[1]

    if amount > burst then
        return redis.error_reply('bucket exhausted')
    end

    local ratio = math.floor(60000 / refill)
    local max = math.floor(60000 * burst / refill)

    local time = redis.call('TIME')
    local ms = math.ceil(time[1] * 1000 + time[2] / 1000)
    local base = ms - max
    local value = redis.call('GET', key)
    if not value then
        value = 0
    end
    value = tonumber(value)

    if base > value then
        value = base
    end

    value = value + ratio * amount

    redis.call('SET', key, value)

    -- add a buffer of one second
    local res = value - ms + 1000
    if res < 0 then
        res = 0
    end
    return res
end

local function penalize(keys, args)
    local refill = args[1]
    local key = keys[1]

    local ratio = 60000 / refill

    local time = redis.call('TIME')
    local ms = math.ceil(time[1] * 1000 + time[2] / 1000)
    local value = redis.call('GET', key)
    if not value then
        value = 0
    end
    value = tonumber(value)

    if ms > value then
        value = ms
    end

    -- the api penalizes for half a request while above the rate limit
    value = math.floor(value + ratio / 2)

    return redis.call('SET', key, value)
end

redis.register_function('ratelimit_take', take)
redis.register_function('ratelimit_penalize', penalize)
