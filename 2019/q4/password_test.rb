require 'minitest/autorun'
require 'minitest/spec'

def array2(num)
  arr = []
  while num > 0 do
    arr << num % 10
    num = num / 10
  end
  arr
end

def valid?(num)
  arr = array2(num).each_cons(2)
  # Backwards, so make sure diminishing
  arr.all? { |i, j| i >= j } && arr.any? { |i, j| i == j }
end

def valid2?(num)
  arr = array2(num)
  # Backwards, so make sure diminishing
  arr.each_cons(2).all? { |i, j| i >= j } && arr.chunk_while { |i, j| i == j }.any? { |a| a.length == 2 }
end

describe 'password' do
  describe '#valid?' do
    it { assert valid?(122345) }
    it { assert valid?(111123) }
    it { assert valid?(111111) }
    it { assert !valid?(223450) }
    it { assert !valid?(123789) }
  end

  describe '#only_valid2?' do
    it { assert valid2?(122345) }
    it { assert !valid2?(111123) }
    it { assert !valid2?(111111) }
    it { assert !valid2?(223450) }
    it { assert !valid2?(123789) }
    it { assert !valid2?(123444) }
    it { assert valid2?(111122) }
  end

  it 'part 1' do
    input = (273025..767253)
    assert_equal 910, input.count { |num| valid?(num) }
  end

  it 'part 2' do
    input = (273025..767253)
    assert_equal 273025, input.first
    assert_equal 767253, input.last
    assert_equal 598, input.count { |num| valid2?(num) }
  end
end
