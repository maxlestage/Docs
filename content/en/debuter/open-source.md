# Contributing to an open source project

An open source project is software whose code is open to everyone: this page explains what that means and guides you, step by step, to your first contribution on GitHub.

> **Goals:**
>
> - understand what open source is and what a license says;
> - know how to help a project, even as a beginner;
> - read a project: README, CONTRIBUTING, issues;
> - make a complete contribution: fork, clone, branch, commit, push, pull request;
> - reply politely to review comments.

## What is open source?

Picture a recipe displayed in a bakery window. Anyone can read it, make it
at home, and even suggest an improvement to the baker.

An **open source** program (its source code is open) works the same way:

- its code is **public**: anyone can read it;
- anyone can **use** it and **change** it;
- anyone can **suggest changes** to the project.

You already use some without knowing it. Python, Node.js, Go, Ruby, Rust,
Git, Firefox and VLC are open source.

> **Remember:** open source means "the code is open". Anyone can read it,
> use it and suggest improvements.

## Licenses, in simple words

A **license** is a short text, usually in a `LICENSE` file. It says what
you are allowed to do with the code. Two licenses come up very often:

| License | In one sentence | What it asks |
|---|---|---|
| **MIT** | "Do whatever you want with this code." | Keep the author's name and the license text. |
| **GPL** | "You can do anything, but share in return." | If you distribute a modified version, its code must stay open, under the GPL. |

A picture for the GPL: someone lends you a recipe book, on the condition
that your new recipes are lent the same way.

There are others (Apache, BSD…): no need to learn them by heart.

> **Warning:** code **without a license** is not free to use, even if it is
> public on GitHub. By default, the author keeps all the rights.

## Why contribute, even as a beginner?

You may be thinking: "I'm not good enough." That's not true. A project
needs lots of things that are not complicated code:

- **fix a typo** in the documentation;
- **make an explanation clearer**, precisely because you didn't
  understand it the first time;
