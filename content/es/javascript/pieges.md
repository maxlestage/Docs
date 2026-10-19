# Las trampas del principiante en JavaScript

Esta página reúne las 11 trampas que más sorprenden a quienes empiezan con JavaScript, con, para cada una, lo que esperas, lo que pasa de verdad, por qué, y cómo evitarla.

> **Objetivos:**
>
> - reconocerás las trampas clásicas de JavaScript cuando te las encuentres;
> - entenderás por qué el resultado es sorprendente;
> - sabrás escribir la versión correcta cada vez.

## Cómo leer esta página

JavaScript se creó muy rápido, en 1995. Intenta ser «amable»: en lugar de detenerse
ante un error, a menudo adivina lo que querías decir. A veces adivina mal.

Cada trampa sigue el mismo camino:

```text
 Lo que esperas  →  Lo que pasa  →  Por qué  →  La corrección
```

Lee una trampa cada vez. Ejecuta tú mismo cada ejemplo: una trampa que has visto con
tus propios ojos ya no se olvida.

## Las trampas de los valores

### Comparar con == en lugar de ===

**Esperas:** que `0 == ""` responda `false`. Un número no es un texto.

```javascript
console.log(0 == "");
console.log(1 == "1");
console.log(null == undefined);
console.log(0 === "");
console.log(1 === "1");
```

```sortie
true
true
true
false
false
```

