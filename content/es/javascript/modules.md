# Módulos y npm

Esta página muestra cómo dividir un programa en varios archivos con `import` y `export`, y luego cómo usar los paquetes de otros desarrolladores con npm.

> **Objetivos:**
>
> - sabrás exportar e importar código entre archivos;
> - sabrás distinguir una exportación por defecto de una exportación con nombre;
> - sabrás crear un proyecto con `npm init` y leer un `package.json`;
> - sabrás instalar un paquete y ejecutar scripts con `npm run`;
> - sabrás por qué `node_modules` no va en Git.

## ¿Por qué dividir en módulos?

Un programa de 50 líneas cabe en un archivo. Uno de 5000 líneas, no: te pierdes en él.
Entonces se divide en [módulos](../debuter/glossaire.md#module): archivos que tienen
cada uno un papel preciso.

Piensa en una cocina: las especias en un armario, la vajilla en otro. Cada armario tiene
una etiqueta, y vas a buscar lo que necesitas.

En JavaScript moderno, **cada archivo es un módulo**. Lo que se declara en un archivo se
queda privado en él, salvo lo que **exporta** explícitamente.

## Los módulos integrados de Node

Node ofrece módulos ya listos, cuyo nombre empieza por `node:`. Se importan con
`import { ... } from "..."` al principio del archivo.

```javascript
import { join, basename, extname } from "node:path";

const ruta = join("fotos", "vacaciones", "playa.jpg");
console.log(ruta);
console.log(basename(ruta));
console.log(extname(ruta));
```

```sortie
fotos/vacaciones/playa.jpg
playa.jpg
.jpg
```

Entre las llaves, eliges lo que quieres tomar del módulo. También puedes renombrar un
elemento con `as`, o tomarlo todo de una vez con `* as`:

```javascript
import { join as unir } from "node:path";
import * as path from "node:path";

console.log(unir("a", "b"));
console.log(path.dirname("/casa/cocina/horno.txt"));
```

```sortie
a/b
/casa/cocina
```

## Exportar e importar tus funciones

Pasemos a tus propios módulos. Hay dos formas de exportar.

### La exportación con nombre

Escribes `export` delante de cada declaración que quieres compartir. Un archivo puede
tener tantas exportaciones con nombre como quiera. Quien importa debe usar **el mismo
nombre**, entre llaves.

```javascript norun
// maths.js
export const VERSION = "1.0";

export function suma(numeros) {
  return numeros.reduce((total, n) => total + n, 0);
}

export function promedio(numeros) {
  return suma(numeros) / numeros.length;
}
```

```javascript norun
// main.js
import { promedio, VERSION } from "./maths.js";
```

### La exportación por defecto

`export default` designa **la** cosa principal de un archivo. Solo hay una por archivo.
Quien importa la escribe **sin llaves**, y elige el nombre que quiera.

```javascript norun
// saludar.js
export default function saludar(nombre) {
  return `¡Hola, ${nombre}!`;
}
```

```javascript norun
// main.js
import saludar from "./saludar.js";
```

| | Exportación con nombre | Exportación por defecto |
|-|------------------------|-------------------------|
| Escritura | `export function suma` | `export default function saludar` |
| ¿Cuántas por archivo? | tantas como quieras | una sola |
| Importación | `import { suma } from "./maths.js"` | `import saludar from "./saludar.js"` |
| Nombre al importar | el mismo (o `as`) | libre |

> **Consejo:** prefiere las exportaciones con nombre: los nombres son los mismos en todo
> el proyecto, y el editor te ayuda mejor.

## Un proyecto en varios archivos

Aquí tienes un pequeño proyecto completo. El árbol de archivos:

```text
mi-proyecto/
├── package.json
└── src/
    ├── main.js
    ├── maths.js
    └── saludar.js
```

`maths.js` y `saludar.js` son los de la sección anterior. `main.js` los usa:

```javascript norun
// src/main.js
import saludar from "./saludar.js";
import { promedio, VERSION } from "./maths.js";

console.log(saludar("Ada"));
console.log(`Promedio: ${promedio([12, 15, 18])}`);
console.log(`Versión ${VERSION}`);
```

Ejecutas el archivo principal desde la carpeta `mi-proyecto`:

```sh
node src/main.js
```

```text
¡Hola, Ada!
Promedio: 15
Versión 1.0
```

Tres reglas que recordar:

- una ruta hacia **tu** archivo empieza por `./` (misma carpeta) o `../` (carpeta
  padre);
- la extensión `.js` es **obligatoria**;
- los `import` se colocan al principio del archivo.

`suma` no se importa en `main.js`, pero `promedio` la usa de todos modos: dentro de
`maths.js`, todo es visible.

### El campo "type": "module"

Para que Node lea los archivos `.js` como módulos `import`/`export`, el archivo
`package.json` del proyecto debe contener `"type": "module"`. Si no, Node muestra una
advertencia. Los archivos `.mjs` son siempre módulos, diga lo que diga el
`package.json`.

## La sintaxis antigua: require

En código más antiguo te encontrarás a menudo otra sintaxis llamada **CommonJS**:

```javascript norun
// Sintaxis antigua CommonJS (archivos .cjs, o proyectos sin "type": "module")
const path = require("node:path");

function saludar(nombre) {
  return `¡Hola, ${nombre}!`;
}

module.exports = { saludar };
```

`require` hace el papel de `import`, y `module.exports` el de `export`. Para un proyecto
nuevo, usa `import` y `export`: es la sintaxis estándar, que también funciona en el
navegador.

## Crear un proyecto con npm

**npm** (*Node Package Manager*, «gestor de paquetes de Node») se instala con Node.
Gestiona tu proyecto y sus [paquetes](../debuter/glossaire.md#paquet): código escrito
por otras personas, listo para reutilizar. Es una inmensa biblioteca pública: más de dos
millones de paquetes.

`npm init -y` crea el archivo `package.json`, el documento de identidad del proyecto
(`-y` acepta todas las respuestas por defecto).

```sh
mkdir mi-proyecto
cd mi-proyecto
npm init -y
npm pkg set type=module
```

```json
{
  "name": "mi-proyecto",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "type": "module"
}
```

Es [JSON](../debuter/glossaire.md#json). Los campos principales:

| Campo | Papel |
|-------|-------|
| `name`, `version` | el nombre y la versión del proyecto |
| `scripts` | comandos abreviados, que se ejecutan con `npm run` |
| `type` | `"module"` para usar `import` y `export` |
| `dependencies` | los paquetes que necesita el proyecto (lo añade `npm install`) |

## Instalar un paquete

`npm install` (o `npm i`) descarga un paquete y lo añade al proyecto. Tomemos
`slugify`, que transforma un título en una dirección web limpia.

```sh
npm install slugify
```

```text
added 1 package, and audited 2 packages in 535ms

found 0 vulnerabilities
```

Han cambiado tres cosas en la carpeta:

- una carpeta `node_modules/` contiene el código del paquete;
- `package.json` tiene una nueva sección `"dependencies": { "slugify": "^1.6.9" }`;
- un archivo `package-lock.json` anota la versión exacta instalada.

Un paquete se importa por su **nombre**, sin `./`:

```javascript norun
// src/titulo.js
import slugify from "slugify";

const titulo = "Fiestas de fin de año en León";
console.log(slugify(titulo, { lower: true, strict: true }));
```

```text
fiestas-de-fin-de-ano-en-leon
```

> **Nota:** `^1.6.9` quiere decir «la versión 1.6.9 o una versión más reciente
> compatible (1.x)». Para quitar un paquete: `npm uninstall slugify`.

## Los scripts de npm

La sección `scripts` de `package.json` da nombres cortos a comandos. Se ejecutan con
`npm run <nombre>`. `start` y `test` tienen un atajo: `npm start`, `npm test`.

```json
"scripts": {
  "start": "node src/main.js",
  "titulo": "node src/titulo.js"
}
```

```sh
npm start
```

```text
> mi-proyecto@1.0.0 start
> node src/main.js

¡Hola, Ada!
Promedio: 15
Versión 1.0
```

Así, alguien que descubre el proyecto no tiene que adivinar qué archivo ejecutar:
basta con `npm start`.

## node_modules y .gitignore

`node_modules/` puede contener miles de archivos. **Nunca** se guarda en
[Git](../debuter/git.md): `package.json` y `package-lock.json` bastan para volver a
crearlo. Añádelo al archivo `.gitignore` del proyecto:

```text
node_modules/
```

Quien obtiene tu proyecto solo tiene que escribir:

```sh
npm install
```

Sin nombre de paquete, `npm install` lee `package.json` e instala todo lo que aparece en
la lista. Es como una lista de la compra: se guarda la lista, no el contenido de la
nevera.

## Ejecutar una herramienta con npx

`npx` ejecuta una herramienta publicada en npm **sin instalarla** en tu proyecto: la
descarga, la ejecuta, y ya está. Es práctico para probar una herramienta o para crear
un proyecto.

```sh
npx --yes cowsay "Hola"
```

```text
 ______
< Hola >
 ------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

`--yes` evita que `npx` pida permiso para descargar.

## Errores frecuentes

### Olvidar la extensión .js

```javascript norun
import { promedio } from "./maths";
```

```text
node:internal/modules/esm/resolve:274
    throw new ERR_MODULE_NOT_FOUND(
          ^

Error [ERR_MODULE_NOT_FOUND]: Cannot find module '/home/vous/mi-proyecto/src/maths' imported from /home/vous/mi-proyecto/src/main.js
```

Node busca un archivo que se llame exactamente `maths`, sin extensión. Escribe el nombre
completo: `"./maths.js"`.

### Importar un nombre que no se exporta

```javascript norun
import { promedio, mediana } from "./maths.js";
```

```text
file:///home/vous/mi-proyecto/src/main.js:2
import { promedio, mediana } from "./maths.js";
                   ^^^^^^^
SyntaxError: The requested module './maths.js' does not provide an export named 'mediana'
```

«El módulo no proporciona ninguna exportación llamada `mediana`»: la función no existe,
o no tiene `export` delante. Comprueba la ortografía y añade `export`. El mismo mensaje
aparece si escribes `import { saludar }` para una exportación **por defecto**: entonces
quita las llaves.

### Usar require en un módulo

```javascript norun
const fs = require("node:fs");
```

```text
file:///home/vous/mi-proyecto/src/viejo.js:1
const fs = require("node:fs");
           ^

ReferenceError: require is not defined in ES module scope, you can use import instead
```

En un módulo (`.mjs`, o `"type": "module"`), `require` no existe. Escribe
`import fs from "node:fs";` o `import { existsSync } from "node:fs";`.

## Ponte a prueba

**Pregunta 1:** un archivo contiene `export default function mostrar() {}` y
`export const LIMITE = 10;`. ¿Cómo importas los dos?

> **Respuesta**
>
> `import mostrar, { LIMITE } from "./archivo.js";`: primero el de por defecto, sin
> llaves, y luego los nombres entre llaves.

**Pregunta 2:** ¿qué diferencia hay entre `"./slugify.js"` y `"slugify"` en un
`import`?

> **Respuesta**
>
> `"./slugify.js"` es **tu** archivo, junto al archivo actual. `"slugify"`, sin `./`,
> es un **paquete** instalado con npm, que se busca en `node_modules`.

**Pregunta 3:** un amigo obtiene tu proyecto desde Git, pero no hay carpeta
`node_modules`. ¿Qué debe hacer?

> **Respuesta**
>
> Escribir `npm install` en la carpeta del proyecto. npm lee `package.json` y
> `package-lock.json`, y vuelve a instalar todos los paquetes.

## En resumen

- Cada archivo es un módulo; `export` comparte, `import` recupera.
- Exportación con nombre: `import { nombre }`; exportación por defecto: `import nombre`,
  sin llaves.
- Las rutas de tus archivos empiezan por `./` y terminan en `.js`.
- `npm init` crea `package.json`; `npm install` añade un paquete; `npm run` ejecuta un
  script.
- `node_modules/` va en `.gitignore`; `npx` ejecuta una herramienta sin instalarla.

Siguiente: [la programación asíncrona](asynchrone.md). Compara con
[los módulos en Ruby](../ruby/modules.md).
