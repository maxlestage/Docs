# Chuleta

Esta página resume de un vistazo la sintaxis de TypeScript vista en todo el recorrido, con un ejemplo corto para cada tema.

## Tipos básicos

| Elemento | Ejemplo |
| --- | --- |
| Constante, variable | `const nombre = "Alice";`, `let edad = 30;` |
| Anotación | `let edad: number = 30;` |
| Números | `number`: `42`, `3.14`, `NaN`; `bigint`: `10n` |
| Texto | `string`: `"a"`, `'a'`, `` `Hola ${nombre}` `` |
| Booleano | `boolean`: `true`, `false` |
| Ausencia | `undefined`, `null` |
| Cualquier cosa, comprobada | `unknown` (estréchalo antes de usarlo) |
| Cualquier cosa, sin comprobar | `any` (evítalo) |
| No devuelve nada | `void` |
| Nunca ocurre | `never` |
| Comentario | `// línea`, `/* bloque */` |
| Mostrar | `console.log(a, b)` |

```typescript
const nombre = "Alice";
let edad: number = 30;
edad += 1;
const desconocido: unknown = "texto";
if (typeof desconocido === "string") console.log(desconocido.length);
console.log(`${nombre} tiene ${edad} años`, typeof edad, 7 / 2, Math.floor(7 / 2));
```

```sortie
5
Alice tiene 31 años number 3.5 3
```

Detalles: [primeros pasos](premiers-pas.md).

## Condiciones y bucles

| Necesidad | Sintaxis |
| --- | --- |
| Condición | `if (…) { … } else if (…) { … } else { … }` |
| Ternario | `x > 0 ? "sí" : "no"` |
| Igualdad estricta | `===`, `!==` (nunca `==`) |
| Lógica | `&&`, `\|\|`, `!` |
| Valor por defecto | `x ?? "defecto"` (si `null` o `undefined`) |
| Acceso prudente | `obj?.prop`, `f?.()` |
| Varios casos | `switch (x) { case 1: …; break; default: … }` |
| Bucles | `for (let i = 0; i < n; i++)`, `while (…)`, `for (const x of arr)` |
| Índice y valor | `for (const [i, x] of arr.entries())` |
| Control | `break`, `continue` |

```typescript
for (let n = 1; n <= 5; n++) {
  if (n === 2) continue;
  console.log(n % 2 === 0 ? `${n} par` : `${n} impar`);
}
```

```sortie
1 impar
3 impar
4 par
5 impar
```

Detalles: [condiciones y bucles](conditions-boucles.md).

## Funciones

| Elemento | Sintaxis |
| --- | --- |
| Declaración | `function f(a: number, b: number): number { return a + b; }` |
| Flecha | `const f = (a: number): number => a * 2;` |
| Parámetro opcional | `function f(nombre?: string)` |
| Valor por defecto | `function f(saludo = "Hola")` |
| Resto | `function suma(...n: number[])` |
| Tipo de función | `type Op = (a: number, b: number) => number;` |
| Sin retorno | `function log(m: string): void` |

```typescript
type Op = (a: number, b: number) => number;

function saludar(nombre: string, saludo = "Hola", educado?: boolean): string {
  const texto = `${saludo} ${nombre}`;
  return educado ? `${texto}, encantado.` : texto;
}
const suma = (...n: number[]): number => n.reduce((a, b) => a + b, 0);
const por: Op = (a, b) => a * b;

console.log(saludar("Alice"));
console.log(saludar("Bob", "Buenas", true));
console.log(suma(1, 2, 3), por(6, 7));
```

```sortie
Hola Alice
Buenas Bob, encantado.
6 42
```

Detalles: [las funciones](fonctions.md).

## Texto

