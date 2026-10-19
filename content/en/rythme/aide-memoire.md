# Cheat sheet

All of Rythme at a glance: the syntax, the most useful functions, the commands, and a table for those who come from Ruby.

## Values and names

| Write | To |
|-------|----|
| `name = value` | give a name to a value (forever) |
| `42`, `3.14`, `"text"`, `true` | Int, Float, Text, Bool |
| `"Hello {name}"`, `"#{name}"` | put a value inside a text |
| `# ...`, `## ...` | comment, documentation comment |
| `:ok` | a symbol |
| `()` | the empty value |

```rythme
name = "Ada"
age = 36
puts "#{name} is {age} years old", type_of(age)
```

```sortie
Ada is 36 years old Int
```

See [first steps](premiers-pas.md).

## Arithmetic and comparisons

| Operators | Meaning |
|-----------|---------|
| `+ - * /` | arithmetic (`/` always gives a Float) |
| `//`, `%`, `**` | whole division, remainder, power |
| `== != < <= > >=` | comparisons |
| `and`, `or`, `not` | logic |
| `a..b` | list of the whole numbers from a to b |
| `x \|> f(a)` | `f(x, a)` |

## Conditions

```rythme
n = 7
print(if n % 2 == 0 then "even" else "odd")
print(unless n > 10 then "small" else "big")
print(match n
  0 -> "zero"
  1..9 -> "one digit"
  _ -> "big"
end)
```

```sortie
odd
small
one digit
```

See [conditions and repetition](conditions-boucles.md).

## Functions

```rythme
fn double(x) = x * 2

fn introduce(name)
  message = "I am {name}"
  message + "."
end

fn fact(0) = 1
fn fact(n) = n * fact(n - 1)

fn sign(n) if n < 0 = "negative"
fn sign(_) = "positive or zero"

triple = fn(x) = x * 3
print(double(4), introduce("Ada"), fact(5), sign(-2), triple(2))
```

```sortie
8 I am Ada. 120 negative 6
```

See [functions](fonctions.md).

## Text

| Function | Example | Result |
|----------|---------|--------|
| `len`, `upper`, `lower` | `upper("café")` | `"CAFÉ"` |
| `capitalize`, `trim` | `trim("  ok ")` | `"ok"` |
| `split`, `join` | `split("a,b", ",")` | `["a", "b"]` |
| `contains`, `starts_with` | `contains("hello", "ell")` | `true` |
| `replace` | `replace("a-b", "-", "+")` | `"a+b"` |
| `chars`, `lines` | `chars("ab")` | `["a", "b"]` |
| `repeat`, `pad_left` | `pad_left("7", 3, "0")` | `"007"` |
| `parse_int`, `parse_float` | `parse_int("42")` | `Ok(42)` |
| `format_number` | `format_number(1234.5, 1, "en")` | `"1,234.5"` |
| `matches`, `find_all`, `replace_all` | `find_all("a1b22", "\\d+")` | `["1", "22"]` |

```rythme
titles = "the cat sleeps" |> split |> map(capitalize)
print(join(titles, " "), len(titles), parse_int("x") |> unwrap_or(0))
```

```sortie
The Cat Sleeps 3 0
```

See [working with text](texte.md).

## Collections

| Write | Gives |
|-------|-------|
| `[1, 2, 3]`, `xs[0]`, `xs[-1]` | a list, its first element, its last one |
| `[..xs, 4]`, `xs + [4]`, `push(xs, 4)` | a new list with 4 added |
| `{name: "Ada", age: 36}`, `r.name` | a record, a field |
| `{..r, age: 37}` | a copy with one field changed |
| `dict([["a", 1]])`, `d["a"]`, `put(d, "b", 2)` | a dictionary |
| `set([1, 2, 2])` | a set: `set(1, 2)` |

See [lists, records and dictionaries](collections.md).

## List functions

| Function | Example | Result |
|----------|---------|--------|
| `map` | `map([1, 2], fn(x) = x * 10)` | `[10, 20]` |
| `filter`, `reject` | `filter(1..6, is_even)` | `[2, 4, 6]` |
| `fold` | `fold([1, 2, 3], 0, fn(a, x) = a + x)` | `6` |
| `each` | `each(xs, print)` | prints each element |
| `sum`, `len`, `max`, `min` | `sum([1, 2, 3])` | `6` |
| `sort`, `reverse`, `unique` | `sort([3, 1, 2])` | `[1, 2, 3]` |
| `sort_by`, `max_by` | `sort_by(["ccc", "a"], len)` | `["a", "ccc"]` |
| `first`, `find`, `get` | `first([])` | `None` |
| `take`, `drop` | `take([1, 2, 3], 2)` | `[1, 2]` |
| `join`, `split` | `join(["a", "b"], "-")` | `"a-b"` |

## Functional style and blocks

```rythme
total = [1, 2, 3, 4, 5, 6]
  |> filter(is_even)
  |> map(fn(x) = x * x)
  |> sum

same = [1, 2, 3, 4, 5, 6]
  .select(even?)
  .map { |x| x * x }
  .sum

[1, 2].each do |x|
  puts "element #{x}"
end
print(total, same)
```

```sortie
element 1
element 2
56 56
```

See [thinking functionally](fonctionnel.md).

## Types and match

