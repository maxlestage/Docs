# Advanced text: numbers, columns and reports

This page shows how to write readable numbers with `format_number`, align columns with `pad_left` and `pad_right`, then build a small report step by step from a list of records.

> **Goals:**
>
> - you will know how to write a number for humans: separated thousands, decimals, the
>   decimal separator of a language;
> - you will know how to choose between `format_number`, `text(x, 2)` and `round`;
> - you will know how to pad a text on the left or on the right to align columns;
> - you will know how to compute the width of a column from the data;
> - you will know how to build a report line by line, then glue the lines back together
>   with `join`.

## The goal: a readable report

You have a list of sales, [records](../debuter/glossaire.md#enregistrement)
like `{name: "Apples", qty: 12, price: 0.85}`. Printing them as they are is
correct, but hard to read. By the end of this page, you will know how to
write this:

```text
Product     Qty      Amount
---------------------------
Apples       12      €10.20
Melons        3       €7.50
Peaches      40      €48.00
---------------------------
Total                €65.70
```

Think of a till receipt: each thing has its column, the numbers are
aligned to the right, the total is at the bottom. Each tool first, the
report afterwards.

The basics of text (`split`, `join`, `pad_left`, `repeat`) are in
[working with text](texte.md). This page goes further. To read data from a
file, see [files](fichiers.md).

## Writing a number for humans

`format_number(number, decimals, language)` turns a number into a
[text](../debuter/glossaire.md#chaine-de-caracteres) that is easy to read:

- it separates the thousands;
- it writes **exactly** the number of decimals you ask for;
- it follows the way a language writes numbers: `"fr"` for French.

```rythme
print(format_number(1234567.891, 2, "fr"))
print(format_number(1234567.891, 2))
print(format_number(1234567))
```

```sortie
1 234 567,89
1,234,567.89
1,234,567
```

Without a language, you get the English style: a comma between the
thousands, a point before the decimals. In French, it is the other way
around: a space between the thousands, a comma before the decimals.

The result is a **text**, not a number: you print it, you no longer
calculate with it.

```rythme
pretty = format_number(1500, 2)
print(pretty, type_of(pretty))
```

```sortie
1,500.00 Text
```

> **Remember:** calculate with numbers, and call `format_number` at the
> very end, just before printing.

### The three arguments, one by one

| Call | What it gives | Why |
|------|---------------|-----|
| `format_number(7)` | `7` | no decimals asked for: the number as it is |
| `format_number(7, 2)` | `7.00` | exactly 2 decimals, zeros if needed |
| `format_number(7, 2, "fr")` | `7,00` | the French decimal comma |
| `format_number(2.5, 0)` | `3` | 0 decimals: it rounds |
| `format_number(12345, 0, "de")` | `12.345` | in German, a point between the thousands |

The known languages are `"en"` (the default), `"fr"`, `"de"`, `"es"`,
`"it"`, `"nl"` and `"pt"`.

Try it: replace `"fr"` with `"de"` in the first example. You get
`1.234.567,89`: points between the thousands, a comma before the
decimals.

### It hides the small errors of decimal numbers

A [decimal number](../debuter/glossaire.md#flottant) is not always exact
in a computer (see [the pitfalls](pieges.md)). `format_number` rounds, and
the problem disappears when you print:

```rythme
print(0.1 + 0.2)
print(format_number(0.1 + 0.2, 2))
```

```sortie
0.30000000000000004
0.30
```

It is like a kitchen scale that shows 300 g: it does not show the
milligrams, and that is just fine.

## Rounding: three related tools

Three functions know how to round. They do not give the same thing:

| Tool | Example | Result | It is... |
|------|---------|--------|----------|
| `round(x, 2)` | `round(3.14159, 2)` | `3.14` | a **number** |
| `text(x, 2)` | `text(3.14159, 2)` | `"3.14"` | a text, with a point |
| `format_number(x, 2)` | `format_number(3.14159, 2)` | `"3.14"` | a text, with thousands separators |

```rythme
x = 1234.5
print(round(x, 2), type_of(round(x, 2)))
print(text(x, 2), type_of(text(x, 2)))
print(format_number(x, 2), type_of(format_number(x, 2)))
```

```sortie
1234.5 Float
1234.50 Text
1,234.50 Text
```

Look at the first line: `round` gives a number, and a number does not
keep the zeros at the end. To **print** two decimals, you need a text:
`text(x, 2)` or `format_number`.

Which function should you choose?

```text
 I want to keep calculating .............. round(x, 2)
 I want a plain text (file, JSON) ......... text(x, 2)
 I want to show it to a person ............ format_number(x, 2)
```

> **Remember:** `round` gives a number; `text` and `format_number` give a
> text, ready to be printed.

### Amounts in cents

For money, a good habit: count in **cents**, with
[integers](../debuter/glossaire.md#entier). Integers are always exact. You
divide by 100 only to print:

```rythme
price_cents = [199, 250, 1005]
total = sum(price_cents)
print(total)
print("€" + format_number(total / 100, 2))
```

```sortie
1454
€14.54
```

The integer `1454` is exact; it only becomes a decimal number (`14.54`),
then a text (`"€14.54"`), at the very last moment.

## Padding a text to align it

A well-aligned column is made of texts that **all have the same width**.
You fill them up with spaces.

- `pad_right(text, width)` adds spaces **on the right**: the text stays
  against the left edge. For words.
- `pad_left(text, width)` adds spaces **on the left**: the text is pushed
  to the right. For numbers.

To **see** the spaces, let's pad with dots (the third argument chooses the
character):

```rythme
print(pad_right("Apples", 10, "."))
print(pad_left("12", 5, "."))
print(pad_left("12", 5) + "|")
```

```sortie
Apples....
...12
   12|
```

Like a ruler: you start from the length of the text, and you add just what
is needed to reach the width.

```text
pad_right("Apples", 10)          pad_left("12", 5)
 A p p l e s · · · ·              · · · 1 2
 └─── 6 ───┘ └─ 4 ─┘              └─ 3 ─┘ └2┘
 └────── 10 ───────┘              └─── 5 ───┘
```

### Why numbers go to the right

Aligned to the right, units are under units and tens under tens. You can
compare them at a glance:

```rythme
numbers = [5, 120, 42]
numbers |> each(fn(n) = print(pad_right(n, 5) + "|" + pad_left(n, 5) + "|"))
```

```sortie
5    |    5|
120  |  120|
42   |   42|
```

On the left, `pad_right`: the digits are not under each other. On the
right, `pad_left`: the column reads like an addition written out at school.

Try it: replace `5` with `2` in `pad_left(n, 5)`. The number `120` already
has 3 characters: it sticks out, and the column shifts. The next section
solves this problem.

> **Remember:** words on the left with `pad_right`, numbers on the right
> with `pad_left`.

### A text that is too long is not cut

If the text is already longer than the width, `pad_left` and `pad_right`
give it back **as it is**. They never cut:

```rythme
print("[" + pad_right("Grapefruits", 8) + "]")
print("[" + take("Grapefruits", 8) + "]")
```

```sortie
[Grapefruits]
[Grapefru]
```

To cut, use `take(text, n)`: it keeps the first `n` characters. A small
function does both, cut then pad:

```rythme
fn fit(t, width) = pad_right(take(t, width), width)

["Kiwi", "Grapefruits"] |> each(fn(w) = print("[" + fit(w, 8) + "]"))
```

```sortie
[Kiwi    ]
[Grapefru]
```

## Choosing the width of a column

Rather than guessing the width, let's compute it: it is the length of the
**longest** text in the column.

```rythme
names = ["Apples", "Melons", "Peaches"]
width = names |> map(len) |> max
print(width)
names |> each(fn(n) = print(pad_right(n, width) + " |"))
```

```sortie
7
Apples  |
Melons  |
Peaches |
```

Let's follow the calculation of `width`, one step at a time:

| Step | Value |
|------|-------|
| `names` | `["Apples", "Melons", "Peaches"]` |
| `\|> map(len)` | `[6, 6, 7]` |
| `\|> max` | `7` |

Try it: add `"Tangerines"` to the `names` list. The width goes up to 10,
and the `|` bars stay aligned.

> **Remember:** the width of a column = the length of its longest text:
> `texts |> map(len) |> max`.

## Building a report step by step

Let's get to the report from the beginning. We follow a recipe in five
steps, like assembling a piece of furniture: one part at a time, checking
at each step.

```text
 1. a function for ONE line
 2. map: one line for each sale
 3. the total
 4. the header, the rules and the footer
 5. join: glue everything back with "\n", and a single print
```

### Step 1: a single line

First we write a [function](../debuter/glossaire.md#fonction) that turns
**one** sale into **one** line of text. And a small `euros` function for
the amounts:

```rythme
fn euros(x) = "€" + format_number(x, 2)

fn row(s) =
  pad_right(s.name, 10) + pad_left(s.qty, 5) + pad_left(euros(s.qty * s.price), 12)

print(row({name: "Apples", qty: 12, price: 0.85}))
print(row({name: "Melons", qty: 3, price: 2.5}))
```

```sortie
Apples       12      €10.20
Melons        3       €7.50
```

Let's break the first line down, piece by piece:

| Piece | Calculation | Text you get | Width |
|-------|-------------|--------------|-------|
| name | `pad_right("Apples", 10)` | `"Apples    "` | 10 |
| quantity | `pad_left(12, 5)` | `"   12"` | 5 |
| amount | `12 * 0.85` → `10.2` → `euros(10.2)` | `"€10.20"` | 6 |
| aligned amount | `pad_left("€10.20", 12)` | `"      €10.20"` | 12 |
| the line | the three glued with `+` | `"Apples       12      €10.20"` | 27 |

`pad_left` also accepts a number, like `12`: it turns it into a text
first.

### Step 2: one line for each sale

There are no loops in Rythme: `map` applies `row` to each sale and gives
the list of lines.

```rythme
fn euros(x) = "€" + format_number(x, 2)
fn row(s) =
  pad_right(s.name, 10) + pad_left(s.qty, 5) + pad_left(euros(s.qty * s.price), 12)

sales = [
  {name: "Apples", qty: 12, price: 0.85},
  {name: "Melons", qty: 3, price: 2.5},
  {name: "Peaches", qty: 40, price: 1.2},
]
body = sales |> map(row)
print(len(body))
print(body[2])
```

```sortie
3
Peaches      40      €48.00
```

| Round of `map` | `s.name` | `s.qty * s.price` | Line produced |
|----------------|----------|-------------------|---------------|
| 1 | `"Apples"` | `10.2` | `Apples       12      €10.20` |
| 2 | `"Melons"` | `7.5` | `Melons        3       €7.50` |
| 3 | `"Peaches"` | `48.0` | `Peaches      40      €48.00` |

### Step 3: the total

`sum` with a function adds up what the function gives for each sale:
`total = sales |> sum(fn(s) = s.qty * s.price)`. The complete program, in
the next step, computes it like this:

| Sale | Amount | Total so far |
|------|--------|--------------|
| (start) | | `0` |
| Apples | `10.2` | `10.2` |
| Melons | `7.5` | `17.7` |
| Peaches | `48.0` | `65.7` |

### Steps 4 and 5: assemble and glue back

We build the header, a rule and the footer with the **same widths** (10, 5
and 12). Then we put all the lines in a single list, and
`join(list, "\n")` glues them back with a line break between each one.

```rythme
fn euros(x) = "€" + format_number(x, 2)
fn row(s) =
  pad_right(s.name, 10) + pad_left(s.qty, 5) + pad_left(euros(s.qty * s.price), 12)

sales = [
  {name: "Apples", qty: 12, price: 0.85},
  {name: "Melons", qty: 3, price: 2.5},
  {name: "Peaches", qty: 40, price: 1.2},
]
body = sales |> map(row)
total = sales |> sum(fn(s) = s.qty * s.price)

header = pad_right("Product", 10) + pad_left("Qty", 5) + pad_left("Amount", 12)
rule = repeat("-", 27)
footer = pad_right("Total", 15) + pad_left(euros(total), 12)

report = [header, rule, ..body, rule, footer] |> join("\n")
print(report)
```

```sortie
Product     Qty      Amount
---------------------------
Apples       12      €10.20
Melons        3       €7.50
Peaches      40      €48.00
---------------------------
Total                €65.70
```

`..body` puts the lines of `body` **at that spot** in the list, one by
one. Without the two dots, `body` would be a list inside the list.

```text
[ header, rule, ..body,                         rule, footer ]
                   │
                   ▼
[ header, rule, line1, line2, line3,            rule, footer ]
                   │
          join("\n")
                   ▼
"header\nrule\nline1\nline2\nline3\nrule\nfooter"
```

The rule is 27 characters long: 10 + 5 + 12, the sum of the widths. The
footer uses 15 for the word "Total": it takes the place of the name **and**
of the quantity (10 + 5).

Try it: add a fourth sale, `{name: "Kiwis", qty: 7, price: 0.4}`. One more
line appears, and the total becomes `€68.50`. You have nothing else to
change.

> **Remember:** a function for one line, `map` for all the lines,
> `join("\n")` to make a single text out of them.

## Gluing lines and words back together

`join` is the opposite of `split` and `lines`. The separator chooses what
goes **between** the pieces, never at the beginning or at the end:

```rythme
parts = ["a", "b", "c"]
print(join(parts, ", "))
print(join(parts, ""))
print(join(parts, "\n"))
```

```sortie
a, b, c
abc
a
b
c
```

`join` also accepts a list of numbers: `join([1, 2, 3], " + ")` gives
`"1 + 2 + 3"`.

### A list in English: "a, b and c"

In English, you write "apples, kiwis and pineapples": commas, except before
the last word. We split the problem into three cases, with a
[definition by cases](fonctions.md#defining-by-cases):

```rythme
fn human_list([]) = ""
fn human_list([only]) = only
fn human_list(words) =
  join(take(words, len(words) - 1), ", ") + " and " + unwrap(last(words))

print(human_list(["apples", "kiwis", "pineapples"]))
print(human_list(["apples", "kiwis"]))
print(human_list(["apples"]))
```

```sortie
apples, kiwis and pineapples
apples and kiwis
apples
```

For `["apples", "kiwis", "pineapples"]`:

| Step | Calculation | Value |
|------|-------------|-------|
| 1 | `take(words, 2)` | `["apples", "kiwis"]` |
| 2 | `join(..., ", ")` | `"apples, kiwis"` |
| 3 | `unwrap(last(words))` | `"pineapples"` |
| 4 | `"apples, kiwis" + " and " + "pineapples"` | `"apples, kiwis and pineapples"` |

`last` gives an [Option](../debuter/glossaire.md#option) (the list could be
empty); here, we know it is not, so `unwrap` opens it.

## Common mistakes

### Giving a text to format_number

The pieces from a `split` are **texts**, even if they contain digits:

```rythme norun
print(format_number("1234"))
```

```text
error[E005]: `format_number` expects a number as argument 1, but got Text "1234"
 --> main.ry:1:21
  |
1 | print(format_number("1234"))
  |                     ^^^^^^ wrong kind of value
  |
  = help: usage: `format_number(x, decimals, language)`
  = more about this error: `rythme explain E005`
```

Read the number first: `parse_float("1234") |> unwrap_or(0.0)`, then give
it to `format_number`.

### Padding with several characters

```rythme norun
print(pad_left(7, 3, "00"))
```

```text
error[E023]: `pad_left` needs a single character, but "00" has 2
 --> main.ry:1:7
  |
1 | print(pad_left(7, 3, "00"))
  |       ^^^^^^^^^^^^^^^^^^^^
  |
  = more about this error: `rythme explain E023`
```

The padding is **one** character, repeated as many times as needed:
`pad_left(7, 3, "0")` gives `"007"`.

### The largest width of an empty list

```rythme norun
names = []
width = names |> map(len) |> max
print(width)
```

```text
error[E023]: `max` of an empty list
 --> main.ry:2:9
  |
2 | width = names |> map(len) |> max
  |         ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = more about this error: `rythme explain E023`
```

An empty list has no largest element. Add a starting value, for example the
length of the column's title:
`[len("Product"), ..(names |> map(len))] |> max` is 7 even if `names` is
empty. The title has to fit in the column anyway.

## Test yourself

**Question 1:** what does `format_number(9876.5, 2)` give? And is it a
number?

> **Answer**
>
> The text `"9,876.50"`: a comma between the thousands, a point, and
> exactly two decimals. It is a **text** (`Text`): you print it, you no
> longer calculate with it.

**Question 2:** what do `pad_left("abc", 6, "*")` and
`pad_right("abc", 2)` give?

> **Answer**
>
> `"***abc"`: three stars on the left to reach 6 characters. And `"abc"`:
> the text is already longer than 2, it is given back as it is, without
> being cut.

**Question 3:** you have a list `rows` of texts. How do you print it
with one line per element, in a single `print`?

> **Answer**
>
> `print(join(rows, "\n"))`: `join` puts a line break **between**
> each element, then `print` prints the whole thing.

## Summary

- `format_number(x, decimals)` writes a number for humans: `1,234.50`
  (`"fr"` gives `1 234,50`). It gives a text: call it just before printing.
- `round` gives a number; `text(x, 2)` and `format_number` give a text. For
  money, count in cents and divide by 100 at the end.
- `pad_right` aligns words to the left, `pad_left` aligns numbers to the
  right; they never cut: `take` cuts.
- The width of a column is the length of its longest text:
  `map(len) |> max`.
- A report: a function for one line, `map` for all of them, then
  `join("\n")` and a single `print`.

Next: [regular expressions](regex.md).
