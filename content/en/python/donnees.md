# Analyzing data

This page shows how to read a table of CSV data, compute simple statistics (mean, median, mode, standard deviation), group, count, sort and present a small report, first with the standard library and then with pandas.

> **Goals:**
>
> - you will be able to read CSV data and convert the numbers;
> - you will be able to compute the mean, median, mode and standard deviation with
>   `statistics`;
> - you will be able to group and count with `Counter` and `defaultdict`;
> - you will be able to sort rows by a column;
> - you will be able to print a neatly aligned report;
> - you will know what the same work looks like with pandas.

## Analyzing means answering a question

Picture a bakery's notebook. Every evening, the baker writes down what she sold. After a
week, she starts asking herself questions:

- "How many baguettes do I sell per day, on average?"
- "Which product earns me the most?"
- "Which day is the quietest?"

That is what analyzing data means: start from a **question**, do a **calculation** on the
table, get an **answer**.

```text
  question  ──►  read the data  ──►  calculate  ──►  answer
"how many?"      (the CSV file)      (mean,          "41.7 a day"
                                       total…)
```

Throughout this page, we use the same small table of sales:

| day | product | quantity | price |
|---|---|---|---|
| monday | baguette | 40 | 1.10 |
| monday | croissant | 25 | 1.20 |
| monday | tart | 3 | 15.00 |
| tuesday | baguette | 35 | 1.10 |
| tuesday | croissant | 30 | 1.20 |
| wednesday | baguette | 50 | 1.10 |
| wednesday | croissant | 20 | 1.20 |
| wednesday | tart | 5 | 15.00 |

> **Remember:** always start by writing your question in one sentence. It tells you
> which columns to look at and which calculation to do.

## Reading CSV data