```rythme
type Shape = Circle(radius) | Square(side)

fn area(shape) = match shape
  Circle(r) -> round(pi * r * r, 2)
  Square(s) -> s * s
end

fn describe([]) = "empty"
fn describe([x, ..others]) = "{x} then {len(others)} more"

print(area(Circle(1)), area(Square(3)), describe([7, 8, 9]))
```

```sortie
3.14 9 7 then 2 more
```

See [your own types and match](types.md).

## Errors

| Write | Meaning |
|-------|---------|
| `Some(x)`, `None` | a value that is there, missing (Option) |
| `Ok(x)`, `Err(e)` | a success, a failure (Result) |
| `value?` | go on with `x`, or give back `None` / `Err` |
| `unwrap_or(x, default)` | the value, or a default value |
| `rythme explain E005` | explain an error code |

```rythme
fn add_texts(a, b)
  x = parse_int(a)?
  y = parse_int(b)?
  Ok(x + y)
end

print(add_texts("2", "3"), add_texts("2", "three") |> is_err)
print(first([]) |> unwrap_or(0))
```

```sortie
Ok(5) true
0
```

See [handling errors](erreurs.md).

## Keyboard and files

| Write | Gives |
|-------|-------|
| `input("Name? ")` | the line typed on the keyboard (a text) |
| `args()` | the words after the file name |
| `write_file(path, content)` | `Ok(())`: replaces the content |
| `append_file(path, content)` | `Ok(())`: adds at the end |
| `read_file(path)` | `Ok(content)` or `Err(message)` |
| `file_exists(path)` | `true` or `false` |
| `to_json(value)`, `parse_json(content)` | JSON, and back (a Result) |

```rythme
write_file("shopping.txt", "bread\nmilk")
append_file("shopping.txt", "\neggs")
content = read_file("shopping.txt") |> unwrap_or("")
print(lines(content), file_exists("shopping.txt"))
```

```sortie
["bread", "milk", "eggs"] true
```

See [reading and writing: keyboard and files](fichiers.md).

## Modules

| Write | To |
|-------|----|
| `use geometry` | use the file `geometry.ry` (without `.ry`) |
| `geometry.square_area(4)` | a name from the module, after the module's name |
| `use shapes/circle` | the file `shapes/circle.ry`, used as `circle` |
| `use shapes/circle as disc` | the same, under the name `disc` |
| `use ../tools/calculations` | `..` goes up one folder |
| `fn _helper(x) = ...` | a private name: invisible from other files |

```rythme norun
use geometry               # at the top of the file, outside any function
print(geometry.square_area(4))
```

See [modules](modules.md).

## Tests

| Write | To |
|-------|----|
| `test "name" = assert_eq(actual, expected)` | a test on one line |
| `test "name"` ... `end` | a test on several lines |
| `assert(condition, message)` | check that a condition is true |
| `rythme test f.ry` | run the tests (`rythme run` skips them) |
| `rythme test f.ry --filter word` | only the tests whose name contains `word` |
| `rythme test f.ry --watch` | run again after each change |
| `rythme test .` | the tests of all the files in the folder |

```rythme
fn square(x) = x * x

test "square of 3" = assert_eq(square(3), 9)

test "several checks"
  assert(square(-2) > 0, "a square is positive")
  assert_eq(parse_int("12")?, 12)
end

print(square(12))
```

```sortie
144
```

A test fails if it stops on an error, or if it ends with `false`,
`Err(...)` or `None`. See [testing your code](tests.md), then
[organizing a project](projet.md).

## Commands

| Command | What it does |
|---------|--------------|
| `rythme` | the interactive mode |
| `rythme run f.ry` | run a program |
| `rythme check f.ry` | check without running |
| `rythme test f.ry` | run the tests |
| `rythme fmt f.ry` | format the code |
| `rythme doc f.ry` | write the documentation |
| `rythme new name` | create a project |
| `rythme help topic` | the help (`rythme help builtins`: all the functions) |
| `rythme explain E005` | explain an error |
| `rythme learn` | the guided tour |
| `rythme exercise` | the 30 exercises |

## If you come from Ruby

| Ruby | Rythme |
|------|--------|
| `def double(x) x * 2 end` | `fn double(x) = x * 2` |
| `x += 1` | `x2 = x + 1` (a new name) |
| `nil` | `None` (and `Some(x)` for a value) |
| `raise` / `rescue` | `Err(e)` / `match` or `?` |
| `if x then a end` | `if x then a else b` (the `else` is required) |
| `case x when 1 then ... end` | `match x` then `1 -> ...`, closed by `end` |
| `for x in xs` | `xs.each do \|x\| ... end` |
| `class Point` | `{x: 1, y: 2}` or `type Point = Point(x, y)` |
| `{ada: 36}` (a hash) | `dict([[:ada, 36]])` or `{ada: 36}` (a record) |
| `return x` | the last line is the result |
| `x = len xs` | `x = len(xs)` (parentheses inside an expression) |
| `xs.map { \|x\| x * 2 }` | the same |
| `puts "Hi #{name}"` | the same |
| `xs.select(&:even?)` | `xs.select(even?)` |
| `gets` | `input("question ")` |
| `File.read("f.txt")` | `read_file("f.txt")` (a Result) |
| `"42".to_i` | `parse_int("42")` (a Result: the text may not be a number) |

`rythme help ruby` explains what stays the same and what changes. See also
[the Ruby cheat sheet](../ruby/aide-memoire.md) and
[the Rust one](../rust/aide-memoire.md).

> **Remember:** values never change, every `if` has an `else`, the last
> line of a function is its result, and `rythme help` is always there.
