module sample;

import std.stdio;
import std.algorithm : map, filter;
import std.conv : to;

/// A sample.
enum limit = 10;

struct Point
{
    double x = 0;
    double y = 0;

    double length() const
    {
        import std.math : sqrt;
        return sqrt(x * x + y * y);
    }
}

interface Shape
{
    double area();
}

class Circle : Shape
{
    private double radius;

    this(double radius)
    {
        this.radius = radius;
    }

    override double area()
    {
        return 3.14159 * radius * radius;
    }
}

T largest(T)(T[] items)
{
    T best = items[0];
    foreach (item; items)
    {
        if (item > best)
            best = item;
    }
    return best;
}

void main(string[] args)
{
    auto points = [Point(3, 4), Point(1)];
    // A comment.
    auto lengths = points.map!(p => p.length).filter!(l => l > 1);
    Shape shape = new Circle(1.0);
    try
    {
        int n = "42".to!int;
        writefln("%s %s %s", largest([1, 2, 3]), n + limit, shape.area());
    }
    catch (Exception e)
    {
        writeln(e.msg);
    }
    foreach (i, l; lengths.array)
        writeln(i, ": ", l);
}
