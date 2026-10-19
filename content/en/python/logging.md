# Logging with logging

This page shows how to replace your debugging `print` calls with a real log using the `logging` module: importance levels, a fixed format, one log per module, a file, and exceptions told in full.

> **Goals:**
>
> - you will know why a log is better than `print` calls;
> - you will be able to use the five levels, from `DEBUG` to `CRITICAL`;
> - you will be able to set up the display once with `basicConfig`;
> - you will be able to create one log per module with `getLogger`;
> - you will be able to write the log to a file;
> - you will be able to record an exception with its traceback.

## Why not print?

The captain of a ship keeps a **logbook**. They write down the time, the weather, the
incidents. They do not write everything in the same ink: a serious incident is
underlined in red. And months later, people read the logbook again to understand what
happened.

A program can keep the same kind of log. With `print`, you write messages, but:

- they all look the same: you cannot tell which one is serious;
- to hide them, you have to delete them one by one;
- they only go to the screen, never to a file;
- they do not say where they come from.

The `logging` module solves these four problems.

```text
     print("ok")                       log.info("ok")
         │                                  │
         ▼                                  ▼
     the screen                   ┌─ which level? ──┐
                                  │ serious enough? │── no ──► ignored
                                  └────────┬────────┘
                                           │ yes
                                   ┌───────┴────────┐
                                   ▼                ▼
                               the screen        a file
```

The [debugging](debogage.md) chapter gave you a first look. Here, we go further.

> **Remember:** `print` is for showing the **result** to the user. `logging` is for
> telling **what** the program **is doing**, for you or for the person who watches over
> it.

## A first log

Three lines are enough to get started.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("shop")

log.info("opening the shop")
log.warning("only 2 baguettes left")
print("Hello!")
```

```sortie
INFO:shop:opening the shop
WARNING:shop:only 2 baguettes left
Hello!
```

Line by line:

| Code | What it does |
|---|---|
| `logging.basicConfig(...)` | sets up the log, **once**, at the start of the program |
| `stream=sys.stdout` | messages go to the normal output (by default: the error output) |
| `level=logging.INFO` | we keep `INFO` messages and more serious ones |
| `format="..."` | the shape of each line (see below) |
| `logging.getLogger("shop")` | gets a log named `shop` |
| `log.info(...)`, `log.warning(...)` | writes a message, with its level |

> **Note:** on this page, `stream=sys.stdout` is there so that you see the messages
> mixed with the `print` output. In a real program, you often keep the default error
> output: that way the log does not get mixed with the result.

## The five levels

Each message has a **level**: how serious it is. It is like weather alerts, from green
to red.

| Level | Number | When to use it | Example |
|---|---|---|---|
| `DEBUG` | 10 | small details for troubleshooting | "x is 3" |
| `INFO` | 20 | normal steps | "file loaded" |
| `WARNING` | 30 | odd, but we carry on | "low stock" |
| `ERROR` | 40 | an operation failed | "payment declined" |
| `CRITICAL` | 50 | the program cannot go on | "disk full" |

The `level` setting is a **threshold**. A message gets through if it is **at least as
serious** as the threshold.

```text
 threshold = WARNING (30)

 CRITICAL 50  ████████████  passes
 ERROR    40  ██████████    passes
 WARNING  30  ████████      passes   ◄── threshold
 INFO     20  ██████        ignored
 DEBUG    10  ████          ignored
```

Look at the same program with the `WARNING` threshold:

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.WARNING,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("weather")

log.debug("sensor read: 18.4")
log.info("reading finished")
log.warning("strong wind")
log.error("rain gauge out of order")
log.critical("station destroyed")
```

```sortie
WARNING:weather:strong wind
ERROR:weather:rain gauge out of order
CRITICAL:weather:station destroyed
```

Let's follow each message:

| Message | Level | Threshold | 30 ≤ level? | Shown? |
|---|---|---|---|---|
| sensor read | 10 | 30 | no | no |
| reading finished | 20 | 30 | no | no |
| strong wind | 30 | 30 | yes | yes |
| rain gauge out of order | 40 | 30 | yes | yes |
| station destroyed | 50 | 30 | yes | yes |

> **Try it:** replace `level=logging.WARNING` with `level=logging.DEBUG`. All five
> lines are shown. With `level=logging.CRITICAL`, only one is left.

Without `level`, the default threshold is `WARNING`. That is why your `log.info` calls
sometimes seem to "do nothing".

> **Remember:** you write **all** the messages in the code, each one with the right
> level. Then **a single** setting, `level`, chooses which ones you see. You no longer
> delete lines to clean up.

