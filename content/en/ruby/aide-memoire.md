# Cheat sheet

This page sums up at a glance the Ruby syntax seen throughout the documentation.

## Basics

| Element | Example |
| --- | --- |
| Variable | `age = 30` |
| Constant | `RATE = 0.2` |
| String | `"Hello #{name}"` |
| Symbol | `:red` |
| Booleans | `true`, `false` |
| Nothing | `nil` |
| Comment | `# text` |
| Print | `puts`, `print`, `p` |

```ruby
name = "Alice"
puts "Hello #{name}"
p :ok
p 7 / 2
p 7.0 / 2
```

```sortie
Hello Alice
:ok
3
3.5
```

Details: [first steps](premiers-pas.md).

## Conditions and loops

| Need | Syntax |
| --- | --- |
| Condition | `if … elsif … else … end` |
| Reverse condition | `unless … end` |
| On one line | `puts "ok" if x > 0` |
| Ternary | `x > 0 ? "yes" : "no"` |
| Several cases | `case x when 1 then … end` |
| Loops | `while`, `until`, `n.times`, `each` |
| Ranges | `1..5` (included), `1...5` (excluded) |
| Control | `break`, `next` |

```ruby
(1..5).each do |n|
  next if n == 2
  puts n.even? ? "#{n} even" : "#{n} odd"
end
```

```sortie
1 odd
3 odd
4 even
5 odd
```

Details: [conditions and loops](conditions-boucles.md).

## Methods

```ruby
def greet(name, greeting = "Hello", polite: false)
  text = "#{greeting} #{name}"
  polite ? "#{text}, nice to meet you." : text
end

puts greet("Alice")
puts greet("Bob", "Hi", polite: true)
```

```sortie
Hello Alice
Hi Bob, nice to meet you.
```

| Convention | Meaning |
| --- | --- |
| `empty?` | returns true or false |
| `sort!` | changes the object |
| last line | returned value |
| `yield` | runs the block it received |

Details: [methods](fonctions.md).

## Text

| Need | Example |
| --- | --- |
| Insert a value | `"Hello #{name}"` |
| Length | `t.length` |
| Uppercase, lowercase | `t.upcase`, `t.downcase`, `t.capitalize` |
| Remove surrounding spaces | `t.strip` |
| Search | `t.include?("a")`, `t.start_with?("B")` |
| Split, join | `t.split(",")`, `array.join(" ")` |
| Replace | `t.sub("a", "b")` (once), `t.gsub("a", "b")` (everywhere) |
| A part | `t[0]`, `t[0..2]`, `t[-1]` |
| Round, format | `x.round(2)`, `format("%.2f", x)` |

```ruby
t = "  hello ruby  ".strip
puts t.capitalize
puts t[0..4]
p t.split(" ")
puts t.gsub("o", "0")
puts format("%.2f", 3.14159)
```

```sortie
Hello ruby
hello
["hello", "ruby"]
hell0 ruby
3.14
```

Details: [working with text](texte.md).

## Arrays and hashes

| Action | Array | Hash |
| --- | --- | --- |
| Create | `[1, 2]` | `{ a: 1 }` |
| Read | `t[0]` | `h[:a]` |
| Add | `t << 3` | `h[:b] = 2` |
| Remove | `t.delete(3)` | `h.delete(:b)` |
| Size | `t.length` | `h.size` |
| Contains | `t.include?(1)` | `h.key?(:a)` |

```ruby
t = [3, 1, 2]
h = { a: 1 }
h[:b] = 2
p t.sort
p h
```

```sortie
[1, 2, 3]
{:a=>1, :b=>2}
```

Details: [arrays and hashes](collections.md).

## Blocks and Enumerable

| Method | Role |
| --- | --- |
| `each` | go through |
| `map` | transform |
| `select` / `reject` | keep / remove |
| `find` | first one that matches |
| `reduce` | combine into one value |
| `sort_by` | sort by a criterion |
| `group_by` | group into a hash |
| `each_with_index` | go through with the index |

```ruby
numbers = [1, 2, 3, 4, 5]
p numbers.map { |n| n * 10 }
p numbers.select(&:odd?)
p numbers.reduce(:+)
square = ->(x) { x * x }
p square.call(4)
```

```sortie
[10, 20, 30, 40, 50]
[1, 3, 5]
15
16
```

Details: [blocks](blocs.md).

## Classes and objects

