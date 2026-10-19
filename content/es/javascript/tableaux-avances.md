# Los arrays en profundidad

Esta página va más lejos con los arrays: aplanar, agrupar, ordenar sin modificar, buscar desde el final, copiar de verdad y entender `reduce` paso a paso.

> **Objetivos:**
>
> - sabrás aplanar un array con `flat` y `flatMap`;
> - sabrás agrupar elementos con `Object.groupBy`;
> - sabrás ordenar o modificar un array **sin tocar el original** con `toSorted`,
>   `toReversed` y `with`;
> - sabrás hacer una copia de verdad con `structuredClone`;
> - sabrás seguir un `reduce` paso a paso.

## Antes de empezar

Esta página supone que conoces [los arrays](collections.md): `push`, `map`, `filter`,
`find` y `sort`. Si no es así, lee primero esa página.

Una idea aparece por todas partes aquí. Un método de array hace una de estas dos cosas:

```text
 MODIFICA el array             DEVUELVE un array nuevo
 (escribes en el cuaderno)     (haces una fotocopia)
 push, pop, sort, reverse      map, filter, flat, toSorted, with
```

Saber en qué columna está un método evita muchas sorpresas.

## El último elemento con at

`at(-1)` da el último elemento. Los números negativos cuentan **desde el final**.

```javascript
const fila = ["Alice", "Bob", "Chloé"];

console.log(fila.at(0));
console.log(fila.at(-1));
console.log(fila.at(-2));
console.log(fila[fila.length - 1]);
```

```sortie
Alice
Chloé
Bob
Chloé
```

`fila.at(-1)` y `fila[fila.length - 1]` hacen lo mismo. La primera forma se lee mejor:
«el elemento 1 empezando por el final».

> **Atención:** `fila[-1]` no funciona: da `undefined`. Solo `at` entiende los números
> negativos.

## Buscar desde el final: findLast

`find` empieza por el principio. `findLast` empieza por **el final** y devuelve el
primer elemento que cumple la condición, yendo hacia atrás.

```javascript
const temperaturas = [12, 25, 18, 27, 15];

console.log(temperaturas.find((t) => t > 20));
console.log(temperaturas.findLast((t) => t > 20));
console.log(temperaturas.findLastIndex((t) => t > 20));
```

```sortie
25
27
3
```

Es práctico para «el **último** día en que hizo más de 20 grados».

Prueba: cambia `20` por `30`. Las dos primeras líneas muestran `undefined`, la última
muestra `-1`: nada cumple la condición.

## some y every: ¿sí o no?

Estos dos métodos responden con `true` o `false`.

- `some`: «¿**al menos un** elemento cumple?»
- `every`: «¿cumplen **todos** los elementos?»

```javascript
const notas = [12, 8, 15];

console.log(notas.some((n) => n < 10));
console.log(notas.every((n) => n >= 10));
console.log(notas.every((n) => n > 0));
```

```sortie
true
false
true
```

Piensa en el control de bolsos a la entrada de un concierto. `some` busca si hay **un**
objeto prohibido. `every` comprueba que **todos** los objetos están permitidos. En
cuanto se conoce la respuesta, la búsqueda se detiene.

> **Recuerda:** `some` = «al menos uno»; `every` = «todos».

## Aplanar con flat y flatMap

### flat: desplegar las cajas dentro de las cajas

Un array puede contener otros arrays. `flat` los **despliega** un nivel.

```javascript
const cajones = [["calcetines", "jerséis"], ["libros"], ["bolígrafos", "gomas"]];

console.log(cajones.flat());
console.log([1, [2, [3, [4]]]].flat());
console.log([1, [2, [3, [4]]]].flat(Infinity));
```

```sortie
[ 'calcetines', 'jerséis', 'libros', 'bolígrafos', 'gomas' ]
[ 1, 2, [ 3, [ 4 ] ] ]
[ 1, 2, 3, 4 ]
```

```text
 [ [a, b], [c] ]   ──flat()──▶   [ a, b, c ]
```

Por defecto, `flat` despliega **un** nivel. `flat(Infinity)` lo despliega todo, sea cual
sea la profundidad.

### flatMap: transformar y luego aplanar

`flatMap(f)` hace un `map(f)` y luego un `flat()`. Es útil cuando cada elemento da
**varios** resultados.

```javascript
const frases = ["hace buen tiempo", "salgamos fuera"];

console.log(frases.map((f) => f.split(" ")));
console.log(frases.flatMap((f) => f.split(" ")));
```

