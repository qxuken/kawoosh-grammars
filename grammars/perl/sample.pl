#!/usr/bin/env perl
use strict;
use warnings;

# A sample.
package Point;

sub new {
    my ($class, %args) = @_;
    my $self = { x => $args{x} // 0, y => $args{y} // 0 };
    return bless $self, $class;
}

sub length {
    my ($self) = @_;
    return sqrt($self->{x}**2 + $self->{y}**2);
}

package main;

use constant LIMIT => 10;

sub largest {
    my @items = @_;
    return unless @items;
    my $best = shift @items;
    for my $item (@items) {
        $best = $item if $item > $best;
    }
    return $best;
}

my @points = (Point->new(x => 3, y => 4), Point->new(x => 1));
my @lengths = grep { $_ > 1 } map { $_->length } @points;
my %count = (long => scalar @lengths, all => scalar @points);

foreach my $key (sort keys %count) {
    printf "%s: %d\n", $key, $count{$key};
}

my $text = "42 apples";
if ($text =~ /^(\d+)\s+(\w+)$/) {
    print "number $1 of $2\n" unless $1 > LIMIT * 10;
}
print largest(1, 2, 3), "\n";
