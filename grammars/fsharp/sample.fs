module Sample

open System

/// A sample.
type Shape =
    | Circle of radius: float
    | Rect of width: float * height: float
    | Empty

type Point = { X: float; Y: float }

let limit = 10

let area shape =
    match shape with
    | Circle r -> Math.PI * r * r
    | Rect(w, h) when w > 0.0 -> w * h
    | _ -> 0.0

let largest items =
    match items with
    | [] -> None
    | _ -> Some(List.max items)

let length p = sqrt (p.X * p.X + p.Y * p.Y)

type Counter() =
    let mutable count = 0
    member this.Add(n: int) = count <- count + n
    member this.Count = count

[<EntryPoint>]
let main argv =
    let shapes = [ Circle 1.0; Rect(2.0, 3.0); Empty ]
    let counter = Counter()
    // A comment.
    shapes
    |> List.map area
    |> List.filter (fun a -> a < float limit)
    |> List.iter (fun a ->
        counter.Add 1
        printfn "%.2f" a)

    try
        let n = Int32.Parse "42"
        printfn "%A %d %f" (largest [ 1; 2; 3 ]) (n + counter.Count) (length { X = 3.0; Y = 4.0 })
        0
    with :? FormatException as e ->
        eprintfn "%s" e.Message
        1
