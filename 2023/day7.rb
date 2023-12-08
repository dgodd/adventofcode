require 'minitest/autorun'
require 'minitest/focus'

class Day7 < Minitest::Test
  def parse(filename)
    File.read(filename).split("\n").map do |l|
      (cards, bid) = l.split(/\s+/)
      cards = cards.split('').map do |c|
        case c
        when "2".."9"
          c.to_i
        when 'T'
          10
        when 'J'
          11
        when 'Q'
          12
        when 'K'
          13
        when 'A'
          14
        else
          raise 'unknown'
        end
      end
      [cards, bid.to_i]
    end
  end

  def part1(filename)
    hands = parse(filename)
    hands = hands.map do |(cards, bid)|
      type = 0
      if cards.uniq.length == 1
        type = 7 # Five of a kind
      elsif cards.tally.values.sort == [1, 4]
        type = 6 # Four of a kind
      elsif cards.tally.values.sort == [2, 3]
        type = 5 # Full house
      elsif cards.tally.values.max == 3
        type = 4 # Three of a kind
      elsif cards.tally.values.sort == [1, 2, 2]
        type = 3 # Two pair
      elsif cards.tally.values.max == 2
        type = 2 # Two of a kind
      else
        type = 1
      end
      [type, *cards, bid]
    end
    hands.sort!
    hands.map(&:last).map.with_index(1) { |bid, idx| bid * idx }.sum
  end

  def test_sample1
    num = part1('fixtures/day7/sample.txt')
    assert_equal 6440, num
  end

  def test_part1
    num = part1('fixtures/day7/input.txt')
    assert_equal 246795406, num
  end
end
