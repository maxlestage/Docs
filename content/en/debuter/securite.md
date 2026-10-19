# Security basics

A program must protect the data people trust it with: this page teaches you good habits for passwords, secrets, the database and everything the user types.

> **Goals:**
>
> - choose a good password and never store it in plain text;
> - compute a hash with `hashlib` and a salt;
> - keep secrets out of the code, in environment variables;
> - prevent SQL injection with `?`;
> - check everything the user types.

## Why it's your business

You lock your door, even if nobody wants to burgle you. Programming is
the same: every day, robots try millions of passwords on every website.
A few simple habits prevent almost all problems.

## A good password

A short password is quickly guessed, even if it's complicated. A
computer tries every combination: the more there are, the longer it takes.
Let's compare:

```python
letters = 26 ** 8          # 8 lowercase letters
phrase = 2000 ** 5         # 5 words picked among 2000 common words
print(f"8 letters: {letters:,} combinations")
print(f"5 words  : {phrase:,} combinations")
print(f"The phrase is {phrase // letters:,} times harder to guess.")
```

```sortie
8 letters: 208,827,064,576 combinations
5 words  : 32,000,000,000,000,000 combinations
The phrase is 153,236 times harder to guess.
```

`**` means "to the power of" (`26 ** 8`: 26 multiplied by itself 8
times) and `:,` separates the thousands. So remember:

- a **phrase** of several words (`horse-lamp-storm-tulip-violin`) is
  stronger and easier to remember than `Xk7$p2!a`;
- **a different password for each site**: if one site gets robbed, the
  others stay protected;
- a **password manager** (Bitwarden, KeePassXC…) remembers them for you.
  You only remember one phrase: the manager's own.

Try it: replace `2000 ** 5` with `2000 ** 3` (3 words). The phrase is
still stronger than 8 letters, but much less than with 5 words.

> **Remember:** a long phrase beats a short, weird word.

## Never store a password in plain text

Does your program manage accounts? It must **never** save the password
itself: if the database is stolen, everything would be readable.

Instead, you store a **hash**. It's like a fingerprint: it recognizes a
person, but you can't rebuild the person from the fingerprint.

### A hash with hashlib