## The format of a line

The `format` is a fill-in-the-blanks template. Each `%(name)s` is replaced by a piece of
information about the message.

| Blank | Replaced by |
|---|---|
| `%(levelname)s` | the level: `INFO`, `ERROR`… |
| `%(name)s` | the name of the log: `shop` |
| `%(message)s` | the text of the message |
| `%(funcName)s` | the name of the function that wrote it |
| `%(lineno)d` | the line number |
| `%(asctime)s` | the date and time |

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.DEBUG,
                    format="[%(levelname)s] %(funcName)s: %(message)s")
log = logging.getLogger("till")

def take_payment(amount):
    log.debug("amount received %s", amount)
    log.info("paid")

take_payment(12)
```

```sortie
[DEBUG] take_payment: amount received 12
[INFO] take_payment: paid
```

> **Note:** in a real program, you almost always add `%(asctime)s` to know **when** it
> happened. This page does not use it: the time changes every time you run the program,
> and the outputs shown here must stay the same.

## Passing values with %s

To put a value in a message, write `%s` in the text and give the value afterwards,
separated by a comma.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("cart")

item = "apples"
quantity = 3
log.info("adding %s %s", quantity, item)
log.debug("very long calculation: %s", sum(range(1000)))
```

```sortie
INFO:cart:adding 3 apples
```

The first `%s` gets `quantity`, the second gets `item`. It is the same idea as the
fill-in form of [sqlite3](sqlite.md).

Why not an f-string? With `%s`, the text is built **only if** the message passes the
threshold. An ignored `DEBUG` message costs almost nothing. An f-string, on the other
hand, is always built, even for nothing.

> **Warning:** `sum(range(1000))` is still calculated, because Python evaluates the
> arguments before the call. Only the formatting of the text is avoided.

> **Remember:** write `log.info("total: %s", total)`, not
> `log.info(f"total: {total}")`. As many `%s` as values.

## One log per module

In a big program, each [module](modules.md) has its own log. You give it the name of
the module with `__name__`. That way, each line says **where** it comes from.

```python norun
# file cart.py
import logging

log = logging.getLogger(__name__)   # the name will be "cart"

def add(item):
    log.info("adding %s", item)
```

In the main program, `__name__` is `"__main__"`. Only this main file calls
`basicConfig`. The modules just use `getLogger(__name__)`.

```text
main program     ── basicConfig(...)  ← a single setup
   │
   ├── cart.py      ── getLogger("cart")
   ├── payment.py   ── getLogger("payment")
   └── stock.py     ── getLogger("stock")
```

### Names in a family

Names with a dot form a family: `app.db` is the child of `app`. You can set the level
of each log separately.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")

log_app = logging.getLogger("app")
log_db = logging.getLogger("app.db")
log_web = logging.getLogger("app.web")

log_db.setLevel(logging.DEBUG)

log_app.info("starting")
log_db.debug("SELECT query sent")
log_web.debug("page requested")
log_web.info("page sent")
print(log_db.parent.name)
```

```sortie
INFO:app:starting
DEBUG:app.db:SELECT query sent
INFO:app.web:page sent
app
```

| Log | Level set | Threshold used | `debug` shown? |
|---|---|---|---|
| `app` | none | the one from `basicConfig`: INFO | no |
| `app.db` | DEBUG | DEBUG | yes |
| `app.web` | none | INFO (inherited) | no |

This is handy: are you investigating the database? You set **only** `app.db` to
`DEBUG`, without drowning in the details of everything else.

> **Try it:** add `log_web.setLevel(logging.WARNING)`. The `page sent` line disappears
> too.

> **Remember:** in each module, `log = logging.getLogger(__name__)`. A single
> `basicConfig`, in the main program.

## Writing the log to a file

A log on the screen disappears when you close the terminal. To keep it, you send it to
a [file](fichiers.md).

### With basicConfig

The simplest way: `filename=` instead of `stream=`.

```python
import logging

