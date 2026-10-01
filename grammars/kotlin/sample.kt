package dev.example

import kotlin.math.sqrt

/** A sample. */
data class Point(val x: Double, val y: Double = 0.0) {
    val length: Double get() = sqrt(x * x + y * y)
}

sealed interface Shape {
    object Empty : Shape
    class Circle(val radius: Double) : Shape
}

enum class Colour { RED, GREEN }

fun <T : Comparable<T>> largest(items: List<T>): T? {
    var best: T? = null
    for (item in items) {
        if (best == null || item > best) best = item
    }
    return best
}

fun describe(shape: Shape): String = when (shape) {
    is Shape.Empty -> "nothing"
    is Shape.Circle -> "a circle of ${shape.radius}"
}

fun main() {
    val points = listOf(Point(3.0, 4.0), Point(1.0))
    // A comment.
    points.filter { it.length > 1 }.forEach { println("$it: ${it.length}") }
    val name: String? = null
    println(name?.length ?: 0)
    try {
        println(describe(Shape.Circle(2.0)) + largest(listOf(1, 2, 3)))
    } catch (e: IllegalStateException) {
        throw RuntimeException(e)
    }
}
