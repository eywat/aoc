open System.IO

let lines = File.ReadLines 

let solve1 data = 
    [for x in data do for y in data -> (x, y)] 
    |> Seq.find(fun (x,y) -> x + y= 2020) 
    |> fun (x,y) -> x*y

let solve2 data = 
    [for x in data do for y in data do for z in data-> (x,y,z)]
    |> Seq.find(fun (x,y,z) -> x + y + z = 2020)
    |> fun (x,y,z) -> x*y*z

let data = 
    "../input/day01.txt" 
    |> lines 
    |> Seq.map(int) 
    |> Seq.toList

data
    |> solve1 
    |> printfn "%A"

data
    |> solve2
    |> printfn "%A"