#!/usr/bin/awk -f
# A sample.

BEGIN {
    FS = ","
    limit = 10
    print "name", "area"
}

function area(radius) {
    return 3.14159 * radius * radius
}

NR == 1 { next }

$2 == "circle" && $3 > 0 {
    total += area($3)
    count[$2]++
    printf "%s %.2f\n", $1, area($3)
}

/^#/ { comments++ }

{
    if (length($0) > limit) {
        long++
    } else {
        short++
    }
    for (i = 1; i <= NF; i++) {
        seen[$i] = 1
    }
}

END {
    for (kind in count) {
        print kind, count[kind]
    }
    print "total", total, long, short, comments
}
