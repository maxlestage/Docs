# Learning to program well

You learn to program by programming, a little every day: this page gives you a method, project ideas, tips for reading documentation, asking questions, using AI, and a four-week plan.

> **Goals:**
>
> - follow a simple method: read, type, change, break;
> - organize your practice in short, regular sessions;
> - find the answer in the official documentation;
> - ask a good question, and use an AI assistant without missing out on learning;
> - follow a four-week plan in the language of your choice.

## Programming is learned like an instrument

Nobody learns the piano by watching videos. You have to put your
fingers on the keyboard, make mistakes, start again.

Programming is the same: reading this site is not enough. Your
fingers and your mistakes are what teach you. The good news: twenty
minutes a day is enough to make fast progress.

## The method in four steps

For each example on this site, do these four steps, in order.

### 1. Read

Read the code **before** running it, and guess what it will print.
Write your answer on a piece of paper. Take this Ruby example:

```ruby
1.upto(3) do |n|
  puts "3 x #{n} = #{3 * n}"
end
```

What does it print? Think, then look:

```sortie
3 x 1 = 3
3 x 2 = 6
3 x 3 = 9
```

If you got it wrong, great: you just learned something.

### 2. Type the code yourself

Don't copy and paste. **Type** the code, letter by letter, in your
[editor](editeur.md). It is slower, but your fingers remember the
syntax: where the parentheses, the quotes, the `end` go.

### 3. Change

Change just one thing, and guess the result before running it again. For
example, in Rust, count backwards with `.rev()`:

```rust
for n in (1..=3).rev() {
    println!("3 x {n} = {}", 3 * n);
}
```

```sortie
3 x 3 = 9
3 x 2 = 6
3 x 1 = 3
```

Ideas for changes: change a number, add a line, use another
multiplication table, go up to 10.

### 4. Break

Remove a piece on purpose, and read the error message. In Rythme,
let's remove the last parenthesis:

```rythme norun
each(1..3, fn(n) = print("3 x {n} = {3 * n}")
```

```text
error[E001]: expected `,` or `)`, found the end of the code
 --> main.ry:2:1
  |
2 | 
  | ^ expected `,` or `)` here
  |
1 | each(1..3, fn(n) = print("3 x {n} = {3 * n}")
  |     - opened here
  |
  = more about this error: `rythme explain E001`
```

The day you make this mistake without meaning to, you will recognize the
message right away. Here is the fixed version:

```rythme
each(1..3, fn(n) = print("3 x {n} = {3 * n}"))
```

```sortie
3 x 1 = 3
3 x 2 = 6
3 x 3 = 9
```

> **Remember:** read, type, change, break. Errors are not
> a failure: they are your best lessons. See also
> [Finding and fixing your mistakes](deboguer.md).

## Practice a little every day

The brain remembers better what it sees several times, with breaks
between reviews. This is **spaced practice**.

- **Twenty to thirty minutes a day** is better than five hours on
  Sunday.
- **Come back to a chapter** two days later, then a week
  later: redo its examples from memory, without looking.
- **Start each session** by rewriting a small program from the day before
  from memory.
- **Stop** when you are tired: the brain keeps sorting things out
  while you rest and sleep.

> **Tip:** the "Test yourself" sections of each chapter are made
> for reviewing. Answer without looking, then check.

## Small projects, by level

A project of your own motivates you much more than an exercise. Pick a small one,
finish it, then move on to the next.

| Level         | Project ideas                                                           |
|---------------|-------------------------------------------------------------------------|
| Total beginner | a multiplication table; a euros ↔ dollars converter; a vowel counter for a sentence |
| Beginner      | a "guess the number" game; a shopping list; an average of grades       |
| Intermediate  | an address book saved in a file; a password generator; a word counter for a book |
| Advanced      | a small terminal game (hangman, tic-tac-toe); a command-line tool; a small website or an API |

The project pages of each language guide you step by step:
[Python](../python/projets.md), [JavaScript](../javascript/projets.md),
[TypeScript](../typescript/projets.md), [Go](../go/projets.md),
[Ruby](../ruby/projets.md), [Rust](../rust/projets.md) and
[Rythme](../rythme/projets.md).

## Reading the documentation

The official documentation is the most accurate source. It is a bit
scary at first; get into the habit of going there early.

