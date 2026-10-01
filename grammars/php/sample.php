<?php

declare(strict_types=1);

namespace Example;

use InvalidArgumentException;

interface Shape
{
    public function area(): float;
}

final class Circle implements Shape
{
    private const LIMIT = 10;

    public function __construct(private float $radius)
    {
    }

    public function area(): float
    {
        return M_PI * $this->radius ** 2;
    }
}

function parse(string $text): int
{
    if (!ctype_digit($text)) {
        throw new InvalidArgumentException("not a number: {$text}");
    }
    return (int) $text;
}

// A comment.
$shapes = [new Circle(1.0), new Circle(2.5)];
$areas = array_map(fn (Shape $s): float => $s->area(), $shapes);
foreach ($areas as $index => $area) {
    if ($area > 5) {
        printf("%d: %.2f\n", $index, $area);
    }
}

try {
    echo parse('42') + count($areas), PHP_EOL;
} catch (InvalidArgumentException $e) {
    echo $e->getMessage();
}
