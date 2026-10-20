# Classes and objects

This page shows how to create your own objects with classes, inheritance, modules and `Struct`.

> **Goals:**
>
> - you will know how to write a class and create objects with `new`;
> - you will know how to give data to an object with `initialize` and `@` variables;
> - you will know how to make that data readable with `attr_reader` and `attr_accessor`;
> - you will know how to hide a method with `private`;
> - you will know how to reuse a class with inheritance and `super`.

## Defining a class

A [class](../debuter/glossaire.md#class) is a template for making objects.
Think of a cake mold: the mold is the class, each cake is an
[object](../debuter/glossaire.md#object). All the cakes have the same shape, but each one
can have its own flavor. The name of a class starts with a capital letter.

```ruby
class Dog
  def bark
    puts "Woof!"
  end
end

rex = Dog.new
rex.bark
```

```sortie
Woof!
```

`Dog.new` creates a new object: an **instance** of the class. `rex.bark` calls
the `bark` method on this object.

You have been using classes from the start: `"hello"` is an instance of `String`,
`42` an instance of `Integer`. `class` and `is_a?` show it.

```ruby
class Dog
end

rex = Dog.new
puts rex.class
puts rex.is_a?(Dog)
puts 42.is_a?(Integer)
puts "hello".class
```

```sortie
Dog
true
true
String
```

## initialize and instance variables

`initialize` is called by `new`. It prepares the object, like filling in a form
when you sign up. The arguments of `new` are passed to `initialize`.

Variables that start with `@` are **instance variables**: they
belong to the object. Each object has its own, and they live as long
as the object does. All the methods of the object can read them.

```ruby
class Dog
  def initialize(name)
    @name = name
  end

  def introduce
    puts "My name is #{@name}."
  end
end

Dog.new("Rex").introduce
Dog.new("Pluto").introduce
```

```sortie
My name is Rex.
My name is Pluto.
```

## attr_accessor

`@` variables are hidden: you cannot read them from outside the object.
To read or change `@name` from outside, use `attr_reader`, `attr_writer`
or `attr_accessor`. They write the small methods `name` and `name=` for you.

```ruby
class Person
  attr_accessor :name
  attr_reader :age

  def initialize(name, age)
    @name = name
    @age = age
  end
end

alice = Person.new("Alice", 30)
alice.name = "Alice Martin"
puts alice.name
puts alice.age
```

```sortie
Alice Martin
30
```

> **Remember:** `attr_reader` lets you read, `attr_writer` lets you write, `attr_accessor`
> does both.

## Methods and to_s

A class can have as many methods as you want. `to_s` decides how the object
is printed with `puts`.

```ruby
class Point
  def initialize(x, y)
    @x = x
    @y = y
  end

  def distance_to_origin
    Math.sqrt(@x**2 + @y**2)
  end

  def to_s
    "(#{@x}, #{@y})"
  end
end

pt = Point.new(3, 4)
puts pt
puts pt.distance_to_origin
```

```sortie
(3, 4)
5.0
```

## self and class methods

Inside a method, `self` means the object itself. In front of a method name
(`def self.name`), it creates a **class method**: you call it on the class, not
on an object.

```ruby
class Temperature
  attr_reader :degrees

  def initialize(degrees)
    @degrees = degrees
  end

  def self.from_fahrenheit(f)
    new(((f - 32) * 5 / 9.0).round(1))
  end

  def hot?
    self.degrees > 25
  end
end

t = Temperature.from_fahrenheit(86)
puts t.degrees
puts t.hot?
```

```sortie
30.0
true
```

## Private methods

After the word `private`, methods can only be used **inside** the class. They
are internal tools, like the kitchen of a restaurant: the customer does not see
it.

```ruby
class Account
  def initialize(balance)
    @balance = balance
  end

  def display
    "Balance: #{format_euros(@balance)}"
  end

  private

  def format_euros(amount)
    format("%.2f €", amount)
  end
end

account = Account.new(42)
puts account.display
puts account.respond_to?(:format_euros)
```

```sortie
Balance: 42.00 €
false
```

`respond_to?` asks whether the object accepts a method from outside.

## Inheritance

A class can inherit from another one with `<`. It gets all its methods.

```ruby
class Animal
  def initialize(name)
    @name = name
  end

  def speak
    "#{@name} makes a noise."
  end
end

class Cat < Animal
  def speak
    "#{@name} meows."
  end
end

puts Animal.new("Thing").speak
puts Cat.new("Felix").speak
```

```sortie
Thing makes a noise.
Felix meows.
```

`Cat` is the **child** class, `Animal` the **parent** class. `Cat` redefines
`speak`: its version replaces the parent's.

### Calling the parent with super

In a redefined method, `super` calls the version of the parent class. It is
handy to add to it instead of rewriting everything.

```ruby
class Animal
  def initialize(name)
    @name = name
  end
end

class Dog < Animal
  def initialize(name, breed)
    super(name)
    @breed = breed
  end

  def to_s
    "#{@name} (#{@breed})"
  end
end

puts Dog.new("Rex", "shepherd")
puts Dog.new("Rex", "shepherd").is_a?(Animal)
```

```sortie
Rex (shepherd)
true
```

A `Dog` **is an** `Animal`: `is_a?(Animal)` answers `true`.

## Modules and mixins

A [module](../debuter/glossaire.md#module) groups methods together. With `include`, a
class gets them: this is a **mixin**. The chapter [modules](modules.md)
presents them in detail.

```ruby
module Flying
  def fly
    "#{name} takes off!"
  end
end

class Bird
  include Flying
  attr_reader :name

  def initialize(name)
    @name = name
  end
end

puts Bird.new("Tweety").fly
```

```sortie
Tweety takes off!
```

> **Note:** a class has only one parent, but it can include several modules.

## Struct

`Struct` quickly creates a small class with attributes: `new`, the accessors and a
readable display are written for you.

```ruby
Book = Struct.new(:title, :author)

book = Book.new("Les Misérables", "Victor Hugo")
puts book.title
book.author = "V. Hugo"
p book
```

```sortie
Les Misérables
#<struct Book title="Les Misérables", author="V. Hugo">
```

## Common mistakes

### Forgetting an argument of new

```ruby norun
class Dog
  def initialize(name)
    @name = name
  end
end
rex = Dog.new
```

```text
main.rb:2:in `initialize': wrong number of arguments (given 0, expected 1) (ArgumentError)
	from main.rb:6:in `new'
	from main.rb:6:in `<main>'
```

`new` passes its arguments to `initialize`, which expects a name. Write `Dog.new("Rex")`.

### Reading an @ variable without an accessor

```ruby norun
class Person
  def initialize(name)
    @name = name
  end
end
alice = Person.new("Alice")
puts alice.name
```

```text
main.rb:7:in `<main>': undefined method `name' for an instance of Person (NoMethodError)

puts alice.name
          ^^^^^
```

`@name` is hidden inside the object. Add `attr_reader :name` to the class.

### A class name in lowercase

```ruby norun
class dog
end
```

```text
main.rb: --> main.rb
expected a constant name after `class`
> 1  class dog
> 2  end
main.rb:1: class/module name must be CONSTANT (SyntaxError)
class dog
      ^~~
```

A class name is a constant: it starts with a capital letter. Write
`class Dog`.

## Test yourself

**Question 1:** which method is called by `Book.new("Dune")`?

> **Answer**
>
> `initialize`, with the argument `"Dune"`.

**Question 2:** what must you add so that this code prints `Rex`?

```ruby norun
class Dog
  def initialize(name)
    @name = name
  end
end
puts Dog.new("Rex").name
```

> **Answer**
>
> A reader accessor in the class:
>
> ```ruby
> class Dog
>   attr_reader :name
>
>   def initialize(name)
>     @name = name
>   end
> end
> puts Dog.new("Rex").name
> ```
>
> ```sortie
> Rex
> ```

**Question 3:** in `class Cat < Animal`, which one is the parent?

> **Answer**
>
> `Animal`. `Cat` gets all its methods and can redefine them.

## Summary

- A class is a template; `new` creates an object and calls `initialize`.
- `@` variables belong to the object.
- `attr_reader`, `attr_writer` and `attr_accessor` make them readable or writable.
- `private` hides methods; `self.` creates a class method.
- `class Cat < Animal` inherits the methods of `Animal`; `super` calls the parent.
- A module shares methods with `include`; `Struct` quickly creates a small class.

Next: [inheritance and polymorphism](heritage.md)
