# Glossary

This glossary explains in simple words the programming terms you will come across in this documentation, sorted in alphabetical order.

## A

### Algorithm

A series of precise steps to solve a problem, like a recipe.
A program is an algorithm written in a programming language. See
[Thinking like a programmer](algorithmes.md) and [pseudo-code](#pseudo-code).

### Any

In TypeScript, the "anything" type: it **switches off** type
checking. An `any` value accepts everything, even mistakes. Prefer
[unknown](#unknown), which forces you to check before using it.

### API

A list of services that a program offers to other programs. On the
web, these are addresses that answer with data, often in
[JSON](#json), instead of a page. The name stands for *Application
Programming Interface*. See [How the web works](web.md#apis-and-json).

### Argument

The value you give to a [function](#function) when you call it.
In `double(21)`, the argument is `21`.

### ASCII

An old code that gives a number to English letters, digits and
punctuation: `A` is 65, `a` is 97. [Unicode](#unicode) builds on it
and extends it to every writing system.

```ruby
puts "A".ord
puts 97.chr
```

```sortie
65
a
```

### Async

**Asynchronous** code: while a task is waiting (for an answer from the
network, a file), the program takes care of something else instead of staying
stuck. In Python and JavaScript, you write it with `async` and `await`;
in Rust, with `async` and `.await`. See also [promise](#promise) and
[async in Rust](../rust/async.md).

## B

### Binary

The computer's way of counting, with only two digits: 0 and 1.
`101` in binary is 5. See
[How a computer works](ordinateur.md).

```rust
println!("{:b}", 5);
```

```sortie
101
```

### Bit

The smallest piece of information: a `0` or a `1`. Eight bits make
a [byte](#byte).

### Block

A group of instructions that belong together. In Ruby and Rythme, a block
is often written between `do` and `end`; in Rust, between braces `{ }`.

```ruby
3.times do
  puts "Hello"
end
```

```sortie
Hello
Hello
Hello
```

### Boolean

A value that can only be **true** or **false**: `true` or `false`.
It is the result of a comparison like `10 > 3`.

### Borrowing

In Rust, using a value without taking it from its owner, with `&`.
It is also called a **reference**. See [ownership](#ownership).

### Bug

A mistake in a program: it crashes, or it doesn't do what you
wanted. The word means "insect".

### Byte

A group of eight [bits](#bit). It can be worth 0 to 255. File sizes
are measured in bytes: KB (thousand), MB (million), GB (billion).

## C

### Cargo

Rust's tool that creates a project (`cargo new`), compiles and runs it
(`cargo run`), and downloads its [crates](#crate).

### Channel

In Go, a pipe through which [goroutines](#goroutine) send each other
values. You send with `channel <- value` and receive with `<-channel`.

```go
package main

import "fmt"

func main() {
	channel := make(chan string)
	go func() { channel <- "Hello from a goroutine" }()
	fmt.Println(<-channel)
}
```

```sortie
Hello from a goroutine
```

### Class

A model for making [objects](#object) of the same kind. The class
`Dog` describes what a dog is; each dog you create is an object. Ruby
uses classes a lot; Rythme has none.

### Client

The program that **asks** a [server](#server) for something. Your
browser is a client: it asks for web pages.

### Closure

A function without a name that remembers the variables around it, where
it was created.

```ruby
bonus = 5
add_bonus = ->(x) { x + bonus }
puts add_bonus.call(10)
```

```sortie
15
```

### Code editor

The software in which you write code, like VS Code. See
[Installing a code editor](editeur.md).

### Comment

A text in the code that the computer ignores. It is there to explain the code
to humans. In Ruby and Rythme, it starts with `#`; in Rust, with `//`.

```ruby
# This is a comment: nothing is printed.
puts "Only this line counts"
```

```sortie
Only this line counts
```

### Commit

A saved version of your project with [Git](#git), with a message
that says what changed. See
[Saving your work with Git](git.md).

### Compilation

The [compiler](#compiler)'s job: translating all the code into
machine language before running it. In Rust, `cargo build` compiles;
`cargo run` compiles, then runs.

### Compiler

A program that translates all your code into machine language **before**
running it. Go and Rust use a compiler. See also
[interpreter](#interpreter).

### Comprehension

In Python, a short way to build a [list](#list) from
another one, in a single line: "the square of each `n` from 1 to 4".

```python
squares = [n * n for n in range(1, 5)]
print(squares)
```

```sortie
[1, 4, 9, 16]
```

### Concurrency

Making progress on several tasks at the same time, for example with several
[threads](#thread). See [concurrency in Rust](../rust/concurrence.md)
and [in Ruby](../ruby/concurrence.md).

### Condition

A test that decides which code to run: "**if** it rains, take an
umbrella, **otherwise** sunglasses". It is written with `if`.

```ruby
weather = "rain"
if weather == "rain"
  puts "Umbrella"
else
  puts "Sunglasses"
end
```

```sortie
Umbrella
```

### Constant

A named value that must never change, like `PI = 3.14159`. By
habit, its name is often written in capital letters.

### Crate

A Rust [package](#package): a library or a program. You find them
on the `crates.io` website, and [Cargo](#cargo) downloads them.

## D

### Debugging

Finding and fixing a [bug](#bug). You read the error messages, you
print values, you test little by little. See
[Finding and fixing your mistakes](deboguer.md).

### Decorator

In Python, a function that **wraps** another function to
add something to it, without touching its code. You put it above with
`@`.

```python
def announce(function):
    def wrapper():
        print("Start")
        function()
    return wrapper

@announce
def hello():
    print("Hello")

hello()
```

```sortie
Start
Hello
```

### Dictionary

A collection that links a **value** to a **key**, like a real
dictionary links a definition to a word. In Python, it is called a *dict*;
in JavaScript, an *object* or a *Map*; in Go, a *map*; in Ruby, a *hash*; in
Rust, a *HashMap*.

```ruby
ages = { "Ada" => 36, "Alan" => 41 }
puts ages["Ada"]
```

```sortie
36
```

### DOM

The web page as the browser keeps it in memory: a tree
of elements (headings, paragraphs, buttons) that JavaScript can read and
change. The name stands for *Document Object Model*.

```javascript norun
document.querySelector("h1").textContent = "Hello!";
```

## E

### Enum

A [type](#type) that can only take one value from a fixed list:
a traffic light is `Red`, `Yellow` or `Green`. In Rust, you write it with `enum`;
in Rythme, with `type`. The full name is **enumeration**.

### Enumerator

In Ruby, a value that gives the elements of a collection **one by one**,
on demand, with `next`. It is Ruby's cousin of the
[iterator](#iterator). See [enumerators](../ruby/enumerateurs.md).

```ruby
letters = ["a", "b", "c"].each
puts letters.next
puts letters.next
```

```sortie
a
b
```

### Error

A message that says something went wrong. It often shows the
file, the line and the cause. Read it: it is your best clue.

### Exception

In Ruby, an error that stops the program, unless you catch it with
`rescue`. Rust and Rythme don't have any: they use [Result](#result).

```ruby
begin
  Integer("abc")
rescue ArgumentError
  puts "That is not a number."
end
```

```sortie
That is not a number.
```

### Expression

A piece of code that gives a [value](#value). `2 + 3` is an
expression worth `5`; `"Ada"` is an expression worth `"Ada"`.

## F

### Float

A number with a decimal point: `3.14`, `0.5`. In programming, you always use
a **point**, never a comma. The full name is *floating-point number*.

### Function

A piece of code that has a name and that you can reuse. It receives
[arguments](#argument) and returns a result.

```rythme
fn double(x) = x * 2
print(double(21))
```

```sortie
42
```

## G

### Gem

A Ruby [package](#package). You install it with `gem install name`, or you
list it in a project's `Gemfile`.

### Generator

A function that gives its values **one at a time**, as you
ask for them, instead of computing everything at once. In Python and
JavaScript, it uses `yield`.

```python
def countdown(n):
    while n > 0:
        yield n
        n -= 1

for n in countdown(3):
    print(n)
```

```sortie
3
2
1
```

### Generics

Code written once for several [types](#type). In Rust, `Vec<T>`
is a list of elements of any type `T`: `Vec<i32>`,
`Vec<String>`... TypeScript (`Array<T>`) and Go (`[T any]`) have the same
idea.

### Git

A program that keeps the history of your code: each version is a
[commit](#commit), stored in a [repository](#repository). See
[Saving your work with Git](git.md).

### Goroutine

In Go, a function that runs **at the same time** as the rest of the
program. You start it by writing `go` in front of the call. They often
communicate through [channels](#channel). See also [concurrency](#concurrency).

## H

### HTML

The language that describes the **structure** of a web page: headings,
paragraphs, links, images. It is written with tags like `<h1>` and
`</h1>`. See [How the web works](web.md#html-css-and-javascript).

### HTTP

The language that the [client](#client) and the [server](#server) speak on the
web: a **request** (`GET /page`), then a **response** with a code,
like `200` (all good) or `404` (page not found). `HTTPS` is its
encrypted version.

## I

### Immutable

Which cannot change. An immutable value stays the same from start to
finish. In Rythme, all values are immutable; in Rust, variables
are immutable by default. See also [mutable](#mutable).

### Indentation

Shifting a line to the right, with spaces. It shows what
is **inside** a loop or a function. In Python,
indentation is part of the syntax: a badly shifted line is an
error.

### Index

The position of an element in a [list](#list). You count from
**zero**: the first element has index `0`.

```ruby
fruits = ["apple", "pear", "kiwi"]
puts fruits[0]
puts fruits[2]
```

```sortie
apple
kiwi
```

### Inheritance

In Ruby, a [class](#class) can take everything another
class can do, then add its own methods. We say it
**inherits** from it, with `<`.

```ruby
class Animal
  def breathe = "I breathe."
end

class Cat < Animal
end

puts Cat.new.breathe
```

```sortie
I breathe.
```

### Integer

A whole number: `0`, `42`, `-7`. See
also [float](#float).

### Interface

A list of what a value must **be able to do** or **contain**, without
saying how. In Go, a type that has the right methods satisfies
the interface on its own; in TypeScript, an interface describes the shape of an
object.

```typescript
interface Animal {
  name: string;
  sound(): string;
}

const cat: Animal = { name: "Felix", sound: () => "Meow" };
console.log(`${cat.name} says ${cat.sound()}`);
```

```sortie
Felix says Meow
```

### Interpreter

A program that reads your code and runs it right away, line after
line. Python, JavaScript, Ruby and Rythme use an interpreter. See also
[compiler](#compiler).

### Iteration

One pass through a [loop](#loop). Iterating over a list means handling
its elements one by one.

### Iterator

A value that gives the elements of a collection one by one. In Rust,
`.iter()` creates an iterator, on which you chain `map`, `filter` or
`sum`.

```rust
let total: i32 = [1, 2, 3].iter().map(|x| x * 10).sum();
println!("{total}");
```

```sortie
60
```

## J

### JSON

A very common text format for exchanging data between
programs: `{"name": "Ada", "age": 36}`. The keys are always in
double quotes.

```rythme
print(to_json({name: "Ada", age: 36}))
```

```sortie
{"name": "Ada", "age": 36}
```

## L

### Lambda

A [function](#function) without a name, stored in a variable. In Ruby, you
write it with `->`; it is often also a [closure](#closure). See
[procs and lambdas](../ruby/procs-lambdas.md).

```ruby
double = ->(x) { x * 2 }
puts double.call(21)
```

```sortie
42
```

### Library

Code already written by other people, which you can reuse in your
program. It saves you from reinventing everything. See also [package](#package).

### Lifetime

In Rust, the period during which a [reference](#borrowing) stays
valid. The compiler checks that a reference never lives
longer than the value it points to. See
[lifetimes in Rust](../rust/durees-de-vie.md).

### List

An ordered series of values: `[1, 2, 3]`. In Ruby, it is called an *array*
(*Array*); in Rust, a *Vec*. Each element has an [index](#index).

### Literal type

In TypeScript, a type that accepts **only one** precise value, like
`"red"`. With a [union](#union-type), you list the allowed values.

```typescript
type Light = "red" | "yellow" | "green";
const light: Light = "green";
console.log(light);
```

```sortie
green
```

### Loop

An instruction that repeats code several times, for example for each
element of a list. See also [iteration](#iteration).

```ruby
[1, 2, 3].each { |n| puts n * 10 }
```

```sortie
10
20
30
```

## M

### Macro

In Rust, a word that ends with `!`, like `println!` or `vec!`. It writes
code for you, before compilation.

### Match

An instruction that compares a value to several possible shapes and
chooses the code of the first one that fits. Rust and Rythme use it
a lot; Ruby has `case`.

```rythme
grade = 15
print(match grade
  0..9 -> "not enough"
  _ -> "good"
end)
```

```sortie
good
```

### Metaprogramming

Code that writes or changes code while the program is running, for
example to create methods automatically. Ruby allows it a lot.
See [metaprogramming in Ruby](../ruby/metaprogrammation.md).

### Method

A function attached to a value. You call it with a dot:
`"hello".upcase` calls the text's `upcase` method.

```ruby
puts "hello".upcase
```

```sortie
HELLO
```

### Mixin

In Ruby, a [module](#module) whose methods are added to a
[class](#class) with `include`. That way, several classes can share
the same methods.

### Module

A way to group code that belongs together under one name, often in
a separate file. It helps organize a big program.

### Mutable

Which can change. A mutable variable can receive a new value.
In Rust, you mark it with `mut`.

```rust
let mut counter = 1;
counter = counter + 1;
println!("{counter}");
```

```sortie
2
```

## N

### Narrowing

In TypeScript, when a check makes the type of a value **more precise**:
after `typeof x === "string"`, TypeScript knows that `x` is a text.

```typescript
function size(x: string | number): number {
  if (typeof x === "string") {
    return x.length;
  }
  return x;
}

console.log(size("Ada"), size(7));
```

```sortie
3 7
```

### Node.js

A program that runs JavaScript **outside the browser**: in the
terminal, or on a server. You use it with `node main.mjs`.

### npm

JavaScript's [package](#package) manager, shipped with
[Node.js](#node-js). `npm install name` downloads a package; the list of
a project's packages is in the `package.json` file.

### Null

A special value that means "nothing" or "missing". Python
calls it `None`; JavaScript has `null` and `undefined`; Go and Ruby
call it `nil`. Rust and Rythme have no `null`: they use `None`, which forces you
to plan for the case where the value is missing.

## O

### Object

A value that groups data and [methods](#method) together. In Ruby,
everything is an object: even the number `5`.

### Operating system

The program that starts first and manages the whole computer: it shares
the processor and the memory, stores the files and starts the programs.
Examples: Windows, macOS, Linux, Android.

### Operator

A symbol that performs an operation: `+`, `-`, `*`, `/` to calculate, `==`
to compare, `&&` for "and".

### Option

A value that can be present, `Some(value)`, or missing, `None`.
Rust and Rythme use it instead of [null](#null): the program must
handle both cases.

```rythme
print(first([5, 8]), first([]))
```

```sortie
Some(5) None
```

### Ownership

Rust's rule that says each value has **a single owner**.
When the owner goes away, the value is freed. See
[ownership in Rust](../rust/possession.md).

## P

### Package

A [library](#library) ready to install. In Python, you
install it with [pip](#pip); in JavaScript, with [npm](#npm); in Go, with
`go get`. In Ruby, it is called a *gem*; in Rust, a *crate*.

### Panic

In Rust, the sudden stop of a program on an error it cannot
get past, like reading element 10 of a list of 3 elements. The message
starts with `panicked at`.

### Parameter

The name that receives the [argument](#argument) in a function's
definition. In `fn double(x) = x * 2`, the parameter is `x`.

### Path

The address of a file or a folder: `/home/alice/code/hello.rb`. An
**absolute** path starts from the root of the disk; a **relative** path starts
from the folder you are in, and `..` means the parent folder.

### pip

Python's [package](#package) manager. `pip install name`
downloads a package from PyPI, the official catalog. Use it in
a [virtual environment](#virtual-environment).

### Pointer

A value that holds the **address** of another value in memory, like
a sign that shows where something is. In Rust, `&x` and
`Box<T>` are kinds of pointers.

### Process

A [program](#program) that is running. The same program started
twice gives two processes, each with its own memory.

### Processor

The part of the computer that runs the instructions, billions per
second. It only understands machine language. Also called the *CPU*
(*Central Processing Unit*).

### Program

A list of instructions that a computer runs. See
[What is programming?](programmer.md).

### Promise

In JavaScript, a value that **will arrive later**, for example
a server's response. `await` waits until it is
ready. See also [async](#async).

```javascript
const promise = new Promise((resolve) => {
  setTimeout(() => resolve("ready"), 10);
});
console.log(await promise);
```

```sortie
ready
```

### Prototype

In JavaScript, the model object from which another object **inherits** its
methods. When a property is missing, JavaScript looks for it in the
prototype. JavaScript's classes are built on this mechanism. See also
[inheritance](#inheritance).

### Pseudo-code

An algorithm written in plain English, shaped like a program, before
you write it in a real language. See
[Thinking like a programmer](algorithmes.md).

```text
for each number in the list
    if the number is even
        print it
```

## R

### RAM

The fast memory where a program keeps its data **while it is
running**. It is wiped when you turn off the computer, unlike the
disk. The name stands for *Random Access Memory*. See
[How a computer works](ordinateur.md).

### Range

A series of numbers given by its start and its end: `1..10`. In Rust,
`1..10` stops at 9 and `1..=10` goes up to 10.

### Record

In Rythme, a value that groups named fields together:
`{name: "Ada", age: 36}`. You read a field with a dot.

```rythme
person = {name: "Ada", age: 36}
print(person.name)
```

```sortie
Ada
```

### Recursion

When a function calls itself, on a smaller problem, until it reaches
a simple case. Here, we add up the numbers from `n` down to 0:

```rythme
fn total(0) = 0
fn total(n) = n + total(n - 1)
print(total(4))
```

```sortie
10
```

### Regex

Short for **regular expression**: a small language to describe the
shape of a text, in order to search for it, split it or replace it.
`[0-9]+` means "one or more digits". See
[regex in Ruby](../ruby/regex.md) and [in Rythme](../rythme/regex.md).

```rythme
print(find_all("Tel: 06 12, landline: 01 45", "[0-9]+"))
```

```sortie
["06", "12", "01", "45"]
```

### Regular expression

A pattern that describes the shape of a text, to search for it or replace it. It is most often called a [regex](#regex).

### REPL

A mode where you type a line of code and see the result right
away. The name stands for *Read, Eval, Print, Loop*: read the line, evaluate it,
print the result, start again. Examples: `irb` for Ruby, `rythme` for Rythme.

### Repository

A folder tracked by [Git](#git), with all its history. Often shortened to
*repo*. A repository can also be online, on GitHub.

### Result

The result of an operation that can fail: `Ok(value)` on
success, `Err(error)` on failure. Rust and Rythme use it instead
of [exceptions](#exception).

```rythme
print(parse_int("42"), parse_int("abc") |> is_err)
```

```sortie
Ok(42) true
```

## S

### Scope

The area of the code where a variable exists. A variable created in a function
doesn't exist outside of it.

### Serialization

Turning a value from the program into text (or bytes), to
save it in a file or send it over the network. The opposite, reading
that text back, is called **deserialization**. [JSON](#json) is a
serialization format.

### Server

A program (and, by extension, the computer that runs it) that
waits for requests and **answers** them. A web server sends pages to
[clients](#client).

### Set

A collection **without duplicates**, where the order of arrival doesn't matter. An
element is in it, or it isn't.

```rythme
print(set([3, 1, 3, 2]))
```

```sortie
set(1, 2, 3)
```

### Slice

In Go, a list that can grow. `append` adds elements to it.

```go
package main

import "fmt"

func main() {
	grades := []int{12, 15}
	grades = append(grades, 9)
	fmt.Println(grades, len(grades))
}
```

```sortie
[12 15 9] 3
```

### Source file

The text file that contains your code, for example `hello.rb`,
`main.rs` or `hello.ry`.

### Stack and heap

The two places where a program stores its data in memory. The
**stack** is fast and tidy, like a stack of plates: it holds
small values of known size. The **heap** is bigger and more flexible:
it holds data whose size changes, like a list that grows.
Rust makes you think about this difference.

### String

A text, written in quotes: `"Hello"`. The full name is **string of
characters**, but everybody just says **string**.

### Struct

A [type](#type) that groups several named values, its **fields**.
In Rust, you write it with `struct`; in Ruby, `Struct` creates one
quickly. The full name is **structure**.

```rust
struct Person {
    name: String,
    age: u32,
}

let ada = Person { name: String::from("Ada"), age: 36 };
println!("{} is {} years old", ada.name, ada.age);
```

```sortie
Ada is 36 years old
```

### Symbol

In Ruby and Rythme, a fixed name that starts with a colon: `:ok`,
`:red`. It is used as a label, often as a
[dictionary](#dictionary) key.

### Syntax

The writing rules of a language: where to put the parentheses, the
quotes, the keywords. A syntax error stops the program from
starting.

## T

### Terminal

A window where you type commands on the keyboard. See
[The terminal in 10 minutes](terminal.md).

### Test

A small piece of code that checks that some other code works well. If you
change your program, the tests warn you when you break
something.

### Thread

A thread of execution: a part of the program that runs at the same time as
the others. Several threads let you do several things at
once. See [concurrency in Rust](../rust/concurrence.md).

### Trait

In Rust, a list of methods that a [type](#type) promises to be able to do.
The `Display` trait, for example, says that a value can be printed with
`{}`.

### Tuple

A small group of values, possibly of different types, without field
names. In Rust, you read its elements by their position: `.0`, `.1`.

```rust
let point = (3, 7);
println!("x = {}, y = {}", point.0, point.1);
```

```sortie
x = 3, y = 7
```

### Type

The kind of a value: an [integer](#integer), a text, a
[boolean](#boolean), a [list](#list)... The type says what you can do
with the value.

```ruby
puts 42.class
puts "Ada".class
```

```sortie
Integer
String
```

### Typing

The way a language checks [types](#type). **Static**: before
running the program (Go, Rust, TypeScript). **Dynamic**: while
it is running (Python, JavaScript, Ruby, Rythme).

## U

### Unicode

The code that gives a number to **every character** of all the writing systems
in the world, emojis included: `é` is 233, `€` is 8364. It is most
often stored as **UTF-8**.

```rythme
print(char_code("é"), char(8364))
```

```sortie
233 €
```

### Union type

In TypeScript, a type that accepts **several** types, separated by `|`:
`string | number` is "a text or a number". Before using it, you
do some [narrowing](#narrowing).

### Unknown

In TypeScript, the type of an **unknown** value: TypeScript forbids
using it until you have checked what it is. It is the safe
version of [any](#any).

```typescript
const value: unknown = JSON.parse("42");
if (typeof value === "number") {
  console.log(value + 1);
}
```

```sortie
43
```

### URL

The address of a resource on the web:
`https://www.example.com/recipes?servings=4`. It gives the protocol, the
machine, the path and sometimes parameters. See
[How the web works](web.md#the-address-the-url).

## V

### Value

A piece of data: a number, a text, a list, `true`... It is what
a program works with.

### Variable

A name that refers to a [value](#value), like a label stuck on
a box. You use it to find the value again later.

```ruby
first_name = "Ada"
puts first_name
```

```sortie
Ada
```

### Virtual environment

In Python, a folder that holds the [packages](#package) of **a single**
project, apart from the others. You create it with `python3 -m venv .venv`,
then you activate it before using [pip](#pip).

## Summary

- A program works with values: integers, floats, texts, booleans, lists.
- You store values in variables, and you reuse code with functions.
- Conditions choose, loops repeat.
- An interpreter runs code right away, a compiler translates it first.
- You fix a bug by reading the error message: that is debugging.
