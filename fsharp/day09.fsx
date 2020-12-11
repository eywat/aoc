open System.IO
open FSharp.Collections

let isAncestorSum (ancestor: uint64 []) (target: uint64): bool =
    let map =
        Seq.map (fun num -> num, ()) ancestor |> Map.ofSeq

    Map.exists (fun key _ -> Map.containsKey (target - key) map) map

let invalidNumber (data: uint64 []): uint64 option =
    Seq.windowed (26) data
    |> Seq.tryPick (fun window ->
        if not (isAncestorSum window.[0..25] window.[25])
        then Some window.[25]
        else None)

let invalidSummands (data: uint64 []) (target: uint64): uint64 [] option =
    seq { 2 .. data.Length }
    |> Seq.tryPick (fun i ->
        Seq.windowed (i) data
        |> Seq.tryFind (fun window -> Seq.sum window = target))

let data =
    File.ReadAllLines "../input/day09.txt"
    |> Array.map (uint64)

let invalid =
    match invalidNumber data with
    | Some number ->
        ignore (printfn "Solution 1: %d" number)
        number
    | _ -> failwith "No invalid number found"


match invalidSummands data invalid with
| Some summands -> printfn "Solution 2: %d" ((Seq.min summands) + (Seq.max summands))
| None -> failwith "No summands found"
