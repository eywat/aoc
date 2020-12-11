open System.IO

let seatID ticket =
    ticket
    |> Seq.fold (fun acc number ->
        match number with
        | 'B'
        | 'R' -> (acc <<< 1) ||| 1
        | _ -> acc <<< 1) 0

let data =
    "../input/day05.txt"
    |> File.ReadAllLines
    |> Array.map seatID

data 
|> Seq.max 
|> printfn "Solution 1: %d"

data 
|> Seq.sort 
|> Seq.windowed 2
|> Seq.find (fun window -> window.[1] - window.[0] = 2)
|> Seq.head 
|> fun seat -> seat + 1
|> printfn "Solution 2: %d"