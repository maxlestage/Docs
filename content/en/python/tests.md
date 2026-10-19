# Testing your code

This page shows how to write automated tests in Python, with `assert`, the `unittest` module that comes with Python, then pytest and doctest, to check that your code really does what you expect.

> **Goals:**
>
> - understand what an automated test is for;
> - check a value with `assert`;
> - write a test class with `unittest` and read its result;
> - know about pytest and doctest;
> - write the test before the code, in three steps.

## Why test?

A **test** is a small program that checks another program (see
[the glossary](../debuter/glossaire.md#test)). It calls your code with known values and
compares the result with the one you expect.

Without tests, you check by hand, with `print`, and you start over after every change.
With tests:

- a single command checks everything, in one second;
- if a change breaks something, you know right away;
- tests show how to use your code: they are documentation that works.

> **Remember:** a test answers a simple question: "with this input, do I really get this
> output?".

## The simplest check: assert

The `assert` keyword checks that a condition is true. If it is, nothing happens.
Otherwise, Python raises an `AssertionError` and stops the program.

```python
def double(x):
    return x * 2


assert double(4) == 8
assert double(0) == 0
assert double(-3) == -6
print("All tests pass.")
```

```sortie
All tests pass.
```

Add a message after a comma: it is displayed when the check fails. Here is a `double`
that gets it wrong (it adds 2 instead of multiplying):

```python norun
def double(x):
    return x + 2


assert double(4) == 8, "double(4) should be 8"
```

```text
Traceback (most recent call last):
  File "/home/you/main.py", line 5, in <module>
    assert double(4) == 8, "double(4) should be 8"
           ^^^^^^^^^^^^^^
AssertionError: double(4) should be 8
```

`assert` is enough for small experiments. But the program stops at the first failure,
and you do not know whether the following checks pass. To go further, you need a real
testing tool.

## Your first test with unittest

The **unittest** module comes with Python: nothing to install. A test file always
follows the same pattern:

1. `import unittest` loads the module;
2. a test class inherits from `unittest.TestCase` (see [inheritance](heritage.md));
3. each method whose name starts with `test_` is a test;
4. `unittest.main()` runs all the tests in the file.

```python
import unittest


def double(x):
    return x * 2


class TestDouble(unittest.TestCase):
    def test_double_of_4(self):
        self.assertEqual(double(4), 8)

    def test_double_of_zero(self):
        self.assertEqual(double(0), 0)


program = unittest.main(argv=["x"], exit=False)
print(program.result.wasSuccessful())
```

```sortie
True
```

`self.assertEqual(double(4), 8)` reads: "check that `double(4)` is equal to `8`".

> **Note:** in a real test file, the last line is simply `unittest.main()`. Here,
> `argv=["x"], exit=False` stops `unittest` from reading the terminal arguments and from
> quitting the program: this way you can display the result with `print`.

## Reading the result

`unittest` does not write its report to the normal output, but to the **error output**
(*stderr*): that is why it does not appear in the output above. In your terminal, you
see both:

```text
..
----------------------------------------------------------------------
Ran 2 tests in 0.000s

OK
True
```

- Each `.` is a passing test.
- `Ran 2 tests` counts the tests that ran; the time changes every time.
- `OK`: everything is green.

> **Tip:** `unittest.main(verbosity=2)` displays the name of each test. In the terminal,
> `python3 -m unittest -v` finds and runs all the `test*.py` files in the folder.

## Reading a failure

When a check fails, `unittest` displays an `F` and explains the problem:

```python norun
import unittest


def double(x):
    return x + 2


class TestDouble(unittest.TestCase):
    def test_double_of_4(self):
        self.assertEqual(double(4), 8)


unittest.main()
```

```text
F
======================================================================
FAIL: test_double_of_4 (__main__.TestDouble.test_double_of_4)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/home/you/main.py", line 10, in test_double_of_4
    self.assertEqual(double(4), 8)
    ~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^
AssertionError: 6 != 8

----------------------------------------------------------------------
Ran 1 test in 0.000s

FAILED (failures=1)
```

The message gives you everything you need: the test (`test_double_of_4`), the line
(`10`) and the failed comparison (`6 != 8`: your code gave `6`, `8` was expected).

- `F` (*failure*): a check gave a wrong result.
- `E` (*error*): the code crashed during the test (an unknown name, for example).

## Assertions

An **assertion** is a check. `unittest` offers many of them; here are the most useful.

| Assertion | Checks that… |
| --- | --- |
| `assertEqual(a, b)` | `a == b` |
| `assertNotEqual(a, b)` | `a != b` |
| `assertTrue(x)` / `assertFalse(x)` | `x` is true / false |
| `assertIsNone(x)` | `x is None` |
| `assertIn(x, items)` | `x` is in the list |
| `assertAlmostEqual(a, b)` | two floats are very close |
| `assertRaises(Error)` | the code really raises this error |

```python
import unittest


class TestAssertions(unittest.TestCase):
    def test_equality(self):
        self.assertEqual("hello".upper(), "HELLO")
        self.assertEqual(sorted([3, 1, 2]), [1, 2, 3])

    def test_true_false(self):
        self.assertTrue(2 in [1, 2, 3])
        self.assertFalse([])

    def test_none_and_in(self):
        self.assertIsNone({"a": 1}.get("b"))
        self.assertIn("pear", ["apple", "pear"])

    def test_floats(self):
        self.assertAlmostEqual(0.1 + 0.2, 0.3)


program = unittest.main(argv=["x"], exit=False)
print(program.result.testsRun, "tests run")
print("Successful:", program.result.wasSuccessful())
```

```sortie
4 tests run
Successful: True
```

> **Note:** `0.1 + 0.2` is `0.30000000000000004` in Python (see
> [numbers](nombres.md)). To compare floats, prefer `assertAlmostEqual` to
> `assertEqual`.

### Testing that an error is raised

Sometimes your code must refuse a value with `raise` (see [errors](erreurs.md)). Write
`with self.assertRaises(...)`: the block below must raise this error. The `as` gets the
error so you can check its message.

```python
import unittest


def withdraw(balance, amount):
    if amount < 0:
        raise ValueError("negative amount")
    return balance - amount


class TestWithdraw(unittest.TestCase):
    def test_normal_withdrawal(self):
        self.assertEqual(withdraw(100, 30), 70)

    def test_negative_withdrawal_refused(self):
        with self.assertRaises(ValueError) as context:
            withdraw(100, -5)
        self.assertEqual(str(context.exception), "negative amount")


program = unittest.main(argv=["x"], exit=False)
print(program.result.wasSuccessful())
```

```sortie
True
```

## Testing a class with setUp

For a class, create an object in the test, call its methods, then check its state. The
`setUp` method runs **before each test**: it prepares a fresh object.

```python
import unittest


class Account:
    def __init__(self):
        self.balance = 0

    def deposit(self, amount):
        self.balance += amount


class TestAccount(unittest.TestCase):
    def setUp(self):
        self.account = Account()

    def test_starting_balance(self):
        self.assertEqual(self.account.balance, 0)

    def test_deposit_increases_balance(self):
        self.account.deposit(50)
        self.account.deposit(20)
        self.assertEqual(self.account.balance, 70)


program = unittest.main(argv=["x"], exit=False)
print(program.result.wasSuccessful())
```

```sortie
True
```

Each test gets its own `self.account`: the deposit in the second test does not change
the first one.

## pytest: shorter tests

Many Python projects use **pytest**. It is not part of Python: you install it with pip
(see [organizing a project](projet.md)).

```sh
python3 -m pip install pytest
```

With pytest, a test is a plain `test_…` function, and the check is an ordinary `assert`.
No class, no `self.assertEqual`.

```python norun
# test_calc.py
import pytest


def double(x):
    return x * 2


def test_double_of_4():
    assert double(4) == 8


def test_double_of_zero():
    assert double(0) == 0


def test_division_by_zero():
    with pytest.raises(ZeroDivisionError):
        1 / 0
```

The `pytest` command finds all the `test_*.py` files in the folder and runs them:

```sh
pytest
```

```text
============================= test session starts ==============================
platform linux -- Python 3.13.16, pytest-9.1.1, pluggy-1.7.0
rootdir: /home/you/my_project
collected 3 items

test_calc.py ...                                                         [100%]

============================== 3 passed in 0.03s ===============================
```

When a test fails, pytest shows the line and the values. With a `double` that adds 2:

```text
=================================== FAILURES ===================================
_______________________________ test_double_of_4 _______________________________

    def test_double_of_4():
>       assert double(4) == 8
E       assert 6 == 8
E        +  where 6 = double(4)

test_calc.py:10: AssertionError
=========================== short test summary info ============================
FAILED test_calc.py::test_double_of_4 - assert 6 == 8
FAILED test_calc.py::test_double_of_zero - assert 2 == 0
========================= 2 failed, 1 passed in 0.04s ==========================
```

> **Tip:** pytest also runs tests written with `unittest`. You can start with
> `unittest`, then switch to pytest without rewriting anything.

## doctest: examples that check themselves

A **docstring** is the text between triple quotes under a function (see
[writing good Python](style.md)). If you write examples in it as in the Python console
(`>>>` then the expected result), the `doctest` module checks them.

```python
def average(grades):
    """Return the average of a list of grades.

    >>> average([10, 20])
    15.0
    >>> average([12, 13, 14])
    13.0
    """
    return sum(grades) / len(grades)


import doctest

print(doctest.testmod())
```

```sortie
TestResults(failed=0, attempted=2)
```

`attempted=2`: two examples checked; `failed=0`: no failures. The example is both
documentation and a test.

## What should you test?

You cannot test everything. Aim for the cases that matter:

| Case | Example for `average(grades)` |
| --- | --- |
| The normal case | `[10, 20]` gives `15.0` |
| A single element | `[12]` gives `12.0` |
| The boundary | a grade of `0`, a grade of `20` |
| The empty case | `[]`: an error or a default value? |
| Bad inputs | a negative grade must be refused |

A few habits:

- a test checks **a single idea**;
- a good test name says what is checked: `test_negative_withdrawal_refused` rather than
  `test_2`;
- when you fix a bug, first add a test that shows it: it will not come back.

## Writing the test first

A very common way of working is to write the test **before** the code. You move forward
in three steps, which repeat: **red**, **green**, **improve**.

Example: a function `rating(grade)` that returns `"Good"` from 14, `"Pass"` from 10,
and `"Fail"` below that.

### Step 1: red, the test fails

First write the test, as if the function already existed.

```python norun
import unittest


class TestRating(unittest.TestCase):
    def test_rating_good(self):
        self.assertEqual(rating(15), "Good")


unittest.main()
```

The test fails, since `rating` does not exist yet:

```text
E
======================================================================
ERROR: test_rating_good (__main__.TestRating.test_rating_good)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/home/you/main.py", line 6, in test_rating_good
    self.assertEqual(rating(15), "Good")
                     ^^^^^^
NameError: name 'rating' is not defined
```

That is normal, and useful: you know the test really checks something.

### Step 2: green, the test passes

Write the simplest code that makes the tests pass. Add one test at a time for each
case, without forgetting the boundary (`14`).

```python
import unittest


def rating(grade):
    if grade >= 14:
        return "Good"
    if grade >= 10:
        return "Pass"
    return "Fail"


class TestRating(unittest.TestCase):
    def test_rating_good(self):
        self.assertEqual(rating(15), "Good")
        self.assertEqual(rating(14), "Good")

    def test_rating_pass(self):
        self.assertEqual(rating(10), "Pass")

    def test_rating_fail(self):
        self.assertEqual(rating(7), "Fail")


program = unittest.main(argv=["x"], exit=False)
print(program.result.wasSuccessful())
```

```sortie
True
```

### Step 3: improve the code

Now that the tests protect you, make the code clearer or more flexible. Here, the
thresholds become a list: adding one no longer needs a new `if`. Run the tests again: if
they stay green, nothing is broken.

```python
import unittest

THRESHOLDS = [(14, "Good"), (10, "Pass")]


def rating(grade):
    for threshold, name in THRESHOLDS:
        if grade >= threshold:
            return name
    return "Fail"


class TestRating(unittest.TestCase):
    def test_rating_good(self):
        self.assertEqual(rating(15), "Good")
        self.assertEqual(rating(14), "Good")

    def test_rating_pass(self):
        self.assertEqual(rating(10), "Pass")

    def test_rating_fail(self):
        self.assertEqual(rating(7), "Fail")


program = unittest.main(argv=["x"], exit=False)
print(program.result.wasSuccessful())
```

```sortie
True
```

> **Remember:** red (a test that fails), green (the minimal code that makes it pass),
> improve (without changing the behavior). Then start again with the next case.

## Common mistakes

### A method name without test_

`unittest` only runs the methods whose name starts with `test_`. The others are
ignored, with no error message.

```python norun
import unittest


class TestDouble(unittest.TestCase):
    def double_of_four(self):
        self.assertEqual(4 * 2, 8)


unittest.main()
```

```text
----------------------------------------------------------------------
Ran 0 tests in 0.000s

NO TESTS RAN
```

`Ran 0 tests` should alert you. Rename the method `test_double_of_four`.

### Parentheses around assert

`assert` is not a function. With parentheses, the condition and the message form a
**tuple**, which is always true: the check becomes useless.

```python norun
def double(x):
    return x + 2


assert(double(4) == 8, "double(4) should be 8")
print("End of program")
```

```text
/home/you/main.py:5: SyntaxWarning: assertion is always true, perhaps remove parentheses?
  assert(double(4) == 8, "double(4) should be 8")
End of program
```

The bug goes unnoticed. Write `assert double(4) == 8, "double(4) should be 8"`.

### Computing the value instead of passing it to assertRaises

The code that must raise the error goes **inside a `with` block**. Otherwise, it runs
before `assertRaises`, and the test crashes.

```python norun
import unittest


class TestDivision(unittest.TestCase):
    def test_division_by_zero(self):
        self.assertRaises(ZeroDivisionError, 10 / 0)


unittest.main()
```

```text
ERROR: test_division_by_zero (__main__.TestDivision.test_division_by_zero)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/home/you/main.py", line 6, in test_division_by_zero
    self.assertRaises(ZeroDivisionError, 10 / 0)
                                         ~~~^~~
ZeroDivisionError: division by zero
```

Write `with self.assertRaises(ZeroDivisionError):` then `10 / 0` below it.

### Comparing floats with assertEqual

```python norun
self.assertEqual(0.1 + 0.2, 0.3)
```

```text
AssertionError: 0.30000000000000004 != 0.3
```

Write `self.assertAlmostEqual(0.1 + 0.2, 0.3)`.

## Test yourself

**1.** What do `.`, `F` and `E` mean in the result of `unittest`?

> **Answer**
>
> `.`: a passing test. `F`: a failure, an assertion gave a wrong result. `E`: an error,
> the code crashed during the test.

**2.** The summary shows `Ran 0 tests`. What is the most likely cause?

> **Answer**
>
> The test methods do not start with `test_`: `unittest` does not see them.

**3.** What is the main difference between a `unittest` test and a pytest test?

> **Answer**
>
> With `unittest`, you write a class that inherits from `TestCase` and you check with
> `self.assertEqual`. With pytest, a plain `test_…` function and an `assert` are enough;
> but pytest must be installed with pip.

## Summary

- A test automatically checks that an input gives the expected output.
- `assert condition, "message"` is the simplest check, without parentheses.
- With `unittest`: a class that inherits from `unittest.TestCase`, `test_…` methods,
  `self.assertEqual` and `with self.assertRaises(...)`.
- pytest (to install) makes tests shorter; doctest checks the examples in docstrings.
- Red, green, improve: write the test, then the code, then make it clearer.

Next: [debugging in Python](debogage.md)
