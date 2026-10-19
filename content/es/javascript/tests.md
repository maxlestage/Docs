# Probar tu código

Esta página muestra cómo escribir tests automáticos en JavaScript con las herramientas que vienen con Node: `console.assert`, el módulo `node:assert` y el lanzador de tests `node:test`.

> **Objetivos:**
>
> - entender para qué sirve un test automático;
> - comprobar valores con `assert.strictEqual`, `assert.deepStrictEqual` y `assert.throws`;
> - escribir y lanzar tests con `test()` y `describe()`;
> - leer un test que falla;
> - escribir el test antes que el código.

## ¿Por qué hacer tests?

Un **test** es un pequeño programa que comprueba otro programa (ver
[el glosario](../debuter/glossaire.md#test)). Llama a tu código con valores conocidos
y compara el resultado con el que esperas.

Sin tests, compruebas a mano, con `console.log`, y vuelves a empezar después de cada
cambio. Con tests:

- un solo comando lo comprueba todo, en un segundo;
- si un cambio rompe algo, lo sabes enseguida;
- los tests muestran cómo usar tu código: son una documentación que funciona.

Es como la inspección técnica de un coche: una lista de puntos precisos, comprobados
siempre de la misma manera.

> **Recuerda:** un test responde a una pregunta sencilla: «con esta entrada, ¿obtengo
> de verdad esta salida?».

## Lo más sencillo: console.assert

`console.assert(condición, mensaje)` comprueba una condición. Si es verdadera, no pasa
nada. Si es falsa, se muestra el mensaje.

```javascript norun
console.assert(1 + 1 === 3, "el cálculo es incorrecto");
console.assert(2 + 2 === 4, "nunca se muestra");
console.log("Fin");
```

```text
Assertion failed: el cálculo es incorrecto
Fin
```

Cuando todo va bien, `console.assert` se queda en silencio:

```javascript
function doble(x) {
  return x * 2;
}

console.assert(doble(4) === 8, "doble(4) debería valer 8");
console.assert(doble(0) === 0, "doble(0) debería valer 0");
console.log("Comprobaciones terminadas.");
```

```sortie
Comprobaciones terminadas.
```

> **Atención:** `console.assert` no detiene el programa: `Fin` se muestra de todos
> modos. Es práctico para una comprobación rápida, pero un test de verdad debe
> **detenerse** y avisar del fallo. Ese es el papel de `node:assert`.

## El módulo node:assert

Node trae el módulo `node:assert`. Importa su versión **estricta**,
`node:assert/strict`: compara con `===`, sin conversión de tipo oculta.

### assert.strictEqual

`assert.strictEqual(obtenido, esperado)` comprueba que dos valores son idénticos. Si lo
son, no se muestra nada.

```javascript
import assert from "node:assert/strict";

function doble(x) {
  return x * 2;
}

assert.strictEqual(doble(4), 8);
assert.strictEqual(doble(-3), -6);
console.log("Todos los tests pasan.");
```

```sortie
Todos los tests pasan.
```

El orden importa: primero el valor **obtenido** (`actual`), luego el valor **esperado**
(`expected`). Así, el mensaje de fallo es correcto.

### Leer un fallo

Si los valores son distintos, `assert` lanza una
[excepción](../debuter/glossaire.md#excepcion) `AssertionError` y el programa
se detiene:

```javascript norun
import assert from "node:assert/strict";

assert.strictEqual(0.1 + 0.2, 0.3);
```

```text
AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:
+ actual - expected

+ 0.30000000000000004
- 0.3
     ^

    at file:///home/alice/proyecto/main.mjs:3:8
```

La línea `+` da el valor obtenido, la línea `-` el valor esperado. El `^` muestra
dónde empiezan a diferir. Aquí, es la sorpresa de los números decimales (ver
[los números](nombres.md)).

### assert.deepStrictEqual para los arrays y los objetos

Dos arrays que contienen los mismos valores **no** son el mismo array: `===` compara
las direcciones, no el contenido. Para comparar el contenido, usa
`assert.deepStrictEqual` («profundamente igual»).

```javascript
import assert from "node:assert/strict";

function pares(numeros) {
  return numeros.filter((n) => n % 2 === 0);
}

assert.deepStrictEqual(pares([1, 2, 3, 4]), [2, 4]);
assert.deepStrictEqual({ nombre: "Alice", edad: 30 }, { edad: 30, nombre: "Alice" });
console.log("Contenidos idénticos.");
```

```sortie
Contenidos idénticos.
```

El orden de las propiedades de un objeto no importa; el orden de los elementos de un
array, sí. Si falla, el mensaje muestra la diferencia línea por línea:

```text
AssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:
+ actual - expected

  {
+   edad: 30,
-   edad: 31,
    nombre: 'Alice'
  }
```

### assert.throws para comprobar un error

Un buen código rechaza los valores incorrectos. `assert.throws` comprueba que una
función **lanza** de verdad un error. Dale una función a la que llamar, no su
resultado: de ahí la flecha `() => …`.

```javascript
import assert from "node:assert/strict";

function comprobarEdad(edad) {
  if (edad < 0) {
    throw new RangeError("edad negativa");
  }
  return edad;
}

assert.throws(() => comprobarEdad(-1), RangeError);
assert.throws(() => comprobarEdad(-1), { message: "edad negativa" });
assert.strictEqual(comprobarEdad(12), 12);
console.log("Los errores se lanzan correctamente.");
```

```sortie
Los errores se lanzan correctamente.
```

El segundo argumento indica el error esperado: una clase (`RangeError`), una expresión
regular (`/negativa/`) o un objeto con las propiedades que hay que comprobar.

### Las demás comprobaciones

| Comprobación | Pasa si… |
| --- | --- |
| `assert.ok(valor)` | el valor es verdadero (*truthy*) |
| `assert.strictEqual(a, b)` | `a === b` |
| `assert.notStrictEqual(a, b)` | `a !== b` |
| `assert.deepStrictEqual(a, b)` | mismo contenido (arrays, objetos) |
| `assert.throws(() => f())` | `f()` lanza un error |
| `assert.match(texto, /patrón/)` | el texto corresponde al patrón |

## El lanzador de tests node:test

Varios `assert` uno tras otro está bien. Pero al primer fallo, todo se detiene, y no
sabes nada de las comprobaciones siguientes. El módulo `node:test` coloca cada
comprobación en un **test** que tiene un nombre. Lanza todos los tests, aunque uno
falle, y luego muestra un resumen.

### Un primer test con test()

`test(nombre, función)` declara un test. El nombre describe lo que comprueba el test.

```javascript
import { test } from "node:test";
import assert from "node:assert/strict";

function suma(a, b) {
  return a + b;
}

test("suma dos números", () => {
  assert.strictEqual(suma(2, 3), 5);
});

test("funciona con negativos", () => {
  assert.strictEqual(suma(-2, -3), -5);
});
```

Lanza el archivo como cualquier otro programa:

```sh
node suma.test.js
```

En un terminal, Node muestra un resumen como este:

```text
✔ suma dos números (0.81ms)
✔ funciona con negativos (0.22ms)
ℹ tests 2
ℹ suites 0
ℹ pass 2
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 52.17
```

- `✔`: el test pasa; `✖`: falla.
- El número entre paréntesis es la duración del test: cambia en cada ejecución.
- `pass` cuenta los tests que pasan, `fail` los fallos.

> **Nota:** cuando la salida no va a un terminal (a un archivo, por ejemplo), Node usa
> otro formato, llamado TAP: `ok 1 - suma dos números`. El sentido es el mismo.

### Agrupar los tests con describe()

`describe(nombre, función)` agrupa los tests que hablan de lo mismo. Es un capítulo;
cada `test` es un párrafo.

```javascript
import { describe, test } from "node:test";
import assert from "node:assert/strict";

function mayuscula(palabra) {
  return palabra.charAt(0).toUpperCase() + palabra.slice(1);
}

describe("mayuscula", () => {
  test("pone la primera letra en mayúscula", () => {
    assert.strictEqual(mayuscula("alice"), "Alice");
  });

  test("no cambia una palabra vacía", () => {
    assert.strictEqual(mayuscula(""), "");
  });
});
```

```text
▶ mayuscula
  ✔ pone la primera letra en mayúscula (0.62ms)
  ✔ no cambia una palabra vacía (0.15ms)
✔ mayuscula (1.21ms)
ℹ tests 2
ℹ suites 1
ℹ pass 2
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 49.88
```

### Lanzar todos los tests con node --test

Coloca tus tests en archivos cuyo nombre termina en `.test.js`, o en una carpeta
`test/`. El comando `node --test` los encuentra y los lanza todos:

```sh
node --test
```

En un proyecto, se coloca en el script `test` del `package.json`, para escribir
simplemente `npm test` (ver [organizar un proyecto](projet.md)).

### Leer un test que falla

Aquí tienes una función `suma` que se equivoca: resta en lugar de sumar.

```javascript norun
function suma(a, b) {
  return a - b;
}

test("suma dos números", () => {
  assert.strictEqual(suma(2, 3), 5);
});
```

```text
✖ suma dos números (1.49ms)
ℹ tests 1
ℹ suites 0
ℹ pass 0
ℹ fail 1
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 51.02

✖ failing tests:

test at suma.test.js:8:1
✖ suma dos números (1.49ms)
  AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:

  -1 !== 5

      at TestContext.<anonymous> (file:///home/alice/proyecto/suma.test.js:9:10)
```

El resumen lo da todo: el nombre del test, su posición (`suma.test.js:8`), el valor
obtenido (`-1`) y el valor esperado (`5`).

### Un test asíncrono

Si la función que pruebas devuelve una promesa, escribe un test `async` y usa `await`
(ver [la asincronía](asynchrone.md)). El test espera a que termine la promesa.

```javascript
import { test } from "node:test";
import assert from "node:assert/strict";

async function cargarPuntuacion() {
  return 42;
}

test("carga la puntuación", async () => {
  const puntuacion = await cargarPuntuacion();
  assert.strictEqual(puntuacion, 42);
});
```

## Vitest y Jest

En los proyectos grandes, se usa a menudo **Vitest** o **Jest**. Son
[paquetes](../debuter/glossaire.md#paquete) que se instalan con npm. Su sintaxis se
parece mucho a `node:test`: `describe`, `test` (o `it`) y `expect` en lugar de
`assert`.

```javascript norun
import { describe, test, expect } from "vitest";
import { suma } from "../src/suma.js";

describe("suma", () => {
  test("suma dos números", () => {
    expect(suma(2, 3)).toBe(5);
  });
});
```

```sh
npm install --save-dev vitest
npx vitest run
```

`expect(obtenido).toBe(esperado)` corresponde a `assert.strictEqual`, y `toEqual` a
`assert.deepStrictEqual`. Empieza con `node:test`: no hay nada que instalar, y las
ideas son las mismas.

## ¿Qué hay que probar?

No hace falta probar todos los valores posibles. Elige unos pocos casos bien
escogidos:

| Caso | Ejemplo para `media(notas)` |
| --- | --- |
| El caso normal | `[10, 12, 14]` da `12` |
| Los límites | una sola nota, una lista vacía |
| Los valores raros | notas con decimales, ceros |
| Los errores | una nota negativa debe rechazarse |

```javascript
import assert from "node:assert/strict";

function media(notas) {
  if (notas.length === 0) return 0;
  if (notas.some((n) => n < 0)) throw new RangeError("nota negativa");
  const suma = notas.reduce((total, n) => total + n, 0);
  return suma / notas.length;
}

assert.strictEqual(media([10, 12, 14]), 12);
assert.strictEqual(media([15]), 15);
assert.strictEqual(media([]), 0);
assert.strictEqual(media([0, 0]), 0);
assert.throws(() => media([10, -2]), RangeError);
console.log("5 casos comprobados.");
```

```sortie
5 casos comprobados.
```

> **Consejo:** cuando encuentres un [bug](../debuter/glossaire.md#bug), escribe
> primero un test que lo muestre, y luego corrígelo. El bug nunca volverá sin que
> te enteres.

## Escribir el test primero

Un método muy extendido consiste en escribir el test **antes** que el código. Se avanza
en tres pasos, siempre los mismos:

1. **Rojo**: escribe un test para una función que todavía no existe. Falla.
2. **Verde**: escribe el código más sencillo que haga pasar el test.
3. **Mejorar**: haz el código más claro, volviendo a lanzar los tests cada vez.

Ejemplo: una función `esPalindromo` que dice si una palabra se lee igual en los dos
sentidos. Primero los tests, que describen lo que queremos:

```javascript norun
test("kayak es un palíndromo", () => {
  assert.strictEqual(esPalindromo("kayak"), true);
});

test("ignora las mayúsculas", () => {
  assert.strictEqual(esPalindromo("Kayak"), true);
});

test("javascript no es un palíndromo", () => {
  assert.strictEqual(esPalindromo("javascript"), false);
});
```

Lanzados tal cual, fallan con `ReferenceError: esPalindromo is not defined`: es el paso
rojo. Luego el código que los hace pasar, el paso verde:

```javascript
import { test } from "node:test";
import assert from "node:assert/strict";

function esPalindromo(palabra) {
  const minusculas = palabra.toLowerCase();
  const alReves = [...minusculas].reverse().join("");
  return minusculas === alReves;
}

test("kayak es un palíndromo", () => {
  assert.strictEqual(esPalindromo("kayak"), true);
});

test("ignora las mayúsculas", () => {
  assert.strictEqual(esPalindromo("Kayak"), true);
});

test("javascript no es un palíndromo", () => {
  assert.strictEqual(esPalindromo("javascript"), false);
});
```

Escribir el test primero te obliga a pensar en **qué** debe hacer la función, antes de
pensar en **cómo** hacerlo.

## Errores frecuentes

### Dar el resultado a assert.throws en lugar de una función

```javascript norun
import assert from "node:assert/strict";

function comprobarEdad(edad) {
  if (edad < 0) throw new RangeError("edad negativa");
  return edad;
}

assert.throws(comprobarEdad(-1), RangeError);
```

```text
file:///home/alice/proyecto/main.mjs:4
  if (edad < 0) throw new RangeError("edad negativa");
                      ^

RangeError: edad negativa
    at comprobarEdad (file:///home/alice/proyecto/main.mjs:4:23)
    at file:///home/alice/proyecto/main.mjs:8:15
```

`comprobarEdad(-1)` se llama **antes** que `assert.throws`: el error salta demasiado
pronto y detiene el programa. Pasa una función:
`assert.throws(() => comprobarEdad(-1), RangeError)`.

### Olvidar importar test

```javascript norun
import assert from "node:assert/strict";

test("doble", () => {
  assert.strictEqual(2 * 2, 4);
});
```

```text
file:///home/alice/proyecto/main.mjs:3
test("doble", () => {
^

ReferenceError: test is not defined
    at file:///home/alice/proyecto/main.mjs:3:1
```

A diferencia de Jest, `test` no existe por sí solo. Añade
`import { test } from "node:test";` al principio del archivo.

### Comparar arrays con strictEqual

```javascript norun
import assert from "node:assert/strict";

assert.strictEqual([1, 2], [1, 2]);
```

```text
AssertionError [ERR_ASSERTION]: Values have same structure but are not reference-equal:

[
  1,
  2
]

    at file:///home/alice/proyecto/main.mjs:3:8
```

Los dos arrays tienen el mismo contenido, pero son dos objetos distintos. Para comparar
el contenido, usa `assert.deepStrictEqual([1, 2], [1, 2])`.

## Ponte a prueba

**Pregunta 1:** ¿qué diferencia hay entre `console.assert` y `assert.strictEqual`?

> **Respuesta**
>
> `console.assert` muestra un mensaje si la condición es falsa, pero el programa
> continúa. `assert.strictEqual` lanza un `AssertionError` que detiene el programa (o
> marca el test como fallido con `node:test`).

**Pregunta 2:** ¿por qué falla este test, y cómo corregirlo?
`assert.strictEqual({ x: 1 }, { x: 1 })`

> **Respuesta**
>
> Son dos objetos distintos: `===` compara los objetos por dirección, no por
> contenido. Usa `assert.deepStrictEqual({ x: 1 }, { x: 1 })`.

**Pregunta 3:** escribe una comprobación que se asegure de que `JSON.parse("{uy")`
lanza un `SyntaxError`.

> **Respuesta**
>
> ```javascript
> import assert from "node:assert/strict";
>
> assert.throws(() => JSON.parse("{uy"), SyntaxError);
> console.log("SyntaxError lanzado correctamente.");
> ```
>
> ```sortie
> SyntaxError lanzado correctamente.
> ```

## En resumen

- Un test comprueba que una entrada conocida da la salida esperada, sin intervención.
- `import assert from "node:assert/strict"` da `strictEqual`, `deepStrictEqual` y
  `throws`; un fallo lanza un `AssertionError`.
- `test()` y `describe()` de `node:test` nombran y agrupan los tests; `node --test`
  los lanza todos.
- Prueba el caso normal, los límites y los errores; escribe un test para cada bug.
- Rojo, verde, mejorar: escribir el test primero aclara lo que debe hacer el código.

Siguiente: [depurar en JavaScript](debogage.md). Compara con
[los tests en Ruby](../ruby/tests.md).
