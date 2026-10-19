# Primeros pasos

Esta página muestra cómo guardar valores en variables con `let` y `const`, cuáles son los tipos básicos de JavaScript y cómo compararlos.

> **Objetivos:**
>
> - sabrás crear una variable con `let` o `const`, y sabrás cuál elegir;
> - conocerás los tipos básicos: número, texto, booleano, `null`, `undefined`;
> - sabrás mostrar varios valores y construir un texto con `${...}`;
> - sabrás comentar tu código y nombrar bien tus variables;
> - sabrás por qué hay que comparar con `===` en lugar de con `==`.

## Las variables con let y const

Una [variable](../debuter/glossaire.md#variable) es una caja con una etiqueta: guardas
un valor en ella y luego lo encuentras por su nombre. En JavaScript, una
variable se crea con `const` o `let`.

```javascript
const nombre = "Alice";
let edad = 30;

console.log(nombre);
console.log(edad);
```

```sortie
Alice
30
```

El signo `=` no significa «igual»: significa «guarda este valor en esta
caja». Se dice que **asignas** un valor.

### const: una caja sellada

`const` (de *constant*) crea una variable que ya no se puede reasignar. `let` crea una
variable que se puede cambiar más tarde.

```javascript
let puntos = 0;
puntos = puntos + 10;
puntos += 5;
console.log(puntos);

const pi = 3.14159;
console.log(pi);
```

```sortie
15
3.14159
```

`puntos += 5` es un atajo de `puntos = puntos + 5`.

> **Recuerda:** usa `const` por defecto. Pasa a `let` solo si el valor
> tiene que cambiar. Al leer `const`, quien lee sabe enseguida que el valor no se moverá
> más.

### ¿Y var?

A menudo verás `var` en código antiguo o en Internet. Es la forma antigua de
crear una variable, de antes de 2015. Tiene comportamientos sorprendentes: se puede
declarar dos veces la misma variable sin error, y «se escapa» de los bloques `{ }`.

```javascript
var ciudad = "Lima";
var ciudad = "Quito";
console.log(ciudad);
```

```sortie
Quito
```

Con `let`, JavaScript te habría avisado del error. **No uses `var`**:
`let` y `const` hacen lo mismo, de forma más segura.

## Los tipos básicos

Cada valor tiene un [tipo](../debuter/glossaire.md#tipo): es su naturaleza (un número, un
texto…). El tipo dice qué se puede hacer con el valor. El operador `typeof` da el
tipo de un valor, en forma de texto.

```javascript
console.log(typeof 42);
console.log(typeof 3.5);
console.log(typeof "hola");
console.log(typeof true);
console.log(typeof undefined);
console.log(typeof 10n);
```

```sortie
number
number
string
boolean
undefined
bigint
```

| Tipo | Ejemplos | Papel |
| --- | --- | --- |
| `number` | `42`, `3.5`, `-7` | todos los números, enteros o decimales |
| `string` | `"hola"`, `'buenas'` | texto |
| `boolean` | `true`, `false` | verdadero o falso |
| `undefined` | `undefined` | «todavía sin valor» |
| `null` | `null` | «vacío, a propósito» |
| `bigint` | `10n` | enteros gigantes (mira [los números](nombres.md)) |
| `symbol` | `Symbol("id")` | un identificador único (raro, para más adelante) |

- Un **número** se escribe con punto, no con coma: `3.5`.
- Una [cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres) (un texto)
  se escribe entre comillas dobles `"..."` o simples `'...'`.
- Un [booleano](../debuter/glossaire.md#booleano) vale `true` (verdadero) o `false` (falso).

### undefined y null

Dos valores significan «nada», con un matiz.

- `undefined`: la caja existe, pero **nadie ha metido nada en ella**. Es el valor
  de una variable declarada sin valor.
- `null`: la caja está **vacía a propósito**. Lo escribes tú, para decir «ningún
  valor» (mira [null](../debuter/glossaire.md#null)).

```javascript
let respuesta;
console.log(respuesta);

let ganador = null;
console.log(ganador);
console.log(typeof null);
```

```sortie
undefined
null
object
```

> **Atención:** `typeof null` responde `"object"`. Es un error histórico de
> JavaScript, que se mantiene para no romper los sitios antiguos. Simplemente, recuérdalo.

## Las plantillas de texto

Para meter un valor en un texto, escribe el texto entre **acentos graves**
`` `...` `` (en un teclado español, la tecla a la derecha de la **P**, seguida de un espacio), y el valor dentro de `${...}`.
Es una **plantilla** (*template literal*).

```javascript
const nombre = "Alice";
const edad = 30;

console.log(`¡Hola, ${nombre}!`);
console.log(`Dentro de 5 años, tendrás ${edad + 5} años.`);
```

```sortie
¡Hola, Alice!
Dentro de 5 años, tendrás 35 años.
```

Dentro de `${...}`, puedes poner cualquier cálculo. La forma antigua usa `+`
para pegar los trozos: `"¡Hola, " + nombre + "!"`. La plantilla se lee mejor.

> **Atención:** `${...}` solo funciona entre acentos graves. Entre comillas
> `"..."`, se muestra tal cual.

## Mostrar varios valores

`console.log` acepta varios valores, separados por comas. Los muestra en
una línea, separados por un espacio.

```javascript
const articulo = "café";
const precio = 2.5;

console.log("Artículo:", articulo, "- precio:", precio);
console.log(articulo, precio, true, null);
```

```sortie
Artículo: café - precio: 2.5
café 2.5 true null
```

> **Consejo:** para depurar, escribe `console.log("precio", precio)`: el nombre delante del
> valor te evita confundir las líneas.

## Los comentarios

Un [comentario](../debuter/glossaire.md#comentario) es una nota para las personas.
JavaScript lo ignora.

```javascript
// Un comentario en una línea
const iva = 0.2; // también al final de la línea

/*
  Un comentario
  en varias líneas
*/
console.log(100 * (1 + iva));
```

```sortie
120
```

Un buen comentario explica **por qué**, no **qué**: el código ya dice lo que hace.

## Nombrar bien las variables

En JavaScript, los nombres se escriben en **camelCase** («en jorobas de camello»): la
primera palabra en minúsculas y luego una mayúscula al principio de cada palabra siguiente.

```javascript
const precioUnitario = 4;
const numeroArticulos = 3;
const precioTotal = precioUnitario * numeroArticulos;

console.log(precioTotal);
```

```sortie
12
```

Algunas reglas:

- un nombre contiene letras, cifras, `_` o `$`, pero **no empieza por una
  cifra**;
- las mayúsculas cuentan: `edad` y `Edad` son dos variables distintas;
- algunas palabras están reservadas (`let`, `const`, `if`, `function`…);
- elige nombres que digan lo que contiene la caja: `precioTotal` mejor que `x`.

> **Nota:** los acentos están permitidos (`const año = 2026`), pero lo habitual es evitarlos
> en los nombres de variables. Guárdalos para el texto entre comillas.

## Comparar: ¿== o ===?

Comparar dos valores da un booleano. JavaScript tiene dos operadores de igualdad:

- `===` (igualdad **estricta**): verdadero si los dos valores tienen el mismo tipo **y** el
  mismo valor;
- `==` (igualdad **amplia**): primero convierte los valores para que tengan el mismo
  tipo y luego compara.

```javascript
console.log(5 === 5);
console.log(5 === "5");
console.log(5 == "5");
console.log(0 == "");
console.log(null == undefined);
console.log(null === undefined);
```

```sortie
true
false
true
true
true
false
```

Con `==`, el texto `"5"` se convierte en número, y `0 == ""` responde `true`: casi
nunca es lo que quieres. Las reglas de conversión son largas y están llenas de
trampas.

> **Recuerda:** usa siempre `===` y su contrario `!==` («distinto de»).
> Olvídate de `==` y `!=`.

## Errores frecuentes

### Cambiar una constante

```javascript norun
const edad = 20;
edad = 21;
```

```text
file:///home/alice/js/main.mjs:2
edad = 21;
     ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:2:6

Node.js v22.22.0
```

No se puede reasignar una variable `const` (*assignment to constant variable*,
«asignación a una variable constante»). Si el valor tiene que cambiar, declárala con
`let edad = 20;`.

### Declarar dos veces la misma variable

```javascript norun
let puntos = 0;
let puntos = 10;
```

```text
file:///home/alice/js/main.mjs:2
let puntos = 10;
    ^

SyntaxError: Identifier 'puntos' has already been declared

Node.js v22.22.0
```

`let` sirve para **crear** la variable, una sola vez. Para modificarla después, escribe
solo `puntos = 10;`, sin `let`.

### Usar una variable antes de crearla

```javascript norun
console.log(total);
let total = 5;
```

```text
file:///home/alice/js/main.mjs:1
console.log(total);
            ^

ReferenceError: Cannot access 'total' before initialization
    at file:///home/alice/js/main.mjs:1:13

Node.js v22.22.0
```

JavaScript lee de arriba abajo. En la línea 1, `total` todavía no tiene valor (*before
initialization*, «antes de ser inicializada»). Declara la variable antes de
usarla.

### Equivocarse de mayúscula

```javascript norun
const precioTotal = 12;
console.log(preciototal);
```

```text
file:///home/alice/js/main.mjs:2
console.log(preciototal);
            ^

ReferenceError: preciototal is not defined
    at file:///home/alice/js/main.mjs:2:13

Node.js v22.22.0
```

`preciototal` y `precioTotal` son dos nombres distintos. Copia el nombre exactamente.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```javascript norun
let n = 4;
n = n * 2;
n += 1;
console.log(`n vale ${n}`);
```

> **Respuesta**
>
> `n vale 9`. `n` pasa de 4 a 8 y luego a 9.

**Pregunta 2:** ¿cuál es la diferencia entre `undefined` y `null`?

> **Respuesta**
>
> `undefined` significa «todavía sin valor»: es JavaScript quien se lo da a una
> variable vacía. `null` significa «vacío a propósito»: lo escribes tú.

**Pregunta 3:** ¿qué responden `"3" == 3` y `"3" === 3`? ¿Cuál hay que usar?

> **Respuesta**
>
> ```javascript
> console.log("3" == 3);
> console.log("3" === 3);
> ```
>
> ```sortie
> true
> false
> ```
>
> `==` convierte el texto en número antes de comparar; `===` ve dos tipos
> distintos. Usa siempre `===`.

## En resumen

- `const` crea una variable que no se reasigna, `let` una variable que puede cambiar;
  evita `var`.
- Los tipos básicos son `number`, `string`, `boolean`, `undefined`, `null`, además de
  `bigint` y `symbol`; `typeof` da el tipo.
- Una plantilla `` `¡Hola, ${nombre}!` `` mete un valor dentro de un texto.
- `console.log(a, b)` muestra varios valores; `//` y `/* */` sirven para escribir
  comentarios.
- Los nombres se escriben en camelCase, y las mayúsculas cuentan.
- Compara siempre con `===` y `!==`.

Siguiente: [números y cálculos](nombres.md).
