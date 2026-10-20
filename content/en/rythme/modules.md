# Modules: a program in several files

When a program grows, you split it into several files: each file is a **module**, which the other files use with `use`.

> **Goals:**
>
> - use the functions of another file with `use name`;
> - put files in folders, with `use folder/name`;
> - give a module a shorter name with `use x as y`;
> - keep a function private to its file, with a name that starts with `_`.

## Why several files

A 30-line program fits very well in a single file. At 300 lines, you get
lost: you look for a function by scrolling up and down, and you mix things
that have nothing to do with each other.

So you sort the code by **topic**: the geometry calculations in one file,
the display in another, the main program in a third. Each file is short,
and its name says what it contains.

In Rythme, every `.ry` file can be used as a **module**: another file uses
it with the `use` keyword.

> **Note:** the examples on this page need several files, so they are not
> run on the site. Their outputs were copied from real runs: create the
> files on your computer to try them.

## A first module with use

Create a folder with two files:

```text
my_folder/
├── geometry.ry
└── main.ry
```

The file `geometry.ry` contains two functions. Nothing new here: it is an
ordinary Rythme file.

```rythme norun
## The area of a rectangle.
fn rectangle_area(width, height) = width * height

## The area of a square.
fn square_area(side) = rectangle_area(side, side)
```

The file `main.ry` uses them:

```rythme norun
use geometry

print(geometry.rectangle_area(2, 3))
print(geometry.square_area(4))
```

In the terminal, from `my_folder`:

```sh
rythme run main.ry
```

```text
6
16
```

Two things to remember:

- `use geometry` loads the file `geometry.ry`. You write the name **without
  quotes and without `.ry`**. The file is looked for in the folder of the
  file that contains the `use`.
- Then you write `geometry.` in front of each name from the module:
  `geometry.square_area(4)`. When you read the code, you know right away
  where each function comes from.

> **Remember:** `use` goes at the top of the file, outside of any function.

## Each module has its own names

Since each name is preceded by its module, two files can have a function
with the same name without getting in each other's way. `geometry.area` and
`circle.area` would be two different functions, and your program can
define an `area` function of its own.

This is also why `geometry` alone is not a value: it is only the name of
the file, which you always follow with a dot and a name.

## A module in a folder with use folder/name

When you have many modules, you put them in folders. Let's add a `shapes`
folder with a file `circle.ry`:

```text
my_folder/
├── shapes/
│   └── circle.ry
├── geometry.ry
└── main.ry
```

Here is `shapes/circle.ry`:

```rythme norun
## The area of a circle, rounded to 2 decimal places.
fn area(radius) = round(pi * radius ** 2, 2)

## The perimeter of a circle, rounded to 2 decimal places.
fn perimeter(radius) = round(2 * pi * radius, 2)
```

In `main.ry`, you write the path with a slash `/`, still without `.ry`:

```rythme norun
use geometry
use shapes/circle

print(geometry.square_area(3))
print(circle.area(1))
print(circle.perimeter(1))
```

```text
9
3.14
6.28
```

The module is named after the **last part** of the path: `circle`, not
`shapes/circle`.

> **Tip:** to use a file from a neighboring folder, `..` goes up one
> folder: `use ../tools/calculations`.

## A shorter name with use x as y

`as` gives the module the name of your choice:

```rythme norun
use shapes/circle as disc

fn area(side) = side * side

print(disc.area(2))
print(area(2))
```

```text
12.57
4
```

Here, `disc.area` is the area of the circle, and `area` is the function of
the file itself. `as` is mostly useful in two cases:

- the module's name is long, and you prefer a short name;
- two files have the same name, like `shapes/circle.ry` and
  `drawings/circle.ry`: two modules cannot both be called `circle`, so you
  rename one of them (`use drawings/circle as drawing`).

## Values and types in a module

A module does not only give functions: it also gives its **values** and
its **types** (see [your own types](types.md)). Here is `light.ry`:

```rythme norun
## The three colors of a traffic light.
type Color = Red | Orange | Green

## The color that comes next.
fn next(Red) = Green
fn next(Green) = Orange
fn next(Orange) = Red
```

In `main.ry`, the colors are also written with the module's name, even in a
`match`:

```rythme norun
use light

fn advice(color) = match color
  light.Red -> "stop"
  light.Orange -> "slow down"
  light.Green -> "go"
end

print(light.next(light.Red))
print(advice(light.Green))
```

```text
Green
go
```

## Keeping a name private with _

Some functions are only used inside their own file: they are **internal
tools**. To hide them from the other files, give them a name that starts
with `_`. Here is `points.ry`:

