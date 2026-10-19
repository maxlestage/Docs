# Drawing in the browser with canvas

This page shows how to draw shapes and animate a ball in a web page with the `<canvas>` element, keeping the drawing (in the browser) apart from the calculations (testable with Node).

> **Goals:**
>
> - you will be able to put a `<canvas>` in a page and get its "brush";
> - you will understand canvas coordinates, where the `y` axis goes down;
> - you will be able to draw rectangles, circles, lines and text;
> - you will be able to compute positions with pure functions, tested in Node;
> - you will be able to animate a bouncing ball, one step at a time.

## A canvas of pixels

A `<canvas>` is an empty rectangle in the page. You draw on it with JavaScript, pixel
by pixel. It is like a sheet of graph paper: each small square is a **pixel**, a dot of
color on the screen.

The canvas is not part of the [DOM](../debuter/glossaire.md#dom) like a heading or a
button: what you draw on it is not an element. It is paint. Once it is on, it stays
there, until you paint over it or erase it.

```html norun
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>My canvas</title>
  </head>
  <body>
    <canvas id="canvas" width="300" height="150"></canvas>
    <script type="module" src="drawing.js"></script>
  </body>
</html>
```

- `width="300"`: the canvas is 300 pixels wide;
- `height="150"`: and 150 pixels tall;
- the script `drawing.js` will do the drawing (see [the DOM](dom.md) to link a script).

To draw, you ask the canvas for its **2D context**: that is the brush, the object that
knows how to draw shapes.

```javascript norun
// drawing.js
const canvas = document.querySelector("#canvas");
const ctx = canvas.getContext("2d");   // the brush

ctx.fillStyle = "tomato";              // the fill color
ctx.fillRect(10, 20, 100, 50);         // a filled rectangle
```

This brush is almost always called `ctx` (for *context*).

> **Remember:** `<canvas>` is an empty canvas; `getContext("2d")` gives you the brush
> `ctx`, and the brush is what draws.

## Coordinates: y goes down

To say where to draw, you give two numbers: `x` and `y`.

- `x` counts pixels **to the right**, from the left edge;
- `y` counts pixels **downward**, from the top edge.

So the point `(0, 0)` is at the **top left**. It is the opposite of school math, where
`y` goes up. Think of a book page: you read from top to bottom, line 1 is at the top.

```text
 (0,0) ─────── x increases ──────▶
   │ ┌──────────────────────────┐
   │ │                          │
   │ │    ■ (60,30)             │
   y │                          │
   │ │               ● (240,110)│
   │ │                          │
   ▼ └──────────────────────────┘
  y increases           (300,150) = bottom right
```

The point `(240, 110)` is **lower** than `(60, 30)`, because its `y` is larger.

| Point | Where in the 300 × 150 canvas |
|---|---|
| `(0, 0)` | top left corner |
| `(300, 0)` | top right corner |
| `(0, 150)` | bottom left corner |
| `(150, 75)` | the center |

If you have coordinates "the right way up", as in math, a small function flips them.
It does not touch the browser: Node can run it.

```javascript
function toCanvas(x, y, height) {
  return { x, y: height - y };
}

console.log(toCanvas(0, 0, 150));    // bottom left in math
console.log(toCanvas(0, 150, 150));  // top left in math
console.log(toCanvas(100, 40, 150));
```

```sortie
{ x: 0, y: 150 }
{ x: 0, y: 0 }
{ x: 100, y: 110 }
```

| Call | `height - y` | `y` in the canvas |
|---|---|---|
| `toCanvas(0, 0, 150)` | `150 - 0` | `150` (bottom) |
| `toCanvas(0, 150, 150)` | `150 - 150` | `0` (top) |
| `toCanvas(100, 40, 150)` | `150 - 40` | `110` |

Try it: call `toCanvas(150, 75, 150)`. The center stays the center:
`{ x: 150, y: 75 }`.

> **Remember:** `(0, 0)` is at the top left. The larger `y` is, the **lower** the point
> is.

## Drawing shapes

Here are the main brush commands. This code only works in a browser (`document` does
not exist in Node).

### Rectangles

```javascript norun
ctx.fillStyle = "steelblue";
ctx.fillRect(10, 10, 80, 40);     // filled: x, y, width, height

ctx.strokeStyle = "black";
ctx.lineWidth = 3;
ctx.strokeRect(110, 10, 80, 40);  // only the outline

ctx.clearRect(0, 0, 300, 150);    // erases the whole canvas
```

`x` and `y` are the **top left** corner of the rectangle. The width goes to the right,
the height goes **down**.

```text
 (10,10)
    ┌────────── 80 ──────────┐
    │                        │ 40
    └────────────────────────┘
                         (90,50)
```

### Lines and paths

A line is drawn in three steps, as with a pencil: you lift the pencil, you put it down,
you draw.

```javascript norun
ctx.beginPath();          // new drawing: forget the previous one
ctx.moveTo(20, 130);      // put the pencil down (without drawing)
ctx.lineTo(150, 20);      // draw up to here
ctx.lineTo(280, 130);     // then up to here
ctx.closePath();          // go back to the starting point
ctx.stroke();             // draw the outline (a triangle)
```

### Circles

A circle is an **arc** that goes all the way around: from angle `0` to angle
`2 * Math.PI` (angles are in radians; `2 * Math.PI` is one full turn).

```javascript norun
ctx.beginPath();
ctx.arc(150, 75, 30, 0, 2 * Math.PI);  // center (150, 75), radius 30
ctx.fillStyle = "gold";
ctx.fill();
```

Warning: for a circle, `(150, 75)` is the **center**, not the corner.

### Text

```javascript norun
ctx.font = "20px sans-serif";
ctx.fillStyle = "black";
ctx.fillText("Hello", 10, 140);  // x, y of the text baseline
```

| Command | What it does |
|---|---|
| `fillRect(x, y, w, h)` | a filled rectangle |
| `strokeRect(x, y, w, h)` | the outline of a rectangle |
| `clearRect(x, y, w, h)` | erases an area |
| `beginPath()` | starts a new path |
| `moveTo(x, y)` / `lineTo(x, y)` | moves the pencil / draws a line |
| `arc(x, y, r, start, end)` | an arc or a circle |
| `fill()` / `stroke()` | fills / draws the path |
| `fillText(text, x, y)` | writes a text |

> **Remember:** you choose the color first (`fillStyle`, `strokeStyle`), then you draw.
> For lines and circles: `beginPath`, the path, then `fill` or `stroke`.

## Seeing a drawing without a browser

To understand what a drawing does, you don't need to open a browser. You can build a
**canvas made of characters**: a grid of dots `.`, where you write `#` in place of a
painted pixel. Each character plays the role of a pixel.

```javascript
class TextCanvas {
  constructor(width, height) {
    this.width = width;
    this.height = height;
    // one row per value of y, one cell per value of x
    this.rows = Array.from({ length: height }, () => Array(width).fill("."));
  }

  point(x, y, char = "#") {
    if (x >= 0 && x < this.width && y >= 0 && y < this.height) {
      this.rows[y][x] = char;
    }
  }

  fillRect(x, y, w, h, char = "#") {
    for (let dy = 0; dy < h; dy++) {
      for (let dx = 0; dx < w; dx++) {
        this.point(x + dx, y + dy, char);
      }
    }
  }

  show() {
    console.log(this.rows.map((row) => row.join("")).join("\n"));
  }
}

const canvas = new TextCanvas(12, 5);
canvas.point(0, 0, "O");       // the origin
canvas.fillRect(3, 1, 4, 2);   // a 4 by 2 rectangle
canvas.point(11, 4, "X");      // the last pixel
canvas.show();
```

```sortie
O...........
...####.....
...####.....
............
...........X
```

The `O` is indeed at the top left, and the `X` at the bottom right. The rectangle
starts at `x = 3`, `y = 1`, and goes down.

Let's follow `fillRect(3, 1, 4, 2)` step by step. The `dy` loop (the rows) contains
the `dx` loop (the columns):

| Step | dy | dx | Painted cell `(x + dx, y + dy)` |
|---|---|---|---|
| 1 | 0 | 0 | (3, 1) |
| 2 | 0 | 1 | (4, 1) |
| 3 | 0 | 2 | (5, 1) |
| 4 | 0 | 3 | (6, 1) |
| 5 | 1 | 0 | (3, 2) |
| 6 | 1 | 1 | (4, 2) |
| 7 | 1 | 2 | (5, 2) |
| 8 | 1 | 3 | (6, 2) |

Eight cells: 4 wide times 2 tall.

Try it: replace `canvas.fillRect(3, 1, 4, 2)` with `canvas.fillRect(10, 3, 4, 4)`.
The rectangle goes past the edge of the canvas: only the cells inside are painted
(that is the job of the `if` in `point`). A real canvas does the same, without an error.

> **Remember:** a grid of characters is a "poor man's canvas". It lets you check the
> logic of a drawing with a simple `console.log`.

## Computing positions

The hardest part of a drawing is the math: where to put each shape. Put it in **pure
functions** (they take numbers, they return numbers). They work everywhere, and you can
check them in Node.

### Centering a rectangle

To center, you start from the middle of the canvas and step back by half the rectangle.

```javascript
function center(canvasWidth, canvasHeight, w, h) {
  return {
    x: (canvasWidth - w) / 2,
    y: (canvasHeight - h) / 2,
  };
}

console.log(center(300, 150, 100, 50));
console.log(center(12, 5, 4, 1));
```

```sortie
{ x: 100, y: 50 }
{ x: 4, y: 2 }
```

| Calculation | Value |
|---|---|
| `(300 - 100) / 2` | `100` |
| `(150 - 50) / 2` | `50` |

So the rectangle goes from `x = 100` to `x = 200`: there are 100 pixels left on each
side.

### A row of squares

You want 4 squares of 20 pixels, 10 pixels apart. Square number `i` starts at
`margin + i * (size + gap)`.

```javascript
function row(count, size, gap, margin) {
  const positions = [];
  for (let i = 0; i < count; i++) {
    positions.push(margin + i * (size + gap));
  }
  return positions;
}

console.log(row(4, 20, 10, 5));
```

```sortie
[ 5, 35, 65, 95 ]
```

| i | `i * (20 + 10)` | `+ 5` | x of the square |
|---|---|---|---|
| 0 | 0 | 5 | 5 |
| 1 | 30 | 35 | 35 |
| 2 | 60 | 65 | 65 |
| 3 | 90 | 95 | 95 |

In the browser, all that is left is drawing:

```javascript norun
for (const x of row(4, 20, 10, 5)) {
  ctx.fillRect(x, 10, 20, 20);
}
```

Try it: call `row(4, 20, 0, 5)`. The gap is zero: `[ 5, 25, 45, 65 ]`, the squares
touch and form a single strip.

> **Remember:** keep the calculations (pure, testable functions) apart from the drawing
> commands (`ctx.fillRect`…). The browser is only there to paint.

## Animating: one step at a time

A cartoon is a series of still pictures. To animate a ball, you repeat three things
over and over:

```text
 ┌──────────────────────────────────────────┐
 │ 1. erase the canvas         (clearRect)  │
 │ 2. compute the next position    (step)   │◀──┐
 │ 3. draw the ball            (arc, fill)  │   │
 └──────────────────────┬───────────────────┘   │
                        └── next frame ─────────┘
```

In the browser, `requestAnimationFrame` calls your function just before each new frame
of the screen (about 60 times per second):

```javascript norun
let ball = { x: 20, y: 20, vx: 3, vy: 2 };

function frame() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);     // 1. erase
  ball = step(ball, canvas.width, canvas.height);       // 2. compute
  ctx.beginPath();                                       // 3. draw
  ctx.arc(ball.x, ball.y, 5, 0, 2 * Math.PI);
  ctx.fill();
  requestAnimationFrame(frame);                          // and start again
}

requestAnimationFrame(frame);
```

### The step function

The ball has a position (`x`, `y`) and a **velocity** (`vx`, `vy`): how far it moves
at each frame. If the next move would take it out of the canvas, you flip the velocity:
it bounces, like a ping-pong ball against a wall.

```javascript
function step(ball, width, height) {
  let { x, y, vx, vy } = ball;
  if (x + vx < 0 || x + vx > width - 1) vx = -vx;    // left or right wall
  if (y + vy < 0 || y + vy > height - 1) vy = -vy;   // ceiling or floor
  return { x: x + vx, y: y + vy, vx, vy };
}

let ball = { x: 0, y: 0, vx: 3, vy: 1 };
for (let i = 1; i <= 7; i++) {
  ball = step(ball, 10, 3);
  console.log(i, ball.x, ball.y, ball.vx, ball.vy);
}
```

```sortie
1 3 1 3 1
2 6 2 3 1
3 9 1 3 -1
4 6 0 -3 -1
5 3 1 -3 1
6 0 2 -3 1
7 3 1 3 -1
```

The canvas is 10 cells wide (`x` from 0 to 9) and 3 tall (`y` from 0 to 2). Let's
follow each call:

| Step | x | y | vx | vy | What happens |
|---|---|---|---|---|---|
| start | 0 | 0 | 3 | 1 | |
| 1 | 3 | 1 | 3 | 1 | nothing special |
| 2 | 6 | 2 | 3 | 1 | the ball touches the floor (`y = 2`) |
| 3 | 9 | 1 | 3 | -1 | `2 + 1 = 3 > 2`: bounce off the floor, `vy` becomes `-1` |
| 4 | 6 | 0 | -3 | -1 | `9 + 3 = 12 > 9`: bounce off the right wall |
| 5 | 3 | 1 | -3 | 1 | `0 - 1 < 0`: bounce off the ceiling |
| 6 | 0 | 2 | -3 | 1 | the ball touches the left wall |
| 7 | 3 | 1 | 3 | -1 | two bounces at once: left wall and floor |

At each step, you check **before** moving. That way the ball never gets out.

### Seeing the path

On a canvas made of characters, you write the number of each step in place of the
ball:

```javascript
function step(ball, width, height) {
  let { x, y, vx, vy } = ball;
  if (x + vx < 0 || x + vx > width - 1) vx = -vx;
  if (y + vy < 0 || y + vy > height - 1) vy = -vy;
  return { x: x + vx, y: y + vy, vx, vy };
}

const rows = Array.from({ length: 3 }, () => Array(10).fill("."));
let ball = { x: 0, y: 0, vx: 3, vy: 1 };
rows[ball.y][ball.x] = "0";
for (let i = 1; i <= 6; i++) {
  ball = step(ball, 10, 3);
  rows[ball.y][ball.x] = String(i);
}
console.log(rows.map((r) => r.join("")).join("\n"));
```

```sortie
0.....4...
...5.....3
6.....2...
```

Read the digits in order: `0`, `2`, `3`… The `1` is missing: the cell of step 1 was
covered by the `5` (same position `(3, 1)`), like fresh paint hides the old paint.

Try it: start with `vx: 2`. The ball no longer bounces at the same place, and the
drawn path changes.

> **Remember:** an animation = erase, compute, draw, start again. The `step` function
> is pure: you can test it in Node, step by step.

## Reacting to a click

A click gives coordinates in the **window**. To know where it lands in the canvas, you
subtract the position of the canvas in the page.

```javascript norun
canvas.addEventListener("click", (event) => {
  const box = canvas.getBoundingClientRect();  // where the canvas is on screen
  const x = event.clientX - box.left;
  const y = event.clientY - box.top;
  console.log(clickedCell(x, y, 20));
});
```

Finding the cell of a checkerboard is a pure calculation: you divide by the size of a
cell, and keep the whole part with `Math.floor`.

```javascript
function clickedCell(x, y, size) {
  return { column: Math.floor(x / size), row: Math.floor(y / size) };
}

console.log(clickedCell(5, 5, 20));
console.log(clickedCell(45, 12, 20));
console.log(clickedCell(60, 99, 20));
```

```sortie
{ column: 0, row: 0 }
{ column: 2, row: 0 }
{ column: 3, row: 4 }
```

| Click | `x / 20` | `y / 20` | Cell |
|---|---|---|---|
| (5, 5) | 0.25 → 0 | 0.25 → 0 | column 0, row 0 |
| (45, 12) | 2.25 → 2 | 0.6 → 0 | column 2, row 0 |
| (60, 99) | 3 → 3 | 4.95 → 4 | column 3, row 4 |

> **Remember:** first convert the click to canvas coordinates, then let a pure function
> do the rest.

## Common mistakes

### The canvas cannot be found

```javascript norun
const canvas = document.querySelector("#canavs");
const ctx = canvas.getContext("2d");
```

```text
Uncaught TypeError: Cannot read properties of null (reading 'getContext')
```

`querySelector` found no element with `id="canavs"` (a typo) and returned `null`.
Check the `id` in the HTML, and load the script after the canvas (`type="module"`).

### A misnamed context

```javascript norun
const ctx = canvas.getContext("2D");
ctx.fillRect(0, 0, 10, 10);
```

```text
Uncaught TypeError: Cannot read properties of null (reading 'fillRect')
```

The context name is written in lowercase: `"2d"`. With an unknown name, `getContext`
returns `null`, without an error. The error only comes on the next line.

### Forgetting beginPath

```javascript norun
ctx.arc(50, 50, 10, 0, 2 * Math.PI);
ctx.fill();
ctx.arc(150, 50, 10, 0, 2 * Math.PI);   // no beginPath before
ctx.fill();
```

There is no error message. But a line joins the two circles and the shape looks odd:
the second `arc` was added to the **same** path. Start each shape with
`ctx.beginPath()`.

### Enlarging the canvas with CSS

```html norun
<canvas id="canvas" style="width: 600px; height: 300px"></canvas>
```

No error here either, but the drawing is blurry and stretched. Without `width` and
`height`, the canvas is 300 × 150 pixels; the CSS stretches it like an enlarged photo.
Give the size with the attributes `width="600" height="300"`.

## Test yourself

**Question 1:** in a 300 × 150 canvas, where is the point `(0, 150)`?

> **Answer**
>
> At the bottom left. `x = 0` is the left edge, and `y = 150` is at the very bottom,
> because `y` goes down.

**Question 2:** a ball is at `x = 8` with `vx = 3`, in a canvas 10 wide (`x` from 0
to 9). What does `step` give for `x` and `vx`?

> **Answer**
>
> `8 + 3 = 11`, larger than `9`: the ball bounces. `vx` becomes `-3`, and `x` becomes
> `8 - 3 = 5`.

**Question 3:** write a function `column(x, size, gap, margin)` that gives the
horizontal position of column number `x` in a grid, then print the first 3 for a size
of 10, a gap of 2 and a margin of 4.

> **Answer**
>
> ```javascript
> function column(x, size, gap, margin) {
>   return margin + x * (size + gap);
> }
>
> console.log([0, 1, 2].map((x) => column(x, 10, 2, 4)));
> ```
>
> ```sortie
> [ 4, 16, 28 ]
> ```

## Summary

- `<canvas width height>` is a canvas; `getContext("2d")` gives the brush `ctx`.
- `(0, 0)` is at the top left, and `y` increases downward.
- `fillRect`, `arc`, `lineTo`, `fillText` draw; `beginPath` starts a shape.
- Put the calculations in pure functions and check them in Node, if needed with a
  canvas made of characters.
- An animation: erase, compute (`step`), draw, with `requestAnimationFrame`.

Next: [keeping data in the browser](stockage.md)
