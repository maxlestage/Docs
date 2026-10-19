# When you're stuck: a step-by-step method

Getting stuck on a problem happens to everyone: this page gives you a simple, step-by-step method to keep moving anyway, with a complete example in Python.

> **Goals:**
>
> - know that being stuck is normal;
> - restate a problem in your own words;
> - solve a small example by hand, on paper;
> - break a problem down and write it one step at a time;
> - use the rubber duck and the break;
> - ask for help effectively.

## Being stuck is normal

You read the instructions. You don't know where to start. You stare at the
empty screen. You tell yourself: "I'm not cut out for this."

That's not true. **Everyone** gets stuck, even after twenty years on the job.
Being stuck doesn't mean you're bad at this: it means you're learning
something new.

The difference between a beginner and someone experienced isn't that they
never get stuck. It's that they have a **method** to get unstuck. Here it
is, in six steps:

```text
  1. Restate           → say the problem in my own words
  2. Do it by hand     → a tiny example, on paper
  3. Break it down     → steps in plain English (pseudocode)
  4. Code step by step → one step, I run it, I check
  5. Explain           → to the rubber duck
  6. Take a break      → then ask for help
```

> **Remember:** getting stuck is part of the job. What matters is having
> a method.

## Step 1: restate it in your own words

Instructions are often written by someone else, in their own words.
Rewrite them in **yours**, in one or two simple sentences.

Take these instructions:

```text
Write a function that determines the number of vowels in a string.
```

Restated:

```text
I get a word. I have to say how many vowels it has.
The vowels are: a, e, i, o, u.
```

Ask yourself three questions:

- **What do I get?** A word (a text).
- **What do I have to give back?** A number.
- **What isn't clear?** Do capital letters count? Yes: `"A"` is a vowel
  too.

If you can't restate it, the problem isn't the code: it's understanding the
instructions. Read them again, or ask for clarification.

## Step 2: solve a tiny example by hand

Grab a pencil. Pick a short word: `banana`. Count its vowels, **slowly**,
writing down everything you do.

| Letter | Vowel? | Count |
|---|---|---|
| (start) | | 0 |
| b | no | 0 |
| a | yes | 1 |
| n | no | 1 |
| a | yes | 2 |
| n | no | 2 |
| a | yes | 3 |

Answer: **3**.

Now look at what you did. You:

1. started a count at 0;
2. looked at each letter, one by one;
3. added 1 to the count when it was a vowel;
4. gave the count at the end.

You've just found the [algorithm](glossaire.md#algorithm). Your brain
already knew it: you just had to **slow it down** to see it.

> **Remember:** if you can do it by hand, you can explain it to the
> computer.

## Step 3: break it down into pseudocode

You write these steps in plain English, shaped like a program. That's
[pseudocode](glossaire.md#pseudo-code) (see [Thinking like a
programmer](algorithmes.md)):

```text
count ← 0
for each letter of the word
    if the letter is a vowel
        count ← count + 1
give count
```

Each line is small enough to become one line of code.

## Step 4: code one step at a time

Don't type everything at once. Write **one step**, run it, check it. Then
the next one. Like assembling furniture: you check each screw before moving
on to the next.

### Try 1: going through the letters

First, can I look at each letter?

```python
word = "banana"
for letter in word:
    print(letter)
```

```sortie
b
a
n
a
n
a
```

Yes. Next step.

### Try 2: recognizing a vowel

`letter in "aeiou"` asks: "is this letter in the text `aeiou`?". The
answer is `True` or `False`.

```python
word = "banana"
for letter in word:
    print(letter, letter in "aeiou")
```

```sortie
b False
a True
n False
a True
n False
a True
```

The table matches the one you made by hand. Next step.

### Try 3: counting… and getting it wrong

```python
word = "banana"
for letter in word:
    count = 0
    if letter in "aeiou":
        count = count + 1
print(count)
```

```sortie
1
```

We expected 3. Don't panic: we ran it early, so the mistake is in the 3
lines we just added. Let's follow the variable by hand:

| Letter | `count = 0` | vowel? | count after |
|---|---|---|---|
| b | 0 | no | 0 |
| a | 0 | yes | 1 |
| n | 0 | no | 0 |
| a | 0 | yes | 1 |
| n | 0 | no | 0 |
| a | 0 | yes | 1 |

The count goes back to 0 **for every letter**. In the pseudocode, `count ←
0` was **before** the loop. We put it inside.

### Try 4: the fix

```python
word = "banana"
count = 0
for letter in word:
    if letter in "aeiou":
        count = count + 1
print(count)
```