```rythme norun
## The starting point, at the center.
origin = {x: 0, y: 0}

## The distance between two points {x, y}.
fn distance(a, b) = sqrt(_square(a.x - b.x) + _square(a.y - b.y))

fn _square(x) = x * x
```

`distance` uses `_square` without any problem: they are in the same file.
But `main.ry` only sees `origin` and `distance`:

```rythme norun
use points

print(points.origin)
print(points.distance(points.origin, {x: 3, y: 4}))
```

```text
{x: 0, y: 0}
5.0
```

A private name leaves you free to change it later, without breaking the
other files. Writing `points._square(3)` in `main.ry` is an error (see
below).

## What use does, step by step

When you run `rythme run main.ry`:

1. Rythme reads `main.ry`, finds its `use` lines, and loads each module,
   then the modules of those modules.
2. It **checks** all the files before running anything: a typo in a module
   is found right away.
3. Each module runs **only once**, before the file that uses it, even if
   several files use it.

`rythme check main.ry` does the same check without running anything.

> **Tip:** a module is for **defining** things. Avoid putting `print` calls
> at its top level: they would be printed as soon as another file uses it.

Two modules cannot use each other (`a.ry` uses `b.ry`, which uses `a.ry`):
put what both of them need in a third file.

## Common mistakes

### Forgetting the module's name

With `use geometry`, you write `square_area(4)` instead of
`geometry.square_area(4)`:

```rythme norun
use geometry

print(square_area(4))
```

```text
error[E003]: unknown name `square_area`
 --> main.ry:3:7
  |
3 | print(square_area(4))
  |       ^^^^^^^^^^^ not defined here
  |
  = help: `square_area` is defined in `geometry.ry`: write `geometry.square_area`
  = more about this error: `rythme explain E003`
```

Fix: `print(geometry.square_area(4))`. If you forgot the `use` itself, the
message also suggests adding it.

### Getting the file name wrong

```rythme norun
use geomerty

print(geomerty.square_area(4))
```

```text
error[E017]: file `geomerty.ry` not found
 --> main.ry:1:5
  |
1 | use geomerty
  |     ^^^^^^^^
  |
  = note: looked for: geomerty.ry
  = help: did you mean `use geometry`?
  = help: the path is relative to the folder of the file that contains `use`
  = more about this error: `rythme explain E017`
```

Fix: `use geometry`. Also check that the file really is in the same folder
as `main.ry` (or in the folder given by the path).

### Writing .ry in the use

```rythme norun
use geometry.ry

print(geometry.square_area(4))
```

```text
error[E001]: write the name of the file without `.ry`
 --> main.ry:1:13
  |
1 | use geometry.ry
  |             ^
  |
  = help: example: `use geometry`
  = more about this error: `rythme explain E001`
```

Fix: `use geometry`, without `.ry`.

### Using a private name

```rythme norun
use points

print(points._square(3))
```

```text
error[E003]: `_square` is private to the file `points.ry`
 --> main.ry:3:7
  |
3 | print(points._square(3))
  |       ^^^^^^^^^^^^^^ private
  |
  = note: a name starting with `_` can only be used in its own file
  = help: to use it from other files, rename it without `_` in `points.ry`
  = more about this error: `rythme explain E003`
```

Fix: if other files really need it, rename the function `square` (without
`_`) in `points.ry`.

## Test yourself

1. The file `tools/text.ry` contains a function `title`. What do you write
   in `main.ry` to call it?

> **Answer**
>
> `use tools/text` at the top of the file, then `text.title(...)`: the
> module is named after the last part of the path.

2. Why write `use calculations as c`?

> **Answer**
>
> To use the module under a shorter name: `c.average(...)` instead of
> `calculations.average(...)`. It is also the solution when two files have
> the same name.

3. In `bank.ry`, can the function `_round_off` be used from `main.ry`?

> **Answer**
>
> No: its name starts with `_`, so it is private to `bank.ry`. The other
> functions of `bank.ry` can use it, but `bank._round_off(...)` in
> `main.ry` is error E003.

## Summary

- `use geometry` loads `geometry.ry` (without quotes, without `.ry`); its
  names are then written `geometry.name`.
- `use shapes/circle` loads `shapes/circle.ry`, used under the name
  `circle`.
- `use shapes/circle as disc` picks another name.
- A module gives its functions, its values and its types; a name that
  starts with `_` stays private to its file.
- Rythme checks all the files before running the program.

Next: [reading and writing files](fichiers.md). Compare with modules in
[Ruby](../ruby/modules.md) and in [Rust](../rust/modules.md).
