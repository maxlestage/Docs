# Chuleta

Esta página resume de un vistazo la sintaxis de JavaScript vista en todo el recorrido, con Node.

## Bases

| Elemento | Ejemplo |
| --- | --- |
| Constante | `const edad = 30;` |
| Variable que cambia | `let total = 0;` |
| Texto | `"Hola"`, `'Hola'` |
| Plantilla | `` `Hola ${nombre}` `` |
| Booleanos | `true`, `false` |
| Ausencia | `undefined` (sin valor), `null` (vacío a propósito) |
| Comentario | `// una línea`, `/* varias */` |
| Mostrar | `console.log(a, b)` |
| Ejecutar | `node main.mjs` |

```javascript
const nombre = "Alice";
let edad = 30;
edad += 1;
console.log(`Hola ${nombre}, ${edad} años`);
console.log(typeof nombre, typeof edad, typeof undefined, typeof null);
```

```sortie
Hola Alice, 31 años
string number undefined object
```

Detalles: [primeros pasos](premiers-pas.md) y [los tipos](types.md).

## Números

| Necesidad | Sintaxis |
| --- | --- |
| Operaciones | `+ - * / ** %` |
| División entera | `Math.trunc(7 / 2)` |
| Redondear | `Math.round(x)`, `x.toFixed(2)` (texto) |
| Texto a número | `Number("12")`, `parseInt("12px")` |
| No es un número | `Number.isNaN(x)` |
| Enteros grandes | `10n ** 20n` (BigInt) |

```javascript
console.log(7 / 2, Math.trunc(7 / 2), 7 % 2, 2 ** 10);
console.log((0.1 + 0.2).toFixed(2), Number("12"), parseInt("12px"));
console.log(Number("abc"), Number.isNaN(Number("abc")));
```

```sortie
3.5 3 1 1024
0.30 12 12
NaN true
```

Detalles: [los números](nombres.md).

## Condiciones y bucles

| Necesidad | Sintaxis |
| --- | --- |
| Condición | `if (…) { … } else if (…) { … } else { … }` |
| Igualdad | `===`, `!==` (nunca `==`) |
| Lógica | `&&`, `\|\|`, `!` |
| Ternario | `x > 0 ? "sí" : "no"` |
| Varios casos | `switch (x) { case 1: …; break; default: … }` |
| Bucles | `for (let i = 0; i < n; i++)`, `while (…)`, `for (const x of lista)` |
| Control | `break`, `continue` |

