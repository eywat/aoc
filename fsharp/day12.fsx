open System.IO
open System.Diagnostics

type Instruction =
    | North of int
    | East of int
    | South of int
    | West of int
    | Left of int
    | Right of int
    | Forward of int

type BoatState1 = int * int * int

type BoatState2 = int * int * int * int

let parse (input: string): Instruction =
    let instr = input.[0]
    let n = input.[1..]
    match instr with
    | 'N' -> North(int n)
    | 'E' -> East(int n)
    | 'S' -> South(int n)
    | 'W' -> West(int n)
    | 'L' -> Left((int n) / 90)
    | 'R' -> Right((int n) / 90)
    | 'F' -> Forward(int n)
    | _ -> failwith "Invalid input"

let rotateDir dir rotation = (dir + (rotation + 4)) % 4

let apply1 ((x, y, dir): BoatState1) (instr: Instruction): BoatState1 =
    match instr with
    | North n -> x, y + n, dir
    | East n -> x + n, y, dir
    | South n -> x, y - n, dir
    | West n -> x - n, y, dir
    | Left n -> x, y, rotateDir dir -n
    | Right n -> x, y, rotateDir dir n
    | Forward n ->
        match dir with
        | 0 -> x, y + n, dir
        | 1 -> x + n, y, dir
        | 2 -> x, y - n, dir
        | 3 -> x - n, y, dir
        | _ -> failwith "Unreachable"

let rotateWP wx wy rotation =
    let sin, cos =
        match abs (rotation - 4) % 4 with
        | 0 -> 0, 1
        | 1 -> 1, 0
        | 2 -> 0, -1
        | 3 -> -1, 0
        | _ -> failwith "Unreachable"

    wx * cos - wy * sin, wx * sin + wy * cos

let apply2 ((x, y, wx, wy): BoatState2) (instr: Instruction): BoatState2 =
    match instr with
    | North n -> x, y, wx, wy + n
    | East n -> x, y, wx + n, wy
    | South n -> x, y, wx, wy - n
    | West n -> x, y, wx - n, wy
    | Left n ->
        let wx, wy = rotateWP wx wy -n
        x, y, wx, wy
    | Right n ->
        let wx, wy = rotateWP wx wy n
        x, y, wx, wy
    | Forward n -> x + wx * n, y + wy * n, wx, wy

let mutable start = Stopwatch.StartNew()

let data =
    File.ReadLines "../input/day12.txt"
    |> Seq.toList
    |> List.map parse

printfn "Parsing took %fms" start.Elapsed.TotalMilliseconds

start.Reset
start.Start

data
|> List.fold apply1 (0, 0, 1)
|> fun (x, y, _) -> abs x + abs y
|> printfn "Solution 1: %d"

printfn "Solution 1 took %fms" start.Elapsed.TotalMilliseconds

start.Reset
start.Start

data
|> List.fold apply2 (0, 0, 10, 1)
|> fun (x, y, _, _) -> abs x + abs y
|> printfn "Solution 2: %d"

printfn "Solution 2 took %fms" start.Elapsed.TotalMilliseconds