| Necesidad | Método |
| --- | --- |
| Longitud | `s.length` |
| Mayúsculas, minúsculas | `s.toUpperCase()`, `s.toLowerCase()` |
| Espacios alrededor | `s.trim()` |
| Contiene, empieza por | `s.includes("x")`, `s.startsWith("x")` |
| Cortar, volver a unir | `s.split(",")`, `arr.join(", ")` |
| Reemplazar | `s.replaceAll("a", "b")` |
| Trozo | `s.slice(0, 3)` |
| Alinear | `s.padEnd(10)`, `s.padStart(5, "0")` |
| Número → texto | `String(n)`, `n.toFixed(2)` |
| Texto → número | `Number("12")` (`NaN` si no es válido) |

```typescript
const s = "  Hola mundo bonito  ";
console.log(s.trim().toUpperCase());
console.log(s.trim().split(" "));
console.log("7".padStart(3, "0"), (2 / 3).toFixed(2), Number("12") + 1);
```

```sortie
HOLA MUNDO BONITO
[ 'Hola', 'mundo', 'bonito' ]
007 0.67 13
```

Detalles: [el texto](texte.md).

## Arrays, tuplas y Map

| Elemento | Sintaxis |
| --- | --- |
| Array | `const a: number[] = [1, 2];` o `Array<number>` |
| Solo lectura | `readonly number[]` |
| Tupla | `const p: [string, number] = ["Alice", 30];` |
| Añadir, quitar | `a.push(x)`, `a.pop()` |
| Transformar | `a.map((x) => x * 2)` |
| Filtrar | `a.filter((x) => x > 0)` |
| Buscar | `a.find(…)` → `T \| undefined`, `a.includes(x)` |
| Reducir | `a.reduce((acc, x) => acc + x, 0)` |
| Ordenar una copia | `[...a].sort((x, y) => x - y)` |
| Desestructurar | `const [primero, ...resto] = a;` |
| Diccionario | `new Map<string, number>()`, `get`, `set`, `has` |
| Conjunto | `new Set<string>()`, `add`, `has`, `size` |

```typescript
const notas = [12, 9, 15, 18];
const [primera, ...otras] = notas;
console.log(notas.filter((n) => n >= 10).map((n) => n * 2));
console.log([...notas].sort((a, b) => b - a), primera, otras.length);

const persona: [string, number] = ["Alice", 30];
const edades = new Map<string, number>([["Alice", 30]]);
edades.set("Bob", 25);
console.log(persona[0], edades.get("Bob"), edades.get("Zoe") ?? "desconocido");
```

```sortie
[ 24, 30, 36 ]
[ 18, 15, 12, 9 ] 12 3
Alice 25 desconocido
```

Detalles: [las colecciones](collections.md).

## Objetos e interfaces

| Elemento | Sintaxis |
| --- | --- |
| Interfaz | `interface Punto { x: number; y: number }` |
| Alias de tipo | `type Punto = { x: number; y: number };` |
| Propiedad opcional | `nombre?: string` |
| Solo lectura | `readonly id: number` |
| Extensión | `interface Punto3D extends Punto { z: number }` |
| Diccionario | `Record<string, number>` o `{ [clave: string]: number }` |
| Desestructurar | `const { x, y } = punto;` |
| Copiar modificando | `{ ...punto, x: 5 }` |
| Claves, valores | `Object.keys(o)`, `Object.entries(o)` |

```typescript
interface Persona {
  readonly id: number;
  nombre: string;
  email?: string;
}

const alice: Persona = { id: 1, nombre: "Alice" };
const modificada = { ...alice, nombre: "Alice B." };
const { nombre, email } = modificada;
console.log(nombre, email ?? "sin email", alice.nombre);
console.log(Object.entries({ mates: 15, arte: 12 }));
```

```sortie
Alice B. sin email Alice
[ [ 'mates', 15 ], [ 'arte', 12 ] ]
```

Detalles: [los objetos](objets.md).

## Uniones y estrechamiento

