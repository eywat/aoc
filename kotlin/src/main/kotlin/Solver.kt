import java.io.File

interface Solver<I,O> {
    fun parse(input: String): I
    fun solvePartOne(input: I): O
    fun solvePartTwo(input: I): O
}

fun <I,O> solve(solver: Solver<I,O>, file: File) {
    val content = file.readText()
    val input = solver.parse(content)

    val solutionOne = solver.solvePartOne(input)
    println("Solution for Part One: $solutionOne")

    val solutionTwo = solver.solvePartTwo(input)
    println("Solution for Part Two: $solutionTwo")
}