```ruby
module Displayable
  def display
    puts to_s
  end
end

class Animal
  include Displayable
  attr_reader :name

  def initialize(name)
    @name = name
  end

  def to_s
    "Animal #{name}"
  end
end

class Cat < Animal
  def to_s
    "Cat #{name}"
  end
end

Cat.new("Felix").display
Point = Struct.new(:x, :y)
p Point.new(1, 2)
```

```sortie
Cat Felix
#<struct Point x=1, y=2>
```

Details: [classes and objects](types.md).

## Modules and mixins

| Need | Syntax |
| --- | --- |
| Module function | `module Tools` then `def self.name`; call `Tools.name` |
| Constant | `module Config; VERSION = "1.0"; end`; `Config::VERSION` |
| Class stored in a module | `Zoo::Animal.new` |
| Share methods | `include ModuleName` in the class |
| Compare, sort | `include Comparable` + `def <=>(other)` |
| `map`, `select`… | `include Enumerable` + `def each` with `yield` |
| Load a file | `require_relative "lib/calc"` |

```ruby
module Calc
  RATE = 0.2

  def self.with_tax(price) = price * (1 + RATE)
end

class Grade
  include Comparable
  attr_reader :value

  def initialize(value) = @value = value
  def <=>(other) = value <=> other.value
end

puts Calc.with_tax(10)
puts Calc::RATE
puts Grade.new(12) < Grade.new(15)
p [Grade.new(15), Grade.new(9)].min.value
```

```sortie
12.0
0.2
true
9
```

Details: [modules and mixins](modules.md).

## Errors

| Keyword | Role |
| --- | --- |
| `begin … rescue … end` | catch an error |
| `rescue Type => e` | read the error in `e` |
| `ensure` | always run |
| `raise` | raise an error |
| `retry` | start the `begin` again |
| `&.` | safe call if `nil` |

```ruby
class MyError < StandardError
end

begin
  raise MyError, "ouch"
rescue MyError => e
  puts "Caught: #{e.message}"
ensure
  puts "Done"
end
p nil&.length
```

```sortie
Caught: ouch
Done
nil
```

Details: [handling errors](erreurs.md).

## Keyboard and files

| Need | Syntax |
| --- | --- |
| Read a line from the keyboard | `gets.chomp` |
| Program arguments | `ARGV[0]` |
| Write (replaces everything) | `File.write("f.txt", text)` |
| Read everything | `File.read("f.txt")` |
| Read the lines | `File.readlines("f.txt", chomp: true)` |
| Line by line | `File.foreach("f.txt")` with a block |
| Does the file exist? | `File.exist?("f.txt")` |
| Add at the end | `File.open("f.txt", "a")` with a block, then `f.puts` |
| JSON | `require "json"`, `JSON.generate(h)`, `JSON.parse(t)` |

```ruby
File.write("f.txt", "one\n")
File.open("f.txt", "a") { |f| f.puts "two" }
p File.readlines("f.txt", chomp: true)
puts File.exist?("f.txt")
```

```sortie
["one", "two"]
true
```

Details: [keyboard and files](fichiers.md).

## Tests with Minitest

| Element | Syntax |
| --- | --- |
| Load Minitest | `require "minitest/autorun"` |
| Test class | `class TestCalc < Minitest::Test` |
| A test | a method `def test_…` |
| Before each test | `def setup` |
| Equality | `assert_equal expected, actual` |
| True, false, nil | `assert x`, `refute x`, `assert_nil x` |
| Decimal numbers | `assert_in_delta 0.3, x, 0.001` |
| An error is raised | `assert_raises(ArgumentError) { … }` |
| Run | `ruby test/test_calc.rb` |
| Result | `.` passed, `F` failure, `E` error |

```ruby
require "minitest/autorun"

def double(x) = x * 2

class TestDouble < Minitest::Test
  def test_double
    assert_equal 8, double(4)
  end

  def test_error
    assert_raises(ZeroDivisionError) { 1 / 0 }
  end
end
```

Details: [testing your code](tests.md).

## Project and tools

| Command | Role |
| --- | --- |
| `ruby file.rb` | run a program |
| `irb` | interactive console |
| `gem install name` | install a gem |
| `bundle install` | install the gems from the `Gemfile` |
| `bundle exec …` | run with the project's gems |
| `rubocop` | check the style |

Details: [organizing a project](projet.md). To practice:
[step-by-step projects](projets.md) and [exercises with solutions](exercices.md). Compare with
[the Rust cheat sheet](../rust/aide-memoire.md) and [the Rythme cheat sheet](../rythme/aide-memoire.md).