The CSV format was introduced in [file formats](formats.md).
Usually, you read a file. Here, so that each example fits in one block, the CSV text is
**inside** the program. `io.StringIO` makes it look like a file: it is a fake file, in
[memory](../debuter/glossaire.md#memoire-vive).

```python
import csv
import io

TEXT = """day,product,quantity,price
monday,baguette,40,1.10
monday,croissant,25,1.20
"""

rows = list(csv.DictReader(io.StringIO(TEXT)))
print(len(rows), "rows")
print(rows[0])
```

```sortie
2 rows
{'day': 'monday', 'product': 'baguette', 'quantity': '40', 'price': '1.10'}
```

Each row becomes a [dictionary](../debuter/glossaire.md#dictionnaire). The keys come
from the first line of the CSV.

> **Note:** with a real file, replace `io.StringIO(TEXT)` with an open file:
> `with open("sales.csv", encoding="utf-8", newline="") as f:` then `csv.DictReader(f)`.
> The rest of the code does not change.

### Converting the numbers

Look closely: `'40'` has quotes. In CSV, **everything is text**. Before calculating, you
have to convert each column to the right type.

```python
import csv
import io

TEXT = """day,product,quantity,price
monday,baguette,40,1.10
monday,croissant,25,1.20
monday,tart,3,15.00
"""

sales = []
for row in csv.DictReader(io.StringIO(TEXT)):
    sales.append({
        "day": row["day"],
        "product": row["product"],
        "quantity": int(row["quantity"]),
        "price": float(row["price"]),
    })

print(sales[2])
print(sales[2]["quantity"] * sales[2]["price"])
```

```sortie
{'day': 'monday', 'product': 'tart', 'quantity': 3, 'price': 15.0}
45.0
```

| Column | Text read | Conversion | Usable value |
|---|---|---|---|
| day | `'monday'` | none | `'monday'` |
| quantity | `'3'` | `int` | `3` |
| price | `'15.00'` | `float` | `15.0` |

> **Try it:** remove the `int(...)` around `row["quantity"]`. The last line crashes:
> `TypeError: can't multiply sequence by non-int of type 'float'`. You cannot multiply a
> text by a decimal number.

> **Remember:** read, then **convert right away**. After that, you only handle real
> numbers.

## Basic statistics

The `statistics` module computes the classic measures. Let's take the baguettes sold
over six days.

```python
import statistics

baguettes = [40, 35, 50, 35, 42, 48]

print("mean:", round(statistics.mean(baguettes), 2))
print("median:", statistics.median(baguettes))
print("mode:", statistics.mode(baguettes))
print("standard deviation:", round(statistics.stdev(baguettes), 2))
print("min, max:", min(baguettes), max(baguettes))
```

```sortie
mean: 41.67
median: 41.0
mode: 35
standard deviation: 6.35
min, max: 35 50
```

Let's see what each measure means.

### The mean: sharing fairly

The mean is the total shared out in equal parts. If you put all the baguettes in a pile
and spread them over the six days, how many would each day get?

| Step | Value read | Total |
|---|---|---|
| 1 | 40 | 40 |
| 2 | 35 | 75 |
| 3 | 50 | 125 |
| 4 | 35 | 160 |
| 5 | 42 | 202 |
| 6 | 48 | 250 |

Total 250, divided by 6 days: 41.67.

### The median: the middle value

You **sort** the values, then take the one in the middle. Like a line of children
ordered by height: the median is the child in the center.

```text
sorted:    35   35   40 │ 42   48   50
                        ▲
         6 values: no single middle,
         so take the mean of the two in the center:
         (40 + 42) / 2 = 41.0
```

With an odd number of values, there is a single element in the middle, and that is it.

Why the median? It is not fooled by an extreme value:

```python
import statistics

salaries = [1800, 1900, 2000, 2100, 25000]
print("mean:", statistics.mean(salaries))
print("median:", statistics.median(salaries))
```

```sortie
mean: 6560
median: 2000
```

A single big salary pushes the mean far above what almost everyone earns. The median,
on the other hand, stays at 2000: half earn less, half earn more.

### The mode: the most frequent

The mode is the value that comes up most often. In `[40, 35, 50, 35, 42, 48]`, 35
appears twice: it is the mode. If there is a tie, `statistics.multimode` gives all the
winners.

```python
import statistics

shoe_sizes = [38, 40, 38, 42, 40, 39]
print(statistics.mode(shoe_sizes))
print(statistics.multimode(shoe_sizes))
```

```sortie
38
[38, 40]
```

### The standard deviation: does it move around a lot?

The standard deviation measures how far the values **stray** from the mean, in general.

- small standard deviation: the values are packed close to the mean;
- large standard deviation: they are widely spread out.

```python
import statistics

steady = [40, 41, 39, 40, 40]
unsteady = [10, 70, 25, 60, 35]
print(statistics.mean(steady), round(statistics.stdev(steady), 2))
print(statistics.mean(unsteady), round(statistics.stdev(unsteady), 2))
```

```sortie
40 0.71
40 24.75
```

```text
steady      :  39 ● 40 ●●● 41 ●              (very close to 40)
unsteady    :  10 ●   25 ●  35 ●    60 ●  70 ●   (all over the place)
                         mean 40
```

Same mean, 40, but very different days. The first baker knows how much to prepare; the
second never does.

| Measure | Function | Question it answers |
|---|---|---|
| mean | `statistics.mean` | "how many, if shared fairly?" |
| median | `statistics.median` | "what is the middle value?" |
| mode | `statistics.mode` | "which value comes up most?" |
| standard deviation | `statistics.stdev` | "does it vary a lot?" |

> **Try it:** in `salaries`, replace 25000 with 2200. The mean drops back to 2000 and
> gets close to the median: when there is no extreme value, the two look alike.

> **Remember:** look at the mean **and** the median. If they are very different, an
> extreme value is pulling the mean.

## Grouping and counting

Most questions have the form "… **per** …": sales **per** product, revenue **per** day.
You have to group the rows. The tools are those from
[advanced collections](collections-avancees.md).

### Counting rows with Counter

"How many times does each product appear in the notebook?"

```python
from collections import Counter

products = ["baguette", "croissant", "tart", "baguette",
            "croissant", "baguette", "croissant", "tart"]
print(Counter(products))
```

```sortie
Counter({'baguette': 3, 'croissant': 3, 'tart': 2})
```

### Adding up per group with defaultdict

"How many units of each product were sold?" This is no longer about counting rows, but
about **adding up** a column per group.

```python
import csv
import io
from collections import defaultdict

TEXT = """day,product,quantity,price
monday,baguette,40,1.10
monday,croissant,25,1.20
monday,tart,3,15.00
tuesday,baguette,35,1.10
tuesday,croissant,30,1.20
"""

total = defaultdict(int)
for row in csv.DictReader(io.StringIO(TEXT)):
    total[row["product"]] += int(row["quantity"])

print(dict(total))
```

```sortie
{'baguette': 75, 'croissant': 55, 'tart': 3}
```

Let's follow the `total` dictionary row after row:

| Row read | product | quantity | baguette | croissant | tart |
|---|---|---|---|---|---|
| 1 | baguette | 40 | 40 | — | — |
| 2 | croissant | 25 | 40 | 25 | — |
| 3 | tart | 3 | 40 | 25 | 3 |
| 4 | baguette | 35 | 75 | 25 | 3 |
| 5 | croissant | 30 | 75 | 55 | 3 |

A dash means "the key does not exist yet". The first time it appears,
`defaultdict(int)` creates it with 0, then the value is added.

```text
The notebook rows                  The boxes per product
 baguette  40 ──────────────►  ┌ baguette ┐  40 + 35 = 75
 croissant 25 ─────────┐       └──────────┘
 tart       3 ──┐      └────►  ┌ croissant┐  25 + 30 = 55
 baguette  35 ──┼──► (baguette)└──────────┘
 croissant 30 ──┼──► (croissant)
                └──────────►   ┌   tart   ┐  3
                               └──────────┘
```

> **Try it:** replace `int(row["quantity"])` with
> `int(row["quantity"]) * float(row["price"])`, and `defaultdict(int)` with
> `defaultdict(float)`. You get the revenue per product instead of the quantities.

> **Remember:** "count the rows per …" → `Counter`. "add up a column per …" →
> `defaultdict(int)` with `+=`.

## Sorting

"Which product sells best?" You have to sort the groups by their total. You sort the
`(product, total)` pairs with `sorted` and a key (see
[sorting by a criterion](collections.md#sorting-by-a-criterion-with-key)).

```python
total = {"baguette": 125, "croissant": 75, "tart": 8}

by_total = sorted(total.items(), key=lambda pair: pair[1], reverse=True)
print(by_total)

for rank, (product, quantity) in enumerate(by_total, start=1):
    print(rank, product, quantity)
```

```sortie
[('baguette', 125), ('croissant', 75), ('tart', 8)]
1 baguette 125
2 croissant 75
3 tart 8
```

| Code | What it does |
|---|---|
| `total.items()` | gives the pairs `('baguette', 125)`, `('croissant', 75)`… |
| `key=lambda pair: pair[1]` | sorts by the 2nd slot of the pair: the total |
| `reverse=True` | from largest to smallest |
| `enumerate(..., start=1)` | numbers the ranks starting from 1 |

### Sorting rows by two columns

You can sort the rows themselves. With a tuple as the key, Python first sorts by the
first slot, then breaks ties with the second.

```python
sales = [
    {"day": "monday", "product": "tart", "quantity": 3},
    {"day": "tuesday", "product": "baguette", "quantity": 35},
    {"day": "monday", "product": "baguette", "quantity": 40},
    {"day": "tuesday", "product": "croissant", "quantity": 30},
]

for s in sorted(sales, key=lambda s: (s["product"], -s["quantity"])):
    print(s["product"], s["day"], s["quantity"])
```

```sortie
baguette monday 40
baguette tuesday 35
croissant tuesday 30
tart monday 3
```

The `-` sign in front of the quantity reverses the order for that column only: the
products are in alphabetical order, and for the same product, the biggest sale comes
first.

> **Remember:** `sorted(..., key=...)` sorts by whatever you want. `reverse=True` for
> largest to smallest.

## A small table report

An answer is easier to read when it is neatly aligned.
[F-strings](texte.md) can reserve a width: `:<12` aligns left over 12 characters, `:>6`
right over 6, `:>8.2f` right over 8 with 2 digits after the decimal point.

```python
import csv
import io
from collections import defaultdict

TEXT = """day,product,quantity,price
monday,baguette,40,1.10
monday,croissant,25,1.20
monday,tart,3,15.00
tuesday,baguette,35,1.10
tuesday,croissant,30,1.20
wednesday,baguette,50,1.10
wednesday,croissant,20,1.20
wednesday,tart,5,15.00
"""

quantities = defaultdict(int)
revenues = defaultdict(float)
for row in csv.DictReader(io.StringIO(TEXT)):
    q = int(row["quantity"])
    quantities[row["product"]] += q
    revenues[row["product"]] += q * float(row["price"])

print(f"{'Product':<12}{'Qty':>6}{'Revenue':>10}")
print("-" * 28)
for product in sorted(revenues, key=revenues.get, reverse=True):
    print(f"{product:<12}{quantities[product]:>6}{revenues[product]:>10.2f}")
print("-" * 28)
print(f"{'Total':<12}{sum(quantities.values()):>6}{sum(revenues.values()):>10.2f}")
```

```sortie
Product        Qty   Revenue
----------------------------
baguette       125    137.50
tart             8    120.00
croissant       75     90.00
----------------------------
Total          208    347.50
```

```text
 {product:<12}         {quantity:>6}  {revenue:>10.2f}
 ├───── 12 ─────┤├─── 6 ──┤├────── 10 ──────┤
 baguette             125          137.50
```

`key=revenues.get` sorts the product names by their revenue: `revenues.get("tart")` is
120.0, and so on.

> **Try it:** replace `:<12` with `:>12`. The product names are then aligned to the
> right, stuck to the numbers.

> **Remember:** for a report, use the same width for the header and for the rows.
> Numbers on the right, text on the left.

## What about pandas?

For big tables, analysts use **pandas**, an external
[library](../debuter/glossaire.md#bibliotheque). It does not come with Python: you
install it with [pip](../debuter/glossaire.md#pip) (`pip install pandas`), preferably in
a [virtual environment](../debuter/glossaire.md#environnement-virtuel). The blocks
below are therefore not run on this site, but their outputs are real.

pandas stores the data in a **DataFrame**: a table, like a spreadsheet, with named
columns. The types are guessed on their own.

```python norun
import io
import pandas as pd

TEXT = """day,product,quantity,price
monday,baguette,40,1.10
monday,croissant,25,1.20
monday,tart,3,15.00
tuesday,baguette,35,1.10
tuesday,croissant,30,1.20
wednesday,baguette,50,1.10
wednesday,croissant,20,1.20
wednesday,tart,5,15.00
"""

df = pd.read_csv(io.StringIO(TEXT))
print(df.head(3))
print(df["quantity"].mean())
```

```text
      day    product  quantity  price
0  monday   baguette        40    1.1
1  monday  croissant        25    1.2
2  monday       tart         3   15.0
26.0
```

Grouping, adding up and sorting fit in one line:

```python norun
df["revenue"] = df["quantity"] * df["price"]
report = df.groupby("product")[["quantity", "revenue"]].sum()
print(report.sort_values("revenue", ascending=False))
```

```text
           quantity  revenue
product
baguette        125    137.5
tart              8    120.0
croissant        75     90.0
```

| With the standard library | With pandas |
|---|---|
| `csv.DictReader` + conversions by hand | `pd.read_csv` (types guessed) |
| `statistics.mean(list)` | `df["column"].mean()` |
| `defaultdict` + loop | `df.groupby("product").sum()` |
| `sorted(..., key=...)` | `df.sort_values("column")` |
| f-strings to align | automatic aligned display |

> **Remember:** for a few hundred rows, `csv` and `statistics` are enough and need
> nothing to install. pandas becomes useful when the tables are big or the questions
> many. Understanding the "by hand" version first helps you understand what pandas does
> for you.

## Common mistakes

### Calculating with text

```python norun
import statistics
grades = ["12", "15", "9"]
print(statistics.mean(grades))
```

```text
Traceback (most recent call last):
  ...
  File "/usr/lib/python3.13/statistics.py", line 334, in _exact_ratio
    raise TypeError(msg)
TypeError: can't convert type 'str' to numerator/denominator
```

Values read from a CSV are text. Convert first:
`statistics.mean([int(g) for g in grades])`.

### A standard deviation with a single value

```python norun
import statistics
print(statistics.stdev([12]))
```

```text
Traceback (most recent call last):
  File "main.py", line 2, in <module>
    print(statistics.stdev([12]))
          ~~~~~~~~~~~~~~~~^^^^^^
  File "/usr/lib/python3.13/statistics.py", line 1207, in stdev
    raise StatisticsError('stdev requires at least two data points')
statistics.StatisticsError: stdev requires at least two data points
```

With a single value, you cannot measure any variation. Likewise,
`statistics.mean([])` raises `StatisticsError: mean requires at least one data point`.
Check the size of the list before calculating: `if len(values) >= 2:`.

### Sorting numbers that are still text

```python
quantities = ["40", "5", "125", "8"]
print(sorted(quantities))
print(sorted(quantities, key=int))
```

```sortie
['125', '40', '5', '8']
['5', '8', '40', '125']
```

No error, but a strange order: texts are sorted **character by character**, like in a
dictionary. `"125"` starts with `1`, so it comes before `"40"`. Convert to numbers, or
sort with `key=int`.

## Test yourself

**Question 1:** a class's grades are `[2, 14, 15, 15, 16]`. Why are the mean and the
median different?

> **Answer**
>
> The mean is 12.4, the median is 15. The grade 2, very low, pulls the mean down. The
> median, on the other hand, only takes the middle value of the sorted grades.

**Question 2:** what does this program print?

```python norun
from collections import defaultdict
purchases = [("Léa", 5), ("Tom", 3), ("Léa", 2)]
total = defaultdict(int)
for name, n in purchases:
    total[name] += n
print(dict(total))
```

> **Answer**
>
> ```python
> from collections import defaultdict
> purchases = [("Léa", 5), ("Tom", 3), ("Léa", 2)]
> total = defaultdict(int)
> for name, n in purchases:
>     total[name] += n
> print(dict(total))
> ```
>
> ```sortie
> {'Léa': 7, 'Tom': 3}
> ```
>
> Léa: 0 + 5 + 2 = 7. Tom: 0 + 3 = 3.

**Question 3:** you read a CSV with `csv.DictReader`. Why does `row["quantity"] * 2`
give `'4040'` instead of `80`?

> **Answer**
>
> Because `row["quantity"]` is the text `'40'`. Multiplying a text by 2 repeats it. You
> have to convert: `int(row["quantity"]) * 2`.

## Summary

- Start from a question; it tells you which columns to look at and what to calculate.
- `csv.DictReader` reads a CSV (a file or `io.StringIO`); everything is text: convert
  with `int` and `float` right away.
- `statistics` gives `mean`, `median`, `mode` and `stdev`; compare the mean and the
  median.
- `Counter` counts the rows per group, `defaultdict(int)` adds up a column per group.
- `sorted(..., key=..., reverse=True)` ranks the results; the f-strings `:<12` and
  `:>8.2f` align a report.
- pandas does all of this in a few lines, for big tables.

Next: [regular expressions](regex.md)