- **Python**: [docs.python.org](https://docs.python.org/3/) has the
  tutorial and the reference. The command `python3 -m pydoc str` shows it in the
  terminal.
- **JavaScript**: [MDN](https://developer.mozilla.org/en-US/) describes the whole
  language, with examples to try.
- **TypeScript**: [typescriptlang.org/docs](https://www.typescriptlang.org/docs/)
  contains the official handbook and a *playground* in the browser.
- **Go**: [go.dev/doc](https://go.dev/doc/) and
  [pkg.go.dev/std](https://pkg.go.dev/std) describe the language and its
  standard library. `go doc strings.ToUpper` shows it in the terminal.
- **Ruby**: [ruby-doc.org](https://ruby-doc.org/) describes each class.
  Search for `String`, for example, to see all the methods for texts.
- **Rust**: [doc.rust-lang.org/std](https://doc.rust-lang.org/std/)
  describes the standard library. The command `rustup doc --std` opens it
  even without Internet.
- **Rythme**: the command `rythme help` lists the topics, and
  `rythme help` followed by a name explains a function.

```sh
rythme help reverse
```

```text
reverse(x)
  The list or text in reverse order.
  Example:
    reverse([1, 2, 3])         # [3, 2, 1]
```

The code itself can also tell you things. In Ruby, `methods` lists what
a value can do:

```ruby
p "hello".methods.grep(/case/).sort
puts 5.respond_to?(:even?)
```

```sortie
[:casecmp, :casecmp?, :downcase, :downcase!, :swapcase, :swapcase!, :upcase, :upcase!]
true
```

> **Tip:** on a documentation page, read the **example** first.
> It often tells you more than the description.

## Asking a good question

Stuck for twenty minutes? Ask for help. A good
question gets a quick answer. It contains:

1. **what you want to do**: "I want to print the even numbers
   of a list";
2. **the code**, as short as possible, that shows the problem (copied as
   text, not as a photo);
3. **the full error message**, copied as is;
4. **what you have already tried**.

Often, while writing the question, you find the answer yourself. That is
the [rubber duck](deboguer.md) method.

## Communities that can help

You are not alone. People help for free:

- **The freeCodeCamp forum** (forum.freecodecamp.org): a friendly forum
  for beginners, for every language;
- **Stack Overflow** (stackoverflow.com): questions and answers for each
  language; search before you ask, your question has often been answered already;
- **The language forums**: discuss.python.org for Python,
  users.rust-lang.org for Rust, with sections for beginners;
- **Local meetups**: Ruby, Python and Rust groups in many
  big cities, often announced on meetup.com;
- **Rythme**: questions and bugs go on the project's GitHub page.

Some great resources exist only in other languages. An online translator helps you
read them: don't hold back.

## Using an AI assistant wisely

An AI assistant (a chatbot that writes code) can be an excellent
teacher, or stop you from learning. It all depends on how you
use it.

**Good ways:**

- ask it to **explain** an error message or a piece of code;
- ask it for a **hint**, not the solution: "give me a lead,
  without the code";
- have it **review** your code once it works, to learn to
  write it better;
- ask it to **ask you questions** to check that you
  understood.

**Bad ways:**

- copying and pasting its answer without understanding it;
- having it do the exercises: you are the one who must improve, not it.

> **Warning:** an AI is sometimes wrong with great confidence. It
> can invent a method that doesn't exist. Always test its code, and
> check in the documentation.

The golden rule: **if you can't explain every line, don't
use it.**

## A four-week plan

Choose **one** language (see [Which language should you choose?](choisir.md)) and
follow its plan. Count on four to five sessions per week. At the end of
each week, do one of the [small projects](#small-projects-by-level).

### In Python

| Week    | Chapters                                                                                                    | At the end, you can…                    |
|---------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1       | [Introduction](../python/introduction.md), [First steps](../python/premiers-pas.md), [Numbers](../python/nombres.md), [Conditions and loops](../python/conditions-boucles.md) | calculate, choose, repeat |
| 2       | [Functions](../python/fonctions.md), [Text](../python/texte.md), [Collections](../python/collections.md), [Dictionaries](../python/dictionnaires.md) | store data and transform it |
| 3       | [Comprehensions](../python/comprehensions.md), [Errors](../python/erreurs.md), [Files](../python/fichiers.md), [Exercises](../python/exercices.md) | read a file and handle errors |
| 4       | [Classes and types](../python/types.md), [Modules](../python/modules.md), [Tests](../python/tests.md), [Guided project](../python/projet.md) | finish a real small program |

### In JavaScript

| Week    | Chapters                                                                                                    | At the end, you can…                    |
|---------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1       | [Introduction](../javascript/introduction.md), [First steps](../javascript/premiers-pas.md), [Numbers](../javascript/nombres.md), [Conditions and loops](../javascript/conditions-boucles.md) | calculate, choose, repeat |
| 2       | [Functions](../javascript/fonctions.md), [Text](../javascript/texte.md), [Arrays](../javascript/collections.md), [Objects](../javascript/objets.md) | store data and transform it |
| 3       | [Errors](../javascript/erreurs.md), [Asynchronous code](../javascript/asynchrone.md), [The DOM](../javascript/dom.md), [Exercises](../javascript/exercices.md) | wait for an answer and bring a page to life |
| 4       | [Modules](../javascript/modules.md), [Files](../javascript/fichiers.md), [Tests](../javascript/tests.md), [Guided project](../javascript/projet.md) | finish a real small program |

### In TypeScript

Start TypeScript after the first two weeks of JavaScript.

| Week    | Chapters                                                                                                    | At the end, you can…                    |
|---------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1       | [Introduction](../typescript/introduction.md), [First steps](../typescript/premiers-pas.md), [Conditions and loops](../typescript/conditions-boucles.md), [Functions](../typescript/fonctions.md) | give types to variables and functions |
| 2       | [Text](../typescript/texte.md), [Collections](../typescript/collections.md), [Objects](../typescript/objets.md), [Unions](../typescript/unions.md) | describe the shape of your data |
| 3       | [Types](../typescript/types.md), [Generics](../typescript/generiques.md), [Errors](../typescript/erreurs.md), [Asynchronous code](../typescript/asynchrone.md) | write reusable, safe code |
| 4       | [Modules](../typescript/modules.md), [Configuration](../typescript/configuration.md), [Tests](../typescript/tests.md), [Guided project](../typescript/projet.md) | finish a real small program |

### In Go

| Week    | Chapters                                                                                                    | At the end, you can…                    |
|---------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1       | [Introduction](../go/introduction.md), [First steps](../go/premiers-pas.md), [Numbers](../go/nombres.md), [Conditions and loops](../go/conditions-boucles.md) | compile, calculate, choose, repeat |
| 2       | [Functions](../go/fonctions.md), [Text](../go/texte.md), [Slices](../go/collections.md), [Maps](../go/maps.md) | store data and transform it |
| 3       | [Types](../go/types.md), [Interfaces](../go/interfaces.md), [Pointers](../go/pointeurs.md), [Errors](../go/erreurs.md) | model data and handle errors |
| 4       | [Modules](../go/modules.md), [Files](../go/fichiers.md), [Tests](../go/tests.md), [Guided project](../go/projet.md) | finish a real small program |

### In Ruby

| Week    | Chapters                                                                                                    | At the end, you can…                    |
|---------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1       | [Introduction](../ruby/introduction.md), [First steps](../ruby/premiers-pas.md), [Numbers](../ruby/nombres.md), [Conditions and loops](../ruby/conditions-boucles.md) | calculate, choose, repeat               |
| 2       | [Functions](../ruby/fonctions.md), [Text](../ruby/texte.md), [Collections](../ruby/collections.md), [Ranges](../ruby/intervalles.md) | store data and transform it             |
| 3       | [Blocks](../ruby/blocs.md), [Errors](../ruby/erreurs.md), [Files](../ruby/fichiers.md), [Exercises](../ruby/exercices.md) | read a file and handle errors           |
| 4       | [Types and classes](../ruby/types.md), [Tests](../ruby/tests.md), [Guided project](../ruby/projet.md)       | finish a real small program             |

### In Rust

| Week    | Chapters                                                                                                    | At the end, you can…                    |
|---------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1       | [Introduction](../rust/introduction.md), [First steps](../rust/premiers-pas.md), [Numbers](../rust/nombres.md), [Conditions and loops](../rust/conditions-boucles.md) | compile, calculate, choose, repeat      |
| 2       | [Functions](../rust/fonctions.md), [Text](../rust/texte.md), [Collections](../rust/collections.md), [Ownership](../rust/possession.md) | understand who owns a value             |
| 3       | [Types](../rust/types.md), [Pattern matching](../rust/pattern-matching.md), [Errors](../rust/erreurs.md), [Iterators](../rust/iterateurs.md) | model data and handle errors            |
| 4       | [Modules](../rust/modules.md), [Files](../rust/fichiers.md), [Tests](../rust/tests.md), [Guided project](../rust/projet.md) | finish a real small program             |

### In Rythme

| Week    | Chapters                                                                                                    | At the end, you can…                    |
|---------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1       | [Introduction](../rythme/introduction.md), [First steps](../rythme/premiers-pas.md), [Numbers](../rythme/nombres.md), [Conditions and loops](../rythme/conditions-boucles.md) | calculate, choose, repeat               |
| 2       | [Functions](../rythme/fonctions.md), [Text](../rythme/texte.md), [Collections](../rythme/collections.md), [The functional style](../rythme/fonctionnel.md) | transform lists without loops           |
| 3       | [Recursion](../rythme/recursion.md), [Pattern matching](../rythme/pattern-matching.md), [Errors](../rythme/erreurs.md), [Files](../rythme/fichiers.md) | break a problem down and handle errors  |
| 4       | [Types](../rythme/types.md), [Tests](../rythme/tests.md), [Exercises](../rythme/exercices.md), [Guided project](../rythme/projet.md) | finish a real small program             |

> **Note:** taking two weeks instead of one is not a problem. A
> regular rhythm matters more than speed.

## Common mistakes

### Copying code with curly quotes

Code copied from a word processor or a web page sometimes contains
curly quotes `“ ”` instead of straight quotes `"`:

```ruby norun
puts “Hello”
```

```text
main.rb:1:in `<main>': undefined local variable or method `“Hello”' for main (NameError)
```

In Rythme, the message tells you directly what to do:

```rythme norun
print(“Hello”)
```

```text
error[E018]: unexpected character `“`
 --> main.ry:1:7
  |
1 | print(“Hello”)
  |       ^
  |
  = help: use straight double quotes `"` for texts
  = more about this error: `rythme explain E018`
```

Type the code yourself: it is the best protection.

### Trusting a made-up method

An AI assistant, or an unreliable website, suggests a method that
seems logical, but doesn't exist:

```ruby norun
puts "hello world".capitalize_words
```

```text
main.rb:1:in `<main>': undefined method `capitalize_words' for an instance of String (NoMethodError)

puts "hello world".capitalize_words
                  ^^^^^^^^^^^^^^^^^
Did you mean?  capitalize
```

Check in the documentation. Here, you capitalize each word
with `"hello world".split.map(&:capitalize).join(" ")`.

### Mixing up languages

After reading Ruby or Python, it is easy to forget the `!` of
`println!` in Rust:

```rust norun
fn main() {
    println("Hello");
}
```

```text
error[E0423]: expected function, found macro `println`
 --> main.rs:2:5
  |
2 |     println("Hello");
  |     ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
2 |     println!("Hello");
  |            +
```

Learn one language at a time, and keep its cheat sheet at hand:
[Python](../python/aide-memoire.md), [JavaScript](../javascript/aide-memoire.md),
[TypeScript](../typescript/aide-memoire.md), [Go](../go/aide-memoire.md),
[Ruby](../ruby/aide-memoire.md), [Rust](../rust/aide-memoire.md),
[Rythme](../rythme/aide-memoire.md).

## Test yourself

**1.** What are the four steps of the method, in order?

> **Answer**
>
> Read (and guess the result), type the code yourself, change it,
> break it on purpose to read the error.

**2.** Is it better to program for four hours on Saturday or thirty minutes
every day?

> **Answer**
>
> Thirty minutes every day: spaced practice helps the brain
> remember, and regular reviews lock in what you have learned.

**3.** An AI assistant gives you code that works, but you don't
understand two lines. What do you do?

> **Answer**
>
> You ask it to explain them, you check in the documentation,
> then you rewrite the code yourself. No line you can't
> explain.

## Summary

- You learn by practicing: read, type, change, break.
- A little every day, with spaced reviews, is better than one long session.
- Small finished projects motivate you more than big abandoned ones.
- The official documentation (docs.python.org, MDN, go.dev, ruby-doc.org, doc.rust-lang.org/std, `rythme help`…) is your best friend.
- A good question contains the goal, the short code, the full error and what you tried.
- AI helps you understand; it must not do the work for you.

Next: [Which language should you choose?](choisir.md).