```sortie
[ [ 'hace', 'buen', 'tiempo' ], [ 'salgamos', 'fuera' ] ]
[ 'hace', 'buen', 'tiempo', 'salgamos', 'fuera' ]
```

Con `map`, obtienes un array de arrays. Con `flatMap`, una sola lista de palabras.

Prueba: `[1, 2, 3].flatMap((n) => [n, n])`. Obtienes `[1, 1, 2, 2, 3, 3]`: cada número
aparece dos veces.

## Agrupar con Object.groupBy

Imagina que ordenas la ropa por colores: un montón de ropa blanca, un montón de ropa de
color. `Object.groupBy` hace esos montones por ti.

Le das el array y una función que dice **a qué montón** va cada elemento.

```javascript
const frutas = [
  { nombre: "manzana", color: "rojo" },
  { nombre: "kiwi", color: "verde" },
  { nombre: "fresa", color: "rojo" },
];

const montones = Object.groupBy(frutas, (f) => f.color);

console.log(montones.rojo.map((f) => f.nombre));
console.log(montones.verde.map((f) => f.nombre));
console.log(Object.keys(montones));
```

```sortie
[ 'manzana', 'fresa' ]
[ 'kiwi' ]
[ 'rojo', 'verde' ]
```

El resultado es un objeto: cada clave es el nombre de un montón, cada valor un array.

La función puede inventar sus propios nombres de montones:

```javascript
const notas = [12, 7, 15, 9, 18];

const grupos = Object.groupBy(notas, (n) => (n >= 10 ? "aprobado" : "a repasar"));

console.log(grupos["aprobado"]);
console.log(grupos["a repasar"]);
```

```sortie
[ 12, 15, 18 ]
[ 7, 9 ]
```

> **Nota:** si muestras directamente el resultado con `console.log(montones)`, Node
> escribe `[Object: null prototype]` delante. Es un objeto «desnudo», sin los métodos
> habituales. Puedes usarlo con normalidad.

## Ordenar sin modificar: toSorted, toReversed, with

`sort` y `reverse` **modifican** el array. A veces quieres mantener el original intacto:
por ejemplo, una lista en orden de llegada **y** la misma lista ordenada.

Tres métodos devuelven una **copia** modificada:

| Modifica el original | Devuelve una copia |
| --- | --- |
| `sort(f)` | `toSorted(f)` |
| `reverse()` | `toReversed()` |
| `t[i] = x` | `t.with(i, x)` |

```javascript
const llegadas = [42, 7, 19];

const ordenadas = llegadas.toSorted((a, b) => a - b);
const invertidas = llegadas.toReversed();
const corregidas = llegadas.with(1, 8);

console.log(ordenadas);
console.log(invertidas);
console.log(corregidas);
console.log(llegadas);
```

```sortie
[ 7, 19, 42 ]
[ 19, 7, 42 ]
[ 42, 8, 19 ]
[ 42, 7, 19 ]
```

La última línea lo demuestra: `llegadas` no ha cambiado.

Compara con `sort`, que escribe en el array:

```javascript
const llegadas = [42, 7, 19];
const ordenadas = llegadas.sort((a, b) => a - b);

console.log(ordenadas);
console.log(llegadas);
console.log(ordenadas === llegadas);
```

```sortie
[ 7, 19, 42 ]
[ 7, 19, 42 ]
true
```

`sort` ha ordenado el propio `llegadas`, y `ordenadas` es **el mismo** array.

> **Recuerda:** los métodos que empiezan por `to…` y `with` nunca tocan el original.

## Fabricar un array con Array.from

`Array.from({ length: n }, funcion)` fabrica un array de `n` casillas. La función recibe
el índice de cada casilla y devuelve lo que se pone en ella.

```javascript
const indices = Array.from({ length: 5 }, (_, i) => i);
const pares = Array.from({ length: 5 }, (_, i) => i * 2);
const estrellas = Array.from({ length: 3 }, (_, i) => "*".repeat(i + 1));

console.log(indices);
console.log(pares);
console.log(estrellas);
```

```sortie
[ 0, 1, 2, 3, 4 ]
[ 0, 2, 4, 6, 8 ]
[ '*', '**', '***' ]
```

El `_` es el primer parámetro, que no usamos (la casilla vacía). Solo nos interesa el
índice `i`.

| Casilla | i | `i * 2` |
| --- | --- | --- |
| 1.ª | 0 | 0 |
| 2.ª | 1 | 2 |
| 3.ª | 2 | 4 |
| 4.ª | 3 | 6 |
| 5.ª | 4 | 8 |

