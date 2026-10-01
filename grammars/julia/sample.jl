module Sample

export area, largest

using LinearAlgebra: norm

const LIMIT = 10

# A sample.
abstract type Shape end

struct Circle <: Shape
    radius::Float64
end

struct Rect <: Shape
    width::Float64
    height::Float64
end

mutable struct Counter
    count::Int
end

area(c::Circle) = pi * c.radius^2
area(r::Rect) = r.width > 0 ? r.width * r.height : 0.0

function largest(items::Vector{T}) where {T<:Real}
    isempty(items) && return nothing
    best = items[1]
    for item in items
        if item > best
            best = item
        end
    end
    return best
end

macro twice(ex)
    quote
        $(esc(ex))
        $(esc(ex))
    end
end

function main()
    shapes = Shape[Circle(1.0), Rect(2.0, 3.0)]
    areas = [area(s) for s in shapes if area(s) < LIMIT]
    counter = Counter(0)
    @twice counter.count += 1
    try
        n = parse(Int, "42")
        println("$(largest(areas)) $(n + counter.count) $(norm([3.0, 4.0]))")
    catch e
        e isa ArgumentError || rethrow()
        @warn "not a number" exception = e
    end
    d = Dict(:a => 1, :b => 2)
    foreach(((k, v),) -> println(k, " => ", v), d)
end

end # module
