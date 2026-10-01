package dev.example

import scala.collection.mutable
import scala.util.{Failure, Success, Try}

/** A sample. */
sealed trait Shape
case class Circle(radius: Double) extends Shape
case class Rect(width: Double, height: Double) extends Shape
case object Empty extends Shape

trait Named {
  def name: String
  override def toString: String = s"<$name>"
}

class Point(val x: Double, val y: Double = 0.0) extends Named {
  def name: String = "point"
  def length: Double = math.sqrt(x * x + y * y)
  def +(other: Point): Point = new Point(x + other.x, y + other.y)
}

object Sample {
  private val Limit = 10
  type Pair[A] = (A, A)

  def area(shape: Shape): Double = shape match {
    case Circle(r) => math.Pi * r * r
    case Rect(w, h) if w > 0 => w * h
    case _ => 0.0
  }

  def largest[T: Ordering](items: Seq[T]): Option[T] =
    if (items.isEmpty) None else Some(items.max)

  def main(args: Array[String]): Unit = {
    val points = List(new Point(3, 4), new Point(1))
    // A comment.
    val seen = mutable.Map.empty[String, Int]
    for (p <- points if p.length > 1) {
      seen(p.name) = seen.getOrElse(p.name, 0) + 1
      println(f"${p.length}%.2f")
    }
    val doubled = points.map(p => p + p).filter(_.x < Limit)
    Try("42".toInt) match {
      case Success(n) => println(n + doubled.size)
      case Failure(e) => throw new IllegalStateException(e)
    }
    lazy val text = """a raw
      |string""".stripMargin
    println(text + area(Circle(1.0)) + largest(Seq(1, 2, 3)))
  }
}