| Necesidad | Sintaxis |
| --- | --- |
| Unión | `string \| number` |
| Puede faltar | `string \| undefined` |
| Comprobar un tipo simple | `typeof x === "string"` |
| Comprobar una clase | `x instanceof Date` |
| Comprobar una propiedad | `"radio" in forma` |
| Comprobar la presencia | `if (x !== undefined)`, `if (x)` |
| Unión discriminada | `{ tipo: "a"; … } \| { tipo: "b"; … }` y luego `switch (v.tipo)` |
| Guarda de tipo | `function esX(v: unknown): v is X` |
| Exhaustividad | `const resto: never = v;` en el `default` |

```typescript
type Forma = { tipo: "circulo"; radio: number } | { tipo: "cuadrado"; lado: number };

function area(f: Forma): number {
  switch (f.tipo) {
    case "circulo":
      return Math.PI * f.radio ** 2;
    case "cuadrado":
      return f.lado ** 2;
    default: {
      const resto: never = f;
      return resto;
    }
  }
}

function describir(v: string | number): string {
  return typeof v === "string" ? v.toUpperCase() : v.toFixed(1);
}

console.log(area({ tipo: "cuadrado", lado: 3 }), describir("ok"), describir(2));
```

```sortie
9 OK 2.0
```

Detalles: [las uniones](unions.md).

## Clases

| Elemento | Sintaxis |
| --- | --- |
| Clase | `class Gato { … }`, `new Gato("Tom")` |
| Propiedad | `nombre: string;` (hay que inicializarla) |
| Atajo del constructor | `constructor(private nombre: string) {}` |
| Visibilidad | `public` (por defecto), `private`, `protected`, `#privado` |
| Solo lectura | `readonly id: number` |
| Herencia | `class Gato extends Animal`, `super(…)` |
| Contrato | `class Gato implements Animal` |
| Estático | `static crear(): Gato` |
| Accesor | `get nombre(): string { … }` |

```typescript
interface Animal {
  maullar(): string;
}

class Gato implements Animal {
  static cantidad = 0;
  #vidas = 7;

  constructor(readonly nombre: string) {
    Gato.cantidad++;
  }

  get vidas(): number {
    return this.#vidas;
  }

  maullar(): string {
    return `${this.nombre}: miau`;
  }
}

const tom = new Gato("Tom");
console.log(tom.maullar(), tom.vidas, Gato.cantidad);
```

```sortie
Tom: miau 7 1
```

Detalles: [las clases](types.md).

## Genéricos

| Elemento | Sintaxis |
| --- | --- |
| Función genérica | `function primero<T>(a: T[]): T \| undefined` |
| Tipo genérico | `type Caja<T> = { valor: T };` |
| Restricción | `<T extends { length: number }>` |
| Varios parámetros | `function par<A, B>(a: A, b: B): [A, B]` |
| Valor por defecto | `<T = string>` |
| Precisar en la llamada | `primero<number>([])` (rara vez útil) |

```typescript
function primero<T>(a: readonly T[]): T | undefined {
  return a[0];
}

function masLargo<T extends { length: number }>(a: T, b: T): T {
  return a.length >= b.length ? a : b;
}

type Caja<T> = { valor: T };
const c: Caja<number> = { valor: 42 };

console.log(primero(["a", "b"]), masLargo("gato", "oso"), masLargo([1, 2], [3]), c.valor);
```

```sortie
a gato [ 1, 2 ] 42
```

Detalles: [los genéricos](generiques.md).

## Tipos literales

| Elemento | Sintaxis |
| --- | --- |
| Literal | `type Si = "sí";` |
| Unión de literales | `type Talla = "S" \| "M" \| "L";` |
| Congelar un valor | `const T = ["S", "M"] as const;` |
| Unión a partir de un array | `type Talla = (typeof T)[number];` |
| Tipo plantilla | `` type Id = `user-${number}`; `` |
| Comprobar sin ensanchar | `const c = { … } satisfies Config;` |

```typescript
const TALLAS = ["S", "M", "L"] as const;
type Talla = (typeof TALLAS)[number];
type Id = `user-${number}`;

const t: Talla = "M";
const id: Id = "user-42";
console.log(t, id, TALLAS.length);
```

