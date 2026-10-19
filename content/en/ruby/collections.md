# Arrays and hashes

This page presents the two main collections in Ruby: arrays (`Array`) and hashes (`Hash`).

> **Goals:**
>
> - you will know how to create an array, read, add and remove its elements;
> - you will know how to sort, count and search in an array;
> - you will know how to create a hash and find a value by its key;
> - you will know how to handle a missing key with `fetch`, a default value or `dig`;
> - you will know how to go through an array and a hash with `each`.

A **collection** is a value that holds several values. Ruby offers two main
ones:

- the **array**, a [list](../debuter/glossaire.md#list) kept in order, like
  a row of numbered lockers;
- the **hash**, a [dictionary](../debuter/glossaire.md#dictionary) that attaches a
  label to each value, like a "name → number" phone book.

## Arrays

An array is an ordered list of values, between square brackets `[ ]`, separated by
commas. Each value is called an **element**.

```ruby
fruits = ["apple", "banana", "cherry"]
p fruits
puts fruits.length
```

```sortie
["apple", "banana", "cherry"]
3
```

An array can hold any values, even mixed ones, and even other
arrays. `[]` is an empty array.

### Reading an element

Positions ([indexes](../debuter/glossaire.md#index)) start at `0`. A negative
index counts from the end: `-1` is the last element.

```ruby
fruits = ["apple", "banana", "cherry"]
puts fruits[0]
puts fruits[-1]
puts fruits.first
p fruits[10]
```

```sortie
apple
cherry
apple
nil
```

> **Note:** reading an index that does not exist does not crash: you get `nil`.

### Reading several elements

A range, or `first(n)` and `last(n)`, give a piece of the array.

```ruby
letters = ["a", "b", "c", "d", "e"]
p letters[1..3]
p letters.first(2)
p letters.last(2)
```

```sortie
["b", "c", "d"]
["a", "b"]
["d", "e"]
```

### Adding and removing

- `<<` and `push` add at the end; `unshift` adds at the beginning;
- `pop` removes the last element and returns it; `shift` removes the first one;
- `delete` removes every slot that holds this value.

```ruby
numbers = [1, 2]
numbers << 3
numbers.push(4)
p numbers
last = numbers.pop
p last
p numbers
numbers.delete(1)
p numbers
```

```sortie
[1, 2, 3, 4]
4
[1, 2, 3]
[2, 3]
```

```ruby
queue = ["Bob", "Chloe"]
queue.unshift("Alice")
p queue
first = queue.shift
puts "Your turn, #{first}"
p queue
queue[0] = "Zoe"
p queue
```

```sortie
["Alice", "Bob", "Chloe"]
Your turn, Alice
["Bob", "Chloe"]
["Zoe", "Chloe"]
```

`queue[0] = "Zoe"` replaces the element at index 0.

### Useful methods

These methods return a result **without changing** the array.

```ruby
grades = [12, 8, 15, 8]
p grades.sort
p grades.uniq
puts grades.include?(15)
puts grades.sum
puts grades.max
p grades.reverse
puts grades.join(" - ")
```

```sortie
[8, 8, 12, 15]
[12, 8, 15]
true
43
15
[8, 15, 8, 12]
12 - 8 - 15 - 8
```

Other useful questions:

```ruby
grades = [12, 8, 15, 8]
puts grades.count(8)
p grades.index(15)
puts grades.empty?
puts [].empty?
p grades.sort.reverse
puts grades.sum.fdiv(grades.length)
```

```sortie
2
2
false
true
[15, 12, 8, 8]
10.75
```

- `count(8)` counts the 8s; `index(15)` gives the position of the first 15.
- `sort.reverse` sorts from largest to smallest.
- `sum.fdiv(length)` calculates the average, with decimals.

## Going through an array

`each` runs a block for each element.

```ruby
["apple", "banana"].each do |fruit|
  puts "I like #{fruit}s"
end
```

```sortie
I like apples
I like bananas
```

With `each_with_index`, you also get the position:

```ruby
["gold", "silver", "bronze"].each_with_index do |medal, i|
  puts "#{i + 1}. #{medal}"
end
```

```sortie
1. gold
2. silver
3. bronze
```

## Hashes

A hash attaches **keys** to **values**. Think of a cloakroom: you hand over
the ticket (the key), you get your coat back (the value). Each key is unique. Keys
are often [symbols](premiers-pas.md#symbols).

```ruby
person = { name: "Alice", age: 30 }
puts person[:name]
puts person[:age]
p person[:city]
```

```sortie
Alice
30
nil
```

> **Remember:** `{ name: "Alice" }` is a shortcut for `{ :name => "Alice" }`. Both
> ways of writing it give the same hash.

### Changing a hash

- `hash[key] = value` adds a key, or replaces its value if it already exists;
- `delete` removes a key;
- `key?` asks whether a key exists.

```ruby
stock = { "apples" => 5 }
stock["pears"] = 3
stock["apples"] += 1
stock.delete("pears")
p stock
puts stock.key?("apples")
```

```sortie
{"apples"=>6}
true
```

### Default value with fetch

`fetch` lets you give a fallback value if the key is missing.

```ruby
config = { language: "en" }
puts config.fetch(:language, "fr")
puts config.fetch(:theme, "light")
```

```sortie
en
light
```

### Default value with Hash.new

`Hash.new(0)` creates a hash where **all** missing keys are worth `0`. It is ideal
for counting.

```ruby
votes = Hash.new(0)
["apple", "kiwi", "apple"].each { |fruit| votes[fruit] += 1 }
p votes
puts votes["strawberry"]
```

```sortie
{"apple"=>2, "kiwi"=>1}
0
```

Without the `0`, `votes[fruit] += 1` would crash: you cannot add 1 to `nil`.

> **Warning:** do not write `Hash.new([])`: all keys would share the same
> array. For one array per key, use `Hash.new { |h, k| h[k] = [] }`.

### Reading nested data with dig

A hash can hold other hashes and arrays. `dig` goes down level by level
and returns `nil` if a level is missing, instead of crashing.

```ruby
student = { name: "Alice", address: { city: "Lyon" }, grades: [12, 15] }
puts student[:address][:city]
puts student.dig(:address, :city)
puts student.dig(:grades, 1)
p student.dig(:parents, :mother)
```

```sortie
Lyon
Lyon
15
nil
```

`student[:parents][:mother]` would crash, because `student[:parents]` is `nil`.

## Going through a hash

`each` gives the key and the value on each turn.

```ruby
prices = { coffee: 2, tea: 3 }
prices.each do |drink, amount|
  puts "#{drink}: #{amount} €"
end
p prices.keys
p prices.values
```

```sortie
coffee: 2 €
tea: 3 €
[:coffee, :tea]
[2, 3]
```

> **Tip:** to transform or filter a collection, use `map` and `select`.
> They are explained in [the page about blocks](blocs.md).

## Common mistakes

### Reading past the end of the array

```ruby norun
fruits = ["apple", "banana", "cherry"]
puts fruits[3].upcase
```

```text
main.rb:2:in `<main>': undefined method `upcase' for nil (NoMethodError)

puts fruits[3].upcase
              ^^^^^^^
```

Three elements have the indexes 0, 1 and 2. `fruits[3]` is `nil`. The last one is
`fruits[2]`, or better `fruits[-1]` or `fruits.last`.

### Mixing up a string key and a symbol key

```ruby norun
stock = { "apples" => 5 }
puts stock[:apples] + 1
```

```text
main.rb:2:in `<main>': undefined method `+' for nil (NoMethodError)

puts stock[:apples] + 1
                    ^
```

`"apples"` (a string) and `:apples` (a symbol) are two different keys. Use the
same one as in the hash: `stock["apples"] + 1`.

### fetch without a fallback value

```ruby norun
config = { language: "en" }
puts config.fetch(:theme)
```

```text
main.rb:2:in `fetch': key not found: :theme (KeyError)
	from main.rb:2:in `<main>'
```

Without a fallback value, `fetch` crashes if the key is missing: this is on purpose, to
spot a forgotten key. Give a default value: `config.fetch(:theme, "light")`.

## Test yourself

**Question 1:** what is `[5, 3, 9][1]`?

> **Answer**
>
> `3`. Indexes start at 0: index 1 is the second element.

**Question 2:** how do you count the letters of `"abracadabra"` in a hash?

> **Answer**
>
> ```ruby
> count = Hash.new(0)
> "abracadabra".chars.each { |c| count[c] += 1 }
> p count
> ```
>
> ```sortie
> {"a"=>5, "b"=>2, "r"=>2, "c"=>1, "d"=>1}
> ```

**Question 3:** what is the difference between `h[:x]` and `h.fetch(:x)` when the key
`:x` is missing?

> **Answer**
>
> `h[:x]` returns `nil` (or the default value of the hash). `h.fetch(:x)` raises a
> `KeyError` error.

## Summary

- An array is an ordered list: `[1, 2, 3]`. Indexes start at `0`.
- `<<` adds at the end, `pop` removes the last one, `delete` removes a value.
- A hash attaches keys to values: `{ name: "Alice" }`.
- A missing key gives `nil`; `fetch`, `Hash.new(0)` and `dig` handle missing keys.
- `each` goes through an array or a hash.

Next: [ranges](intervalles.md)
