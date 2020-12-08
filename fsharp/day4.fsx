open System.IO
open FSharp.Collections

let parse (data: string) =
    data.Split([| '\n'; ' ' |], System.StringSplitOptions.RemoveEmptyEntries)
    |> Array.map (fun s -> s.Split(':', 2))
    |> Array.fold (fun map kv -> Map.add kv.[0] kv.[1] map) Map.empty

let contains (required: seq<string>) (map: Map<string, string>): bool =
    required
    |> Seq.forall (fun k -> Map.containsKey k map)

let validByr (byr: string) =
    let yr = int byr
    String.length byr = 4 && 1920 <= yr && yr <= 2002

let validIyr (iyr: string) =
    let yr = int iyr
    String.length iyr = 4 && 2010 <= yr && yr <= 2020

let validEyr (eyr: string) =
    let yr = int eyr
    String.length eyr = 4 && 2020 <= yr && yr <= 2030

let validHgt (hgt: string) =
    if hgt.EndsWith "cm" then
        let ht = int (hgt.Replace("cm", ""))
        150 <= ht && ht <= 193
    elif hgt.EndsWith "in" then
        let ht = int (hgt.Replace("in", ""))
        59 <= ht && ht <= 76
    else
        false

let validHcl (hcl: string) =
    if String.length hcl = 7 && hcl.StartsWith '#' then
        hcl.[1..]
        |> String.forall ("0123456789abcdef".Contains)
    else
        false

let validEcl ecl =
    match ecl with
    | "amb"
    | "blu"
    | "brn"
    | "gry"
    | "grn"
    | "hzl"
    | "oth" -> true
    | _ -> false

let validPid (pid: string) =
    String.length pid = 9
    && String.forall System.Char.IsDigit pid

let valid key value =
    value
    |> match key with
       | "byr" -> validByr
       | "iyr" -> validIyr
       | "eyr" -> validEyr
       | "hgt" -> validHgt
       | "hcl" -> validHcl
       | "ecl" -> validEcl
       | "pid" -> validPid
       | _ -> fun _ -> true

let required =
    [| "byr"
       "iyr"
       "eyr"
       "hgt"
       "hcl"
       "ecl"
       "pid" |]

let data =
    "../input/day4.txt"
    |> File.ReadAllText
    |> fun s -> s.Split("\n\n", System.StringSplitOptions.RemoveEmptyEntries)
    |> Array.map (parse)
    |> Array.filter (contains required)

data |> Array.length |> printfn "Solution 1: %d"

data
|> Array.filter (Map.forall valid)
|> Array.length
|> printfn "Solution 2: %d"
