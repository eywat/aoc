package day02

import Solver

data class PassWordRule(val min: Int, val max: Int, val letter: Char, val password: String)

object DayTwoSolver : Solver<List<PassWordRule>, Int> {
    override fun parse(input: String): List<PassWordRule> {
        val passwordRegex = Regex("""^(\d+)-(\d+)\s(\w):\s(\w+)$""")
        return input
            .lineSequence()
            .map { passwordRegex.matchEntire(it) }
            .map {
                if (it == null) {
                    throw NullPointerException()
                }
                with(it.destructured) {
                    PassWordRule(component1().toInt(), component2().toInt(), component3()[0], component4())
                }
            }
            .toList()
    }

    override fun solvePartOne(input: List<PassWordRule>): Int =
        input.filter { pr: PassWordRule -> pr.password.count { it == pr.letter } in pr.min..pr.max }.count()

    override fun solvePartTwo(input: List<PassWordRule>): Int =
        input.filter { pr: PassWordRule -> (pr.password[pr.min - 1] == pr.letter).xor(pr.password[pr.max - 1] == pr.letter) }
            .count()
}
