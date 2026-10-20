# Modules and mixins

This page shows how to organize your code in modules and share methods between several classes with `include`.

> **Goals:**
>
> - store methods and constants in a module;
> - call a module function with `Module.method`;
> - share methods between classes with `include` (a mixin);
> - compare and sort your objects with `Comparable`;
> - give the `Enumerable` methods to your own class;
> - split your code into several files with `require_relative`.

## What is a module?

A **module** is a box with a name. It holds methods and constants that belong
together. The name of a module starts with a capital letter, like a class.

```ruby
module Greetings
  def self.hello(name)
    "Hello, #{name}!"
  end
end

puts Greetings.hello("Alice")
```

```sortie
Hello, Alice!
```

A module looks like a class, with one big difference: **you cannot create an object
from a module**. No `Greetings.new`.

> **Note:** the word "module" exists in other languages, with a similar meaning
> (see [the glossary](../debuter/glossaire.md#module)).

## A module as a namespace

Two libraries can have a method or a class with the same name. If you put them in
different modules, they don't get in each other's way: each module is a **namespace**.

```ruby
module French
  def self.greet
    "Bonjour"
  end
end

module English
  def self.greet
    "Hello"
  end
end

puts French.greet
puts English.greet
```

```sortie
Bonjour
Hello
```

A class can also live inside a module. To reach it, write
`Module::Class` (two colons).

```ruby
module Zoo
  class Animal
    def initialize(name)
      @name = name
    end

    def to_s
      "#{@name} (zoo animal)"
    end
  end
end

puts Zoo::Animal.new("Rex")
```

```sortie
Rex (zoo animal)
```

## Module functions

A method defined with `def self.name` belongs to the module itself. You call it with
`Module.name`, without creating an object. This is handy for a toolbox.

```ruby
module Calc
  def self.double(x)
    x * 2
  end

  def self.average(numbers)
    numbers.sum.to_f / numbers.size
  end
end

puts Calc.double(21)
puts Calc.average([12, 15, 18])
```

```sortie
42
15.0
```

Ruby itself stores its math tools this way, in the `Math` module:

```ruby
puts Math.sqrt(16)
puts Math::PI.round(2)
```

```sortie
4.0
3.14
```

## Module constants

A module also holds **constants** (values that don't change). Their names are written
in capital letters. Outside the module, write `Module::CONSTANT`.

```ruby
module Config
  VERSION = "1.2.0"
  LANGUAGES = ["en", "fr"]
end

puts Config::VERSION
p Config::LANGUAGES
```

```sortie
1.2.0
["en", "fr"]
```

Inside the module, the name alone is enough:

```ruby
module Circle
  PI = 3.14159

  def self.area(radius)
    (PI * radius * radius).round(2)
  end
end

puts Circle.area(2)
```

```sortie
12.57
```

## Sharing methods with include

Here is the other big use of modules. Write methods **without** `self.` in a module,
then add `include ModuleName` in a class: every object of the class gets these methods.
Such a module is called a **mixin** (something you "mix in").

```ruby
module Presentable
  def introduce
    "My name is #{name}."
  end
end

class Student
  include Presentable
  attr_reader :name

  def initialize(name)
    @name = name
  end
end

class Dog
  include Presentable
  attr_reader :name

  def initialize(name)
    @name = name
  end
end

puts Student.new("Alice").introduce
puts Dog.new("Rex").introduce
```

```sortie
My name is Alice.
My name is Rex.
```

`Student` and `Dog` have nothing in common, but they share the `introduce` method.
Inside the module, `name` calls the `name` method of the object that included the module.

> **Remember:** a class has only one parent (`class Cat < Animal`), but it can include
> as many modules as it wants. See also
> [inheritance](types.md#inheritance).

### Checking that a class includes a module

`include?` answers the question "did this class receive this module?".

```ruby
module Flyable
  def fly
    "I'm flying!"
  end
end

class Bird
  include Flyable
end

class Fish
end

puts Bird.include?(Flyable)
puts Fish.include?(Flyable)
puts Bird.new.fly
```

```sortie
true
false
I'm flying!
```

## Comparable: comparing your objects

Ruby provides ready-made mixins. The first one is `Comparable`. It gives you `<`,
`>`, `<=`, `>=`, `==`, `between?` and `clamp`. In return, you write **a single**
method: `<=>`.

### The <=> operator

`a <=> b` compares two values. It returns `-1` if `a` is smaller, `0` if they are
equal, `1` if `a` is bigger.

```ruby
p 3 <=> 5
p 5 <=> 5
p 7 <=> 5
p "apricot" <=> "banana"
```

```sortie
-1
0
1
-1
```

### Including Comparable

Define `<=>` by comparing what matters to you (here, the value of the grade),
then add `include Comparable`.

```ruby
class Grade
  include Comparable
  attr_reader :value

  def initialize(value)
    @value = value
  end

  def <=>(other)
    value <=> other.value
  end

  def to_s
    "#{value}/20"
  end
end

a = Grade.new(12)
b = Grade.new(15)

puts a < b
puts a == Grade.new(12)
puts b.between?(Grade.new(10), Grade.new(16))
puts [b, a, Grade.new(9)].sort.join(", ")
puts [b, a].max
```

```sortie
true
true
true
9/20, 12/20, 15/20
15/20
```

> **Tip:** with `<=>`, `sort`, `min` and `max` also work on an array of
> your objects.

## Enumerable: making your class iterable

The second famous mixin is `Enumerable`, the one that gives `map`, `select`, `sort`…
to arrays (see [blocks](blocs.md#the-enumerable-methods)). You can give it to your own
class. You only need to write **one** method: `each`, which passes each element to the
block with `yield`.

```ruby
class Playlist
  include Enumerable

  def initialize
    @titles = []
  end

  def add(title)
    @titles << title
    self
  end

  def each
    @titles.each { |title| yield title }
  end
end

list = Playlist.new
list.add("Imagine").add("Yesterday").add("Hey Jude")

list.each { |t| puts "- #{t}" }
p list.map(&:upcase)
p list.select { |t| t.start_with?("Y") }
p list.sort
p list.count
p list.include?("Imagine")
```

```sortie
- Imagine
- Yesterday
- Hey Jude
["IMAGINE", "YESTERDAY", "HEY JUDE"]
["Yesterday"]
["Hey Jude", "Imagine", "Yesterday"]
3
true
```

With a single `each` method, your `Playlist` gets dozens of methods:
`map`, `select`, `sort`, `count`, `include?`, `first`, `min`, `sum`…

> **Note:** `add` returns `self` (the object itself). This is what lets you
> chain the calls: `list.add(…).add(…)`.

## One module per file with require_relative

In a real project, each module goes in its own file. `require_relative`
loads a file located next to the current file (without the `.rb` extension).

```text
my_project/
├── main.rb
└── lib/
    └── calc.rb
```

The file `lib/calc.rb` contains the module:

```ruby norun
# lib/calc.rb
module Calc
  def self.double(x)
    x * 2
  end
end
```

The file `main.rb` loads it, then uses it:

```ruby norun
# main.rb
require_relative "lib/calc"

puts Calc.double(21)
```

In the terminal, from the `my_project` folder:

```sh
ruby main.rb
```

```text
42
```

> **Remember:** the path in `require_relative` starts from the folder **of the file that
> contains it**, not from the folder where you run the command. The chapter
> [Organizing a project](projet.md) goes further.

## Common mistakes

### Calling a method without self. on the module

A method written without `self.` is meant for `include`. The module itself doesn't
know it.

```ruby norun
module Tools
  def double(x)
    x * 2
  end
end

Tools.double(2)
```

```text
main.rb:7:in `<main>': undefined method `double' for module Tools (NoMethodError)
```

To call it with `Tools.double`, write `def self.double(x)`.

### Forgetting the module name before a constant

Outside the module, the constant is not visible by its name alone.

```ruby norun
module Geometry
  PI = 3.14
end

puts PI
```

```text
main.rb:5:in `<main>': uninitialized constant PI (NameError)
```

Write `Geometry::PI`.

### Including Comparable without writing <=>

`Comparable` needs `<=>` to compare. Without it, Ruby doesn't know how.

```ruby norun
class Grade
  include Comparable
  attr_reader :value

  def initialize(value)
    @value = value
  end
end

p Grade.new(12) < Grade.new(15)
```

```text
main.rb:10:in `<': comparison of Grade with Grade failed (ArgumentError)
```

Add `def <=>(other) = value <=> other.value`. In the same way, including `Enumerable`
without writing `each` gives a `NoMethodError` ("undefined method each").

### Creating an object from a module

A module doesn't build objects.

```ruby norun
module Tools
end

Tools.new
```

```text
main.rb:4:in `<main>': undefined method `new' for module Tools (NoMethodError)
```

If you need objects, write a `class`.

## Test yourself

**1.** What is the difference between `def self.area` and `def area` in a module?

> **Answer**
>
> `def self.area` creates a module function: you call it with `Module.area`.
> `def area` creates a method to share: a class receives it with `include`.

**2.** You want to sort an array of your `Player` objects by score. What do you need to write?

> **Answer**
>
> Add `include Comparable` and a `<=>` method that compares the scores:
> `def <=>(other) = score <=> other.score`. Then `players.sort` works.

**3.** Which method must you write so that a class that includes `Enumerable` has `map`
and `select`?

> **Answer**
>
> The `each` method, which passes each element to the block with `yield`.

## Summary

- A module holds methods and constants; you don't create objects from it.
- `def self.name` creates a module function, called with `Module.name`; a constant
  is written `Module::CONSTANT`.
- `include Module` gives the module's methods to a class: it is a mixin.
- `Comparable` needs `<=>`; `Enumerable` needs `each`.
- `require_relative "lib/file"` loads a file from your project.

Next: [Struct, Data and Set](structures.md)
