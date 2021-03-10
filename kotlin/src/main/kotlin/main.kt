import day01.DayOneSolver
import day02.DayTwoSolver
import day03.DayThreeSolver
import kotlinx.cli.*
import java.io.File
import kotlin.io.path.ExperimentalPathApi

@ExperimentalPathApi
fun main(args: Array<String>) {
    val parser = ArgParser("Advent of Code 2020")

    val day by parser.argument(ArgType.Int, fullName = "day", description = "Day to solve")
    val file by parser.option(ArgType.String, shortName = "f", fullName = "file", description = "Input file to read")

    parser.parse(args)

    val fileName = if (file != null) {
        File(file)
    } else {
        dayFileName(day)
    }

    println("Solving day $day")
    when (day) {
        1 -> solve(DayOneSolver, fileName)
        2 -> solve(DayTwoSolver, fileName)
        3 -> solve(DayThreeSolver, fileName)
        else -> println("No solution for this day exists: %02d".format(day))
    }
}