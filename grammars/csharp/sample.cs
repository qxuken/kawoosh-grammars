using System;
using System.Collections.Generic;
using System.Linq;

namespace Example
{
    /// <summary>A sample.</summary>
    public record Point(double X, double Y)
    {
        public double Length => Math.Sqrt(X * X + Y * Y);
    }

    public enum Colour { Red, Green, Blue }

    public interface IShape
    {
        double Area();
    }

    public sealed class Circle : IShape
    {
        private readonly double radius;

        public Circle(double radius)
        {
            this.radius = radius;
        }

        public double Area() => Math.PI * radius * radius;
    }

    public static class Program
    {
        private const int Limit = 10;

        public static T? Largest<T>(IEnumerable<T> items) where T : struct, IComparable<T>
        {
            T? best = null;
            foreach (var item in items)
            {
                if (best is null || item.CompareTo(best.Value) > 0)
                {
                    best = item;
                }
            }
            return best;
        }

        public static void Main(string[] args)
        {
            var points = new List<Point> { new(3, 4), new(1, 0) };
            // A comment.
            var long_ones = points.Where(p => p.Length > 1).Select(p => p.Length).ToList();
            string text = long_ones.Count switch
            {
                0 => "none",
                1 => "one",
                _ => "many",
            };
            try
            {
                Console.WriteLine($"{text}: {Largest(new[] { 1, 2, 3 })} of {Limit}");
            }
            catch (InvalidOperationException e)
            {
                throw new ApplicationException("failed", e);
            }
        }
    }
}