Prueba: escribe `(_, i) => i + 1`. Obtienes los números del 1 al 5.

## Copiar de verdad: spread o structuredClone

### El spread copia un solo nivel

`[...t]` copia el array. Pero si el array contiene **objetos**, esos objetos no se
copian: la copia contiene los **mismos** objetos.

```javascript
const equipo = [{ nombre: "Alice", puntos: 10 }];
const copia = [...equipo];

copia[0].puntos = 99;
console.log(equipo[0].puntos);

copia.push({ nombre: "Bob", puntos: 5 });
console.log(equipo.length);
```

```sortie
99
1
```

Añadir un elemento a `copia` no cambia `equipo`: los dos arrays son distintos. Pero
modificar el objeto de Alice lo cambia **en los dos**, porque es el mismo objeto.

```text
 equipo ──▶ [ • ]─────┐
                      ├──▶ { nombre: 'Alice', puntos: 99 }
 copia  ──▶ [ • , • ]─┘          (¡un solo objeto!)
                  └──▶ { nombre: 'Bob', puntos: 5 }
```

Es lo que se llama una copia **superficial**: fotocopias la lista, no las fichas que
hay guardadas dentro.

### structuredClone copia todos los niveles

`structuredClone` hace una copia **profunda**: el array y todo lo que contiene.

```javascript
const equipo = [{ nombre: "Alice", puntos: 10 }];
const copia = structuredClone(equipo);

copia[0].puntos = 99;
console.log(equipo[0].puntos);
console.log(copia[0].puntos);
```

```sortie
10
99
```

| Copia | El array | Los objetos de dentro |
| --- | --- | --- |
| `b = a` | compartido | compartidos |
| `[...a]` | copiado | compartidos |
| `structuredClone(a)` | copiado | copiados |

> **Recuerda:** un array de números o de textos se copia con `[...a]`. Un array de
> objetos que vas a modificar se copia con `structuredClone(a)`.

## reduce paso a paso

`reduce` es el método que más miedo da. Sin embargo, es un simple bucle con una caja que
se va llenando.

```text
 array.reduce((caja, elemento) => nuevo valor de la caja, valor inicial)
```

En cada vuelta, la función recibe **la caja** (el acumulador) y **el elemento**. Lo que
devuelve se convierte en la nueva caja para la vuelta siguiente.

### Una suma

```javascript
const cesta = [4, 10, 6];
const total = cesta.reduce((caja, precio) => caja + precio, 0);
console.log(total);
```

```sortie
20
```

| Vuelta | caja (antes) | precio | caja + precio (después) |
| --- | --- | --- | --- |
| inicio | – | – | 0 |
| 1 | 0 | 4 | 4 |
| 2 | 4 | 10 | 14 |
| 3 | 14 | 6 | 20 |

Después de la última vuelta, `reduce` devuelve la caja: `20`.

### El mismo cálculo con un bucle

Esto es exactamente lo que hace `reduce`, escrito con un bucle:

```javascript
const cesta = [4, 10, 6];

let caja = 0;
for (const precio of cesta) {
  caja = caja + precio;
}
console.log(caja);
```

```sortie
20
```

Si `reduce` te parece oscuro, escribe primero el bucle. Después, transfórmalo.

### El elemento más grande

La caja puede contener otra cosa que un total: aquí, el número más grande visto hasta
el momento.

```javascript
const puntuaciones = [7, 21, 13];
const mejor = puntuaciones.reduce((caja, p) => (p > caja ? p : caja), 0);
console.log(mejor);
```

```sortie
21
```

| Vuelta | caja (antes) | p | ¿p > caja? | caja (después) |
| --- | --- | --- | --- | --- |
| inicio | – | – | – | 0 |
| 1 | 0 | 7 | sí | 7 |
| 2 | 7 | 21 | sí | 21 |
| 3 | 21 | 13 | no | 21 |

### Contar con un objeto

La caja también puede ser un objeto. Contemos los votos:

```javascript
const votos = ["sí", "no", "sí", "sí"];

const recuento = votos.reduce((caja, v) => {
  caja[v] = (caja[v] ?? 0) + 1;
  return caja;
}, {});

console.log(recuento);
```

```sortie
{ 'sí': 3, no: 1 }
```

| Vuelta | v | caja (después) |
| --- | --- | --- |
| inicio | – | `{}` |
| 1 | `"sí"` | `{ 'sí': 1 }` |
| 2 | `"no"` | `{ 'sí': 1, no: 1 }` |
| 3 | `"sí"` | `{ 'sí': 2, no: 1 }` |
| 4 | `"sí"` | `{ 'sí': 3, no: 1 }` |

