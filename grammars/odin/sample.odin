package sample

import "core:fmt"
import "core:strings"
import m "core:math"

LIMIT :: 10

Colour :: enum {
	Red,
	Green,
	Blue,
}

Point :: struct {
	x, y: f32,
	name: string,
}

Shape :: union {
	Point,
	f32,
}

Flags :: bit_set[Colour]

length :: proc(p: Point) -> f32 {
	return m.sqrt(p.x * p.x + p.y * p.y)
}

parse :: proc(text: string) -> (n: int, ok: bool) {
	for c in text {
		if c < '0' || c > '9' {
			return 0, false
		}
		n = n * 10 + int(c - '0')
	}
	return n, true
}

@(private)
describe :: proc(s: Shape) -> string {
	switch v in s {
	case Point:
		return v.name
	case f32:
		return "a number"
	}
	return "nothing"
}

main :: proc() {
	points := [?]Point{{3, 4, "a"}, {x = 1, name = "b"}}
	// A comment.
	total: f32
	for p, i in points {
		if i >= LIMIT do break
		total += length(p)
	}
	defer fmt.println("done")

	names := make([dynamic]string)
	defer delete(names)
	append(&names, "one", "two")

	if n, ok := parse("42"); ok {
		fmt.printf("%d %f %s\n", n, total, strings.join(names[:], ", "))
	}
	flags := Flags{.Red, .Blue}
	when ODIN_OS == .Windows {
		fmt.println(flags, describe(points[0]))
	}
	ptr := &points[0]
	ptr.x = cast(f32)len(points)
	raw := `a raw
string`
	fmt.println(raw, Colour.Green)
}
