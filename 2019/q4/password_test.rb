require 'minitest/autorun'
require 'minitest/spec'

def valid?(num)
  double = false
  digit = nil
  # Go backwards for digits, so make sure they get smaller
  loop do
    prev_digit = digit
    digit = num % 10
    num = num / 10 

    return false if prev_digit && prev_digit < digit
    double = true if prev_digit == digit

    return double if num == 0
  end
end

def two_adjacent_matching?(num)
  num.to_s.split('').chunk_while { |i, j| i == j }.map(&:count).any?(2)
end

describe 'password' do
  describe '#valid?' do
    it { assert valid?(122345) }
    it { assert valid?(111123) }
    it { assert valid?(111111) }
    it { assert !valid?(223450) }
    it { assert !valid?(123789) }
  end

  describe '#only_two_adjacent_matching?' do
    it { assert two_adjacent_matching?(122345) }
    it { assert !two_adjacent_matching?(111123) }
    it { assert !two_adjacent_matching?(111111) }
    it { assert two_adjacent_matching?(223450) }
    it { assert !two_adjacent_matching?(123789) }
    it { assert !two_adjacent_matching?(123444) }
    it { assert two_adjacent_matching?(111122) }
  end

  it 'part 1' do
    input = (273025..767253)
    assert_equal 910, input.count { |num| valid?(num) }
  end

  it 'part 2' do
    input = (273025..767253)
    assert_equal 273025, input.first
    assert_equal 767253, input.last
    assert_equal 598, input.count { |num| valid?(num) && two_adjacent_matching?(num) }
  end
end
