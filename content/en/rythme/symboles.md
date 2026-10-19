# Symbols and when to use them

A symbol like `:done` or `:right` is a small name used as a value: this page shows how to write it, compare it, recognize it with `match`, and when to prefer a text or a `type`.

> **Goals:**
>
> - you will know how to write a symbol and recognize its type, `Symbol`;
> - you will know how to compare symbols and recognize them with `match`;
> - you will know how to store symbols in records and dictionaries;
> - you will know why a symbol is never equal to a text;
> - you will know how to choose between a symbol, a text and a `type`.

## A name used as a value

A [symbol](../debuter/glossaire.md#symbol) is a name preceded by a
colon: `:yes`, `:done`, `:right`. It is not a name that refers to a
value. It is **the value itself**.

Think of a cloakroom token. The token contains nothing: it is only there
to be recognized. A symbol is the same: its only job is to say "it's this
one".

```rythme
answer = :yes
print(answer, type_of(answer))
```

```sortie
:yes Symbol
```

Read the first line as: "the name `answer` refers to the symbol `:yes`".

```text
 answer ──► [ :yes ]
 (a name)   (a value: the symbol yes)
```

`type_of` answers `Symbol`: it is a kind of value of its own, like `Int`
or `Text`.

```rythme
print(type_of(:yes), type_of("yes"), type_of(1))
```

```sortie
Symbol Text Int
```

Try it: replace `:yes` with `:maybe`. Rythme accepts it without any
declaration: a symbol does not need to be created before it is used.

> **Remember:** `:yes` is a ready-made value. The colon is stuck to the
> name.

## Writing a symbol

A symbol follows the same rules as a name: letters, digits and `_`, and
accents are allowed.

```rythme
print(:done, :to_do, :café, :step2)
```

```sortie
:done :to_do :café :step2
```

Three rules to know:

- the colon is **stuck** to the name: `:done`, never `: done`;
- uppercase and lowercase matter: `:Done` and `:done` are two different
  symbols;
- by habit, symbols are written in lowercase, with `_` between words:
  `:in_progress`.

## Comparing symbols

Two symbols are equal when they have the same name. You compare them with
`==` and `!=`, like numbers:

```rythme
answer = :yes
print(answer == :yes, answer == :no, answer != :no)
```

```sortie
true false true
```

| Comparison | Question | Result |
|------------|----------|--------|
| `answer == :yes` | is it the `yes` token? | `true` |
| `answer == :no` | is it the `no` token? | `false` |
| `answer != :no` | is it anything other than `no`? | `true` |

Symbols can also be sorted in the alphabetical order of their names:

```rythme
print(sort([:pear, :apricot, :kiwi]), :apricot < :kiwi)
```

```sortie
[:apricot, :kiwi, :pear] true
```

## A symbol is not a text

`:yes` and `"yes"` look alike, but they are two kinds of values. Rythme
never mixes them up: a symbol is **never** equal to a text.

```rythme
answer = :yes
print(answer == "yes")
```

```sortie
false
```

Rythme warns you about it:

```text
warning[W006]: this comparison is always false: Symbol and Text are never equal
 --> main.ry:2:14
  |
2 | print(answer == "yes")
  |              ^^ Symbol == Text "yes"
  |
  = help: values of different kinds are never equal: Rythme never converts values silently
```

To put a symbol in a text, the `{...}` insertion and `text` both work. The
symbol keeps its colon:

```rythme
print("answer: {:yes}", text(:yes), len(text(:yes)))
```

```sortie
answer: :yes :yes 4
```

`text(:yes)` is the text `":yes"`: four characters, colon included.

When you write [JSON](../debuter/glossaire.md#json) with `to_json`, a
symbol becomes an ordinary text, without the colon:

```rythme
print(to_json({state: :done, count: 2}))
```

```sortie
{"state": "done", "count": 2}
```

> **Remember:** compare a symbol with a symbol: `state == :done`, never
> `state == "done"`.

## Recognizing a symbol with match

A symbol is made to be recognized. `match` compares it with each case,
from top to bottom:

```rythme
fn icon(state) = match state
  :done -> "[x]"
  :in_progress -> "[~]"
  _ -> "[ ]"
end

print(icon(:done), icon(:in_progress), icon(:to_do))
```

```sortie
[x] [~] [ ]
```

Let's follow `icon(:in_progress)` step by step:

| Step | Case tried | Matches? | Result |
|------|------------|----------|--------|
| 1 | `:done` | no | |
| 2 | `:in_progress` | yes | `"[~]"` |

The `_` case catches all the other symbols, and even the other values.

You can also write a function **by cases**, one line per symbol (see
[defining by cases](fonctions.md#defining-by-cases)):

```rythme
fn translate(:yes) = "oui"
fn translate(:no) = "non"
fn translate(_) = "?"

print(translate(:yes), translate(:no), translate(:meh))
```

```sortie
oui non ?
```

Try it: remove the line `fn translate(_) = "?"`. `translate(:meh)` then
stops the program: no case recognizes it.

## Symbols in records

A symbol is ideal to describe the **state** of something: a task done or
not, an order pending or delivered.

```rythme
task = {name: "shopping", state: :to_do}
task2 = {..task, state: :done}
print(task.state, task2.state)
```

```sortie
:to_do :done
```

Notice the two `:` in `state: :done`. The first one separates the name of
the field from its value. The second one starts the symbol.

```text
 { state:   :done }
        ▲   ▲
        │   └── the start of the symbol :done
        └────── "the field state is..."
```

With a list of tasks, you can filter by state:

```rythme
tasks = [
  {name: "shopping", state: :done},
  {name: "cleaning", state: :to_do},
  {name: "homework", state: :in_progress},
  {name: "dishes", state: :done},
]
fn icon(:done) = "[x]"
fn icon(:in_progress) = "[~]"
fn icon(_) = "[ ]"
tasks |> each(fn(t) = print("{icon(t.state)} {t.name}"))
finished = tasks |> filter(fn(t) = t.state == :done)
print("{len(finished)} tasks done out of {len(tasks)}")
```

```sortie
[x] shopping
[ ] cleaning
[~] homework
[x] dishes
2 tasks done out of 4
```

Try it: set the cleaning to `:done`. The last line then prints
`3 tasks done out of 4`.

## Symbols as dictionary keys

A symbol also makes a good
[dictionary](../debuter/glossaire.md#dictionary) key (see
[dictionaries and sets](dictionnaires.md)):

```rythme
stock = dict([[:apple, 3], [:pear, 0], [:kiwi, 5]])
print(stock[:kiwi])
print(get(stock, :banana) |> unwrap_or(0))
print(keys(stock))
```

```sortie
5
0
[:apple, :kiwi, :pear]
```

The dictionary is sorted in the order of the symbols' names: `apple`,
`kiwi`, `pear`.

> **Note:** if you come from Ruby, `{apple: 3}` in Rythme is not a
> dictionary with the key `:apple`. It is a record with a field `apple`.
> For symbol keys, write `dict([[:apple, 3]])`.

## An example: moving on a grid

Symbols describe a **direction** well. Here is a pawn that starts on the
square `{x: 0, y: 0}`:

```rythme
fn move(p, :up) = {..p, y: p.y - 1}
fn move(p, :down) = {..p, y: p.y + 1}
fn move(p, :left) = {..p, x: p.x - 1}
fn move(p, :right) = {..p, x: p.x + 1}

start = {x: 0, y: 0}
p1 = move(start, :right)
p2 = move(p1, :right)
p3 = move(p2, :down)
print(start, p1, p2, p3)
```

```sortie
{x: 0, y: 0} {x: 1, y: 0} {x: 2, y: 0} {x: 2, y: 1}
```

| Step | Direction | x | y |
|------|-----------|---|---|
| start | | 0 | 0 |
| 1 | `:right` | 1 | 0 |
| 2 | `:right` | 2 | 0 |
| 3 | `:down` | 2 | 1 |

Since the path is a list of symbols, `fold` can take all the steps at once
(see [thinking functionally](fonctionnel.md)):

```rythme
fn move(p, :up) = {..p, y: p.y - 1}
fn move(p, :down) = {..p, y: p.y + 1}
fn move(p, :left) = {..p, x: p.x - 1}
fn move(p, :right) = {..p, x: p.x + 1}

path = [:right, :right, :down]
print(fold(path, {x: 0, y: 0}, move))
```

```sortie
{x: 2, y: 1}
```

## Going from a text to a symbol

What the user types is always a text. To turn it into a symbol, write a
function by cases. It gives an [option](../debuter/glossaire.md#option),
because the text may match nothing:

```rythme
fn to_answer("yes") = Some(:yes)
fn to_answer("no") = Some(:no)
fn to_answer(_) = None

print(map(["yes", "no", "meh"], to_answer))
```

```sortie
[Some(:yes), Some(:no), None]
```

Once you have the symbol, the rest of the program only handles `:yes` and
`:no`. Typos are stopped at the entrance.

## Symbol, text or type?

All three can describe a state. Here is how to choose:

| You have... | Choose | Example |
|-------------|--------|---------|
| words to display, or that come from the user | a text | `"Lea"`, `"hello"` |
| a few labels, for a small program | a symbol | `:done`, `:right` |
| a closed list of cases, never to be forgotten | a `type` | `type State = ToDo \| Done` |

A symbol is quick to write. But Rythme does not know the list of your
symbols: a typo goes through silently.

```rythme
fn icon(:done) = "[x]"
fn icon(:in_progress) = "[~]"
fn icon(_) = "[ ]"

task = {name: "homework", state: :in_progres}
print(icon(task.state))
```

```sortie
[ ]
```

An `s` is missing in `:in_progres`. The program says nothing, and prints a
wrong result.

With a [type](types.md), Rythme knows all the cases. A typo is an error,
and a case forgotten in a `match` gives a warning:

```rythme
type State = ToDo | InProgress | Done

fn icon(s) = match s
  ToDo -> "[ ]"
  InProgress -> "[~]"
  Done -> "[x]"
end

print(icon(InProgress), icon(Done))
```

```sortie
[~] [x]
```

Writing `InProgres` here would be error E003, with "did you mean
`InProgress`?".

> **Remember:** a symbol to go fast, a `type` when the cases are known in
> advance and none must be forgotten.

## Common mistakes

### Putting a space after the colon

```rythme norun
state = : done
```

```text
error[E001]: expected an expression, found `:`
 --> main.ry:1:9
  |
1 | state = : done
  |         ^ expected an expression here
  |
  = help: Rythme does not use `:` to start a block; see `rythme help syntax`
  = more about this error: `rythme explain E001`
```

Stick the colon to the name: `state = :done`.

### Forgetting the space in a record

```rythme norun
task = {state:finished}
```

```text
error[E003]: unknown name `finished`
 --> main.ry:1:15
  |
1 | task = {state:finished}
  |               ^^^^^^^^ not defined
  |
  = help: define it before using it: `finished = ...` or `fn finished(...) = ...`
  = more about this error: `rythme explain E003`
```

Right after a field name, `:` separates the field from its value. Here,
the value is the **name** `finished`, which does not exist. Write
`{state: :finished}`, with two `:` and a space between them.

### A match without a case for everything else

```rythme norun
fn icon(state) = match state
  :done -> "[x]"
  :in_progress -> "[~]"
end

print(icon(:done))
print(icon(:to_do))
```

```text
[x]
error[E006]: no case of this `match` handles Symbol :to_do
 --> main.ry:1:24
  |
1 | fn icon(state) = match state
  |                        ^^^^^ this value
  |
  = help: add a case for this value, or a last case `_ -> ...` that handles everything else
  = call trace (most recent first):
      in `icon` called at main.ry:7:7
  = more about this error: `rythme explain E006`
```

The first call works, the second one stops the program. Rythme cannot know
all the possible symbols: add a last case `_ -> "[ ]"`.

### Calculating with a symbol

```rythme norun
print(:apple + :pear)
```

```text
error[E005]: cannot use `+` with Symbol and Symbol
 --> main.ry:1:14
  |
1 | print(:apple + :pear)
  |              ^ Symbol :apple + Symbol :pear
  |
  = more about this error: `rythme explain E005`
```

A symbol is neither a number nor a text: you compare it, you recognize it,
you don't calculate with it. For a text, go through `text(:apple)`.

## Test yourself

**Question 1:** what are the values of `:yes == :yes`, `:yes == :Yes` and
`:yes == "yes"`?

> **Answer**
>
> `true`, `false` and `false`. Uppercase matters, and a symbol is never
> equal to a text.

**Question 2:** what does this program print?

```rythme norun
fn signal(:green) = "go"
fn signal(:red) = "stop"
fn signal(_) = "careful"
print(signal(:red), signal(:orange))
```

> **Answer**
>
> `stop careful`. `:red` has its own line; `:orange` doesn't, so it falls
> on `_`.

**Question 3:** the state of an order can be "pending", "shipped" or
"delivered", and no case must be forgotten. Symbol or `type`?

> **Answer**
>
> A `type`: `type Order = Pending | Shipped | Delivered`. Rythme then
> knows all the cases: it refuses a typo and reports a case forgotten in a
> `match` (W002).

## Summary

- A symbol is written `:` stuck to a name: `:done`. Its type is `Symbol`.
- Two symbols are equal when they have the same name; a symbol is never
  equal to a text.
- `match` and functions by cases recognize symbols; keep a last case `_`.
- In a record, write `{state: :done}`, with a space between the two `:`.
- A symbol to go fast; a `type` when the cases are known and must not be
  forgotten.

Next: [pattern matching](pattern-matching.md)
