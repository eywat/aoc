open System.IO
open System.Text.RegularExpressions

let lines = File.ReadLines

let parse str =
    let m =
        Regex.Match(str, "(?<first>\d+)-(?<second>\d+)\s(?<char>\w):\s(?<pw>\w+)")

    if m.Success then Some(m.Groups) else None

let isValidCount (rule: GroupCollection) =
    let min = int rule.["first"].Value
    let max = int rule.["second"].Value
    let ltr = char rule.["char"].Value
    rule.["pw"].Value
    |> String.filter (fun c -> c = ltr)
    |> String.length
    |> fun l -> min <= l && l <= max

let isValidPosition (rule: GroupCollection) =
    let pos1 = int rule.["first"].Value - 1
    let pos2 = int rule.["second"].Value - 1
    let ltr = char rule.["char"].Value
    rule.["pw"].Value
    |> fun pw ->
        (pw.[pos1] = ltr && pw.[pos2] <> ltr)
        || (pw.[pos1] <> ltr && pw.[pos2] = ltr)


let data =
    "input/day2.txt"
    |> lines
    |> Seq.map (parse)
    |> Seq.filter (Option.isSome)
    |> Seq.map (Option.get)
    |> Seq.toList

data
|> Seq.filter (isValidCount)
|> Seq.length
|> printfn "Solution 1: %d"

data
|> Seq.filter (isValidPosition)
|> Seq.length
|> printfn "Solution 2: %d"
