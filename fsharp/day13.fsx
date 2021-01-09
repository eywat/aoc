open System.IO
open System.Diagnostics

let parse (input: string []) =
    let earliest = int64 input.[0]

    let busLines =
        input.[1].Split(',')
        |> Array.map (System.Int64.TryParse)
        |> Array.indexed
        |> Array.filter (fun (i, (b, id)) -> b)
        |> Array.map (fun (i, (_, id)) -> int64 i, id)

    earliest, busLines

let earliestDepature earliest =
    Array.map (fun (_, id) ->
        id,
        id
        * (int64 (ceil ((float earliest) / (float id)))))
    >> Array.minBy (fun (id, nextDeparture) -> nextDeparture)
    >> fun (id, nextDeparture) -> id * (nextDeparture - earliest)

let rec egcd a b =
    if a = int64 0 then
        b, int64 0, int64 1
    else
        let r, m, n = egcd (b % a) a
        r, n - (b / a) * m, m

let inverseModulo x n =
    let r, x, _ = egcd x n
    if r = int64 1 then Some((x % n + n) % n) else None

let chineseRemainder busLines =
    let lines =
        Array.map (fun (a, n) -> (n - a), n) busLines

    let prod =
        Array.fold (fun acc (_, n) -> acc * n) (int64 1) lines

    Array.sumBy (fun (a, n) ->
        let p = prod / n
        a * Option.get (inverseModulo p n) * p) lines
    |> fun x -> x % prod



let mutable start = Stopwatch.StartNew()

let earliest, busLines =
    File.ReadAllLines "../input/day13.txt" |> parse

printfn "Parsing took %fms" start.Elapsed.TotalMilliseconds

start.Reset
start.Start

earliestDepature earliest busLines
|> printfn "Solution 1: %d"

printfn "Solution 1 took %fms" start.Elapsed.TotalMilliseconds

start.Reset
start.Start

chineseRemainder busLines
|> printfn "Solution 2: %d"

printfn "Solution 2 took %fms" start.Elapsed.TotalMilliseconds
