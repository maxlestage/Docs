# Reading and writing: keyboard and files

This page shows how a Rythme program receives information (the keyboard, the command line) and how it keeps its results in files: text, lines, CSV and JSON.

> **Goals:**
>
> - you will know how to read a line typed on the keyboard, and turn it into a number;
> - you will know how to read the words given on the command line;
> - you will know how to write, extend and read a text file, handling failures;
> - you will know how to process a file line by line, and read a small CSV file;
> - you will know how to keep data in JSON.

## Reading from the keyboard with input

`input(question)` shows the question, waits for the user to type a
line and press Enter, then gives back what they typed, as text:

```rythme norun
name = input("What is your name? ")
print("Hello, {name}!")
```

In the terminal, this gives:

```text
What is your name? Ada
Hello, Ada!
```

What `input` gives is always **text**. For a number, read it
with `parse_int`, which gives a `Result` (see
[working with text](texte.md)):

```rythme norun
entry = input("Your age? ")
match parse_int(entry)
  Ok(age) -> print("In ten years, you will be {age + 10}.")
  Err(_) -> print("That is not a number.")
end
```

```text
Your age? 30
In ten years, you will be 40.
```

> **Note:** the examples with `input` are not run on this site: they
> wait for someone at the keyboard. Try them at home with `rythme run`.

## The words of the command line with args

The words written after the file name, in the terminal, arrive in the
list `args()`:

```sh
rythme run greet.ry Ada Alan
```

Here is `greet.ry`. It also handles the case where no word is given:

```rythme
fn greet([]) = print("Give at least one name after the file name.")
fn greet(names) = names.each do |name|
  print("Hello, {name}!")
end

greet(args())
```

Run without any word, it prints:

```sortie
Give at least one name after the file name.
```

And with `Ada Alan`:

```text
Hello, Ada!
Hello, Alan!
```

## Writing a file with write_file

A **file** is a text stored on the computer's disk: it stays
there when the program stops. The program finds it by its
**path**: its name and, if needed, the folders it is in
(`notes.txt`, `data/notes.txt`).

`write_file(path, content)` creates the file, or replaces all its content.
Writing can fail (a folder that does not exist, no permission to write...),
so the answer is a `Result`: `Ok(())` if all goes well.

```rythme
result = write_file("shopping.txt", "bread\nmilk\neggs")
print(result)
```

```sortie
Ok(())
```

The file `shopping.txt` is created in the folder where you run the
program. `\n` starts a new line: the file has three lines.

> **Warning:** `write_file` replaces **all** the content, without asking.
> It's like rewriting a page as a clean copy: the old version is lost.

## Reading a file with read_file

`read_file(path)` gives the whole content of the file, in a `Result`:
`Ok(content)`, or `Err(message)` if the file does not exist.

```rythme
write_file("shopping.txt", "bread\nmilk\neggs")

print(read_file("shopping.txt"))
print(read_file("forgotten.txt"))
```

```sortie
Ok("bread\nmilk\neggs")
Err("file not found: forgotten.txt")
```

You handle both cases with `match`:

```rythme
write_file("shopping.txt", "bread\nmilk\neggs")

match read_file("shopping.txt")
  Ok(content) -> print(content)
  Err(e) -> print("Problem: {e}")
end
```

```sortie
bread
milk
eggs
```

## Passing the error on with ?