```sortie
M user-42 3
```

Detalles: [los tipos literales](litteraux.md).

## Tipos utilitarios y tipos avanzados

| Tipo | Efecto |
| --- | --- |
| `Partial<T>` | todas las propiedades opcionales |
| `Required<T>` | todas las propiedades obligatorias |
| `Readonly<T>` | todas las propiedades de solo lectura |
| `Pick<T, "a" \| "b">` | solo algunas propiedades |
| `Omit<T, "a">` | todas salvo algunas |
| `Record<K, V>` | un objeto con claves `K` y valores `V` |
| `ReturnType<typeof f>` | el tipo de retorno de una función |
| `Awaited<P>` | el tipo que se obtiene tras `await` |
| `keyof T` | la unión de los nombres de propiedades |
| `T[K]` | el tipo de una propiedad |
| `typeof x` | el tipo de un valor |
| `T extends U ? A : B` | tipo condicional |
| `{ [K in keyof T]: … }` | tipo transformado (*mapped*) |

```typescript
interface Persona {
  nombre: string;
  edad: number;
}

function modificar(p: Persona, cambios: Partial<Persona>): Persona {
  return { ...p, ...cambios };
}

type Resumen = Pick<Persona, "nombre">;
type Clave = keyof Persona;

const alice: Readonly<Persona> = { nombre: "Alice", edad: 30 };
const r: Resumen = { nombre: alice.nombre };
const clave: Clave = "edad";
console.log(modificar(alice, { edad: 31 }), r, alice[clave]);
```

```sortie
{ nombre: 'Alice', edad: 31 } { nombre: 'Alice' } 30
```

Detalles: [los tipos avanzados](types-avances.md).

## Errores

| Necesidad | Sintaxis |
| --- | --- |
| Lanzar | `throw new Error("mensaje")` |
| Capturar | `try { … } catch (e) { … } finally { … }` |
| Tipo de `e` | `unknown`: comprueba `e instanceof Error` |
| Clase de error | `class MiError extends Error { … }` |
| Resultado sin excepción | `type R<T> = { ok: true; value: T } \| { ok: false; error: string }` |

```typescript
class EntradaInvalida extends Error {
  override name = "EntradaInvalida";
}

function leerEdad(texto: string): number {
  const n = Number(texto);
  if (!Number.isInteger(n)) throw new EntradaInvalida(`«${texto}» no es un entero`);
  return n;
}

for (const entrada of ["34", "abc"]) {
  try {
    console.log(leerEdad(entrada));
  } catch (e) {
    if (e instanceof Error) console.log(`${e.name}: ${e.message}`);
  }
}
```

```sortie
34
EntradaInvalida: «abc» no es un entero
```

Detalles: [los errores](erreurs.md).

## Asincronía

| Necesidad | Sintaxis |
| --- | --- |
| Función asíncrona | `async function f(): Promise<string>` |
| Esperar | `const x = await f();` |
| Varias en paralelo | `await Promise.all([f(), g()])` |
| Aunque algunas fallen | `await Promise.allSettled([…])` |
| La primera en terminar | `await Promise.race([…])` |
| Esperar un tiempo | `await setTimeout(100)` con `node:timers/promises` |
| Error asíncrono | `try { await f(); } catch (e) { … }` |

```typescript
import { setTimeout as esperar } from "node:timers/promises";

async function cargar(nombre: string, ms: number): Promise<string> {
  await esperar(ms);
  return `${nombre} cargado`;
}

const resultados = await Promise.all([cargar("perfil", 20), cargar("historial", 10)]);
console.log(resultados);
```

```sortie
[ 'perfil cargado', 'historial cargado' ]
```

`Promise.all` mantiene el orden del array, aunque `historial` termine primero. Detalles:
[la asincronía](asynchrone.md).

## Módulos