```sortie
3
```

It works for `banana`. But one example isn't enough.

### Try 5: testing other words

Let's try a word with a capital letter, and a word with no vowels:

```python
for word in ["banana", "Orange", "pff"]:
    count = 0
    for letter in word:
        if letter in "aeiou":
            count = count + 1
    print(word, count)
```

```sortie
banana 3
Orange 2
pff 0
```

`Orange` has 3 vowels (O, a, e), not 2. The capital `O` isn't in
`"aeiou"`. Yet we wrote down that question in step 1! We turn each letter
into lowercase with `lower()`.

### Try 6: the final version

We put it all in a [function](glossaire.md#function):

```python
def count_vowels(word):
    count = 0
    for letter in word.lower():
        if letter in "aeiou":
            count = count + 1
    return count

print(count_vowels("banana"))
print(count_vowels("Orange"))
print(count_vowels("pff"))
```

```sortie
3
3
0
```

Six small tries, two mistakes found right away. If we had written
everything at once, both mistakes would have shown up together, and we
wouldn't have known where to start.

Try it: call `count_vowels("")`, with an empty word. The result is `0`:
the loop doesn't run at all.

> **Remember:** one step, run it, check it. The more often you run your
> code, the easier the mistake is to find.

## Step 5: explain it to the rubber duck

Put a rubber duck (or a plant, or a mug) next to your screen. Explain your
code to it **out loud**, line by line: "here, I set the count to zero; here,
for each letter…".

Very often, right in the middle of a sentence, you hear the mistake
yourself: "… and I set the count back to zero for every letter. Oh."

Talking forces you to slow down. You can't skip a line while explaining it.

## Step 6: take a break

After 20 minutes stuck on the same thing, your brain is going in circles.
Stand up. Walk, drink a glass of water, do something else for ten minutes.

That's not giving up. Your brain keeps working in the background. Many
programmers find the solution in the shower or while walking, never in
front of the screen.

## Asking for help

Asking for help isn't cheating. But a good question gets an answer much
faster. It contains four things:

```text
1. A minimal example: the smallest code that shows the problem.
2. What I expected:   count_vowels("Orange") should give 3.
3. What I get:        it gives 2 (or: the full error message).
4. What I tried:      I printed each letter, the O isn't counted.
```

**Minimal** means: remove everything that isn't needed to show the
problem. Often, while shrinking the code, you find the mistake yourself.

Copy the code and the message **as text**, not as a photo of your screen:
the person helping you will be able to run it. To go further, see [Finding
and fixing your mistakes](deboguer.md).

## Common mistakes

### Copying the pseudocode without the indentation

```python norun
word = "banana"
for letter in word:
if letter in "aeiou":
    print(letter)
```

```text
  File "/home/ada/vowels.py", line 3
    if letter in "aeiou":
    ^^
IndentationError: expected an indented block after 'for' statement on line 2
```

In Python, what is **inside** the loop must be shifted to the right (the
[indentation](glossaire.md#indentation)). The message says so: an indented
block was expected after the `for` on line 2. Fix: shift the `if` and
everything after it by 4 spaces.

### Forgetting a step of the pseudocode

```python norun
word = "banana"
for letter in word:
    if letter in "aeiou":
        count = count + 1
print(count)
```

```text
Traceback (most recent call last):
  File "/home/ada/vowels.py", line 4, in <module>
    count = count + 1
            ^^^^^
NameError: name 'count' is not defined. Did you mean: 'round'?
```

The `count ← 0` line of the pseudocode was forgotten. Python doesn't know
`count` when it's time to add 1. Its suggestion (`round`) is wrong here: a
suggestion is a guess, not an order. Fix: add `count = 0` before the loop.

## Test yourself

**1.** You don't know where to start on an exercise. What's the first
thing to do?

> **Answer**
>
> Restate the instructions in your own words: what do I get, what do I have
> to give back? Then solve a tiny example by hand.

**2.** Why run the program after every small step?

> **Answer**
>
> Because if something doesn't work, the mistake has to be in the few lines
> you just added. So it's easy to find.

**3.** What are the four parts of a good request for help?

> **Answer**
>
> A minimal example, what you expected, what you get (or the full error
> message), and what you've already tried.

## Summary

- Getting stuck is normal: what matters is the method.
- Restate it in your own words, then solve a small example by hand.
- Write the pseudocode, then code one step at a time, running it after each one.
- Test several examples, including odd cases (capital letter, empty word).
- Explain it to the duck, take a break, then ask for help with a minimal example.

Next: [the terminal in 10 minutes](terminal.md).
