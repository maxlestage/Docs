# Modules, packages and pip

This page explains how to import code, split your program into several files, use the standard library and install packages with pip in a virtual environment.

> **Goals:**
>
> - you will be able to use the different forms of `import`;
> - you will be able to write your own module and use `if __name__ == "__main__"`;
> - you will be able to organize modules into a package with `__init__.py`;
> - you will know how to find help in the standard library;
> - you will be able to create a virtual environment and install packages with pip.

## What is a module?

A [module](../debuter/glossaire.md#module) is simply **a `.py` file** whose contents you
reuse: its functions, its classes, its variables. Python comes with hundreds of
ready-to-use modules: this is the **standard library**.

A module is a toolbox stored in the workshop. `import` fetches the box; then you take
the tools you need.

## The forms of import

### import module

The simplest form loads the whole module. You then use its items with a dot:
`module.name`.

```python
import math

print(math.sqrt(16))
print(math.pi)
print(math.floor(3.7))
```

```sortie
4.0
3.141592653589793
3
```

The `math.` prefix shows clearly where each function comes from.

### from module import name

To use an item without a prefix, import it directly:

```python
from math import sqrt, pi

print(sqrt(25))
print(round(pi, 2))
```

```sortie
5.0
3.14
```

### Renaming with as

`as` gives another name, shorter or clearer, to the imported module or item:

```python
import statistics as stats
from collections import Counter as Tally

grades = [12, 15, 9, 15, 18]
print(stats.mean(grades))
print(stats.median(grades))
print(Tally(grades).most_common(1))
```

```sortie
13.8
15
[(15, 2)]
```

| Form | Usage | When? |
|------|-------|-------|
| `import math` | `math.sqrt(4)` | the default case, the clearest |
| `from math import sqrt` | `sqrt(4)` | one or two items used often |
| `import numpy as np` | `np.array(...)` | a long name, with a usual alias |
| `from math import *` | `sqrt(4)` | **avoid it**: you no longer know where names come from |

> **Tip:** put all the `import` lines at the top of the file: you see right away what
> the program needs.

## Writing your own module

When a program grows, you split it into several files. Create two files in the **same
folder**:

```text
calc/
├── geometry.py
└── main.py
```

The file `geometry.py` contains functions:

```python norun
"""Small geometry functions."""

PI = 3.14159


def circle_area(radius):
    return PI * radius ** 2


def circle_perimeter(radius):
    return 2 * PI * radius
```

The file `main.py` imports them by the file name, **without** `.py`:

```python norun
import geometry
from geometry import circle_perimeter

print(geometry.circle_area(2))
print(round(circle_perimeter(1), 2))
```

You run the main program from the `calc` folder:

```sh
python3 main.py
```

```text
12.56636
6.28
```

Python looks for modules first in the **folder of the program you run**, then in the
standard library, then in the installed packages.

## if __name__ == "__main__"

When Python imports a module, it **runs all its code**, from top to bottom. Every
module has a special variable, `__name__`:

- if the file is run directly (`python3 geometry.py`), `__name__` is `"__main__"`;
- if it is imported, `__name__` is the module name, `"geometry"`.

Let's add at the end of `geometry.py`:

```python norun
print("geometry.py is loaded, __name__ =", __name__)

if __name__ == "__main__":
    print("Quick test:", circle_area(1))
```

The result depends on how you run the file:

```sh
python3 main.py
```

```text
geometry.py is loaded, __name__ = geometry
12.56636
6.28
```

```sh
python3 geometry.py
```

```text
geometry.py is loaded, __name__ = __main__
Quick test: 3.14159
```

> **Remember:** the code under `if __name__ == "__main__":` runs **only** when the file
> is run directly. You put quick tests or the program's starting point there, so that
> importing stays silent.

A well-organized Python program often looks like this:

```python
def main():
    print("The program starts")

if __name__ == "__main__":
    main()
```

```sortie
The program starts
```

## Packages

A [package](../debuter/glossaire.md#package) is a **folder of modules**. It contains an
`__init__.py` file, run the first time the package is imported. This file can be
empty.

```text
shop/
├── main.py
└── store/
    ├── __init__.py
    ├── prices.py
    └── stock.py
```

```python norun
# store/prices.py
VAT = 0.20


def with_vat(amount):
    return round(amount * (1 + VAT), 2)
```

```python norun
# store/stock.py
STOCK = {"pen": 12, "notebook": 0}


def available(item):
    return STOCK.get(item, 0) > 0
```

In `__init__.py`, you can bring up the main items to shorten imports. The dot in
`.prices` means "the `prices` module of **this** package":

```python norun
# store/__init__.py
from .prices import with_vat
```

You then import with dots, which follow the folders:

```python norun
# main.py
from store import with_vat
from store.stock import available
import store.prices

print(with_vat(10))
print(available("pen"), available("notebook"))
print(store.prices.VAT)
```

```text
12.0
True False
0.2
```

## A tour of the standard library

The standard library covers most everyday needs, without installing anything. A few
modules worth knowing:

| Module | For… |
|--------|------|
| `math`, `statistics` | calculations, means, medians |
| `random` | random draws |
| `datetime` | dates and times (see [dates](dates.md)) |
| `pathlib`, `os` | paths and files (see [files](fichiers.md)) |
| `json`, `csv` | data formats (see [formats](formats.md)) |
| `collections` | `Counter`, `defaultdict`, `deque` |
| `itertools`, `functools` | tools for iterators and functions |
| `re` | [regular expressions](regex.md) |
| `sys`, `argparse` | [command line](ligne-de-commande.md) |
| `unittest` | [tests](tests.md) |

A small sample:

```python
from collections import Counter, defaultdict
from datetime import date
from pathlib import Path
import json

print(Counter("abracadabra").most_common(2))

by_letter = defaultdict(list)
for word in ["ant", "bus", "arc"]:
    by_letter[word[0]].append(word)
print(dict(by_letter))

print(date(2024, 7, 14).strftime("%d/%m/%Y"))
print(Path("photos/holidays/beach.jpg").suffix)
print(json.dumps({"name": "Zoé", "age": 9}, ensure_ascii=False))
```

```sortie
[('a', 5), ('b', 2)]
{'a': ['ant', 'arc'], 'b': ['bus']}
14/07/2024
.jpg
{"name": "Zoé", "age": 9}
```

> **Tip:** to explore a module, use `help(math)` or `dir(math)` in the Python console,
> or the official documentation at docs.python.org.

## Installing packages with pip

Hundreds of thousands of packages written by other people are published on **PyPI**
(*Python Package Index*, pypi.org). The **pip** tool downloads and installs them.

Example with `requests`, a very popular package for talking to websites:

```sh
python3 -m pip install requests
```

```text
Collecting requests
  Using cached requests-2.34.2-py3-none-any.whl.metadata (4.8 kB)
...
Installing collected packages: urllib3, idna, charset_normalizer, certifi, requests
Successfully installed certifi-2026.10.18 charset_normalizer-3.5.2 idna-3.20 requests-2.34.2 urllib3-2.8.0
```

pip also installed the **dependencies** of `requests`: the packages it needs to work.
Then you import it like a module from the standard library:

```python norun
import requests

response = requests.get("https://example.com")
print(response.status_code)
```

```text
200
```

A few useful pip commands:

```sh
python3 -m pip list                 # the installed packages
python3 -m pip show requests        # details about a package
python3 -m pip install --upgrade requests
python3 -m pip uninstall requests
```

> **Note:** write `python3 -m pip` rather than plain `pip`: you are sure to install for
> **this** Python, if there are several on the computer.

## Virtual environments

If all your projects install their packages in the same place, they end up getting in
each other's way: project A wants version 1 of a package, project B wants version 2. A
**virtual environment** is a folder that contains a Python and packages **specific to
one project**. Each project has its own box, isolated from the others.

You create it with the `venv` module, usually in a folder named `.venv`:

```sh
cd my-project
python3 -m venv .venv
```

Then you **activate** it, once per terminal:

```sh
# Linux and macOS
source .venv/bin/activate

# Windows (PowerShell)
.venv\Scripts\Activate.ps1
```

The name of the environment then appears at the start of the prompt, for example
`(.venv) $`. From now on, `python` and `pip` are the ones from the environment:

```sh
python -m pip install requests
deactivate
```

`deactivate` leaves the environment.

> **Warning:** do not put the `.venv` folder in [Git](../debuter/git.md): add it to the
> `.gitignore` file. You recreate it from `requirements.txt`.

## requirements.txt

So that someone else (or you, on another computer) installs exactly the same packages,
you list them in a `requirements.txt` file. `pip freeze` shows the installed packages
with their version:

```sh
python -m pip freeze > requirements.txt
```

```text
certifi==2026.10.18
charset-normalizer==3.5.2
idna==3.20
requests==2.34.2
urllib3==2.8.0
```

On another machine, in a new virtual environment, a single command reinstalls
everything:

```sh
python -m pip install -r requirements.txt
```

## Common mistakes

### Importing a package that is not installed

```python norun
import requests
print(requests.get("https://example.com").status_code)
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    import requests
ModuleNotFoundError: No module named 'requests'
```

The package is not installed **in the Python that runs the program**. Activate your
virtual environment, then `python -m pip install requests`.

### Naming your file like a standard module

A file named `random.py` that contains:

```python norun
import random
print(random.randint(1, 6))
```

```text
Traceback (most recent call last):
  File "random.py", line 1, in <module>
    import random
  File "random.py", line 2, in <module>
    print(random.randint(1, 6))
          ^^^^^^^^^^^^^^
AttributeError: module 'random' has no attribute 'randint' (consider renaming 'random.py' since it has the same name as the standard library module named 'random' and prevents importing that standard library module)
```

Python looks first in the program's folder: `import random` imports **your** file
instead of the standard module. Rename it, for example `dice.py`.

### Importing a name that does not exist

```python norun
from math import square_root
print(square_root(16))
```

```text
Traceback (most recent call last):
  File "main.py", line 1, in <module>
    from math import square_root
ImportError: cannot import name 'square_root' from 'math' (unknown location)
```

The module exists, but not this name. The function is called `sqrt`. Check with
`dir(math)`.

## Test yourself

**Question 1:** what is the difference between `import math` and `from math import sqrt`?

> **Answer**
>
> With `import math`, you write `math.sqrt(4)`. With `from math import sqrt`, you write
> `sqrt(4)` directly, but you no longer see where the function comes from.

**Question 2:** a file `tools.py` contains `print("loaded")` then a function. What
happens when `main.py` does `import tools`?

> **Answer**
>
> `loaded` is printed: importing a module runs all its code. To avoid it, put this
> `print` under `if __name__ == "__main__":`.

**Question 3:** in which order do you type the commands to start a project that uses
`requests`?

> **Answer**
>
> ```sh
> python3 -m venv .venv
> source .venv/bin/activate
> python -m pip install requests
> python -m pip freeze > requirements.txt
> ```

## Summary

- A module is a `.py` file; a package is a folder of modules with `__init__.py`.
- `import math`, `from math import sqrt` and `import ... as ...` are the three forms of
  import.
- `if __name__ == "__main__":` holds the code that runs only when the file is run
  directly.
- The standard library covers many needs before you install anything.
- `python3 -m venv .venv` creates a virtual environment; `pip install` adds packages to
  it; `requirements.txt` lists them.

Next: [reading and writing files](fichiers.md). Compare with
[modules in Ruby](../ruby/modules.md).
