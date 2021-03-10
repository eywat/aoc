import java.io.File
import kotlin.io.path.ExperimentalPathApi
import kotlin.io.path.Path
import kotlin.reflect.typeOf

fun readFile(fileName: String): String =  File(fileName).readText()

@ExperimentalPathApi
fun dayFileName(day: Int): File {
    return Path("")
        .toAbsolutePath()
        .parent
        .resolve("input")
        .resolve("day%02d.txt".format(day))
        .toFile()
}