# A web server with node:http

This page shows how to write a small web server with Node's `node:http` module: answer according to the method and the address, send JSON, choose the right status code and read the body of a request.

> **Goals:**
>
> - you will be able to create a server with `createServer` and test it from the same script;
> - you will be able to read the method, the address and the parameters of a request;
> - you will be able to organise routes by method and path;
> - you will be able to answer in JSON with the right status code;
> - you will be able to read the body of a `POST` request.

## A server is a ticket counter

A [server](../debuter/glossaire.md#server) is a program that waits for **requests** and
sends back **responses**, like a ticket counter at a train station. The
[client](../debuter/glossaire.md#client) (a browser, `fetch`, `curl`) comes up to the
counter and asks for something. The clerk reads the request, then answers.

```text
   client                                  server (your code)
 ┌─────────┐   request: GET /hello       ┌──────────────────────┐
 │  fetch  │ ──────────────────────────▶ │ (req, res) => { … }  │
 │         │ ◀────────────────────────── │                      │
 └─────────┘   response: 200 + "Hi"      └──────────────────────┘
```

The chapter [JavaScript for the web](web.md) introduced [HTTP](../debuter/glossaire.md#http),
`fetch` and a minimal server. Here, we build a real small server, step by step.

> **Remember:** a request comes in, your function reads it, and writes a response.

## The first server

A normal server runs **forever**, waiting for visits. So that the examples on this page
stop on their own, each one follows the same plan:

1. create the server;
2. start it on **port 0**: the system picks a free port;
3. call it with `fetch` from the same script;
4. print the responses;
5. close it with `close()`.

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

const server = createServer((req, res) => {
  res.writeHead(200, { "Content-Type": "text/plain; charset=utf-8" });
  res.end("Hello from Node!");
});

server.listen(0);                  // 0: a free port, at random
await once(server, "listening");   // wait until the server is ready
const address = `http://localhost:${server.address().port}`;

const response = await fetch(address);
console.log(response.status);
console.log(await response.text());

server.close();
```

```sortie
200
Hello from Node!
```

The port changes on every run: that is why we never print it.

| Step | Who acts | What happens |
|---|---|---|
| 1 | script | `createServer` prepares the counter (nothing is listening yet) |
| 2 | script | `listen(0)` opens the counter on a free port |
| 3 | script | `await once(...)` waits until the counter is open |
| 4 | client | `fetch(address)` sends `GET /` |
| 5 | server | the function `(req, res)` is called with the request |
| 6 | server | `writeHead(200, …)` chooses the status, `end(…)` sends the text |
| 7 | client | `response.status` is `200`, `response.text()` reads the body |
| 8 | script | `close()` closes the counter; the program ends |

The function given to `createServer` is called **once per request**. It receives two
objects:

- `req` (*request*): what the client asks for;
- `res` (*response*): the response form to fill in.

Try it: replace `200` with `418` in `writeHead`. `response.status` prints `418`
("I'm a teapot", a joke code, but a real one).

> **Note:** for a real server, write `server.listen(3000)` without `close()`, then open
> `http://localhost:3000` in the browser. Stop it with `Ctrl`+`C`.

> **Remember:** `createServer(function)` creates the server, `listen` opens it,
> `res.end` sends the response. Without `res.end`, the client waits forever.

## Reading the request

The `req` object contains:

- `req.method`: the method (`"GET"`, `"POST"`…);
- `req.url`: the path and the parameters, for example `"/books?page=2"`;
- `req.headers`: the headers, with lowercase names.

To split `req.url`, give it to `new URL` (see [the web](web.md)), with any base
address:

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

const server = createServer((req, res) => {
  const url = new URL(req.url, "http://localhost");
  const name = url.searchParams.get("name") ?? "unknown";
  res.end(`${req.method} ${url.pathname} name=${name}`);
});

server.listen(0);
await once(server, "listening");
const address = `http://localhost:${server.address().port}`;

console.log(await (await fetch(`${address}/hi?name=Lea`)).text());
console.log(await (await fetch(`${address}/hi`)).text());
console.log(await (await fetch(`${address}/`, { method: "DELETE" })).text());

server.close();
```

```sortie
GET /hi name=Lea
GET /hi name=unknown
DELETE / name=unknown
```

| Request sent | `req.method` | `url.pathname` | `searchParams.get("name")` |
|---|---|---|---|
| `/hi?name=Lea` | `GET` | `/hi` | `"Lea"` |
| `/hi` | `GET` | `/hi` | `null` → `"unknown"` |
| `/` with `DELETE` | `DELETE` | `/` | `null` → `"unknown"` |

`fetch` sends `GET` by default. The `method` option picks another one.

> **Remember:** `req.method` says **what to do**, `url.pathname` says **on what**,
> `url.searchParams` gives the small settings after the `?`.

## Routes: method + path

A **route** is a method and a path, linked to a function. Like the signs in a public
office: "Submit a file → counter 1", "Pick up → counter 2". The simplest way: glue the
two into a single key, then look it up in an object.

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

const routes = {
  "GET /": () => "Home",
  "GET /contact": () => "Write to us",
  "POST /contact": () => "Message received",
};

const server = createServer((req, res) => {
  const { pathname } = new URL(req.url, "http://localhost");
  const key = `${req.method} ${pathname}`;
  const action = routes[key];
  if (action) {
    res.writeHead(200, { "Content-Type": "text/plain; charset=utf-8" });
    res.end(action());
  } else {
    res.writeHead(404, { "Content-Type": "text/plain; charset=utf-8" });
    res.end("Not found");
  }
});

server.listen(0);
await once(server, "listening");
const address = `http://localhost:${server.address().port}`;

for (const [method, path] of [["GET", "/"], ["GET", "/contact"], ["POST", "/contact"], ["GET", "/price"]]) {
  const response = await fetch(address + path, { method: method });
  console.log(method, path, "→", response.status, await response.text());
}

server.close();
```

```sortie
GET / → 200 Home
GET /contact → 200 Write to us
POST /contact → 200 Message received
GET /price → 404 Not found
```

| Request | `key` | `routes[key]` | Status |
|---|---|---|---|
| 1 | `"GET /"` | found | 200 |
| 2 | `"GET /contact"` | found | 200 |
| 3 | `"POST /contact"` | found (another function!) | 200 |
| 4 | `"GET /price"` | `undefined` | 404 |

The same path `/contact` leads to two functions: the method makes the difference.

Try it: add the route `"GET /price": () => "Free"`. The last line becomes
`GET /price → 200 Free`.

> **Remember:** a route = method + path. If no route matches, answer 404.

## Answering in JSON

An API almost always answers in [JSON](../debuter/glossaire.md#json). `res.end` only
accepts text: you convert with `JSON.stringify`, and you tell the client with the
`Content-Type: application/json` header. A small function saves you from repeating
it:

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

function sendJson(res, status, data) {
  res.writeHead(status, { "Content-Type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(data));
}

const fruits = ["apple", "pear", "kiwi"];

const server = createServer((req, res) => {
  sendJson(res, 200, { total: fruits.length, fruits });
});

server.listen(0);
await once(server, "listening");

const response = await fetch(`http://localhost:${server.address().port}/fruits`);
console.log(response.headers.get("content-type"));
const data = await response.json();
console.log(data.total, data.fruits[2]);

server.close();
```

```sortie
application/json; charset=utf-8
3 kiwi
```

```text
 server  : { total: 3, fruits: [...] }   (object)
              │ JSON.stringify
              ▼
 network : '{"total":3,"fruits":["apple","pear","kiwi"]}'   (text)
              │ response.json()
              ▼
 client  : { total: 3, fruits: [...] }   (object)
```

> **Remember:** object → `JSON.stringify` → network → `response.json()` → object. And
> the `Content-Type: application/json` header.

## Status codes

The status code is the **first thing** the client reads. It is a three-digit number.
The first digit gives the family:

| Code | Name | When to use it |
|---|---|---|
| 200 | OK | all is well, here is the response |
| 201 | Created | a resource was created (after a `POST`) |
| 204 | No Content | done, nothing to send back (after a `DELETE`) |
| 400 | Bad Request | the request is malformed (invalid JSON, missing field) |
| 404 | Not Found | this address does not exist |
| 405 | Method Not Allowed | the path exists, but not with this method |
| 500 | Internal Server Error | the server crashed |

- `2xx`: success;
- `4xx`: the **client's** fault (it asked badly);
- `5xx`: the **server's** fault (your code).

> **Remember:** 2xx = it worked, 4xx = the client got it wrong, 5xx = the server got it
> wrong. Choosing the right code helps whoever uses your API.

## Reading the body of a request

With `POST`, the client sends data in the **body** of the request. It does not arrive
all at once: it arrives in **chunks**, like a long letter sent in several envelopes.
`req` is a stream that you walk through with `for await` (see
[asynchronous code](asynchrone.md)), you store each chunk in an array, and you glue them
back together at the end with `Buffer.concat`.

### The chunks step by step

A small body often arrives in a single chunk. To see the gluing, let's give the
`readBody` function a fake stream of three chunks, made with `Readable.from`:

```javascript
import { Readable } from "node:stream";

async function readBody(req) {
  const chunks = [];
  for await (const chunk of req) {
    chunks.push(chunk);
    console.log("received:", chunk.toString(), "| chunks:", chunks.length);
  }
  return Buffer.concat(chunks).toString("utf8");
}

const fakeReq = Readable.from([Buffer.from('{"tex'), Buffer.from('t":"br'), Buffer.from('ead"}')]);
const text = await readBody(fakeReq);
console.log(text);
console.log(JSON.parse(text).text);
```

```sortie
received: {"tex | chunks: 1
received: t":"br | chunks: 2
received: ead"} | chunks: 3
{"text":"bread"}
bread
```

| Round | Chunk received | What we have glued so far |
|---|---|---|
| 1 | `{"tex` | `{"tex` |
| 2 | `t":"br` | `{"text":"br` |
| 3 | `ead"}` | `{"text":"bread"}` |

No chunk alone is valid JSON: you must wait for **everything** before `JSON.parse`.

### A JSON echo

The server reads the body, converts it, and sends back what it understood. If the JSON
is invalid, it is the client's fault: 400.

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

async function readBody(req) {
  const chunks = [];
  for await (const chunk of req) chunks.push(chunk);
  return Buffer.concat(chunks).toString("utf8");
}

function sendJson(res, status, data) {
  res.writeHead(status, { "Content-Type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(data));
}

const server = createServer(async (req, res) => {
  let data;
  try {
    data = JSON.parse(await readBody(req));
  } catch {
    return sendJson(res, 400, { error: "invalid JSON" });
  }
  sendJson(res, 200, { received: data });
});

server.listen(0);
await once(server, "listening");
const address = `http://localhost:${server.address().port}`;

for (const body of ['{"age": 12}', "{age: 12}"]) {
  const response = await fetch(address, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: body,
  });
  console.log(response.status, await response.text());
}

server.close();
```

```sortie
200 {"received":{"age":12}}
400 {"error":"invalid JSON"}
```

The server's function is now `async`: it waits for the end of the body with `await`.

Try it: send `body: ""` (an empty body). `JSON.parse("")` fails too: the response is
`400 {"error":"invalid JSON"}`.

> **Remember:** the body arrives in chunks. Glue them back together, **then**
> `JSON.parse` inside a `try...catch`, and answer 400 if it fails.

## A small task API

Let's put it all together: a list of tasks kept in memory, with three routes.

| Route | Role | Possible statuses |
|---|---|---|
| `GET /tasks` | the list | 200 |
| `GET /tasks/:id` | one task | 200, 404 |
| `POST /tasks` | add a task | 201, 400 |

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

async function readBody(req) {
  const chunks = [];
  for await (const chunk of req) chunks.push(chunk);
  return Buffer.concat(chunks).toString("utf8");
}

function sendJson(res, status, data) {
  res.writeHead(status, { "Content-Type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(data));
}

const tasks = [{ id: 1, text: "bread" }];
let nextId = 2;

async function respond(req, res) {
  const { pathname } = new URL(req.url, "http://localhost");
  const [, resource, id] = pathname.split("/");   // "/tasks/1" → ["", "tasks", "1"]

  if (resource !== "tasks") return sendJson(res, 404, { error: "not found" });

  if (req.method === "GET" && id === undefined) {
    return sendJson(res, 200, tasks);
  }
  if (req.method === "GET") {
    const task = tasks.find((t) => t.id === Number(id));
    return task ? sendJson(res, 200, task) : sendJson(res, 404, { error: "not found" });
  }
  if (req.method === "POST" && id === undefined) {
    let body;
    try {
      body = JSON.parse(await readBody(req));
    } catch {
      return sendJson(res, 400, { error: "invalid JSON" });
    }
    if (typeof body.text !== "string" || body.text.trim() === "") {
      return sendJson(res, 400, { error: "missing text" });
    }
    const task = { id: nextId++, text: body.text.trim() };
    tasks.push(task);
    return sendJson(res, 201, task);
  }
  sendJson(res, 405, { error: "method not allowed" });
}

const server = createServer(respond);
server.listen(0);
await once(server, "listening");
const address = `http://localhost:${server.address().port}`;

async function call(method, path, body) {
  const options = { method: method };
  if (body !== undefined) options.body = JSON.stringify(body);
  const response = await fetch(address + path, options);
  console.log(method, path, "→", response.status, await response.text());
}

await call("GET", "/tasks");
await call("POST", "/tasks", { text: " milk " });
await call("POST", "/tasks", { title: "oops" });
await call("GET", "/tasks/2");
await call("GET", "/tasks/9");
await call("DELETE", "/tasks/1");
await call("GET", "/tasks");

server.close();
```

```sortie
GET /tasks → 200 [{"id":1,"text":"bread"}]
POST /tasks → 201 {"id":2,"text":"milk"}
POST /tasks → 400 {"error":"missing text"}
GET /tasks/2 → 200 {"id":2,"text":"milk"}
GET /tasks/9 → 404 {"error":"not found"}
DELETE /tasks/1 → 405 {"error":"method not allowed"}
GET /tasks → 200 [{"id":1,"text":"bread"},{"id":2,"text":"milk"}]
```

Let's follow the state of the server after each call:

| Call | `id` | Branch taken | Status | `tasks` after | `nextId` |
|---|---|---|---|---|---|
| `GET /tasks` | `undefined` | the list | 200 | bread | 2 |
| `POST` "milk" | `undefined` | add | 201 | bread, milk | 3 |
| `POST` without text | `undefined` | `text` missing | 400 | bread, milk | 3 |
| `GET /tasks/2` | `"2"` | one task, found | 200 | bread, milk | 3 |
| `GET /tasks/9` | `"9"` | one task, missing | 404 | bread, milk | 3 |
| `DELETE /tasks/1` | `"1"` | none: the end | 405 | bread, milk | 3 |

Careful: `id` comes from the address, so it is a **text** (`"2"`). You convert it with
`Number(id)` before comparing with `===`.

The data lives in a variable: it disappears when the server stops. To keep it, you need
a file (see [files](fichiers.md)) or a database.

Try it: add a branch for `DELETE` with an `id` that removes the task
(`tasks.splice(index, 1)`) and answers `204` with `res.writeHead(204); res.end();`.

> **Remember:** an API is routes, JSON, and an honest status code for every case,
> errors included.

## Common mistakes

### Forgetting res.end

```javascript norun
const server = createServer((req, res) => {
  res.writeHead(200);
  // no res.end()
});
// on the client side:
await fetch(address, { signal: AbortSignal.timeout(300) });
```

```text
TimeoutError: The operation was aborted due to timeout
```

The server never finished its response: the client waits. Without the `timeout`, it
would wait for a very long time. Every path of your function must end with `res.end`
(a `return` helps you not forget it).

### Sending an object without JSON.stringify

```javascript norun
res.writeHead(200, { "Content-Type": "application/json" });
res.end({ message: "Hello" });
```

```text
TypeError [ERR_INVALID_ARG_TYPE]: The "chunk" argument must be of type string or an instance of Buffer or Uint8Array. Received an instance of Object
    at write_ (node:_http_outgoing:950:11)
    at ServerResponse.end (node:_http_outgoing:1107:5)
```

`res.end` only accepts text (or bytes). Write
`res.end(JSON.stringify({ message: "Hello" }))`, or use `sendJson`.

### Answering twice

```javascript norun
const server = createServer((req, res) => {
  if (req.url === "/") {
    res.end("Home");
  }
  res.writeHead(404);
  res.end("Not found");
});
```

```text
Error [ERR_HTTP_HEADERS_SENT]: Cannot write headers after they are sent to the client
    at ServerResponse.writeHead (node:_http_server:354:11)
```

After `res.end("Home")`, the function **keeps going** and tries a second response. The
server crashes. Write `return res.end("Home");`, or use `else`.

## Test yourself

**Question 1:** why do the examples on this page use `listen(0)`?

> **Answer**
>
> Port `0` asks the system for a free port. The script does not risk the `EADDRINUSE`
> error (port already taken), and finds the real number with `server.address().port`.

**Question 2:** a client sends `POST /tasks` with the body `{text: bread}`. Which status
code should you send back?

> **Answer**
>
> `400 Bad Request`: the body is not valid JSON (the quotes are missing), it is the
> client's fault.

**Question 3:** write a server that answers the path `/double?n=21` with the JSON
`{"result":42}`, call it once, print the response and close it.

> **Answer**
>
> ```javascript
> import { createServer } from "node:http";
> import { once } from "node:events";
>
> const server = createServer((req, res) => {
>   const url = new URL(req.url, "http://localhost");
>   const n = Number(url.searchParams.get("n"));
>   res.writeHead(200, { "Content-Type": "application/json" });
>   res.end(JSON.stringify({ result: n * 2 }));
> });
>
> server.listen(0);
> await once(server, "listening");
> const response = await fetch(`http://localhost:${server.address().port}/double?n=21`);
> console.log(await response.text());
> server.close();
> ```
>
> ```sortie
> {"result":42}
> ```

## Summary

- `createServer((req, res) => …)` calls your function for each request; `listen` opens it, `close` closes it.
- `req.method` and `new URL(req.url, "http://localhost")` tell what the client wants.
- A route = method + path; otherwise 404 (or 405 for a wrong method).
- Answer in JSON with `JSON.stringify` and `Content-Type: application/json`, and a fair status code.
- The body of a `POST` arrives in chunks: glue them back together, then `JSON.parse` inside a `try...catch` (400 if invalid).

Next: [testing your code](tests.md)
