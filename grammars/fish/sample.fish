#!/usr/bin/env fish
# A sample.

set -g limit 10
set -l names one two three

function area --description 'The area of a circle' --argument-names radius
    math "3.14159 * $radius * $radius"
end

function largest
    set -l best $argv[1]
    for item in $argv
        if test $item -gt $best
            set best $item
        end
    end
    echo $best
end

for name in $names
    if string match -q 't*' -- $name
        echo "$name starts with t"
    else if test (string length $name) -gt 3
        echo "$name is long"
    else
        continue
    end
end

switch (uname)
    case Darwin
        set -x PATH /opt/homebrew/bin $PATH
    case '*'
        echo other
end

set -l total (count $names)
while test $total -gt 0
    set total (math $total - 1)
end

echo (largest 1 2 3) (area 2) | tee /dev/null
command -q git; and git status 2>/dev/null; or echo "no git"
