# Creating and publishing your own gem

This page shows, step by step, how to turn your code into a gem: write it, test it, package it, publish it, and use it in another project.

> **Goals:**
>
> - know what a gem is and what its gemspec contains;
> - create the skeleton of a gem with `bundle gem`;
> - write the code in `lib/`, with a version number, and test it;
> - build the package with `gem build` and publish it on rubygems.org;
> - choose a version number and use a local gem with Bundler.

## What is a gem?

A [gem](../debuter/glossaire.md#gem) is a Ruby [library](../debuter/glossaire.md#library)
packaged to be shared. You already use some: the page
[Organizing a project](projet.md) shows `gem install` and Bundler. Here, you move to the
other side: you **make** the gem.

Think of a parcel:

```text
  +--------------------------------------------+
  |  LABEL (the gemspec)                       |
  |  name: greetings      version: 0.1.0       |
  |  author: Alice        summary: Say hello   |
  +--------------------------------------------+
  |  CONTENTS                                  |
  |    lib/greetings.rb                        |
  |    lib/greetings/version.rb                |
  |    README.md                               |
  +--------------------------------------------+
         |
         |  gem build   (you seal the parcel)
         v
  greetings-0.1.0.gem
         |
         |  gem push    (you send it to the post office: rubygems.org)
         v
  everybody can type: gem install greetings
```

- The **contents** are your Ruby code, stored in `lib/`.
- The **label** is the *gemspec*: the name, the version, the author, the list of
  files.
- The **post office** is [rubygems.org](https://rubygems.org), the website that
  distributes gems.

> **Remember:** a gem = code in `lib/` + a label (the gemspec). You build it with
> `gem build` and publish it with `gem push`.

## The gem's code: a module and a method

All the code of a gem lives in a [module](../debuter/glossaire.md#module) that bears its
name (see [modules](modules.md)). That way, its names don't get mixed up with those of
other gems. Our `greetings` gem says hello in three languages:

```ruby
# lib/greetings.rb
module Greetings
  PHRASES = { en: "Hello", fr: "Bonjour", es: "Hola" }

  def self.hello(name, language: :en)
    phrase = PHRASES.fetch(language)
    "#{phrase}, #{name}!"
  end
end

puts Greetings.hello("Alice")
puts Greetings.hello("Bob", language: :es)
```

```sortie
Hello, Alice!
Hola, Bob!
```

Let's follow the second call, `Greetings.hello("Bob", language: :es)`:

| Step | Line | name | language | phrase | Result |
|------|------|------|----------|--------|--------|
| 1 | entering `hello` | `"Bob"` | `:es` | — | — |
| 2 | `PHRASES.fetch(language)` | `"Bob"` | `:es` | `"Hola"` | — |
| 3 | `"#{phrase}, #{name}!"` | `"Bob"` | `:es` | `"Hola"` | `"Hola, Bob!"` |

- `self.hello` is called on the module itself: `Greetings.hello`.
- `language: :en` is a keyword argument, `:en` by default (see [methods](fonctions.md)).
- `fetch` raises a clear error if the language doesn't exist.

Try it: add `it: "Ciao"` to `PHRASES`, then call
`Greetings.hello("Léa", language: :it)`. You get `Ciao, Léa!`.

> **Remember:** a gem's code is stored in a module with the same name as the gem.

## The version number

Every gem has a version number, stored in a [constant](../debuter/glossaire.md#constant)
`VERSION`, in its own file:

```ruby
# lib/greetings/version.rb
module Greetings
  VERSION = "0.1.0"
end

puts "greetings version #{Greetings::VERSION}"
```

```sortie
greetings version 0.1.0
```

The main file `lib/greetings.rb` loads it on its first line:
`require_relative "greetings/version"`.

Why a separate file? Because the gemspec must read the version **without** loading
all the code. It's like writing the parcel number on a sticky note stuck outside: you
can read it without opening the box.

> **Remember:** the version is a string in `Greetings::VERSION`, in
> `lib/greetings/version.rb`.

## Creating the skeleton with bundle gem

Bundler creates all the starting folders and files for you. Type in the
[terminal](../debuter/glossaire.md#terminal):

```sh
bundle gem greetings --test=minitest
```

```text
Creating gem 'greetings'...
      create  greetings/Gemfile
      create  greetings/lib/greetings.rb
      create  greetings/lib/greetings/version.rb
      ...
      create  greetings/greetings.gemspec
      create  greetings/test/test_greetings.rb
Gem 'greetings' was successfully created.
```

Bundler sometimes asks questions first: answer `n` if you're not sure. Here is the layout you get:

```text
greetings/
├── lib/                        ← the CONTENTS of the parcel
│   ├── greetings.rb            ← the Greetings module
│   └── greetings/
│       └── version.rb          ← VERSION = "0.1.0"
├── test/                       ← the tests (not sent in the parcel)
│   ├── test_helper.rb
│   └── test_greetings.rb
├── greetings.gemspec           ← the LABEL
├── Gemfile                     ← the gems useful for development
├── Rakefile                    ← the tasks: rake test, rake build…
└── README.md                   ← the user guide
```

The naming rule: the gem `greetings` → the file `lib/greetings.rb` → the module
`Greetings`. For a gem `good_mood`, it would be `lib/good_mood.rb` and the module
`GoodMood`.

> **Remember:** `bundle gem name` creates the skeleton; the code goes in `lib/`, the
> tests in `test/`.

## The gemspec: the parcel's label

The file `greetings.gemspec` describes the gem. `bundle gem` writes one full of
`TODO`s to replace. Here is a simple, complete version:

```ruby norun
# greetings.gemspec
require_relative "lib/greetings/version"

Gem::Specification.new do |spec|
  spec.name     = "greetings"
  spec.version  = Greetings::VERSION
  spec.authors  = ["Alice Martin"]
  spec.email    = ["alice@example.com"]
  spec.summary  = "Say hello politely, in several languages."
  spec.homepage = "https://github.com/alice/greetings"
  spec.license  = "MIT"
  spec.required_ruby_version = ">= 3.0"

  spec.files = Dir["lib/**/*.rb"] + ["README.md"]
  spec.require_paths = ["lib"]
end
```

| Field | What it says | Required? |
|-------|--------------|-----------|
| `name` | the name of the gem, the one used by `gem install` | yes |
| `version` | the version number | yes |
| `authors` | who wrote it | yes |
| `summary` | a one-sentence summary | yes |
| `files` | the files put in the parcel | yes, otherwise the parcel is empty |
| `license` | what others are allowed to do with it | recommended |
| `require_paths` | where to look for the code for `require` | `lib` by default |

`Dir["lib/**/*.rb"]`: "all the `.rb` files in `lib/` and its subfolders".

### A label is a Ruby object

`Gem::Specification` is an ordinary Ruby class, always available. You can
make a label and read it:

```ruby
spec = Gem::Specification.new do |s|
  s.name    = "greetings"
  s.version = "0.1.0"
  s.summary = "Say hello"
  s.authors = ["Alice Martin"]
end

puts spec.name
puts spec.version
puts spec.full_name
puts spec.file_name
```

```sortie
greetings
0.1.0
greetings-0.1.0
greetings-0.1.0.gem
```

`file_name` is the name of the parcel that `gem build` will make.

Try it: replace `"0.1.0"` with `"1.0.0"`. The last two lines become
`greetings-1.0.0` and `greetings-1.0.0.gem`.

> **Remember:** the gemspec is Ruby: a `Gem::Specification.new` with at least
> the name, the version, the authors, the summary and the files.

## Testing the gem

Before sending a parcel, you check what's inside. For a gem, that's the
[tests](../debuter/glossaire.md#test) (see [Testing your code](tests.md)).

### With simple checks

The simplest way: compare the expected result with the actual result.

```ruby
module Greetings
  PHRASES = { en: "Hello", fr: "Bonjour", es: "Hola" }

  def self.hello(name, language: :en)
    "#{PHRASES.fetch(language)}, #{name}!"
  end
end

def check(expected, actual)
  status = expected == actual ? "ok" : "FAILED"
  puts "#{status}: #{actual.inspect}"
end

check "Hello, Alice!", Greetings.hello("Alice")
check "Hola, Ana!", Greetings.hello("Ana", language: :es)
check "Bonjour, Bob !", Greetings.hello("Bob", language: :fr)
```

```sortie
ok: "Hello, Alice!"
ok: "Hola, Ana!"
FAILED: "Bonjour, Bob!"
```

| Check | expected | actual | Equal? |
|-------|----------|--------|--------|
| 1 | `"Hello, Alice!"` | `"Hello, Alice!"` | yes → `ok` |
| 2 | `"Hola, Ana!"` | `"Hola, Ana!"` | yes → `ok` |
| 3 | `"Bonjour, Bob !"` | `"Bonjour, Bob!"` | no → `FAILED` |

The third failure comes from the **test**, not from the code: there is an extra space
before `!` in the expected value. A test can be wrong too: read it again.

Try it: fix the third line to `"Bonjour, Bob!"`. All three lines print
`ok`.

### With Minitest

In the skeleton, `test/test_greetings.rb` uses Minitest, which ships with Ruby:

```ruby
require "minitest/autorun"

module Greetings
  PHRASES = { en: "Hello", fr: "Bonjour", es: "Hola" }

  def self.hello(name, language: :en)
    "#{PHRASES.fetch(language)}, #{name}!"
  end
end

class TestGreetings < Minitest::Test
  def test_hello_in_english_by_default
    assert_equal "Hello, Alice!", Greetings.hello("Alice")
  end

  def test_hello_in_spanish
    assert_equal "Hola, Ana!", Greetings.hello("Ana", language: :es)
  end

  def test_unknown_language
    assert_raises(KeyError) { Greetings.hello("Max", language: :de) }
  end
end
```

In the real gem, the module isn't copied: `test/test_helper.rb` loads it from
`lib/`. You run the tests from the gem's folder:

```sh
bundle exec rake test
```

```text
Run options: --seed 40213

# Running:

...

Finished in 0.000731s, 4103.9672 runs/s, 4103.9672 assertions/s.

3 runs, 3 assertions, 0 failures, 0 errors, 0 skips
```

Three dots, three passing tests. The number after `--seed` and the time change
every time.

> **Remember:** test the gem before building it: `bundle exec rake test` must
> end with `0 failures, 0 errors`.

## Building the package: gem build

`gem build` reads the label, puts the files in the parcel and seals it:

```sh
gem build greetings.gemspec
```

```text
  Successfully built RubyGem
  Name: greetings
  Version: 0.1.0
  File: greetings-0.1.0.gem
```

The file `greetings-0.1.0.gem` is an archive. Inside, there are three pieces:

```text
greetings-0.1.0.gem
├── metadata.gz       ← the label (the gemspec, in YAML)
├── data.tar.gz       ← the contents: lib/ and README.md
└── checksums.yaml.gz ← fingerprints to check that nothing is damaged
```

Before publishing, try the parcel at home. `gem install` accepts a local file:

```sh
gem install ./greetings-0.1.0.gem
ruby -e 'require "greetings"; puts Greetings.hello("Alice", language: :es)'
```

```text
Successfully installed greetings-0.1.0
1 gem installed
Hola, Alice!
```

> **Remember:** `gem build file.gemspec` makes the `.gem` file;
> `gem install ./file.gem` tries it on your computer.

## Publishing on rubygems.org

Publishing means sending the parcel to the post office. These commands need the Internet
and an account: they are not run here.

First create an account on [rubygems.org](https://rubygems.org/sign_up). Then,
`gem signin` logs you in (only once) and `gem push` sends the parcel:

```sh
gem signin
gem push greetings-0.1.0.gem
```

```text
Pushing gem to https://rubygems.org...
Successfully registered gem: greetings (0.1.0)
```

A few minutes later, anyone can type `gem install greetings`.

> **Warning:** the name must be free (search for it on rubygems.org first), and a
> published version can **never** be replaced. To fix something, publish a new
> version (`0.1.1`). Never put a password or a secret key in a gem.

> **Remember:** `gem signin` once, then `gem push file.gem` for each new
> version.

## Version numbers

Gems follow **semantic versioning** (*semver*): three numbers,
`MAJOR.MINOR.PATCH`. Each number is a message to users.

```text
   1   .   4   .   2
   |       |       |
   |       |       +-- PATCH : a bug fixed, nothing else changes
   |       +---------- MINOR : something new, old code still works
   +------------------ MAJOR : a change that can break old code
```

| You have… | Before | After |
|-----------|--------|-------|
| fixed a typo in `"Hola"` | `1.4.2` | `1.4.3` |
| added the language `:it` | `1.4.3` | `1.5.0` |
| renamed `hello` to `greet` | `1.5.0` | `2.0.0` |
| the very first version, still unstable | — | `0.1.0` |

When a number goes up, the ones to its right go back to `0`.

### Comparing versions

Careful: comparing versions as text gives a wrong result.
`Gem::Version` compares number by number:

```ruby
p "1.10.0" > "1.9.0"
p Gem::Version.new("1.10.0") > Gem::Version.new("1.9.0")
p Gem::Version.new("1.4.2").segments
```

```sortie
false
true
[1, 4, 2]
```

As text, `"1.1…"` comes before `"1.9…"`, because the character `1` comes before `9`. As
versions, `10` is greater than `9`.

### The ~> operator

In a `Gemfile`, `gem "greetings", "~> 1.2"` means: "`1.2` or higher, but
lower than `2.0`". You accept new features, not changes that break things.

```ruby
rule = Gem::Requirement.new("~> 1.2")

%w[1.1.0 1.2.0 1.9.3 2.0.0].each do |v|
  puts "#{v}: #{rule.satisfied_by?(Gem::Version.new(v))}"
end
```

```sortie
1.1.0: false
1.2.0: true
1.9.3: true
2.0.0: false
```

| Step | v | At least 1.2? | Lower than 2.0? | Accepted? |
|------|---|---------------|-----------------|-----------|
| 1 | `1.1.0` | no | yes | `false` |
| 2 | `1.2.0` | yes | yes | `true` |
| 3 | `1.9.3` | yes | yes | `true` |
| 4 | `2.0.0` | yes | no | `false` |

Try it: replace `"~> 1.2"` with `"~> 1.2.0"`. Only `1.2.0` is still accepted: the
rule becomes "`1.2.0` or higher, but lower than `1.3`".

> **Remember:** MAJOR.MINOR.PATCH. Change the MAJOR when you break something,
> the MINOR for a new feature, the PATCH for a bug.

## Using a local gem with Bundler

While you're developing the gem, you want to try it in another project, without
publishing it. Bundler can take a gem from a folder, with `path:`.

```text
my-projects/
├── greetings/        ← the gem
└── welcome/          ← the project that uses it
    ├── Gemfile
    └── main.rb
```

```ruby norun
# welcome/Gemfile
source "https://rubygems.org"

gem "greetings", path: "../greetings"
```

```ruby norun
# welcome/main.rb
require "greetings"

puts Greetings.hello("Bob")
```

```sh
cd welcome
bundle install
bundle exec ruby main.rb
```

```text
Resolving dependencies...
Bundle complete! 1 Gemfile dependency, 2 gems now installed.
Use `bundle info [gemname]` to see where a bundled gem is installed.
Hello, Bob!
```

`bundle exec` runs the program with the gems from the `Gemfile`. With `path:`, Bundler
reads the folder directly: no need for `gem build`.

Try it: change `"Hello"` to `"Hi"` in the gem, then run
`bundle exec ruby main.rb` again. The program prints `Hi, Bob!`.

Once the gem is published, replace the line with `gem "greetings", "~> 0.1"`.

> **Remember:** `gem "name", path: "../folder"` uses a gem that isn't published yet,
> and follows your changes live.

## Common mistakes

### A required field is missing from the gemspec

```ruby norun
Gem::Specification.new do |spec|
  spec.name    = "greetings"
  spec.version = "0.1.0"
  spec.authors = ["Alice Martin"]
  spec.files   = Dir["lib/**/*.rb"]
end
```

```text
WARNING:  See https://guides.rubygems.org/specification-reference/ for help
ERROR:  While executing gem ... (Gem::InvalidSpecificationException)
    missing value for attribute summary
```

`gem build` refuses a parcel without a summary. Add `spec.summary = "…"`. Read the line
`missing value for attribute …`: it names the missing field.

### A missing key with fetch

```ruby norun
PHRASES = { en: "Hello", fr: "Bonjour" }
puts PHRASES.fetch(:de)
```

```text
main.rb:2:in `fetch': key not found: :de (KeyError)
	from main.rb:2:in `<main>'
```

The language `:de` isn't in the hash. Add it (`de: "Hallo"`), or give a
fallback value: `PHRASES.fetch(language, "Hello")`.

### A wrong path in path:

```ruby norun
# welcome/Gemfile
gem "greetings", path: "../greeting"
```

```text
The path `/home/alice/my-projects/greeting` does not exist.
```

The folder name is missing an `s`. The path starts from the `Gemfile`'s folder: check it
with `ls ../`.

## Test yourself

**Question 1:** which file serves as a gem's "label", and what does it contain?

> **Answer**
>
> The gemspec (`greetings.gemspec`). It gives the name, the version, the authors, the
> summary, the license and the list of files put in the gem.

**Question 2:** your gem is at `2.3.1`. You add a method without breaking anything.
What is the new version?

> **Answer**
>
> `2.4.0`: a compatible new feature bumps the MINOR, and the PATCH goes back to
> `0`.

**Question 3:** write the code that prints the name of the `.gem` file of a gem
`weather` at version `0.2.0`.

> **Answer**
>
> ```ruby
> spec = Gem::Specification.new do |s|
>   s.name = "weather"
>   s.version = "0.2.0"
> end
> puts spec.file_name
> ```
>
> ```sortie
> weather-0.2.0.gem
> ```

## Summary

- A gem is code in `lib/` (a module with the same name) and a gemspec, its
  label.
- `bundle gem name` creates the skeleton; the version lives in `lib/name/version.rb`.
- Test with Minitest (`bundle exec rake test`) before building.
- `gem build` makes the `.gem`, `gem push` publishes it on rubygems.org.
- Versions follow MAJOR.MINOR.PATCH; `~> 1.2` accepts everything up to `2.0`
  excluded.
- `gem "name", path: "…"` uses a local gem with Bundler.

Next: [step-by-step projects](projets.md)
