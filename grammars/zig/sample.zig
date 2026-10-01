const std = @import("std");

const Point = struct {
    x: f32,
    y: f32 = 0,

    pub fn length(self: Point) f32 {
        return @sqrt(self.x * self.x + self.y * self.y);
    }
};

const Colour = enum { red, green, blue };

fn parse(text: []const u8) !u32 {
    var total: u32 = 0;
    for (text) |c| {
        if (c < '0' or c > '9') return error.NotADigit;
        total = total * 10 + (c - '0');
    }
    return total;
}

pub fn main() !void {
    const p = Point{ .x = 3, .y = 4 };
    const n = parse("42") catch 0;
    // A comment.
    std.debug.print("{d} {d} {s}\n", .{ p.length(), n, @tagName(Colour.green) });
}

test "parse" {
    try std.testing.expectEqual(@as(u32, 7), try parse("7"));
}
