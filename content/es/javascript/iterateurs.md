# Iteradores y generadores

Esta página explica qué pasa cuando escribes `for...of`, cómo hacer que tus propios objetos se puedan recorrer, y cómo producir valores uno a uno con los generadores `function*`.

> **Objetivos:**
>
> - entenderás el protocolo de iteración que hay detrás de `for...of`;
> - sabrás hacer que un objeto se pueda recorrer con `Symbol.iterator`;
> - sabrás escribir un generador con `function*` y `yield`;
> - sabrás manejar una secuencia infinita sin bloquear el programa;
> - sabrás usar `...`, `entries()`, `keys()` y `values()`.

## Lo que recorre for...of

`for...of` recorre muchas cosas: un array, una cadena, un `Map`, un `Set`…
Decimos que estos valores son **iterables**: se pueden recorrer elemento a elemento.

```javascript
for (const letra of "gato") {
  console.log(letra);
}

for (const [clave, valor] of new Map([["a", 1], ["b", 2]])) {
  console.log(clave, valor);
}
```

```sortie
g
a
t
o
a 1
b 2
```

Pero un objeto normal `{ }` **no** es iterable. Para recorrer sus propiedades, hay que
pasar por `Object.entries` (mira [los objetos](objets.md)).

## El protocolo de iteración