- **translate** a page into another language;
- **report a [bug](glossaire.md#bug)** properly, in an issue;
- **add an example** or a [test](glossaire.md#test).

And you gain from it: you read real code, you learn to work in a team
with [Git](glossaire.md#git), and you get advice from experienced people.

> **Remember:** a typo fixed in some documentation is a real contribution.
> Everyone starts there.

## Reading a project before touching it

A project on GitHub is a [repository](glossaire.md#repository). Before changing
anything, read three things.

### The README

The `README.md` file is shown on the repository's home page. It says what
the project is for, and how to install and run it.

### The CONTRIBUTING file

The `CONTRIBUTING.md` file is the project's **house rules**. It says how to
suggest a change: branch name, commit message, tests to run. Read it **in
full**: every project has its own habits.

### Issues

An **issue** (also called a "ticket") is a discussion page on GitHub: a
reported bug, an idea, a question. You find them in the repository's
**Issues** tab.

Welcoming projects put a **label** on easy tasks:

```text
good first issue      ← a good first task
help wanted           ← the project is looking for help
documentation         ← the task is about the documentation
```

Before you start, write under the issue "Hi! I'd like to work on this, is
it still available?". That way, two people don't do the same work.

> **Tip:** on GitHub, search for `label:"good first issue"` in the issues
> search bar to filter the easy tasks.

## The path of a contribution

Here is the whole journey, before doing it step by step.

```text
  THE PROJECT (github.com/project/tool)
      │                         ▲
      │ 1. fork                 │ 6. pull request
      ▼                         │
  YOUR COPY (github.com/alice/tool)
      │                         ▲
      │ 2. clone                │ 5. push
      ▼                         │
  YOUR COMPUTER: 3. branch → 4. commit
```

| Step | Word | In plain words |
|---|---|---|
| 1 | fork | Make **your copy** of the project, on GitHub. |
| 2 | clone | Download your copy to your computer. |
| 3 | branch | Open a **separate draft** for your change. |
| 4 | commit | Save your change (see [Git](git.md)). |
| 5 | push | Send the draft to your GitHub copy. |
| 6 | pull request | **Ask** the project to accept your change. |

In the examples, your name is `alice` and the project is called
`project/tool`.

## Step 1: make a fork

You are not allowed to write directly to the project's repository. So you
make a copy of your own: a **fork**.

1. Open the project's page on GitHub.
2. Click the **Fork** button, at the top right.
3. Click **Create fork**.

You now have `github.com/alice/tool`. This copy belongs to you.

## Step 2: clone your fork

Download **your** copy (not the project's) to your computer, in the
[terminal](terminal.md):

```sh norun
git clone https://github.com/alice/tool.git
cd tool
```

```text
Cloning into 'tool'...
```

For Git, this online copy is now called `origin`.

## Step 3: create a branch

A **branch** is a separate draft. The `main` branch stays clean; your work
goes on a new branch.

Give it a name that says what you are doing:

```sh norun
git checkout -b fix-readme-typo
```

```text
Switched to a new branch 'fix-readme-typo'
```

> **Remember:** one contribution = one branch. Never work directly on
> `main`.

## Step 4: make a change and a commit

Open the file in your [editor](editeur.md), make **a single** small
change, save. Then:

```sh norun
git status
git add README.md
git commit -m "Fix a typo in the README"
```

```text
[fix-readme-typo 3f2a1b7] Fix a typo in the README
 1 file changed, 1 insertion(+), 1 deletion(-)
```

If the project has tests, run them now. The CONTRIBUTING file says how.

## Step 5: push the branch

Send your branch to **your** fork, `origin`:

```sh norun
git push -u origin fix-readme-typo
```

`-u` is needed the first time: it tells Git where to send this branch.
After that, `git push` is enough.

GitHub replies with a ready-made link:

```text
remote:
remote: Create a pull request for 'fix-readme-typo' on GitHub by visiting:
remote:      https://github.com/alice/tool/pull/new/fix-readme-typo
remote:
To https://github.com/alice/tool.git
 * [new branch]      fix-readme-typo -> fix-readme-typo
branch 'fix-readme-typo' set up to track 'origin/fix-readme-typo'.
```

## Step 6: open the pull request

A **pull request** (**PR**) is a request: "Would you like to add my change?"

1. Open the link shown by `git push`, or your fork's page.
2. Click **Compare & pull request**.
3. Write a clear title and a short description.
4. Click **Create pull request**.

A good description answers three questions:

```text
Title: Fix a typo in the README

What: "installed" was written "instaled" in the Installation section.
Why: a typo gets in the way of reading.
Link: fixes issue #42.
```

> **Tip:** writing `Fixes #42` links the PR to the issue. GitHub will close
> the issue when the PR is accepted.

## Replying to the review

Someone from the project will **review** your PR. They can accept it (we
say **merge** it), ask a question or request changes.

A change request is **not** a rejection. It is a normal step, even for
experts.

To fix it, stay on the same branch, make the change, then:

```sh norun
git add README.md
git commit -m "Apply the review comment"
git push
```

The PR updates itself. Then reply to the comment:

```text
Thanks for the review! It's fixed in the latest commit.
```

> **Remember:** a comment is about the code, not about you. Fix, say
> thanks, push.

## Being polite and patient

Many projects are run by volunteers, in their free time.

- Say **hello** and **thank you**.
- Be **patient**: an answer can take several days, or weeks.
- After a week without news, a short friendly message is enough:
  "Hi, have you had time to take a look? Thanks!"
- If you don't understand a comment, **ask**. That's normal.
- Follow the **code of conduct** (the `CODE_OF_CONDUCT.md` file).

## First contribution ideas

From the easiest to the least easy:

| Idea | Where to look |
|---|---|
| Fix a typo | README, documentation |
| Make a sentence clearer | A page you found hard to understand |
| Translate a page into another language | A `docs/fr` or `translations` folder |
| Report a bug with the steps to reproduce it | Issues tab, **New issue** button |
| Add an example to the documentation | A function without an example |
| Solve a `good first issue` | Issues tab, filter by label |

> **Try it:** pick a tool you use. Open its repository, read the README and
> look for a `good first issue`. Do nothing else today: just read.

## Common mistakes

### Working directly on main

You make two changes on `main`, without a branch. They end up mixed in
the same PR, and the reviewer cannot accept one without the other.

Fix: before **each** contribution, create a branch from `main`:
`git checkout -b add-translation`.

### A rejected git push

The online repository has commits that you don't have on your computer
(for example, you changed a file directly on GitHub).

```sh norun
git push origin main
```

```text
To https://github.com/alice/tool.git
 ! [rejected]        main -> main (fetch first)
error: failed to push some refs to 'https://github.com/alice/tool.git'
hint: Updates were rejected because the remote contains work that you do not
hint: have locally. This is usually caused by another repository pushing to
hint: the same ref. If you want to integrate the remote changes, use
hint: 'git pull' before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
```

First get the online commits with `git pull`, then run `git push` again.

### A huge PR, without reading the CONTRIBUTING file

You change 40 files, the code style and three functions at once. Nobody
dares to review it: the PR stays open for months, or gets closed.

Fix:

- read `CONTRIBUTING.md` before you start;
- **one** change per PR, as small as possible;
- for a big idea, first open an issue to discuss it.

## Test yourself

**1.** What is the difference between a fork and a clone?

> **Answer**
>
> A fork is your copy of the project **on GitHub**. A clone is a copy **on
> your computer**. You make the fork first, then you clone that fork.

**2.** A reviewer asks for a change in your PR. Should you open a new PR?

> **Answer**
>
> No. Make the change on the same branch, commit and `git push`: the PR
> updates itself. Then say thanks and reply to the comment.

**3.** You are a beginner and don't know what to do. Name two possible
contributions.

> **Answer**
>
> Fix a typo in the documentation, translate a page, report a bug with
> the steps to reproduce it, or take an issue labeled `good first issue`.

## Summary

- Open source: the code is public, anyone can read it, use it and suggest changes.
- The license says what is allowed: MIT lets you do anything, GPL asks you to share in return.
- Read the README, the CONTRIBUTING file and the `good first issue` issues before you start.
- The path: fork, clone, branch, commit, push, pull request.
- A review comment is normal: fix it on the same branch, push, say thanks.
- Small PRs, one branch per contribution, and lots of politeness.

Next: [Careers in software development](metiers.md)
