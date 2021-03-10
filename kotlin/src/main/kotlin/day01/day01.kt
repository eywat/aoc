package day01

import Solver

object DayOneSolver : Solver<HashSet<Int>, Int?> {

    override fun parse(input: String): HashSet<Int> = input.lineSequence().map { it.toInt() }.toHashSet()

    override fun solvePartOne(input: HashSet<Int>): Int? {
        for (first in input) {
            val second = input.find { it == 2020 - first }
            if (second != null) {
                return first * second
            }
        }
        return null
    }

    override fun solvePartTwo(input: HashSet<Int>): Int? {
        for (first in input) {
            for (second in input) {
                val third = input.find { it == 2020 - second - first }
                if (third != null) {
                    return first * second * third
                }
            }
        }
        return null
    }
}

