# La programación asíncrona

Esta página explica cómo TypeScript describe las operaciones que tardan con el tipo `Promise<T>`, y cómo esperarlas de forma limpia con `async` y `await`.

> **Objetivos:**
>
> - entender qué es una promesa y leer el tipo `Promise<T>`;
> - escribir una función `async` y esperar su resultado con `await`;
> - lanzar varias tareas a la vez con `Promise.all`;
> - capturar los errores de una promesa con `try` y `catch`;
> - convertir `setTimeout` en una promesa para hacer una pausa.

## ¿Por qué esperar?

Algunas operaciones no responden enseguida: leer un archivo, consultar un
[servidor](../debuter/glossaire.md#servidor), esperar un segundo. Si el programa se
quedara bloqueado durante ese tiempo, no podría hacer nada más.

JavaScript, y por tanto TypeScript, funciona de otra manera: **lanza** la operación,
sigue con su trabajo y vuelve al resultado cuando está listo. Se dice que el código es
**asíncrono** (mira [el glosario](../debuter/glossaire.md#async)).

Es como en un restaurante: pides, el camarero te da un ticket y charlas mientras
esperas. El ticket es la **promesa** de un plato que llegará.

## El tipo Promise<T>

Una **promesa** es un objeto que más tarde contendrá un valor. Su tipo se escribe
`Promise<T>`, donde `T` es el tipo del valor prometido:

| Tipo | Se lee |
|------|--------|
| `Promise<number>` | «una promesa de número» |
| `Promise<string>` | «una promesa de texto» |
| `Promise<void>` | «una promesa que no da ningún valor, solo una señal de fin» |

`Promise.resolve(valor)` crea una promesa ya cumplida. Para leer su valor, escribes
`await` delante:

```typescript
const promesa: Promise<number> = Promise.resolve(42);
const valor: number = await promesa;
console.log(valor);
```

```sortie
42
```

`await` quiere decir «espera a que la promesa se cumpla y luego dame su valor». El tipo
cambia: `promesa` es un `Promise<number>`, `valor` es un simple `number`.

> **Recuerda:** `await` «desenvuelve» una promesa: `await` sobre un `Promise<T>` da un `T`.

## Las funciones async

La palabra `async` delante de una función la vuelve asíncrona. Una función `async`
devuelve **siempre** una promesa, aunque escribas `return 42`. TypeScript lo sabe: el
tipo de retorno se escribe `Promise<number>`, no `number`.

```typescript
async function precioConIva(precioSinIva: number): Promise<number> {
  return precioSinIva * 1.2;
}

const precio = await precioConIva(50);
console.log(`Precio con IVA: ${precio} €`);
```

```sortie
Precio con IVA: 60 €
```

Dentro de una función `async`, puedes usar `await` para esperar otras promesas. El
código se lee entonces de arriba abajo, como código normal:

```typescript
async function cargarNombre(id: number): Promise<string> {
  return id === 1 ? "Ada" : "Desconocido";
}

async function saludar(id: number): Promise<string> {
  const nombre = await cargarNombre(id);
  return `¡Hola, ${nombre}!`;
}

console.log(await saludar(1));
console.log(await saludar(7));
```

```sortie
¡Hola, Ada!
¡Hola, Desconocido!
```

> **Nota:** en este sitio, los ejemplos son módulos: `await` funciona directamente a
> nivel del archivo («top-level await»). En otros sitios, `await` solo se permite dentro
> de una función `async`.

### Una función flecha async

La palabra `async` también se coloca delante de una función flecha:

```typescript
const cuadrado = async (n: number): Promise<number> => n * n;

console.log(await cuadrado(9));
```

```sortie
81
```

## Hacer una pausa con setTimeout

`setTimeout(funcion, ms)` llama a una función tras un retraso en milisegundos. No usa
promesas, pero se puede **envolver** en una: `new Promise` recibe una función que llama
a `resolve` cuando el trabajo ha terminado.

```typescript
function esperar(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

console.log("Pido.");
await esperar(100);
console.log("Llega el plato.");
```

```sortie
Pido.
Llega el plato.
```

El programa se detiene 100 milisegundos entre las dos líneas. El tipo `Promise<void>`
dice que esta promesa no da ningún valor: solo avisa de que «ya está».

> **Consejo:** Node también ofrece una versión ya hecha:
> `import { setTimeout as esperar } from "node:timers/promises";`.

### El orden de ejecución

Sin `await`, el programa no espera: sigue en la línea siguiente. Fíjate en el orden de
los mensajes:

```typescript
function esperar(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function tarea(nombre: string, ms: number): Promise<void> {
  await esperar(ms);
  console.log(`${nombre} terminada`);
}

const lenta = tarea("Tarea lenta", 60);
const rapida = tarea("Tarea rápida", 20);
console.log("Las dos tareas están lanzadas");
await lenta;
await rapida;
console.log("Fin");
```

```sortie
Las dos tareas están lanzadas
Tarea rápida terminada
Tarea lenta terminada
Fin
```

Las dos tareas se ejecutan a la vez. La rápida termina primero, aunque se lanzó en
segundo lugar.

## Promise.all y las tuplas

`Promise.all` espera **varias** promesas a la vez y devuelve un array con sus
resultados, en el orden en que las diste (no en el orden en que terminan).

Si las promesas tienen tipos distintos, TypeScript da una
[tupla](../debuter/glossaire.md#tupla): un array en el que cada casilla tiene su propio
tipo. Aquí, el resultado es de tipo `[string, number]`.

```typescript
function esperar(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function cargarNombre(): Promise<string> {
  await esperar(50);
  return "Ada";
}

async function cargarEdad(): Promise<number> {
  await esperar(10);
  return 36;
}

const [nombre, edad] = await Promise.all([cargarNombre(), cargarEdad()]);
console.log(`${nombre} tiene ${edad} años`);
console.log(nombre.toUpperCase(), edad + 1);
```

```sortie
Ada tiene 36 años
ADA 37
```

Gracias a la tupla, TypeScript sabe que `nombre` es un `string` y `edad` un `number`:
puedes llamar a `toUpperCase()` en uno y hacer un cálculo con el otro.

### Una lista de promesas

Con `map`, creas una promesa por elemento y luego `Promise.all` las espera todas. El
resultado es un `number[]`:

```typescript
async function longitud(palabra: string): Promise<number> {
  return palabra.length;
}

const palabras = ["gato", "elefante", "oso"];
const longitudes: number[] = await Promise.all(palabras.map(longitud));
console.log(longitudes);
```

```sortie
[ 4, 8, 3 ]
```

> **Recuerda:** lanza primero todas las promesas y luego espéralas juntas con
> `Promise.all`. Es más rápido que esperarlas una por una.

## Gestionar los errores

Una promesa puede **fallar**: se dice que es **rechazada**. Con `await`, un rechazo se
convierte en una [excepción](../debuter/glossaire.md#excepcion) normal, que capturas
con `try` y `catch`.

En modo estricto, la variable del `catch` es de tipo `unknown`: TypeScript no sabe qué
se lanzó. Comprueba con `instanceof Error` antes de leer `.message`.

```typescript
async function dividir(a: number, b: number): Promise<number> {
  if (b === 0) {
    throw new Error("división por cero");
  }
  return a / b;
}

try {
  console.log(await dividir(10, 2));
  console.log(await dividir(1, 0));
  console.log("Nunca se muestra");
} catch (e) {
  if (e instanceof Error) {
    console.log(`Error: ${e.message}`);
  }
}
```

```sortie
5
Error: división por cero
```

Si una sola promesa de `Promise.all` falla, todo el `Promise.all` falla.

### Promise.allSettled: conservar todos los resultados

`Promise.allSettled` espera todas las promesas, cumplidas o no. Cada resultado tiene un
campo `status` que vale `"fulfilled"` (cumplida) o `"rejected"` (rechazada). TypeScript
usa este campo para saber si puedes leer `value` o `reason`:

```typescript
async function comprobar(edad: number): Promise<number> {
  if (edad < 0) {
    throw new Error(`edad no válida: ${edad}`);
  }
  return edad;
}

const resultados = await Promise.allSettled([comprobar(12), comprobar(-3), comprobar(40)]);
for (const r of resultados) {
  if (r.status === "fulfilled") {
    console.log("OK", r.value);
  } else {
    console.log("Fallo", String(r.reason));
  }
}
```

```sortie
OK 12
Fallo Error: edad no válida: -3
OK 40
```

## Errores frecuentes

### Olvidar await

```typescript norun
async function leerNumero(): Promise<number> {
  return 42;
}

const n: number = leerNumero();
console.log(n + 1);
```

```text
main.ts(5,7): error TS2322: Type 'Promise<number>' is not assignable to type 'number'.
```

`leerNumero()` devuelve una promesa, no un número. Añade `await`:
`const n: number = await leerNumero();`.

### Escribir un tipo de retorno equivocado

```typescript norun
async function leerNumero(): number {
  return 42;
}
```

```text
main.ts(1,30): error TS1064: The return type of an async function or method must be the global Promise<T> type. Did you mean to write 'Promise<number>'?
```

Una función `async` siempre devuelve una promesa. Escribe `Promise<number>`.

### Usar await fuera de una función async

```typescript norun
function esperar(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function arrancar() {
  await esperar(100);
  console.log("Listo");
}
```

```text
main.ts(6,3): error TS1308: 'await' expressions are only allowed within async functions and at the top levels of modules.
```

Añade `async` delante de la función: `async function arrancar() { … }`.

### Leer e.message sin comprobar

```typescript norun
try {
  await Promise.reject(new Error("avería"));
} catch (e) {
  console.log(e.message);
}
```

```text
main.ts(4,15): error TS18046: 'e' is of type 'unknown'.
```

Se puede lanzar cualquier cosa, no solo un `Error`. Comprueba primero:
`if (e instanceof Error) { console.log(e.message); }`.

## Ponte a prueba

**Pregunta 1:** ¿cuál es el tipo de retorno de `async function f() { return "ok"; }`?

> **Respuesta**
>
> `Promise<string>`. Una función `async` siempre envuelve su valor de retorno en una
> promesa.

**Pregunta 2:** con `const [a, b] = await Promise.all([Promise.resolve(1), Promise.resolve("x")])`,
¿cuáles son los tipos de `a` y `b`?

> **Respuesta**
>
> `a` es un `number` y `b` un `string`. `Promise.all` da una tupla
> `[number, string]`, en el orden de las promesas.

**Pregunta 3:** escribe una función `esperar(ms)` que devuelva una promesa cumplida tras
`ms` milisegundos, y luego úsala para mostrar «Uno», esperar y mostrar «Dos».

> **Respuesta**
>
> ```typescript
> function esperar(ms: number): Promise<void> {
>   return new Promise((resolve) => setTimeout(resolve, ms));
> }
>
> console.log("Uno");
> await esperar(50);
> console.log("Dos");
> ```
>
> ```sortie
> Uno
> Dos
> ```

## En resumen

- Una promesa `Promise<T>` contendrá más tarde un valor de tipo `T`.
- Una función `async` siempre devuelve una promesa; `await` extrae su valor.
- `Promise.all` espera varias promesas y da una tupla bien tipada.
- Una promesa rechazada se convierte en una excepción: captúrala con `try` y `catch`,
  comprobando `e instanceof Error`.
- `new Promise((resolve) => setTimeout(resolve, ms))` crea una pausa.

Siguiente: [configurar TypeScript](configuration.md). Compara con
[la concurrencia en Ruby](../ruby/concurrence.md).
