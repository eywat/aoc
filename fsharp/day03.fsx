open System.IO

let folder1 (mov, pos, hits) (line: char []) =
    let npos = (pos + mov) % line.Length
    match line.[pos] with
    | '#' -> (mov, npos, hits + 1)
    | _ -> (mov, npos, hits)

let folder2 (data: char [] [], hits: int64) ((mov, down): int * int) =
    let hits =
        data
        |> Seq.indexed
        |> Seq.filter (fun (idx, _) -> idx % down = 0)
        |> Seq.map (fun (_, value) -> value)
        |> Seq.fold folder1 (mov, 0, 0)
        |> fun (_, _, hit) -> hits * int64 (hit)

    data, hits


let data =
    "../input/day3.txt"
    |> File.ReadLines
    |> Seq.map (Seq.toArray)
    |> Seq.toArray

data
|> Array.fold folder1 (3, 0, 0)
|> fun (_, _, hits) -> hits |> printfn "Solution 1: %d"

[| (1, 1)
   (3, 1)
   (5, 1)
   (7, 1)
   (1, 2) |]
|> Array.fold folder2 (data, int64 (1))
|> fun (_, hits) -> hits
|> printfn "Solution 2: %d"