```javascript
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

| Forma | Sintaxis |
| --- | --- |
| Declaración | `function cuadrado(x) { return x * x; }` |
| Flecha | `const cuadrado = (x) => x * x;` |
| Valor por defecto | `function saludar(nombre = "tú") { … }` |
| Resto de los argumentos | `function suma(...numeros) { … }` |
| Parámetro objeto | `function crear({ nombre, edad }) { … }` |

```javascript
const cuadrado = (x) => x * x;
function saludar(nombre = "tú") {
  return `¡Hola, ${nombre}!`;
}
function suma(...numeros) {
  return numeros.reduce((total, n) => total + n, 0);
}
console.log(cuadrado(4), saludar(), saludar("Léa"), suma(1, 2, 3));
```

```sortie
16 ¡Hola, tú! ¡Hola, Léa! 6
```

Detalles: [las funciones](fonctions.md).

## Texto

| Necesidad | Método |
| --- | --- |
| Longitud | `s.length` |
| Mayúsculas | `s.toUpperCase()`, `s.toLowerCase()` |
| Buscar | `s.includes("a")`, `s.startsWith("a")`, `s.indexOf("a")` |
| Cortar, volver a unir | `s.split(",")`, `lista.join(", ")` |
| Trozo | `s.slice(0, 3)`, `s.at(-1)` |
| Limpiar | `s.trim()` |
| Reemplazar | `s.replaceAll("a", "b")` |
| Alinear | `s.padStart(5)`, `s.padEnd(5)`, `"-".repeat(10)` |

```javascript
const s = "  Hola el mundo  ";
const limpio = s.trim();
console.log(limpio.toUpperCase());
console.log(limpio.split(" "), limpio.slice(0, 4), limpio.at(-1));
console.log(`[${"42".padStart(5)}]`, limpio.includes("mundo"));
```

```sortie
HOLA EL MUNDO
[ 'Hola', 'el', 'mundo' ] Hola o
[   42] true
```

Detalles: [el texto](texte.md).

## Arrays

| Necesidad | Sintaxis |
| --- | --- |
| Crear, leer | `const t = [1, 2, 3]`, `t[0]`, `t.at(-1)`, `t.length` |
| Añadir, quitar | `t.push(4)`, `t.pop()`, `t.unshift(0)`, `t.shift()` |
| Transformar | `t.map((x) => x * 2)` |
| Conservar | `t.filter((x) => x > 1)` |
| Reducir | `t.reduce((acc, x) => acc + x, 0)` |
| Buscar | `t.find(…)`, `t.includes(2)`, `t.some(…)`, `t.every(…)` |
| Ordenar sin modificar | `t.toSorted((a, b) => a - b)` |
| Copiar, fusionar | `[...t]`, `[...a, ...b]` |
| Desestructurar | `const [primero, ...resto] = t` |

```javascript
const notas = [12, 8, 15, 10];
console.log(notas.map((n) => n + 1));
console.log(notas.filter((n) => n >= 10));
console.log(notas.reduce((suma, n) => suma + n, 0));
console.log(notas.toSorted((a, b) => b - a), notas);
const [primera, ...otras] = notas;
console.log(primera, otras);
```

```sortie
[ 13, 9, 16, 11 ]
[ 12, 15, 10 ]
45
[ 15, 12, 10, 8 ] [ 12, 8, 15, 10 ]
12 [ 8, 15, 10 ]
```

Detalles: [las colecciones](collections.md).

## Objetos, Map y Set

| Necesidad | Sintaxis |
| --- | --- |
| Crear | `const p = { nombre: "Alice", edad: 30 }` |
| Leer | `p.nombre`, `p["nombre"]`, `p.direccion?.ciudad` |
| Valor por defecto | `p.ciudad ?? "desconocida"` |
| Recorrer | `Object.keys(p)`, `Object.values(p)`, `Object.entries(p)` |
| Copiar cambiando | `{ ...p, edad: 31 }` |
| Desestructurar | `const { nombre, edad } = p` |
| Diccionario | `new Map()`, `m.set(k, v)`, `m.get(k)`, `m.has(k)` |
| Conjunto sin duplicados | `new Set([1, 1, 2])` |

```javascript
const p = { nombre: "Alice", edad: 30 };
const mayor = { ...p, edad: 31 };
const { nombre, edad } = mayor;
console.log(nombre, edad, p.edad, p.ciudad ?? "desconocida");
for (const [clave, valor] of Object.entries(p)) {
  console.log(`${clave} = ${valor}`);
}
const vistos = new Set(["a", "b", "a"]);
const stock = new Map([["manzanas", 3]]);
console.log(vistos.size, stock.get("manzanas"), stock.has("peras"));
```

```sortie
Alice 31 30 desconocida
nombre = Alice
edad = 30
2 3 false
```

Detalles: [los objetos](objets.md).

## Clases y herencia

| Elemento | Sintaxis |
| --- | --- |
| Clase | `class Animal { constructor(nombre) { this.nombre = nombre; } }` |
| Objeto | `new Animal("Rex")` |
| Campo privado | `#secreto = 0` |
| Accesor | `get nombre() { … }` |
| Método de clase | `static crear() { … }` |
| Herencia | `class Perro extends Animal { … }` |
| Padre | `super(nombre)`, `super.hablar()` |
| Comprobar | `rex instanceof Animal` |

```javascript
class Animal {
  constructor(nombre) {
    this.nombre = nombre;
  }
  hablar() {
    return `${this.nombre} hace un ruido.`;
  }
}

class Perro extends Animal {
  hablar() {
    return `${this.nombre} ladra.`;
  }
}

const animales = [new Animal("Bob"), new Perro("Rex")];
for (const a of animales) console.log(a.hablar());
console.log(animales[1] instanceof Animal);
```

```sortie
Bob hace un ruido.
Rex ladra.
true
```

Detalles: [la herencia](heritage.md).

## Closures

Una función recuerda las variables del lugar donde fue creada.

```javascript
function multiplicador(factor) {
  return (x) => x * factor;
}
const doble = multiplicador(2);
const triple = multiplicador(3);
console.log(doble(5), triple(5));
```

```sortie
10 15
```

Detalles: [las closures](closures.md).

## Iteradores y generadores

| Elemento | Sintaxis |
| --- | --- |
| Recorrer un iterable | `for (const x of iterable)` |
| Índice y valor | `for (const [i, x] of t.entries())` |
| Generador | `function* gen() { yield 1; }` |
| Recuperarlo todo | `[...gen()]`, `Array.from(gen())` |

```javascript
function* cuentaAtras(n) {
  while (n > 0) {
    yield n;
    n -= 1;
  }
}
console.log([...cuentaAtras(3)]);
for (const [i, letra] of ["a", "b"].entries()) {
  console.log(i, letra);
}
```

```sortie
[ 3, 2, 1 ]
0 a
1 b
```

Detalles: [los iteradores](iterateurs.md).

## Errores

