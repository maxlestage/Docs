# Los arrays

Esta página explica cómo guardar varios valores en un array, añadirlos, quitarlos, buscarlos, transformarlos con `map`, `filter` y `reduce`, y ordenarlos sin caer en las trampas.

> **Objetivos:**
>
> - sabrás crear un array, leer un elemento y conocer su longitud;
> - sabrás añadir y quitar elementos al principio o al final;
> - sabrás buscar en un array con `includes`, `indexOf` y `find`;
> - sabrás transformar un array con `map`, `filter` y `reduce`;
> - sabrás ordenar números correctamente y copiar un array.

## Crear un array

Imagina una lista de la compra de veinte artículos. Crear veinte variables sería largo
y pesado. Un array guarda todos esos valores bajo un solo nombre.

Un array (*arreglo*) es una [lista](../debuter/glossaire.md#lista) de valores guardados
en orden, como los vagones de un tren. Se escribe entre corchetes `[ ]`, con comas entre
los elementos.

```javascript
const frutas = ["manzana", "pera", "kiwi"];
const notas = [12, 15, 9];
const vacio = [];
const mezcla = ["Alicia", 30, true];

console.log(frutas);
console.log(notas.length);
console.log(vacio.length);
console.log(mezcla);
```

```sortie
[ 'manzana', 'pera', 'kiwi' ]
3
0
[ 'Alicia', 30, true ]
```

Línea por línea:

- `frutas` contiene tres textos, `notas` tres números y `vacio` ningún elemento;
- `console.log(frutas)` muestra todo el array, entre corchetes;
- `notas.length` vale 3: hay tres notas. `vacio.length` vale 0;
- `mezcla` muestra que un mismo array puede contener un texto, un número y un booleano.

`length` da el número de elementos. Un array puede mezclar tipos, pero en la práctica se
guardan en él valores de la misma clase.

## Leer y cambiar un elemento

Una vez guardada la lista, necesitas sacar un elemento concreto, como leer la tercera
línea de tu lista de la compra, o corregir uno.

Cada elemento tiene un [índice](../debuter/glossaire.md#indice): su posición, **a partir
de 0**. El primer vagón es el vagón 0.

```javascript
const frutas = ["manzana", "pera", "kiwi"];

console.log(frutas[0]);
console.log(frutas[2]);
console.log(frutas.at(-1));
console.log(frutas[5]);

frutas[1] = "mango";
console.log(frutas);
```

```sortie
manzana
kiwi
kiwi
undefined
[ 'manzana', 'mango', 'kiwi' ]
```

Línea por línea:

- `frutas[0]` lee el primer elemento, `"manzana"`; `frutas[2]` lee el tercero, `"kiwi"`;
- `frutas.at(-1)` cuenta desde el final: da el último, otra vez `"kiwi"`;
- `frutas[5]` no existe: JavaScript responde `undefined`;
- `frutas[1] = "mango"` reemplaza `"pera"`, que estaba en el índice 1.

- `at(-1)` da el último elemento, igual que con [el texto](texte.md).
- Un índice que no existe da `undefined`, sin error.
- `frutas` es una `const`, y aun así puedes cambiar su contenido. `const` prohíbe
  reemplazar **el array entero** (`frutas = [...]`), no cambiar lo que contiene.

## Añadir y quitar

Una lista cambia sin parar. Piensa en una fila: la gente llega por el final y se va por
el principio. Estos métodos hacen exactamente eso.

Cuatro métodos añaden o quitan un elemento en un extremo del array. **Modifican** el
array.

```javascript
const fila = ["Alicia", "Bruno"];

fila.push("Clara");
console.log(fila);

const ultimo = fila.pop();
console.log(ultimo, fila);

fila.unshift("Zoe");
console.log(fila);

const primero = fila.shift();
console.log(primero, fila);
```

```sortie
[ 'Alicia', 'Bruno', 'Clara' ]
Clara [ 'Alicia', 'Bruno' ]
[ 'Zoe', 'Alicia', 'Bruno' ]
Zoe [ 'Alicia', 'Bruno' ]
```

Línea por línea:

- `push("Clara")` añade a Clara al final: la fila tiene tres nombres;
- `pop()` quita a Clara, la última, y la guarda en `ultimo`;
- `unshift("Zoe")` pone a Zoe al principio, delante de Alicia;
- `shift()` quita a Zoe del principio y la guarda en `primero`.

| Método | Extremo | Acción |
| --- | --- | --- |
| `push(x)` | final | añade `x` |
| `pop()` | final | quita y devuelve el último |
| `unshift(x)` | principio | añade `x` |
| `shift()` | principio | quita y devuelve el primero |

**Te toca:** escribe un programa que cree un array `compra` con `"pan"` y `"leche"`,
añada `"huevos"` al final y luego muestre el array y su longitud.

> **Solución**
>
> ```javascript
> const compra = ["pan", "leche"];
> compra.push("huevos");
> console.log(compra);
> console.log(compra.length);
> ```
>
> ```sortie
> [ 'pan', 'leche', 'huevos' ]
> 3
> ```

## slice y splice

A veces solo quieres una parte de la lista, por ejemplo los tres primeros de una
clasificación. O quieres quitar un elemento del medio, no de un extremo.

Estos dos nombres se parecen, pero hacen cosas muy distintas.

- `slice(inicio, fin)` **copia** un trozo, sin tocar el array (igual que con el texto).
- `splice(inicio, cuantos, ...nuevos)` **corta dentro** del array: quita elementos,
  puede insertar otros, y modifica el array.

```javascript
const a = [1, 2, 3, 4, 5];
console.log(a.slice(1, 3));
console.log(a);

const b = [1, 2, 3, 4, 5];
const quitados = b.splice(1, 2);
console.log(quitados);
console.log(b);

b.splice(1, 0, "x", "y");
console.log(b);
```

```sortie
[ 2, 3 ]
[ 1, 2, 3, 4, 5 ]
[ 2, 3 ]
[ 1, 4, 5 ]
[ 1, 'x', 'y', 4, 5 ]
```

Línea por línea:

- `a.slice(1, 3)` copia los elementos de índice 1 y 2 (el final, 3, queda fuera);
- `console.log(a)` lo demuestra: `a` no ha cambiado;
- `b.splice(1, 2)` quita el 2 y el 3 de `b`, y los devuelve en `quitados`;
- el último `splice` inserta dos elementos sin quitar nada.

`b.splice(1, 2)` empieza en el índice 1 y quita 2 elementos. `b.splice(1, 0, "x", "y")`
no quita ninguno e inserta `"x"` e `"y"` en el índice 1.

> **Consejo:** para recordarlo, `splice` tiene una **p** como «partir»: modifica el
> array. `slice` es una simple rebanada, copiada.

## Buscar en un array

Antes de añadir un artículo a tu lista de la compra, miras si ya está. Estos métodos
hacen esa pregunta por ti, incluso en una lista muy larga.

```javascript
const frutas = ["manzana", "pera", "kiwi"];

console.log(frutas.includes("kiwi"));
console.log(frutas.indexOf("pera"));
console.log(frutas.indexOf("plátano"));
```

```sortie
true
1
-1
```

`includes` responde sí o no; `indexOf` da la posición, o `-1` si el elemento no está.

### find y findIndex

Para buscar según una **regla**, `find` recibe una función que responde verdadero o
falso. Devuelve el primer elemento que cumple, o `undefined`.

```javascript
const notas = [8, 14, 11, 17];

console.log(notas.find((n) => n > 10));
console.log(notas.findIndex((n) => n > 10));
console.log(notas.find((n) => n > 20));
```

```sortie
14
1
undefined
```

La función que se da a `find` es un *callback* (mira [las funciones](fonctions.md#los-callbacks)).
`find` la llama sobre cada elemento, hasta que responde `true`.

**Te toca:** a partir de `const edades = [12, 17, 21, 15]`, muestra si 21 está en el
array y luego la primera edad mayor o igual que 18.

> **Solución**
>
> ```javascript
> const edades = [12, 17, 21, 15];
> console.log(edades.includes(21));
> console.log(edades.find((edad) => edad >= 18));
> ```
>
> ```sortie
> true
> 21
> ```

## Recorrer y transformar

Muy a menudo quieres hacer lo mismo con cada elemento: mostrar cada artículo, añadir
el IVA a cada precio, quedarte solo con las buenas notas. Estos métodos lo hacen por
ti, sin escribir el mismo código para cada elemento.

### forEach: hacer algo con cada elemento

```javascript
const frutas = ["manzana", "pera"];

frutas.forEach((fruta, i) => {
  console.log(`${i + 1}. ${fruta}`);
});
```

```sortie
1. manzana
2. pera
```

Línea por línea:

- `forEach` llama a la función una vez por elemento, así que dos veces aquí;
- en la primera vuelta, `fruta` vale `"manzana"` e `i` vale 0; en la segunda, `"pera"`
  y 1;
- `${i + 1}` suma 1 al índice, para numerar a partir de 1.

El callback recibe el elemento y luego su índice. Un bucle `for...of` hace el mismo
trabajo; elige el que te resulte más legible.

### map: transformar cada elemento

`map` crea un **nuevo array**, de la misma longitud, donde cada elemento se transforma.

```javascript
const precios = [10, 20, 30];
const preciosConIva = precios.map((p) => p * 1.2);
const nombres = ["alicia", "bruno"].map((nombre) => nombre.toUpperCase());

console.log(preciosConIva);
console.log(nombres);
console.log(precios);
```

```sortie
[ 12, 24, 36 ]
[ 'ALICIA', 'BRUNO' ]
[ 10, 20, 30 ]
```

El array original no ha cambiado.

### filter: quedarse con algunos elementos

`filter` crea un nuevo array solo con los elementos para los que el callback responde
`true`.

```javascript
const notas = [8, 14, 11, 17, 6];
const aprobadas = notas.filter((n) => n >= 10);

console.log(aprobadas);
console.log(notas.filter((n) => n > 20));
```

```sortie
[ 14, 11, 17 ]
[]
```

### reduce: resumirlo todo en un valor

`reduce` recorre el array guardando un **acumulador**: un valor que crece en cada vuelta.
Es como una hucha: con cada elemento, metes algo dentro.

```javascript
const notas = [12, 15, 9];

const total = notas.reduce((suma, n) => suma + n, 0);
console.log(total);
console.log(total / notas.length);
```

```sortie
36
12
```

- `0` es el valor inicial del acumulador `suma`.
- Vuelta 1: `0 + 12 = 12`; vuelta 2: `12 + 15 = 27`; vuelta 3: `27 + 9 = 36`.

### Encadenar métodos

`map` y `filter` devuelven arrays: puedes encadenarlos.

```javascript
const carrito = [4.5, 12, 2, 30];

const total = carrito
  .filter((precio) => precio > 3)
  .map((precio) => precio * 0.9)
  .reduce((suma, precio) => suma + precio, 0);

console.log(total.toFixed(2));
```

```sortie
41.85
```

Lee de arriba abajo: te quedas con los precios mayores de 3 €, aplicas un 10 % de
descuento y luego sumas.

**Te toca:** a partir de `const precios = [5, 12, 8]`, crea con `map` un array con los
precios duplicados y muéstralo. Luego muestra su suma, calculada con `reduce`.

> **Solución**
>
> ```javascript
> const precios = [5, 12, 8];
> const dobles = precios.map((p) => p * 2);
> console.log(dobles);
> console.log(dobles.reduce((suma, p) => suma + p, 0));
> ```
>
> ```sortie
> [ 10, 24, 16 ]
> 50
> ```

## Ordenar

Poner las cosas en orden sirve en todas partes: notas de la más baja a la más alta, una
clasificación, una lista de nombres por orden alfabético.

`sort` ordena el array **en el sitio** (lo modifica). Con textos simples, funciona como
se espera. Con números, hay una trampa.

```javascript
const numeros = [10, 1, 5, 100, 25];
numeros.sort();
console.log(numeros);
```

```sortie
[ 1, 10, 100, 25, 5 ]
```

Sin indicaciones, `sort` convierte cada elemento en **texto** y los ordena como en el
diccionario: `"10"` va antes que `"5"` porque `"1"` va antes que `"5"`, igual que «ab»
va antes que «b».

Dale una función de comparación `(a, b) => ...`, que devuelve un número: negativo si `a`
debe ir antes que `b`, positivo si va después.

```javascript
const numeros = [10, 1, 5, 100, 25];

numeros.sort((a, b) => a - b);
console.log(numeros);

numeros.sort((a, b) => b - a);
console.log(numeros);
```

```sortie
[ 1, 5, 10, 25, 100 ]
[ 100, 25, 10, 5, 1 ]
```

`a - b` ordena de menor a mayor, `b - a` de mayor a menor. Para palabras en español,
usa `(a, b) => a.localeCompare(b, "es")` (mira [el texto](texte.md#ordenar-palabras-en-espanol)).

> **Recuerda:** para ordenar números, escribe **siempre** `sort((a, b) => a - b)`.

**Te toca:** ordena el array `[42, 7, 19, 3]` de mayor a menor y luego muéstralo.

> **Solución**
>
> ```javascript
> const puntos = [42, 7, 19, 3];
> puntos.sort((a, b) => b - a);
> console.log(puntos);
> ```
>
> ```sortie
> [ 42, 19, 7, 3 ]
> ```

## El spread y la desestructuración

Estas dos escrituras cortas sirven para juntar varios arrays en uno solo, o para sacar
elementos de un array y guardarlos en variables con buenos nombres.

### El spread: desplegar un array

Tres puntos `...` delante de un array lo **despliegan**: sus elementos se colocan uno a
uno.

```javascript
const frutas = ["manzana", "pera"];
const verduras = ["zanahoria"];

const compra = [...frutas, ...verduras, "pan"];
console.log(compra);
console.log(Math.max(...[3, 8, 1]));
```

```sortie
[ 'manzana', 'pera', 'zanahoria', 'pan' ]
8
```

Línea por línea:

- `...frutas` coloca `"manzana"` y `"pera"`, y luego `...verduras` coloca `"zanahoria"`;
- después viene `"pan"`: `compra` es un nuevo array de cuatro elementos;
- `Math.max(...[3, 8, 1])` equivale a escribir `Math.max(3, 8, 1)`: el mayor es 8.

### La desestructuración

La **desestructuración** (*destructuring*) guarda los elementos de un array en varias
variables de una vez.

```javascript
const podio = ["Alicia", "Bruno", "Clara", "David"];

const [primero, segundo, ...otros] = podio;
console.log(primero);
console.log(segundo);
console.log(otros);

let a = 1;
let b = 2;
[a, b] = [b, a];
console.log(a, b);
```

```sortie
Alicia
Bruno
[ 'Clara', 'David' ]
2 1
```

La última línea intercambia dos variables sin variable intermedia.

## Fabricar arrays

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

A veces no quieres escribir cada elemento a mano, por ejemplo los cuadrados de los
números de 0 a 4. JavaScript puede fabricar el array por ti.

`Array.from` fabrica un array a partir de otra cosa: un texto, o una longitud y una
función. `flat` aplana un array de arrays.

```javascript
console.log(Array.from("abc"));
console.log(Array.from({ length: 5 }, (_, i) => i * i));

const cuadricula = [[1, 2], [3, 4], [5]];
console.log(cuadricula.flat());
console.log(Array.isArray(cuadricula));
```

```sortie
[ 'a', 'b', 'c' ]
[ 0, 1, 4, 9, 16 ]
[ 1, 2, 3, 4, 5 ]
true
```

En `(_, i) => i * i`, el `_` es un parámetro que no se usa: solo nos interesa el índice
`i`. `Array.isArray` comprueba que un valor es un array (`typeof` solo responde
`"object"`).

## ¿Copiar o compartir?

A menudo querrás conservar el original intacto y trabajar sobre una copia, como cuando
fotocopias un formulario antes de rellenarlo. Pero hace falta una copia de verdad.

Una variable no contiene el array en sí, sino una **referencia** a él: una dirección,
como una etiqueta pegada a una caja. Escribir `b = a` pega una segunda etiqueta en **la
misma caja**.

```javascript
const a = [1, 2, 3];
const b = a;
b.push(4);

console.log(a);
console.log(a === b);
```

```sortie
[ 1, 2, 3, 4 ]
true
```

Línea por línea:

- `const b = a` no copia nada: `b` apunta al mismo array que `a`;
- `b.push(4)` añade 4 a ese único array;
- por eso `a` también muestra el 4, y `a === b` responde `true`: es la misma caja.

Modificar `b` ha modificado `a`: es el mismo array. Para una copia de verdad, usa el
spread (o `slice()`).

```javascript
const a = [1, 2, 3];
const copia = [...a];
copia.push(4);

console.log(a);
console.log(copia);
console.log([1, 2] === [1, 2]);
```

```sortie
[ 1, 2, 3 ]
[ 1, 2, 3, 4 ]
false
```

`===` compara las referencias, no el contenido: dos arrays distintos nunca son iguales,
aunque tengan los mismos elementos.

## Errores frecuentes

### Ordenar números sin función de comparación

```javascript norun
const edades = [9, 25, 100, 3];
edades.sort();
console.log(edades);
```

```text
[ 100, 25, 3, 9 ]
```

No hay error, pero el orden es incorrecto: `sort` compara textos. Escribe
`edades.sort((a, b) => a - b)`.

### Leer después del final del array

```javascript norun
const frutas = ["manzana", "pera"];
console.log(frutas[2].toUpperCase());
```

```text
file:///home/alice/js/main.mjs:2
console.log(frutas[2].toUpperCase());
                      ^

TypeError: Cannot read properties of undefined (reading 'toUpperCase')
    at file:///home/alice/js/main.mjs:2:23

Node.js v22.22.0
```

El array tiene 2 elementos, con índices 0 y 1. `frutas[2]` vale `undefined`, que no
tiene método `toUpperCase`. El último elemento es `frutas[frutas.length - 1]` o
`frutas.at(-1)`.

### Olvidar el valor inicial de reduce

```javascript norun
const notas = [];
const total = notas.reduce((suma, n) => suma + n);
```

```text
file:///home/alice/js/main.mjs:2
const total = notas.reduce((suma, n) => suma + n);
                    ^

TypeError: Reduce of empty array with no initial value
    at Array.reduce (<anonymous>)
    at file:///home/alice/js/main.mjs:2:21

Node.js v22.22.0
```

Sin valor inicial, `reduce` toma el primer elemento… que no existe en un array vacío.
Da siempre el valor inicial: `reduce((suma, n) => suma + n, 0)`.

### Creer que has copiado un array

```javascript norun
const original = ["a", "b"];
const copia = original;
copia.pop();
console.log(original);
```

```text
[ 'a' ]
```

`copia` y `original` apuntan al mismo array. Copia con `[...original]`.

## Ponte a prueba

**Pregunta 1:** ¿qué contiene `t` después de este código?

```javascript norun
const t = [1, 2, 3];
t.push(4);
t.shift();
```

> **Respuesta**
>
> `[2, 3, 4]`. `push` añade 4 al final, `shift` quita el 1 del principio.

**Pregunta 2:** a partir de `[3, 8, 12, 5]`, ¿cómo obtienes el doble de los números
mayores que 4?

> **Respuesta**
>
> ```javascript
> const resultado = [3, 8, 12, 5].filter((n) => n > 4).map((n) => n * 2);
> console.log(resultado);
> ```
>
> ```sortie
> [ 16, 24, 10 ]
> ```

**Pregunta 3:** ¿cuál es la diferencia entre `slice` y `splice`?

> **Respuesta**
>
> `slice` copia un trozo sin modificar el array. `splice` quita (y puede insertar)
> elementos **dentro** del array, que modifica.

## En resumen

- Un array se escribe `[a, b, c]`; sus índices empiezan en 0; `length` da su tamaño.
- `push`/`pop` actúan al final, `unshift`/`shift` al principio; `splice` modifica,
  `slice` copia.
- `includes`, `indexOf`, `find` y `findIndex` buscan; `map`, `filter` y `reduce`
  transforman sin modificar el original.
- Ordena los números con `sort((a, b) => a - b)`, nunca con `sort()` solo.
- `...` despliega un array; `const [a, b] = array` lo desestructura.
- `b = a` comparte el mismo array; `[...a]` hace una copia.

Siguiente: [los objetos](objets.md).
