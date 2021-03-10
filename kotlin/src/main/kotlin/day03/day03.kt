package day03

import Solver

object DayThreeSolver : Solver<Array<CharArray>, Long> {
    override fun parse(input: String) = input.lines().map { it.toCharArray() }.toTypedArray()

    override fun solvePartOne(input: Array<CharArray>): Long = solve(input, 3 to 1)

    override fun solvePartTwo(input: Array<CharArray>): Long {
        return listOf(1 to 1, 3 to 1, 5 to 1, 7 to 1, 1 to 2).map { solve(input, it) }
            .reduce { mul, elem -> mul * elem }

    }
}

fun solve(input: Array<CharArray>, slope: Pair<Int, Int>): Long {
    val (right, down) = slope
    var counter = 0L
    var j = 0
    for (i in input.indices.step(down)) {
        val array = input[i]
        if (array[j] == '#') {
            counter++
        }
        j = (j + right) % array.size
    }
    return counter
}