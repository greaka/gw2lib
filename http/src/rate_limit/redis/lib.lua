#!lua name=gw2lib

--- returns amount of tickets available based on time
local function available_slots(key, amount, burst, refill, update)
    local ratio = math.floor(60000 / refill)
    local max = math.floor(60000 * burst / refill)

    -- returns unix timestamp + microseconds in that second
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

    -- if update is set, timer will be updated based on amount tickets to deduct
    if update then
        value = value + ratio * amount
        redis.call('SET', key, value)
    end

    return math.floor(-(value - ms) / ratio)
end

--- notifies up to amount waiting subscribers based on both time and available permits
local function claim(bucket, semaphore, pub, waitlist, burst, refill)
    local permits = redis.call('GET', semaphore)
    -- (not 0) == false
    if not permits then
        permits = burst
    end
    local ratelimit = available_slots(bucket, 0, burst, refill, false)
    local available = math.min(permits, ratelimit)

    local ids = redis.call('LPOP', waitlist, available)
    if not ids then
        return
    end

    local sum = 0
    for i in ids do
        redis.call('PUBLISH', pub, i)
        sum = sum + 1
    end

    permits = permits - sum

    redis.call('SET', semaphore, permits)

    return sum
end

local function take(keys, args)
    local bucket = keys[1]
    local semaphore = keys[2]
    local pub = keys[3]
    local waitlist = keys[4]
    local amount = args[1]
    local burst = args[2]
    local refill = args[3]
    local pub_id = args[4]

    if amount > burst then
        return redis.error_reply('bucket exhausted')
    end

    local waiters = 0
    for _ = 1, amount do
        waiters = redis.call('RPUSH', waitlist, pub_id)
    end

    local claimed = claim(bucket, semaphore, pub, waitlist, burst, refill)

    waiters = waiters - claimed
    local ratio = math.floor(60000 / refill)

    -- eta in ms
    return waiters * ratio
end

local function poke(keys, args)
    local bucket = keys[1]
    local semaphore = keys[2]
    local pub = keys[3]
    local waitlist = keys[4]
    local burst = args[1]
    local refill = args[2]

    claim(bucket, semaphore, pub, waitlist, burst, refill)
end

local function release(keys, args)
    local bucket = keys[1]
    local semaphore = keys[2]
    local pub = keys[3]
    local waitlist = keys[4]
    local amount = args[1]
    local burst = args[2]
    local refill = args[3]

    local permits = redis.call('GET', semaphore)
    redis.call('SET', semaphore, permits + amount)
    available_slots(bucket, amount, burst, refill, true)

    claim(bucket, semaphore, pub, waitlist, burst, refill)
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

    redis.call('SET', key, value)
end

redis.register_function('ratelimit_take', take)
redis.register_function('ratelimit_poke', poke)
redis.register_function('ratelimit_release', release)
redis.register_function('ratelimit_penalize', penalize)