| Necesidad | Sintaxis |
| --- | --- |
| Atrapar | `try { … } catch (error) { … }` |
| Ejecutar siempre | `finally { … }` |
| Lanzar | `throw new Error("mensaje")` |
| Tipos comunes | `TypeError`, `RangeError`, `SyntaxError`, `ReferenceError` |
| Tu propio error | `class MiError extends Error { … }` |

```javascript
function comprobarEdad(edad) {
  if (edad < 0) throw new RangeError("edad negativa");
  return edad;
}

try {
  comprobarEdad(-1);
} catch (error) {
  console.log(`${error.name}: ${error.message}`);
} finally {
  console.log("Comprobación terminada");
}
```

```sortie
RangeError: edad negativa
Comprobación terminada
```

Detalles: [los errores](erreurs.md).

## Módulos

| Necesidad | Sintaxis |
| --- | --- |
| Exportar | `export function f() {}`, `export const X = 1;` |
| Exportación por defecto | `export default class Carrito {}` |
| Importar | `import { f, X } from "./utilidades.js";` |
| Importar el valor por defecto | `import Carrito from "./carrito.js";` |
| Módulo de Node | `import { readFile } from "node:fs/promises";` |

```javascript norun
// utilidades.js
export const IVA = 0.2;
export function precioConIva(base) {
  return base * (1 + IVA);
}

// main.js
import { precioConIva } from "./utilidades.js";
console.log(precioConIva(10));
```

Detalles: [los módulos](modules.md).

## Asincronía

| Necesidad | Sintaxis |
| --- | --- |
| Función asíncrona | `async function cargar() { … }` |
| Esperar una promesa | `const x = await promesa;` |
| Crear una promesa | `new Promise((resolve, reject) => { … })` |
| Más tarde | `setTimeout(f, 100)` |
| Todo en paralelo | `await Promise.all([p1, p2])` |

```javascript
const esperar = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function cargar(nombre, ms) {
  await esperar(ms);
  return `${nombre} cargado`;
}

console.log("Inicio");
const resultados = await Promise.all([cargar("perfil", 30), cargar("notas", 10)]);
console.log(resultados);
```

```sortie
Inicio
[ 'perfil cargado', 'notas cargado' ]
```

Detalles: [la asincronía](asynchrone.md).

## El DOM, en el navegador

| Necesidad | Sintaxis |
| --- | --- |
| Encontrar un elemento | `document.querySelector("#titulo")` |
| Varios elementos | `document.querySelectorAll(".tarjeta")` |
| Cambiar el texto | `el.textContent = "Hola"` |
| Clases CSS | `el.classList.add("activo")`, `toggle`, `remove` |
| Crear, añadir | `document.createElement("li")`, `lista.append(li)` |
| Reaccionar | `boton.addEventListener("click", () => { … })` |

```javascript norun
const boton = document.querySelector("#contar");
let clics = 0;
boton.addEventListener("click", () => {
  clics += 1;
  boton.textContent = `Pulsado ${clics} veces`;
});
```

Detalles: [el DOM](dom.md).

## Archivos y JSON

| Necesidad | Sintaxis |
| --- | --- |
| Leer | `await readFile("f.txt", "utf8")` |
| Escribir, añadir | `await writeFile("f.txt", texto)`, `await appendFile(…)` |
| Archivo ausente | `error.code === "ENOENT"` |
| Objeto a JSON | `JSON.stringify(obj, null, 2)` |
| JSON a objeto | `JSON.parse(texto)` |

```javascript
import { readFile, writeFile } from "node:fs/promises";

const datos = { nombre: "Alice", notas: [12, 15] };
await writeFile("alumno.json", JSON.stringify(datos, null, 2));
const releido = JSON.parse(await readFile("alumno.json", "utf8"));
console.log(releido.notas);

try {
  await readFile("ausente.txt", "utf8");
} catch (error) {
  console.log(error.code);
}
```

```sortie
[ 12, 15 ]
ENOENT
```

Detalles: [los archivos](fichiers.md) y [los formatos](formats.md).

## Expresiones regulares

| Elemento | Significado |
| --- | --- |
| `\d`, `\w`, `\s` | un dígito, una letra o dígito, un espacio |
| `+`, `*`, `?`, `{2,4}` | uno o más, cero o más, opcional, de 2 a 4 |
| `^`, `$` | inicio, fin |
| `( )`, `(?<nombre> )` | grupo, grupo con nombre |
| Indicadores | `g` (todos), `i` (sin distinguir mayúsculas) |
| Probar, buscar | `/re/.test(s)`, `s.match(/re/)`, `s.matchAll(/re/g)` |
| Reemplazar | `s.replace(/re/g, "…")` |

