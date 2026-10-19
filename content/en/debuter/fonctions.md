# Functions, without the jargon

A function is a little machine that you give a name: you hand it ingredients, it does its job and gives back a result.

> **Goals:**
>
> - understand what a function is, with the picture of a machine;
> - define a function, then call it;
> - give parameters to a function;
> - tell the difference between `return` and `print`;
> - use a function inside another one;
> - know that a variable created in a function stays inside it.

## A function is a machine

Think of a toaster. You put in bread (the **input**). It does its
job. It gives back toast (the **output**). You don't need to know how
it works inside.

A [function](glossaire.md#function) works the same way:

```text
            ┌────────────────┐
   3   ───► │     double     │ ───►  6
 input      │  (times two)   │      output
            └────────────────┘
```

You can also see it as a **recipe**: it has a name ("pancakes"), a list
of ingredients, and steps. You write it once. After that, you just say
"make pancakes".

> **Remember:** a function has a name, receives inputs and gives back an
> output.

## Define, then call

There are two clearly separate moments:

1. **defining** the function: writing the recipe in the notebook;
2. **calling** the function: cooking with the recipe.

In Python, you define with `def`:

```python
def say_hello():
    print("Hello!")
    print("How are you?")

say_hello()
say_hello()
```

```sortie
Hello!
How are you?
Hello!
How are you?
```

- `def say_hello():` gives the recipe a name;
- the **indented** lines are the steps of the recipe;
- `say_hello()`, with the parentheses, **runs** the recipe.

### In what order Python reads the program

Defining runs **nothing**. Python stores the recipe and moves on.

| Step | Line read | What happens |
|------|-----------|--------------|
| 1 | `def say_hello():` | Python remembers the recipe, prints nothing |
| 2 | `say_hello()` | jumps into the function, prints the 2 lines |
| 3 | return | comes back right after the call |
| 4 | `say_hello()` | jumps into the function again, prints the 2 lines |

Try it: delete the two calls. The program doesn't print anything anymore.
The recipe exists, but nobody is cooking.

> **Remember:** `def` writes the recipe. The parentheses `()` make it
> work.

## Parameters and arguments

A recipe can change depending on what you give it. To greet someone,
you need their first name.

```python
def greet(name):
    print("Hello", name)

greet("Léa")
greet("Tom")
```

```sortie
Hello Léa
Hello Tom
```

Two words, two places:

- `name` is the [parameter](glossaire.md#parameter): an empty box, in
  the definition;
- `"Léa"` is the [argument](glossaire.md#argument): the value you put
  in the box, at the moment of the call.

| Call | Value of `name` in the function | Prints |
|------|---------------------------------|--------|
| `greet("Léa")` | `"Léa"` | Hello Léa |
| `greet("Tom")` | `"Tom"` | Hello Tom |

A function can have several parameters, separated by commas. The order
matters: the first argument goes into the first parameter.

```python
def introduce(name, age):
    print(name, "is", age, "years old")

introduce("Léa", 12)
```

```sortie
Léa is 12 years old
```

Try it: write `introduce(12, "Léa")`. Python prints `12 is Léa years old`:
the boxes were filled in the wrong order.

> **Remember:** the parameter is the box (in `def`), the argument is what
> you put in it (in the call).

## Giving back a result: return

`return` means "here is the result, I'm giving it back to you". The
function stops and hands the value to whoever called it.

```python
def double(x):
    return x * 2

result = double(3)
print(result)
print(double(10) + 1)
```

```sortie
6
21
```

The call `double(3)` is **replaced** by its answer, as if you had
written `6`.

```text
result = double(3)
         └───┬───┘
             6          → result = 6
```

The result can be used elsewhere: in a calculation, in a variable, in
another call.

### return or print: the big mix-up

This is the most common trap. Both seem to "give" a value. They are not
the same at all.

- `print` **shows** a value on the screen, for you. The program can't
  reuse it.
- `return` **gives back** a value to the program. Nothing is shown.

Compare these two functions:

```python
def double_print(x):
    print(x * 2)

def double_return(x):
    return x * 2

a = double_print(3)
b = double_return(3)
print("a is", a)
print("b is", b)
```

```sortie
6
a is None
b is 6
```

Line by line:

| Line | What happens | Printed |
|------|--------------|---------|
| `a = double_print(3)` | the function **shows** 6, but gives nothing back | `6` |
| `b = double_return(3)` | the function **gives back** 6, without showing anything | nothing |
| `print("a is", a)` | `a` is empty: Python says `None` | `a is None` |
| `print("b is", b)` | `b` contains 6 | `b is 6` |

`None` means "nothing". A function without `return` gives back `None`.

Think of the toaster. `print` is a photo of the toast sent by text
message: you see the toast, but you can't eat it. `return` is the toast
put on your plate.

Try it: write `print(double_print(3) + 1)`. Python prints 6, then an
error: you can't add 1 to "nothing".

> **Remember:** `print` shows, `return` gives back. If you want to reuse
> the result, you need `return`.

### In other languages

The idea is the same: a name, parameters in parentheses, a `return`.

```javascript
function double(x) {
  return x * 2;
}
console.log(double(3));
```

```sortie
6
```

```ruby
def double(x)
  return x * 2
end
puts double(3)
```

```sortie
6
```

In Ruby, the last line is given back automatically: you can write just
`x * 2`.

```rust
fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("{}", double(3));
}
```

```sortie
6
```

Rust asks for the [type](glossaire.md#type) of each input (`x: i32`, a
whole number) and of the output (`-> i32`). The last line, without a
semicolon, is the result given back.

## A function that calls another one

A function can use another function. It's like a pie recipe that says
"make the dough (see page 12)".

```python
def square(n):
    return n * n

def sum_of_squares(a, b):
    return square(a) + square(b)

print(sum_of_squares(3, 4))
```

```sortie
25
```

Let's follow the calculation:

| Step | What is calculated | Result |
|------|--------------------|--------|
| 1 | `sum_of_squares(3, 4)` starts, `a` = 3, `b` = 4 | – |
| 2 | `square(3)` | 9 |
| 3 | `square(4)` | 16 |
| 4 | `9 + 16`, given back by `sum_of_squares` | 25 |

Try it: replace `return n * n` with `return n * n * n`. The result
becomes 27 + 64 = 91.

## Why functions help

Without functions, you copy the same calculation everywhere. If the
calculation changes, you have to fix it in every place, and you forget
one.

```python
def price_with_tax(price):
    return price * 1.2

print(price_with_tax(10))
print(price_with_tax(50))
print(price_with_tax(100))
```

```sortie
12.0
60.0
120.0
```

Functions help in two ways:

- **no repetition**: the formula is written only once. If the tax
  changes, you change a single line;
- **a clear name**: `price_with_tax(10)` reads better than `10 * 1.2`.
  You understand what the code does without reading the calculation.

> **Remember:** as soon as you copy the same code, turn it into a
> function, with a name that says what it does.

## Local variables

A variable created **inside** a function only exists **inside** that
function. We say it is **local**. The zone where it lives is called the
[scope](glossaire.md#scope).

Think of scrap paper in an exam room: you write on it, then it stays in
the room. Only the paper you hand in (the `return`) goes out.

```python
def calculate():
    result = 6 * 7
    return result

answer = calculate()
print(answer)
```

```sortie
42
```

Outside the function, `result` doesn't exist:

```python norun
def calculate():
    result = 6 * 7

calculate()
print(result)
```

```text
Traceback (most recent call last):
  File "local.py", line 5, in <module>
    print(result)
          ^^^^^^
NameError: name 'result' is not defined
```

That's a good thing: two functions can each have their own `total`
variable without getting in each other's way. To get a value out, use
`return`.

> **Remember:** what is created in a function stays in the function.
> Only `return` gets a value out.

## Common mistakes

### Forgetting the parentheses in the call

Without `()`, the function isn't run. You talk about it without making
it work.

```python norun
def say_hello():
    print("Hello!")

print(say_hello)
```

```text
<function say_hello at 0x7f8377a271a0>
```

Python describes the function instead of running it. (The number
`0x7f…` changes every time: it's an address in memory.) Fix:
`say_hello()`.

### Forgetting return

The function calculates, but gives nothing back.

```python
def double(x):
    x * 2

print(double(3))
```

```sortie
None
```

No error message, but `None` instead of 6. The calculation is done,
then thrown away. Fix: `return x * 2`.

### The wrong number of arguments

The function expects a first name, and you give it nothing.

```python norun
def greet(name):
    print("Hello", name)

greet()
```

```text
Traceback (most recent call last):
  File "greet.py", line 4, in <module>
    greet()
    ~~~~~^^
TypeError: greet() missing 1 required positional argument: 'name'
```

Read the last line: "1 argument is missing: `name`". With one argument
too many, `greet("Léa", "Tom")`, the message becomes:

```text
TypeError: greet() takes 1 positional argument but 2 were given
```

Fix: give as many arguments as there are parameters, `greet("Léa")`.
See also [reading an error message](lire-une-erreur.md).

## Test yourself

**1.** What does this program do: `def f(): print("Hi")` and nothing else?

> **Answer**
>
> Nothing is printed. The function is defined, but never called. You
> need to add `f()`.

**2.** In `def area(width, height):` and `area(3, 5)`, which are the
parameters and which are the arguments?

> **Answer**
>
> The parameters are `width` and `height` (the boxes, in `def`). The
> arguments are `3` and `5` (the values, in the call). `width` receives
> 3, `height` receives 5.

**3.** Why does `x = my_function()` give `None` even though the function
does print a number?

> **Answer**
>
> Because the function uses `print` instead of `return`. It **shows**
> the number, but doesn't **give it back**. A function without `return`
> gives back `None`.

## Summary

- A function is a machine: inputs, a job, an output.
- `def` writes the recipe; `name()` runs it.
- The parameter is the box in `def`; the argument is the value given in the call.
- `print` shows a value; `return` gives it back to the program.
- A function can call another one; it saves you from copying code.
- A variable created in a function stays in the function.

Next: [data](donnees.md).