In a function, `?` saves you a `match` at each step: if reading
fails, the function stops and returns the error (see
[handling errors](erreurs.md#the-operator-to-pass-the-error-on)).

```rythme
fn line_count(path)
  content = read_file(path)?
  Ok(len(lines(content)))
end

write_file("shopping.txt", "bread\nmilk\neggs")
print(line_count("shopping.txt"))
print(line_count("forgotten.txt"))
```

```sortie
Ok(3)
Err("file not found: forgotten.txt")
```

> **Warning:** `?` only works inside a function. At the top level of the
> program, use `match` or `unwrap_or`.

## Adding at the end with append_file

`write_file` replaces all the content. `append_file` adds at the end, and creates
the file if it does not exist yet. It's handy for a log:

```rythme
append_file("log.txt", "monday: 5 km\n")
append_file("log.txt", "tuesday: rest\n")
append_file("log.txt", "wednesday: 8 km\n")

print(read_file("log.txt") |> unwrap_or(""))
```

```sortie
monday: 5 km
tuesday: rest
wednesday: 8 km
```

Don't forget the `\n` at the end of each added line.

## Checking whether a file exists

`file_exists(path)` simply answers `true` or `false`:

```rythme
write_file("notes.txt", "17")

print(file_exists("notes.txt"), file_exists("absent.txt"))
message = if file_exists("settings.txt") then "settings found" else "default settings"
print(message)
```

```sortie
true false
default settings
```

## Processing a file line by line

Often, you read the file, split it into lines with `lines`, then
work on the list with `map`, `filter` or `each`:

```rythme
write_file("shopping.txt", "bread\nmilk\neggs")

content = read_file("shopping.txt") |> unwrap_or("")
lines(content) |> each_with_index(fn(item, i) = print("{i + 1}. {item}"))
```

```sortie
1. bread
2. milk
3. eggs
```

`each_with_index` also gives the position of each element, starting from 0.

## A small CSV file

A **CSV** file is a table written as text: one line per row, with
commas between the columns. You read it with `lines`, then `split(line, ",")`
for each line:

```rythme
write_file("grades.csv", "Ada,17\nAlan,12\nGrace,19")

fn read_student(line)
  [name, grade] = split(line, ",")
  {name: name, grade: parse_int(grade) |> unwrap_or(0)}
end

students = read_file("grades.csv")
  |> unwrap_or("")
  |> lines
  |> map(read_student)

students.each do |s|
  print("{s.name}: {s.grade}")
end
print("Average:", sum(map(students, fn(s) = s.grade)) / len(students))
```

```sortie
Ada: 17
Alan: 12
Grace: 19
Average: 16.0
```

`[name, grade] = split(...)` gives a name to each of the two pieces (see
[taking a value apart](collections.md#taking-a-value-apart)).

## The JSON format

**JSON** is a very common text format for storing data.
`to_json` turns a value into JSON text, and `parse_json` reads that
text back. Records become JSON objects, and the other way around.

```rythme
ada = {name: "Ada", grades: [17, 19]}
print(to_json(ada))
```

```sortie
{"name": "Ada", "grades": [17, 19]}
```

To keep data from one run to the next, you write the JSON to a
file, then read it back. `parse_json` gives a `Result`, because the text may
not be valid JSON:

```rythme
fn save(path, value) = write_file(path, to_json(value))

fn load(path)
  content = read_file(path)?
  parse_json(content)
end

save("ada.json", {name: "Ada", grades: [17, 19]})

match load("ada.json")
  Ok(student) -> print("{student.name} has {len(student.grades)} grades.")
  Err(e) -> print("Problem: {e}")
end
print(parse_json("not json") |> is_err)
```

```sortie
Ada has 2 grades.
true
```

> **Tip:** `to_json(value, 2)` writes one element per line, indented by
> 2 spaces: the file is easier for a human to read.

## Common mistakes

### Computing with what the user types

```rythme norun
age = input("Your age? ")
print("In ten years: {age + 10}")
```

```text
error[E005]: cannot use `+` with Text and Int
 --> main.ry:2:27
  |
2 | print("In ten years: {age + 10}")
  |                           ^ Text + Int 10
  |
  = help: to build a text with a value inside, use braces: "total: {x}"
      or convert the value first: text(x)
  = more about this error: `rythme explain E005`
```

`input` always gives text, even if you type `30`. Read it first
with `parse_int`, as in [reading from the keyboard](#reading-from-the-keyboard-with-input).
The error is found before the question is even asked.

### Forgetting that read_file gives a Result

```rythme norun
write_file("shopping.txt", "bread\nmilk")
content = read_file("shopping.txt")
print(lines(content))
```

```text
error[E005]: `lines` expects a text as argument 1, but got a value of type Result
 --> main.ry:3:13
  |
3 | print(lines(content))
  |             ^^^^^^^ wrong kind of value
  |
  = help: usage: `lines(text)`
  = more about this error: `rythme explain E005`
```

`content` is `Ok("bread\nmilk")`, not the text itself: reading could
have failed. Open the `Result` with `match`, `unwrap_or("")`, or `?` inside
a function.

### Using the wrong separator

```rythme norun
line = "Ada;17"
[name, grade] = split(line, ",")
print(name, grade)
```

```text
error[E006]: the value List ["Ada;17"] does not match this pattern
 --> main.ry:2:1
  |
2 | [name, grade] = split(line, ",")
  | ^^^^^^^^^^^^^
  |
  = help: use `match` to handle the different possible shapes of a value
  = more about this error: `rythme explain E006`
```

The line is separated by `;`, not `,`: `split` gives a single
piece, and `[name, grade]` expects two. Look at your file and
write `split(line, ";")`.

## Test yourself

**Question 1:** what is the difference between `write_file` and
`append_file`?

> **Answer**
>
> `write_file` replaces all the content of the file; `append_file` adds
> at the end, without erasing what is already there.

**Question 2:** what does `read_file("absent.txt")` give if the file does not
exist?

> **Answer**
>
> `Err("file not found: absent.txt")`. The program does not stop:
> it's up to you to handle this case.

**Question 3:** the program `grades.ry` is run with
`rythme run grades.ry maths 2024`. What is `args()`?

> **Answer**
>
> `["maths", "2024"]`: a list of texts, without the file name.

## Summary

- `input("question ")` reads a line from the keyboard, always as
  text; `args()` gives the words of the command line.
- `write_file` replaces the content of a file, `append_file` adds at the
  end, `file_exists` tells whether it exists.
- `read_file` gives a `Result`: `Ok(content)` or `Err(message)`. Inside a
  function, `?` passes the error on.
- To process a file, you split it with `lines`, then `split` for the
  columns of a CSV.
- `to_json` and `parse_json` write and read back data in JSON.

Next: [JSON data](formats.md)
