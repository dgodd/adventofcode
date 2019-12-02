require 'minitest/autorun'
require 'minitest/spec'
require 'bigdecimal'

def fuel(mass)
  (BigDecimal(mass) / BigDecimal(3)).floor - 2
end

def fuel_part2(mass)
  total = extra_fuel = fuel(mass)
  loop do
    extra_fuel = fuel(extra_fuel)
    break unless extra_fuel.positive?

    total += extra_fuel
  end
  total
end

FUEL_DATA = DATA.map(&:to_i)

describe 'fuel' do
  it 'For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.' do
    assert fuel(12) == 2
  end

  it 'For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.' do
    assert fuel(14) == 2
  end

  it 'For a mass of 1969, the fuel required is 654.' do
    assert fuel(1969) == 654
  end

  it 'For a mass of 100756, the fuel required is 33583.' do
    assert_equal fuel(100_756), 33_583
  end

  it 'answer: part 1' do
    sum = FUEL_DATA.sum do |mass|
      fuel(mass)
    end
    assert_equal sum, 3_235_550
  end

  it 'total fuel (including fuel for fuel); mass 100756' do
    assert_equal fuel_part2(100_756), 50_346
  end

  it 'total fuel (including fuel for fuel); mass 1969' do
    assert_equal fuel(1_969), 654
    assert_equal fuel_part2(1_969), 966
  end

  it 'answer: part 2' do
    subtotal = FUEL_DATA.sum do |mass|
      fuel_part2(mass)
    end
    assert_equal subtotal, 4_850_462
  end
end

__END__
115810
58892
76569
87782
103850
103320
62798
98400
71197
124777
97523
52210
122364
112858
58303
72246
130616
118911
120467
62299
71680
83273
87791
89728
112402
94325
118423
54979
99132
70851
89887
54131
103911
139205
97804
68670
113097
104705
109659
85259
138145
56602
140942
144354
104776
63627
100050
90929
130607
104809
69613
93375
136009
81838
84705
61669
84975
95055
107505
126406
116391
57303
128320
93274
78225
116717
84915
109201
102855
61361
146332
127109
78523
61900
59891
135089
55323
51659
87020
86431
132494
51020
126660
81594
73209
71717
135977
78521
82396
118952
144343
149121
119233
79917
125447
127014
138309
107308
146818
63364
