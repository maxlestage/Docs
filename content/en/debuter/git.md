# Saving your work with Git

Git keeps the history of your code and lets you go back in time: this page teaches you how to install it, save your changes and send them to GitHub.

> **Goals:**
>
> - understand what Git is for;
> - create a repository with `git init`;
> - save a version with `git add` and `git commit`;
> - see the status and the history with `git status` and `git log`;
> - ignore files with `.gitignore`;
> - send your code to GitHub and get it back somewhere else.

## Why Git

You may have already seen folders like these:

```text
project.rb
project_v2.rb
project_v2_final.rb
project_v2_final_REALLY.rb
```

**Git** does this job for you, cleanly. It is a **version control**
program:

- it keeps **every saved version** of your code, with the date and a
  message;
- it lets you **go back** if you broke everything;
- it lets you send your code **to the Internet** (GitHub), to back it up,
  show it or work with other people.

A folder tracked by Git is called a **repository** (often shortened to
*repo*). A saved version is called a **commit**.

> **Remember:** a commit is a snapshot of your project at a given moment.
> Git keeps the whole photo album.

## Installing Git

### On Windows

Download Git from the `git-scm.com` website and run the installer.
Accept the suggested choices.

### On macOS

Type `git --version` in the terminal. If Git is not installed, macOS
offers to install it: accept.

### On Linux

Install the `git` package with your system's package manager, for
example on Ubuntu or Debian:

```sh
sudo apt install git
```

### Checking and introducing yourself

Check the installation:

```sh
git --version
```

```text
git version 2.43.0
```

Then tell Git who you are, once and for all. This name and this
email address will appear in every commit:

```sh
git config --global user.name "Alice Martin"
git config --global user.email "alice@example.com"
```

## Creating a repository: git init

Go to your project's folder (see
[the terminal](terminal.md)), then type:

```sh
cd code
git init
```

```text
Initialized empty Git repository in /home/alice/code/.git/
```

Git created a hidden `.git` folder. That is where it stores the whole history.
Don't touch it.

## Seeing the status: git status

`git status` tells you what changed since the last commit. Create a file
`hello.rb`, then:

```sh
git status
```

```text
On branch main

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	hello.rb

nothing added to commit but untracked files present (use "git add" to track)
```

*Untracked files*: Git sees the file, but does not track it yet.

> **Tip:** type `git status` before and after each command. It always
> tells you where you are, and often what to do next.

## Preparing: git add

`git add` chooses the files to put in the next commit. It is like
putting things in a box before closing it.

```sh
git add hello.rb
```

To add all the changed files in the folder:

```sh
git add .
```

`git status` now shows the file ready:

```text
Changes to be committed:
  (use "git rm --cached <file>..." to unstage)
	new file:   hello.rb
```

## Saving: git commit

`git commit` closes the box and labels it. The message, after `-m`, says
what you did:

```sh
git commit -m "First program"
```

```text
[main (root-commit) 499728c] First program
 1 file changed, 1 insertion(+)
 create mode 100644 hello.rb
```

`499728c` is the start of the commit's **identifier**: its unique number.

Write short, clear messages that say **what changes**:
"Add the average calculation", "Fix the division by zero".
Avoid "changes" or "it works".

## Seeing the history: git log

`git log` lists the commits, from newest to oldest:

```sh
git log
```

```text
commit 499728c1890cc53a9d89212eaccef8917f0d2eda
Author: Alice <alice@example.com>
Date:   Sun Oct 18 19:55:35 2026 +0000

    First program
```

For one line per commit:

```sh
git log --oneline
```

```text
499728c First program
```

To leave a long history, press **q**.

## Ignoring files: .gitignore

Some files must not go into the repository: files created by the
compiler, passwords, system files. Write their names in a `.gitignore`
file, at the root of the project:

```text
# What Rust compiles (cargo build)
target/
# macOS files
.DS_Store
# Secrets
.env
```

Git won't see these files anymore. Add the `.gitignore` itself to the repository:

```sh
git add .gitignore
git commit -m "Add the .gitignore"
```

