import gleam/int
import gleam/io
import gleam/list
import gleam/result

/// A sample.
pub type Shape {
  Circle(radius: Float)
  Rect(width: Float, height: Float)
  Empty
}

const limit = 10

pub fn area(shape: Shape) -> Float {
  case shape {
    Circle(r) -> 3.14159 *. r *. r
    Rect(w, h) if w >. 0.0 -> w *. h
    _ -> 0.0
  }
}

pub fn largest(items: List(Int)) -> Result(Int, Nil) {
  list.reduce(items, fn(a, b) { int.max(a, b) })
}

fn describe(shape: Shape) -> String {
  case shape {
    Circle(..) -> "a circle"
    Rect(..) -> "a rect"
    Empty -> "nothing"
  }
}

pub fn main() {
  let shapes = [Circle(1.0), Rect(2.0, 3.0), Empty]
  // A comment.
  shapes
  |> list.map(describe)
  |> list.each(io.println)

  let assert Ok(n) = int.parse("42")
  let total =
    largest([1, 2, 3])
    |> result.unwrap(0)
    |> int.add(n)
  case total > limit {
    True -> io.println("large: " <> int.to_string(total))
    False -> io.println("small")
  }
}
