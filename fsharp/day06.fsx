open System.IO
open System.Diagnostics
open FSharp.Collections

let mutable start = Stopwatch.StartNew()
let data =
    "../input/day06.txt"
    |> File.ReadAllText
    |> fun s -> s.Split("\n\n")
printfn "Parsing took %fms" start.Elapsed.TotalMilliseconds

start.Reset
start.Start
data
|> Seq.sumBy
    (String.filter (fun s -> s <> '\n')
     >> Set.ofSeq
     >> Seq.length)
|> printfn "Solution 1: %d"
printfn "Solution 1 took %fms" start.Elapsed.TotalMilliseconds

start.Reset
start.Start
data
|> Seq.sumBy (fun group ->
    group.Split('\n')
    |> (Seq.map (Set.ofSeq)
        >> Set.intersectMany
        >> Seq.length))
|> printfn "Solution 2: %d"
printfn "Solution 2 took %fms" start.Elapsed.TotalMilliseconds