Node escribe `'sí'` entre comillas por la tilde, pero es una clave normal.

Con llaves, la flecha debe terminar con `return caja;`. Sin él, la caja de la vuelta
siguiente sería `undefined`.

> **Recuerda:** en `reduce`, la función siempre devuelve **la nueva caja**. Da siempre
> un valor inicial.

## Errores frecuentes

### Creer que sort no toca el original

```javascript norun
const llegadas = [42, 7, 19];
const ordenadas = llegadas.sort((a, b) => a - b);
console.log("Primero en llegar:", llegadas[0]);
```

```text
Primero en llegar: 7
```

No hay error, pero el resultado es falso: el primero en llegar era `42`. `sort` ha
modificado `llegadas`. Usa `llegadas.toSorted((a, b) => a - b)`.

### Llamar a groupBy sobre el array

```javascript norun
const palabras = ["uno", "dos", "tres"];
console.log(palabras.groupBy((p) => p.length));
```

```text
file:///home/alice/js/main.mjs:2
console.log(palabras.groupBy((p) => p.length));
                     ^

TypeError: palabras.groupBy is not a function
    at file:///home/alice/js/main.mjs:2:22

Node.js v22.22.0
```

`groupBy` no es un método de los arrays: es una función de `Object`. Escribe
`Object.groupBy(palabras, (p) => p.length)`.

### Usar with con un índice que no existe

```javascript norun
const notas = [12, 15];
const nuevas = notas.with(5, 20);
```

```text
file:///home/alice/js/main.mjs:2
const nuevas = notas.with(5, 20);
                         ^

RangeError: Invalid index : 5
    at Array.with (<anonymous>)
    at file:///home/alice/js/main.mjs:2:26

Node.js v22.22.0
```

`with` reemplaza una casilla que **ya existe**. Aquí solo están los índices 0 y 1. Para
añadir al final sin modificar el original, escribe `[...notas, 20]`.

### Copiar con structuredClone un objeto que contiene una función

```javascript norun
const boton = { texto: "OK", clic() {} };
const copia = structuredClone(boton);
```

```text
node:internal/per_context/domexception:76
    const self = new Error();
                 ^

DOMException [DataCloneError]: clic() {} could not be cloned.
    at new DOMException (node:internal/per_context/domexception:76:18)
    at structuredClone (node:internal/worker/js_transferable:126:26)
    at file:///home/alice/js/main.mjs:2:15

Node.js v22.22.0
```

`structuredClone` copia **datos** (números, textos, arrays, objetos, fechas), no
funciones. Guarda tus datos en objetos simples, sin métodos.

## Ponte a prueba

**Pregunta 1:** ¿cuánto vale `[[1, 2], [3]].flat().at(-1)`?

> **Respuesta**
>
> ```javascript
> console.log([[1, 2], [3]].flat().at(-1));
> ```
>
> ```sortie
> 3
> ```
>
> `flat()` da `[1, 2, 3]`, y `at(-1)` toma el último elemento.

**Pregunta 2:** quieres la lista ordenada **y** conservar la lista original. ¿Qué método
eliges?

> **Respuesta**
>
> `toSorted`, que devuelve una copia ordenada. `sort` modificaría la lista original.

**Pregunta 3:** rellena la tabla de seguimiento de
`[2, 3, 4].reduce((caja, n) => caja * n, 1)`.

> **Respuesta**
>
> | Vuelta | caja (antes) | n | caja (después) |
> | --- | --- | --- | --- |
> | inicio | – | – | 1 |
> | 1 | 1 | 2 | 2 |
> | 2 | 2 | 3 | 6 |
> | 3 | 6 | 4 | 24 |
>
> ```javascript
> console.log([2, 3, 4].reduce((caja, n) => caja * n, 1));
> ```
>
> ```sortie
> 24
> ```

## En resumen

- `at(-1)` lee el último elemento; `findLast` busca desde el final; `some` y `every`
  responden «al menos uno» y «todos».
- `flat` despliega los arrays anidados; `flatMap` transforma y luego despliega.
- `Object.groupBy(array, f)` reparte los elementos en montones con el nombre que da `f`.
- `toSorted`, `toReversed` y `with` devuelven una copia: el original no cambia.
- `[...a]` copia un nivel; `structuredClone(a)` lo copia todo.
- `reduce` pasa una caja de una vuelta a otra: síguela en una tabla.

Siguiente: [gestionar los errores](erreurs.md).
