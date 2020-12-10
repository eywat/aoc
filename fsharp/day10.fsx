open System.IO
open FSharp.Collections

let parse input =
    let list = List.map (int) input |> List.sort
    0
    :: list
    @ [ (List.last list) + 3 ]

let rec joltageDiff (ones, threes) =
    function
    | head :: (next :: _ as tail) when next - head = 3 -> joltageDiff (ones, threes + 1) tail
    | head :: (next :: _ as tail) when next - head = 1 -> joltageDiff (ones + 1, threes) tail
    | head :: [ next ] when next - head = 3 -> ones, threes + 1
    | head :: [ next ] when next - head = 1 -> ones + 1, threes
    | _ :: tail -> joltageDiff (ones, threes) tail
    | _ -> ones, threes


let rec numCombinations (paths: Map<int, uint64>) =
    function 
    | [] -> paths 
    | head::tail ->
        let nPaths = 
            seq { 1 .. 3 }
            |> Seq.filter(fun i -> Map.containsKey (head - i) paths) 
            |> Seq.sumBy(fun i -> Map.find (head - i) paths)
        numCombinations (Map.add head nPaths paths) tail

  
let data =
    File.ReadLines "../input/day10.txt"
    |> Seq.toList
    |> parse


joltageDiff (0, 0) data
|> fun (ones, threes) -> ones * threes
|> printfn "Solution 1: %d"


data 
|> function 
    | head::tail -> 
        numCombinations (Map.add head (uint64 1) Map.empty) tail
        |> Map.find (List.last data)
        |> printfn "Solution 2: %d" 
    | [] -> failwith "Empty list"
