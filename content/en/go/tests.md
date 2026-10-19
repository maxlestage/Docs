# Testing your code

This page shows how to write automated tests with the `testing` package and run them with `go test`, from the simplest tests to table-driven tests.

> **Goals:**
>
> - you will know how to write a `_test.go` file and a `TestX(t *testing.T)` function;
> - you will know how to report a failure with `t.Errorf` and `t.Fatalf`;
> - you will know how to write table-driven tests, with `t.Run` subtests;
> - you will know how to run tests with `go test` and read their result;
> - you will know what benchmarks and testable examples are.

## Why test?

A [test](../debuter/glossaire.md#test) is a small program that checks that another
program does what it should. You write it once, then you run it again after every
change, in one second.

It's like a pilot's checklist before takeoff: you always check the same points, in the
same order, without relying on your memory.

Without a tool, you would check by hand, with `fmt.Println`:

```go
package main

import "fmt"

func Sum(numbers []int) int {
	total := 0
	for _, n := range numbers {
		total += n
	}
	return total
}

func main() {
	if Sum([]int{1, 2, 3}) != 6 {
		fmt.Println("FAILED: the sum of 1, 2, 3 should be 6")
	} else {
		fmt.Println("OK")
	}
}
```

```sortie
OK
```

It works, but you have to read the output, and the checks get mixed with the program.
Go provides a tool made for this: the `testing` package and the `go test` command.

## The project to test

Tests live in a **module** (see [modules](modules.md)). Let's create a `calc`
folder:

```sh
mkdir calc
cd calc
go mod init calc
```

Then the `calc.go` file, with two functions:

```go norun
// Package calc groups small calculation functions.
package calc

import "errors"

// Sum returns the sum of the numbers.
func Sum(numbers []int) int {
	total := 0
	for _, n := range numbers {
		total += n
	}
	return total
}

// Max returns the largest number in the slice.
func Max(numbers []int) (int, error) {
	if len(numbers) == 0 {
		return 0, errors.New("empty slice")
	}
	largest := numbers[0]
	for _, n := range numbers[1:] {
		if n > largest {
			largest = n
		}
	}
	return largest, nil
}
```

This package has no `main`: it doesn't run on its own. The tests are what will call
it. That's why the blocks on this page are marked `norun`: together they form a
project with several files.

## A first test

Go's rules are simple:

- tests go in a file whose name ends with **`_test.go`**, next to the code: here
  `calc_test.go`;
- this file is in the **same package** (`package calc`);
- each test is a function whose name starts with **`Test`** followed by a capital
  letter, with a single parameter `t *testing.T`.

```go norun
package calc

import "testing"

func TestSum(t *testing.T) {
	result := Sum([]int{1, 2, 3})
	if result != 6 {
		t.Errorf("Sum([1 2 3]) = %d, want 6", result)
	}
}
```

If the result is wrong, `t.Errorf` reports the failure with a message, like
`fmt.Printf`. Otherwise, the test says nothing: it passes.

Run the tests from the module folder:

```sh
go test
```

```text
PASS
ok  	calc	0.003s
```

`PASS`: all the tests passed. The time at the end changes from one run to the next.

> **Remember:** a good failure message says what was called, what was obtained and
> what was expected. It will save you time the day it shows up.

## Errorf or Fatalf?

`t` offers two families to report a problem:

| Method | Effect |
|--------|--------|
| `t.Errorf(…)` / `t.Error(…)` | records the failure and **continues** the test |
| `t.Fatalf(…)` / `t.Fatal(…)` | records the failure and **stops** this test right away |

Use `t.Fatalf` when going on makes no sense, for example if a function returns an
unexpected error: no point checking a value that doesn't exist.

```go norun
func TestMaxEmpty(t *testing.T) {
	_, err := Max([]int{})
	if err == nil {
		t.Error("an empty slice should return an error")
	}
}
```

This test checks that the **error** is really returned: testing error cases is as
important as testing normal cases (see [errors](erreurs.md)).

## Table-driven tests

To try several inputs, you don't copy the test ten times. You write a **table of
cases**: a slice of anonymous structs, each with a name, an input and the expected
result. Then a loop checks them one by one. This is the most common way to write
tests in Go.

`t.Run(name, function)` creates a **subtest** for each case: if one fails, Go tells
you which one.

```go norun
func TestMax(t *testing.T) {
	cases := []struct {
		name  string
		input []int
		want  int
	}{
		{"positives", []int{3, 8, 1}, 8},
		{"negatives", []int{-5, -2, -9}, -2},
		{"single one", []int{4}, 4},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			got, err := Max(c.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if got != c.want {
				t.Errorf("Max(%v) = %d, want %d", c.input, got, c.want)
			}
		})
	}
}
```

Adding a case means adding a line to the table.

## Reading the result of go test -v

The `-v` option (*verbose*) shows each test and each subtest:

```sh
go test -v
```

```text
=== RUN   TestSum
--- PASS: TestSum (0.00s)
=== RUN   TestMax
=== RUN   TestMax/positives
=== RUN   TestMax/negatives
=== RUN   TestMax/single_one
--- PASS: TestMax (0.00s)
    --- PASS: TestMax/positives (0.00s)
    --- PASS: TestMax/negatives (0.00s)
    --- PASS: TestMax/single_one (0.00s)
=== RUN   TestMaxEmpty
--- PASS: TestMaxEmpty (0.00s)
PASS
ok  	calc	0.006s
```

Spaces in subtest names become `_`: `single one` → `single_one`.

### When a test fails

Let's add a bug: in `Max`, we start from `0` instead of the first number
(`largest := 0`). The result becomes wrong for negative numbers, and for a single
number too, since the loop skips the first one:

```text
--- FAIL: TestMax (0.00s)
    --- FAIL: TestMax/negatives (0.00s)
        calc_test.go:29: Max([-5 -2 -9]) = 0, want -2
    --- FAIL: TestMax/single_one (0.00s)
        calc_test.go:29: Max([4]) = 0, want 4
FAIL
exit status 1
FAIL	calc	0.002s
```

The report gives you everything you need: the subtest (`negatives`), the file and the
line (`calc_test.go:29`), then your message. The `positives` case passed: without `-v`,
Go only shows failures.

### Choosing which tests to run

| Command | Effect |
|---------|--------|
| `go test` | runs the tests of the current folder |
| `go test ./...` | runs the tests of every package in the module |
| `go test -v` | shows each test |
| `go test -run Max` | runs only the tests whose name contains `Max` |
| `go test -cover` | shows how much of the code the tests run |

```sh
go test -cover
```

```text
PASS
coverage: 100.0% of statements
ok  	calc	0.003s
```

## Testable examples

A function whose name starts with `Example` is both **documentation** and a test. The
`// Output:` comment gives the expected output: `go test` runs the example and
compares. They often go in `example_test.go`.

```go norun
package calc

import "fmt"

func ExampleSum() {
	fmt.Println(Sum([]int{10, 20, 12}))
	// Output: 42
}

func ExampleMax() {
	largest, err := Max([]int{3, 8, 1})
	fmt.Println(largest, err)
	// Output: 8 <nil>
}
```

```text
=== RUN   ExampleSum
--- PASS: ExampleSum (0.00s)
=== RUN   ExampleMax
--- PASS: ExampleMax (0.00s)
PASS
ok  	calc	0.003s
```

These examples show up in the package documentation, and they can't become wrong
without the tests reporting it. That's also the idea behind the `sortie` blocks on
this site.

## Benchmarks

A **benchmark** measures the speed of a function. Its name starts with `Benchmark` and
it receives `b *testing.B`. The `for b.Loop()` loop repeats the call as many times as
needed to get a reliable measurement.

```go norun
func BenchmarkSum(b *testing.B) {
	numbers := make([]int, 1000)
	for i := range numbers {
		numbers[i] = i
	}
	for b.Loop() {
		Sum(numbers)
	}
}
```

```sh
go test -bench .
```

```text
goos: linux
goarch: amd64
pkg: calc
cpu: Intel(R) Xeon(R) Processor @ 2.10GHz
BenchmarkSum-4   	 1617744	       725.0 ns/op
PASS
ok  	calc	1.179s
```

The function was called 1,617,744 times, at about 725 nanoseconds per call. These
numbers depend on your machine: only compare them between two versions of your code,
on the same computer.

> **Note:** `b.Loop()` exists since Go 1.24. In older code, you will see
> `for i := 0; i < b.N; i++`.

## Common mistakes

### A badly named file

```go norun
// file calc_tests.go (with an "s")
package calc

import "testing"

func TestSum(t *testing.T) {
	if Sum([]int{1, 2, 3}) != 6 {
		t.Error("wrong sum")
	}
}
```

```text
?   	calc	[no test files]
```

Go only looks for tests in files that end **exactly** with `_test.go`. Rename the file
to `calc_test.go`.

### A test name in lowercase

```go norun
func testSum(t *testing.T) {
	if Sum([]int{1, 2, 3}) != 6 {
		t.Error("wrong sum")
	}
}
```

```text
testing: warning: no tests to run
PASS
ok  	calc	0.002s
```

Trap: `PASS` shows up, but no test ran. The name must start with `Test` with a capital
letter: `TestSum`.

### Forgetting the star in *testing.T

```go norun
func TestSum(t testing.T) {
	if Sum([]int{1, 2, 3}) != 6 {
		t.Error("wrong sum")
	}
}
```

```text
# calc
…/calc/calc_test.go:5:1: wrong signature for TestSum, must be: func TestSum(t *testing.T)
FAIL	calc [setup failed]
```

The message gives the right form: the parameter is a **pointer**, `t *testing.T`.

### A wrong example output

```go norun
func ExampleSum() {
	fmt.Println(Sum([]int{10, 20, 12}))
	// Output: 43
}
```

```text
--- FAIL: ExampleSum (0.00s)
got:
42
want:
43
FAIL
exit status 1
FAIL	calc	0.002s
```

`got` is what the example printed, `want` is what the comment announces. Fix whichever
one is wrong (here, the comment).

## Test yourself

**Question 1:** what conditions must a function meet for `go test` to run it as a
test?

> **Answer**
>
> It must be in a file that ends with `_test.go`, be called `TestSomething` (with a
> capital letter after `Test`) and have a single parameter `t *testing.T`.

**Question 2:** what is the difference between `t.Errorf` and `t.Fatalf`?

> **Answer**
>
> `t.Errorf` reports the failure and lets the test continue. `t.Fatalf` reports the
> failure and stops this test immediately.

**Question 3:** write a table-driven test `TestSum` with three cases: an empty slice
(0), `[5]` (5) and `[1, 2, 3]` (6).

> **Answer**
>
> ```go norun
> func TestSum(t *testing.T) {
> 	cases := []struct {
> 		name  string
> 		input []int
> 		want  int
> 	}{
> 		{"empty", []int{}, 0},
> 		{"one number", []int{5}, 5},
> 		{"three numbers", []int{1, 2, 3}, 6},
> 	}
> 	for _, c := range cases {
> 		t.Run(c.name, func(t *testing.T) {
> 			if got := Sum(c.input); got != c.want {
> 				t.Errorf("Sum(%v) = %d, want %d", c.input, got, c.want)
> 			}
> 		})
> 	}
> }
> ```

## Summary

- Tests go in `_test.go` files, in the same package as the code.
- A test is written `func TestName(t *testing.T)`; `t.Errorf` continues, `t.Fatalf`
  stops.
- Table-driven tests with `t.Run` check many cases in few lines.
- `go test` runs the tests, `-v` details them, `-run` filters them, `-cover` measures
  coverage.
- `ExampleX` documents and tests at the same time; `BenchmarkX` measures speed.

Next: [debugging a program](debogage.md). Compare with
[tests in Ruby](../ruby/tests.md).
