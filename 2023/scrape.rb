#!/usr/bin/env ruby
require 'fileutils'
require 'nokogiri'
require 'excon'
require 'reverse_markdown'


AOC_SESSION = File.read('/Users/dgodd/.adventofcode.session').chomp
res = Excon.get('https://adventofcode.com/2023', headers: { Cookie: "session=#{AOC_SESSION}" })
doc = Nokogiri::HTML(res.body)
days = doc.css('pre.calendar a').map { |a| a[:href] }

day = days.first.match(/(\d+)$/)[1].to_i
FileUtils.mkdir_p("fixtures/day#{day}")
system("aoc", "download", "--day=#{day}", "--input-file=fixtures/day#{day}/input.txt", "--puzzle-file=fixtures/day#{day}/README.txt", "--overwrite")

# day = '4'
# res = Excon.get("https://adventofcode.com/2023/day/#{day}", headers: { Cookie: "session=#{AOC_SESSION}" })
# doc = Nokogiri::HTML(res.body)
# FileUtils.mkdir_p("fixtures/day#{day}")
# txt = ReverseMarkdown.convert(doc.css('article.day-desc').to_s)
# puts txt
# File.write("fixtures/day#{day}/README.txt", txt)
# File.write("fixtures/day#{day}/sample1.txt", doc.css('article.day-desc > pre > code').text)
# 
# res = Excon.get("https://adventofcode.com/2023/day/#{day}/input", headers: { Cookie: "session=#{AOC_SESSION}" })
# File.write("fixtures/day#{day}/input.txt", res.body)

system('git', 'add', "fixtures/day#{day}")