| Necesidad | Sintaxis |
| --- | --- |
| Exportar | `export function f() {}`, `export const X = 1;` |
| Exportación por defecto | `export default class Carrito {}` |
| Importar | `import { f, X } from "./utiles.js";` |
| Renombrar | `import { f as g } from "./utiles.js";` |
| Importarlo todo | `import * as utiles from "./utiles.js";` |
| Importar un tipo | `import type { Persona } from "./tipos.js";` |
| Módulo de Node | `import { readFile } from "node:fs/promises";` |

```typescript norun
// utiles.ts
export interface Persona {
  nombre: string;
}
export function saludar(p: Persona): string {
  return `Hola ${p.nombre}`;
}

// main.ts
import { saludar, type Persona } from "./utiles.js";
const alice: Persona = { nombre: "Alice" };
console.log(saludar(alice));
```

El import termina en `.js`, el nombre del archivo **compilado**. Detalles:
[los módulos](modules.md).

## Archivos y JSON

| Necesidad | Sintaxis (`node:fs/promises`) |
| --- | --- |
| Escribir | `await writeFile("f.txt", texto, "utf8")` |
| Leer | `await readFile("f.txt", "utf8")` |
| Añadir | `await appendFile("f.txt", texto)` |
| Archivo ausente | error con código `"ENOENT"` |
| Hacia JSON | `JSON.stringify(v, null, 2)` |
| Desde JSON | `JSON.parse(t)`: guárdalo en un `unknown` |
| Argumentos | `process.argv.slice(2)` |

```typescript
import { appendFile, readFile, writeFile } from "node:fs/promises";

await writeFile("f.txt", "uno\n", "utf8");
await appendFile("f.txt", "dos\n");
console.log((await readFile("f.txt", "utf8")).trim().split("\n"));

const datos: unknown = JSON.parse(JSON.stringify({ ok: true }));
if (typeof datos === "object" && datos !== null && "ok" in datos) {
  console.log(datos.ok);
}
```

```sortie
[ 'uno', 'dos' ]
true
```

Detalles: [los archivos](fichiers.md), [los formatos](formats.md).

## tsconfig.json

| Opción | Función |
| --- | --- |
| `"strict": true` | todas las comprobaciones estrictas (imprescindible) |
| `"target": "es2022"` | versión de JavaScript generada |
| `"module": "nodenext"` | módulos al estilo de Node |
| `"rootDir": "src"` | carpeta de las fuentes |
| `"outDir": "dist"` | carpeta del JavaScript compilado |
| `"sourceMap": true` | archivos `.js.map` para depurar |
| `"noUncheckedIndexedAccess": true` | `a[i]` es `T \| undefined` |
| `"noEmit": true` | comprobar sin escribir nada |
| `"skipLibCheck": true` | no volver a comprobar los paquetes instalados |
| `"include": ["src"]` | archivos que compilar |

Detalles: [la configuración](configuration.md).

## Comandos

| Comando | Función |
| --- | --- |
| `npm init -y` | crear un `package.json` |
| `npm install -D typescript @types/node tsx` | instalar las herramientas |
| `npx tsc` | comprobar y compilar según `tsconfig.json` |
| `npx tsc --noEmit` | comprobar solo los tipos |
| `npx tsc --init` | crear un `tsconfig.json` |
| `npx tsx archivo.ts` | ejecutar directamente (sin comprobar los tipos) |
| `npx tsx watch archivo.ts` | volver a lanzar con cada cambio |
| `node --enable-source-maps dist/main.js` | lanzar con las líneas de TypeScript en los errores |
| `npx tsx --test` | lanzar los tests de `node:test` |
| `npx eslint src/`, `npx prettier --write src/` | comprobar y dar formato |

Detalles: [organizar un proyecto](projet.md), [los tests](tests.md),
[depurar](debogage.md), [escribir buen TypeScript](style.md). Para practicar:
[los proyectos paso a paso](projets.md) y [los ejercicios resueltos](exercices.md).
Compara con [la chuleta de Ruby](../ruby/aide-memoire.md) y
[la chuleta de Rust](../rust/aide-memoire.md).
