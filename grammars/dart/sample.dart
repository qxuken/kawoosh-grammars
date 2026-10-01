import 'dart:math' as math;

/// A sample.
abstract class Shape {
  double get area;
}

class Circle implements Shape {
  final double radius;
  const Circle(this.radius);

  @override
  double get area => math.pi * radius * radius;
}

class Point {
  final double x, y;
  Point(this.x, [this.y = 0]);

  double get length => math.sqrt(x * x + y * y);
  Point operator +(Point other) => Point(x + other.x, y + other.y);
}

enum Colour { red, green, blue }

const limit = 10;

T? largest<T extends Comparable<T>>(Iterable<T> items) {
  T? best;
  for (final item in items) {
    if (best == null || item.compareTo(best) > 0) best = item;
  }
  return best;
}

Future<int> parse(String text) async {
  final n = int.tryParse(text);
  if (n == null) throw FormatException('not a number: $text');
  return n;
}

void main(List<String> args) async {
  final points = [Point(3, 4), Point(1)];
  // A comment.
  final lengths = points.map((p) => p.length).where((l) => l > 1).toList();
  final text = switch (lengths.length) {
    0 => 'none',
    1 => 'one',
    _ => 'many',
  };
  try {
    final n = await parse('42');
    print('$text: ${largest([1, 2, 3])} ${n + limit} ${Colour.green.name}');
  } on FormatException catch (e) {
    print(e.message);
  }
}
