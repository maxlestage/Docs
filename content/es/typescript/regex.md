# Las expresiones regulares en TypeScript

Esta página muestra lo que TypeScript añade a las expresiones regulares: resultados que pueden valer `null`, grupos tipados y funciones de análisis seguras.

> **Objetivos:**
>
> - sabrás escribir una regex y reconocer el tipo `RegExp`;
> - sabrás tratar el `null` que devuelve `match` gracias al estrechamiento;
> - sabrás leer los grupos numerados y con nombre sin errores de tipo;
> - sabrás reemplazar con una función tipada y recorrer `matchAll`;
> - sabrás escribir un analizador que devuelve un objeto tipado o `null`, y una guarda
>   de tipo que valida un formato;
> - sabrás escapar un texto escrito por el usuario.

## Escribir una regex en TypeScript

Una [expresión regular](../debuter/glossaire.md#expresion-regular) (o **regex**)
describe un patrón de texto. La sintaxis de los patrones (`\d`, `+`, `[a-z]`, `^`,
`$`…) es exactamente la de JavaScript: se explica en
[las expresiones regulares en JavaScript](../javascript/regex.md). Esta página no la
repite: muestra lo que cambian los **tipos**.

Una regex se escribe entre dos barras, o se construye con `new RegExp` a partir de un
texto. En ambos casos, su tipo es `RegExp`.

```typescript
const codigoPostal: RegExp = /^\d{5}$/;
const palabra = "gato";
const patron = new RegExp(palabra, "i");

console.log(codigoPostal.test("28013"));
console.log(codigoPostal.test("280"));
console.log(patron.test("El GATO duerme."));
console.log(patron.source, patron.flags);
```

```sortie
true
false
true
gato i
```

`test` devuelve un `boolean`: es el método más sencillo, y no plantea ningún problema
de tipos.

> **Nota:** TypeScript también comprueba la **sintaxis** de una regex escrita entre
> barras. Un paréntesis olvidado en `/(abc/` es un error de compilación. Con
> `new RegExp("(abc")`, el patrón es un simple texto: el error solo aparece al
> ejecutar.

## match puede devolver null

`match` busca el patrón en un texto. Si lo encuentra, devuelve un array; si no,
devuelve `null`. Su tipo de retorno lo dice: `RegExpMatchArray | null`.

En modo estricto, TypeScript no te deja usar el resultado sin haber descartado `null`
(mira los [errores frecuentes](#errores-frecuentes)). Basta con un simple `if`: es el
[estrechamiento](../debuter/glossaire.md#estrechamiento).

```typescript
function primerNumero(texto: string): number | null {
  const m = texto.match(/\d+/);
  if (m === null) {
    return null;
  }
  // Aquí, m es un RegExpMatchArray.
  return Number(m[0]);
}

console.log(primerNumero("Quedan 12 plazas."));
console.log(primerNumero("Completo."));
```

```sortie
12
null
```

Para un valor por defecto, `?.` y `??` son más cortos:
`texto.match(/\d+/)?.[0] ?? "desconocido"`. `?.[0]` solo lee la casilla 0 si el
resultado no es `null`.

### Con el modificador g

Con el modificador `g`, `match` devuelve **todas** las coincidencias, en forma de array
de textos. Pero cuando no hay ninguna, sigue siendo `null`, no un array vacío:

```typescript
function numeros(texto: string): number[] {
  const encontrados = texto.match(/\d+/g) ?? [];
  return encontrados.map(Number);
}

console.log(numeros("3 manzanas, 12 peras, 7 kiwis"));
console.log(numeros("ninguna fruta"));
```

```sortie
[ 3, 12, 7 ]
[]
```

`?? []` sustituye `null` por un array vacío: el resto del código ya no tiene que
preocuparse por él.

## Los modificadores

Los modificadores (*flags*) se colocan después de la última barra: `/patron/gi`. El
capítulo de JavaScript los detalla; aquí tienes un recordatorio.

| Modificador | Efecto |
|-------------|--------|
| `g` | todas las coincidencias |
| `i` | ignora mayúsculas y minúsculas |
| `m` | `^` y `$` funcionan en cada línea |
| `s` | el `.` también acepta el salto de línea |
| `u` | lee bien los acentos y los emojis, activa `\p{…}` |
| `v` | una versión más reciente de `u`, con conjuntos de caracteres más ricos |

```typescript
const nombres = "Élise, zoé, Ana";
console.log(nombres.match(/\p{Lu}\p{Ll}+/gu));
console.log(/inicio.fin/s.test("inicio\nfin"));
```

```sortie
[ 'Élise', 'Ana' ]
true
```

`\p{Lu}` es una letra mayúscula de cualquier idioma, `\p{Ll}` una minúscula: `zoé`
queda fuera porque empieza por minúscula.

> **Atención:** TypeScript rechaza el modificador `v` si el objetivo de compilación
> (`target` en `tsconfig.json`) es anterior a `es2024`. El modificador `u` basta en
> casi todos los casos.

## Los grupos numerados

Unos paréntesis en el patrón forman **grupos**. El texto encontrado por el primer
grupo está en `m[1]`, el segundo en `m[2]`, y así sucesivamente. `m[0]` contiene toda
la coincidencia.

```typescript
const m = "Cita a las 14h30.".match(/(\d{1,2})h(\d{2})/);
if (m) {
  const horas = Number(m[1]);
  const minutos = Number(m[2]);
  console.log(`${m[0]} = ${horas * 60 + minutos} minutos`);
}
```

```sortie
14h30 = 870 minutos
```

TypeScript da el tipo `string` a `m[1]` y `m[2]`. No cuenta los grupos: no sabe
cuántos contiene tu patrón.

> **Atención:** un grupo **opcional** que no ha encontrado nada vale `undefined`,
> aunque TypeScript anuncie `string`. Para un grupo seguido de `?`, anota la variable
> con `string | undefined` (`const centimos: string | undefined = m[2];`) y trata el
> caso en que falta.

## Los grupos con nombre

Un grupo con nombre se escribe `(?<nombre>…)`. Se lee en `m.groups`, más legible que
los números. Pero cuidado con el tipo de `groups`:

```text
{ [key: string]: string } | undefined
```

Dos consecuencias:

- `groups` puede valer `undefined` (cuando el patrón no tiene ningún grupo con
  nombre): por eso hay que comprobarlo, como `m`;
- TypeScript no conoce los nombres de tus grupos: `groups.anio` y `groups.ani`
  (errata) tienen los dos el tipo `string`.

```typescript
const patron = /(?<anio>\d{4})-(?<mes>\d{2})-(?<dia>\d{2})/;
const m = "Entrega el 2026-10-19.".match(patron);

if (m?.groups) {
  const { anio, mes, dia } = m.groups;
  console.log(`${dia}/${mes}/${anio}`);
}
```

```sortie
19/10/2026
```

`m?.groups` comprueba de una vez que `m` no es `null` y que `groups` existe. Dentro
del `if`, TypeScript sabe que `m.groups` es un objeto.

## Recorrer con matchAll

`matchAll` devuelve un [iterador](../debuter/glossaire.md#iterador): cada elemento es
una coincidencia completa, con sus grupos. El patrón debe tener el modificador `g`. Se
recorre con `for…of`, y cada elemento tiene el tipo `RegExpExecArray`.

```typescript
const cesta = "manzanas=3; peras=12; kiwis=7";
let total = 0;

for (const m of cesta.matchAll(/(?<fruta>\w+)=(?<cantidad>\d+)/g)) {
  const fruta = m.groups?.fruta ?? "?";
  const cantidad = Number(m.groups?.cantidad ?? 0);
  console.log(`${fruta.padEnd(8)} ${cantidad} (posición ${m.index})`);
  total += cantidad;
}
console.log(`Total: ${total}`);
```

```sortie
manzanas 3 (posición 0)
peras    12 (posición 12)
kiwis    7 (posición 22)
Total: 22
```

A diferencia de `match`, `matchAll` nunca devuelve `null`: sin coincidencias, el
iterador simplemente está vacío. `Array.from(…)` lo convierte en un array de verdad.

## Reemplazar con una función tipada

`replace` y `replaceAll` aceptan una función en lugar del texto de reemplazo. Recibe la
coincidencia y luego un argumento por grupo. Si la pasas directamente, TypeScript
adivina los tipos (`string` para cada grupo). Si escribes la función aparte, tipa tú
mismo sus parámetros. Aquí pasamos fechas al formato español:

```typescript
function aEspanol(_todo: string, anio: string, mes: string, dia: string): string {
  return `${dia}/${mes}/${anio}`;
}

const registro = "Inicio: 2026-10-19, fin: 2026-12-31.";
console.log(registro.replaceAll(/(\d{4})-(\d{2})-(\d{2})/g, aEspanol));
```

```sortie
Inicio: 19/10/2026, fin: 31/12/2026.
```

El primer parámetro es toda la coincidencia; el `_` del nombre indica que no se usa.

> **Nota:** `replaceAll` con una regex exige el modificador `g`; si no, lanza un
> `TypeError` al ejecutar. TypeScript no lo detecta.

## Dividir con split

`split` acepta una regex como separador. El resultado siempre es un `string[]`: no hay
`null` que gestionar.

```typescript
const entrada = "rojo, verde ;azul  ;  amarillo";
const colores: string[] = entrada.split(/\s*[,;]\s*/);
console.log(colores);
```

```sortie
[ 'rojo', 'verde', 'azul', 'amarillo' ]
```

El separador «una coma o un punto y coma, con espacios alrededor» limpia la entrada en
una sola línea.

## Escribir un analizador tipado

La gran ventaja de TypeScript: encerrar la regex en una función que transforma un
texto en un **objeto tipado**. El resto del programa ya no maneja textos, sino objetos
cuyos campos están todos comprobados.

### Devolver T o null

Esta es una función que lee una fecha `2026-10-19`. Devuelve un objeto `FechaSimple`,
o `null` si el texto no tiene la forma correcta:

```typescript
interface FechaSimple {
  anio: number;
  mes: number;
  dia: number;
}

function leerFecha(texto: string): FechaSimple | null {
  const m = texto.match(/^(?<anio>\d{4})-(?<mes>\d{2})-(?<dia>\d{2})$/);
  if (!m?.groups) {
    return null;
  }
  const anio = Number(m.groups.anio);
  const mes = Number(m.groups.mes);
  const dia = Number(m.groups.dia);
  if (mes < 1 || mes > 12 || dia < 1 || dia > 31) {
    return null;
  }
  return { anio, mes, dia };
}

for (const texto of ["2026-10-19", "2026-13-01", "19/10/2026"]) {
  const f = leerFecha(texto);
  console.log(texto, "->", f ?? "fecha no válida");
}
```

```sortie
2026-10-19 -> { anio: 2026, mes: 10, dia: 19 }
2026-13-01 -> fecha no válida
19/10/2026 -> fecha no válida
```

La regex comprueba la **forma** (cuatro cifras, un guion…); el código comprueba el
**sentido** (un mes entre 1 y 12). Las dos cosas son útiles.

### Devolver una unión discriminada

Cuando hay varias formas posibles, devuelve una
[unión discriminada](unions.md#las-uniones-discriminadas). Aquí, una línea de registro (*log*) es una petición,
un error o una línea desconocida:

```typescript
type Linea =
  | { kind: "peticion"; metodo: "GET" | "POST"; ruta: string; duracion: number }
  | { kind: "error"; mensaje: string }
  | { kind: "desconocida"; texto: string };

function leerLinea(texto: string): Linea {
  const pet = texto.match(/^(?<metodo>GET|POST) (?<ruta>\S+) (?<duracion>\d+)ms$/);
  if (pet?.groups) {
    const metodo = pet.groups.metodo === "GET" ? "GET" : "POST";
    return { kind: "peticion", metodo, ruta: pet.groups.ruta, duracion: Number(pet.groups.duracion) };
  }
  const err = texto.match(/^ERROR: (?<mensaje>.+)$/);
  if (err?.groups) {
    return { kind: "error", mensaje: err.groups.mensaje };
  }
  return { kind: "desconocida", texto };
}

const registro = ["GET /inicio 12ms", "ERROR: base de datos no disponible", "POST /cesta 85ms", "???"];

for (const texto of registro) {
  const linea = leerLinea(texto);
  switch (linea.kind) {
    case "peticion":
      console.log(`${linea.metodo} ${linea.ruta} en ${linea.duracion} ms`);
      break;
    case "error":
      console.log(`Problema: ${linea.mensaje}`);
      break;
    case "desconocida":
      console.log(`Ignorada: ${linea.texto}`);
      break;
  }
}
```

```sortie
GET /inicio en 12 ms
Problema: base de datos no disponible
POST /cesta en 85 ms
Ignorada: ???
```

En cada `case`, TypeScript sabe qué campos existen: `linea.duracion` solo está
permitido para una petición. El texto en bruto se analiza en un único sitio.

## Validar con una guarda de tipo

Una regex también puede servir de **guarda de tipo**: una función que devuelve
`v is T` (mira [tus propias guardas de tipo](unions.md#tus-propias-guardas-de-tipo)). Aquí, el tipo `Email`
describe «un texto que contiene una `@`», y la regex hace la comprobación de verdad:

```typescript
type Email = `${string}@${string}`;

function esEmail(texto: string): texto is Email {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(texto);
}

function enviar(destinatario: Email) {
  console.log(`Envío a ${destinatario}`);
}

for (const entrada of ["ada@ejemplo.es", "ada@", "no es un correo"]) {
  if (esEmail(entrada)) {
    enviar(entrada);
  } else {
    console.log(`Dirección rechazada: ${entrada}`);
  }
}
```

```sortie
Envío a ada@ejemplo.es
Dirección rechazada: ada@
Dirección rechazada: no es un correo
```

`enviar` solo acepta un `Email`: es imposible pasarle por descuido un texto sin
comprobar. El mismo modelo sirve para un código postal:

```typescript
type CodigoPostal = `${number}`;

function esCodigoPostal(texto: string): texto is CodigoPostal {
  return /^\d{5}$/.test(texto);
}

const entradas = ["28013", "2801", "28O13"];
console.log(entradas.filter(esCodigoPostal));
```

```sortie
[ '28013' ]
```

Con `filter`, la guarda de tipo estrecha todo el array: el resultado tiene el tipo
`CodigoPostal[]`.

> **Consejo:** no existe una regex de correo perfecta. Hazla sencilla («algo, una `@`,
> algo, un punto, algo») y envía un correo de confirmación para estar seguro.

## ¿Tipos de plantilla o regex?

Los [template literal types](types-avances.md) como `` `${string}@${string}` ``
se parecen a patrones. Pero no tienen el mismo papel:

| | Template literal type | Regex |
|---|---|---|
| ¿Cuándo? | al compilar | al ejecutar |
| ¿Sobre qué? | los textos escritos en el código | cualquier texto, incluso escrito por el usuario |
| Precisión | gruesa: `${number}` acepta `"1.5"` | fina: `\d{5}` exige 5 cifras |

Por ejemplo, `"99h99"` sí tiene el tipo `` `${number}h${number}` ``, pero la regex
`/^([01]\d|2[0-3])h[0-5]\d$/` lo rechaza. El tipo sirve para documentar y guiar; la
regex comprueba de verdad. Para un texto que viene de fuera, solo la regex (dentro de
una guarda de tipo) protege tu programa.

## Escapar una entrada

Para buscar un texto escrito por el usuario, se construye la regex con `new RegExp`.
Pero si el texto contiene `.`, `(`, `+` o `?`, esos caracteres conservan su sentido
especial: la búsqueda se vuelve errónea, o lanza un error.

Hace poco se añadió a JavaScript un método `RegExp.escape`. Todavía no existe en
Node 22, y TypeScript 5.8 no lo conoce (mira los
[errores frecuentes](#errores-frecuentes)). Así que escribe tu propia función:

```typescript
function escapar(texto: string): string {
  return texto.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function contar(texto: string, busqueda: string): number {
  const patron = new RegExp(escapar(busqueda), "gi");
  return texto.match(patron)?.length ?? 0;
}

console.log(escapar("1+1=2 (¿verdad?)"));
console.log(contar("Precio: 3.5 €, o 3.5 € en oferta, no 345 €", "3.5"));
```

```sortie
1\+1=2 \(¿verdad\?\)
2
```

En el reemplazo, `$&` representa el carácter encontrado: cada carácter especial queda
precedido de una barra invertida. Sin `escapar`, el `.` de `3.5` también aceptaría el
`345`.

## Errores frecuentes

### Usar el resultado de match sin comprobar null

```typescript norun
const m = "2026-10-19".match(/(\d{4})-(\d{2})/);
console.log(m[1]);
```

```text
main.ts(2,13): error TS18047: 'm' is possibly 'null'.
```

`match` devuelve `null` cuando no encuentra nada. Compruébalo antes: `if (m) { … }`, o
lee con `m?.[1]`.

### Leer groups sin comprobar undefined

```typescript norun
const m = "2026-10-19".match(/(?<anio>\d{4})/);
if (m) {
  console.log(m.groups.anio);
}
```

```text
main.ts(3,15): error TS18048: 'm.groups' is possibly 'undefined'.
```

Comprobar `m` no basta: `groups` también puede faltar. Escribe
`if (m?.groups) { … }`.

### Usar el modificador v con un objetivo antiguo

```typescript norun
const emoji = /\p{Emoji}/v;
```

```text
main.ts(1,26): error TS1501: This regular expression flag is only available when targeting 'es2024' or later.
```

Cambia `target` a `es2024` (o posterior) en `tsconfig.json`, o usa el modificador `u`:
`/\p{Emoji}/u`.

### Llamar a RegExp.escape

```typescript norun
const patron = new RegExp(RegExp.escape("3.5"));
```

```text
main.ts(1,34): error TS2339: Property 'escape' does not exist on type 'RegExpConstructor'.
```

`RegExp.escape` es demasiado reciente para TypeScript 5.8 y para Node 22 (donde vale
`undefined`). Usa tu propia función `escapar`, vista más arriba.

## Ponte a prueba

**Pregunta 1:** ¿cuál es el tipo de retorno de `texto.match(/\d+/)`, y qué hay que
hacer antes de leer `[0]`?

> **Respuesta**
>
> `RegExpMatchArray | null`. Hay que descartar `null`: con un `if (m)`, o con
> `m?.[0]` seguido de un valor de reserva `?? …`.

**Pregunta 2:** ¿por qué se rechaza `if (m) { m.groups.nombre }`?

> **Respuesta**
>
> `groups` tiene el tipo `{ [key: string]: string } | undefined`: puede faltar aunque
> `m` exista. Hay que comprobar `m.groups`, por ejemplo con `if (m?.groups)`.

**Pregunta 3:** escribe una guarda de tipo `esHora(texto: string): texto is Hora` para
``type Hora = `${number}:${number}` ``, que acepte `08:30` pero rechace `25:00`.

> **Respuesta**
>
> ```typescript
> type Hora = `${number}:${number}`;
>
> function esHora(texto: string): texto is Hora {
>   return /^([01]\d|2[0-3]):[0-5]\d$/.test(texto);
> }
>
> console.log(esHora("08:30"), esHora("25:00"));
> ```
>
> ```sortie
> true false
> ```

## En resumen

- La sintaxis de las regex es la de JavaScript; TypeScript les da el tipo `RegExp`.
- `match` devuelve `RegExpMatchArray | null`: descarta `null` con `if`, `?.` o `??`.
- `m.groups` es `{ [key: string]: string } | undefined`: compruébalo, y recuerda que
  TypeScript no conoce los nombres de tus grupos.
- Encierra cada regex en una función que devuelva un objeto tipado, `T | null` o una
  unión discriminada.
- Una regex dentro de una guarda de tipo `texto is T` valida al ejecutar lo que los
  tipos solo comprueban al compilar.
- Escapa toda entrada antes de `new RegExp` con tu propia función `escapar`.

Siguiente: [TypeScript para la web](web.md).
