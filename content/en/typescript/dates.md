# Dates and times

This page shows how to create, read, calculate and display dates in TypeScript, while avoiding the month trap and the time zone trap.

> **Goals:**
>
> - you will know what a `Date` object is and how to create a precise date;
> - you will know how to avoid the trap of months numbered from 0;
> - you will know how to work in UTC to get the same result everywhere;
> - you will know how to display a date in English with `Intl.DateTimeFormat`;
> - you will know how to calculate a gap in days and add days;
> - you will know how to put these calculations in well-typed functions.

TypeScript uses the JavaScript `Date` object: everything in
[the JavaScript chapter on dates](../javascript/dates.md) is still true here. This
page goes over the essentials, step by step, and shows what **types** add.

## A date is a number

For the computer, a date is **a single number**: the milliseconds elapsed since
January 1, 1970 at midnight (universal time). Imagine a stopwatch started on that day
and never stopped. A date is what the stopwatch showed at a given moment.

```text
   Jan 1, 1970                                     March 5, 2026, 14:30
   00:00 UTC
       │─────────────── 1 772 721 000 000 ms ─────────────────│
       0                                                  getTime()
```

```typescript
const zero: Date = new Date(0);
console.log(zero.toISOString());

const appointment: Date = new Date(Date.UTC(2026, 2, 5, 14, 30));
const milliseconds: number = appointment.getTime();
console.log(milliseconds);
console.log(appointment.toISOString());
```

```sortie
1970-01-01T00:00:00.000Z
1772721000000
2026-03-05T14:30:00.000Z
```

- `Date` is a **type**: a variable of type `Date` holds a date object;
- `getTime()` returns the stopwatch number, of type `number`;
- `toISOString()` writes the date in **ISO 8601** format: year-month-day, a `T`,
  the time, and `Z`, which means "universal time".

> **Note:** `new Date()` **without arguments** gives the current moment. That's useful in
> a real program, but the result changes on every run. So all the examples on this
> page use fixed dates.

> **Remember:** a `Date` object wraps a number of milliseconds. Everything else
> (day, month, hour) is calculated from that number.

## Creating a precise date

### With Date.UTC

`Date.UTC(year, month, day, hour, minute)` calculates the stopwatch number for a date
given in universal time. You then pass it to `new Date`.

```typescript
const christmas = new Date(Date.UTC(2026, 11, 25));
const noon = new Date(Date.UTC(2026, 6, 14, 12, 0));
console.log(christmas.toISOString());
console.log(noon.toISOString());
```

```sortie
2026-12-25T00:00:00.000Z
2026-07-14T12:00:00.000Z
```

Look closely: Christmas is written with `11`, and July 14 with `6`. This is not a
typo.

### The month trap

In `Date.UTC` and `new Date(year, month, …)`, **months start at 0**. January is 0,
December is 11. Days, on the other hand, do start at 1.

It's like the slots of an egg carton numbered from 0: the first slot is slot 0.

```text
   real month:  Jan   Feb   Mar   Apr  May  Jun   Jul   Aug   Sep  Oct  Nov  Dec
   JS number:     0     1     2    3    4     5     6     7     8    9   10   11
```

```typescript
const months = [0, 1, 11, 12];
for (const m of months) {
  const d = new Date(Date.UTC(2026, m, 1));
  console.log(`number ${m} -> ${d.toISOString().slice(0, 10)}`);
}
```

```sortie
number 0 -> 2026-01-01
number 1 -> 2026-02-01
number 11 -> 2026-12-01
number 12 -> 2027-01-01
```

| Step | `m` | Real month | Resulting date |
|------|-----|------------|----------------|
| 1 | 0 | January | 2026-01-01 |
| 2 | 1 | February | 2026-02-01 |
| 3 | 11 | December | 2026-12-01 |
| 4 | 12 | (there is no 13th month) | 2027-01-01 |

Number 12 causes **no error**: the date overflows into January of the following year.
TypeScript can't protect you here, because `12` is a valid `number`.

Try it: replace `12` with `13`. You get `2027-02-01`.

> **Tip:** when you have a "human" month (1 to 12), always write `month - 1` when you
> create the date, and `+ 1` when you read it.

### From an ISO text

A text in ISO format is read directly, with **normal** months (03 = March). End it
with `Z` to say "universal time":

```typescript
const d = new Date("2026-03-05T14:30:00Z");
console.log(d.toISOString());
console.log(d.getTime() === Date.UTC(2026, 2, 5, 14, 30));
```

```sortie
2026-03-05T14:30:00.000Z
true
```

