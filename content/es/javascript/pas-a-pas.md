# Seguir un programa paso a paso en JavaScript

Esta página te enseña a ejecutar un programa JavaScript en tu cabeza, línea por línea, con una tabla que sigue el valor de cada variable.

> **Objetivos:**
>
> - sabrás rellenar una tabla de traza, línea tras línea;
> - sabrás adivinar lo que muestra un programa pequeño antes de ejecutarlo;
> - sabrás seguir una condición, un bucle y una llamada a una función;
> - sabrás hacer que el programa muestre sus pasos con `console.log`;
> - sabrás evitar los errores más comunes al seguir un programa.

## ¿Por qué hacer de ordenador?

El ordenador no adivina nada. Lee tu programa **una línea cada vez**, de arriba abajo, y
hace exactamente lo que está escrito. Ni más ni menos.

Para entender un programa, haz como él. Es como seguir una receta de cocina con el
dedo: pones el dedo en una línea, la haces y luego bajas una línea.

```text
  línea 1   ◄── el dedo está aquí: hago la línea 1
  línea 2
  línea 3
```

Lo que ganas:

- entiendes **por qué** el programa muestra lo que muestra;
- encuentras un [bug](../debuter/glossaire.md#bug) sin siquiera ejecutar el código;
- lees más rápido el código de los demás.

> **Recuerda:** el ordenador lee una línea, la hace y pasa a la siguiente. Seguir un
> programa es hacer lo mismo, en papel.

## La tabla de traza

Una [variable](../debuter/glossaire.md#variable) es una caja con una etiqueta. Su valor
puede cambiar durante el programa. Para no olvidar nada, se anota todo en una **tabla de
traza**:

- una columna por variable;
- una fila por paso (una línea de código ejecutada);
- una columna «mostrado» para lo que enseña `console.log`.

```text
 ┌───────┬──────┬──────┬───────────┐
 │ Paso  │  a   │  b   │ mostrado  │   ← una columna por caja
 ├───────┼──────┼──────┼───────────┤
 │   1   │  3   │  –   │           │   ← una fila por paso
 │   2   │  3   │  5   │           │
 └───────┴──────┴──────┴───────────┘
```

El guion `–` quiere decir «la caja todavía no existe». Cuando un valor cambia, escribes
el nuevo valor en la fila de ese paso. El valor antiguo ya no cuenta.

Es como la libreta de un cajero: con cada compra, escribe el nuevo total. No guarda el
antiguo en la cabeza.

> **Recuerda:** una columna por variable, una fila por paso. Siempre se lee el valor
> **más reciente**, el de la fila justo encima.

## Programa 1: variables que cambian

Aquí tienes un programa de cinco líneas. Solo contiene cálculos.

```javascript
let a = 3;
let b = 5;
a = a + b;
b = a - b;
a = a - b;
console.log(a, b);
```

**Antes de ejecutarlo, adivina lo que se muestra.** Coge una hoja. No sigas leyendo
todavía.

Aquí está la traza. En cada paso, se calcula la parte a la derecha del `=` con los
valores **actuales** y luego se guarda el resultado en la caja de la izquierda.

| Paso | Línea | Cálculo | a | b | mostrado |
| --- | --- | --- | --- | --- | --- |
| 1 | `let a = 3;` | – | 3 | – | |
| 2 | `let b = 5;` | – | 3 | 5 | |
| 3 | `a = a + b;` | 3 + 5 = 8 | 8 | 5 | |
| 4 | `b = a - b;` | 8 − 5 = 3 | 8 | 3 | |
| 5 | `a = a - b;` | 8 − 3 = 5 | 5 | 3 | |
| 6 | `console.log(a, b);` | – | 5 | 3 | `5 3` |

```sortie
5 3
```

Los dos valores se han **intercambiado**: `a` valía 3 y ahora vale 5, `b` valía 5 y
ahora vale 3. En el paso 4, el cálculo usa el `a` **nuevo** (8), no el antiguo (3). Ese
es todo el secreto.

Prueba: cambia `let a = 3;` por `let a = 10;`. La traza da `5 10`: el intercambio
funciona con cualquier número.

> **Recuerda:** a la derecha del `=`, se leen los valores actuales. A la izquierda, se
> guarda el resultado. Solo cambia una caja por línea.

## Programa 2: una condición

Una [condición](../debuter/glossaire.md#condition) es un cambio de agujas: el programa
toma **un solo** camino. Aquí tienes el precio de una entrada de cine según la edad.

```javascript
const edad = 15;
let precio = 10;
if (edad < 12) {
  precio = 5;
} else if (edad < 18) {
  precio = 7;
}
console.log(`Precio: ${precio} €`);
```

**Antes de ejecutarlo, adivina lo que se muestra.**

En la traza, se añade una columna para la respuesta de cada prueba: `true` (verdadero) o
`false` (falso).

| Paso | Línea | Prueba | edad | precio | mostrado |
| --- | --- | --- | --- | --- | --- |
| 1 | `const edad = 15;` | – | 15 | – | |
| 2 | `let precio = 10;` | – | 15 | 10 | |
| 3 | `if (edad < 12)` | 15 < 12 → `false` | 15 | 10 | |
| 4 | `else if (edad < 18)` | 15 < 18 → `true` | 15 | 10 | |
| 5 | `precio = 7;` | – | 15 | 7 | |
| 6 | `console.log(…)` | – | 15 | 7 | `Precio: 7 €` |

```sortie
Precio: 7 €
```

La línea `precio = 5;` **nunca** se ejecuta: su prueba era falsa. Se salta, y no tiene
fila en la traza.

```text
            edad < 12 ?
           /          \
         sí            no
         │              │
    precio = 5      edad < 18 ?
                   /          \
                 sí            no
                 │              │
            precio = 7    (nada: precio sigue en 10)
```

Prueba: pon `const edad = 30;`. Las dos pruebas son falsas, no se ejecuta ningún bloque y
el programa muestra `Precio: 10 €`. Con `const edad = 12;`, la primera prueba es falsa
(12 no es menor que 12): el precio es 7.

> **Recuerda:** en cada `if`, escribe la prueba y su respuesta, `true` o `false`. Solo se
> ejecuta el bloque de la primera prueba verdadera; los demás se saltan.

## Programa 3: un bucle que acumula

Un [bucle](../debuter/glossaire.md#boucle) repite líneas. Aquí, una hucha: la semana 1
metes 2 €, la semana 2 metes 4 €, y así sucesivamente.

```javascript
let ahorro = 0;
let semana = 1;
while (semana <= 4) {
  ahorro = ahorro + semana * 2;
  semana = semana + 1;
}
console.log(`Semana ${semana}, ahorro ${ahorro} €`);
```

**Antes de ejecutarlo, adivina lo que se muestra.** Cuidado con el valor de `semana` al
final.

La prueba `semana <= 4` se vuelve a leer **antes de cada vuelta**. Se anota cada vez.

| Paso | Línea | Prueba | semana | ahorro |
| --- | --- | --- | --- | --- |
| 1 | `let ahorro = 0;` | – | – | 0 |
| 2 | `let semana = 1;` | – | 1 | 0 |
| 3 | `while (semana <= 4)` | 1 <= 4 → `true` | 1 | 0 |
| 4 | `ahorro = …` | 0 + 1 × 2 | 1 | 2 |
| 5 | `semana = …` | 1 + 1 | 2 | 2 |
| 6 | `while (semana <= 4)` | 2 <= 4 → `true` | 2 | 2 |
| 7 | `ahorro = …` | 2 + 2 × 2 | 2 | 6 |
| 8 | `semana = …` | 2 + 1 | 3 | 6 |
| 9 | `while (semana <= 4)` | 3 <= 4 → `true` | 3 | 6 |
| 10 | `ahorro = …` | 6 + 3 × 2 | 3 | 12 |
| 11 | `semana = …` | 3 + 1 | 4 | 12 |
| 12 | `while (semana <= 4)` | 4 <= 4 → `true` | 4 | 12 |
| 13 | `ahorro = …` | 12 + 4 × 2 | 4 | 20 |
| 14 | `semana = …` | 4 + 1 | 5 | 20 |
| 15 | `while (semana <= 4)` | 5 <= 4 → `false` | 5 | 20 |
| 16 | `console.log(…)` | – | 5 | 20 |

```sortie
Semana 5, ahorro 20 €
```

El bucle ha dado **4 vueltas**, pero la prueba se ha leído **5 veces**. La quinta prueba
es falsa: es ella la que detiene el bucle. Por eso `semana` vale 5 al final, y no 4.

```text
   ┌──► prueba: semana <= 4 ? ── no ──► salir del bucle
   │          │
   │          sí
   │          │
   │     sumar al ahorro
   │     semana + 1
   └──────────┘
```

Prueba: cambia `semana <= 4` por `semana < 4`. El bucle da 3 vueltas y el programa
muestra `Semana 4, ahorro 12 €`.

> **Recuerda:** en un bucle, anota la prueba en cada vuelta, **incluida la última**, la
> que es falsa. Ahí se esconden los errores de una vuelta de más o de menos.

## Programa 4: una función llamada dos veces

Una [función](../debuter/glossaire.md#fonction) es una pequeña máquina: le das un valor,
trabaja y te devuelve un resultado. Aquí, la llamamos dos veces.

```javascript
function cuadrado(n) {
  const resultado = n * n;
  console.log(`cuadrado(${n}) devuelve ${resultado}`);
  return resultado;
}

const a = cuadrado(3);
const b = cuadrado(a - 5);
console.log(`a + b = ${a + b}`);
```

**Antes de ejecutarlo, adivina lo que se muestra.** ¿Cuántas líneas? ¿En qué orden?

Las líneas 1 a 5 **definen** la función: todavía no se ejecuta nada. Es una receta
guardada en un cajón. El programa empieza de verdad en `const a = cuadrado(3);`.

Aquí tienes quién llama a quién, y lo que vuelve:

```text
 programa                           cuadrado
 ────────                           ────────
 const a = cuadrado(3)  ── n = 3 ─► resultado = 9
                                    muestra «cuadrado(3) devuelve 9»
           a = 9        ◄── 9 ────  return 9

 const b = cuadrado(4)  ── n = 4 ─► resultado = 16
                                    muestra «cuadrado(4) devuelve 16»
           b = 16       ◄── 16 ───  return 16

 muestra «a + b = 25»
```

La traza tiene una columna «¿dónde?» para saber en qué parte estamos:

| Paso | ¿Dónde? | Línea | n | resultado | a | b | mostrado |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | programa | `const a = cuadrado(3);` | – | – | – | – | |
| 2 | cuadrado | `const resultado = n * n;` | 3 | 9 | – | – | |
| 3 | cuadrado | `console.log(…)` | 3 | 9 | – | – | `cuadrado(3) devuelve 9` |
| 4 | cuadrado | `return resultado;` | 3 | 9 | – | – | |
| 5 | programa | `a` recibe 9 | – | – | 9 | – | |
| 6 | programa | `const b = cuadrado(a - 5);`: 9 − 5 = 4 | – | – | 9 | – | |
| 7 | cuadrado | `const resultado = n * n;` | 4 | 16 | 9 | – | |
| 8 | cuadrado | `console.log(…)` | 4 | 16 | 9 | – | `cuadrado(4) devuelve 16` |
| 9 | cuadrado | `return resultado;` | 4 | 16 | 9 | – | |
| 10 | programa | `b` recibe 16 | – | – | 9 | 16 | |
| 11 | programa | `console.log(…)` | – | – | 9 | 16 | `a + b = 25` |

```sortie
cuadrado(3) devuelve 9
cuadrado(4) devuelve 16
a + b = 25
```

Tres observaciones:

- en cada llamada, `n` y `resultado` son cajas **nuevas**. Desaparecen en el `return`
  (el guion `–` en los pasos 5 y 10);
- antes de llamar, JavaScript calcula el [argumento](../debuter/glossaire.md#argument):
  `a - 5` se convierte en `4`, y es `4` lo que entra en `n`;
- el `console.log` **dentro** de la función se muestra durante la llamada, antes de la
  última línea del programa.

Prueba: cambia `cuadrado(a - 5)` por `cuadrado(cuadrado(2))`. La llamada interior se hace
primero (`cuadrado(2) devuelve 4`) y luego la exterior (`cuadrado(4) devuelve 16`). La
salida pasa a ser `cuadrado(3) devuelve 9`, `cuadrado(2) devuelve 4`,
`cuadrado(4) devuelve 16` y luego `a + b = 25`.

> **Recuerda:** una llamada a una función es un viaje de ida y vuelta. Sales con el
> argumento, haces las líneas de la función con cajas nuevas y vuelves con el valor del
> `return`.

## Programa 5: un bucle sobre un array con una condición

El último programa lo mezcla todo: un array, un bucle y dos condiciones. Cuenta las
notas aprobadas (10 o más) y busca la mejor nota.

```javascript
const notas = [12, 7, 15, 9, 18];
let aprobadas = 0;
let mejor = notas[0];

for (const nota of notas) {
  if (nota >= 10) {
    aprobadas = aprobadas + 1;
  }
  if (nota > mejor) {
    mejor = nota;
  }
}

console.log(`Aprobadas: ${aprobadas}`);
console.log(`Mejor: ${mejor}`);
```

**Antes de ejecutarlo, adivina lo que se muestra.**

`for (const nota of notas)` pone en `nota` cada valor del array, uno tras otro. Es como
repartir cartas: una carta por vuelta.

```text
 notas:   [ 12 ,  7 , 15 ,  9 , 18 ]
            │     │    │    │    │
 vuelta:    1     2    3    4    5
```

Esta vez, hacemos **una fila por vuelta** (es más corto), con las dos pruebas:

| Vuelta | nota | nota >= 10 ? | aprobadas | nota > mejor ? | mejor |
| --- | --- | --- | --- | --- | --- |
| inicio | – | – | 0 | – | 12 |
| 1 | 12 | `true` | 1 | 12 > 12 → `false` | 12 |
| 2 | 7 | `false` | 1 | 7 > 12 → `false` | 12 |
| 3 | 15 | `true` | 2 | 15 > 12 → `true` | 15 |
| 4 | 9 | `false` | 2 | 9 > 15 → `false` | 15 |
| 5 | 18 | `true` | 3 | 18 > 15 → `true` | 18 |

```sortie
Aprobadas: 3
Mejor: 18
```

En la vuelta 4, se compara 9 con **15**, no con 12: `mejor` cambió en la vuelta 3.
Siempre se lee el valor de la fila de arriba.

Prueba: añade `20` al principio del array. La mejor nota ya es la primera: `mejor` no
cambia nunca, y el programa muestra `Aprobadas: 4` y luego `Mejor: 20`.

> **Recuerda:** para un bucle sobre un array, basta una fila por vuelta: el valor de la
> vuelta, la respuesta de cada prueba y luego las cajas que cambian.

## Hacer que el programa muestre sus pasos

En papel, eres tú quien lleva la traza. Pero el programa también puede escribirla él
mismo. Añade un `console.log` **con etiquetas** dentro del bucle:

```javascript
const notas = [12, 7, 15];
let aprobadas = 0;

for (const nota of notas) {
  if (nota >= 10) {
    aprobadas = aprobadas + 1;
  }
  console.log(`nota=${nota} aprobadas=${aprobadas}`);
}
```

```sortie
nota=12 aprobadas=1
nota=7 aprobadas=1
nota=15 aprobadas=2
```

Cada línea mostrada es una fila de tu tabla de traza. Compáralas: la primera diferencia
muestra dónde está el problema.

Un atajo práctico: pon las variables entre llaves. `console.log` escribe entonces el
nombre **y** el valor de cada una.

```javascript
let a = 3;
let b = 5;
console.log({ a, b });
a = a + b;
console.log({ a, b });
```

```sortie
{ a: 3, b: 5 }
{ a: 8, b: 5 }
```

Para obtener una tabla de verdad, guarda cada paso en una lista y luego usa
`console.table`:

```javascript
const pasos = [];
let total = 0;
for (let i = 1; i <= 3; i++) {
  total = total + i;
  pasos.push({ i, total });
}
console.table(pasos);
```

```sortie
┌─────────┬───┬───────┐
│ (index) │ i │ total │
├─────────┼───┼───────┤
│ 0       │ 1 │ 1     │
│ 1       │ 2 │ 3     │
│ 2       │ 3 │ 6     │
└─────────┴───┴───────┘
```

> **Recuerda:** un `console.log` con etiquetas dentro de un bucle es una tabla de traza
> escrita por el programa. Quítalo cuando encuentres el bug.

## El depurador en dos frases

Un [depurador](../debuter/glossaire.md#debogage) pone el programa en pausa en una línea
(con la instrucción `debugger;` o con un clic en el margen del editor) y te muestra el
valor de cada variable. Luego avanzas una línea cada vez: es la tabla de traza, rellenada
por el ordenador; todo se explica en [depurar](debogage.md).

## Errores frecuentes

### Olvidar la última prueba de un bucle

Crees que el bucle se para «en el momento justo», sin comprobar la última vuelta.

```javascript norun
const notas = [12, 15, 9];
let suma = 0;
for (let i = 0; i <= notas.length; i++) {
  suma = suma + notas[i];
}
console.log(suma);
```

```text
NaN
```

La traza muestra el problema. `notas.length` vale 3. La prueba `3 <= 3` es verdadera:
hay una **cuarta** vuelta, con `i = 3`. Pero `notas[3]` no existe: vale `undefined`, y
`36 + undefined` da `NaN`.

| Vuelta | i | i <= 3 ? | notas[i] | suma |
| --- | --- | --- | --- | --- |
| 1 | 0 | `true` | 12 | 12 |
| 2 | 1 | `true` | 15 | 27 |
| 3 | 2 | `true` | 9 | 36 |
| 4 | 3 | `true` | `undefined` | `NaN` |

Corrección: `i < notas.length`. Los [índices](../debuter/glossaire.md#indice) van de `0`
a `notas.length - 1`.

### Usar un valor antiguo

Quieres intercambiar `a` y `b`, y en tu cabeza lees el valor **antiguo** de `a`.

```javascript norun
let a = 3;
let b = 5;
a = b;
b = a;
console.log(a, b);
```

```text
5 5
```

Esperabas `5 3`. Pero después de `a = b;`, la caja `a` contiene 5: el 3 se ha perdido.
Así que `b = a;` copia 5. En la traza, la línea 3 escribe 5 en la columna `a`, y la línea
4 lee esa columna.

Corrección: guarda el valor antiguo en una tercera caja.

```javascript
let a = 3;
let b = 5;
const antiguaA = a;
a = b;
b = antiguaA;
console.log(a, b);
```

```sortie
5 3
```

### Usar una caja antes de crearla

En la traza, una columna que vale `–` no se puede leer. El programa tampoco lo acepta.

```javascript norun
let total = total + 5;
console.log(total);
```

```text
let total = total + 5;
            ^

ReferenceError: Cannot access 'total' before initialization
```

Corrección: da primero un valor inicial (`let total = 0;`) y luego modifícalo en la
línea siguiente (`total = total + 5;`).

### Cambiar una caja const

Tu traza muestra que `cuenta` cambia en cada vuelta. Una caja que cambia debe crearse con
`let`, no con `const`.

```javascript norun
const cuenta = 0;
for (const n of [4, 8]) {
  cuenta = cuenta + n;
}
```

```text
  cuenta = cuenta + n;
         ^

TypeError: Assignment to constant variable.
```

Corrección: `let cuenta = 0;`. Consejo: si una columna de tu tabla cambia de valor, es un
`let`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto vale `x` al final?

```javascript norun
let x = 2;
x = x * 3;
x = x + x;
```

> **Respuesta**
>
> `12`. Traza: `x` vale 2, luego 2 × 3 = 6, luego 6 + 6 = 12. La tercera línea lee 6, el
> valor más reciente.

**Pregunta 2:** ¿cuántas veces se lee la prueba `i < 3` en
`for (let i = 0; i < 3; i++)`?

> **Respuesta**
>
> 4 veces: para `i = 0`, `1` y `2` (verdadera, el bucle sigue), y luego para `i = 3`
> (falsa, el bucle se para). El bucle da 3 vueltas, pero la prueba se lee una vez más.

**Pregunta 3:** ¿qué muestra este programa?

```javascript norun
function masUno(n) {
  return n + 1;
}
const r = masUno(masUno(5));
console.log(r);
```

> **Respuesta**
>
> `7`. La llamada interior se hace primero: `masUno(5)` devuelve 6. Luego `masUno(6)`
> devuelve 7, que se guarda en `r`.

## En resumen

- Una tabla de traza tiene una columna por variable y una fila por paso.
- A la derecha del `=`, se lee el valor **más reciente**; a la izquierda, se guarda el
  resultado.
- Para un `if`, anota la prueba y su respuesta; para un bucle, anota la prueba en cada
  vuelta, incluida la última, que es falsa.
- Una llamada a una función es un viaje de ida y vuelta, con cajas nuevas en cada
  llamada.
- `console.log` con etiquetas hace que el programa escriba la traza; el depurador la
  muestra línea por línea.

Siguiente: [las trampas del principiante en JavaScript](pieges.md)
