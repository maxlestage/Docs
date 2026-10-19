# Los bucles

Esta página explica cómo repetir acciones con los bucles `for`, `while` y `for...of`, y cómo detenerlos con `break` y `continue`.

> **Objetivos:**
>
> - sabrás repetir con `for`, `while` y `for...of`;
> - sabrás saltar una vuelta con `continue` y salir de un bucle con `break`.

Este capítulo sigue a [las condiciones](conditions-boucles.md): un bucle se detiene gracias a una condición, como las que ya sabes escribir.

## Los bucles

Imagina que te piden mostrar los números del 1 al 100. Escribir 100 `console.log`
sería largo y pesado. Un bucle hace ese trabajo repetitivo por ti.

Un [bucle](../debuter/glossaire.md#bucle) repite un bloque de código.

### for: contar

El bucle `for` tiene tres partes, separadas por puntos y coma: el inicio, la
condición para continuar, y el paso.

```typescript
for (let i = 1; i <= 3; i++) {
  console.log(`Vuelta ${i}`);
}
```

```sortie
Vuelta 1
Vuelta 2
Vuelta 3
```

- `let i = 1`: el contador empieza en 1;
- `i <= 3`: se continúa mientras `i` valga como mucho 3;
- `i++`: después de cada vuelta, se suma 1 a `i`.

**Te toca:** escribe un programa que muestre la tabla del 7, de «7 x 1 = 7» a
«7 x 5 = 35», una línea por cálculo.

> **Solución**
>
> ```typescript
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

### while: mientras

`while` («mientras») repite mientras su condición sea verdadera. Úsalo cuando
no sabes de antemano cuántas vueltas hacen falta.

```typescript
let dinero = 100;
let anios = 0;

while (dinero < 150) {
  dinero = dinero * 1.1;
  anios++;
}

console.log(`${anios} años, ${dinero.toFixed(2)} euros`);
```

```sortie
5 años, 161.05 euros
```

Línea por línea:

- al principio, `dinero` vale 100 y `anios` vale 0;
- antes de cada vuelta, `while` comprueba `dinero < 150`. 100 es menor: entra en el
  bloque;
- en cada vuelta, `dinero` crece un 10 % (110, luego 121, 133.1, 146.41, 161.051) y
  `anios` aumenta en 1;
- tras 5 vueltas, `dinero < 150` es falso: el bucle se detiene, y el último
  `console.log` muestra el resultado, redondeado a dos cifras por `toFixed(2)`.

> **Atención:** si la condición sigue siendo siempre verdadera, el bucle no se detiene nunca.
> Comprueba que algo cambia en cada vuelta. Para detener un programa bloqueado,
> pulsa `Ctrl+C` en la terminal.

### for...of: recorrer una lista

`for...of` toma los elementos de una lista uno a uno. La lista se escribe entre corchetes;
los [arrays](collections.md) se explican más adelante.

```typescript
const frutas = ["manzana", "pera", "kiwi"];

for (const fruta of frutas) {
  console.log(fruta.toUpperCase());
}
```

```sortie
MANZANA
PERA
KIWI
```

TypeScript sabe que `frutas` contiene `string`: por tanto `fruta` es un `string`, y
`toUpperCase()` está permitido. Basta con un `const`: cada vuelta crea una nueva `fruta`.

**Te toca:** escribe un programa que guarde la lista de la compra `["pan", "leche",
"huevos"]` en una constante y luego muestre cada artículo en su línea, precedido de
un guion.

> **Solución**
>
> ```typescript
> const compras = ["pan", "leche", "huevos"];
>
> for (const articulo of compras) {
>   console.log(`- ${articulo}`);
> }
> ```
>
> ```sortie
> - pan
> - leche
> - huevos
> ```

### break y continue

- `break` sale del bucle de inmediato;
- `continue` pasa directamente a la vuelta siguiente.

```typescript
for (let i = 1; i <= 10; i++) {
  if (i % 2 === 0) {
    continue;
  }
  if (i > 7) {
    break;
  }
  console.log(i);
}
```

```sortie
1
3
5
7
```

Los números pares se saltan (`continue`), y el bucle se detiene en 9 (`break`).

## Errores frecuentes

### Usar el contador después del bucle

```typescript norun
for (let i = 0; i < 3; i++) {
  console.log(i);
}
console.log(i);
```

```text
main.ts:4:13 - error TS2304: Cannot find name 'i'.

4 console.log(i);
              ~


Found 1 error in main.ts:4
```

Una variable creada con `let` solo existe dentro de su bloque: es su
[ámbito](../debuter/glossaire.md#ambito). Si necesitas el valor después del
bucle, crea la variable antes: `let i = 0; for (; i < 3; i++) { ... }`.

### Declarar el contador con const

```typescript norun
for (const i = 0; i < 3; i++) {
  console.log(i);
}
```

```text
main.ts:1:26 - error TS2588: Cannot assign to 'i' because it is a constant.

1 for (const i = 0; i < 3; i++) {
                           ~


Found 1 error in main.ts:1
```

«No se puede modificar `i`: es una constante.» `i++` cambia el contador en cada
vuelta, así que debe crearse con `let`, como en `for (let i = 0; i < 3; i++)`. En un
`for...of`, en cambio, `const` sirve: cada vuelta crea un elemento nuevo.

## Ponte a prueba

**Pregunta 1:** escribe un bucle que muestre los números de 10 a 0, de 2 en 2.

> **Respuesta**
>
> ```typescript
> for (let i = 10; i >= 0; i -= 2) {
>   console.log(i);
> }
> ```
>
> ```sortie
> 10
> 8
> 6
> 4
> 2
> 0
> ```
>
> `i -= 2` es un atajo de `i = i - 2`.

**Pregunta 2:** ¿qué muestra este código?

```typescript norun
let n = 1;
while (n < 20) {
  n = n * 3;
}
console.log(n);
```

> **Respuesta**
>
> `27`. `n` vale 1, luego 3, 9 y 27. Con 27, `n < 20` es falso: el bucle se detiene y
> `console.log` muestra 27.

**Pregunta 3:** ¿cuál es la diferencia entre `break` y `continue`?

> **Respuesta**
>
> `break` sale del bucle de inmediato. `continue` salta el resto de la vuelta y pasa
> directamente a la vuelta siguiente: el bucle sigue.

## En resumen

- Un bucle repite un bloque de código.
- `for` cuenta: un inicio, una condición para seguir y un paso.
- `while` repite mientras su condición sea verdadera: algo debe cambiar en cada
  vuelta.
- `for...of` recorre una lista, elemento por elemento.
- `break` sale del bucle, `continue` pasa a la vuelta siguiente.

Siguiente: [las funciones](fonctions.md).