> **Warning:** a text in another format, like `"05/03/2026"`, gives an **invalid** date
> (or worse, a wrong date). Use the ISO format, or split the text yourself (see
> [regular expressions](regex.md#returning-t-or-null)).

> **Remember:** with numbers, months go from 0 to 11. With an ISO text, they go from
> 01 to 12.

## UTC or local time

**UTC** is the world reference time, the same for everyone, with no daylight saving
time. **Local time** depends on where the program runs: in Paris, it is one or two
hours ahead of UTC.

```text
   the same moment, seen from three cities

   UTC         ──────●──────  14:30
   Paris       ──────●──────  15:30
   New York    ──────●──────  09:30
```

So the `Date` object has **two families** of methods:

| Local time (avoid here) | UTC (used on this page) |
|-------------------------|-------------------------|
| `getHours()` | `getUTCHours()` |
| `getDate()` | `getUTCDate()` |
| `setDate()` | `setUTCDate()` |
| `new Date(2026, 2, 5)` | `new Date(Date.UTC(2026, 2, 5))` |

The "local" methods give a different result depending on the computer. A program that
works in Paris can be off by one day on a server in Tokyo. Always use the **UTC**
family to calculate, and only switch to a specific time zone when you **display**.

> **Remember:** calculate in UTC (`getUTC…`, `setUTC…`, `Date.UTC`). The result is
> then the same on every machine.

## Reading the parts of a date

Each `getUTC…` method returns a `number`:

```typescript
const d = new Date(Date.UTC(2026, 2, 5, 14, 30));

console.log("year:", d.getUTCFullYear());
console.log("month:", d.getUTCMonth(), "(so March)");
console.log("day:", d.getUTCDate());
console.log("hour:", d.getUTCHours());
console.log("minute:", d.getUTCMinutes());
console.log("day of the week:", d.getUTCDay());
```

```sortie
year: 2026
month: 2 (so March)
day: 5
hour: 14
minute: 30
day of the week: 4
```

`getUTCDay()` gives the day of the week: **0 for Sunday**, 1 for Monday… and 4 for
Thursday. March 5, 2026 is a Thursday.

To get the name of the day, an array is enough:

```typescript
const DAYS = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"] as const;

const d = new Date(Date.UTC(2026, 2, 5));
const name = DAYS[d.getUTCDay()];
console.log(name);
```

```sortie
Thursday
```

Try it: replace the `5` with `8`. You get `Sunday`.

> **Remember:** `getUTCMonth()` returns 0 to 11, `getUTCDay()` returns 0 (Sunday)
> to 6 (Saturday).

## Displaying a date in English

`Intl.DateTimeFormat` formats a date for a given language. Pass `"en-US"` for the
English of the United States, and **always** `timeZone: "UTC"` to get the same display
everywhere.

```typescript
const d = new Date(Date.UTC(2026, 2, 5, 14, 30));

const short = new Intl.DateTimeFormat("en-US", { timeZone: "UTC" });
const long = new Intl.DateTimeFormat("en-US", { timeZone: "UTC", dateStyle: "long" });
const full = new Intl.DateTimeFormat("en-US", {
  timeZone: "UTC",
  dateStyle: "full",
  timeStyle: "short",
});

console.log(short.format(d));
console.log(long.format(d));
console.log(full.format(d));
```

```sortie
3/5/2026
March 5, 2026
Thursday, March 5, 2026 at 2:30 PM
```

### Options checked by TypeScript

The options have a type: `Intl.DateTimeFormatOptions`. So TypeScript knows the allowed
values. A typo, like `dateStyle: "Long"`, is refused before the program even runs (see
the [common mistakes](#common-mistakes)).

You can store your settings in a typed variable and pick each part:

```typescript
const options: Intl.DateTimeFormatOptions = {
  timeZone: "UTC",
  weekday: "long",
  day: "numeric",
  month: "long",
};

const format = new Intl.DateTimeFormat("en-US", options);
console.log(format.format(new Date(Date.UTC(2026, 6, 14))));
console.log(format.format(new Date(Date.UTC(2026, 11, 25))));
```

```sortie
Tuesday, July 14
Friday, December 25
```

Try it: add `year: "numeric"` to the options. Both lines then end with `2026`.

> **Remember:** `Intl.DateTimeFormat("en-US", { timeZone: "UTC", … })` displays in
> English; the `Intl.DateTimeFormatOptions` type checks your options.

## Calculating with dates

### Comparing two dates

To compare, compare the stopwatch numbers, with `getTime()`:

```typescript
const departure = new Date(Date.UTC(2026, 0, 10));
const arrival = new Date(Date.UTC(2026, 0, 24));

console.log(departure.getTime() < arrival.getTime());
console.log(departure.getTime() === new Date("2026-01-10T00:00:00Z").getTime());
console.log(departure === new Date("2026-01-10T00:00:00Z"));
```

```sortie
true
true
false
```

The last line prints `false`: `===` compares two **objects**, not their contents.
They are two different boxes, even if they hold the same number.

### The gap in days

A day lasts 24 × 60 × 60 × 1000 = 86,400,000 milliseconds. For a gap in days, you
subtract the two numbers, then divide.

```typescript
const MS_PER_DAY = 24 * 60 * 60 * 1000;

const departure = new Date(Date.UTC(2026, 0, 10));
const arrival = new Date(Date.UTC(2026, 0, 24));

const gapMs = arrival.getTime() - departure.getTime();
const days = Math.round(gapMs / MS_PER_DAY);
console.log(gapMs);
console.log(`${days} days`);
```

```sortie
1209600000
14 days
```

| Step | Calculation | Value |
|------|-------------|-------|
| 1 | `departure.getTime()` | 1,768,003,200,000 |
| 2 | `arrival.getTime()` | 1,769,212,800,000 |
| 3 | `gapMs` = 2 − 1 | 1,209,600,000 |
| 4 | `gapMs / MS_PER_DAY` | 14 |

`Math.round` rounds to the nearest day: useful if the times are not exactly the same.

> **Note:** in JavaScript, you often see `arrival - departure`. TypeScript **refuses**
> it: you don't subtract two objects. Go through `getTime()` (see the
> [common mistakes](#common-mistakes)).

### Adding days

To add days, you **copy** the date, then change the day of the copy with
`setUTCDate`. If the day goes past the end of the month, the date moves on to the next
month by itself.

```typescript
const invoice = new Date(Date.UTC(2026, 0, 20));

const dueDate = new Date(invoice.getTime());
dueDate.setUTCDate(dueDate.getUTCDate() + 30);

console.log(invoice.toISOString().slice(0, 10));
console.log(dueDate.toISOString().slice(0, 10));
```

```sortie
2026-01-20
2026-02-19
```

| Step | Instruction | `dueDate` |
|------|-------------|-----------|
| 1 | copy of `invoice` | 2026-01-20 |
| 2 | `getUTCDate()` | 20 |
| 3 | `20 + 30` | 50 |
| 4 | `setUTCDate(50)`: January only has 31 days, 50 − 31 = 19 | 2026-02-19 |

Why copy? Because `setUTCDate` **modifies** the object. Without a copy, the invoice
date would change too. It's like writing on the original of a document instead of
writing on a photocopy.

Try it: replace `+ 30` with `- 25`. You get `2025-12-26`: the calculation even goes
back one year.

> **Remember:** compare and subtract with `getTime()`. To add days, copy the date and
> then use `setUTCDate`.

## Well-typed date functions

Date calculations are easy to get wrong. The right habit: write them **once**, in
small typed functions, and leave them alone. The rest of the program calls these
functions.

```typescript
const MS_PER_DAY = 24 * 60 * 60 * 1000;

// Reads "2026-03-05"; returns null if the text is not a real date.
function readDay(text: string): Date | null {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(text)) {
    return null;
  }
  const d = new Date(`${text}T00:00:00Z`);
  if (Number.isNaN(d.getTime()) || d.toISOString().slice(0, 10) !== text) {
    return null;
  }
  return d;
}

function addDays(date: Date, days: number): Date {
  const copy = new Date(date.getTime());
  copy.setUTCDate(copy.getUTCDate() + days);
  return copy;
}

function daysBetween(start: Date, end: Date): number {
  return Math.round((end.getTime() - start.getTime()) / MS_PER_DAY);
}

const inEnglish = new Intl.DateTimeFormat("en-US", { timeZone: "UTC", dateStyle: "long" });

function display(date: Date): string {
  return inEnglish.format(date);
}

for (const text of ["2026-01-20", "2026-02-30", "20/01/2026"]) {
  const day = readDay(text);
  if (day === null) {
    console.log(`${text}: invalid date`);
    continue;
  }
  const dueDate = addDays(day, 30);
  console.log(`${display(day)} -> pay before ${display(dueDate)} (${daysBetween(day, dueDate)} days)`);
}
```

```sortie
January 20, 2026 -> pay before February 19, 2026 (30 days)
2026-02-30: invalid date
20/01/2026: invalid date
```

What each function guarantees:

| Function | Receives | Returns | What it prevents |
|----------|----------|---------|------------------|
| `readDay` | `string` | `Date \| null` | badly formed texts and impossible dates |
| `addDays` | `Date`, `number` | a **new** `Date` | modifying the original |
| `daysBetween` | `Date`, `Date` | `number` | subtracting objects |
| `display` | `Date` | `string` | forgetting `timeZone: "UTC"` |

Why is `2026-02-30` refused? `new Date` accepts this text and quietly turns it into
March 2. `readDay` reads the resulting date back with `toISOString()`: since it's no
longer the same text, it returns `null`.

Thanks to the `Date | null` type, TypeScript **forces** the caller to handle the
invalid case before using the date (this is
[narrowing](../debuter/glossaire.md#retrecissement)).

Try it: delete the `if (day === null) { … }` block. TypeScript refuses the program:
`day` may be `null`.

> **Remember:** wrap date calculations in small typed functions. A function that reads
> a text returns `Date | null`.

## What about Temporal?

A new JavaScript API, **Temporal**, fixes the flaws of `Date`: months that start at 1,
objects that can't be modified and real time zones. It is not yet available in Node 22
nor in the TypeScript 5.8 types: in the meantime, use `Date` with the precautions on
this page.

## Common mistakes

### Subtracting two dates directly

```typescript norun
const departure = new Date(Date.UTC(2026, 0, 10));
const arrival = new Date(Date.UTC(2026, 0, 24));
const gap = arrival - departure;
console.log(gap);
```

```text
main.ts(3,13): error TS2362: The left-hand side of an arithmetic operation must be of type 'any', 'number', 'bigint' or an enum type.
main.ts(3,23): error TS2363: The right-hand side of an arithmetic operation must be of type 'any', 'number', 'bigint' or an enum type.
```

JavaScript accepts this subtraction, TypeScript doesn't: `arrival` and `departure` are
objects, not numbers. Write `arrival.getTime() - departure.getTime()`.

### Getting a display option wrong

```typescript norun
const format = new Intl.DateTimeFormat("en-US", { timeZone: "UTC", dateStyle: "Long" });
```

```text
main.ts(1,68): error TS2769: No overload matches this call.
  Overload 1 of 2, '(locales?: LocalesArgument, options?: DateTimeFormatOptions | undefined): DateTimeFormat', gave the following error.
    Type '"Long"' is not assignable to type '"long" | "short" | "medium" | "full" | undefined'. Did you mean '"long"'?
  Overload 2 of 2, '(locales?: string | string[] | undefined, options?: DateTimeFormatOptions | undefined): DateTimeFormat', gave the following error.
    Type '"Long"' is not assignable to type '"long" | "short" | "medium" | "full" | undefined'. Did you mean '"long"'?
```

The message is long, but the end says it all: `Did you mean '"long"'?`. Option values
are written in lowercase, exactly: `"short"`, `"medium"`, `"long"`, `"full"`.

### Passing a text instead of a date

```typescript norun
function addDays(date: Date, days: number): Date {
  const copy = new Date(date.getTime());
  copy.setUTCDate(copy.getUTCDate() + days);
  return copy;
}

console.log(addDays("2026-03-01", 7));
```

```text
main.ts(7,21): error TS2345: Argument of type 'string' is not assignable to parameter of type 'Date'.
```

A text is not a `Date`. Convert it first, and check the result:
`const d = readDay("2026-03-01"); if (d) { addDays(d, 7); }`.

### Using an invalid date

```typescript norun
const d = new Date("31/12/2026");
console.log(d.toISOString());
```

```text
RangeError: Invalid time value
```

This is **not** a TypeScript error: `new Date` always returns a `Date`, even an invalid
one. The error happens at run time. Use the ISO format (`"2026-12-31"`) and check with
`Number.isNaN(d.getTime())`, as in `readDay`.

## Test yourself

**Question 1:** what date does this program print?

```typescript
const d = new Date(Date.UTC(2026, 9, 1));
console.log(d.toISOString().slice(0, 10));
```

> **Answer**
>
> October 1st, because months start at 0: 9 is the tenth month.
>
> ```sortie
> 2026-10-01
> ```

**Question 2:** why use `getUTCDate()` rather than `getDate()`?

> **Answer**
>
> `getDate()` reads the date in the computer's time zone: the result can change from
> one machine to another. `getUTCDate()` gives the same result everywhere.

**Question 3:** write a function `isBefore(a: Date, b: Date): boolean` and test it with
March 1 and March 2, 2026.

> **Answer**
>
> ```typescript
> function isBefore(a: Date, b: Date): boolean {
>   return a.getTime() < b.getTime();
> }
>
> const first = new Date(Date.UTC(2026, 2, 1));
> const second = new Date(Date.UTC(2026, 2, 2));
> console.log(isBefore(first, second), isBefore(second, first));
> ```
>
> ```sortie
> true false
> ```

## Summary

- A `Date` object wraps a number of milliseconds since January 1, 1970 (UTC).
- With `Date.UTC` and `new Date(year, month, …)`, months go from **0 to 11**.
- Calculate in UTC (`getUTC…`, `setUTC…`) and display with
  `Intl.DateTimeFormat("en-US", { timeZone: "UTC" })`.
- TypeScript refuses `date1 - date2`: go through `getTime()`.
- To add days, copy the date and then use `setUTCDate`.
- Put these calculations in small typed functions, which return `Date | null` when
  reading can fail.

Next: [a typed command-line tool](ligne-de-commande.md).
