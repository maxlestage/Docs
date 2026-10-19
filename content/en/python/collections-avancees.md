# Advanced collections

This page presents Python's specialised boxes: `namedtuple`, `deque`, `heapq`, `bisect`, `Counter` and `defaultdict`, and helps you pick the right one for each problem.

> **Goals:**
>
> - you will know how to give names to the slots of a tuple with `namedtuple`;
> - you will know how to manage a waiting line with `deque`;
> - you will know how to always take out the most urgent item with `heapq`;
> - you will know how to search and insert in a sorted list with `bisect`;
> - you will know how to count with `Counter` and group with `defaultdict`;
> - you will know how to choose the right collection thanks to a table.

## Why other collections?

You already know [lists](collections.md), [tuples](collections.md#tuples)
and [dictionaries](dictionnaires.md). They do almost everything. But for some
jobs, there is a better-suited tool.

Think of a kitchen. A kitchen knife cuts everything. But to peel a potato, a peeler is
faster and cuts your fingers less. The collections on this page are peelers: each one
does **one** thing, very well.

```text
The problem                          The tool
─────────────────────────────────    ─────────────
"slots with names"               →   namedtuple
"first come, first served"       →   deque
"most urgent first"              →   heapq
"keep a list sorted"             →   bisect
"count"                          →   Counter
"sort into groups"               →   defaultdict
```

Almost all of them come from the `collections` [module](../debuter/glossaire.md#module).
`heapq` and `bisect` are two small separate modules. Everything ships with Python:
nothing to install.

> **Remember:** these tools do nothing a list or a dictionary can't do.
> They do it more **clearly**, and often **faster**.

## namedtuple: a tuple with names

A [tuple](../debuter/glossaire.md#tuple) stores values in numbered places.
What is `point[0]`? The x? The y? You have to remember.

A `namedtuple` is a tuple where each slot carries a **label**. It's an ID card: you
read "Last name", "First name", not "slot 0", "slot 1".

```python
from collections import namedtuple

Point = namedtuple("Point", ["x", "y"])

p = Point(3, 4)
print(p)
print(p.x)
print(p.y)
print(p[0])
```

```sortie
Point(x=3, y=4)
3
4
3
```

Line by line:

| Code | What it does |
|---|---|
| `namedtuple("Point", ["x", "y"])` | makes a new card **template**, with two slots |
| `Point(3, 4)` | fills in a card: `x` gets 3, `y` gets 4 |
| `p.x` | reads the slot named `x` |
| `p[0]` | still works: it's a real tuple |

```text
         Point(3, 4)
   ┌──────────┬──────────┐
   │ x  =  3  │ y  =  4  │
   └──────────┴──────────┘
     p.x or p[0]  p.y or p[1]
```

> **Try it:** create `Point(y=10, x=2)`. The order of keyword arguments doesn't
> matter: `print` displays `Point(x=2, y=10)`.

### Unpacking and changing a slot

A `namedtuple` unpacks like a tuple. It is [immutable](../debuter/glossaire.md#immutable):
to "change" a slot, `_replace` makes a **new** card.

```python
from collections import namedtuple

Student = namedtuple("Student", ["name", "grade"])

s = Student("Léa", 12)
name, grade = s
print(name, grade)

s2 = s._replace(grade=15)
print(s)
print(s2)
print(s2._asdict())
```

```sortie
Léa 12
Student(name='Léa', grade=12)
Student(name='Léa', grade=15)
{'name': 'Léa', 'grade': 15}
```

`s` hasn't changed. `s2` is a copy with a different grade. `_asdict()` turns the card
into a dictionary.

> **Remember:** use a `namedtuple` for a small fixed group of values (a point,
> a colour, a table row). The code reads `s.grade` instead of `s[1]`: it explains
> itself.

## deque: a waiting line

Picture the line at the bakery. People arrive **at the end**. The person **at the front**
gets served. First come, first served.

With a list, removing the first item (`list.pop(0)`) forces Python to shift all the
others by one place. With 10 people, it doesn't matter. With a million, it's slow.

A `deque` (pronounced "deck", for *double-ended queue*, a "line with two ends")
adds and removes **at both ends** without shifting anything.

```python
from collections import deque

queue = deque()
queue.append("Alice")
queue.append("Bob")
queue.append("Chloé")
print(queue)

served = queue.popleft()
print("Serving", served)
print(queue)
```

```sortie
deque(['Alice', 'Bob', 'Chloé'])
Serving Alice
deque(['Bob', 'Chloé'])
```

Let's follow the line at each step:

| Step | Code | Line (front → back) |
|---|---|---|
| 1 | `deque()` | *(empty)* |
| 2 | `append("Alice")` | Alice |
| 3 | `append("Bob")` | Alice, Bob |
| 4 | `append("Chloé")` | Alice, Bob, Chloé |
| 5 | `popleft()` → `"Alice"` | Bob, Chloé |

```text
             popleft()                    append(x)
   out     ◄──────────  [ Bob | Chloé ]  ◄──────────  in
            (front)                         (back)
```

### The four moves

| Move | Left end (front) | Right end (back) |
|---|---|---|
| add | `appendleft(x)` | `append(x)` |
| remove | `popleft()` | `pop()` |

### Keeping the last N: maxlen

With `maxlen`, the deque has a maximum size. When it is full, adding on the right
pushes out the item on the left. It's the "last 3 pages visited" history.

```python
from collections import deque

history = deque(maxlen=3)
for page in ["home", "products", "cart", "payment", "thanks"]:
    history.append(page)
    print(list(history))
```

```sortie
['home']
['home', 'products']
['home', 'products', 'cart']
['products', 'cart', 'payment']
['cart', 'payment', 'thanks']
```

At the 4th page, the deque is full: `"home"` drops out on the left.

> **Try it:** replace `maxlen=3` with `maxlen=2`. Each line then has at most two
> pages, and the last one shows `['payment', 'thanks']`.

### An example: serving customers

```python
from collections import deque

customers = deque(["Alice", "Bob", "Chloé"])
turn = 1
while customers:
    name = customers.popleft()
    print(f"Turn {turn}: serving {name}, {len(customers)} left")
    turn += 1
```

```sortie
Turn 1: serving Alice, 2 left
Turn 2: serving Bob, 1 left
Turn 3: serving Chloé, 0 left
```

| Turn | `name` | `customers` after `popleft` |
|---|---|---|
| 1 | Alice | Bob, Chloé |
| 2 | Bob | Chloé |
| 3 | Chloé | *(empty)* → `while` stops |

`while customers:` keeps going as long as the deque is not empty, just like a list.

> **Remember:** for a waiting line, use `deque` with `append` (get in) and
> `popleft` (get out). Never `list.pop(0)` on a big list.

## heapq: most urgent first

In the hospital emergency room, patients are not treated in order of arrival. The
**most serious case goes first**. It's a **priority queue**.

The `heapq` module turns an ordinary list into a **heap**. A heap always keeps the
**smallest** item at place 0. The rest is not sorted: it's just organised enough to
find the smallest one quickly.

```python
import heapq

heap = []
heapq.heappush(heap, 5)
heapq.heappush(heap, 1)
heapq.heappush(heap, 8)
heapq.heappush(heap, 3)
print("the smallest:", heap[0])

print(heapq.heappop(heap))
print(heapq.heappop(heap))
print("left:", sorted(heap))
```

```sortie
the smallest: 1
1
3
left: [5, 8]
```

| Code | What it does |
|---|---|
| `heappush(heap, x)` | adds `x` to the heap |
| `heap[0]` | looks at the smallest, without removing it |
| `heappop(heap)` | removes and returns the smallest |

```text
We pour numbers into a funnel:
the smallest always falls out first.

   5  1  8  3
   ╲        ╱
    ╲      ╱
     ╲    ╱
      ╲  ╱
       1      ← heappop() takes it out
```

### Tasks with a priority

We store `(priority, task)` tuples. Python compares tuples **slot by slot**:
the priority first. The smaller the number, the more urgent.

```python
import heapq

tasks = []
heapq.heappush(tasks, (3, "water the plants"))
heapq.heappush(tasks, (1, "turn off the oven"))
heapq.heappush(tasks, (2, "answer the mail carrier"))

while tasks:
    priority, task = heapq.heappop(tasks)
    print(priority, task)
```

```sortie
1 turn off the oven
2 answer the mail carrier
3 water the plants
```

The order they were added was 3, 1, 2. The order they come out is 1, 2, 3.

| Turn | `heappop` takes out | What's left |
|---|---|---|
| 1 | (1, turn off the oven) | (2, …), (3, …) |
| 2 | (2, answer the mail carrier) | (3, …) |
| 3 | (3, water the plants) | *(empty)* |

> **Try it:** add `(0, "call the fire brigade")`. It comes out first.

### The N largest or smallest

To just get the podium, `nlargest` and `nsmallest` are enough:

```python
import heapq

scores = [42, 17, 99, 63, 8, 75]
print(heapq.nlargest(3, scores))
print(heapq.nsmallest(2, scores))
```

```sortie
[99, 75, 63]
[8, 17]
```

> **Remember:** `heapq` always takes out the **smallest** first. Store
> `(priority, value)` tuples for a priority queue. The heap is **not** a sorted list:
> don't print `heap` expecting an order.

## bisect: searching a sorted list

To find a word in a paper dictionary, you don't read every page.
You open it in the middle, check whether the word comes before or after, and start
again with the right half. This is **binary search**.

The `bisect` module does exactly that, on a list that is **already sorted**.

```python
import bisect

grades = [5, 8, 12, 15, 18]
print(bisect.bisect(grades, 13))
```

```sortie
3
```

`bisect(grades, 13)` answers: "to keep the list sorted, 13 would go at place 3"
(just before 15). Here is the search step by step:

| Step | Part looked at | Middle | 13 is… |
|---|---|---|---|
| 1 | `[5, 8, 12, 15, 18]` (places 0–4) | place 2 → 12 | after 12 → keep the right side |
| 2 | `[15, 18]` (places 3–4) | place 4 → 18 | before 18 → keep the left side |
| 3 | `[15]` (place 3) | place 3 → 15 | before 15 → it's here: **3** |

Three looks for 5 items. For a million items, about twenty are enough.

### insort: inserting at the right place

`insort` inserts an item **at its place**. The list stays sorted, without calling `sort`.

```python
import bisect

grades = [5, 8, 15, 18]
bisect.insort(grades, 12)
bisect.insort(grades, 1)
print(grades)
```

```sortie
[1, 5, 8, 12, 15, 18]
```

### A grading scale with bisect

A classic use: turning a grade into a label. The thresholds are sorted; `bisect`
tells which "band" the grade falls into.

```python
import bisect

thresholds = [10, 12, 14, 16]
labels = ["Fail", "Pass", "Fair", "Good", "Very good"]

def label(grade):
    return labels[bisect.bisect(thresholds, grade)]

for grade in [7, 10, 13, 17]:
    print(grade, "→", label(grade))
```

```sortie
7 → Fail
10 → Pass
13 → Fair
17 → Very good
```

```text
thresholds:     10        12        14        16
         ───────┼─────────┼─────────┼─────────┼────────
place:      0   │    1    │    2    │    3    │    4
          Fail  │  Pass   │  Fair   │  Good   │Very good
                ▲            ▲                     ▲
             grade 10     grade 13              grade 17
```

| `grade` | `bisect(thresholds, grade)` | `labels[...]` |
|---|---|---|
| 7 | 0 | Fail |
| 10 | 1 | Pass |
| 13 | 2 | Fair |
| 17 | 4 | Very good |

A grade equal to a threshold (10) goes **after** it: it gets the label of the band
above.

> **Try it:** test `label(16)`. You get `Very good`. With
> `bisect.bisect_left`, a grade equal to the threshold would go **before**: 16 would
> give `Good`.

> **Remember:** `bisect` only works on a **sorted** list. On an unordered list,
> it doesn't crash: it gives a wrong answer.

## Counter in detail

`Counter` was already introduced in [dictionaries](dictionnaires.md#with-collections-counter).
It's a dictionary whose values are **numbers of occurrences**. Think of tallying votes
on the board: one stroke per vote.

```python
from collections import Counter

votes = ["pizza", "sushi", "pizza", "tacos", "pizza", "sushi"]
c = Counter(votes)
print(c)
print(c["pizza"], c["crêpes"])
print(c.most_common(1))
```

```sortie
Counter({'pizza': 3, 'sushi': 2, 'tacos': 1})
3 0
[('pizza', 3)]
```

Step by step, Counter does this for you:

| Vote read | pizza | sushi | tacos |
|---|---|---|---|
| pizza | 1 | 0 | 0 |
| sushi | 1 | 1 | 0 |
| pizza | 2 | 1 | 0 |
| tacos | 2 | 1 | 1 |
| pizza | 3 | 1 | 1 |
| sushi | 3 | 2 | 1 |

### Adding, removing, summing

`update` adds counts. `subtract` removes some. Two `Counter`s add up with
`+`.

```python
from collections import Counter

stock = Counter(apples=5, pears=2)
stock.update({"apples": 3, "kiwis": 4})
print(stock)

stock.subtract({"apples": 6})
print(stock["apples"])

monday = Counter(coffee=3, tea=1)
tuesday = Counter(coffee=2, cocoa=1)
print(monday + tuesday)
print(sum(monday.values()))
```

```sortie
Counter({'apples': 8, 'kiwis': 4, 'pears': 2})
2
Counter({'coffee': 5, 'tea': 1, 'cocoa': 1})
4
```

> **Try it:** remove 10 apples instead of 6. `stock["apples"]` is then `-2`:
> `Counter` accepts negative counts.

> **Remember:** as soon as you write `count[x] = count.get(x, 0) + 1`, think
> `Counter`. A missing key is worth 0, with no error.

## defaultdict in detail

`defaultdict` was also covered in [dictionaries](dictionnaires.md#with-defaultdict).
It's a dictionary that, when you read a missing key, **creates** a starting value
instead of raising `KeyError`.

It's a magic binder: you ask for the "B" divider, it doesn't exist, the binder makes
it by itself, empty, and hands it to you.

You give it a **function with no argument** that makes the starting value:

| You write | Value created for a missing key |
|---|---|
| `defaultdict(int)` | `0` |
| `defaultdict(list)` | `[]` |
| `defaultdict(set)` | `set()` |
| `defaultdict(str)` | `""` |

### Grouping

```python
from collections import defaultdict

students = [("Léa", "4A"), ("Hugo", "4B"), ("Inès", "4A"), ("Tom", "4B"), ("Zoé", "4A")]

classes = defaultdict(list)
for name, room in students:
    classes[room].append(name)

for room, names in classes.items():
    print(room, names)
```

```sortie
4A ['Léa', 'Inès', 'Zoé']
4B ['Hugo', 'Tom']
```

| Turn | `name`, `room` | Key exists? | `classes` after the turn |
|---|---|---|---|
| 1 | Léa, 4A | no → `[]` created | 4A: [Léa] |
| 2 | Hugo, 4B | no → `[]` created | 4A: [Léa], 4B: [Hugo] |
| 3 | Inès, 4A | yes | 4A: [Léa, Inès], 4B: [Hugo] |
| 4 | Tom, 4B | yes | 4A: [Léa, Inès], 4B: [Hugo, Tom] |
| 5 | Zoé, 4A | yes | 4A: [Léa, Inès, Zoé], 4B: [Hugo, Tom] |

### Summing per group

With `int`, the starting value is 0: perfect for totals.

```python
from collections import defaultdict

purchases = [("Léa", 12.5), ("Hugo", 8.0), ("Léa", 4.5), ("Hugo", 2.0), ("Inès", 10.0)]

total = defaultdict(int)
for name, price in purchases:
    total[name] += price

print(dict(total))
```

```sortie
{'Léa': 17.0, 'Hugo': 10.0, 'Inès': 10.0}
```

### The trap: reading is enough to create

With `defaultdict`, **looking at** a missing key creates it. To test without creating,
use `in`.

```python
from collections import defaultdict

d = defaultdict(list)
print("x" in d)
print(d["x"])
print("x" in d)
print(dict(d))
```

```sortie
False
[]
True
{'x': []}
```

> **Remember:** `defaultdict(list)` to group, `defaultdict(int)` to
> sum. Test a key with `in`, not with `d[key]`.

## Which collection should you choose?

Here is the decision table. Read the "You want…" column and take the tool next to it.

| You want… | Tool | Main move | Real-life example |
|---|---|---|---|
| an ordered, changeable sequence | `list` | `append`, `[i]` | a shopping list |
| fixed, named slots | `namedtuple` | `p.x` | an ID card |
| to serve in order of arrival | `deque` | `append` + `popleft` | the line at the bakery |
| to keep the last N | `deque(maxlen=N)` | `append` | the browser history |
| to take out the most urgent | `heapq` | `heappush` + `heappop` | the hospital emergency room |
| the podium of the N best | `heapq.nlargest` | a single call | the top 3 scores |
| to search a sorted list | `bisect` | `bisect`, `insort` | the paper dictionary |
| to count occurrences | `Counter` | `Counter(list)`, `most_common` | counting the votes |
| to sort into groups | `defaultdict(list)` | `d[key].append(x)` | the dividers of a binder |
| to find by a key | `dict` | `d[key]` | a phone book |
| no duplicates | `set` | `add`, `in` | the guest list |

```text
I need to…
 ├─ count?                             → Counter
 ├─ group?                             → defaultdict(list)
 ├─ handle things in order of arrival? → deque
 ├─ handle the most important first?   → heapq
 ├─ keep a list always sorted?         → bisect
 ├─ name the slots of a tuple?         → namedtuple
 └─ none of these                      → list or dict, plain and simple
```

> **Remember:** always start with a list or a dictionary. Switch to a specialised
> tool when your code looks like one of the rows of the table.

## Common mistakes

### Changing a namedtuple

```python norun
from collections import namedtuple
Point = namedtuple("Point", ["x", "y"])
p = Point(1, 2)
p.x = 5
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    p.x = 5
    ^^^
AttributeError: can't set attribute
```

A `namedtuple` is immutable, like a tuple. Make a new card:
`p = p._replace(x=5)`. If you need to change it often, use a
[dataclass](typage.md) instead.

### Removing from an empty deque

```python norun
from collections import deque
queue = deque()
queue.popleft()
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    queue.popleft()
    ~~~~~~~~~~~~~^^
IndexError: pop from an empty deque
```

There is nobody in the line. Check first: `if queue: queue.popleft()`, or loop
with `while queue:`.

### Two equal priorities with values that can't be compared

```python norun
import heapq
tasks = []
heapq.heappush(tasks, (2, {"name": "wash"}))
heapq.heappush(tasks, (2, {"name": "tidy"}))
```

```text
Traceback (most recent call last):
  File "main.py", line 4, in <module>
    heapq.heappush(tasks, (2, {"name": "tidy"}))
    ~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
TypeError: '<' not supported between instances of 'dict' and 'dict'
```

Both priorities are 2. Python then compares the next slot: two dictionaries,
which it can't compare. Add a unique sequence number in the middle:
`(2, number, {"name": "tidy"})`, with `number` going up by 1 at each push.

### bisect on an unsorted list

```python
import bisect

grades = [15, 5, 18, 8]
bisect.insort(grades, 16)
print(grades)
```

```sortie
[15, 5, 16, 18, 8]
```

No error, but the result makes no sense: `insort` assumes the list is sorted and puts
16 in the wrong place. Sort first with `grades.sort()`, then keep the list sorted
with `insort`.

## Test yourself

**Question 1:** you are writing a game where players wait for their turn, in order of
arrival. Which collection do you use, and with which methods?

> **Answer**
>
> A `deque`. You add a player with `append` and serve the next one with `popleft`.
> Unlike `list.pop(0)`, `popleft` shifts nothing, even with lots of
> players.

**Question 2:** what does this program print?

```python norun
import heapq
heap = []
for x in [7, 2, 9, 4]:
    heapq.heappush(heap, x)
print(heapq.heappop(heap), heapq.heappop(heap))
```

> **Answer**
>
> ```python
> import heapq
> heap = []
> for x in [7, 2, 9, 4]:
>     heapq.heappush(heap, x)
> print(heapq.heappop(heap), heapq.heappop(heap))
> ```
>
> ```sortie
> 2 4
> ```
>
> `heappop` always takes out the smallest: first 2, then 4.

**Question 3:** what is the difference between `Counter` and `defaultdict(int)`?

> **Answer**
>
> Both give 0 for a missing key. But `Counter` counts a whole list in one
> go (`Counter(votes)`) and offers `most_common`. Also, reading a missing key from a
> `Counter` doesn't create it, whereas a `defaultdict` adds it.

## Summary

- `namedtuple` gives names to the slots of a tuple: `p.x` instead of `p[0]`.
- `deque` is a line with two ends: `append` to get in, `popleft` to get out;
  `maxlen` keeps the last N.
- `heapq` always takes out the smallest: store `(priority, value)` tuples.
- `bisect` searches and `insort` inserts in a **sorted** list, very fast.
- `Counter` counts, `defaultdict(list)` groups, `defaultdict(int)` sums.
- When in doubt, start with a list or a dictionary.

Next: [Advanced functions and decorators](decorateurs.md)
