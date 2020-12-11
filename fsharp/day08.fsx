open System.IO
open FSharp.Core
open FSharp.Collections

type Instruction =
    | ACC of int
    | JMP of int
    | NOP of int


let parse (instr: string) =
    match instr.Split(' ', 2) with
    | [| "acc"; value |] -> ACC(int value)
    | [| "jmp"; value |] -> JMP(int value)
    | [| "nop"; value |] -> NOP(int value)
    | _ -> failwith "Invalid op"


let rec run (instructions: Instruction [])
            (executedInstructions: Set<int>)
            (instrPtr: int)
            (acc: int)
            : Result<int, int * Set<int>> =
    if Set.contains instrPtr executedInstructions then
        Error(acc, executedInstructions)
    else
        let nextInstrPtr, acc =
            match Array.tryItem instrPtr instructions with
            | Some (ACC value) -> instrPtr + 1, acc + value
            | Some (JMP value) -> instrPtr + value, acc
            | Some (NOP _) -> instrPtr + 1, acc
            | None -> failwith "Invalid Program"

        if instrPtr = instructions.Length
           - 1
           && nextInstrPtr = instructions.Length then
            Ok acc
        else
            run instructions (Set.add instrPtr executedInstructions) nextInstrPtr acc


let fix (instructions: Instruction []) (executedInstructions: Set<int>): int =
    executedInstructions
    |> Seq.filter (fun instrPtr ->
        match instructions.[instrPtr] with
        | JMP _
        | NOP _ -> true
        | _ -> false)
    |> Seq.map (fun instrPtr ->
        match instructions.[instrPtr] with
        | JMP value -> (instrPtr, JMP value, NOP value)
        | NOP value -> (instrPtr, NOP value, JMP value)
        | _ -> failwith "Invalid op")
    |> Seq.pick (fun (instrPtr, oldInstr, newInstr) ->
        ignore (Array.set instructions instrPtr newInstr)
        match run instructions Set.empty 0 0 with
        | Ok acc ->
            ignore (Array.set instructions instrPtr oldInstr)
            Some(acc)
        | Error _ ->
            ignore (Array.set instructions instrPtr oldInstr)
            None)


let data =
    "../input/day08.txt"
    |> File.ReadAllLines
    |> Array.map parse

let _, exec =
    run data Set.empty 0 0
    |> function
        | Error (acc, exec) -> printfn "Solution 1: %d" acc, exec
        | Ok _ -> failwith "Programm exited successfully"

fix data exec |> printfn "Solution 2: %d"
