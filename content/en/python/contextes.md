# Context managers

This page explains how `with` opens and releases a resource for sure, and how to write your own context managers with a class or with `contextlib`.

> **Goals:**
>
> - you will know why `with` is safer than a `close()` written by hand;
> - you will be able to open several resources in a single `with`;
> - you will be able to write a manager with a class, `__enter__` and `__exit__`;
> - you will be able to write a manager with `@contextmanager` and `yield`;
> - you will be able to use `suppress`, `redirect_stdout`, `ExitStack` and
>   `TemporaryDirectory`.

## The problem: forgetting to clean up

Some things must be **given back** after use: an open file must be closed, a connection
cut, a lock released. Such a thing is called a **resource**.

Closing by hand looks simple:

```python
f = open("notes.txt", "w", encoding="utf-8")
f.write("15\n")
f.close()
print(f.closed)
```

```sortie
True
```

But if an [error](../debuter/glossaire.md#error) happens between `open` and `close`,
the line `f.close()` is never reached:

```python
f = open("notes.txt", "w", encoding="utf-8")
try:
    f.write("15\n")
    1 / 0
    f.close()
except ZeroDivisionError:
    print("error!")
print("closed?", f.closed)
```

```sortie
error!
closed? False
```

The file stays open. In a real program, forgotten files end up blocking the system, or
some data is never written to the disk.

## The solution: with

`with` guarantees the cleanup. When the indented block ends, the resource is released,
**whether there was an error or not**. It is like a door that closes by itself behind
you.

```python
try:
    with open("notes.txt", "w", encoding="utf-8") as f:
        f.write("15\n")
        1 / 0
except ZeroDivisionError:
    print("error!")
print("closed?", f.closed)
```

```sortie
error!
closed? True
```

This time, the file is closed despite the error. The object placed after `with` (here,
the file) is called a **context manager**: it knows what to do when entering and when
leaving the block.

> **Remember:** `with` does not replace `try` / `except`. It does not catch the error:
> it cleans up, then lets the error go on its way.

## with open(...) as f

This is the most common use, seen in [reading and writing files](fichiers.md). Read the
line from left to right:

- `open(...)` creates the context manager;
- `as f` gives a name to what it provides (the open file);
- the indented block is the time during which the file is open.

```python
with open("fruits.txt", "w", encoding="utf-8") as f:
    f.write("apple\npear\n")

with open("fruits.txt", encoding="utf-8") as f:
    for line in f:
        print(line.strip())

print(f.closed)
```

```sortie
apple
pear
True
```

The variable `f` still exists after the block, but the file is closed: you can no longer
read it.

## Several contexts in a single with

To open two files at once, separate them with a comma. They are opened from left to
right, and closed in reverse order.

```python
with open("source.txt", "w", encoding="utf-8") as f:
    f.write("hello\ngoodbye\n")

with open("source.txt", encoding="utf-8") as source, \
     open("copy.txt", "w", encoding="utf-8") as copy:
    for line in source:
        copy.write(line.upper())

with open("copy.txt", encoding="utf-8") as f:
    print(f.read(), end="")
```

```sortie
HELLO
GOODBYE
```

## Writing a manager with a class

Any object can be used after `with`, as long as it has two special methods:

- `__enter__(self)` is called when **entering** the block. What it returns is stored in
  the variable after `as`;
- `__exit__(self, err_type, err, trace)` is called when **leaving** the block, always.

```python
class Section:
    def __init__(self, title):
        self.title = title

    def __enter__(self):
        print(f"[start {self.title}]")
        return self

    def __exit__(self, err_type, err, trace):
        print(f"[end {self.title}]")

with Section("calculation") as s:
    print("in the block:", s.title)
    print(2 + 2)
```

```sortie
[start calculation]
in the block: calculation
4
[end calculation]
```

Python does exactly this: it calls `__enter__`, runs the block, then calls `__exit__`,
even if the block raised an [exception](../debuter/glossaire.md#exception).

> **Warning:** do not forget `return self` in `__enter__`. Without `return`, it returns
> `None`, and the variable after `as` is `None`.

### What __exit__ receives

`__exit__` receives three arguments that describe the possible error:

| Argument | If there is no error | If there is an error |
|----------|----------------------|----------------------|
| `err_type` | `None` | the class, for example `ZeroDivisionError` |
| `err` | `None` | the exception object itself |
| `trace` | `None` | the *traceback* |

```python
class Spy:
    def __enter__(self):
        return self

    def __exit__(self, err_type, err, trace):
        if err_type is None:
            print("normal exit")
        else:
            print("exit on error:", err_type.__name__, "-", err)

with Spy():
    print("all is well")

try:
    with Spy():
        int("abc")
except ValueError:
    print("the error went on its way")
```

```sortie
all is well
normal exit
exit on error: ValueError - invalid literal for int() with base 10: 'abc'
the error went on its way
```

`__exit__` **sees** the error, but does not stop it: the error then goes up to the
`except`.

### Returning True to swallow the error

The value returned by `__exit__` matters:

- if it returns a **true** value (`True`), Python considers the error handled: it
  disappears, and the program continues **after** the `with` block;
- if it returns a false value (`False`, or `None` when there is no `return`), the error
  keeps going up.

```python
class IgnoreZero:
    def __enter__(self):
        return self

    def __exit__(self, err_type, err, trace):
        if err_type is ZeroDivisionError:
            print("division by zero ignored")
            return True
        return False

with IgnoreZero():
    print("before")
    1 / 0
    print("never printed")
print("after the with")
```

```sortie
before
division by zero ignored
after the with
```

Notice two things:

1. `"never printed"` is not printed. Swallowing the error does not "resume" the block
   where it stopped: the rest of the block is lost.
2. Only `ZeroDivisionError` is swallowed. Any other error, such as `ValueError`, gets
   `return False` and goes up normally.

> **Warning:** only swallow the **specific** errors you expect. An `__exit__` that
> always returns `True` hides every bug in the block, like a bare `except:`.

## contextmanager: a manager with yield

Writing a class for each manager is a bit long. The
[decorator](../debuter/glossaire.md#decorator) `@contextmanager` from the `contextlib`
module turns a [generator](../debuter/glossaire.md#generator) into a manager:

- the code **before** `yield` plays the role of `__enter__`;
- the value given to `yield` is stored in the variable after `as`;
- the code **after** `yield` plays the role of `__exit__`.

```python
from contextlib import contextmanager

@contextmanager
def tag(name):
    print(f"<{name}>")
    yield name.upper()
    print(f"</{name}>")

with tag("p") as t:
    print("  hello", t)
```

```sortie
<p>
  hello P
</p>
```

### Always with try / finally

If an error happens in the `with` block, Python raises it again **at the `yield`**. So,
without care, the code after `yield` is not run. Put the cleanup in a `finally`:

```python
from contextlib import contextmanager

@contextmanager
def tag(name):
    print(f"<{name}>")
    try:
        yield
    finally:
        print(f"</{name}>")

try:
    with tag("div"):
        print("  content")
        1 / 0
except ZeroDivisionError:
    print("error caught")
```

```sortie
<div>
  content
</div>
error caught
```

`</div>` is printed despite the error: the cleanup is guaranteed.

> **Remember:** a generator decorated with `@contextmanager` must do **a single**
> `yield`, surrounded by `try` / `finally`.

## suppress: ignoring a specific error

`contextlib.suppress` does what our `IgnoreZero` class did: it swallows the errors of
the given classes, and only those.

```python
from contextlib import suppress
from pathlib import Path

with suppress(FileNotFoundError):
    Path("draft.txt").unlink()
    print("file deleted")
print("moving on")
```

```sortie
moving on
```

The file did not exist: `unlink` raised `FileNotFoundError`, `suppress` swallowed it,
and the line `"file deleted"` was skipped. It is a shortcut for a `try` /
`except FileNotFoundError: pass`.

## redirect_stdout: capturing what print displays

`print` writes to the **standard output** (*stdout*), usually the screen.
`contextlib.redirect_stdout` sends it somewhere else for the duration of the block. With
`io.StringIO`, a kind of file kept in memory, you get the text back in a string.

```python
import io
from contextlib import redirect_stdout

def greet():
    print("Hello!")
    print("Goodbye!")

buffer = io.StringIO()
with redirect_stdout(buffer):
    greet()

text = buffer.getvalue()
print("captured:", repr(text))
print(len(text.splitlines()), "lines")
```

```sortie
captured: 'Hello!\nGoodbye!\n'
2 lines
```

During the block, nothing appears on the screen. When it ends, `print` writes normally
again. It is very handy to [test](tests.md) a function that prints.

## ExitStack: a variable number of contexts

When the number of files is only known at run time, you cannot write them all in the
`with`. `contextlib.ExitStack` collects them with `enter_context`, then closes them all
at the end of the block, in reverse order.

```python
from contextlib import ExitStack

names = ["monday.txt", "tuesday.txt", "wednesday.txt"]

with ExitStack() as stack:
    files = [stack.enter_context(open(name, "w", encoding="utf-8")) for name in names]
    for name, f in zip(names, files):
        f.write(f"journal of {name}\n")
    print(len(files), "files open")

print(all(f.closed for f in files))
```

```sortie
3 files open
True
```

> **Note:** you will not often need `ExitStack`. Just remember that it exists for the
> day the number of resources varies.

## Practical example: a temporary folder

`tempfile.TemporaryDirectory` creates an empty folder, with a randomly chosen name, and
**deletes it with all its contents** when the block ends. It is ideal for throwaway
working files.

```python
import tempfile
from pathlib import Path

with tempfile.TemporaryDirectory() as folder_name:
    folder = Path(folder_name)
    (folder / "a.txt").write_text("one", encoding="utf-8")
    (folder / "b.txt").write_text("two", encoding="utf-8")
    print(sorted(f.name for f in folder.iterdir()))
    print("exists:", folder.exists())

print("exists after:", folder.exists())
```

```sortie
['a.txt', 'b.txt']
exists: True
exists after: False
```

The folder name changes every time (for example `/tmp/tmpk3x9a1`), so it is not
printed. Only what does not depend on chance is printed.

## Common mistakes

### Using with on an object that is not a manager

```python norun
with "notes.txt" as f:
    print(f.read())
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    with "notes.txt" as f:
         ^^^^^^^^^^^
TypeError: 'str' object does not support the context manager protocol
```

A string has neither `__enter__` nor `__exit__`. `open` is missing:
`with open("notes.txt", encoding="utf-8") as f:`.

### Forgetting return self in __enter__

```python norun
class Counter:
    def __init__(self):
        self.total = 0

    def __enter__(self):
        print("start")

    def __exit__(self, err_type, err, trace):
        print("end")

with Counter() as c:
    c.total += 1
```

```text
start
end
Traceback (most recent call last):
  File "main.py", line 12, in <module>
    c.total += 1
    ^^^^^^^
AttributeError: 'NoneType' object has no attribute 'total'
```

`__enter__` returns nothing, so `c` is `None`. Add `return self` at the end of
`__enter__`.

### Forgetting the parameters of __exit__

```python norun
class Section:
    def __enter__(self):
        return self

    def __exit__(self):
        print("end")

with Section():
    print("inside")
```

```text
inside
Traceback (most recent call last):
  File "main.py", line 8, in <module>
    with Section():
         ~~~~~~~^^
TypeError: Section.__exit__() takes 1 positional argument but 4 were given
```

Python always calls `__exit__` with three arguments on top of `self`. Write
`def __exit__(self, err_type, err, trace):`, even if you do not use them.

## Test yourself

**Question 1:** what does this code print?

```python norun
class Door:
    def __enter__(self):
        print("open")
        return self

    def __exit__(self, err_type, err, trace):
        print("close")

try:
    with Door():
        print("enter")
        raise ValueError("oops")
except ValueError:
    print("caught")
```

> **Answer**
>
> `open`, `enter`, `close`, then `caught`. `__exit__` is called before the error reaches
> the `except`. It returns nothing (so `None`, a false value): the error is not
> swallowed.

**Question 2:** in a function decorated with `@contextmanager`, why surround the `yield`
with `try` / `finally`?

> **Answer**
>
> Because an error in the `with` block is raised again at the `yield`. Without
> `finally`, the cleanup code placed after `yield` would not be run.

**Question 3:** using `suppress`, write code that converts `"abc"` to an integer, and
prints `"done"` without crashing.

> **Answer**
>
> ```python
> from contextlib import suppress
>
> with suppress(ValueError):
>     n = int("abc")
>     print("converted:", n)
> print("done")
> ```
>
> ```sortie
> done
> ```

## Summary

- `with` releases a resource when the block ends, even in case of error; it does not
  catch the error.
- `with a as x, b as y:` opens several resources, closed in reverse order.
- A class becomes a manager with `__enter__` (its value goes into `as`) and
  `__exit__(self, err_type, err, trace)`.
- If `__exit__` returns `True`, the error is swallowed and the program resumes after the
  block; otherwise, it goes up.
- `@contextmanager`: the code before `yield` runs on entry, the code after `yield` (in
  `finally`) on exit.
- `contextlib` also provides `suppress`, `redirect_stdout` and `ExitStack`;
  `tempfile.TemporaryDirectory` creates a throwaway folder.

Next: [modules](modules.md).
