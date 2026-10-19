# Probar tu código

Esta página muestra cómo escribir pruebas automáticas en TypeScript, con las herramientas que trae Node (`node:assert` y `node:test`), con Vitest, e incluso cómo probar los propios tipos.

> **Objetivos:**
>
> - entender para qué sirve una prueba automática;
> - comprobar resultados con `node:assert/strict`;
> - organizar y lanzar pruebas con `node:test`;
> - escribir las mismas pruebas con Vitest;
> - comprobar que un mal uso se rechaza de verdad con `// @ts-expect-error`.

## ¿Por qué probar?

Una [prueba](../debuter/glossaire.md#test) es un pequeño programa que comprueba otro
programa. Llama a tu código con valores conocidos y compara el resultado con el que
esperas.

TypeScript ya atrapa muchos errores: un texto en lugar de un número, una propiedad
mal escrita. Pero no sabe si tu **cálculo** es correcto. Una función `media` que
divide por el número equivocado tiene tipos perfectos… y un resultado falso.
Los tipos comprueban la **forma**, las pruebas comprueban el **sentido**.

Con pruebas:

- un solo comando lo comprueba todo, en unos segundos;
- si un cambio rompe algo, lo sabes enseguida;
- las pruebas muestran cómo usar tu código: una documentación que funciona.

> **Recuerda:** una prueba responde a una pregunta sencilla: «con esta entrada,
> ¿obtengo de verdad esta salida?».

## Comprobar con node:assert/strict

El módulo `node:assert/strict` ofrece funciones de comprobación, llamadas
**aserciones**. Si la comprobación tiene éxito, no pasa nada. Si falla, un error
detiene el programa.

```typescript
import assert from "node:assert/strict";

function doble(n: number): number {
  return n * 2;
}

assert.equal(doble(4), 8);
assert.equal(doble(0), 0);
assert.ok(doble(3) > 5);
console.log("Todas las comprobaciones pasan.");
```

```sortie
Todas las comprobaciones pasan.
```

`assert.equal(real, esperado)` se lee: «comprueba que `doble(4)` vale de verdad `8`».
El valor calculado va primero, el valor esperado después.

### Las aserciones más útiles

| Aserción | Comprueba que… |
|-----------|--------------|
| `assert.equal(a, b)` | `a` y `b` son idénticos (`===`) |
| `assert.deepEqual(a, b)` | dos objetos o arrays tienen el mismo contenido |
| `assert.ok(valor)` | el valor es verdadero |
| `assert.throws(() => f(), /mensaje/)` | `f()` lanza un error cuyo mensaje coincide |
| `await assert.rejects(promesa)` | una promesa es rechazada |

`deepEqual` compara el **contenido**: dos arrays creados por separado nunca son el
mismo objeto, pero pueden contener los mismos valores.

```typescript
import assert from "node:assert/strict";

function ordenar(numeros: number[]): number[] {
  return [...numeros].sort((a, b) => a - b);
}

assert.deepEqual(ordenar([3, 1, 2]), [1, 2, 3]);
assert.deepEqual({ nombre: "Ada", edad: 36 }, { nombre: "Ada", edad: 36 });
console.log("Contenidos idénticos.");
```

```sortie
Contenidos idénticos.
```

### Cuando una aserción falla

El mensaje de error muestra el valor obtenido y el valor esperado. Aquí lo atrapamos
para mostrarlo:

```typescript
import assert from "node:assert/strict";

function media(notas: number[]): number {
  return notas.reduce((a, b) => a + b, 0) / (notas.length + 1);
}

try {
  assert.equal(media([10, 14, 18]), 14);
} catch (e) {
  if (e instanceof assert.AssertionError) {
    console.log(e.message);
  }
}
```

```sortie
Expected values to be strictly equal:

10.5 !== 14
```

La prueba encontró el bug: dividimos por `notas.length + 1` en lugar de
`notas.length`. TypeScript, en cambio, no había visto nada.

### Probar los errores y las promesas

`assert.throws` comprueba que una función lanza de verdad un error. Para código
asíncrono (ver [la programación asíncrona](asynchrone.md)), `assert.rejects` hace lo
mismo con una promesa, y se usa con `await`:

```typescript
import assert from "node:assert/strict";

function raiz(n: number): number {
  if (n < 0) {
    throw new RangeError("número negativo");
  }
  return Math.sqrt(n);
}

async function cargar(id: number): Promise<string> {
  if (id <= 0) {
    throw new Error("identificador no válido");
  }
  return `elemento ${id}`;
}

assert.equal(raiz(9), 3);
assert.throws(() => raiz(-1), RangeError);
assert.throws(() => raiz(-1), /negativo/);

assert.equal(await cargar(2), "elemento 2");
await assert.rejects(cargar(0), /identificador no válido/);
console.log("Errores bien detectados.");
```

```sortie
Errores bien detectados.
```

## Organizar las pruebas con node:test

Unas aserciones sueltas están bien para empezar. Pero al primer error, todo se
detiene, y no sabes qué otras pruebas habrían fallado. El módulo `node:test`, que
viene con Node, organiza las aserciones en **pruebas** con nombre, agrupadas con
`describe`:

- `describe("nombre", () => { … })` agrupa las pruebas de una misma función;
- `test("lo que se comprueba", () => { … })` es una prueba: una frase que describe el
  comportamiento esperado.

```typescript
import { describe, test } from "node:test";
import assert from "node:assert/strict";

function media(notas: number[]): number {
  if (notas.length === 0) {
    throw new Error("lista vacía");
  }
  return notas.reduce((a, b) => a + b, 0) / notas.length;
}

describe("media", () => {
  test("calcula la media de varias notas", () => {
    assert.equal(media([10, 14, 18]), 14);
  });

  test("funciona con una sola nota", () => {
    assert.equal(media([12]), 12);
  });

  test("rechaza una lista vacía", () => {
    assert.throws(() => media([]), /lista vacía/);
  });
});
```

Una prueba también puede ser `async`: escribe `test("…", async () => { … })` y usa
`await` dentro.

### Lanzar las pruebas

Guarda este código en `src/media.test.ts`. Compilas (ver
[configurar TypeScript](configuration.md)), y luego `node --test` lanza todos los
archivos de prueba:

```sh
npx tsc
node --test "dist/**/*.test.js"
```

```text
▶ media
  ✔ calcula la media de varias notas (0.736478ms)
  ✔ funciona con una sola nota (0.23265ms)
  ✔ rechaza una lista vacía (0.831555ms)
✔ media (3.066365ms)
ℹ tests 3
ℹ suites 1
ℹ pass 3
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 121.217153
```

Cada `✔` es una prueba superada. Las duraciones entre paréntesis cambian en cada
ejecución.

Si una prueba falla, se marca con `✖`, y el detalle aparece al final, con la línea
culpable:

```text
▶ media
  ✖ calcula la media de varias notas (2.054574ms)
  ✔ funciona con una sola nota (0.163259ms)
  ✔ rechaza una lista vacía (0.505934ms)
✖ media (4.078187ms)
ℹ tests 3
ℹ suites 1
ℹ pass 2
ℹ fail 1
...

✖ failing tests:

test at dist/media.test.js:10:5
✖ calcula la media de varias notas (2.054574ms)
  AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:

  14 !== 15
```

Las demás pruebas se lanzaron de todos modos: ves de un vistazo todo lo que funciona y
todo lo que no.

> **Consejo:** añade `"test": "tsc && node --test \"dist/**/*.test.js\""` en la parte
> `scripts` de `package.json`. Después, basta con `npm test`.

## Vitest

**Vitest** es una herramienta de pruebas muy usada en los proyectos TypeScript, sobre
todo en la web. Su gran ventaja: lee directamente los archivos `.ts`, sin compilarlos
antes. Instálala en el proyecto:

```sh
npm install --save-dev vitest
```

El código que se va a probar, en `src/carrito.ts`:

```typescript norun
export interface Articulo {
  precio: number;
  cantidad: number;
}

export function total(articulos: Articulo[]): number {
  return articulos.reduce((suma, a) => suma + a.precio * a.cantidad, 0);
}
```

Las pruebas, en `src/carrito.test.ts`. En lugar de `assert`, Vitest usa
`expect(valor)`, seguido de una comprobación: `toBe` (idéntico), `toEqual` (mismo
contenido), `toThrow` (lanza un error).

```typescript norun
import { describe, it, expect } from "vitest";
import { total } from "./carrito.js";

describe("total", () => {
  it("suma los artículos", () => {
    expect(total([{ precio: 2, cantidad: 3 }, { precio: 4, cantidad: 1 }])).toBe(10);
  });

  it("vale 0 para un carrito vacío", () => {
    expect(total([])).toBe(0);
  });

  it("devuelve un objeto nuevo bien formado", () => {
    const resumen = { numero: 2, total: total([{ precio: 5, cantidad: 2 }]) };
    expect(resumen).toEqual({ numero: 2, total: 10 });
  });
});
```

```sh
npx vitest run
```

```text
 RUN  v3.2.4 /home/tu/mi-proyecto

 ✓ src/carrito.test.ts (3 tests) 6ms

 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  21:12:24
   Duration  807ms (transform 181ms, setup 0ms, collect 109ms, tests 6ms, environment 0ms, prepare 156ms)
```

Sin `run`, `npx vitest` se queda abierto y vuelve a lanzar las pruebas cada vez que
guardas, como el modo watch de `tsc`.

> **Nota:** `it` y `test` son sinónimos. Con nombres en inglés,
> `it("adds up the items")` se lee como una frase: «it adds up the items» («suma los
> artículos»).

## Probar los tipos con @ts-expect-error

Con TypeScript, también puedes comprobar que un **mal** uso se **rechaza** de verdad.
El comentario `// @ts-expect-error` dice: «la línea siguiente debe provocar un error
de tipo». Dos casos:

- la línea contiene de verdad un error: se ignora, todo va bien;
- la línea **no** contiene ningún error: es `tsc` quien señala el comentario inútil.

Así que es una prueba: si algún día alguien debilita tus tipos, la compilación falla.

```typescript
interface Usuario {
  nombre: string;
  edad: number;
}

function crear(nombre: string, edad: number): Usuario {
  return { nombre, edad };
}

function comprobarTipos(): void {
  // @ts-expect-error: la edad debe ser un número
  crear("Ada", "36");

  // @ts-expect-error: falta la edad
  crear("Ada");

  const u = crear("Ada", 36);
  // @ts-expect-error: la propiedad email no existe
  console.log(u.email);
}

console.log(crear("Grace", 45));
```

```sortie
{ nombre: 'Grace', edad: 45 }
```

La función `comprobarTipos` nunca se llama: solo sirve al compilador. Ejecuta
`tsc --noEmit` para ejecutar estas «pruebas de tipos».

> **Atención:** no uses `@ts-expect-error` (ni `@ts-ignore`) para silenciar un error
> real de tu programa. Mejor corrígelo.

## ¿Qué hay que probar?

No puedes probarlo todo. Céntrate en:

- **el caso normal**: una entrada corriente y su resultado esperado;
- **los casos límite**: lista vacía, cero, número negativo, texto vacío, un solo elemento;
- **los errores**: una entrada no válida debe lanzar el error correcto;
- **los bugs ya encontrados**: cada bug corregido merece una prueba, para que no vuelva.

En cambio, no hace falta probar lo que TypeScript ya garantiza: que una función
`(n: number) => number` devuelve un número, por ejemplo.

Un buen hábito: escribir la prueba **antes** que el código. Escribe una prueba que
falle (la función todavía no existe), luego el código más sencillo que la haga pasar,
y después mejora el código manteniendo las pruebas en verde.

## Errores frecuentes

### Comparar dos objetos con equal

```typescript norun
import assert from "node:assert/strict";

assert.equal({ x: 1 }, { x: 1 });
```

```text
AssertionError [ERR_ASSERTION]: Values have same structure but are not reference-equal:

{
  x: 1
}
```

`equal` comprueba que es **el mismo objeto**. Para comparar el contenido, usa
`assert.deepEqual` (o `toEqual` con Vitest).

### Olvidar await en una prueba asíncrona

```typescript norun
import assert from "node:assert/strict";

async function cargar(): Promise<number> {
  return 3;
}

assert.equal(cargar(), 3);
```

```text
AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:
+ actual - expected

+ Promise {
+   3
+ }
- 3
```

Estás comparando una promesa con un número. Escribe
`assert.equal(await cargar(), 3);`.

### Un @ts-expect-error que no espera nada

```typescript norun
function doble(n: number): number {
  return n * 2;
}

// @ts-expect-error
doble(4);
```

```text
main.ts(5,1): error TS2578: Unused '@ts-expect-error' directive.
```

`doble(4)` es correcto: no hay ningún error que esperar. Quita el comentario, o ponlo
encima de una llamada realmente no válida, como `doble("4")`.

## Ponte a prueba

**Pregunta 1:** ¿qué diferencia hay entre `assert.equal` y `assert.deepEqual`?

> **Respuesta**
>
> `equal` comprueba que los dos valores son idénticos (`===`): para objetos, que sean
> el mismo objeto. `deepEqual` compara el contenido, propiedad por propiedad.

**Pregunta 2:** ¿por qué probar una función cuyos tipos ya comprueba TypeScript?

> **Respuesta**
>
> Los tipos comprueban la forma de los datos, no que el cálculo sea correcto. Una
> función puede tener tipos perfectos y devolver un resultado equivocado.

**Pregunta 3:** escribe una función `iniciales(nombre: string): string` («Ada
Lovelace» da «AL») y compruébala con dos aserciones, una de ellas sobre un texto
vacío.

> **Respuesta**
>
> ```typescript
> import assert from "node:assert/strict";
>
> function iniciales(nombre: string): string {
>   return nombre
>     .split(" ")
>     .filter((palabra) => palabra !== "")
>     .map((palabra) => palabra[0]?.toUpperCase() ?? "")
>     .join("");
> }
>
> assert.equal(iniciales("Ada Lovelace"), "AL");
> assert.equal(iniciales(""), "");
> console.log("OK");
> ```
>
> ```sortie
> OK
> ```

## En resumen

- Los tipos comprueban la forma del código; las pruebas comprueban que hace el cálculo
  correcto.
- `node:assert/strict`: `equal`, `deepEqual`, `ok`, `throws`, `rejects`.
- `node:test` agrupa las pruebas con `describe` y `test`; `node --test` las lanza.
- Vitest lee directamente TypeScript y usa `expect(...).toBe(...)`.
- `// @ts-expect-error` comprueba que el compilador rechaza de verdad un mal uso.
- Prueba el caso normal, los casos límite, los errores y los bugs ya corregidos.

Siguiente: [depurar un programa](debogage.md). Compara con
[las pruebas en Ruby](../ruby/tests.md).
