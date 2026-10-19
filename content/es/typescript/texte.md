# Trabajar con texto

Esta página muestra cómo manejar texto en TypeScript: medir, buscar, cortar, reemplazar, formatear números y alinear columnas, con tipos que protegen cada paso.

> **Objetivos:**
>
> - sabrás usar los métodos habituales de las cadenas: `toUpperCase`, `includes`,
>   `slice`, `split`, `replace`…;
> - sabrás leer el tipo que devuelve cada método;
> - sabrás convertir un texto en número y al revés;
> - sabrás limitar un texto a unos pocos valores con un tipo como `"sí" | "no"`;
> - sabrás formatear números con `Intl` y alinear columnas con `padStart`.

## Las cadenas de caracteres

Un texto es una [cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres),
de tipo `string`. Cada carácter tiene una posición, su
[índice](../debuter/glossaire.md#indice), que empieza en **0**.

```typescript
const palabra: string = "Saludos";

console.log(palabra.length);
console.log(palabra[0]);
console.log(palabra[3]);
console.log(palabra.at(-1));
```

```sortie
7
S
u
s
```

- `length` da el número de caracteres. Es una **propiedad**: sin paréntesis.
- `palabra[0]` da el primer carácter, `palabra[3]` el cuarto.
- `at(-1)` cuenta desde el final: `-1` es el último carácter.

Un [método](../debuter/glossaire.md#metodo) es una acción que un valor sabe hacer. Se
llama con un punto y paréntesis: `palabra.toUpperCase()`. Tu editor conoce todos los
métodos de un `string`: escribe `palabra.` y te los propone, con su tipo.

## Cambiar mayúsculas y minúsculas, y limpiar

```typescript
const entrada = "  Marie Curie  ";

console.log(`[${entrada.toUpperCase()}]`);
console.log(`[${entrada.toLowerCase()}]`);
console.log(`[${entrada.trim()}]`);
console.log(`[${entrada.trimStart()}]`);
```

```sortie
[  MARIE CURIE  ]
[  marie curie  ]
[Marie Curie]
[Marie Curie  ]
```

- `toUpperCase()` pasa a mayúsculas, `toLowerCase()` a minúsculas;
- `trim()` quita los espacios del principio y del final; `trimStart()` solo los del
  principio.

Estos métodos no modifican `entrada`: **devuelven un texto nuevo**. Una cadena es
[inmutable](../debuter/glossaire.md#inmutable): nunca se puede cambiar en su sitio.
Además, TypeScript lo prohíbe: `palabra[0] = "s"` se rechaza (mira los
[errores frecuentes](#errores-frecuentes)).

> **Consejo:** aplica `trim()` a todo texto que escriba un usuario: los espacios de más
> son una causa clásica de comparaciones fallidas.

## Buscar en un texto

Estos métodos responden a una pregunta. Su tipo de retorno te dice qué forma tiene la
respuesta:

| Método | Pregunta | Devuelve |
|---------|----------|---------|
| `includes(x)` | ¿contiene `x`? | `boolean` |
| `startsWith(x)` | ¿empieza por `x`? | `boolean` |
| `endsWith(x)` | ¿termina en `x`? | `boolean` |
| `indexOf(x)` | ¿en qué posición está `x`? | `number` (`-1` si no está) |

```typescript
const frase = "El gato duerme en el sofá.";

console.log(frase.includes("gato"));
console.log(frase.startsWith("El"));
console.log(frase.endsWith("!"));
console.log(frase.indexOf("duerme"));
console.log(frase.indexOf("perro"));
```

```sortie
true
true
false
8
-1
```

Todas estas búsquedas distinguen mayúsculas y minúsculas: `"Gato"` no es `"gato"`.
Para buscar sin tener en cuenta las mayúsculas, pásalo todo a minúsculas primero.

```typescript
const titulo = "Aprender TypeScript";
console.log(titulo.toLowerCase().includes("typescript"));
```

```sortie
true
```

## Extraer un trozo con slice

`slice(inicio, fin)` recorta una parte del texto, desde el carácter `inicio`
**incluido** hasta el carácter `fin` **excluido**. Sin `fin`, llega hasta el final. Un
número negativo cuenta desde el final.

```typescript
const archivo = "informe-2024.pdf";

console.log(archivo.slice(0, 7));
console.log(archivo.slice(8, 12));
console.log(archivo.slice(-3));
```

```sortie
informe
2024
pdf
```

Piensa en las posiciones como en las rayas de una regla: `slice(0, 7)` toma todo lo
que hay entre la raya 0 y la raya 7.

## Cortar y volver a unir

`split(separador)` corta un texto en trozos. Devuelve un **array de textos**, de tipo
`string[]`. `join(separador)` hace lo contrario: vuelve a unir un array en un solo
texto.

```typescript
const linea = "manzana;pera;kiwi";
const frutas: string[] = linea.split(";");

console.log(frutas);
console.log(frutas.length);
console.log(frutas.join(", "));
console.log("abc".split(""));
```

```sortie
[ 'manzana', 'pera', 'kiwi' ]
3
manzana, pera, kiwi
[ 'a', 'b', 'c' ]
```

`split("")` corta entre cada carácter. Los arrays se explican en detalle en
[el capítulo sobre los arrays](collections.md).

## Reemplazar

`replace(a, b)` reemplaza la **primera** aparición de `a` por `b`. `replaceAll`
reemplaza todas las apariciones.

```typescript
const texto = "El gato ve al gato.";

console.log(texto.replace("gato", "perro"));
console.log(texto.replaceAll("gato", "perro"));
console.log("06 12 34 56 78".replaceAll(" ", ""));
```

```sortie
El perro ve al gato.
El perro ve al perro.
0612345678
```

## Convertir entre texto y número

Un texto que contiene cifras sigue siendo un texto: `"42"` no es `42`. TypeScript
impide que los confundas. Para convertir, usa:

- `Number(texto)`: texto → número (da `NaN`, «no es un número», si es imposible);
- `String(numero)` o `numero.toString()`: número → texto.

```typescript
const entrada = "42";
const numero: number = Number(entrada);

console.log(numero + 1);
console.log(entrada + 1);
console.log(Number("3,5"), Number("3.5"));
console.log(String(7) + String(7));
```

```sortie
43
421
NaN 3.5
77
```

- `numero + 1` suma: `43`.
- `entrada + 1` pega los textos: `"421"`.
- `Number("3,5")` falla: en el código, los decimales se escriben con un punto.

`NaN` es de tipo `number`, pero no es un número de verdad. Compruébalo con
`Number.isNaN`:

```typescript
const intentos = ["12", "doce"];

for (const intento of intentos) {
  const n = Number(intento);
  if (Number.isNaN(n)) {
    console.log(`"${intento}" no es un número.`);
  } else {
    console.log(`${n} x 2 = ${n * 2}`);
  }
}
```

```sortie
12 x 2 = 24
"doce" no es un número.
```

## Los tipos literales: "sí" | "no"

Esta es una ventaja propia de TypeScript. Un tipo puede aceptar **solo ciertos
textos**. Se enumeran los valores permitidos, separados por `|` («o»):

```typescript
type Respuesta = "sí" | "no";

function responder(r: Respuesta): string {
  return r === "sí" ? "¡Perfecto!" : "Qué pena.";
}

console.log(responder("sí"));
console.log(responder("no"));
```

```sortie
¡Perfecto!
Qué pena.
```

`"sí"` y `"no"` son **tipos literales**: cada uno acepta un solo texto. Su unión
`"sí" | "no"` acepta uno u otro, y nada más. Es como un formulario con casillas para
marcar en lugar de un campo libre: imposible responder «bah».

- Tu editor propone `"sí"` y `"no"` cuando escribes el argumento.
- Una errata (`"sii"`) se rechaza antes de la ejecución.
- Un `switch` sobre este tipo puede comprobar que no se olvida ningún caso (mira
  [condiciones y bucles](conditions-boucles.md#un-switch-que-no-olvida-ningun-caso)).

```typescript
type Color = "rojo" | "verde" | "azul";

const favorito: Color = "verde";
const todos: Color[] = ["rojo", "verde", "azul"];

console.log(favorito, todos.length);
```

```sortie
verde 3
```

> **Recuerda:** cuando un texto solo puede tomar unos pocos valores (un estado, una
> dirección, un tamaño), usa una unión de tipos literales en lugar de `string`.

### Los tipos plantilla, en breve

TypeScript puede incluso describir la **forma** de un texto, con la sintaxis de las
plantillas. Son los *template literal types*:

```typescript
type Pixeles = `${number}px`;

const ancho: Pixeles = "120px";
const alto: Pixeles = "80px";

console.log(ancho, alto);
```

```sortie
120px 80px
```

`` `${number}px` `` quiere decir «un número seguido de `px`». `"120"` o `"120 px"` se
rechazarían. Volverás a ver esta idea en [los tipos literales](litteraux.md).

## Formatear números con Intl

Para mostrar un número como se escribe en Francia (espacio entre los millares, coma
decimal, símbolo €), usa `Intl.NumberFormat`. `Intl` (*internacionalización*) conoce
las costumbres de cada país.

```typescript
const importe = 1234567.891;

const numeroFr = new Intl.NumberFormat("fr-FR");
const euros = new Intl.NumberFormat("fr-FR", { style: "currency", currency: "EUR" });
const dolares = new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" });
const porcentaje = new Intl.NumberFormat("fr-FR", { style: "percent" });

console.log(numeroFr.format(importe));
console.log(euros.format(importe));
console.log(dolares.format(importe));
console.log(porcentaje.format(0.256));
```

```sortie
1 234 567,891
1 234 567,89 €
$1,234,567.89
26 %
```

- `"fr-FR"` quiere decir «francés de Francia», `"en-US"` «inglés de Estados Unidos».
- `style: "currency"` muestra una moneda; `currency` indica cuál.
- `style: "percent"` multiplica por 100 y añade `%`.

> **Nota:** los espacios del resultado francés son **espacios de no separación**:
> impiden un salto de línea en medio de un número. Parecen espacios normales, pero son
> otros caracteres. Así que no compares este resultado con un texto escrito a mano.

Las opciones están tipadas: `{ style: "moneda" }` se rechazaría, porque `style` solo
acepta unos pocos textos precisos (`"decimal"`, `"currency"`, `"percent"`, `"unit"`).
¡Otra vez una unión de tipos literales!

Para elegir el número de decimales, usa `maximumFractionDigits`:

```typescript
const corto = new Intl.NumberFormat("fr-FR", { maximumFractionDigits: 1 });
console.log(corto.format(3.14159));
console.log(corto.format(2));
```

```sortie
3,1
2
```

## Alinear columnas con padStart y padEnd

`padStart(longitud, relleno)` completa un texto **por el principio** hasta la longitud
deseada. `padEnd` completa **por el final**. Sin relleno, se usan espacios.

```typescript
console.log("7".padStart(3, "0"));
console.log("42".padStart(5) + "|");
console.log("ok".padEnd(5, ".") + "|");
```

```sortie
007
   42|
ok...|
```

Es perfecto para mostrar una tabla bien alineada: los textos a la izquierda con
`padEnd`, los números a la derecha con `padStart`.

```typescript
function linea(articulo: string, precio: number): string {
  return articulo.padEnd(12, ".") + precio.toFixed(2).padStart(8);
}

console.log(linea("Pan", 1.2));
console.log(linea("Queso", 12.5));
console.log(linea("Café", 104));
console.log("-".repeat(20));
console.log("Total".padEnd(12) + (1.2 + 12.5 + 104).toFixed(2).padStart(8));
```

```sortie
Pan.........    1.20
Queso.......   12.50
Café........  104.00
--------------------
Total         117.70
```

`repeat(20)` repite un texto 20 veces. Los puntos decimales de los precios quedan
alineados, como en un tique de caja.

## Errores frecuentes

### Modificar un carácter de una cadena

```typescript norun
let palabra = "gato";
palabra[0] = "G";
```

```text
main.ts:2:1 - error TS2542: Index signature in type 'String' only permits reading.

2 palabra[0] = "G";
  ~~~~~~~~~~


Found 1 error in main.ts:2
```

«Las posiciones de un `String` son de solo lectura.» Una cadena es inmutable. Construye
un texto nuevo: `palabra = "G" + palabra.slice(1);`.

### Llamar a length como a un método

```typescript norun
const nombre = "Alice";
console.log(nombre.length());
```

```text
main.ts:2:20 - error TS2349: This expression is not callable.
  Type 'Number' has no call signatures.

2 console.log(nombre.length());
                     ~~~~~~


Found 1 error in main.ts:2
```

«Esta expresión no se puede llamar: es un número.» `length` es una propiedad, no un
método. Escribe `nombre.length`, sin paréntesis.

### Sumar un texto y un número

```typescript norun
const edad = "30";
const edadFutura: number = edad + 10;
```

```text
main.ts:2:7 - error TS2322: Type 'string' is not assignable to type 'number'.

2 const edadFutura: number = edad + 10;
        ~~~~~~~~~~


Found 1 error in main.ts:2
```

`edad` es un texto, así que `edad + 10` pega `"30"` y `"10"`: el resultado es el texto
`"3010"`. Convierte primero: `Number(edad) + 10`.

### Dar un valor que no está en la lista

```typescript norun
type Respuesta = "sí" | "no";
function responder(r: Respuesta): void {
  console.log(`Dijiste ${r}.`);
}
responder("quizás");
```

```text
main.ts:5:11 - error TS2345: Argument of type '"quizás"' is not assignable to parameter of type 'Respuesta'.

5 responder("quizás");
            ~~~~~~~~


Found 1 error in main.ts:5
```

`Respuesta` solo acepta `"sí"` o `"no"`. Usa un valor permitido, o añade `"quizás"` al
tipo si es una respuesta posible de verdad.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra `"TypeScript".slice(4, 10).toLowerCase()`?

> **Respuesta**
>
> `script`. `slice(4, 10)` toma los caracteres de las posiciones 4 a 9 (`Script`), y
> luego `toLowerCase()` los pasa a minúsculas.

**Pregunta 2:** ¿cuál es el tipo de `"a-b-c".split("-")`?

> **Respuesta**
>
> `string[]`, un array de textos: `["a", "b", "c"]`.

**Pregunta 3:** escribe un tipo `Direccion` que acepte solo `"arriba"`, `"abajo"`,
`"izquierda"` y `"derecha"`, y luego una variable de ese tipo.

> **Respuesta**
>
> ```typescript
> type Direccion = "arriba" | "abajo" | "izquierda" | "derecha";
>
> const d: Direccion = "izquierda";
> console.log(d);
> ```
>
> ```sortie
> izquierda
> ```

## En resumen

- Un `string` es inmutable: sus métodos siempre devuelven un texto nuevo.
- `length` mide, `includes` / `startsWith` / `indexOf` buscan, `slice` extrae, `split`
  corta en un `string[]`, `replaceAll` reemplaza.
- `Number(texto)` convierte en número (`NaN` si es imposible), `String(n)` en texto.
- Una unión de tipos literales como `"sí" | "no"` limita un texto a unos pocos
  valores, comprobados antes de la ejecución.
- `Intl.NumberFormat("fr-FR", ...)` formatea números, monedas y porcentajes;
  `padStart` y `padEnd` alinean columnas.

Siguiente: [arrays y tuplas](collections.md).
