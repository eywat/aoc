open System.IO
open FSharp.Collections

let data =
    "../input/day6.txt"
    |> File.ReadAllText
    |> fun s -> s.Split("\n\n")

data
|> Seq.sumBy
    (String.filter (fun s -> s <> '\n')
     >> Set.ofSeq
     >> Seq.length)
|> printfn "Solution 1: %d"

data
|> Seq.sumBy (fun group ->
    group.Split('\n')
    |> (Seq.map (Set.ofSeq)
        >> Set.intersectMany
        >> Seq.length))
|> printfn "Solution 2: %d"
