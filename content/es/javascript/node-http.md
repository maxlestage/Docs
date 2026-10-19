# Un servidor web con node:http

Esta página muestra cómo escribir un pequeño servidor web con el módulo `node:http` de Node: responder según el método y la dirección, enviar JSON, elegir el código de estado correcto y leer el cuerpo de una petición.

> **Objetivos:**
>
> - sabrás crear un servidor con `createServer` y probarlo desde el mismo script;
> - sabrás leer el método, la dirección y los parámetros de una petición;
> - sabrás organizar rutas según el método y la ruta;
> - sabrás responder en JSON con el código de estado correcto;
> - sabrás leer el cuerpo de una petición `POST`.

## Un servidor es una ventanilla

Un [servidor](../debuter/glossaire.md#servidor) es un programa que espera **peticiones**
y devuelve **respuestas**, como la ventanilla de una estación de tren. El
[cliente](../debuter/glossaire.md#cliente) (un navegador, `fetch`, `curl`) se acerca a la
ventanilla y pide algo. El empleado lee la petición y luego responde.

```text
   cliente                                 servidor (tu código)
 ┌─────────┐   petición: GET /hola       ┌──────────────────────┐
 │  fetch  │ ──────────────────────────▶ │ (req, res) => { … }  │
 │         │ ◀────────────────────────── │                      │
 └─────────┘   respuesta: 200 + "Hola"   └──────────────────────┘
```

El capítulo [JavaScript para la web](web.md) presentó [HTTP](../debuter/glossaire.md#http),
`fetch` y un servidor mínimo. Aquí construimos un pequeño servidor de verdad, paso a
paso.

> **Recuerda:** entra una petición, tu función la lee y escribe una respuesta.

## El primer servidor

Un servidor normal funciona **sin fin**, esperando visitas. Para que los ejemplos de
esta página se detengan solos, cada uno sigue el mismo plan:

1. crear el servidor;
2. lanzarlo en el **puerto 0**: el sistema elige un puerto libre;
3. llamarlo con `fetch` desde el mismo script;
4. mostrar las respuestas;
5. cerrarlo con `close()`.

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

const servidor = createServer((req, res) => {
  res.writeHead(200, { "Content-Type": "text/plain; charset=utf-8" });
  res.end("¡Hola desde Node!");
});

servidor.listen(0);                  // 0: un puerto libre, al azar
await once(servidor, "listening");   // esperar a que el servidor esté listo
const direccion = `http://localhost:${servidor.address().port}`;

const respuesta = await fetch(direccion);
console.log(respuesta.status);
console.log(await respuesta.text());

servidor.close();
```

```sortie
200
¡Hola desde Node!
```

El puerto cambia en cada ejecución: por eso nunca lo mostramos.

| Paso | Quién actúa | Qué pasa |
|---|---|---|
| 1 | script | `createServer` prepara la ventanilla (todavía nada escucha) |
| 2 | script | `listen(0)` abre la ventanilla en un puerto libre |
| 3 | script | `await once(...)` espera a que la ventanilla esté abierta |
| 4 | cliente | `fetch(direccion)` envía `GET /` |
| 5 | servidor | se llama a la función `(req, res)` con la petición |
| 6 | servidor | `writeHead(200, …)` elige el estado, `end(…)` envía el texto |
| 7 | cliente | `respuesta.status` vale `200`, `respuesta.text()` lee el cuerpo |
| 8 | script | `close()` cierra la ventanilla; el programa termina |

La función que se da a `createServer` se llama **una vez por petición**. Recibe dos
objetos:

- `req` (*request*, la petición): lo que pide el cliente;
- `res` (*response*, la respuesta): el formulario de respuesta que hay que rellenar.

Prueba: cambia `200` por `418` en `writeHead`. `respuesta.status` muestra `418`
(«soy una tetera», un código de broma, pero real).

> **Nota:** para un servidor de verdad, escribe `servidor.listen(3000)` sin `close()`, y
> luego abre `http://localhost:3000` en el navegador. Detenlo con `Ctrl`+`C`.

> **Recuerda:** `createServer(funcion)` crea el servidor, `listen` lo abre, `res.end`
> envía la respuesta. Sin `res.end`, el cliente espera para siempre.

## Leer la petición

El objeto `req` contiene:

- `req.method`: el método (`"GET"`, `"POST"`…);
- `req.url`: la ruta y los parámetros, por ejemplo `"/libros?pagina=2"`;
- `req.headers`: las cabeceras, con nombres en minúsculas.

Para descomponer `req.url`, se la damos a `new URL` (ver [la web](web.md)), con una
dirección base cualquiera:

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

const servidor = createServer((req, res) => {
  const url = new URL(req.url, "http://localhost");
  const nombre = url.searchParams.get("nombre") ?? "desconocido";
  res.end(`${req.method} ${url.pathname} nombre=${nombre}`);
});

servidor.listen(0);
await once(servidor, "listening");
const direccion = `http://localhost:${servidor.address().port}`;

console.log(await (await fetch(`${direccion}/hola?nombre=Lucía`)).text());
console.log(await (await fetch(`${direccion}/hola`)).text());
console.log(await (await fetch(`${direccion}/`, { method: "DELETE" })).text());

servidor.close();
```

```sortie
GET /hola nombre=Lucía
GET /hola nombre=desconocido
DELETE / nombre=desconocido
```

| Petición enviada | `req.method` | `url.pathname` | `searchParams.get("nombre")` |
|---|---|---|---|
| `/hola?nombre=Lucía` | `GET` | `/hola` | `"Lucía"` |
| `/hola` | `GET` | `/hola` | `null` → `"desconocido"` |
| `/` con `DELETE` | `DELETE` | `/` | `null` → `"desconocido"` |

`fetch` envía `GET` por defecto. La opción `method` elige otro.

> **Recuerda:** `req.method` dice **qué hacer**, `url.pathname` dice **sobre qué**,
> `url.searchParams` da los pequeños ajustes que van después del `?`.

## Rutas: método + ruta

Una **ruta** es un método y un camino, unidos a una función. Como los carteles de una
oficina pública: «Entrega de documentos → ventanilla 1», «Recogida → ventanilla 2». Lo
más sencillo: pegar los dos en una sola clave y luego buscarla en un objeto.

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

const rutas = {
  "GET /": () => "Inicio",
  "GET /contacto": () => "Escríbenos",
  "POST /contacto": () => "Mensaje recibido",
};

const servidor = createServer((req, res) => {
  const { pathname } = new URL(req.url, "http://localhost");
  const clave = `${req.method} ${pathname}`;
  const accion = rutas[clave];
  if (accion) {
    res.writeHead(200, { "Content-Type": "text/plain; charset=utf-8" });
    res.end(accion());
  } else {
    res.writeHead(404, { "Content-Type": "text/plain; charset=utf-8" });
    res.end("No encontrado");
  }
});

servidor.listen(0);
await once(servidor, "listening");
const direccion = `http://localhost:${servidor.address().port}`;

for (const [metodo, camino] of [["GET", "/"], ["GET", "/contacto"], ["POST", "/contacto"], ["GET", "/precio"]]) {
  const respuesta = await fetch(direccion + camino, { method: metodo });
  console.log(metodo, camino, "→", respuesta.status, await respuesta.text());
}

servidor.close();
```

```sortie
GET / → 200 Inicio
GET /contacto → 200 Escríbenos
POST /contacto → 200 Mensaje recibido
GET /precio → 404 No encontrado
```

| Petición | `clave` | `rutas[clave]` | Estado |
|---|---|---|---|
| 1 | `"GET /"` | encontrada | 200 |
| 2 | `"GET /contacto"` | encontrada | 200 |
| 3 | `"POST /contacto"` | encontrada (¡otra función!) | 200 |
| 4 | `"GET /precio"` | `undefined` | 404 |

El mismo camino `/contacto` lleva a dos funciones: es el método el que marca la
diferencia.

Prueba: añade la ruta `"GET /precio": () => "Gratis"`. La última línea pasa a ser
`GET /precio → 200 Gratis`.

> **Recuerda:** una ruta = método + camino. Si ninguna ruta coincide, responde 404.

## Responder en JSON

Una API responde casi siempre en [JSON](../debuter/glossaire.md#json). `res.end` solo
acepta texto: se convierte con `JSON.stringify`, y se avisa al cliente con la cabecera
`Content-Type: application/json`. Una pequeña función evita repetirlo:

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

function enviarJson(res, estado, datos) {
  res.writeHead(estado, { "Content-Type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(datos));
}

const frutas = ["manzana", "pera", "kiwi"];

const servidor = createServer((req, res) => {
  enviarJson(res, 200, { total: frutas.length, frutas });
});

servidor.listen(0);
await once(servidor, "listening");

const respuesta = await fetch(`http://localhost:${servidor.address().port}/frutas`);
console.log(respuesta.headers.get("content-type"));
const datos = await respuesta.json();
console.log(datos.total, datos.frutas[2]);

servidor.close();
```

```sortie
application/json; charset=utf-8
3 kiwi
```

```text
 servidor : { total: 3, frutas: [...] }   (objeto)
              │ JSON.stringify
              ▼
 red      : '{"total":3,"frutas":["manzana","pera","kiwi"]}'   (texto)
              │ respuesta.json()
              ▼
 cliente  : { total: 3, frutas: [...] }   (objeto)
```

> **Recuerda:** objeto → `JSON.stringify` → red → `respuesta.json()` → objeto. Y la
> cabecera `Content-Type: application/json`.

## Los códigos de estado

El código de estado es lo **primero** que lee el cliente. Es un número de tres cifras.
La primera cifra da la familia:

| Código | Nombre | Cuándo usarlo |
|---|---|---|
| 200 | OK | todo va bien, aquí está la respuesta |
| 201 | Created | se ha creado un recurso (después de un `POST`) |
| 204 | No Content | hecho, nada que devolver (después de un `DELETE`) |
| 400 | Bad Request | la petición está mal formada (JSON no válido, campo que falta) |
| 404 | Not Found | esta dirección no existe |
| 405 | Method Not Allowed | el camino existe, pero no con este método |
| 500 | Internal Server Error | el servidor ha fallado |

- `2xx`: éxito;
- `4xx`: culpa del **cliente** (ha pedido mal);
- `5xx`: culpa del **servidor** (tu código).

> **Recuerda:** 2xx = salió bien, 4xx = el cliente se equivocó, 5xx = el servidor se
> equivocó. Elegir el código correcto ayuda a quien usa tu API.

## Leer el cuerpo de una petición

Con `POST`, el cliente envía datos en el **cuerpo** de la petición. No llegan de una
vez: llegan en **trozos**, como una carta larga enviada en varios sobres. `req` es un
flujo que se recorre con `for await` (ver [la asincronía](asynchrone.md)), se guarda
cada trozo en un array y al final se vuelven a pegar con `Buffer.concat`.

### Los trozos paso a paso

Un cuerpo pequeño suele llegar en un solo trozo. Para ver el pegado, demos a la función
`leerCuerpo` un flujo falso de tres trozos, fabricado con `Readable.from`:

```javascript
import { Readable } from "node:stream";

async function leerCuerpo(req) {
  const trozos = [];
  for await (const trozo of req) {
    trozos.push(trozo);
    console.log("recibido:", trozo.toString(), "| trozos:", trozos.length);
  }
  return Buffer.concat(trozos).toString("utf8");
}

const reqFalsa = Readable.from([Buffer.from('{"tex'), Buffer.from('to":"pa'), Buffer.from('n"}')]);
const texto = await leerCuerpo(reqFalsa);
console.log(texto);
console.log(JSON.parse(texto).texto);
```

```sortie
recibido: {"tex | trozos: 1
recibido: to":"pa | trozos: 2
recibido: n"} | trozos: 3
{"texto":"pan"}
pan
```

| Vuelta | Trozo recibido | Lo que llevamos pegado |
|---|---|---|
| 1 | `{"tex` | `{"tex` |
| 2 | `to":"pa` | `{"texto":"pa` |
| 3 | `n"}` | `{"texto":"pan"}` |

Ningún trozo por sí solo es JSON válido: hay que esperar a **todo** antes de
`JSON.parse`.

### Un eco en JSON

El servidor lee el cuerpo, lo convierte y devuelve lo que ha entendido. Si el JSON no
es válido, es culpa del cliente: 400.

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

async function leerCuerpo(req) {
  const trozos = [];
  for await (const trozo of req) trozos.push(trozo);
  return Buffer.concat(trozos).toString("utf8");
}

function enviarJson(res, estado, datos) {
  res.writeHead(estado, { "Content-Type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(datos));
}

const servidor = createServer(async (req, res) => {
  let datos;
  try {
    datos = JSON.parse(await leerCuerpo(req));
  } catch {
    return enviarJson(res, 400, { error: "JSON no válido" });
  }
  enviarJson(res, 200, { recibido: datos });
});

servidor.listen(0);
await once(servidor, "listening");
const direccion = `http://localhost:${servidor.address().port}`;

for (const cuerpo of ['{"edad": 12}', "{edad: 12}"]) {
  const respuesta = await fetch(direccion, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: cuerpo,
  });
  console.log(respuesta.status, await respuesta.text());
}

servidor.close();
```

```sortie
200 {"recibido":{"edad":12}}
400 {"error":"JSON no válido"}
```

La función del servidor ahora es `async`: espera el final del cuerpo con `await`.

Prueba: envía `body: ""` (un cuerpo vacío). `JSON.parse("")` también falla: la
respuesta es `400 {"error":"JSON no válido"}`.

> **Recuerda:** el cuerpo llega en trozos. Pégalos, **luego** `JSON.parse` dentro de un
> `try...catch`, y responde 400 si falla.

## Una pequeña API de tareas

Juntemos todo: una lista de tareas guardada en memoria, con tres rutas.

| Ruta | Papel | Estados posibles |
|---|---|---|
| `GET /tareas` | la lista | 200 |
| `GET /tareas/:id` | una tarea | 200, 404 |
| `POST /tareas` | añadir una tarea | 201, 400 |

```javascript
import { createServer } from "node:http";
import { once } from "node:events";

async function leerCuerpo(req) {
  const trozos = [];
  for await (const trozo of req) trozos.push(trozo);
  return Buffer.concat(trozos).toString("utf8");
}

function enviarJson(res, estado, datos) {
  res.writeHead(estado, { "Content-Type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(datos));
}

const tareas = [{ id: 1, texto: "pan" }];
let siguienteId = 2;

async function responder(req, res) {
  const { pathname } = new URL(req.url, "http://localhost");
  const [, recurso, id] = pathname.split("/");   // "/tareas/1" → ["", "tareas", "1"]

  if (recurso !== "tareas") return enviarJson(res, 404, { error: "no encontrado" });

  if (req.method === "GET" && id === undefined) {
    return enviarJson(res, 200, tareas);
  }
  if (req.method === "GET") {
    const tarea = tareas.find((t) => t.id === Number(id));
    return tarea ? enviarJson(res, 200, tarea) : enviarJson(res, 404, { error: "no encontrado" });
  }
  if (req.method === "POST" && id === undefined) {
    let cuerpo;
    try {
      cuerpo = JSON.parse(await leerCuerpo(req));
    } catch {
      return enviarJson(res, 400, { error: "JSON no válido" });
    }
    if (typeof cuerpo.texto !== "string" || cuerpo.texto.trim() === "") {
      return enviarJson(res, 400, { error: "falta el texto" });
    }
    const tarea = { id: siguienteId++, texto: cuerpo.texto.trim() };
    tareas.push(tarea);
    return enviarJson(res, 201, tarea);
  }
  enviarJson(res, 405, { error: "método no permitido" });
}

const servidor = createServer(responder);
servidor.listen(0);
await once(servidor, "listening");
const direccion = `http://localhost:${servidor.address().port}`;

async function llamar(metodo, camino, cuerpo) {
  const opciones = { method: metodo };
  if (cuerpo !== undefined) opciones.body = JSON.stringify(cuerpo);
  const respuesta = await fetch(direccion + camino, opciones);
  console.log(metodo, camino, "→", respuesta.status, await respuesta.text());
}

await llamar("GET", "/tareas");
await llamar("POST", "/tareas", { texto: " leche " });
await llamar("POST", "/tareas", { titulo: "olvido" });
await llamar("GET", "/tareas/2");
await llamar("GET", "/tareas/9");
await llamar("DELETE", "/tareas/1");
await llamar("GET", "/tareas");

servidor.close();
```

```sortie
GET /tareas → 200 [{"id":1,"texto":"pan"}]
POST /tareas → 201 {"id":2,"texto":"leche"}
POST /tareas → 400 {"error":"falta el texto"}
GET /tareas/2 → 200 {"id":2,"texto":"leche"}
GET /tareas/9 → 404 {"error":"no encontrado"}
DELETE /tareas/1 → 405 {"error":"método no permitido"}
GET /tareas → 200 [{"id":1,"texto":"pan"},{"id":2,"texto":"leche"}]
```

Sigamos el estado del servidor después de cada llamada:

| Llamada | `id` | Rama seguida | Estado | `tareas` después | `siguienteId` |
|---|---|---|---|---|---|
| `GET /tareas` | `undefined` | la lista | 200 | pan | 2 |
| `POST` «leche» | `undefined` | añadir | 201 | pan, leche | 3 |
| `POST` sin texto | `undefined` | falta `texto` | 400 | pan, leche | 3 |
| `GET /tareas/2` | `"2"` | una tarea, encontrada | 200 | pan, leche | 3 |
| `GET /tareas/9` | `"9"` | una tarea, ausente | 404 | pan, leche | 3 |
| `DELETE /tareas/1` | `"1"` | ninguna: el final | 405 | pan, leche | 3 |

Atención: `id` viene de la dirección, así que es un **texto** (`"2"`). Se convierte con
`Number(id)` antes de comparar con `===`.

Los datos están en una variable: desaparecen cuando el servidor se detiene. Para
guardarlos hace falta un archivo (ver [los archivos](fichiers.md)) o una base de datos.

Prueba: añade una rama para `DELETE` con un `id` que quite la tarea
(`tareas.splice(indice, 1)`) y responda `204` con `res.writeHead(204); res.end();`.

> **Recuerda:** una API son rutas, JSON y un código de estado honesto para cada caso,
> incluidos los errores.

## Errores frecuentes

### Olvidar res.end

```javascript norun
const servidor = createServer((req, res) => {
  res.writeHead(200);
  // sin res.end()
});
// en el lado del cliente:
await fetch(direccion, { signal: AbortSignal.timeout(300) });
```

```text
TimeoutError: The operation was aborted due to timeout
```

El servidor nunca terminó su respuesta: el cliente espera. Sin el `timeout`, esperaría
muchísimo tiempo. Cada camino de tu función debe terminar con `res.end` (un `return`
ayuda a no olvidarlo).

### Enviar un objeto sin JSON.stringify

```javascript norun
res.writeHead(200, { "Content-Type": "application/json" });
res.end({ mensaje: "Hola" });
```

```text
TypeError [ERR_INVALID_ARG_TYPE]: The "chunk" argument must be of type string or an instance of Buffer or Uint8Array. Received an instance of Object
    at write_ (node:_http_outgoing:950:11)
    at ServerResponse.end (node:_http_outgoing:1107:5)
```

`res.end` solo acepta texto (o bytes). Escribe
`res.end(JSON.stringify({ mensaje: "Hola" }))`, o usa `enviarJson`.

### Responder dos veces

```javascript norun
const servidor = createServer((req, res) => {
  if (req.url === "/") {
    res.end("Inicio");
  }
  res.writeHead(404);
  res.end("No encontrado");
});
```

```text
Error [ERR_HTTP_HEADERS_SENT]: Cannot write headers after they are sent to the client
    at ServerResponse.writeHead (node:_http_server:354:11)
```

Después de `res.end("Inicio")`, la función **sigue** y intenta una segunda respuesta.
El servidor falla. Escribe `return res.end("Inicio");`, o usa `else`.

## Ponte a prueba

**Pregunta 1:** ¿por qué los ejemplos de esta página usan `listen(0)`?

> **Respuesta**
>
> El puerto `0` pide al sistema un puerto libre. El script no corre el riesgo del error
> `EADDRINUSE` (puerto ya ocupado), y obtiene el número real con
> `servidor.address().port`.

**Pregunta 2:** un cliente envía `POST /tareas` con el cuerpo `{texto: pan}`. ¿Qué
código de estado hay que devolver?

> **Respuesta**
>
> `400 Bad Request`: el cuerpo no es JSON válido (faltan las comillas), es culpa del
> cliente.

**Pregunta 3:** escribe un servidor que responda al camino `/doble?n=21` con el JSON
`{"resultado":42}`, llámalo una vez, muestra la respuesta y ciérralo.

> **Respuesta**
>
> ```javascript
> import { createServer } from "node:http";
> import { once } from "node:events";
>
> const servidor = createServer((req, res) => {
>   const url = new URL(req.url, "http://localhost");
>   const n = Number(url.searchParams.get("n"));
>   res.writeHead(200, { "Content-Type": "application/json" });
>   res.end(JSON.stringify({ resultado: n * 2 }));
> });
>
> servidor.listen(0);
> await once(servidor, "listening");
> const respuesta = await fetch(`http://localhost:${servidor.address().port}/doble?n=21`);
> console.log(await respuesta.text());
> servidor.close();
> ```
>
> ```sortie
> {"resultado":42}
> ```

## En resumen

- `createServer((req, res) => …)` llama a tu función en cada petición; `listen` lo abre, `close` lo cierra.
- `req.method` y `new URL(req.url, "http://localhost")` dicen lo que quiere el cliente.
- Una ruta = método + camino; si no, 404 (o 405 para un método equivocado).
- Responde en JSON con `JSON.stringify` y `Content-Type: application/json`, y un código de estado justo.
- El cuerpo de un `POST` llega en trozos: pégalos, luego `JSON.parse` dentro de un `try...catch` (400 si no es válido).

Siguiente: [probar tu código](tests.md)
