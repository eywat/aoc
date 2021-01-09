open System.IO
open System.Diagnostics
open FSharp.Collections

let solve1 data =
    Seq.pick (fun x -> if Set.contains (2020 - x) data then Some((2020 - x) * x) else None) data


let solve2 (data: Set<int>) =
    Seq.pick (fun x ->
        Seq.tryPick (fun y -> if Set.contains (2020 - x - y) data then Some((2020 - x - y) * x * y) else None) data)
        data

let mutable start = Stopwatch.StartNew()

let data =
    File.ReadLines "../input/day01.txt"
    |> Seq.map int
    |> Set.ofSeq

printfn "Parsing took %fms" start.Elapsed.TotalMilliseconds

start.Reset
start.Start
data |> solve1 |> printfn "Solution 1: %d"
printfn "Solution 1 took %fms" start.Elapsed.TotalMilliseconds

start.Start
start.Reset
data |> solve2 |> printfn "Solution 2: %d"
printfn "Solution 2 took %fms" start.Elapsed.TotalMilliseconds
