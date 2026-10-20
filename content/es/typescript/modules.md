# Módulos y npm

Esta página explica cómo dividir un programa TypeScript en varios archivos, cómo compartir tipos entre ellos y cómo usar paquetes npm con sus tipos.

> **Objetivos:**
>
> - sabrás exportar e importar funciones, valores y tipos;
> - sabrás cuándo escribir `import type`;
> - sabrás organizar y compilar un proyecto de varios archivos;
> - sabrás instalar un paquete npm y sus tipos `@types/…`;
> - sabrás qué es un archivo de declaración `.d.ts`.

## ¿Por qué módulos?

Un programa que crece se vuelve difícil de leer en un solo archivo. Se divide en
[módulos](../debuter/glossaire.md#module): un archivo por tema. Es como ordenar una
cocina: las especias en un cajón, las cacerolas en otro.

En TypeScript, **cada archivo que contiene `import` o `export` es un módulo**. Lo que
declara queda privado, salvo lo que **exporta** explícitamente.

## Importar un módulo de Node

Node trae módulos listos para usar. Su nombre empieza por `node:`. Importas solo lo
que necesitas, entre llaves.

```typescript
import { writeFileSync, readFileSync } from "node:fs";
import path from "node:path";

const ruta = path.join("notas", "..", "compras.txt");
console.log(ruta);

writeFileSync(ruta, "pan\nleche\n");
const contenido = readFileSync(ruta, "utf8");
console.log(contenido.split("\n").filter((l) => l !== "").length, "artículos");
```

```sortie
compras.txt
2 artículos
```

Hay dos formas de importar:

- `import { a, b } from "…"`: **exportaciones con nombre**, elegidas una por una;
- `import path from "…"`: la **exportación por defecto** del módulo, con el nombre que
  quieras.

Gracias a los tipos de Node, TypeScript conoce la firma de `readFileSync`: sabe que,
con `"utf8"`, devuelve un `string`.

## Exportar desde tus archivos

Pon `export` delante de lo que quieras compartir: una función, una constante, una
clase, y también un **tipo** o una **interfaz**.

```typescript norun
// src/tipos.ts
export interface Tarea {
  titulo: string;
  hecha: boolean;
}
```

```typescript norun
// src/herramientas.ts
import type { Tarea } from "./tipos.js";

export function formatear(t: Tarea): string {
  return `${t.hecha ? "[x]" : "[ ]"} ${t.titulo}`;
}

export default function contar(tareas: Tarea[]): number {
  return tareas.filter((t) => !t.hecha).length;
}

const SECRETO = 42; // no exportado: invisible desde los otros archivos
```

Un archivo tiene tantas exportaciones con nombre como quieras, y como mucho un
`export default`.

> **Consejo:** prefiere las exportaciones con nombre. El nombre es el mismo en todas
> partes, y tu editor las encuentra y las renombra fácilmente.

### La extensión .js en los imports

```typescript norun
// src/main.ts
import contar, { formatear } from "./herramientas.js";
import type { Tarea } from "./tipos.js";

const tareas: Tarea[] = [
  { titulo: "Comprar pan", hecha: true },
  { titulo: "Repasar TypeScript", hecha: false },
];

for (const t of tareas) {
  console.log(formatear(t));
}
console.log(`Quedan por hacer: ${contar(tareas)}`);
```

El archivo se llama `herramientas.ts`, pero el import dice `./herramientas.js`. Es a
propósito: Node ejecutará el archivo **compilado**, que se llamará `herramientas.js`.
TypeScript entiende que debe comprobar `herramientas.ts`. El `./` indica un archivo de
tu proyecto, no un paquete.

## import type

Un tipo no existe en tiempo de ejecución: la
[compilación](../debuter/glossaire.md#compilation) borra todos los tipos. `import type`
dice claramente: «solo importo un tipo». Este import desaparece por completo del
JavaScript generado.

```typescript norun
import type { Tarea } from "./tipos.js";         // solo tipos
import { formatear, type Opciones } from "./herramientas.js"; // una mezcla
```

También puedes poner `type` delante de un solo nombre, en medio de un import normal.

¿Por qué preocuparse? Quien lee ve enseguida qué es un tipo. Y algunas herramientas,
que quitan los tipos sin analizarlo todo, lo necesitan para saber qué borrar.
Acostúmbrate: un import de tipo se escribe con `type`.

## Un proyecto de varios archivos

Así se organiza un proyecto pequeño:

```text
tareas/
├── package.json
├── tsconfig.json
├── src/
│   ├── herramientas.ts
│   ├── main.ts
│   └── tipos.ts
└── dist/            ← creado por tsc
    ├── herramientas.js
    ├── main.js
    └── tipos.js
```

El código fuente está en `src/`, el JavaScript compilado en `dist/`. El archivo
`package.json` describe el proyecto para npm. La línea `"type": "module"` le dice a
Node que use los módulos `import` / `export`:

```json
{
  "name": "tareas",
  "type": "module",
  "scripts": {
    "build": "tsc",
    "start": "node dist/main.js"
  },
  "devDependencies": {
    "typescript": "^5.8.3"
  }
}
```

El archivo `tsconfig.json` configura el compilador (consulta
[la configuración](configuration.md) para los detalles):

```json
{
  "compilerOptions": {
    "target": "es2022",
    "module": "nodenext",
    "strict": true,
    "rootDir": "src",
    "outDir": "dist"
  },
  "include": ["src"]
}
```

Se compila todo el proyecto y luego se ejecuta el archivo principal:

```sh
npm run build
npm start
```

```text
[x] Comprar pan
[ ] Repasar TypeScript
Quedan por hacer: 1
```

`npm run build` lanza `tsc`, que lee `tsconfig.json`, comprueba **todos** los archivos
de `src/` juntos y escribe `dist/`. Si un archivo usa mal un tipo exportado por otro,
el error aparece en ese momento.

> **Nota:** `tipos.js` está casi vacío: el archivo solo contenía una interfaz, y los
> tipos se borran.

## Los paquetes npm

npm es el gestor de [paquetes](../debuter/glossaire.md#paquet) de JavaScript.
Descarga bibliotecas escritas por otras personas en la carpeta `node_modules/` y las
anota en `package.json`.

```sh
npm install lodash
```

Luego se importa el paquete por su nombre, sin `./`:

```typescript norun
import _ from "lodash";

console.log(_.chunk([1, 2, 3, 4, 5], 2));
```

Sorpresa: TypeScript se niega a compilar.

```text
src/main.ts(1,15): error TS7016: Could not find a declaration file for module 'lodash'. '/home/alice/tareas/node_modules/lodash/lodash.js' implicitly has an 'any' type.
  Try `npm i --save-dev @types/lodash` if it exists or add a new declaration (.d.ts) file containing `declare module 'lodash';`
```

`lodash` está escrito en JavaScript, sin tipos. TypeScript no sabe qué contiene `_`.

### Los paquetes @types

Para muchos paquetes JavaScript, unos voluntarios han escrito los tipos aparte, en el
proyecto **DefinitelyTyped**. Se publican con el nombre `@types/<paquete>`. El mensaje
de error te lo sugiere:

```sh
npm install --save-dev @types/lodash
```

```text
[ [ 1, 2 ], [ 3, 4 ], [ 5 ] ]
```

El programa compila y se ejecuta. `--save-dev` guarda este paquete en
`devDependencies`: los tipos solo sirven durante el desarrollo, no en la ejecución.

```json
{
  "dependencies": {
    "lodash": "^4.18.1"
  },
  "devDependencies": {
    "@types/lodash": "^4.17.26"
  }
}
```

Los tipos del propio Node vienen de `@types/node`.

### Los paquetes que traen sus tipos

Los paquetes recientes suelen estar escritos en TypeScript y **traen sus tipos**. No
hay nada que añadir: basta con `npm install zod`. Para saber en qué caso estás:

| Situación | ¿Qué hacer? |
|-----------|-------------|
| El paquete trae sus tipos | nada |
| Error TS7016, y `@types/<nombre>` existe | `npm install --save-dev @types/<nombre>` |
| Error TS7016, y no existe nada | escribir un pequeño archivo `.d.ts` |

> **Consejo:** en la web de npm, una pequeña insignia «TS» junto al nombre indica un
> paquete que trae sus tipos; una insignia «DT» indica tipos disponibles en `@types`.

## Los archivos de declaración .d.ts

Un archivo `.d.ts` contiene **solo tipos**, sin ningún código ejecutable. Es el manual
de instrucciones de un módulo: describe lo que el módulo exporta y con qué tipos. Los
paquetes `@types/…` solo contienen eso.

### Escribir una declaración

Imagina un paquete `colores-texto`, en JavaScript, sin tipos en ninguna parte. Puedes
escribir su manual tú mismo, en `src/types/colores-texto.d.ts`:

```typescript norun
declare module "colores-texto" {
  export function rojo(texto: string): string;
  export function verde(texto: string): string;
}
```

`declare` quiere decir: «esto existe en otro sitio, solo describo su forma».
TypeScript te cree sin comprobar: describe solo las funciones que usas, y descríbelas
bien.

### Generar declaraciones

Al revés, si escribes una biblioteca, la opción `"declaration": true` de
`tsconfig.json` le pide a `tsc` que escriba un `.d.ts` junto a cada `.js`. Para
`herramientas.ts`, genera:

```typescript norun
import type { Tarea } from "./tipos.js";
export declare function formatear(t: Tarea): string;
export default function contar(tareas: Tarea[]): number;
```

Los cuerpos de las funciones han desaparecido: solo quedan las firmas. Así, quienes
usen tu biblioteca tendrán los tipos sin instalar nada más.

## Errores frecuentes

### Olvidar la extensión .js

```typescript norun
// src/main.ts
import { formatear } from "./herramientas";
```

```text
src/main.ts(2,27): error TS2835: Relative import paths need explicit file extensions in ECMAScript imports when '--moduleResolution' is 'node16' or 'nodenext'. Did you mean './herramientas.js'?
```

Con `"module": "nodenext"`, Node exige la extensión completa. Escribe
`"./herramientas.js"`, como propone el mensaje.

### Importar lo que no está exportado

```typescript norun
// src/main.ts
import { SECRETO } from "./herramientas.js";
```

```text
src/main.ts(2,10): error TS2614: Module '"./herramientas.js"' has no exported member 'SECRETO'. Did you mean to use 'import SECRETO from "./herramientas.js"' instead?
```

`SECRETO` existe en `herramientas.ts`, pero sin `export`. Añade `export` delante de su
declaración, si de verdad quieres compartirlo. No sigas la sugerencia del mensaje:
`import SECRETO from …` importaría la exportación por defecto, es decir, la función
`contar`.

### Usar como valor un import de tipo

```typescript norun
// src/main.ts
import type { formatear } from "./herramientas.js";
console.log(formatear({ titulo: "Leer", hecha: false }));
```

```text
src/main.ts(3,13): error TS1361: 'formatear' cannot be used as a value because it was imported using 'import type'.
```

`import type` se borra al compilar: la función ya no existiría en la ejecución. Para
una función, una clase o una constante, usa un `import` normal.

## Ponte a prueba

**Pregunta 1:** en `src/main.ts`, ¿por qué se escribe
`import { f } from "./herramientas.js"` si el archivo se llama `herramientas.ts`?

> **Respuesta**
>
> Porque Node ejecuta el código compilado, donde el archivo se llama
> `herramientas.js`. Con `"module": "nodenext"`, TypeScript exige la ruta que usará
> Node, y sabe que debe comprobar `herramientas.ts` en su lugar.

**Pregunta 2:** instalas un paquete y obtienes el error TS7016. ¿Qué haces?

> **Respuesta**
>
> El paquete no trae sus tipos. Instala `@types/<nombre>` con
> `npm install --save-dev @types/<nombre>` si existe; si no, escribe un archivo `.d.ts`
> con `declare module "<nombre>" { … }`.

**Pregunta 3:** con el módulo `node:path`, muestra la extensión y el nombre base (sin
carpeta) de la ruta `"fotos/vacaciones/playa.jpg"`.

> **Respuesta**
>
> ```typescript
> import path from "node:path";
>
> const ruta = "fotos/vacaciones/playa.jpg";
> console.log(path.extname(ruta));
> console.log(path.basename(ruta));
> ```
>
> ```sortie
> .jpg
> playa.jpg
> ```

## En resumen

- Un archivo con `import` o `export` es un módulo; solo lo exportado es visible desde
  fuera.
- Tus propios archivos se importan con `./nombre.js`, y los módulos de Node con
  `node:…`.
- `import type` importa solo un tipo: desaparece al compilar.
- `tsc` compila todo el proyecto de `src/` a `dist/`, guiado por `tsconfig.json`.
- Un paquete npm sin tipos se completa con `@types/<nombre>` o un archivo `.d.ts`.

Siguiente: [la programación asíncrona](asynchrone.md)
