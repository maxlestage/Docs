# Una herramienta de línea de comandos tipada

Esta página muestra cómo escribir en TypeScript un pequeño programa que se lanza en la terminal, con argumentos, opciones comprobadas, un código de salida y la lectura de la entrada estándar.

> **Objetivos:**
>
> - sabrás leer los argumentos con `process.argv` y sabrás que siempre son texto;
> - sabrás describir opciones con `util.parseArgs` y aprovechar los tipos que deduce;
> - sabrás devolver un código de salida (`0` para éxito, otra cosa para fallo);
> - sabrás leer la entrada estándar;
> - sabrás construir un contador de palabras en el que cada pieza se prueba con arrays
>   fijos.

El [capítulo de JavaScript](../javascript/ligne-de-commande.md) presenta las mismas
herramientas de Node. Aquí insistimos en los **tipos** y en una forma de escribir el
programa que lo hace fácil de probar.

## ¿Qué es una herramienta de línea de comandos?

Es un programa que se lanza escribiendo su nombre en la
[terminal](../debuter/glossaire.md#terminal), seguido de algunas palabras:

```sh
node contar.ts --lineas poema.txt
```

Piensa en una ventanilla. Entregas una **hoja de pedido** rellenada (los argumentos). El
empleado la lee, hace el trabajo, te devuelve el resultado y te dice «todo bien» o «hay
un problema» (el código de salida).

```text
   tú ──►  node contar.ts --lineas poema.txt
                       │
                       ▼
             ┌─────────────────────┐
             │  leer las opciones  │  --lineas = sí, archivo = poema.txt
             │  hacer el trabajo   │  contar las palabras
             └──────────┬──────────┘
                        ▼
             resultado en pantalla  +  código de salida (0 = todo bien)
```

> **Nota:** desde Node 22.18, `node archivo.ts` ejecuta directamente un archivo
> TypeScript, sin comprobar los tipos (mira la [introducción](introduction.md)). Ejecuta
> `npx tsc --noEmit` para comprobar los tipos antes.

> **Recuerda:** una herramienta de línea de comandos recibe palabras, trabaja, muestra
> un resultado y devuelve un código de salida.

## Los argumentos con process.argv

`process.argv` contiene todo lo que se ha escrito. Su tipo es `string[]`: un array de
textos.

```text
   node  /home/ada/saludar.ts  Ada  3
   ───┬  ─────────┬──────────  ─┬─  ┬
   casilla 0   casilla 1      casilla 2  casilla 3
   (node)      (el script)    tus argumentos
```

Así que tus argumentos empiezan en la casilla `2`. Quitamos las dos primeras casillas
con `slice(2)`.

Para que el ejemplo dé siempre la misma salida, aquí escribimos el array a mano, como
si hubieras escrito `node saludar.ts Ada 3`:

```typescript
const argv: string[] = ["/usr/bin/node", "/home/ada/saludar.ts", "Ada", "3"];

const args = argv.slice(2);
console.log(args);
console.log(`¡Hola, ${args[0]}!`);
console.log(typeof args[1]);
```

```sortie
[ 'Ada', '3' ]
¡Hola, Ada!
string
```

Fíjate en la última línea: `"3"` es un **texto**, no un número. Un argumento siempre es
texto.

En un programa real, escribes simplemente:

```typescript norun
const args = process.argv.slice(2);
```

### Un argumento puede faltar

TypeScript anuncia `args[0]` como un `string`. Pero si el usuario no escribe nada, la
casilla no existe y vale `undefined`. TypeScript no lo señala (no cuenta las casillas
de un array): te toca a ti prever un valor por defecto con `??`.

```typescript
function saludar(args: string[]): string {
  const nombre = args[0] ?? "desconocido";
  return `¡Hola, ${nombre}!`;
}

console.log(saludar(["Ada"]));
console.log(saludar([]));
```

```sortie
¡Hola, Ada!
¡Hola, desconocido!
```

Fíjate en la forma: `saludar` recibe el array **como parámetro**, en lugar de leer
`process.argv` ella misma. Así puedes probarla con cualquier array. Esta idea sirve en
toda la página.

Prueba: llama a `saludar(["Ada", "Grace"])`. Solo se saluda a `Ada`: la casilla 1 se
ignora.

### Convertir un argumento en número

Para obtener un número, conviertes el texto con `Number`. Si el texto no es un número,
obtienes `NaN` («no es un número»). Una pequeña función tipada gestiona los dos casos:

```typescript
function leerEntero(texto: string | undefined): number | null {
  if (texto === undefined) {
    return null;
  }
  const n = Number(texto);
  return Number.isInteger(n) ? n : null;
}

for (const intento of ["3", "tres", "2.5", undefined]) {
  console.log(intento, "->", leerEntero(intento));
}
```

```sortie
3 -> 3
tres -> null
2.5 -> null
undefined -> null
```

| Paso | `texto` | `Number(texto)` | ¿Entero? | Resultado |
|------|---------|-----------------|----------|-----------|
| 1 | `"3"` | 3 | sí | 3 |
| 2 | `"tres"` | `NaN` | no | `null` |
| 3 | `"2.5"` | 2.5 | no | `null` |
| 4 | `undefined` | (no se calcula) | | `null` |

> **Recuerda:** `process.argv.slice(2)` da tus argumentos, de tipo `string[]`. Siempre
> son textos, y pueden faltar.

## Las opciones con util.parseArgs

Las herramientas reales aceptan **opciones**: `--lineas`, `-l`, `--max 10`… Ordenarlas
a mano es pesado. La función `parseArgs` del módulo `node:util` lo hace por ti. Le
describes las opciones esperadas, como un formulario con casillas que rellenar:

- `type: "boolean"`: una casilla que marcar, presente o ausente (`--lineas`);
- `type: "string"`: una casilla que rellenar, seguida de un valor (`--max 10`);
- `short: "l"`: un atajo de una letra (`-l`);
- `default`: el valor si la opción falta.

Normalmente, `parseArgs` lee `process.argv` por sí sola. Aquí le damos un array `args`
escrito a mano: simulamos `node herramienta.ts -l --max 3 a.txt b.txt`.

```typescript
import { parseArgs } from "node:util";

const { values, positionals } = parseArgs({
  args: ["-l", "--max", "3", "a.txt", "b.txt"],
  options: {
    lineas: { type: "boolean", short: "l", default: false },
    max: { type: "string" },
  },
  allowPositionals: true,
});

console.log(values.lineas);
console.log(values.max);
console.log(positionals);
```

```sortie
true
3
[ 'a.txt', 'b.txt' ]
```

Así lee `parseArgs` el array, casilla por casilla:

| Paso | Casilla leída | Qué es | `values` después | `positionals` después |
|------|---------------|--------|------------------|-----------------------|
| 1 | `"-l"` | atajo de `--lineas` | `{ lineas: true }` | `[]` |
| 2 | `"--max"` | opción de texto: toma la casilla siguiente | `{ lineas: true, max: "3" }` | `[]` |
| 3 | `"3"` | (ya tomada por `--max`) | | |
| 4 | `"a.txt"` | sin guion: argumento | | `["a.txt"]` |
| 5 | `"b.txt"` | sin guion: argumento | | `["a.txt", "b.txt"]` |

`allowPositionals: true` permite los argumentos sin guion (los nombres de archivo).

### Los tipos deducidos por TypeScript

TypeScript lee la descripción de las opciones y **deduce** el tipo de cada valor:

| Descripción | Tipo del valor |
|-------------|----------------|
| `{ type: "boolean" }` | `boolean \| undefined` |
| `{ type: "boolean", default: false }` | `boolean` |
| `{ type: "string" }` | `string \| undefined` |
| `{ type: "string", default: "10" }` | `string` |

Sin `default`, la opción puede faltar: el tipo contiene `undefined`, y TypeScript te
obliga a tratar ese caso. Y una opción `string` sigue siendo un **texto**: `values.max`
nunca es un `number` (mira los [errores frecuentes](#errores-frecuentes)).

```typescript
import { parseArgs } from "node:util";

function leerMax(args: string[]): number {
  const { values } = parseArgs({
    args,
    options: { max: { type: "string", default: "10" } },
  });
  // values.max es un string (gracias a default): lo convertimos.
  return Number(values.max);
}

console.log(leerMax(["--max", "3"]));
console.log(leerMax(["--max=25"]));
console.log(leerMax([]));
```

```sortie
3
25
10
```

Prueba: quita `default: "10"`. El programa sigue compilando (`Number(undefined)` está
permitido), pero la última llamada muestra `NaN`. Añade `?? "10"` para corregirlo.

### Una opción desconocida

`parseArgs` es **estricta**: una opción que no conoce lanza un error. Es algo bueno,
porque una errata (`--lienas`) no pasa desapercibida. Lo atrapamos con `try` / `catch`
(mira [gestionar los errores](erreurs.md)):

```typescript
import { parseArgs } from "node:util";

function probar(args: string[]): void {
  try {
    const { values } = parseArgs({ args, options: { lineas: { type: "boolean" } } });
    console.log("OK:", values.lineas);
  } catch (e) {
    const mensaje = e instanceof Error ? e.message : String(e);
    console.log("Rechazado:", mensaje);
  }
}

probar(["--lineas"]);
probar(["--lienas"]);
probar(["poema.txt"]);
```

```sortie
OK: true
Rechazado: Unknown option '--lienas'
Rechazado: Unexpected argument 'poema.txt'. This command does not take positional arguments
```

La tercera llamada se rechaza porque falta `allowPositionals`: sin él, un nombre de
archivo está prohibido.

> **Recuerda:** `parseArgs` ordena las opciones y TypeScript deduce sus tipos. Una
> opción `string` sigue siendo un texto; sin `default`, puede valer `undefined`.

## El código de salida

Al terminar, un programa devuelve a la terminal un **código de salida**, un número
pequeño:

| Código | Significado habitual |
|--------|----------------------|
| `0` | todo ha ido bien |
| `1` | el trabajo ha fallado (archivo no encontrado…) |
| `2` | el comando estaba mal escrito (opción desconocida…) |

Es el «todo bien» o «hay un problema» del empleado de la ventanilla. Los scripts y las
herramientas de automatización lo leen para saber si el paso ha funcionado.

Lo eliges con `process.exitCode`. Lo más sencillo: una función `principal` que
**devuelve** el código, y una sola línea que se lo pasa a Node.

```typescript
function principal(args: string[]): number {
  if (args.length === 0) {
    console.log("Error: da al menos un nombre.");
    return 2;
  }
  for (const nombre of args) {
    console.log(`¡Hola, ${nombre}!`);
  }
  return 0;
}

console.log("código:", principal(["Ada", "Grace"]));
console.log("código:", principal([]));
```

```sortie
¡Hola, Ada!
¡Hola, Grace!
código: 0
Error: da al menos un nombre.
código: 2
```

En el programa real, la última línea pasa a ser:

```typescript norun
process.exitCode = principal(process.argv.slice(2));
```

Y en la terminal, `echo $?` muestra el código del último comando:

```sh
node saludar.ts
echo $?
```

```text
Error: da al menos un nombre.
2
```

> **Nota:** en una herramienta real, escribe los mensajes de error con `console.error`:
> van a la **salida de errores**, separada de los resultados (mira el
> [capítulo de JavaScript](../javascript/ligne-de-commande.md)). Aquí usamos
> `console.log` para que los ejemplos lo muestren todo.

> **Consejo:** prefiere `process.exitCode = …` a `process.exit(…)`. `process.exit`
> corta el programa de golpe, incluso a mitad de la escritura de un archivo.

> **Recuerda:** `0` quiere decir éxito, cualquier otro código quiere decir fallo. Una
> función `principal(args): number` lo hace fácil de probar.

## Leer la entrada estándar

Una herramienta también puede recibir texto por una **tubería** (`|`): la salida de un
comando se convierte en la **entrada estándar** (*stdin*) del siguiente.

```text
   cat poema.txt  ──|──►  node contar.ts
   (muestra el texto)     (lo lee en su entrada estándar)
```

La función `text` de `node:stream/consumers` lee toda la entrada estándar y devuelve
una [promesa](../debuter/glossaire.md#promesa) de `string`. Este ejemplo no puede
ejecutarse aquí, porque espera un texto que llega de la terminal:

```typescript norun
import { text } from "node:stream/consumers";

const entrada: string = await text(process.stdin);
const palabras = entrada.split(/\s+/).filter((p) => p !== "");
console.log(`${palabras.length} palabras`);
```

```sh
printf 'uno dos\ntres\n' | node contar.ts
```

```text
3 palabras
```

Como siempre, deja el cálculo en una función que recibe un `string`: podrás probarla sin
tubería, con un texto escrito a mano. Es lo que hace la sección siguiente.

> **Recuerda:** `await text(process.stdin)` lee todo lo que llega por la tubería, en
> forma de un solo `string`.

## Un contador de palabras tipado

Juntémoslo todo: una herramienta `contar` que cuenta las palabras de uno o varios
archivos y, si se le pide, las líneas.

```text
   Uso: contar [-l] <archivo>...
     -l, --lineas   cuenta también las líneas
     -h, --ayuda    muestra esta ayuda
```

El programa está dividido en pequeñas funciones, cada una con un papel:

```text
   args (string[]) ──► analizar ──► Comando ──► principal ──► código de salida
                                                   │
                                   leer(archivo) ──┤
                                                   ▼
                                                 contar ──► Cuenta
```

La lectura de los archivos se **pasa como parámetro** (`leer`). En el ejemplo, le damos
un disco falso: un simple objeto que asocia un nombre de archivo con su contenido. En el
programa real, le daremos `readFileSync`.

```typescript
import { parseArgs } from "node:util";

type Comando =
  | { kind: "ayuda" }
  | { kind: "error"; mensaje: string }
  | { kind: "contar"; lineas: boolean; archivos: string[] };

interface Cuenta {
  palabras: number;
  lineas: number;
}

const AYUDA = "Uso: contar [-l] <archivo>...";

function analizar(args: string[]): Comando {
  try {
    const { values, positionals } = parseArgs({
      args,
      options: {
        lineas: { type: "boolean", short: "l", default: false },
        ayuda: { type: "boolean", short: "h", default: false },
      },
      allowPositionals: true,
    });
    if (values.ayuda) {
      return { kind: "ayuda" };
    }
    if (positionals.length === 0) {
      return { kind: "error", mensaje: "da al menos un archivo" };
    }
    return { kind: "contar", lineas: values.lineas, archivos: positionals };
  } catch (e) {
    return { kind: "error", mensaje: e instanceof Error ? e.message : String(e) };
  }
}

function contar(texto: string): Cuenta {
  const palabras = texto.split(/\s+/).filter((p) => p !== "").length;
  const lineas = texto === "" ? 0 : texto.split("\n").filter((l) => l !== "").length;
  return { palabras, lineas };
}

function principal(args: string[], leer: (archivo: string) => string | null): number {
  const comando = analizar(args);
  switch (comando.kind) {
    case "ayuda":
      console.log(AYUDA);
      return 0;
    case "error":
      console.log(`Error: ${comando.mensaje}`);
      console.log(AYUDA);
      return 2;
    case "contar": {
      let codigo = 0;
      for (const archivo of comando.archivos) {
        const texto = leer(archivo);
        if (texto === null) {
          console.log(`${archivo}: no encontrado`);
          codigo = 1;
          continue;
        }
        const { palabras, lineas } = contar(texto);
        const detalle = comando.lineas ? `, ${lineas} líneas` : "";
        console.log(`${archivo}: ${palabras} palabras${detalle}`);
      }
      return codigo;
    }
  }
}

// Un disco falso, para probar sin archivos de verdad.
const disco: Record<string, string> = {
  "poema.txt": "El gato duerme\nsobre la alfombra\n",
  "lista.txt": "pan leche\nhuevos\n\nmantequilla\n",
};
const leerFalso = (archivo: string): string | null => disco[archivo] ?? null;

const intentos = [
  ["poema.txt"],
  ["-l", "poema.txt", "lista.txt"],
  ["ausente.txt"],
  ["--palabras", "poema.txt"],
];

for (const args of intentos) {
  console.log(`$ contar ${args.join(" ")}`);
  console.log(`(código ${principal(args, leerFalso)})`);
}
```

```sortie
$ contar poema.txt
poema.txt: 6 palabras
(código 0)
$ contar -l poema.txt lista.txt
poema.txt: 6 palabras, 2 líneas
lista.txt: 4 palabras, 3 líneas
(código 0)
$ contar ausente.txt
ausente.txt: no encontrado
(código 1)
$ contar --palabras poema.txt
Error: Unknown option '--palabras'. To specify a positional argument starting with a '-', place it at the end of the command after '--', as in '-- "--palabras"
Uso: contar [-l] <archivo>...
(código 2)
```

Con `allowPositionals: true`, Node añade un consejo al mensaje: si de verdad quieres un
archivo llamado `--palabras`, escríbelo después de `--`.

### Seguir contar paso a paso

Tomemos el texto de `lista.txt`: `"pan leche\nhuevos\n\nmantequilla\n"` (`\n` es un
salto de línea).

| Paso | Cálculo | Valor |
|------|---------|-------|
| 1 | `texto.split(/\s+/)`: cortamos en cada grupo de espacios o saltos | `["pan", "leche", "huevos", "mantequilla", ""]` |
| 2 | `.filter((p) => p !== "")`: quitamos los trozos vacíos | `["pan", "leche", "huevos", "mantequilla"]` |
| 3 | `.length` | `palabras` = 4 |
| 4 | `texto.split("\n")`: cortamos en cada salto de línea | `["pan leche", "huevos", "", "mantequilla", ""]` |
| 5 | `.filter((l) => l !== "")`: quitamos las líneas vacías | `["pan leche", "huevos", "mantequilla"]` |
| 6 | `.length` | `lineas` = 3 |

### Lo que aportan los tipos

- `Comando` es una [unión discriminada](unions.md#las-uniones-discriminadas): en el
  `case "contar"`, TypeScript sabe que `comando.archivos` existe; en el
  `case "error"`, que `comando.mensaje` existe.
- `leer` devuelve `string | null`: TypeScript obliga a `principal` a tratar el archivo
  no encontrado antes de llamar a `contar`.
- `principal` devuelve `number` en **todos** los casos: si olvidas un `return`,
  TypeScript lo señala.

Prueba: añade `"vacio.txt": ""` al disco falso y el intento `["-l", "vacio.txt"]`.
Obtienes `vacio.txt: 0 palabras, 0 líneas` y el código 0.

### Conectarlo con el mundo real

Solo queda sustituir el disco falso por archivos de verdad. El archivo `contar.ts`
contiene los tipos y las funciones de arriba, y luego estas líneas en lugar de los
intentos:

```typescript norun
import { readFileSync } from "node:fs";

function leerArchivo(archivo: string): string | null {
  try {
    return readFileSync(archivo, "utf8");
  } catch {
    return null;
  }
}

process.exitCode = principal(process.argv.slice(2), leerArchivo);
```

```sh
node contar.ts -l poema.txt
echo $?
```

```text
poema.txt: 6 palabras, 2 líneas
0
```

Solo estas pocas líneas tocan el mundo real (`process.argv`, el disco,
`process.exitCode`). Todo lo demás se prueba con arrays fijos, como antes. Eso es lo que
hace que la herramienta sea fácil de [probar](tests.md).

> **Recuerda:** separa el «núcleo» (funciones que reciben arrays y textos) de las pocas
> líneas que leen `process.argv` y el disco.

## Errores frecuentes

### Usar una opción de texto como un número

```typescript norun
import { parseArgs } from "node:util";

const { values } = parseArgs({ options: { max: { type: "string" } } });
const max: number = values.max;
```

```text
main.ts(4,7): error TS2322: Type 'string | undefined' is not assignable to type 'number'.
  Type 'undefined' is not assignable to type 'number'.
```

Una opción `string` da un texto, que puede faltar. Da un valor por defecto y convierte:
`const max = Number(values.max ?? "10");`.

### Una opción desconocida sin atrapar

```typescript norun
import { parseArgs } from "node:util";

const { values } = parseArgs({ options: { palabras: { type: "boolean" } } });
console.log(values);
```

```sh
node contar.ts --palbras
```

```text
TypeError [ERR_PARSE_ARGS_UNKNOWN_OPTION]: Unknown option '--palbras'
```

TypeScript no puede ver nada: el error depende de lo que escriba el usuario. Rodea
`parseArgs` con un `try` / `catch`, muestra la ayuda y devuelve el código 2, como en
`analizar`.

### Leer la casilla equivocada de process.argv

```typescript norun
const nombre = process.argv[0];
console.log(`¡Hola, ${nombre}!`);
```

```sh
node saludar.ts Ada
```

```text
¡Hola, /usr/bin/node!
```

Ningún error, pero un resultado falso: la casilla 0 es la ruta de `node`. Usa
`process.argv.slice(2)` y luego la casilla 0 de ese nuevo array.

## Ponte a prueba

**Pregunta 1:** con el comando `node herramienta.ts 5 gatos`, ¿cuánto vale
`process.argv.slice(2)`, y cuál es el tipo de su primera casilla?

> **Respuesta**
>
> `["5", "gatos"]`. La primera casilla es el texto `"5"`, de tipo `string`: hay que
> convertirlo con `Number` para calcular.

**Pregunta 2:** ¿cuál es el tipo de `values.detallado` con
`options: { detallado: { type: "boolean" } }`? ¿Y con `default: false`?

> **Respuesta**
>
> Sin `default`: `boolean | undefined`, porque la opción puede faltar. Con
> `default: false`: `boolean`.

**Pregunta 3:** escribe `principal(args: string[]): number`, que muestra la suma de los
números recibidos y devuelve 0, o muestra `Error: X no es un número` y devuelve 1.
Pruébala con `["2", "3"]` y luego con `["2", "x"]`.

> **Respuesta**
>
> ```typescript
> function principal(args: string[]): number {
>   let suma = 0;
>   for (const a of args) {
>     const n = Number(a);
>     if (Number.isNaN(n)) {
>       console.log(`Error: ${a} no es un número`);
>       return 1;
>     }
>     suma += n;
>   }
>   console.log(suma);
>   return 0;
> }
>
> console.log("código:", principal(["2", "3"]));
> console.log("código:", principal(["2", "x"]));
> ```
>
> ```sortie
> 5
> código: 0
> Error: x no es un número
> código: 1
> ```

## En resumen

- `process.argv.slice(2)` da tus argumentos: un `string[]`, siempre texto, cuyas
  casillas pueden faltar.
- `parseArgs` de `node:util` ordena las opciones; TypeScript deduce sus tipos
  (`boolean`, `string`, con `| undefined` sin `default`).
- `parseArgs` lanza un error para una opción desconocida: atrápalo y devuelve el
  código 2.
- El código de salida vale `0` si todo va bien; elígelo con
  `process.exitCode = principal(…)`.
- `await text(process.stdin)` lee la entrada estándar.
- Escribe el núcleo de la herramienta con funciones que reciben arrays: las pruebas con
  valores fijos.

Siguiente: [TypeScript para la web](web.md).
