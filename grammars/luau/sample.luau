--!strict
-- A sample.

type Point = {
	x: number,
	y: number,
}

type Shape = { kind: "circle", radius: number } | { kind: "rect", width: number, height: number }

local LIMIT = 10

local function area(shape: Shape): number
	if shape.kind == "circle" then
		return math.pi * shape.radius ^ 2
	elseif shape.width > 0 then
		return shape.width * shape.height
	end
	return 0
end

local function largest<T>(items: { T }, less: (T, T) -> boolean): T?
	local best: T? = nil
	for _, item in ipairs(items) do
		if best == nil or less(best, item) then
			best = item
		end
	end
	return best
end

local Counter = {}
Counter.__index = Counter

function Counter.new()
	return setmetatable({ count = 0 }, Counter)
end

function Counter:add(n: number)
	self.count += n
end

local shapes: { Shape } = {
	{ kind = "circle", radius = 1 },
	{ kind = "rect", width = 2, height = 3 },
}

local counter = Counter.new()
for i, shape in shapes do
	local a = area(shape)
	if a < LIMIT then
		counter:add(1)
		print(`{i}: {a}`)
	else
		continue
	end
end

print(largest({ 1, 2, 3 }, function(a, b)
	return a < b
end), counter.count)
