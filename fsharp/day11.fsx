open System.IO

type Tile =
    | Empty
    | Occupied
    | Ground

let parse (input: string []) =
    input
    |> Array.map
        (Seq.toArray
         >> Array.map (function
             | 'L' -> Empty
             | '#' -> Occupied
             | '.' -> Ground
             | _ -> failwith "Invalid symbol"))
    |> Array.fold Array.append Array.empty

let adjacencyOccupation (grid: Tile []) (dimX, dimY) pos =
    let x = pos / dimY
    let y = pos - x * dimY
    let x0 = max (x - 1) 0
    let y0 = max (y - 1) 0
    let xN = min (x + 1) (dimX - 1)
    let yN = min (y + 1) (dimY - 1)
    seq { x0 .. xN }
    |> Seq.sumBy (fun x ->
        seq { y0 .. yN }
        |> Seq.filter (fun y ->
            dimY
            * x
            + y
            <> pos
            && grid.[dimY * x + y] = Occupied)
        |> Seq.length)

let isOccupied (grid: Tile []) dimY (x, y) =
    match grid.[dimY * x + y] with
    | Occupied -> Some 1
    | Empty -> Some 0
    | Ground -> None

let viewOccupancy (grid: Tile []) (dimX, dimY) pos: int =
    let x = pos / dimY
    let y = pos - x * dimY

    let xminr = [ max (x - 1) 0 .. -1 .. 0 ]
    // ignore(printfn "%A" xminr)
    let yminr = [ max (y - 1) 0 .. -1 .. 0 ]
    let xmaxr = [ min (x + 1) (dimX-1) .. dimX-1 ]
    let ymaxr = [ min (y + 1) (dimY-1) .. dimY-1 ]

    let occupiedDiag =
        fun xl yl ->
            Seq.zip xl yl
            |> Seq.tryPick (isOccupied grid dimY)
            |> Option.defaultValue 0

    let occupiedX =
        fun xl y ->
            List.tryPick (fun x -> isOccupied grid dimY (x, y)) xl
            |> Option.defaultValue 0

    let occupiedY =
        fun yl x ->
            List.tryPick (fun y -> isOccupied grid dimY (x, y)) yl
            |> Option.defaultValue 0

    let diag1 = occupiedDiag xminr yminr
    let diag2 = occupiedDiag xminr ymaxr
    let diag3 = occupiedDiag xmaxr yminr
    let diag4 = occupiedDiag xmaxr ymaxr

    let xmin = occupiedX xminr y
    let xmax = occupiedX xmaxr y
    let ymin = occupiedY yminr x
    let ymax = occupiedY ymaxr x

    diag1
    + diag2
    + diag3
    + diag4
    + xmin
    + xmax
    + ymin
    + ymax

let ruleOne grid dims pos =
    let occupied = adjacencyOccupation grid dims pos
    match grid.[pos] with
    | Empty when occupied = 0 -> Occupied
    | Occupied when occupied >= 4 -> Empty
    | tile -> tile

let ruleTwo grid dims pos =
    let occupied = viewOccupancy grid dims pos
    match grid.[pos] with
    | Empty when occupied = 0 -> Occupied
    | Occupied when occupied >= 5 -> Empty
    | tile -> tile

let rec evolve (grid: Tile []) (dims: int * int) (rule: Tile [] -> int * int -> int -> Tile): Tile [] =
    let next =
        seq { 0 .. grid.Length - 1 }
        |> Seq.map (rule grid dims)
        |> Seq.toArray

    if next = grid then next else evolve next dims rule

let data =
    "../input/day11.txt" |> File.ReadAllLines

let dimX = data.Length
let dimY = data.[0].Length
let grid = parse data
let evolveGrid = evolve grid (dimX, dimY)

evolveGrid ruleOne
|> Seq.filter (fun tile -> tile = Occupied)
|> Seq.length
|> printfn "Solution 1: %d"

evolveGrid ruleTwo
|> Seq.filter (fun tile -> tile = Occupied)
|> Seq.length
|> printfn "Solution 2: %d"