logging.basicConfig(filename="journal.log", level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s", encoding="utf-8")
log = logging.getLogger("backup")

log.info("copying 3 files")
log.warning("1 file skipped")
logging.shutdown()

with open("journal.log", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
INFO:backup:copying 3 files
WARNING:backup:1 file skipped
```

Nothing is shown while the program runs: everything goes to `journal.log`. We read the
file again at the end to check. `logging.shutdown()` closes the file properly.

> **Note:** by default, the file is appended to on every run (mode `"a"`). Add
> `filemode="w"` to start again from an empty file.

### With handlers: the screen and the file

A **handler** is a destination for messages. A log can have several handlers, each with
its own threshold. It is like a letter sent with a copy: one copy for you, one copy for
the archives.

A common example: on the screen, only the problems; in the file, everything.

```python
import logging
import sys

log = logging.getLogger("robot")
log.setLevel(logging.DEBUG)
layout = logging.Formatter("%(levelname)s:%(name)s:%(message)s")

screen = logging.StreamHandler(sys.stdout)
screen.setLevel(logging.WARNING)
screen.setFormatter(layout)
log.addHandler(screen)

file = logging.FileHandler("robot.log", mode="w", encoding="utf-8")
file.setLevel(logging.DEBUG)
file.setFormatter(layout)
log.addHandler(file)

log.debug("left motor: 40 %")
log.info("moving forward 2 m")
log.warning("obstacle detected")

file.close()
print("--- contents of robot.log ---")
with open("robot.log", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
WARNING:robot:obstacle detected
--- contents of robot.log ---
DEBUG:robot:left motor: 40 %
INFO:robot:moving forward 2 m
WARNING:robot:obstacle detected
```

Let's follow each message through the two handlers:

| Message | Level | Screen (threshold WARNING) | File (threshold DEBUG) |
|---|---|---|---|
| left motor | DEBUG | no | yes |
| moving forward 2 m | INFO | no | yes |
| obstacle detected | WARNING | yes | yes |

```text
                    log.warning("obstacle detected")
                               │
                    log "robot" (threshold DEBUG)
                       ┌───────┴────────┐
                       ▼                ▼
             StreamHandler          FileHandler
           threshold WARNING      threshold DEBUG
                   │                    │
               the screen           robot.log
```

So there are **two** filters: the threshold of the log (`log.setLevel`), then the
threshold of each handler. A message must pass both.

> **Try it:** set the screen threshold to `logging.DEBUG`. The three messages are then
> also shown on the screen, before the contents of the file.

> **Remember:** `basicConfig(filename=...)` for a simple file. Handlers when you want
> several destinations with different thresholds.

## Logging an exception

When an [exception](../debuter/glossaire.md#exception) is caught, you need to keep a
record of **what** failed. There are two ways.

### The error message with log.error

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("import")

lines = ["12", "8", "twelve", "5"]
total = 0
for number, text in enumerate(lines, start=1):
    try:
        total += int(text)
    except ValueError as error:
        log.error("line %s skipped: %s", number, error)

log.info("total = %s", total)
```

```sortie
ERROR:import:line 3 skipped: invalid literal for int() with base 10: 'twelve'
INFO:import:total = 25
```

| Round | `number` | `text` | `int(text)` | `total` |
|---|---|---|---|---|
| 1 | 1 | "12" | 12 | 12 |
| 2 | 2 | "8" | 8 | 20 |
| 3 | 3 | "twelve" | `ValueError` → ERROR message | 20 |
| 4 | 4 | "5" | 5 | 25 |

The program does not stop. But the faulty line is recorded in the log.

### The whole traceback with log.exception

`log.exception` writes a message at the `ERROR` level **and** the full traceback of the
error, as Python would have shown it. You only call it inside an `except`.

```python norun
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("calc")

def divide(a, b):
    return a / b

try:
    divide(10, 0)
except ZeroDivisionError:
    log.exception("the division failed")
print("the program goes on")
```

```text
ERROR:calc:the division failed
Traceback (most recent call last):
  File "main.py", line 12, in <module>
    divide(10, 0)
    ~~~~~~^^^^^^^
  File "main.py", line 9, in divide
    return a / b
           ~~^~~
ZeroDivisionError: division by zero
the program goes on
```

The traceback tells you exactly where to look: line 9, in `divide`. And the program goes
on. The chapter on [errors](erreurs.md) explains `try` and `except`.

> **Remember:** in an `except`, `log.exception("...")` keeps the full traceback. Without
> a traceback, an error recorded as "it failed" will not help you fix it.

## A complete example

A small program that processes orders. It tells what it is doing, without a single
debugging `print`.

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.INFO,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("orders")

stock = {"pen": 10, "notebook": 2}

def order(item, quantity):
    log.debug("request: %s x %s", quantity, item)
    if item not in stock:
        log.error("unknown item: %s", item)
        return False
    if stock[item] < quantity:
        log.warning("not enough stock for %s (%s requested, %s in stock)",
                    item, quantity, stock[item])
        return False
    stock[item] -= quantity
    log.info("%s x %s shipped", quantity, item)
    return True

order("pen", 3)
order("notebook", 5)
order("eraser", 1)
print("Final stock:", stock)
```

```sortie
INFO:orders:3 x pen shipped
WARNING:orders:not enough stock for notebook (5 requested, 2 in stock)
ERROR:orders:unknown item: eraser
Final stock: {'pen': 7, 'notebook': 2}
```

| Call | Item known? | Enough stock? | Level written | `stock` afterwards |
|---|---|---|---|---|
| `order("pen", 3)` | yes | yes (10 ≥ 3) | INFO | pen 7, notebook 2 |
| `order("notebook", 5)` | yes | no (2 < 5) | WARNING | pen 7, notebook 2 |
| `order("eraser", 1)` | no | — | ERROR | pen 7, notebook 2 |

> **Try it:** set `level` to `logging.DEBUG`. Each order first shows its
> `DEBUG:orders:request: …` line.

## Common mistakes

### More %s than values (or the other way round)

```python norun
import logging
import sys
logging.basicConfig(stream=sys.stdout, format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("app")
log.warning("Hello %s", "Lea", "Hugo")
```

```text
--- Logging error ---
Traceback (most recent call last):
  File "/usr/lib/python3.13/logging/__init__.py", line 1151, in emit
    msg = self.format(record)
  ...
TypeError: not all arguments converted during string formatting
Call stack:
  File "main.py", line 5, in <module>
    log.warning("Hello %s", "Lea", "Hugo")
Message: 'Hello %s'
Arguments: ('Lea', 'Hugo')
```

Only one `%s` but two values. The program does not crash, but the message is lost.
Count them: as many `%s` as values, `"Hello %s and %s"`.

### Calling basicConfig twice

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, level=logging.WARNING,
                    format="%(levelname)s:%(name)s:%(message)s")
logging.basicConfig(stream=sys.stdout, level=logging.DEBUG,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("app")
log.debug("am I here?")
log.warning("me, yes")
```

```sortie
WARNING:app:me, yes
```

No error, but the second `basicConfig` is ignored: the log was already set up. A single
`basicConfig`, at the very beginning. To really set everything up again, add
`force=True`.

### log.exception outside an except

```python
import logging
import sys

logging.basicConfig(stream=sys.stdout, format="%(levelname)s:%(name)s:%(message)s")
logging.getLogger("app").exception("something is wrong")
```

```sortie
ERROR:app:something is wrong
NoneType: None
```

No exception is in progress: there is no traceback to show, hence the strange
`NoneType: None`. Outside an `except`, use `log.error`.

## Test yourself

**Question 1:** the threshold is `INFO`. Which messages are shown among `debug`, `info`,
`warning` and `error`?

> **Answer**
>
> `info`, `warning` and `error`. Only `debug` (10) is below the `INFO` (20) threshold.

**Question 2:** what does this program print?

```python norun
import logging
import sys
logging.basicConfig(stream=sys.stdout, level=logging.ERROR,
                    format="%(levelname)s:%(name)s:%(message)s")
log = logging.getLogger("test")
log.warning("watch out")
log.error("failure %s", 42)
```

> **Answer**
>
> ```python
> import logging
> import sys
> logging.basicConfig(stream=sys.stdout, level=logging.ERROR,
>                     format="%(levelname)s:%(name)s:%(message)s")
> log = logging.getLogger("test")
> log.warning("watch out")
> log.error("failure %s", 42)
> ```
>
> ```sortie
> ERROR:test:failure 42
> ```
>
> `WARNING` (30) is below the `ERROR` (40) threshold: it is ignored. The `%s` gets 42.

**Question 3:** you want to see the errors on the screen, but keep **all** the details in
a file. What do you do?

> **Answer**
>
> A log with the `DEBUG` threshold and two handlers: a `StreamHandler` with the
> `WARNING` (or `ERROR`) threshold for the screen, and a `FileHandler` with the `DEBUG`
> threshold for the file.

## Summary

- `print` shows the result; `logging` tells what the program is doing.
- Five levels: `DEBUG` < `INFO` < `WARNING` < `ERROR` < `CRITICAL`. The `level`
  threshold chooses what gets through; by default, it is `WARNING`.
- A single `basicConfig`, at the start of the main program; `getLogger(__name__)` in
  each module.
- Values with `%s`: `log.info("total: %s", total)`.
- `filename=` or a `FileHandler` to keep the log in a file.
- In an `except`, `log.exception` keeps the full traceback of the error.

Next: [writing good Python](style.md)
