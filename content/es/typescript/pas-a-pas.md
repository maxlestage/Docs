# Seguir un programa paso a paso

Esta página te enseña a ejecutar un programa TypeScript en tu cabeza, línea a línea, con una tabla de seguimiento, en cinco pequeños programas cada vez más completos.

> **Objetivos:**
>
> - sabrás rellenar una tabla de seguimiento: una columna por variable, una fila por
>   paso;
> - sabrás seguir una condición, un bucle y la llamada a una función;
> - sabrás adivinar lo que muestra un programa **antes** de ejecutarlo;
> - sabrás hacer que el programa muestre sus pasos con `console.log`;
> - sabrás dónde buscar cuando tu tabla y el ordenador no están de acuerdo.

## Por qué seguir un programa en tu cabeza

El ordenador es un cocinero muy obediente, pero sin ninguna imaginación. Lee la receta
una línea cada vez, de arriba abajo. Hace **exactamente** lo que está escrito, aunque
sea absurdo.

Para entender un programa, haz como él. Toma un papel y un lápiz. Lee una línea, haz lo
que dice, apunta el resultado. Luego pasa a la línea siguiente.

Este papel se llama **tabla de seguimiento**. Es la mejor arma contra los
[bugs](../debuter/glossaire.md#bug): la mayoría vienen de una diferencia entre lo que
**crees** que hace el programa y lo que hace **de verdad**.

```text
   lo que tú crees               lo que hace el ordenador
   ┌────────────────┐            ┌────────────────┐
   │ total vale 20  │    ≠       │ total vale 12  │   ◄── el bug está aquí
   └────────────────┘            └────────────────┘
       la tabla de seguimiento muestra la diferencia
```

> **Recuerda:** el ordenador nunca adivina. Sigue las líneas una a una. Para
> entenderlo, haz lo mismo, en papel.

## El método de la tabla de seguimiento

Una tabla de seguimiento se parece a un cuaderno de notas:

- una columna **Paso** (o **Línea**): dónde está el programa;
- una columna por [variable](../debuter/glossaire.md#variable): su
  [valor](../debuter/glossaire.md#valeur) en ese momento;
- una columna **Muestra**: lo que `console.log` escribe en la pantalla.

Cuatro reglas sencillas:

1. Una fila de la tabla por **cada** línea de código ejecutada, en orden.
2. Cuando una variable cambia, escribe su **nuevo** valor. Tacha el antiguo en tu
   cabeza: ya no existe.
3. Una casilla vacía quiere decir «todavía no creada» o «sin cambios».
4. Antes de ejecutar el programa, **adivina** la salida. Luego compara.

En los programas de esta página, cada línea lleva un comentario `// L1`, `// L2`… Es
el número de línea: sirve para rellenar la columna **Paso**.

> **Recuerda:** una línea de código ejecutada = una fila de la tabla. Nunca te saltes
> una línea, aunque parezca evidente.

## Programa 1: variables que cambian

Aquí tienes unas líneas que juegan con dos cajas, `a` y `b`.

```typescript
let a = 5; // L1
let b = 3; // L2
a = a + b; // L3
b = a - b; // L4
a = a - b; // L5
console.log(a, b); // L6
```

**Antes de ejecutarlo, adivina lo que muestra.** Escribe tu respuesta en un papel.

```sortie
3 5
```

Aquí está la tabla de seguimiento. Léela fila a fila.

| Paso | Qué pasa | a | b | Muestra |
|---|---|---|---|---|
| L1 | se crea `a` | 5 | | |
| L2 | se crea `b` | 5 | 3 | |
| L3 | `a + b` = 5 + 3 = 8 | **8** | 3 | |
| L4 | `a - b` = 8 − 3 = 5 | 8 | **5** | |
| L5 | `a - b` = 8 − 5 = 3 | **3** | 5 | |
| L6 | se muestra | 3 | 5 | `3 5` |

La trampa está en la línea L5. Se calcula `a - b` con el **nuevo** valor de `b` (5),
no con el antiguo (3). Así que el programa ha **intercambiado** los valores de `a` y
`b`.

El signo `=` no quiere decir «es igual a» como en matemáticas. Quiere decir: «calcula
la parte derecha y luego guarda el resultado en la caja de la izquierda».

```text
   a = a + b
   ▲   └─┬─┘
   │     └── 1. se calcula con los valores ACTUALES: 5 + 3 = 8
   └──────── 2. se guarda 8 en la caja a (el antiguo 5 se tira)
```

> **Pruébalo:** cambia `let b = 3;` por `let b = 10;`. Rehaz la tabla. El programa
> muestra `10 5`: sigue intercambiando los dos valores.

> **Recuerda:** en cada línea, usa el valor más **reciente** de cada variable, el de la
> fila justo encima en tu tabla.

## Programa 2: una condición

Un cine cobra 10 €, 5 € a los menores de 12 años y 7 € a los menores de 18 años.

```typescript
const edad = 15; // L1
let precio = 10; // L2
if (edad < 12) { // L3
  precio = 5; // L4
} else if (edad < 18) { // L5
  precio = 7; // L6
}
console.log(`Precio: ${precio} €`); // L7
```

**Antes de ejecutarlo, adivina lo que muestra.**

```sortie
Precio: 7 €
```

Una [condición](../debuter/glossaire.md#condition) es como un cambio de agujas de
tren: el programa toma **un solo** camino. Añade una columna **Prueba** para apuntar
el resultado de cada pregunta: `true` (verdadero) o `false` (falso).

| Paso | Prueba | edad | precio | Muestra |
|---|---|---|---|---|
| L1 | | 15 | | |
| L2 | | 15 | 10 | |
| L3 | `15 < 12` → `false` | 15 | 10 | |
| L4 | *saltada* | | | |
| L5 | `15 < 18` → `true` | 15 | 10 | |
| L6 | | 15 | **7** | |
| L7 | | 15 | 7 | `Precio: 7 €` |

La línea L4 **nunca** se ejecuta: la primera prueba es falsa, el programa salta
directamente al `else if`.

```text
              edad < 12 ?
             /           \
           sí             no
           │               │
      precio = 5      edad < 18 ?
                     /           \
                   sí             no
                   │               │
              precio = 7     (nada: precio sigue en 10)
```

> **Pruébalo:** pon `const edad = 30;`. Las dos pruebas son falsas: el programa
> muestra `Precio: 10 €`. Luego pon `const edad = 12;`: `12 < 12` es **falso**, así
> que obtienes `Precio: 7 €`. Los casos «justo en el límite» siempre merecen una fila
> en tu tabla.

> **Recuerda:** para una condición, escribe la prueba con los valores reales
> (`15 < 12`) y su resultado (`false`). Luego sigue **un solo** camino.

## Programa 3: un bucle que acumula

Un [bucle](../debuter/glossaire.md#boucle) repite las mismas líneas. Aquí se suma el
doble de `i` a `total`, cuatro veces.

```typescript
let total = 0; // L1
for (let i = 1; i <= 4; i++) { // L2
  total = total + i * 2; // L3
  console.log(`i = ${i}, total = ${total}`); // L4
}
console.log(`Fin: ${total}`); // L5
```

**Antes de ejecutarlo, adivina lo que muestra.** ¿Cuántas líneas? ¿Cuál es la última?

```sortie
i = 1, total = 2
i = 2, total = 6
i = 3, total = 12
i = 4, total = 20
Fin: 20
```

La línea L2 hace tres cosas, en tres momentos distintos:

```text
   for (let i = 1;   i <= 4;   i++)
        └──┬────┘    └──┬─┘    └┬┘
     1. una sola    2. antes    3. al final
        vez al         de cada     de cada
        principio      vuelta      vuelta
```

Aquí está la tabla completa. Es larga: es normal, un bucle da muchos pasos.

| Paso | Prueba `i <= 4` | i | total | Muestra |
|---|---|---|---|---|
| L1 | | | 0 | |
| L2 (inicio) | | 1 | 0 | |
| L2 (prueba) | `1 <= 4` → `true` | 1 | 0 | |
| L3 | | 1 | 0 + 1 × 2 = **2** | |
| L4 | | 1 | 2 | `i = 1, total = 2` |
| L2 (`i++`, prueba) | `2 <= 4` → `true` | **2** | 2 | |
| L3 | | 2 | 2 + 2 × 2 = **6** | |
| L4 | | 2 | 6 | `i = 2, total = 6` |
| L2 (`i++`, prueba) | `3 <= 4` → `true` | **3** | 6 | |
| L3 | | 3 | 6 + 3 × 2 = **12** | |
| L4 | | 3 | 12 | `i = 3, total = 12` |
| L2 (`i++`, prueba) | `4 <= 4` → `true` | **4** | 12 | |
| L3 | | 4 | 12 + 4 × 2 = **20** | |
| L4 | | 4 | 20 | `i = 4, total = 20` |
| L2 (`i++`, prueba) | `5 <= 4` → `false` | **5** | 20 | |
| L5 | | | 20 | `Fin: 20` |

Mira la penúltima fila: `i` vale **5**, la prueba es falsa, el bucle se detiene. Esta
última prueba es el paso que todo el mundo olvida.

`total` se comporta como una hucha: en cada vuelta se mete una moneda, y nunca se
vacía.

> **Pruébalo:** cambia `i <= 4` por `i < 4`. El bucle da una vuelta menos. El programa
> muestra tres líneas y luego `Fin: 12`.

> **Recuerda:** en un bucle, apunta la prueba en **cada** vuelta, incluida la última,
> la que es falsa y lo detiene todo.

## Programa 4: una función llamada dos veces

Una [función](../debuter/glossaire.md#fonction) es una máquina: le das valores y te
devuelve un resultado. Aquí, `precioRebajado` calcula un precio después de una rebaja
en porcentaje.

```typescript
function precioRebajado(precio: number, porcentaje: number): number { // L1
  const rebaja = (precio * porcentaje) / 100; // L2
  return precio - rebaja; // L3
}

const libro = precioRebajado(20, 10); // L4
const juego = precioRebajado(50, 20); // L5
console.log(libro, juego, libro + juego); // L6
```

**Antes de ejecutarlo, adivina lo que muestra.**

```sortie
18 40 58
```

Cuidado con la primera trampa: las líneas L1 a L3 no hacen **nada** cuando el programa
las lee por primera vez. **Describen** la máquina, sin ponerla en marcha. El programa
empieza de verdad en la línea L4.

Así se ve quién llama a quién, y qué vuelve:

```text
   programa principal                         precioRebajado
   ──────────────────                         ──────────────
L4 libro = precioRebajado(20, 10)  ────────►  precio = 20, porcentaje = 10
                                              L2 rebaja = 2
          libro = 18               ◄────────  L3 devuelve 18
L5 juego = precioRebajado(50, 20)  ────────►  precio = 50, porcentaje = 20
                                              L2 rebaja = 10
          juego = 40               ◄────────  L3 devuelve 40
L6 console.log(...)
```

Cada llamada abre una **hoja nueva** de borrador para la función. Sus
[parámetros](../debuter/glossaire.md#parametre) `precio` y `porcentaje` reciben los
[argumentos](../debuter/glossaire.md#argument) de la llamada. En el `return`, la hoja
se tira: `rebaja` ya no existe.

| Paso | Dónde | precio | porcentaje | rebaja | libro | juego | Muestra |
|---|---|---|---|---|---|---|---|
| L4 | principal: llamada `precioRebajado(20, 10)` | | | | | | |
| L1 | dentro de la función | 20 | 10 | | | | |
| L2 | dentro de la función | 20 | 10 | 20 × 10 / 100 = 2 | | | |
| L3 | devuelve 20 − 2 = 18 | 20 | 10 | 2 | | | |
| L4 | principal: se guarda 18 | | | | **18** | | |
| L5 | principal: llamada `precioRebajado(50, 20)` | | | | 18 | | |
| L1 | dentro de la función (hoja nueva) | 50 | 20 | | 18 | | |
| L2 | dentro de la función | 50 | 20 | 50 × 20 / 100 = 10 | 18 | | |
| L3 | devuelve 50 − 10 = 40 | 50 | 20 | 10 | 18 | | |
| L5 | principal: se guarda 40 | | | | 18 | **40** | |
| L6 | principal | | | | 18 | 40 | `18 40 58` |

Las casillas vacías de `precio`, `porcentaje` y `rebaja` en el programa principal no
son un olvido: esas variables existen **solo** durante la llamada.

> **Pruébalo:** cambia `precioRebajado(20, 10)` por `precioRebajado(10, 20)`. El orden
> de los argumentos importa: `precio` vale ahora 10 y `porcentaje` 20. El libro cuesta
> `8`, y el programa muestra `8 40 48`.

> **Recuerda:** para una función, dibuja la flecha de ida (los argumentos) y la flecha
> de vuelta (el valor devuelto). Cada llamada tiene su propia hoja, completamente nueva.

## Programa 5: un bucle sobre un array, con una condición

Último programa: lo reúne todo. Se recorren unas notas. Se cuentan las notas aprobadas
(10 o más) y se busca la mejor.

```typescript
const notas = [12, 8, 15, 9, 18]; // L1
let aprobadas = 0; // L2
let mejor = 0; // L3
for (const nota of notas) { // L4
  if (nota >= 10) { // L5
    aprobadas = aprobadas + 1; // L6
  }
  if (nota > mejor) { // L7
    mejor = nota; // L8
  }
}
console.log(`Aprobadas: ${aprobadas} de ${notas.length}`); // L9
console.log(`Mejor: ${mejor}`); // L10
```

**Antes de ejecutarlo, adivina lo que muestra.**

```sortie
Aprobadas: 3 de 5
Mejor: 18
```

`for...of` toma los elementos del array uno a uno, como se toman las cartas de una
baraja, de la primera a la última.

```text
   notas:   [ 12 | 8 | 15 | 9 | 18 ]
              ▲    ▲    ▲   ▲    ▲
   vuelta:    1    2    3   4    5      luego no quedan cartas: fin del bucle
```

Para ir más rápido, se agrupa una vuelta del bucle en una sola fila de la tabla, con
sus dos pruebas:

| Vuelta | nota | ¿`nota >= 10`? | aprobadas | ¿`nota > mejor`? | mejor |
|---|---|---|---|---|---|
| antes | | | 0 | | 0 |
| 1 | 12 | `true` | **1** | `12 > 0` → `true` | **12** |
| 2 | 8 | `false` | 1 | `8 > 12` → `false` | 12 |
| 3 | 15 | `true` | **2** | `15 > 12` → `true` | **15** |
| 4 | 9 | `false` | 2 | `9 > 15` → `false` | 15 |
| 5 | 18 | `true` | **3** | `18 > 15` → `true` | **18** |
| después | | | 3 | | 18 |

Los dos `if` son **independientes**: no es un `if … else`. En cada vuelta, el programa
hace las dos preguntas.

> **Pruébalo:** cambia las notas por `[-3, -1, -7]`. El programa muestra `Mejor: 0`,
> ¡aunque 0 no está en el array! Tu tabla de seguimiento lo muestra enseguida: ninguna
> prueba `nota > mejor` es verdadera. Para corregirlo, empieza con
> `let mejor = notas[0];`.

> **Recuerda:** para un bucle sobre un array, una fila de la tabla de seguimiento por
> elemento. Escribe cada prueba con los valores reales.

## Hacer que el programa muestre sus pasos

La tabla en papel es tu predicción. Para comprobarla, pide al programa que escriba
**él mismo** su tabla, con `console.log`.

### Etiquetas en console.log

Un número solo en la pantalla no dice de dónde viene. Añade siempre una etiqueta.

```typescript
let total = 0;
for (let i = 1; i <= 3; i++) {
  total = total + i * 2;
  console.log("vuelta", i, "| total =", total);
}
```

```sortie
vuelta 1 | total = 2
vuelta 2 | total = 6
vuelta 3 | total = 12
```

### El truco de las llaves

Rodea tus variables con llaves: `console.log({ i, total })`. Node muestra entonces el
**nombre** y el **valor** de cada variable, sin que tengas que escribir las etiquetas.

```typescript
let total = 0;
for (let i = 1; i <= 3; i++) {
  total = total + i * 2;
  console.log({ i, total });
}
```

```sortie
{ i: 1, total: 2 }
{ i: 2, total: 6 }
{ i: 3, total: 12 }
```

### Entrar y salir de una función

Para una función, muestra lo que entra y lo que sale. Una flecha ayuda a leerlo.

```typescript
function precioRebajado(precio: number, porcentaje: number): number {
  console.log(`  → entrada: precio = ${precio}, porcentaje = ${porcentaje}`);
  const resultado = precio - (precio * porcentaje) / 100;
  console.log(`  ← salida: ${resultado}`);
  return resultado;
}

console.log("llamada 1");
const libro = precioRebajado(20, 10);
console.log("llamada 2");
const juego = precioRebajado(50, 20);
console.log("fin:", libro, juego);
```

```sortie
llamada 1
  → entrada: precio = 20, porcentaje = 10
  ← salida: 18
llamada 2
  → entrada: precio = 50, porcentaje = 20
  ← salida: 40
fin: 18 40
```

> **Pruébalo:** en el programa 5, añade `console.log({ nota, aprobadas, mejor });` al
> final del bucle. Obtienes cinco líneas: compáralas con tu tabla de seguimiento.

> **Recuerda:** un `console.log` con una etiqueta es una tabla de seguimiento escrita
> por el ordenador. Quítalos cuando hayas encontrado el bug.

## El depurador, en dos frases

Un [depurador](../debuter/glossaire.md#debogage) detiene el programa en una línea
elegida (un punto de interrupción) y te deja avanzar línea a línea, mostrando todas
las variables: es una tabla de seguimiento automática. En VS Code, haz clic en el
margen a la izquierda de una línea y luego lanza «Ejecutar y depurar»; todo se explica
en [el capítulo sobre la depuración](debogage.md).

## Errores frecuentes

### Olvidar la última prueba del bucle

A menudo se piensa que `i` vale 4 después de un bucle `for (let i = 1; i <= 4; i++)`.
Pero el bucle se detiene porque la prueba se vuelve falsa: `i` vale entonces 5. Un
principiante escribe por ejemplo:

```typescript norun
let i = 1;
while (i <= 4) {
  i++;
}
console.log(`El bucle dio ${i} vueltas`);
```

```text
El bucle dio 5 vueltas
```

El programa miente: dio 4 vueltas. La tabla de seguimiento lo muestra, porque la
última fila es `5 <= 4` → `false`. **La corrección:** cuenta las vueltas en una
variable aparte.

```typescript
let i = 1;
let vueltas = 0;
while (i <= 4) {
  vueltas++;
  i++;
}
console.log(`El bucle dio ${vueltas} vueltas, e i vale ${i}`);
```

```sortie
El bucle dio 4 vueltas, e i vale 5
```

### Reutilizar un valor antiguo

En el programa 1, mucha gente calcula la línea L5 con `b = 3`, el valor del principio.
Obtienen `a = 8 − 3 = 5`. Es falso: la línea L4 ya ha puesto `b` a 5.

**La corrección:** mira siempre la fila **justo encima** en tu tabla, nunca la
primera.

### Equivocarse en una vuelta (el error de uno)

Un array de 3 notas tiene las posiciones 0, 1 y 2. Este código va una casilla
demasiado lejos:

```typescript norun
const notas = [12, 8, 15];
for (let i = 0; i <= notas.length; i++) {
  console.log(notas[i].toFixed(1));
}
```

`tsc` acepta este código. El programa muestra tres notas y luego falla (extracto del
mensaje):

```text
TypeError: Cannot read properties of undefined (reading 'toFixed')
```

Tu tabla de seguimiento habría mostrado la vuelta de más: `i = 3`, prueba `3 <= 3` →
`true`, y `notas[3]` vale `undefined`. **La corrección:** `<` en lugar de `<=`, o
mejor, `for...of`.

```typescript
const notas = [12, 8, 15];
for (const nota of notas) {
  console.log(nota.toFixed(1));
}
```

```sortie
12.0
8.0
15.0
```

### Creer que una variable de la función sigue existiendo

En el programa 4, `rebaja` vive en la hoja de borrador de la función. Después del
`return`, ha desaparecido.

```typescript norun
function precioRebajado(precio: number, porcentaje: number): number {
  const rebaja = (precio * porcentaje) / 100;
  return precio - rebaja;
}

const libro = precioRebajado(20, 10);
console.log(libro, rebaja);
```

```text
main.mts(7,20): error TS2304: Cannot find name 'rebaja'.
```

**La corrección:** si necesitas un valor, haz que la función lo **devuelva**, o
vuelve a calcularlo fuera. El capítulo sobre
[las funciones](fonctions.md#el-ambito-de-las-variables) detalla esta regla de ámbito.

## Ponte a prueba

1. ¿Cuánto vale `x` al final? `let x = 2; x = x * 3; x = x - 1; x = x * x;`

   > **Respuesta**
   >
   > `x` vale 25. Tabla: 2, luego 2 × 3 = 6, luego 6 − 1 = 5, luego 5 × 5 = 25. En
   > cada línea, se parte del valor justo encima.

2. ¿Cuántas veces se ejecuta `console.log` en `for (let i = 0; i < 3; i++) { console.log(i); }`,
   y cuántas veces se evalúa la prueba `i < 3`?

   > **Respuesta**
   >
   > `console.log` se ejecuta 3 veces (`i` = 0, 1, 2). La prueba se evalúa 4 veces: la
   > cuarta vez, `3 < 3` es falso y detiene el bucle.

3. ¿Qué muestra este programa?
   `function f(n: number): number { return n + 1; }` y luego `console.log(f(f(1)));`

   > **Respuesta**
   >
   > `3`. Primero se calcula la llamada interior: `f(1)` devuelve 2. Luego la llamada
   > exterior: `f(2)` devuelve 3. Dos llamadas, dos hojas de borrador.

## En resumen

- Una tabla de seguimiento: una columna por variable, una fila por línea de código
  ejecutada.
- Usa siempre el valor más reciente de una variable.
- Para una condición, escribe la prueba con los valores reales y sigue un solo
  camino.
- Para un bucle, apunta cada prueba, incluida la última, que es falsa.
- Para una función, dibuja la ida (argumentos) y la vuelta (valor devuelto).
- `console.log({ i, total })` hace que el programa escriba su tabla de seguimiento.

Siguiente: [las trampas del principiante en TypeScript](pieges.md)