> **Note:** `cargo new` already creates a `.gitignore` for your Rust projects.

> **Warning:** never put a password in a commit. Even if you delete it
> later, it stays in the history.

## GitHub: backing up online

For now, your repository is **on your computer**. **GitHub** is a
website that hosts Git repositories online. There are others, like
GitLab or Codeberg; they work the same way.

### Creating a repository on GitHub

1. Create a free account on `github.com`.
2. Click **+** at the top right, then **New repository**.
3. Give it a name, for example `code`.
4. Leave it **empty**: don't tick "Add a README".
5. Click **Create repository**.

GitHub then shows the repository's address, for example
`https://github.com/alice/code.git`.

### Sending your code: git push

Tell your local repository where the online repository is. The name `origin`
is the convention:

```sh
git remote add origin https://github.com/alice/code.git
```

Then send your commits:

```sh
git push -u origin main
```

The first time, GitHub asks you to log in. After that, a simple
`git push` is enough. Reload the GitHub page: your code is online.

### Getting a repository: git clone

On another computer, or to get someone else's project,
`git clone` downloads the whole repository, with its history:

```sh
git clone https://github.com/alice/code.git
cd code
```

If the repository has changed online since then, `git pull` gets the new
commits.

## The everyday routine

Once the repository is created, daily work fits in a few commands:

| When                          | Command                     |
|-------------------------------|-----------------------------|
| When you start                | `git pull`                  |
| To see where you are          | `git status`                |
| After a small step that works | `git add .`                 |
| To save that step             | `git commit -m "Message"`   |
| To see the history            | `git log --oneline`         |
| When you leave                | `git push`                  |

```sh
git pull
git status
git add .
git commit -m "Add the average function"
git push
```

> **Tip:** make small commits, often: every time
> something works. If you break everything afterwards, you can go back to a
> version that worked.

## Common mistakes

### Running git outside a repository

```sh
git status
```

```text
fatal: not a git repository (or any of the parent directories): .git
```

You are not in the project's folder, or you haven't run
`git init`. Check with `pwd`, then run `cd code` or `git init`.

### Committing without git add

```sh
git commit -m "First program"
```

```text
On branch main

Initial commit

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	hello.rb

nothing added to commit but untracked files present (use "git add" to track)
```

The box is empty. First run `git add hello.rb` (or `git add .`).

### Forgetting to introduce yourself

```sh
git commit -m "First program"
```

```text
Author identity unknown

*** Please tell me who you are.

Run

  git config --global user.email "you@example.com"
  git config --global user.name "Your Name"

to set your account's default identity.
Omit --global to set the identity only in this repository.
```

Type the two `git config` commands with your name and your email address, then
commit again.

### Pushing without an online repository

```sh
git push
```

```text
fatal: No configured push destination.
Either specify the URL from the command-line or configure a remote repository using

    git remote add <name> <url>

and then push using the remote name

    git push <name>
```

Git doesn't know where to send. Add the address with
`git remote add origin …`, then `git push -u origin main`.

## Test yourself

**1.** What is the difference between `git add` and `git commit`?

> **Answer**
>
> `git add` chooses the changes to save (you fill the box).
> `git commit` saves them for good, with a message (you close and
> label the box).

**2.** Your Rust project contains a very heavy `target/` folder. How do
you avoid sending it to GitHub?

> **Answer**
>
> Write `target/` in the `.gitignore` file, at the root of the project.

**3.** You change computers. How do you get your project back from
GitHub?

> **Answer**
>
> With `git clone` followed by the repository's address, for example
> `git clone https://github.com/alice/code.git`.

## Summary

- Git keeps every version of your code; a tracked folder is a repository, a version is a commit.
- `git init` creates the repository, only once.
- `git add` prepares, `git commit -m "…"` saves, `git status` and `git log` show where you are.
- `.gitignore` lists the files not to track.
- `git push` sends to GitHub, `git clone` gets a repository, `git pull` updates it.
- Make small commits, often.

Next: [How the web works](web.md)