```javascript
const texto = "Pedido 42 el 2025-03-14, pedido 7 el 2025-04-01";
console.log(/\d+/.test(texto));
console.log(texto.match(/\d+/g).join(" "));
const fecha = texto.match(/(?<anio>\d{4})-(?<mes>\d{2})/);
console.log(fecha.groups.anio, fecha.groups.mes);
console.log("a  b   c".replace(/\s+/g, " "));
```

```sortie
true
42 2025 03 14 7 2025 04 01
2025 03
a b c
```

Detalles: [las expresiones regulares](regex.md).

## Fechas

| Necesidad | Sintaxis |
| --- | --- |
| Ahora | `new Date()` |
| Una fecha precisa | `new Date("2025-03-14T10:30:00Z")` |
| Partes | `d.getFullYear()`, `d.getMonth()` (0 = enero), `d.getDate()` |
| Formato ISO | `d.toISOString()` |
| Formato español | `d.toLocaleDateString("es-ES", { … })` |
| Diferencia | `d2 - d1` (en milisegundos) |

```javascript
const d = new Date("2025-03-14T10:30:00Z");
console.log(d.toISOString());
console.log(d.getUTCFullYear(), d.getUTCMonth() + 1, d.getUTCDate());
console.log(d.toLocaleDateString("es-ES", { dateStyle: "full", timeZone: "UTC" }));
const fin = new Date("2025-03-24T10:30:00Z");
console.log((fin - d) / (1000 * 60 * 60 * 24), "días");
```

```sortie
2025-03-14T10:30:00.000Z
2025 3 14
viernes, 14 de marzo de 2025
10 días
```

Detalles: [las fechas](dates.md).

## Línea de comandos y web

| Necesidad | Sintaxis |
| --- | --- |
| Argumentos | `process.argv.slice(2)` |
| Variable de entorno | `process.env.HOME` |
| Salir con un código | `process.exit(1)` |
| Error en pantalla | `console.error("…")` |
| Petición HTTP | `const resp = await fetch(url); await resp.json()` |
| Pequeño servidor | `http.createServer((req, res) => { … }).listen(3000)` |

```javascript norun
const [nombre = "desconocido"] = process.argv.slice(2);
console.log(`Hola ${nombre}`);

const respuesta = await fetch("https://api.example.com/alumnos");
if (!respuesta.ok) throw new Error(`HTTP ${respuesta.status}`);
const alumnos = await respuesta.json();
```

Detalles: [la línea de comandos](ligne-de-commande.md) y [la web](web.md).

## Pruebas

| Necesidad | Sintaxis |
| --- | --- |
| Importar | `import { test, describe } from "node:test";` |
| Aserciones | `import assert from "node:assert/strict";` |
| Igualdad | `assert.strictEqual(obtenido, esperado)` |
| Contenido | `assert.deepStrictEqual([1, 2], [1, 2])` |
| Error lanzado | `assert.throws(() => f(), RangeError)` |
| Ejecutar | `node --test` o `npm test` |

```javascript
import assert from "node:assert/strict";

const doble = (x) => x * 2;
assert.strictEqual(doble(4), 8);
assert.deepStrictEqual([1, 2].map(doble), [2, 4]);
assert.throws(() => JSON.parse("{uy"), SyntaxError);
console.log("3 comprobaciones superadas");
```

```sortie
3 comprobaciones superadas
```

Detalles: [probar tu código](tests.md).

## Depuración y estilo

| Necesidad | Herramienta |
| --- | --- |
| Nombres y valores | `console.log({ a, b })` |
| Lista de objetos | `console.table(lista)` |
| Objeto profundo | `console.dir(obj, { depth: null })` |
| Pausa | `debugger;` con las DevTools (`F12`) |
| Pausa con Node | `node --inspect-brk main.js` y luego `chrome://inspect` |
| Estilo | `const` por defecto, `===`, retornos anticipados, `?.` y `??` |
| Comprobar, formatear | `npx eslint src/`, `npx prettier --write src/` |

Detalles: [depurar](debogage.md) y [escribir buen JavaScript](style.md).

## Proyecto y herramientas

| Comando | Función |
| --- | --- |
| `node main.js` | ejecutar un programa |
| `node --watch main.js` | volver a ejecutar en cada cambio |
| `npm init -y` | crear `package.json` |
| `npm install nombre` | añadir un paquete (`dependencies`) |
| `npm install --save-dev nombre` | añadir una herramienta (`devDependencies`) |
| `npm install` | instalar los paquetes de un proyecto descargado |
| `npm start`, `npm test`, `npm run nombre` | ejecutar un script |
| `npx herramienta` | ejecutar una herramienta del proyecto |

Detalles: [organizar un proyecto](projet.md). Para practicar:
[los proyectos paso a paso](projets.md) y [los ejercicios resueltos](exercices.md). Compara
con [la chuleta de Ruby](../ruby/aide-memoire.md) y
[la chuleta de Rust](../rust/aide-memoire.md).
