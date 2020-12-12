open System.IO
open System.Diagnostics

let seatID ticket =
    ticket
    |> Seq.fold (fun acc number ->
        match number with
        | 'B'
        | 'R' -> (acc <<< 1) ||| 1
        | _ -> acc <<< 1) 0

let mutable start = Stopwatch.StartNew()
let data =
    "../input/day05.txt"
    |> File.ReadAllLines
    |> Array.map seatID
printfn "Parsing took %fms" start.Elapsed.TotalMilliseconds

start.Reset
start.Start
data 
|> Seq.max 
|> printfn "Solution 1: %d"
printfn "Solution 1 took %fms" start.Elapsed.TotalMilliseconds

start.Reset
start.Start
data 
|> Seq.sort 
|> Seq.windowed 2
|> Seq.find (fun window -> window.[1] - window.[0] = 2)
|> Seq.head 
|> fun seat -> seat + 1
|> printfn "Solution 2: %d"
printfn "Solution 2 took %fms" start.Elapsed.TotalMilliseconds