The `hashlib` [module](glossaire.md#module) computes a hash:

```python
import hashlib

for word in ["sunny", "sunny", "Sunny"]:
    digest = hashlib.sha256(word.encode()).hexdigest()
    print(f"{word:7} → {digest[:16]}…")
```

```sortie
sunny   → c2333a7e3a607935…
sunny   → c2333a7e3a607935…
Sunny   → adf259f684470448…
```

| Word    | Hash (start)       | What you notice                           |
|---------|--------------------|-------------------------------------------|
| `sunny` | `c2333a7e3a607935` | always the same for the same word         |
| `sunny` | `c2333a7e3a607935` | identical: so you can **compare**         |
| `Sunny` | `adf259f684470448` | change a single letter, everything changes |

`.encode()` turns the text into [bytes](glossaire.md#byte), the only
thing `hashlib` accepts. `[:16]` only shows the start.

### Adding a salt

One problem remains: attackers have ready-made lists of hashes
(`sunny` → `c2333a7e…`). The solution: a **salt**, a short text added to
the password, different for each person and stored next to the hash. We
also use `pbkdf2_hmac`, a function that is slow on purpose, to discourage
attackers.

```python
import hashlib
import hmac

def hash_password(password, salt):
    return hashlib.pbkdf2_hmac("sha256", password.encode(), salt, 100_000)

# In real life, the salt is random: salt = os.urandom(16)
# Here, it is fixed so the result is always the same.
salt = b"salt-for-alice"
stored = hash_password("horse-lamp-storm", salt)
print("Stored in the database:", stored.hex()[:16], "…")

def check(attempt):
    return hmac.compare_digest(hash_password(attempt, salt), stored)

print("horse-lamp-storm:", check("horse-lamp-storm"))
print("horse-lamp      :", check("horse-lamp"))
```

```sortie
Stored in the database: 4c01a1b221867acf …
horse-lamp-storm: True
horse-lamp      : False
```

Let's follow the check step by step:

| Step | What the program does                                 | Result         |
|------|-------------------------------------------------------|----------------|
| 1    | the person types their password                       | `attempt`      |
| 2    | we compute the hash of `attempt` with the same salt   | a hash         |
| 3    | we compare it to the stored hash                      | `True`/`False` |

The plain-text password is **never** saved. `hmac.compare_digest`
compares without letting an attacker measure the time it takes: prefer it
to `==`.

Try it: change the salt to `b"salt-for-bob"`. The hash changes, the check still works.

> **Remember:** store a salted hash, never the password.

## Secrets out of the code

A **secret** (database password, [API](glossaire.md#api) key…) is never
written in the code: the code ends up on [GitHub](git.md).

You put it in an **environment variable**: a label that the
[system](glossaire.md#operating-system) gives the program when it starts.

```python
import os

key = os.environ.get("WEATHER_KEY")
if key is None:
    print("Error: the WEATHER_KEY variable is not set.")
else:
    print("Key found, it is", len(key), "characters long.")
```

```sortie
Error: the WEATHER_KEY variable is not set.
```

To set the variable, in the [terminal](terminal.md), before running:

```sh
export WEATHER_KEY="my-secret-key"      # macOS and Linux
set WEATHER_KEY=my-secret-key           # Windows (Command Prompt)
python3 weather.py
```

```text
Key found, it is 13 characters long.
```

Many projects keep these variables in a `.env` file. This file must
**never** go into Git: write `.env` in your
[`.gitignore`](git.md#ignoring-files-gitignore).

> **Remember:** the code says **where** to find the secret, never **what**
> the secret is.

## SQL injection

In [databases](bases-de-donnees.md), you saw that you must not glue text
into an SQL statement. Here's why.

### The vulnerable code

A site checks a username by gluing what you type into the SQL statement.
An attacker types a strange text instead of a name:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE accounts (name TEXT, secret TEXT)")
con.executemany("INSERT INTO accounts VALUES (?, ?)",
                [("alice", "code 1234"), ("bob", "code 9876")])

user_input = "x' OR '1'='1"
query = "SELECT name, secret FROM accounts WHERE name = '" + user_input + "'"
print(query)
print(con.execute(query).fetchall())
```

```sortie
SELECT name, secret FROM accounts WHERE name = 'x' OR '1'='1'
[('alice', 'code 1234'), ('bob', 'code 9876')]
```

Look at the statement that was built. The attacker closed the quote, then
added `OR '1'='1'`, which is **always true**. So the database returns
**all** the accounts, with their secrets.

```text
WHERE name = 'x' OR '1'='1'
      └───┬────┘    └──┬──┘
        false     always true   →  false OR true = true for every row
```

### The fix: `?`

With a `?`, the database receives the statement and the input
**separately**. The input stays plain text, never SQL:

```python
import sqlite3

con = sqlite3.connect(":memory:")
con.execute("CREATE TABLE accounts (name TEXT, secret TEXT)")
con.executemany("INSERT INTO accounts VALUES (?, ?)",
                [("alice", "code 1234"), ("bob", "code 9876")])

user_input = "x' OR '1'='1"
result = con.execute("SELECT name, secret FROM accounts WHERE name = ?", (user_input,))
print(result.fetchall())
```

```sortie
[]
```

The database looks for a user who is really called `x' OR '1'='1`: there
isn't one. Empty list, attack failed.

Try it: replace `user_input` with `"alice"`. You get
`[('alice', 'code 1234')]`.

> **Remember:** never use `+` or an f-string to build SQL: use `?`.

## Never trust what you're sent

Anything that comes from outside can be wrong, by mistake or on purpose: a
form, a file, a URL. Check it **before** you use it. It's the bouncer at
the door of a club: he looks at every ticket.

```python
def read_age(text):
    text = text.strip()
    if not text.isdigit():
        return "rejected: not a number"
    age = int(text)
    if age > 130:
        return "rejected: impossible age"
    return f"accepted: {age} years old"

for user_input in ["42", " 7 ", "-3", "twelve", "999", "'; DROP TABLE"]:
    print(f"{user_input!r:16} {read_age(user_input)}")
```

```sortie
'42'             accepted: 42 years old
' 7 '            accepted: 7 years old
'-3'             rejected: not a number
'twelve'         rejected: not a number
'999'            rejected: impossible age
"'; DROP TABLE"  rejected: not a number
```

`strip()` removes the spaces around, `isdigit()` checks that there are
only digits, `!r` shows the quotes so you can see the spaces.

Try it: add `"130"` to the list. It is accepted: the limit is "more than
130".

> **Remember:** check the **form** (a number?) and the **value** (no more than 130?).

## HTTPS in two sentences

`https://` (with an **s**) encrypts what travels between the browser and
the [server](glossaire.md#server): nobody along the way can read a
password or a card number. Never send personal data to an address that
starts with `http://` without the **s**.

## Updates

Security holes are found every week; updates fix them. Update your
system, your browser, your language and the
[packages](glossaire.md#package) of your projects.

## The 5 rules to remember

```text
1. A long passphrase, different for each site, in a password manager.
2. Never a password in plain text: a salted hash.
3. Never a secret in the code or in Git: environment variables.
4. Never SQL built with +: always ?.
5. Never trust an input: check it, and keep things up to date.
```

## Common mistakes

### A secret sent to Git

You write a key in `config.py`, you make a commit, then you delete the
file in a second commit. You think you're safe:

```sh
git show HEAD~1:config.py
```

```text
API_KEY = "sk-secret-123456"
```

This command shows `config.py` as it was in the previous commit: the key
is still there. The fix: consider the key **stolen**, **revoke** it
(cancel it) on the site that gave it to you, and keep the new one in an
environment variable.

### Comparing the typed password to the hash

You do store a hash… but you compare it to the typed text:

```python
import hashlib

stored = hashlib.sha256("sunny".encode()).hexdigest()
attempt = "sunny"

if attempt == stored:
    print("Welcome!")
else:
    print("Wrong password.")
```

```sortie
Wrong password.
```

The right password is rejected: you are comparing `sunny` to `c2333a7e…`.
You must compute the hash of the attempt, then compare two hashes
(`check`).

### Forgetting .encode()

```python norun
import hashlib

digest = hashlib.sha256("sunny")
```

```text
Traceback (most recent call last):
  File "main.py", line 3, in <module>
    digest = hashlib.sha256("sunny")
TypeError: Strings must be encoded before hashing
```

`hashlib` wants bytes, not text. Write
`hashlib.sha256("sunny".encode())`.

## Test yourself

**1.** Why store a hash rather than the password?

> **Answer**
>
> If the database is stolen, the attacker only has hashes, which are useless.

**2.** A key has already been sent to GitHub. Is it enough to delete it and make a new commit?

> **Answer**
>
> No, it stays in the history: revoke it and create a new one.

**3.** What is the danger of `"… WHERE name = '" + user_input + "'"`?

> **Answer**
>
> SQL injection: the input can contain SQL (`x' OR '1'='1`). Write
> `WHERE name = ?` and pass `(user_input,)` separately.

## Summary

- A long passphrase, unique for each site, kept in a password manager.
- Store a salted hash (`hashlib.pbkdf2_hmac`), never the password.
- Secrets go in environment variables, never in the code or in Git.
- `?` prevents SQL injection; check every input before you use it.
- `https://` encrypts exchanges; updates plug security holes.

Next: [learning to program well](apprendre.md).
