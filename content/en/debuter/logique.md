# True, false and conditions

A program makes decisions by asking itself questions that are answered with true or false: this page shows how to compare, combine questions with and, or, not, then choose a path with `if` and `else`.

> **Goals:**
>
> - know what a boolean is: true or false;
> - compare two values with `==`, `!=`, `<`, `>=`…;
> - combine questions with and, or, not;
> - choose between two paths with `if` and `else`;
> - choose between several paths with `elif`;
> - avoid the classic mistakes: `=` instead of `==`, the forgotten "equal" case.

## True or false: booleans

Some questions can only be answered with **yes** or **no**:
"Is it raining?", "Am I over 18?".

In programming, yes is called **true** and no is called **false**. These two
values are called **booleans** (see [boolean](glossaire.md#boolean)).

```python
is_raining = True
im_hungry = False
print(is_raining)
print(im_hungry)
```

```sortie
True
False
```

In Python, you write `True` and `False`, with a capital letter. In JavaScript
and in Ruby, you write `true` and `false`, in lowercase.

A boolean can be stored in a variable, like a number. See
[variables](variables.md).

> **Remember:** a boolean has only two possible values: true or false.

## Comparing two values

Most of the time, you don't write `True` by hand. You get it by
**comparing** two values. The answer is a boolean.

```python
age = 20
print(age > 18)
print(age < 18)
print(age == 20)
```

```sortie
True
False
True
```

Here are the six comparisons. They are written the same way in Python,
JavaScript and Ruby:

| You write | You read | True example |
|---|---|---|
| `==` | is equal to | `3 == 3` |
| `!=` | is different from | `3 != 4` |
| `<` | is less than | `2 < 5` |
| `>` | is greater than | `5 > 2` |
| `<=` | is less than **or equal** | `5 <= 5` |
| `>=` | is greater than **or equal** | `5 >= 5` |

You can also compare texts: `"sun" == "Sun"` is `False`. For the computer,
`s` and `S` are two different letters.

> **Warning:** `=` stores a value in a variable. `==` asks a question: "is
> it equal?". These are two very different things.

Try it: replace `age = 20` with `age = 18`. The first line prints `False`:
18 is not greater than 18.

## Combining questions: and, or, not

Sometimes, a single question is not enough. "I go out **if** the weather is
nice **and** I've finished my work."

There are three words to combine booleans. Python writes them in English;
JavaScript and Ruby use symbols:

| In English | Python | JavaScript, Ruby |
|---|---|---|
| and | `and` | `&&` |
| or | `or` | `\|\|` |
| not | `not` | `!` |

### "and": both must be true

"The weather is nice **and** I've finished" is only true if **both** are
true.

| Nice weather? | Finished? | Do I go out? |
|---|---|---|
| yes | yes | **yes** |
| yes | no | no |
| no | yes | no |
| no | no | no |

```python
nice_weather = True
work_done = False
print(nice_weather and work_done)
```

```sortie
False
```

### "or": one is enough

"I have coffee **or** tea" is true as soon as **at least one** of the two is
true.

| Coffee? | Tea? | Do I have a drink? |
|---|---|---|
| yes | yes | **yes** |
| yes | no | **yes** |
| no | yes | **yes** |
| no | no | no |

```python
coffee = False
tea = True
print(coffee or tea)
```

```sortie
True
```

### "not": the opposite

`not` flips the answer: true becomes false, false becomes true.

```python
is_raining = False
print(not is_raining)
```

```sortie
True
```

The same calculation in JavaScript (Ruby writes exactly the same thing,
with `puts` instead of `console.log`):

```javascript
const niceWeather = true;
const workDone = false;
console.log(niceWeather && workDone, niceWeather || workDone, !niceWeather);
```

```sortie
false true false
```

> **Remember:** "and" needs everything to be true; "or" is happy with a
> single true; "not" flips.

## Choosing a path: if and else

Imagine a road that splits in two. A sign asks a question. Depending on the
answer, you go left or right. Then the two roads join again.

```text
               start
                |
        +---------------+
        | age >= 18 ?   |
        +---------------+
          yes /     \ no
             /       \
   "You can        "Not yet."
     vote."
             \       /
              \     /
                |
             what's next
```

That's exactly what `if` and `else` do:

```python
age = 20
if age >= 18:
    print("You can vote.")
else:
    print("Not yet.")
print("Goodbye.")
```

```sortie
You can vote.
Goodbye.
```

Read it out loud: "**if** the age is 18 or more, print…; **else**,
print…". Only one of the two branches runs. The last line, however, always
runs: it's "what's next".

In Python, the colon `:` and the **shift** of the lines (called
**indentation**, see [indentation](glossaire.md#indentation)) show what is
**inside** the `if`.

Try it: set `age = 15`. The program prints "Not yet." then "Goodbye.".

The `else` is optional: if there's nothing to do when the answer is no, you
don't write it.

### In JavaScript and in Ruby

JavaScript puts the question in parentheses and the branches in curly
braces `{ }`:

```javascript
const age = 20;
if (age >= 18) {
  console.log("You can vote.");
} else {
  console.log("Not yet.");
}
```

```sortie
You can vote.
```

Ruby closes the condition with `end`:

```ruby
age = 20
if age >= 18
  puts "You can vote."
else
  puts "Not yet."
end
```

```sortie
You can vote.
```

> **Remember:** `if` asks a question; depending on the answer, only one
> branch runs.

## More than two paths: elif

A grade from 0 to 20 gives a rating. There are **four** paths, not two.

A first idea: put an `if` inside the `else`, then another one, and so on.
These are called **nested** conditions:

```python
grade = 13
if grade >= 16:
    print("Very good")
else:
    if grade >= 12:
        print("Good")
    else:
        if grade >= 10:
            print("Pass")
        else:
            print("Fail")
```

```sortie
Good
```

It works, but the code drifts further and further to the right. It's hard
to read. Python has a word for "else, if": `elif`.

```python
grade = 13
if grade >= 16:
    print("Very good")
elif grade >= 12:
    print("Good")
elif grade >= 10:
    print("Pass")
else:
    print("Fail")
```

```sortie
Good
```

The computer asks the questions **in order**, and stops at the **first**
"yes" answer:

| Step | Question | Answer for 13 | What happens? |
|---|---|---|---|
| 1 | `grade >= 16` | no | next question |
| 2 | `grade >= 12` | yes | prints "Good", and it's over |
| 3 | `grade >= 10` | not asked | – |

13 is also greater than 10, but that question is never asked. So the order
of the questions matters: put the most demanding one first.

In JavaScript, you write `else if`: `} else if (grade >= 12) {`. In Ruby,
you write `elsif` (without an "e"):

```ruby
grade = 13
if grade >= 16
  puts "Very good"
elsif grade >= 12
  puts "Good"
else
  puts "Fail"
end
```

```sortie
Good
```

Try it: set `grade = 16`, then `grade = 9`. Follow the questions one by one
before running the program.

> **Remember:** with `elif`, the first true condition wins; the following
> ones are ignored.

## Common mistakes

### Writing `=` instead of `==`

`=` stores a value; it doesn't ask a question. Python refuses:

```python norun
x = 5
if x = 5:
    print("five")
```

```text
  File "five.py", line 2
    if x = 5:
       ^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

"Maybe you meant '=='?": Python suggests `==`. JavaScript, on the other
hand, says **nothing**, and that's worse:

```javascript
let x = 3;
if (x = 0) {
  console.log("zero");
} else {
  console.log("not zero");
}
console.log(x);
```

```sortie
not zero
0
```

`x = 0` stored 0 in `x`: the value of `x` is lost. The fix: `if (x == 0)`,
or better in JavaScript, `if (x === 0)`.

### Testing a range with `0 < x < 10` in JavaScript

In math, `0 < x < 10` means "x is between 0 and 10". JavaScript reads it
differently:

```javascript
const x = 50;
if (0 < x < 10) {
  console.log("between 0 and 10");
}
```

```sortie
between 0 and 10
```

50 is not between 0 and 10! JavaScript first calculates `0 < 50`, which is
`true`. Then it calculates `true < 10`. It turns `true` into `1`, and
`1 < 10` is true.

The fix: two questions joined by "and".

```javascript
const x = 50;
if (0 < x && x < 10) {
  console.log("between 0 and 10");
} else {
  console.log("outside");
}
```

```sortie
outside
```

In Python, `0 < x < 10` works like in math. In Ruby, it causes an error
(`undefined method '<' for true`): write `0 < x && x < 10`.

### Forgetting the "equal" case

A shop offers free delivery **from** €50.

```python
cart = 50
if cart > 50:
    print("Free delivery")
else:
    print("Delivery: €5")
```

```sortie
Delivery: €5
```

There's no error message: it's a **logic error**. With exactly €50, the
customer should get free delivery. "From 50" includes 50: you needed `>=`.
With `if cart >= 50:`, the program prints "Free delivery".

> **Tip:** always test the **exact** value of the limit (here 50), plus one
> just below and one just above.

## Test yourself

**1.** What is the value of `(3 > 2) and (1 == 2)`?

> **Answer**
>
> `False`. `3 > 2` is true, but `1 == 2` is false. With "and", both must be
> true.

**2.** With `grade = 10`, what does the rating program with `elif` print?

> **Answer**
>
> "Pass". `10 >= 16`: no. `10 >= 12`: no. `10 >= 10`: yes, because `>=`
> includes equality.

**3.** What is the difference between `=` and `==`?

> **Answer**
>
> `=` stores a value in a variable (`age = 20`). `==` compares two values
> and answers true or false (`age == 20`).

## Summary

- A boolean is true or false: `True`/`False` in Python, `true`/`false` in JavaScript and Ruby.
- A comparison (`==`, `!=`, `<`, `<=`, `>`, `>=`) answers with a boolean.
- "and" wants everything true, "or" a single true, "not" flips.
- `if` / `else` chooses a branch; `elif` chains questions, and the first true one wins.
- Watch out for `=` instead of `==`, and for the "equal" case: `>` or `>=`?

Next: [loops](boucles.md)
