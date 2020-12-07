open System.IO
open System.Text.RegularExpressions
open FSharp.Collections

type Bag = Bag of string
type Contents = (int * Bag) list option

let parse rule =
    let m =
        Regex.Match
            (rule,
             "^(?<color>\w+\s\w+) bags contain ((?<contents>\d+ \w+ \w+) bag(s)?(, )?)+|(?<nothing>no other bags)\.$")

    if m.Success then
        let m = m.Groups
        let bag = Bag m.["color"].Value

        let contents =
            if m.["contents"].Success then
                m.["contents"].Captures
                |> Seq.map (fun s ->
                    s.Value.Split(' ', 2)
                    |> fun split -> int split.[0], Bag split.[1])
                |> Seq.toList
                |> Some
            else
                None
        bag, contents
    else
        failwith (sprintf "Could not parse: '%s'" rule)


let rec containsBag  (bag: Bag) (rule: Bag) (rules: Map<Bag, Contents> ) =
    match Map.tryFind rule rules  with
    | Some (Some content) when List.exists (fun (_, b) -> b = bag) content -> true
    | Some (Some content) -> List.exists (fun (_, rule) -> containsBag bag rule rules) content
    | _ -> false


let rec numberOfBags (bag: Bag) (rules: Map<Bag, Contents>): int =
    match Map.tryFind bag rules with
    | Some (Some content) ->
        List.sumBy(fun (count, bag) -> count + count * numberOfBags bag rules) content
    | _ -> 0


let data =
    "input/day7.txt"
    |> File.ReadLines
    |> Seq.map parse
    |> Map.ofSeq

data
|> Map.toSeq
|> Seq.filter (fun (rule, _) -> containsBag (Bag "shiny gold") rule data)
|> Seq.length
|> printfn "Solution 1: %d"

data
|> numberOfBags (Bag "shiny gold")
|> printfn "Solution 2: %d"