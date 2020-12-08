open System.IO
open FSharp.Core
open FSharp.Collections

type Instruction = 
    | ACC of int 
    | JMP of int 
    | NOP of int


let parse (instr: string) = 
    match instr.Split(' ', 2) with
    | [|"acc"; value|] -> ACC(int value)
    | [|"jmp"; value|] -> JMP(int value)
    | [|"nop"; value|] -> NOP(int value)
    | _ -> failwith "Unallowed instruction"


let rec run (instructions: Instruction []) (executedInstructions: Set<int>) (instrPtr: int) (acc: int): Result<int, int> = 
    if Set.contains instrPtr executedInstructions then 
        Error acc
    else 
        let nextInstrPtr, acc = 
            match Array.tryItem instrPtr instructions with
            | Some(ACC value) -> instrPtr+1, acc + value
            | Some(JMP value) -> instrPtr+value, acc
            | Some(NOP _) -> instrPtr+1, acc
            | None -> failwith "Invalid Program"
        if instrPtr = instructions.Length - 1 && nextInstrPtr = instructions.Length then
            Ok acc
        else 
            run instructions (Set.add instrPtr executedInstructions) nextInstrPtr acc


let rec fix (instructions: Instruction []) (executedInstructions: Set<int>) (instrPtr: int) (acc: int): Result<int, int> = 
    if Set.contains instrPtr executedInstructions then 
        executedInstructions
        |> Seq.filter(fun instrPtr -> 
            match instructions.[instrPtr] with 
            | JMP _ | NOP _ -> true
            | _ -> false)
        |> Seq.map(fun instrPtr -> 
            match instructions.[instrPtr] with 
            | JMP value -> instrPtr, NOP value
            | NOP value -> instrPtr, JMP value)
        |> Seq.pick(fun (instrPtr, newInstr) -> 
            let instructions = Array.copy instructions // This is horribly inefficient
            ignore(Array.set instructions instrPtr newInstr)
            match fix instructions (Set.empty) 0 0 with 
            | Ok acc -> Some (Ok acc)
            | Error _-> None)

    else 
        let nextInstrPtr, acc = 
            match Array.tryItem instrPtr instructions with
            | Some(ACC value) -> instrPtr+1, acc + value
            | Some(JMP value) -> instrPtr+value, acc
            | Some(NOP _) -> instrPtr+1, acc
            | None -> failwith "Invalid Program"
        if instrPtr = instructions.Length - 1 && nextInstrPtr = instructions.Length then
            Ok acc
        else 
            fix instructions (Set.add instrPtr executedInstructions) nextInstrPtr acc
 

let data = 
    "../input/day8.txt"
    |> File.ReadAllLines
    |> Array.map parse


run data Set.empty 0 0 |> printfn "Solution 1: %A"