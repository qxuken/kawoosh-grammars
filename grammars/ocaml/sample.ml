(* A sample. *)
type shape =
  | Circle of float
  | Rect of float * float
  | Empty

type point = { x : float; y : float }

module Named = struct
  let name = function
    | Circle _ -> "circle"
    | Rect _ -> "rect"
    | Empty -> "empty"
end

let area = function
  | Circle r -> Float.pi *. r *. r
  | Rect (w, h) when w > 0. -> w *. h
  | _ -> 0.

let rec largest = function
  | [] -> None
  | [ x ] -> Some x
  | x :: rest -> (
      match largest rest with
      | Some y when y > x -> Some y
      | _ -> Some x)

let length p = sqrt ((p.x *. p.x) +. (p.y *. p.y))

let () =
  let shapes = [ Circle 1.; Rect (2., 3.); Empty ] in
  List.iter (fun s -> Printf.printf "%s: %f\n" (Named.name s) (area s)) shapes;
  let p = { x = 3.; y = 4. } in
  match largest (List.map area shapes) with
  | Some a -> Printf.printf "largest %f, length %f\n" a (length p)
  | None -> ()
