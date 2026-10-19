# Los bucles

Esta página muestra cómo repetir acciones con `while`, `for` y `for...of`, y cómo detener un bucle o saltarte una vuelta.

> **Objetivos:**
>
> - sabrás repetir código con `while`, `for` y `for...of`;
> - sabrás salir de un bucle con `break` o saltarte una vuelta con `continue`;
> - sabrás elegir entre `for...of` (los valores) y `for...in` (las claves).

Este capítulo sigue a [las condiciones](conditions-boucles.md): un bucle se detiene gracias
a una condición, como las que escribes en un `if`.

## El bucle while

Imagina que te piden mostrar los números del 1 al 100. Escribir 100 `console.log`
sería largo y pesado. Un bucle hace este trabajo repetitivo por ti.

Un [bucle](../debuter/glossaire.md#bucle) repite un bloque de código. `while` («mientras»)
lo repite mientras la condición sea verdadera.

```javascript
let contador = 3;

while (contador > 0) {
  console.log(contador);
  contador--;
}
console.log("¡Despegue!");
```

```sortie
3
2
1
¡Despegue!
```

Línea por línea:

- `let contador = 3;`: la variable vale 3 al principio. Es un `let`, porque va a
  cambiar;
- `while (contador > 0)`: antes de cada vuelta, JavaScript hace la pregunta. `3 > 0`
  es verdadero: entra en el bloque;
- el bloque muestra `contador` y luego `contador--` le resta 1: vale 2, luego 1, luego
  0;
- cuando `contador` vale 0, `0 > 0` es falso: JavaScript sale del bucle y pasa al
  último `console.log`.

> **Atención:** si la condición sigue siendo siempre verdadera, el bucle no se detiene nunca.
> Aquí, `contador--` acerca el contador a 0 en cada vuelta. Para detener un programa
> bloqueado, pulsa **Ctrl + C** en el terminal.

**Te toca:** escribe un programa que muestre los números del 1 al 5 con un bucle
`while` y luego «¡Terminado!».

> **Solución**
>
> ```javascript
> let n = 1;
>
> while (n <= 5) {
>   console.log(n);
>   n++;
> }
> console.log("¡Terminado!");
> ```
>
> ```sortie
> 1
> 2
> 3
> 4
> 5
> ¡Terminado!
> ```

## El bucle for

Muchos bucles cuentan: del 1 al 5, del 1 al 10… Con `while`, el inicio, la pregunta y
el paso están repartidos en varias líneas, y es fácil olvidar uno. `for` los reúne.

`for` agrupa en una línea las tres partes de un bucle que cuenta:
`for (inicio; condición; paso)`.

```javascript
for (let i = 1; i <= 5; i++) {
  console.log(`${i} x 3 = ${i * 3}`);
}
```

```sortie
1 x 3 = 3
2 x 3 = 6
3 x 3 = 9
4 x 3 = 12
5 x 3 = 15
```

1. **Inicio**: `let i = 1`, se ejecuta una vez al principio.
2. **Condición**: `i <= 5`, se comprueba antes de cada vuelta.
3. **Paso**: `i++`, se ejecuta después de cada vuelta.

**Te toca:** escribe un programa que muestre la tabla del 7, de «7 x 1 = 7» a
«7 x 5 = 35», una línea por cálculo.

> **Solución**
>
> ```javascript
> for (let i = 1; i <= 5; i++) {
>   console.log(`7 x ${i} = ${7 * i}`);
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```

## for...of y for...in

Muy a menudo tienes una lista de cosas que tratar una a una: cada artículo de una
lista de la compra, cada alumno de una clase. Estos dos bucles están hechos para eso.

### for...of: recorrer los valores

Para recorrer los elementos de un array (o las letras de un texto), usa
`for...of`. No hay contador que manejar: recibes cada valor, uno a uno.

```javascript
const frutas = ["manzana", "pera", "kiwi"];

for (const fruta of frutas) {
  console.log(`Me gusta la fruta: ${fruta}.`);
}

for (const letra of "abc") {
  console.log(letra);
}
```

```sortie
Me gusta la fruta: manzana.
Me gusta la fruta: pera.
Me gusta la fruta: kiwi.
a
b
c
```

Línea por línea:

- `frutas` contiene un array de tres textos;
- `for (const fruta of frutas)` toma el primer elemento, `"manzana"`, y lo guarda en
  `fruta`;
- el bloque muestra la frase con `"manzana"`. Luego JavaScript vuelve al `for`, que
  guarda `"pera"` en `fruta`, y así sucesivamente;
- después de `"kiwi"` no quedan elementos: el bucle se detiene. El segundo bucle hace
  lo mismo con las letras de `"abc"`.

Aquí `const` es correcto: en cada vuelta se crea una **nueva** variable `fruta`.

### for...in: recorrer las claves

`for...in` recorre los **nombres de las propiedades** de un objeto, no los valores.

```javascript
const puntuaciones = { alice: 12, bob: 8 };

for (const nombre in puntuaciones) {
  console.log(nombre, puntuaciones[nombre]);
}
```

```sortie
alice 12
bob 8
```

> **Recuerda:** `for...of` para los **valores** de un array o de un texto;
> `for...in` para las **claves** de un objeto. No mezcles los dos.

## break y continue

A veces quieres cambiar el rumbo de un bucle por el camino: dejar de buscar en cuanto
lo encuentras, o ignorar algunos elementos. Para eso sirven estas dos palabras.

- `break` sale del bucle enseguida.
- `continue` se salta el resto de la vuelta actual y pasa a la siguiente.

```javascript
const numeros = [4, 7, -1, 12, 0, 5];

for (const n of numeros) {
  if (n < 0) {
    continue;
  }
  if (n === 0) {
    break;
  }
  console.log(n);
}
```

```sortie
4
7
12
```

Línea por línea:

- `for (const n of numeros)` da a `n` los valores 4, 7, -1, 12… uno a uno;
- si `n` es negativo, `continue` pasa directamente a la vuelta siguiente: no se llega
  al `console.log`;
- si `n` vale 0, `break` detiene todo el bucle;
- si no, `console.log(n)` muestra el número.

`-1` se salta con `continue`. En `0`, `break` lo detiene todo: `5` nunca se muestra.

## Errores frecuentes

### Poner const en un for que cuenta

```javascript norun
for (const i = 0; i < 3; i++) {
  console.log(i);
}
```

```text
0
file:///home/alice/js/main.mjs:1
for (const i = 0; i < 3; i++) {
                          ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:1:27

Node.js v22.22.0
```

La primera vuelta funciona, y luego `i++` intenta modificar una constante. En un `for`
clásico, el contador cambia: escribe `let i = 0`. (En un `for...of`, `const` es
correcto.)

### Usar for...in con un array

```javascript norun
const frutas = ["manzana", "pera"];
for (const i in frutas) {
  console.log(i + 1);
}
```

```text
01
11
```

`for...in` da las **claves** del array, en forma de textos `"0"` y `"1"`. Y
`"0" + 1` pega los dos: `"01"`. Para los valores, usa `for...of`.

## Ponte a prueba

**Pregunta 1:** ¿cuántas veces muestra este código «Hola»?

```javascript norun
let n = 0;
while (n < 3) {
  console.log("Hola");
  n++;
}
```

> **Respuesta**
>
> Tres veces. `n` vale 0, 1 y luego 2 durante las vueltas; cuando vale 3, `n < 3` es
> falso y el bucle se detiene.

**Pregunta 2:** ¿qué muestra este código?

```javascript norun
for (const n of [1, 2, 3, 4, 5]) {
  if (n === 2) {
    continue;
  }
  if (n === 4) {
    break;
  }
  console.log(n);
}
```

> **Respuesta**
>
> `1` y luego `3`. `continue` se salta el `2`. En el `4`, `break` detiene el bucle: nunca se llega al
> `5`.

**Pregunta 3:** escribe un bucle que muestre los números pares del 2 al 10.

> **Respuesta**
>
> ```javascript
> for (let i = 2; i <= 10; i += 2) {
>   console.log(i);
> }
> ```
>
> ```sortie
> 2
> 4
> 6
> 8
> 10
> ```
>
> El paso `i += 2` se salta los números impares.

## En resumen

- `while` repite mientras una condición sea verdadera, `for` cuenta, `for...of` recorre los
  valores, `for...in` las claves de un objeto.
- `break` sale del bucle, `continue` pasa a la vuelta siguiente.
- En un `for` que cuenta, el contador se declara con `let`; en un `for...of`, `const`
  sirve.

Siguiente: [las funciones](fonctions.md).
