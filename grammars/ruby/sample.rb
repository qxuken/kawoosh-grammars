# frozen_string_literal: true

require "json"

module Shapes
  class Point
    attr_reader :x, :y

    def initialize(x, y = 0)
      @x = x
      @y = y
    end

    def length
      Math.sqrt(x**2 + y**2)
    end

    def to_s = "(#{x}, #{y})"
  end
end

points = [1, 2, 3].map { |n| Shapes::Point.new(n, n * 2) }
points.each do |point|
  next if point.x.zero?

  puts "#{point}: #{point.length.round(2)}"
end

config = { name: "sample", "size" => 3, list: %w[a b c] }
text = <<~TEXT
  A heredoc with #{config[:name]}.
TEXT

begin
  JSON.parse(text)
rescue JSON::ParserError => e
  warn e.message unless text =~ /\A\s*\z/
ensure
  puts :done
end
