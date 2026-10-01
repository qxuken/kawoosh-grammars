package dev.example;

import java.util.List;
import java.util.Map;

/** A sample. */
public final class Sample<T extends Comparable<T>> implements Runnable {
    private static final int LIMIT = 10;
    private final List<T> items;

    public Sample(List<T> items) {
        this.items = items;
    }

    @Override
    public void run() {
        for (T item : items) {
            if (item == null) {
                continue;
            }
            System.out.println("item: " + item);
        }
        var sum = items.stream().mapToInt(Object::hashCode).sum();
        String text = switch (sum % 3) {
            case 0 -> "none";
            case 1 -> "one";
            default -> "many";
        };
        try {
            Map.of("k", text).forEach((k, v) -> System.out.printf("%s=%s%n", k, v));
        } catch (RuntimeException e) {
            throw new IllegalStateException(e);
        }
    }

    record Pair(int left, int right) {}

    enum Colour { RED, GREEN }
}
