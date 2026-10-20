# Inheritance and polymorphism

This page shows how a class can inherit from another one, how different objects can respond to the same message, and when to prefer composition over inheritance.

> **Goals:**
>
> - you will know how to create a subclass with `<` and call the parent with `super`;
> - you will know how to override a method and read the `ancestors` chain;
> - you will know how to handle a list of different objects in the same way (polymorphism);
> - you will know the difference between `private` and `protected`;
> - you will know when to use composition rather than inheritance.

## Creating a subclass

The [classes and objects](types.md#inheritance) chapter introduced inheritance. Reminder: a
[class](../debuter/glossaire.md#classe) can **inherit** from another one with `<`. It
receives all of its methods, just as a child inherits traits from their parents.

```ruby
class Vehicle
  def initialize(brand)
    @brand = brand
  end

  def start
    "#{@brand} starts."
  end
end

class Car < Vehicle
end

puts Car.new("Renault").start
puts Car.superclass
```

```sortie
Renault starts.
Vehicle
```

`Car` wrote nothing, but it already knows how to `start`. `superclass` gives the direct
parent.

## Overriding a method

The subclass can write its own version of a method of the parent. We say that it
**overrides** it. Its own version is the one that is used.

```ruby
class Vehicle
  def noise
    "..."
  end
end

class Motorbike < Vehicle
  def noise
    "Vroom!"
  end
end

class Bike < Vehicle
  def noise
    "Ring ring!"
  end
end

puts Vehicle.new.noise
puts Motorbike.new.noise
puts Bike.new.noise
```

```sortie
...
Vroom!
Ring ring!
```

## Extending with super

`super` calls the method with the same name in the parent. This way you can **add**
something instead of rewriting everything.

```ruby
class Employee
  def initialize(name, salary)
    @name = name
    @salary = salary
  end

  def card
    "#{@name}: #{@salary} €"
  end
end

class Manager < Employee
  def initialize(name, salary, team)
    super(name, salary)
    @team = team
  end

  def card
    super + " (team of #{@team.size})"
  end
end

puts Employee.new("Léa", 2500).card
puts Manager.new("Hugo", 4000, ["Léa", "Tom"]).card
```

```sortie
Léa: 2500 €
Hugo: 4000 € (team of 2)
```

> **Warning:** `super` on its own, without parentheses, passes on **the same arguments**
> that were received. `super()` with empty parentheses passes none.

## The ancestors chain

When you call a method, Ruby looks for it in the object's class, then in its parent,
then in the parent's parent… `ancestors` shows this search order.

```ruby
class Animal
end

class Dog < Animal
end

p Dog.ancestors
puts Dog.new.is_a?(Animal)
puts Dog.new.instance_of?(Animal)
```

```sortie
[Dog, Animal, Object, Kernel, BasicObject]
true
false
```

Every class ultimately inherits from `Object`. `Kernel` is a
[module](../debuter/glossaire.md#module) included in `Object`: it is the one that
provides `puts`. `is_a?` looks at the whole chain, `instance_of?` only at the exact class.

## Polymorphism

"Polymorphism" means "many forms". Objects of different classes respond to the **same
message**, each in its own way. The code that uses them does not need to know which is
which.

```ruby
class Shape
  def description
    "#{self.class.name} with area #{area.round(2)}"
  end
end

class Rectangle < Shape
  def initialize(width, height)
    @width = width
    @height = height
  end

  def area = @width * @height
end

class Circle < Shape
  def initialize(radius)
    @radius = radius
  end

  def area = Math::PI * @radius**2
end

shapes = [Rectangle.new(3, 4), Circle.new(1), Rectangle.new(2, 2)]
shapes.each { |s| puts s.description }
puts "Total: #{shapes.sum(&:area).round(2)}"
```

```sortie
Rectangle with area 12
Circle with area 3.14
Rectangle with area 4
Total: 19.14
```

The loop contains no `if`. To add a triangle, you only need to write a new class: the
loop does not change.

## Abstract methods

`Shape` above uses `area`, but does not define it. If a subclass forgets to write it,
the error is confusing. An **abstract** method makes the rule clear: it exists in the
parent only to raise `NotImplementedError`.

```ruby
class Shape
  def area
    raise NotImplementedError, "#{self.class} must define area"
  end
end

class Triangle < Shape
end

begin
  Triangle.new.area
rescue NotImplementedError => e
  puts "Forgotten: #{e.message}"
end
```

```sortie
Forgotten: Triangle must define area
```

## Duck typing

In Ruby, inheritance is not even needed for polymorphism. What matters is that the
object **knows how to respond** to the message. "If it walks like a duck and it quacks
like a duck, then it is a duck": this is called *duck typing*.

```ruby
class Duck
  def speak = "Quack!"
end

class Robot
  def speak = "Beep."
end

[Duck.new, Robot.new, "text"].each do |thing|
  if thing.respond_to?(:speak)
    puts thing.speak
  else
    puts "#{thing.inspect} cannot speak"
  end
end
```

```sortie
Quack!
Beep.
"text" cannot speak
```

`Duck` and `Robot` have no common parent (apart from `Object`). `respond_to?` asks
whether the object knows a method.

## private and protected

- A `private` method can only be called **from inside** the object, on itself.
- A `protected` method can also be called on **another object of the same family** of
  classes. It is useful to compare two objects without showing everything to the
  outside.

```ruby
class Account
  def initialize(balance)
    @balance = balance
  end

  def richer_than?(other)
    balance > other.balance
  end

  protected

  def balance
    @balance
  end
end

a = Account.new(500)
b = Account.new(200)
puts a.richer_than?(b)

begin
  a.balance
rescue NoMethodError => e
  puts "Forbidden: #{e.message}"
end
```

```sortie
true
Forbidden: protected method `balance' called for an instance of Account
```

With `private`, `other.balance` would have been refused, even inside the class.

## Prefer composition

Inheritance says "**is a**": a `Dog` is an `Animal`. If the sentence sounds wrong, do
not use inheritance. A `Car` is not an `Engine`: it **has an** engine. You then store the
object in a variable: this is **composition**.

```ruby
class Engine
  def initialize(power)
    @power = power
  end

  def start = "#{@power} hp engine started"
end

class Car
  def initialize(engine)
    @engine = engine
  end

  def start = "#{@engine.start}, the car is moving."
end

puts Car.new(Engine.new(90)).start
puts Car.new(Engine.new(150)).start
```

```sortie
90 hp engine started, the car is moving.
150 hp engine started, the car is moving.
```

You can change the engine without changing the class. To share methods between classes
that are not related, also think of [modules](modules.md).

> **Remember:** use inheritance for a real "is a" relationship, over one or two levels.
> For the rest, composition and modules are more flexible.

## Common mistakes

### Forgetting to call super in initialize

```ruby norun
class Animal
  def initialize(name)
    @name = name
  end

  def name = @name
end

class Cat < Animal
  def initialize(name, color)
    @color = color
  end
end

p Cat.new("Felix", "ginger").name
```

```text
nil
```

No error, but `@name` was never filled in: the parent's `initialize` was not called. Add
`super(name)` at the beginning.

### Writing super without parentheses with other arguments

```ruby norun
class Animal
  def initialize(name)
    @name = name
  end
end

class Cat < Animal
  def initialize(name, color)
    super
    @color = color
  end
end

Cat.new("Felix", "ginger")
```

```text
main.rb:2:in `initialize': wrong number of arguments (given 2, expected 1) (ArgumentError)
	from main.rb:9:in `initialize'
	from main.rb:14:in `new'
	from main.rb:14:in `<main>'
```

`super` on its own passes on **all** the arguments received (here 2). Write `super(name)`.

### Calling a private method from outside

```ruby norun
class Safe
  private

  def code = 1234
end

Safe.new.code
```

```text
main.rb:7:in `<main>': private method `code' called for an instance of Safe (NoMethodError)

Safe.new.code
        ^^^^^
```

A `private` method is for internal workings. If the outside needs it, put it before
`private`.

## Test yourself

**Question 1:** what does `Integer.ancestors.include?(Comparable)` return?

> **Answer**
>
> `true`: `Integer` includes the `Comparable` module, which is one of its ancestors.
>
> ```ruby
> puts Integer.ancestors.include?(Comparable)
> ```
>
> ```sortie
> true
> ```

**Question 2:** in an overridden method `speak`, how do you reuse the parent's text and
add "!" to it?

> **Answer**
>
> With `super + "!"`: `super` calls the parent's `speak` and returns its result.

**Question 3:** a `Library` class contains books. Should it inherit from `Array`?

> **Answer**
>
> No: a library **has** books, it is not an array. Store an array in a `@books`
> variable (composition).

## Summary

- `class Child < Parent` inherits all the methods of the parent.
- A subclass can override a method; `super` calls the parent's version.
- `ancestors` shows the order in which Ruby looks for a method.
- Polymorphism: different objects respond to the same message; duck typing only looks
  at `respond_to?`.
- `private`: only the object itself; `protected`: also objects of the same family.
- "Is a" → inheritance; "has a" → composition.

Next: [modules and mixins](modules.md)
