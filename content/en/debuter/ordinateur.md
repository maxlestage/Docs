# How a computer works

A computer stores everything as numbers and transforms them very fast: this page explains the processor, memory, files, bytes and the role of the operating system, so you better understand what your programs do.

> **Goals:**
>
> - know what the processor, RAM and the disk are for;
> - read a file path and understand the folder tree;
> - understand bits and bytes, and count in binary;
> - see how text is stored as numbers (ASCII, Unicode);
> - tell apart the operating system, a program and a process.

## The main parts of a computer

Picture a kitchen:

- the **processor** is the cook: it does the work;
- **RAM** is the countertop: fast, within reach, but cleared at the end of
  the day;
- the **disk** is the cupboard: slower, but everything stays put, even
  when the lights are off.

These three parts work together every time you run a program.

## The processor

The [processor](glossaire.md#processor) (*CPU*) runs very simple
instructions: add two numbers, compare two numbers, copy a number from one
slot to another, jump to another instruction.

It only does tiny ones, but **billions per second**. A program that
displays a whole page is millions of these small instructions put end to
end.

> **Remember:** the processor understands neither Ruby, nor Rust, nor
> Rythme. It only understands its own language, made of numbers: **machine
> language**.

## RAM and the disk

[RAM](glossaire.md#ram) (random-access memory) holds the data of
programs **while they run**: your variables, your lists, the text you
type. It is very fast, but it is wiped when you turn the computer off.

The **disk** (hard drive or SSD) keeps data **for good**: your photos,
your documents, your programs. It is slower, but nothing disappears when
you shut down.

| Part           | Speed         | Keeps data after shutdown?           | Common size     |
|----------------|---------------|--------------------------------------|-----------------|
| RAM            | very fast     | no                                   | 8 to 32 GB      |
| Disk           | slower        | yes                                  | 256 GB to 2 TB  |

That's why you need to **save** a file: until you do, your work only lives
in RAM.

## Files and folders

On the disk, data is stored in **files**: a file is a sequence of bytes
with a name, like `hello.rb` or `photo.jpg`.

Files are stored in **folders**, which can contain other folders. This
forms a tree:

```text
/home/alice
├── Documents
│   └── letter.txt
└── code
    ├── hello.rb
    └── calc.ry
```

### Paths

A [path](glossaire.md#path) is the address of a file: the list of
folders to go through, separated by `/` (or `\` on Windows).

- An **absolute** path starts from the root: `/home/alice/code/hello.rb`,
  or `C:\Users\Alice\code\hello.rb` on Windows.
- A **relative** path starts from the folder you're in: if you're in
  `/home/alice`, the same file is written `code/hello.rb`.
- `..` means "the parent folder": from `code`, the file `letter.txt` is
  written `../Documents/letter.txt`.

The [terminal](terminal.md) makes you use these paths every day.

## Bits and bytes

Deep down, the computer only knows two states: current or no current. We
write them `0` and `1`. One of these digits is called a
[bit](glossaire.md#bit).

A single bit doesn't say much. So bits are grouped by eight: eight bits
make a [byte](glossaire.md#byte). A byte can take
2 × 2 × 2 × 2 × 2 × 2 × 2 × 2 = 256 values, from 0 to 255.

| Unit    | Size                    | Rough idea                |
|---------|-------------------------|---------------------------|
| 1 byte  | 8 bits                  | one simple letter         |
| 1 KB    | about 1,000 bytes       | a small text file         |
| 1 MB    | about 1 million         | a photo                   |
| 1 GB    | about 1 billion         | a movie                   |

## Counting in binary

We count with ten digits (0 to 9): that's base 10. The computer counts with
two digits (0 and 1): that's base 2, [binary](glossaire.md#binary).

In base 10, each position is worth ten times more than the previous one:
1, 10, 100… In binary, each position is worth **twice** as much: 1, 2, 4,
8, 16, 32, 64, 128.

```text
  64  32  16   8   4   2   1
   1   0   0   0   0   0   1    →  64 + 1 = 65
```

All three languages can write a number in binary. In Ruby:

```ruby
puts 65.to_s(2)
puts 0b1000001
```

```sortie
1000001
65
```

In Rust, `{:b}` displays in binary, and `{:08b}` pads with zeros to get
eight digits, which is one byte:

```rust
println!("{:b}", 65);
println!("{:08b}", 5);
println!("A byte goes up to {}", u8::MAX);
```

```sortie
1000001
00000101
A byte goes up to 255
```

Rythme has no ready-made function for this, but you can write one with a
[recursive function](glossaire.md#recursion): divide by two, and keep the
remainder.

```rythme
fn binary(0) = "0"
fn binary(1) = "1"
fn binary(n) = binary(n // 2) + text(n % 2)

print(binary(65))
print(binary(5))
```

```sortie
1000001
101
```

## Text is numbers too

The computer only stores numbers. To keep text, every **character was
given a number**. It's a code shared by everybody.

### ASCII: the first 128 characters

The [ASCII](glossaire.md#ascii) code, created in the 1960s, numbers the
English letters, the digits and punctuation. `A` is 65, `B` is 66, `a` is
97, the space is 32.

In Ruby, `ord` gives the number of a character, and `chr` does the
opposite:

```ruby
puts "A".ord
puts 66.chr
puts "Ada".bytes.inspect
```

```sortie
65
B
[65, 100, 97]
```

In Rust, a character (`char`) is converted to a number with `as u32`:

```rust
println!("{}", 'A' as u32);
println!("{}", char::from_u32(66).unwrap());
```

```sortie
65
B
```

In Rythme, `char_code` and `char` do the same job:

```rythme
print(char_code("A"), char(66))
print(map(chars("Ada"), char_code))
```

```sortie
65 B
[65, 100, 97]
```

### Unicode: every character in the world

ASCII has no `é`, no `ç`, no ideograms, no emojis. So [Unicode](glossaire.md#unicode)
numbered everything: more than 150,000 characters, in every writing
system. `é` is 233, `€` is 8364.

To store these numbers in bytes, we almost always use **UTF-8**: ASCII
characters take 1 byte, `é` takes 2, `€` takes 3.

```ruby
puts "é".ord
puts "é".bytesize
puts "Aé".bytes.inspect
```

```sortie
233
2
[65, 195, 169]
```

In Rust, `.len()` counts **bytes**, not characters:

```rust
println!("{}", "é".len());
println!("{}", "é".chars().count());
```

```sortie
2
1
```

> **Note:** that's why you should save your files in UTF-8. Your editor
> does it by default.

## The operating system

The [operating system](glossaire.md#operating-system) (Windows,
macOS, Linux, Android…) is the program that starts first. It's the
conductor:

- it shares the processor and memory between programs;
- it stores files on the disk;
- it handles the screen, the keyboard, the network;
- it starts and stops programs.

Your programs never talk to the disk directly: they ask the operating
system, for example "open the file `notes.txt`".

## Programs and processes

A **program** is a file on the disk: a recipe stored in a book. A
[process](glossaire.md#process) is that program **running**: the recipe
being cooked.

When you type `ruby hello.rb`, the operating system creates a process: it
copies the program into RAM, gives it a number, and the processor starts
running its instructions. When the program ends, the process disappears
and its memory is freed.

You can start the same program twice: that makes two processes. The Task
Manager (Windows), Activity Monitor (macOS) or the `ps` command (Linux and
macOS) show them.

## Why programs need translating

The processor only understands machine language. Your code, on the other
hand, is a text written for humans. So it must be **translated**. There
are two ways to do it, explained in
[Interpreted or compiled](programmer.md#interpreted-or-compiled):

- a [compiler](glossaire.md#compiler) translates the whole program once,
  ahead of time, into a file of machine language. That's the case for Rust;
- an [interpreter](glossaire.md#interpreter) reads the code and runs it as
  it goes. That's the case for Ruby and Rythme.

It's like a book translated in advance (compiled), or an interpreter who
translates while the person is speaking (interpreted).

## Common mistakes

### Adding a text and a number

To you, `"A"` and `65` look alike. To the program, they are two different
[types](glossaire.md#type):

```ruby norun
letter = "A"
puts letter + 1
```

```text
main.rb:2:in `+': no implicit conversion of Integer into String (TypeError)

puts letter + 1
              ^
	from main.rb:2:in `<main>'
```

Convert first: `letter.ord + 1` gives the number `66`, and
`(letter.ord + 1).chr` gives the letter `"B"`.

### Putting a number in a char in Rust

```rust norun
fn main() {
    let letter: char = 65;
    println!("{letter}");
}
```

```text
error[E0308]: mismatched types
 --> main.rs:2:24
  |
2 |     let letter: char = 65;
  |                 ----   ^^ expected `char`, found `u8`
  |                 |
  |                 expected due to this
```

Rust never turns a number into a character on its own. Write the letter
`'A'`, or convert: `let letter = 65u8 as char;`.

### Giving several characters to char_code

```rythme norun
print(char_code("AB"))
```

```text
error[E023]: `char_code` needs a single character, but "AB" has 2
 --> main.ry:1:7
  |
1 | print(char_code("AB"))
  |       ^^^^^^^^^^^^^^^
```

A number matches **one** character. For a whole word, split it first:
`map(chars("AB"), char_code)` gives `[65, 66]`.

### Being in the wrong folder

```ruby norun
text = File.read("notes.txt")
```

```text
main.rb:1:in `read': No such file or directory @ rb_sysopen - notes.txt (Errno::ENOENT)
	from main.rb:1:in `<main>'
```

A relative path starts from the folder where you **run** the program.
Check with `pwd` and `ls` in the terminal, or give the right path.

## Test yourself

**1.** You type a long text without saving it, and the battery dies. Why
is the text lost?

> **Answer**
>
> It was only in RAM, which is wiped at shutdown. Only what is saved on
> the disk stays.

**2.** What is the binary number `101` worth?

> **Answer**
>
> 5: read the positions from right to left, 1 + 0 × 2 + 1 × 4 = 5.

**3.** What is the difference between a program and a process?

> **Answer**
>
> The program is a file on the disk. The process is that program running,
> in RAM, with its own number.

## Summary

- The processor computes, RAM holds data during the work, the disk keeps it for good.
- A path gives the address of a file: absolute from the root, relative from the current folder.
- Everything is stored in bits (0 or 1), grouped by eight into bytes.
- A text is a sequence of numbers: ASCII for English, Unicode (in UTF-8) for everything else.
- The operating system shares the machine; a process is a running program.
- Code must be translated into machine language, by a compiler or an interpreter.

Next: [thinking like a programmer](algorithmes.md).
