require 'minitest/autorun'

class Day3 < Minitest::Test
  Pos = Struct.new(:x, :y, keyword_init: true)

  def neighbours(val, pos)
    (val.to_s.length + 2).times.map do |idx|
      [
        Pos.new(x: pos.x + idx - 1, y: pos.y - 1),
        Pos.new(x: pos.x + idx - 1, y: pos.y),
        Pos.new(x: pos.x + idx - 1, y: pos.y + 1),
      ]
    end.flatten
  end

  def part1(filename)
    symbols = {}
    vals = []
    lines = File.read(filename).split("\n")
    lines.each.with_index do |l, y|
      ls = l.split(/(\d+)/)
      ls = ls.flat_map { |v| v.match(/\d/) ? v.to_i : v.split('') }
      x = 0
      ls.each do |v|
        if v == '.'
          # no-op
        elsif v.is_a?(Integer)
          vals << [v, Pos.new(x: x, y: y)]
          x += v.to_s.length - 1
        else
          symbols[Pos.new(x: x, y: y)] = 1
        end
        x += 1
      end
    end
    # p symbols.keys
    found = vals.map do |(val, pos)|
      # p [val, pos, neighbours(val, pos)]
      if neighbours(val, pos).any? { |pos| symbols[pos] }
        # puts " ---- FOUND"
        val
      else
        0
      end
    end
    found.sum
  end

  def test_sample1
    num = part1('fixtures/day3/sample1.txt')
    assert_equal num, 4361
  end

  def test_part1
    num = part1('fixtures/day3/input.txt')
    assert_equal num, 549908
  end
end
