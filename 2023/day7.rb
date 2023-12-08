# frozen_string_literal: true

require 'minitest/autorun'
require 'minitest/focus'

class Day7 < Minitest::Test
  def parse(filename)
    File.read(filename).split("\n").map do |l|
      (cards, bid) = l.split(/\s+/)
      cards = cards.split('').map do |c|
        case c
        when '2'..'9'
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

  def hand_type(cards)
    if cards.uniq.length == 1
      7 # Five of a kind
    elsif cards.tally.values.sort == [1, 4]
      6 # Four of a kind
    elsif cards.tally.values.sort == [2, 3]
      5 # Full house
    elsif cards.tally.values.max == 3
      4 # Three of a kind
    elsif cards.tally.values.sort == [1, 2, 2]
      3 # Two pair
    elsif cards.tally.values.max == 2
      2 # Two of a kind
    else
      1
    end
  end

  def part1(filename)
    hands = parse(filename)
    hands = hands.map do |(cards, bid)|
      type = hand_type(cards)
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
    assert_equal 246_795_406, num
  end

  def jacks_wild_type(cards, orig_cards = nil)
    orig_cards ||= cards.dup
    type = hand_type(cards)
    idx = cards.find_index(1)
    return type unless idx

    (cards.uniq - [1]).each do |c2|
      cards[idx] = c2
      t2 = jacks_wild_type(cards, orig_cards)
      type = t2 if t2 > type
    end

    type
  end

  def part2(filename)
    hands = parse(filename)
    hands.each { |hand| hand[0] = hand[0].map { |c| c == 11 ? 1 : c } }
    hands = hands.map do |(cards, bid)|
      type = jacks_wild_type(cards.dup)
      [type, cards, bid]
    end
    hands.sort!
    hands.map(&:last).map.with_index(1) { |bid, idx| bid * idx }.sum
  end

  def test_sample2
    num = part2('fixtures/day7/sample.txt')
    assert_equal 5905, num
  end

  def test_part2
    num = part2('fixtures/day7/input.txt')
    assert_equal 249_356_515, num
  end
end