**Por qué:** `==` (igualdad «flexible») **convierte** los dos valores antes de
compararlos. El texto vacío `""` se convierte en el número `0`, así que `0 == ""` es
verdadero. `===` (igualdad «estricta») no convierte nada: dos
[tipos](../debuter/glossaire.md#tipo) distintos siempre son distintos.

**La corrección:** usa **siempre** `===` y `!==`. Mira también
[los primeros pasos](premiers-pas.md#comparar-o).

> **Recuerda:** tres signos `===` para comparar. Dos signos `==`, nunca.

### "1" + 1 no da 2

**Esperas:** que `"1" + 1` dé `2`.

```javascript
console.log("1" + 1);
console.log("5" - 2);
console.log("5" * "2");
console.log("10" + 5 - 3);
```

```sortie
11
3
10
102
```

**Por qué:** el `+` tiene dos oficios. Con dos números, suma. En cuanto hay **un
texto**, pega los dos trozos, como dos pedazos de cinta. Los demás operadores (`-`,
`*`, `/`) solo saben calcular: convierten el texto en número.

Sigamos `"10" + 5 - 3` paso a paso:

| Paso | Cálculo | Resultado |
| --- | --- | --- |
| 1 | `"10" + 5` | `"105"` (un texto, pegado) |
| 2 | `"105" - 3` | `102` (un número, calculado) |

Es muy habitual con un valor que escribe el usuario: llega **siempre** en forma de
texto.

**La corrección:** convierte el texto en número con `Number(...)` antes de calcular.

```javascript
const entrada = "1";
console.log(Number(entrada) + 1);
```

```sortie
2
```

Prueba: cambia `"1"` por `"abc"`. Obtienes `NaN`, la trampa siguiente.

### NaN no es igual a nada, ni siquiera a sí mismo

`NaN` quiere decir *Not a Number*, «no es un número». Es el resultado de un cálculo
imposible.

**Esperas:** que `resultado === NaN` te diga si el cálculo ha fallado.

```javascript
const resultado = Number("abc");

console.log(resultado);
console.log(resultado === NaN);
console.log(NaN === NaN);
console.log(Number.isNaN(resultado));
```

```sortie
NaN
false
false
true
```

**Por qué:** por convención, `NaN` no es igual a **ningún** valor, ni a sí mismo. Por
eso la comparación con `===` siempre responde `false`.

**La corrección:** comprueba con `Number.isNaN(valor)`. Mira
[los números](nombres.md#nan-no-es-un-numero).

### 0.1 + 0.2 no da 0.3

**Esperas:** `0.3`.

```javascript
console.log(0.1 + 0.2);
console.log(0.1 + 0.2 === 0.3);
console.log((0.1 + 0.2).toFixed(2));
```

```sortie
0.30000000000000004
false
0.30
```

**Por qué:** el ordenador guarda los números decimales en
[binario](../debuter/glossaire.md#binario) (con 0 y 1). Algunos números, como `0.1`,
no salen exactos en binario, igual que `1/3` no sale exacto en decimal (`0,3333…`).
Queda un error muy pequeño.

**La corrección:**

- para **mostrar**, redondea con `toFixed(2)`;
- para **comparar**, comprueba que la diferencia es minúscula;
- para el **dinero**, cuenta en céntimos (números enteros).

```javascript
const diferencia = Math.abs(0.1 + 0.2 - 0.3);
console.log(diferencia < 0.000001);

const totalCentimos = 10 + 20;
console.log(totalCentimos / 100);
```

```sortie
true
0.3
```

Mira [los números](nombres.md#los-numeros-decimales-no-son-exactos).

### undefined y null: dos clases de «vacío»

**Esperas:** que exista un solo valor para decir «nada».

```javascript
let nombre;
const gato = { nombre: "Félix" };
const datos = null;

console.log(nombre);
console.log(gato.edad);
console.log(datos);
console.log(typeof undefined);
console.log(typeof null);
```

```sortie
undefined
undefined
null
undefined
object
```

**Por qué:** JavaScript tiene **dos** valores vacíos.

| Valor | ¿Quién lo pone? | Sentido |
| --- | --- | --- |
| `undefined` | JavaScript, solo | «aquí nunca se ha puesto nada» |
| `null` | tú, a propósito | «aquí no hay nada, lo sé» |

Y `typeof null` responde `"object"`: es un viejo error del lenguaje, que se mantiene
para no romper los programas antiguos.

**La corrección:** para comprobar «vacío o no», usa `??` para un valor de reserva y
`?.` para leer sin que el programa falle (mira
[los valores ausentes](conditions-boucles.md#los-valores-ausentes-y)).

```javascript
const datos = null;
console.log(datos ?? "sin datos");
console.log(datos?.length);
```

```sortie
sin datos
undefined
```

> **Recuerda:** `undefined` = nunca rellenado; `null` = vaciado a propósito. `??` y
> `?.` tratan los dos de la misma forma.

## Las trampas de los arrays y los objetos

### sort() ordena los números como palabras

**Esperas:** `[1, 5, 10, 25, 100]`.

```javascript
const numeros = [10, 1, 5, 100, 25];
numeros.sort();
console.log(numeros);
```

```sortie
[ 1, 10, 100, 25, 5 ]
```

**Por qué:** si no le dices nada, `sort` convierte cada número en texto y ordena como
en el diccionario. En un diccionario, «10» va antes que «5», porque primero se compara
el primer carácter: `1` antes que `5`.

**La corrección:** dale una función de comparación.

```javascript
const numeros = [10, 1, 5, 100, 25];
numeros.sort((a, b) => a - b);
console.log(numeros);
```

```sortie
[ 1, 5, 10, 25, 100 ]
```

Prueba: escribe `b - a` en lugar de `a - b`. El orden se invierte. Mira
[los arrays](collections.md#ordenar).

### const no impide modificar el contenido

**Esperas:** que un array declarado con `const` ya no pueda cambiar.

```javascript
const carrito = ["manzana"];
carrito.push("pera");
carrito[0] = "kiwi";
console.log(carrito);

const gato = { nombre: "Félix" };
gato.nombre = "Tom";
console.log(gato);
```

```sortie
[ 'kiwi', 'pera' ]
{ nombre: 'Tom' }
```

**Por qué:** imagina una caja con una etiqueta. `const` pega la etiqueta para siempre
en **la misma caja**. Pero siempre puedes cambiar lo que hay **dentro** de la caja.

```text
 carrito ──(pegada para siempre)──▶ [ 'kiwi', 'pera' ]
                                      ▲ el contenido puede cambiar
```

Lo que está prohibido es poner la etiqueta en **otra** caja: `carrito = ["pera"]`
provoca `TypeError: Assignment to constant variable.`

**La corrección:** si el contenido de verdad no debe cambiar, congélalo con
`Object.freeze`.

```javascript
const dias = Object.freeze(["lunes", "martes"]);
console.log(Object.isFrozen(dias));
```

```sortie
true
```

### b = a no copia

**Esperas:** que `b` sea una copia independiente de `a`.

```javascript
const a = [1, 2, 3];
const b = a;
b.push(4);
console.log(a);
```

```sortie
[ 1, 2, 3, 4 ]
```

**Por qué:** `b = a` pega una **segunda etiqueta** en la misma caja. Solo hay un array,
con dos nombres.

```text
 a ──┐
     ├──▶ [ 1, 2, 3, 4 ]
 b ──┘
```

**La corrección:** haz una copia con el spread `...`.

```javascript
const a = [1, 2, 3];
const b = [...a];
b.push(4);
console.log(a);
console.log(b);
```

```sortie
[ 1, 2, 3 ]
[ 1, 2, 3, 4 ]
```

Cuidado, esta copia solo copia el primer nivel. Para un array de objetos, mira
[los arrays en profundidad](tableaux-avances.md).

> **Recuerda:** para los arrays y los objetos, `=` comparte. `[...a]` y `{ ...o }`
> copian.

### this se pierde cuando separas un método

**Esperas:** que el método funcione en cualquier sitio, aunque lo guardes en una
variable.

```javascript norun
const cuenta = {
  titular: "Alice",
  resumen() {
    return `Cuenta de ${this.titular}`;
  },
};

const mostrar = cuenta.resumen;
console.log(mostrar());
```

```text
file:///home/alice/js/main.mjs:4
    return `Cuenta de ${this.titular}`;
                             ^

TypeError: Cannot read properties of undefined (reading 'titular')
    at resumen (file:///home/alice/js/main.mjs:4:30)
    at file:///home/alice/js/main.mjs:9:13

Node.js v22.22.0
```

**Por qué:** `this` no se fija cuando **escribes** el método. Se elige en el momento
de la **llamada**: es el objeto que va antes del punto. En `cuenta.resumen()`, `this`
es `cuenta`. En `mostrar()`, no hay punto, así que no hay objeto: `this` vale
`undefined`.

**La corrección:** llama al método con su objeto, o átalo con `bind`.

```javascript
const cuenta = {
  titular: "Alice",
  resumen() {
    return `Cuenta de ${this.titular}`;
  },
};

console.log(cuenta.resumen());

const mostrar = cuenta.resumen.bind(cuenta);
console.log(mostrar());
```

```sortie
Cuenta de Alice
Cuenta de Alice
```

Mira [los métodos y this](objets.md#los-metodos-y-this).

## Las trampas del tiempo y de los bucles

### Olvidar await

**Esperas:** que `leerNota()` te dé la nota `15`.

```javascript
function esperar(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function leerNota() {
  await esperar(10);
  return 15;
}

const nota = leerNota();
console.log(nota);
console.log(nota + 1);
```

```sortie
Promise { <pending> }
[object Promise]1
```

**Por qué:** una función `async` no devuelve el valor enseguida. Devuelve una
[promesa](../debuter/glossaire.md#promesa): un tique de pedido, como en un restaurante
de comida rápida. ¡El tique no es el plato! `<pending>` quiere decir «en
preparación».

**La corrección:** espera el plato con `await`.

```javascript
function esperar(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function leerNota() {
  await esperar(10);
  return 15;
}

const nota = await leerNota();
console.log(nota);
console.log(nota + 1);
```

```sortie
15
16
```

Mira [la asincronía](asynchrone.md).

### var en un bucle: la misma caja para todos

**Esperas:** `0`, `1`, `2`.

```javascript
for (var i = 0; i < 3; i++) {
  setTimeout(() => console.log("var:", i), 0);
}
```

```sortie
var: 3
var: 3
var: 3
```

**Por qué:** `setTimeout` muestra **más tarde**, después del final del bucle. Con
`var`, hay **una sola** caja `i` para todo el bucle. Cuando se muestran los mensajes,
el bucle ha terminado e `i` vale 3.

| Momento | i | Lo que pasa |
| --- | --- | --- |
| vuelta 1 | 0 | se programa una salida para más tarde |
| vuelta 2 | 1 | se programa una salida para más tarde |
| vuelta 3 | 2 | se programa una salida para más tarde |
| fin del bucle | 3 | `3 < 3` es falso: se sale |
| más tarde | 3 | las 3 salidas leen la misma caja: `3` |

**La corrección:** usa `let`. Con `let`, **cada vuelta** tiene su propia caja `i`.

```javascript
for (let i = 0; i < 3; i++) {
  setTimeout(() => console.log("let:", i), 0);
}
```

```sortie
let: 0
let: 1
let: 2
```

> **Recuerda:** no uses nunca `var`. Escribe `const`, o `let` si el valor tiene que
> cambiar.

## Errores frecuentes

### Leer una propiedad de null

```javascript norun
const respuesta = { datos: null };
console.log(respuesta.datos.length);
```

```text
file:///home/alice/js/main.mjs:2
console.log(respuesta.datos.length);
                            ^

TypeError: Cannot read properties of null (reading 'length')
    at file:///home/alice/js/main.mjs:2:29

Node.js v22.22.0
```

No se puede leer nada en una caja vacía. Usa `respuesta.datos?.length`, que da
`undefined` en lugar de fallar.

### Llamar a un método de número sobre un texto

```javascript norun
const edad = "17";
console.log(edad.toFixed(1));
```

```text
file:///home/alice/js/main.mjs:2
console.log(edad.toFixed(1));
                 ^

TypeError: edad.toFixed is not a function
    at file:///home/alice/js/main.mjs:2:18

Node.js v22.22.0
```

`"17"` es un texto, no un número: no tiene método `toFixed`. Conviértelo antes:
`Number(edad).toFixed(1)`.

### Reemplazar un array declarado con const

```javascript norun
const carrito = ["manzana"];
carrito = ["pera"];
```

```text
file:///home/alice/js/main.mjs:2
carrito = ["pera"];
        ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:2:9

Node.js v22.22.0
```

Modifica el contenido (`carrito.length = 0; carrito.push("pera")`), o declara con `let`
si de verdad necesitas cambiar de array.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra `console.log("3" + 4 + 5)`?

> **Respuesta**
>
> `345`. Se lee de izquierda a derecha: `"3" + 4` da el texto `"34"`, y luego
> `"34" + 5` da `"345"`.
>
> ```javascript
> console.log("3" + 4 + 5);
> console.log(3 + 4 + "5");
> ```
>
> ```sortie
> 345
> 75
> ```
>
> En la segunda línea, `3 + 4` primero da `7` (dos números), y luego `7 + "5"` pega:
> `"75"`.

**Pregunta 2:** ¿cómo saber si `x` vale `NaN`?

> **Respuesta**
>
> Con `Number.isNaN(x)`. La prueba `x === NaN` siempre responde `false`, porque `NaN`
> no es igual a nada.

**Pregunta 3:** ¿por qué `[3, 20, 100].sort()` da `[100, 20, 3]`?

> **Respuesta**
>
> Porque `sort()` sin argumento compara **textos**: `"1"` va antes que `"2"`, que va
> antes que `"3"`. Escribe `sort((a, b) => a - b)`.

## En resumen

- Compara con `===`, nunca con `==`.
- `+` pega en cuanto ve un texto: convierte con `Number(...)`; comprueba `NaN` con
  `Number.isNaN`.
- Los números decimales no son exactos: redondea para mostrar, cuenta en céntimos para
  el dinero.
- `undefined` = nunca rellenado, `null` = vaciado a propósito; `??` y `?.` manejan los
  dos.
- `sort((a, b) => a - b)` para los números; `const` no impide modificar el contenido;
  `b = a` comparte, `[...a]` copia.
- `this` depende de la llamada; una función `async` devuelve una promesa que hay que
  esperar con `await`; `let` mejor que `var`.

Siguiente: [ejercicios para empezar bien](exercices-faciles.md).
