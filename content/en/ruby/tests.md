# Testing your code

This page shows how to write automated tests with Minitest, the tool that comes with Ruby, to check that your code really does what you expect.

> **Goals:**
>
> - understand what an automated test is for;
> - write a test class with Minitest;
> - use `assert_equal`, `assert`, `assert_nil` and `assert_raises`;
> - run the tests and read the result;
> - write the test before the code, in three steps.

## Why test?

A **test** is a small program that checks another program (see
[the glossary](../debuter/glossaire.md#test)). It calls your code with known values
and compares the result with the one you expect.

Without tests, you check by hand, with `puts`, and you start again after every
change. With tests:

- a single command checks everything, in one second;
- if a change breaks something, you know right away;
- the tests show how to use your code: it is documentation that works.

> **Remember:** a test answers a simple question: "with this input, do I
> really get this output?".

## Your first test with Minitest

**Minitest** comes with Ruby: nothing to install. A test file always follows
the same pattern:

1. `require "minitest/autorun"` loads Minitest and runs the tests at the end of the file;
2. a test class inherits from `Minitest::Test`;
3. each method whose name starts with `test_` is a test.

```ruby
# test_double.rb
require "minitest/autorun"

def double(x)
  x * 2
end

class TestDouble < Minitest::Test
  def test_double_of_4
    assert_equal 8, double(4)
  end

  def test_double_of_zero
    assert_equal 0, double(0)
  end
end
```

`assert_equal 8, double(4)` reads: "I expect `8`; check that `double(4)` really
equals `8`".

## Running the tests

A test file runs like any other program:

```sh
ruby test_double.rb
```

Ruby prints a summary:

```text
Run options: --seed 31187

# Running:

..

Finished in 0.000812s, 2463.0542 runs/s, 2463.0542 assertions/s.

2 runs, 2 assertions, 0 failures, 0 errors, 0 skips
```

- Each `.` is a passing test.
- The last line counts the tests (`runs`), the checks (`assertions`), the
  failures (`failures`) and the errors (`errors`).
- The number after `--seed` and the time change every time: Minitest runs the tests
  in a random order, to make sure they do not depend on each other.

> **Tip:** `ruby test_double.rb --verbose` prints the name of each test, and
> `ruby test_double.rb -n test_double_of_4` runs only that test.

## Reading a failure

When a check fails, Minitest prints an `F` and explains the problem. Here is
a `double` that gets it wrong (it adds 2 instead of multiplying):

```ruby norun
# test_double.rb
require "minitest/autorun"

def double(x)
  x + 2
end

class TestDouble < Minitest::Test
  def test_double_of_4
    assert_equal 8, double(4)
  end
end
```

```text
F

  1) Failure:
TestDouble#test_double_of_4 [test_double.rb:10]:
Expected: 8
  Actual: 6

1 runs, 1 assertions, 1 failures, 0 errors, 0 skips
```

The message gives you everything you need: the test (`test_double_of_4`), the line (`10`),
the expected value (`Expected: 8`) and the value you got (`Actual: 6`).

- `F` (*failure*): a check gave a wrong result.
- `E` (*error*): the code crashed during the test (an unknown method, for
  example).

## Assertions

An **assertion** is a check. Minitest offers several; here are the most
useful ones.

| Assertion | Checks that… |
| --- | --- |
| `assert_equal expected, actual` | the two values are equal |
| `assert value` | the value is true |
| `refute value` | the value is false |
| `assert_nil value` | the value is `nil` |
| `assert_includes array, x` | the array contains `x` |
| `assert_in_delta expected, actual, 0.01` | two decimal numbers are very close |
| `assert_raises(Error) { … }` | the block really raises this error |

```ruby
require "minitest/autorun"

class TestAssertions < Minitest::Test
  def test_assert_equal
    assert_equal "HELLO", "hello".upcase
    assert_equal [1, 2, 3], [3, 1, 2].sort
  end

  def test_assert_and_refute
    assert [1, 2, 3].include?(2)
    refute [].any?
  end

  def test_assert_nil
    assert_nil [].first
    assert_nil({ a: 1 }[:b])
  end

  def test_assert_includes
    assert_includes ["apple", "pear"], "pear"
  end

  def test_assert_in_delta
    assert_in_delta 0.3, 0.1 + 0.2, 0.001
  end

  def test_assert_raises
    assert_raises(ZeroDivisionError) { 10 / 0 }
  end
end
```

> **Note:** `0.1 + 0.2` equals `0.30000000000000004` in Ruby. To compare decimal
> numbers, prefer `assert_in_delta` to `assert_equal`.

### Testing that an error is raised

Sometimes your code must refuse a value with `raise` (see
[raising an error](erreurs.md#raising-an-error-with-raise)). `assert_raises` checks that the error really
happens. It returns the error: you can also check its message.

```ruby
require "minitest/autorun"

def withdraw(balance, amount)
  raise ArgumentError, "negative amount" if amount < 0
  balance - amount
end

class TestWithdraw < Minitest::Test
  def test_normal_withdrawal
    assert_equal 70, withdraw(100, 30)
  end

  def test_negative_withdrawal_refused
    error = assert_raises(ArgumentError) { withdraw(100, -5) }
    assert_equal "negative amount", error.message
  end
end
```

### A clearer message

Every assertion accepts one last argument: the message printed when it fails.

```ruby
require "minitest/autorun"

class TestCart < Minitest::Test
  def test_cart_empty_at_start
    cart = []
    assert cart.empty?, "the cart should be empty at the start"
  end
end
```

## Testing a class

For a class, create an object in the test, call its methods, then check its
state. The `setup` method runs **before each test**: it prepares a fresh object.

```ruby
require "minitest/autorun"

class Account
  attr_reader :balance

  def initialize
    @balance = 0
  end

  def deposit(amount)
    @balance += amount
  end
end

class TestAccount < Minitest::Test
  def setup
    @account = Account.new
  end

  def test_starting_balance
    assert_equal 0, @account.balance
  end

  def test_deposit_increases_balance
    @account.deposit(50)
    @account.deposit(20)
    assert_equal 70, @account.balance
  end
end
```

Each test gets its own `@account`: the deposit in the second test does not change the
first one.

## Organizing code and tests

In a project, the code and the tests live in separate files. By convention, the
code goes in `lib/` and the tests in `test/`. The test file has the name of the
tested file, preceded by `test_`.

```text
my_project/
├── lib/
│   └── calc.rb
└── test/
    └── test_calc.rb
```

```ruby norun
# lib/calc.rb
module Calc
  def self.double(x)
    x * 2
  end
end
```

```ruby norun
# test/test_calc.rb
require "minitest/autorun"
require_relative "../lib/calc"

class TestCalc < Minitest::Test
  def test_double
    assert_equal 42, Calc.double(21)
  end
end
```

From the `my_project` folder:

```sh
ruby test/test_calc.rb
```

> **Note:** `../` means "the parent folder". From `test/`, `../lib/calc`
> refers to `lib/calc.rb` (see [modules](modules.md)).

## Writing the test first

A very common way of working is to write the test **before** the code. You
move forward in three steps, which repeat: **red**, **green**, **improve**.

Example: a method `grade(score)` that returns `"Good"` from 14 up,
`"Pass"` from 10 up, and `"Fail"` below.

### Step 1: red, the test fails

First write the test, as if the method already existed.

```ruby norun
require "minitest/autorun"

class TestGrade < Minitest::Test
  def test_grade_good
    assert_equal "Good", grade(15)
  end
end
```

The test fails, since `grade` does not exist yet:

```text
  1) Error:
TestGrade#test_grade_good:
NoMethodError: undefined method `grade' for an instance of TestGrade
```

That is normal, and useful: you know the test really checks something.

### Step 2: green, the test passes

Write the simplest code that makes the tests pass. Add one test at a time
for each case.

```ruby
require "minitest/autorun"

def grade(score)
  if score >= 14
    "Good"
  elsif score >= 10
    "Pass"
  else
    "Fail"
  end
end

class TestGrade < Minitest::Test
  def test_grade_good
    assert_equal "Good", grade(15)
  end

  def test_grade_pass
    assert_equal "Pass", grade(10)
  end

  def test_grade_fail
    assert_equal "Fail", grade(7)
  end
end
```

All the tests are green: three dots `...` in the summary.

### Step 3: improve the code

Now that the tests protect you, make the code clearer. Here, a `case`
with ranges reads better. Run the tests again: if they stay green, nothing is
broken.

```ruby
require "minitest/autorun"

def grade(score)
  case score
  when 14.. then "Good"
  when 10...14 then "Pass"
  else "Fail"
  end
end

class TestGrade < Minitest::Test
  def test_grade_good
    assert_equal "Good", grade(15)
  end

  def test_grade_pass
    assert_equal "Pass", grade(10)
  end

  def test_grade_fail
    assert_equal "Fail", grade(7)
  end
end
```

> **Remember:** red (a test that fails), green (the minimal code that makes it
> pass), improve (without changing the behavior). Then start again with the next
> case.

## Naming your tests well

A good test name says **what is checked**. When it fails, its name is enough to
understand the problem.

| Less clear | Clearer |
| --- | --- |
| `test_1` | `test_double_of_zero_is_zero` |
| `test_withdraw` | `test_negative_withdrawal_refused` |
| `test_grade` | `test_grade_good_from_14` |

A few habits:

- a test checks **a single idea**;
- test the edge cases: zero, an empty array, the value right at the boundary
  (`grade(14)`, `grade(13.9)`);
- the test class is named after what it tests: `TestAccount` for `Account`.

## RSpec in a nutshell

Many Ruby projects use **RSpec**, another testing library. It is
installed as a gem (`gem install rspec`) and reads almost like an English
sentence:

```ruby norun
# spec/calc_spec.rb
require_relative "../lib/calc"

RSpec.describe Calc do
  it "doubles a number" do
    expect(Calc.double(21)).to eq(42)
  end
end
```

```sh
rspec
```

The ideas are the same as with Minitest. Start with Minitest: it is already installed.

## Common mistakes

### A method name without test_

Minitest only runs the methods whose name starts with `test_`. The others are
ignored, without any error message.

```ruby norun
class TestDouble < Minitest::Test
  def double_of_four
    assert_equal 8, double(4)
  end
end
```

```text
0 runs, 0 assertions, 0 failures, 0 errors, 0 skips
```

`0 runs` should alert you. Rename the method `test_double_of_four`.

### Forgetting require "minitest/autorun"

Without this line, Ruby does not know Minitest.

```ruby norun
class TestDouble < Minitest::Test
end
```

```text
main.rb:1:in `<main>': uninitialized constant Minitest (NameError)
```

Add `require "minitest/autorun"` as the first line.

### Swapping the expected value and the actual value

`assert_equal` expects the expected value **first**, **then** the computed one.
Swapped, the failure message misleads you.

```ruby norun
def price_with_tax(price)
  price * 1.2
end

assert_equal price_with_tax(10), 12.5
```

```text
Expected: 12.0
  Actual: 12.5
```

The message claims that your code gave `12.5`, when it gave `12.0`. Write
`assert_equal 12.5, price_with_tax(10)`.

### Giving a value to assert_raises instead of a block

The code that must raise the error goes **in a block**. Otherwise, it runs before
`assert_raises`, and the test crashes.

```ruby norun
assert_raises(ZeroDivisionError, 10 / 0)
```

```text
  1) Error:
TestDivision#test_division_by_zero:
ZeroDivisionError: divided by 0
```

Write `assert_raises(ZeroDivisionError) { 10 / 0 }`.

## Test yourself

**1.** What do `.`, `F` and `E` mean in the Minitest result?

> **Answer**
>
> `.`: a passing test. `F`: a failure, an assertion gave a wrong result.
> `E`: an error, the code crashed during the test.

**2.** The summary shows `0 runs`. What is the most likely cause?

> **Answer**
>
> The test methods do not start with `test_`: Minitest does not see them.

**3.** What are the three steps when you write the test first?

> **Answer**
>
> Red: write a test that fails. Green: write the simplest code that makes it
> pass. Improve: make the code clearer, keeping the tests green.

## Summary

- A test automatically checks that an input gives the expected output.
- With Minitest: `require "minitest/autorun"`, a class that inherits from
  `Minitest::Test`, `test_…` methods.
- `assert_equal expected, actual` is the most common assertion; `assert_raises`
  takes a block.
- `ruby test_file.rb` runs the tests: `.` passed, `F` failure, `E` error.
- Red, green, improve: write the test, then the code, then make it clearer.

Next: [debugging in Ruby](debogage.md)