¿Cómo lo hace `for...of`? Le pide al valor un
[iterador](../debuter/glossaire.md#iterador): un objeto que da los elementos **uno a
uno**, cuando se le piden. Piensa en un dispensador de turnos: cada vez que pulsas el
botón, sale el siguiente número; cuando el rollo se acaba, te lo dice.

El botón se llama `next()`. Devuelve un pequeño objeto:

- `value`: el siguiente elemento;
- `done`: `true` cuando ya no queda nada.

El iterador se obtiene con un método de nombre especial, `[Symbol.iterator]()`.

```javascript
const frutas = ["manzana", "kiwi"];
const iterador = frutas[Symbol.iterator]();

console.log(iterador.next());
console.log(iterador.next());
console.log(iterador.next());
```

```sortie
{ value: 'manzana', done: false }
{ value: 'kiwi', done: false }
{ value: undefined, done: true }
```

Es exactamente lo que `for...of` hace por ti: llama a `next()` en bucle, y se detiene
en cuanto `done` vale `true`.

> **Nota:** `Symbol.iterator` es un **símbolo**: un nombre de propiedad especial, que no
> puede chocar con tus propios nombres. Se escribe entre corchetes.

## Hacer que tu objeto sea iterable

Para que `for...of` acepte tu objeto, dale un método `[Symbol.iterator]()` que devuelva
un iterador, es decir, un objeto con un método `next()`.

Aquí tienes una cuenta atrás.

```javascript
class CuentaAtras {
  constructor(inicio) {
    this.inicio = inicio;
  }

  [Symbol.iterator]() {
    let actual = this.inicio;
    return {
      next() {
        if (actual < 0) {
          return { value: undefined, done: true };
        }
        const valor = actual;
        actual -= 1;
        return { value: valor, done: false };
      },
    };
  }
}

for (const n of new CuentaAtras(3)) {
  console.log(n);
}
console.log("¡Despegue!");
```

```sortie
3
2
1
0
¡Despegue!
```

Cada llamada a `[Symbol.iterator]()` crea una nueva variable `actual` (una
[closure](closures.md)): se puede recorrer la misma cuenta atrás varias veces.

Funciona, pero es largo. Los generadores hacen lo mismo de forma mucho más corta.

## Los generadores: function* y yield

Un **generador** es una función especial, escrita `function*` (con un asterisco). En
lugar de calcularlo todo de golpe, produce sus valores uno a uno con `yield` («ceder»,
«entregar»).

En cada `yield`, la función se **pausa** y entrega un valor. En el siguiente `next()`,
continúa exactamente donde se había quedado. Es como un libro con un marcapáginas.

```javascript
function* tresColores() {
  console.log("(inicio)");
  yield "rojo";
  console.log("(continúa)");
  yield "verde";
  yield "azul";
}

const gen = tresColores();
console.log(gen.next().value);
console.log(gen.next().value);

for (const color of tresColores()) {
  console.log(color);
}
```

```sortie
(inicio)
rojo
(continúa)
verde
(inicio)
rojo
(continúa)
verde
azul
```

Llamar a `tresColores()` no ejecuta nada: devuelve un **objeto generador**, que es un
iterador. El código solo se ejecuta cuando se le pide, en cada `next()`.

### Reescribir la cuenta atrás

Con un generador, la cuenta atrás cabe en unas pocas líneas. Se escribe
`*[Symbol.iterator]()`: un método generador.

```javascript
class CuentaAtras {
  constructor(inicio) {
    this.inicio = inicio;
  }

  *[Symbol.iterator]() {
    for (let n = this.inicio; n >= 0; n--) {
      yield n;
    }
  }
}

console.log([...new CuentaAtras(5)]);
```

```sortie
[ 5, 4, 3, 2, 1, 0 ]
```

Ya no hay que escribir `next()`, `value` ni `done`: el generador se encarga.

### Un generador con parámetros

Un generador recibe parámetros como una función normal.

```javascript
function* intervalo(inicio, fin, paso = 1) {
  for (let n = inicio; n < fin; n += paso) {
    yield n;
  }
}

for (const n of intervalo(0, 10, 3)) {
  console.log(n);
}
```

```sortie
0
3
6
9
```

## Un generador infinito

Como un generador solo calcula lo que se le pide, puede ser **infinito**: el bucle
`while (true)` no bloquea el programa, porque la función se pausa en cada `yield`.

Para tomar solo una parte, escribamos una pequeña herramienta `tomar(n, iterable)`: es a
su vez un generador, que se detiene después de `n` elementos.

```javascript
function* enteros() {
  let n = 1;
  while (true) {
    yield n;
    n += 1;
  }
}

function* tomar(n, iterable) {
  if (n <= 0) return;
  let cuenta = 0;
  for (const elemento of iterable) {
    yield elemento;
    cuenta += 1;
    if (cuenta === n) return;
  }
}

function* cuadrados(iterable) {
  for (const x of iterable) {
    yield x * x;
  }
}

console.log([...tomar(5, enteros())]);
console.log([...tomar(4, cuadrados(enteros()))]);
```

```sortie
[ 1, 2, 3, 4, 5 ]
[ 1, 4, 9, 16 ]
```

`return` en un generador lo termina: `done` pasa a `true`. Los generadores se encajan
como tuberías: `enteros` → `cuadrados` → `tomar`. Cada valor atraviesa las tuberías uno
a uno, y solo se calculan cuatro valores en total.

### La sucesión de Fibonacci

Cada número es la suma de los dos anteriores: 0, 1, 1, 2, 3, 5, 8…

```javascript
function* fibonacci() {
  let [a, b] = [0, 1];
  while (true) {
    yield a;
    [a, b] = [b, a + b];
  }
}

const primeros = [];
for (const n of fibonacci()) {
  if (n > 50) break;
  primeros.push(n);
}
console.log(primeros.join(", "));
```

```sortie
0, 1, 1, 2, 3, 5, 8, 13, 21, 34
```

`break` detiene el bucle `for...of`; el generador se abandona entonces limpiamente.

## Desplegar un iterable con ...

El operador `...` (**spread**, «esparcir») despliega cualquier iterable: array, cadena,
`Set`, `Map`, generador… Se usa para construir un array o para pasar argumentos.

```javascript
const letras = [..."hola"];
console.log(letras);

const sinDuplicados = [...new Set([3, 1, 3, 2, 1])];
console.log(sinDuplicados);

function* notas() {
  yield 12;
  yield 17;
  yield 9;
}
console.log(Math.max(...notas()));

const [primera, ...resto] = notas();
console.log(primera, resto);
```

```sortie
[ 'h', 'o', 'l', 'a' ]
[ 3, 1, 2 ]
17
12 [ 17, 9 ]
```

`Array.from(iterable)` hace lo mismo que `[...iterable]`.

## entries(), keys() y values()

Los arrays, los `Map` y los `Set` tienen tres métodos que devuelven iteradores:

| Método | Da |
|--------|----|
| `keys()` | las claves (los índices en un array) |
| `values()` | los valores |
| `entries()` | pares `[clave, valor]` |

```javascript
const dias = ["lun", "mar", "mié"];

for (const [indice, dia] of dias.entries()) {
  console.log(`${indice + 1}. ${dia}`);
}

const stock = new Map([
  ["manzanas", 4],
  ["peras", 0],
]);
console.log([...stock.keys()]);
console.log([...stock.values()]);
```

```sortie
1. lun
2. mar
3. mié
[ 'manzanas', 'peras' ]
[ 4, 0 ]
```

Para un objeto normal, usa las funciones `Object.keys`, `Object.values` y
`Object.entries`: devuelven arrays directamente.

```javascript
const puntos = { alice: 12, bob: 8 };
for (const [nombre, punto] of Object.entries(puntos)) {
  console.log(`${nombre}: ${punto}`);
}
```

```sortie
alice: 12
bob: 8
```

## Errores frecuentes

### Recorrer un objeto normal con for...of

```javascript norun
const persona = { nombre: "Ada", edad: 36 };
for (const x of persona) console.log(x);
```

```text
file:///home/vous/projet/main.mjs:2
for (const x of persona) console.log(x);
                ^

TypeError: persona is not iterable
    at file:///home/vous/projet/main.mjs:2:17
```

«`persona` no es iterable»: un objeto `{ }` no tiene `Symbol.iterator`. Escribe
`for (const [clave, valor] of Object.entries(persona))`.

### Olvidar llamar al generador

```javascript norun
function* f() { yield 1; }
const it = f;
for (const x of it) console.log(x);
```

```text
file:///home/vous/projet/main.mjs:3
for (const x of it) console.log(x);
                ^

TypeError: it is not iterable
    at file:///home/vous/projet/main.mjs:3:17
```

`f` es la función; es `f()`, la llamada, lo que devuelve el objeto generador. Añade los
paréntesis: `const it = f();`.

### Desplegar un generador infinito

```javascript norun
function* numeros() { let n = 1; while (true) yield n++; }
console.log([...numeros()].length);
```

```text
<--- JS stacktrace --->
FATAL ERROR: invalid array length Allocation failed - JavaScript heap out of memory
```

Después de largos segundos, Node se rinde: la memoria está llena. `...` quiere
desplegarlo todo, y la secuencia no termina nunca. Limítala primero:
`[...tomar(10, numeros())]`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```javascript norun
function* dos() {
  yield "a";
  yield "b";
}
const g = dos();
console.log([...g]);
console.log([...g]);
```

> **Respuesta**
>
> `[ 'a', 'b' ]` y después `[]`. Un objeto generador solo se recorre **una vez**: se
> agota tras el primer `...`. Para volver a empezar, llama otra vez a `dos()`.

**Pregunta 2:** ¿qué devuelve `next()` cuando a un iterador ya no le quedan elementos?

> **Respuesta**
>
> Un objeto cuyo `done` vale `true` (y cuyo `value` suele valer `undefined`).

**Pregunta 3:** escribe un generador `pares()` que produzca 0, 2, 4, 6… sin fin, y
muestra sus 4 primeros valores.

> **Respuesta**
>
> ```javascript
> function* pares() {
>   for (let n = 0; ; n += 2) {
>     yield n;
>   }
> }
>
> const resultado = [];
> for (const n of pares()) {
>   if (resultado.length === 4) break;
>   resultado.push(n);
> }
> console.log(resultado);
> ```
>
> ```sortie
> [ 0, 2, 4, 6 ]
> ```

## En resumen

- Un iterable tiene un método `[Symbol.iterator]()` que devuelve un iterador.
- Un iterador tiene un método `next()` que devuelve `{ value, done }`; `for...of` lo
  llama por ti.
- `function*` y `yield` escriben un iterador en unas pocas líneas: la función se pausa
  en cada `yield`.
- Un generador puede ser infinito: toma una parte con `break` o con una herramienta como
  `tomar`.
- `...`, `entries()`, `keys()` y `values()` funcionan con todos los iterables.

Siguiente: [los arrays en profundidad](tableaux-avances.md)
