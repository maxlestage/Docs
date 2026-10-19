# Números y cálculos

Esta página muestra cómo calcula TypeScript: el tipo `number`, los operadores, el resto `%`, los redondeos, la forma de mostrar un número, la conversión de un texto en número y las pequeñas sorpresas de los números decimales.

> **Objetivos:**
>
> - sabrás que un solo tipo, `number`, sirve para los enteros y para los números decimales;
> - sabrás calcular con `+ - * / ** %` y modificar una variable con `+=`;
> - sabrás redondear con `Math.round`, `Math.floor`, `Math.ceil` y `Math.trunc`;
> - sabrás mostrar un número con `toFixed` e `Intl.NumberFormat`;
> - sabrás transformar un texto en número y detectar `NaN`;
> - sabrás por qué `0.1 + 0.2` no da exactamente `0.3`.

## Un solo tipo para todos los números

Un programa calcula sin parar: un precio, una edad, una puntuación, una distancia. Antes
de calcular, hay que saber cómo guarda TypeScript esos números.

En muchos lenguajes hay un tipo para los [enteros](../debuter/glossaire.md#entero)
(`7`) y otro para los números decimales, los [flotantes](../debuter/glossaire.md#flotante)
(`7.5`). En TypeScript, no: los dos son `number`.

```typescript
const manzanas: number = 7;
const precio: number = 2.5;
const temperatura: number = -3;

console.log(manzanas, precio, temperatura);
console.log(typeof manzanas, typeof precio);
console.log(Number.isInteger(manzanas), Number.isInteger(precio));
```

```sortie
7 2.5 -3
number number
true false
```

Línea por línea:

- los tres `const` guardan tres números: un entero, un número decimal y un número
  negativo. Los tres tienen el tipo `number`;
- el primer `console.log` muestra los tres valores, separados por un espacio;
- `typeof` da el tipo de un valor: aquí, `number` para los dos;
- `Number.isInteger` responde `true` para `7` y `false` para `2.5`.

`Number.isInteger` responde a la pregunta «¿es un entero?».

> **Atención:** en el código, la coma decimal se escribe con un **punto**: `2.5`, nunca
> `2,5`.

## Los operadores de cálculo

Calcular es el trabajo básico de un ordenador. Los operadores son las teclas de su
calculadora: los combinas para obtener un total, una media, un precio.

Un [operador](../debuter/glossaire.md#operador) es un símbolo que calcula. Estos son
los de los números:

| Operador | Función | Ejemplo | Resultado |
|---|---|---|---|
| `+` | suma | `7 + 2` | `9` |
| `-` | resta | `7 - 2` | `5` |
| `*` | multiplicación | `7 * 2` | `14` |
| `/` | división | `7 / 2` | `3.5` |
| `**` | potencia | `2 ** 3` | `8` |
| `%` | resto de la división | `7 % 2` | `1` |

```typescript
console.log(7 + 2, 7 - 2, 7 * 2);
console.log(7 / 2);
console.log(2 ** 3);
console.log(1 / 0, 0 / 0);
```

```sortie
9 5 14
3.5
8
Infinity NaN
```

Línea por línea:

- la primera línea hace tres cálculos y muestra los tres resultados uno al lado del otro;
- `7 / 2` da `3.5`: la división conserva la parte decimal;
- `2 ** 3` quiere decir 2 × 2 × 2, es decir, `8`;
- la última línea muestra dos casos raros, explicados justo debajo.

- `/` hace una división de verdad: `7 / 2` da `3.5`, no `3`.
- Dividir entre cero no hace fallar el programa: obtienes `Infinity` (el infinito).
- `0 / 0` da `NaN`: «*Not a Number*», no es un número. Volveremos a ello.

### El orden de los cálculos

Como en el colegio, `*` y `/` van antes que `+` y `-`. Los paréntesis van antes que
todo.

```typescript
console.log(2 + 3 * 4);
console.log((2 + 3) * 4);
console.log(10 - 4 - 3);
```

```sortie
14
20
3
```

```text
2 + 3 * 4        (2 + 3) * 4
    -----         -------
2 +  12            5    * 4
   14                 20
```

> **Pruébalo:** cambia `10 - 4 - 3` por `10 - (4 - 3)`. El resultado pasa a ser `9`: el
> paréntesis se calcula primero.

## El resto: %

`%` da lo que **sobra** después de una división entera. Piensa en huevos guardados en
cajas de 6: con 14 huevos, llenas 2 cajas y sobran 2 huevos.

```text
14 huevos, cajas de 6:

[oooooo] [oooooo]  oo
  caja 1   caja 2  resto = 14 % 6 = 2
```

```typescript
console.log(14 % 6);
console.log(10 % 2, 11 % 2);
console.log(Math.floor(14 / 6));
```

```sortie
2
0 1
2
```

Línea por línea:

- `14 % 6`: en 14 cabe 2 veces 6 (es decir, 12), y sobra `2`;
- `10 % 2` da `0` (10 es par) y `11 % 2` da `1` (11 es impar);
- `Math.floor(14 / 6)` da el número de cajas llenas: `14 / 6` vale `2.33…`, y
  `Math.floor` conserva la parte entera, `2`. Volverás a ver `Math.floor` más abajo.

Dos usos muy habituales:

- **par o impar**: `n % 2` vale `0` si `n` es par, `1` si no;
- **dividir una duración**: 135 minutos son 2 horas y 15 minutos.

```typescript
const minutos = 135;
const horas = Math.floor(minutos / 60);
const resto = minutos % 60;
console.log(`${horas} h ${resto} min`);
```

```sortie
2 h 15 min
```

| Paso | Cálculo | Valor |
|---|---|---|
| 1 | `minutos / 60` | `2.25` |
| 2 | `Math.floor(2.25)` | `2` → `horas` |
| 3 | `135 % 60` | `15` → `resto` |

> **Pruébalo:** pon `minutos = 59`. Obtienes `0 h 59 min`.

> **Recuerda:** `/` divide, `%` da el resto. Juntos, dividen un número en trozos (horas
> y minutos, cajas y huevos sobrantes).

**Te toca:** escribe un programa que divida 200 segundos en minutos y segundos, y
muestre `3 min 20 s`.

> **Solución**
>
> ```typescript
> const segundos = 200;
> const min = Math.floor(segundos / 60);
> const resto = segundos % 60;
> console.log(`${min} min ${resto} s`);
> ```
>
> ```sortie
> 3 min 20 s
> ```

## Modificar una variable que contiene un número

Muchos números cambian durante el programa: una puntuación que sube, un stock que baja,
el saldo de una hucha. Actualizas la misma variable, en lugar de crear una nueva cada
vez.

Para sumar algo a una variable, puedes escribir `total = total + 5`. Existen atajos:

| Atajo | Quiere decir |
|---|---|
| `total += 5` | `total = total + 5` |
| `total -= 2` | `total = total - 2` |
| `total *= 3` | `total = total * 3` |
| `total++` | `total = total + 1` |
| `total--` | `total = total - 1` |

```typescript
let total = 10;
total += 5;
total -= 2;
total *= 3;
total++;
console.log(total);
```

```sortie
40
```

| Paso | Instrucción | total |
|---|---|---|
| 1 | `let total = 10` | 10 |
| 2 | `total += 5` | 15 |
| 3 | `total -= 2` | 13 |
| 4 | `total *= 3` | 39 |
| 5 | `total++` | 40 |

Estos atajos solo funcionan con `let`: una `const` no cambia nunca.

## Redondear con Math

Un cálculo da a menudo un número lleno de decimales, como `2.4` autobuses o `3.333…`
euros. En la vida real hace falta un número redondo: para eso sirve redondear.

`Math` es una caja de herramientas siempre disponible. Cuatro herramientas redondean:

- `Math.round`: al más cercano;
- `Math.floor`: hacia abajo (el «suelo»);
- `Math.ceil`: hacia arriba (el «techo»);
- `Math.trunc`: corta todo lo que va después de la coma.

```typescript
for (const x of [2.4, 2.5, 2.6, -2.5, -2.6]) {
  console.log(x, Math.round(x), Math.floor(x), Math.ceil(x), Math.trunc(x));
}
```

```sortie
2.4 2 2 3 2
2.5 3 2 3 2
2.6 3 2 3 2
-2.5 -2 -3 -2 -2
-2.6 -3 -3 -2 -2
```

Línea por línea:

- `for (const x of [...])` toma los números de la lista uno a uno, y los guarda por
  turnos en `x`. Los bucles se explican en el
  [capítulo siguiente](conditions-boucles.md);
- en cada vuelta, `console.log` muestra `x` y luego sus cuatro redondeos, en este orden:
  `round`, `floor`, `ceil`, `trunc`;
- así que cada línea de la salida corresponde a un número de la lista.

Fíjate en los números negativos: `Math.floor(-2.6)` da `-3`, porque `-3` es **más
bajo** que `-2.6`. `Math.trunc` simplemente corta: `-2`.

Un uso concreto de `Math.ceil`: ¿cuántos autobuses de 50 plazas hacen falta para 120
alumnos?

```typescript
const alumnos = 120;
console.log(alumnos / 50);
console.log(Math.ceil(alumnos / 50));
```

```sortie
2.4
3
```

Hacen falta 3 autobuses: no se pueden pedir 2,4 autobuses.

**Te toca:** las botellas se guardan en cajas de 12. Escribe un programa que muestre
cuántas cajas hacen falta para 30 botellas.

> **Solución**
>
> ```typescript
> const botellas = 30;
> console.log(Math.ceil(botellas / 12));
> ```
>
> ```sortie
> 3
> ```

### Más pequeño, más grande, distancia

```typescript
console.log(Math.min(4, 9, 2));
console.log(Math.max(4, 9, 2));
console.log(Math.abs(-7), Math.abs(7));
console.log(Math.sqrt(16));
```

```sortie
2
9
7 7
4
```

`Math.abs` da el valor sin el signo: la «distancia» a cero. `Math.sqrt` da la raíz
cuadrada.

> **Pruébalo:** para una nota entre 0 y 20, escribe `Math.min(20, Math.max(0, nota))`.
> Con `nota = 23`, obtienes `20`; con `nota = -1`, obtienes `0`.

## Mostrar un número de forma bonita

Un cálculo puede dar `3.3333333333333335`. Nadie quiere leer eso en un tique de compra.
Estas herramientas eligen cómo se muestra un número, sin cambiar el cálculo.

### toFixed: un número fijo de decimales

`toFixed(2)` da el número con 2 cifras después de la coma.

```typescript
const precio = 19.9;
console.log(precio.toFixed(2));
console.log((10 / 3).toFixed(2));
console.log((2.5).toFixed(0));
console.log(typeof precio.toFixed(2));
```

```sortie
19.90
3.33
3
string
```

Línea por línea:

- `precio` vale `19.9`. `precio.toFixed(2)` lo escribe con dos decimales: `19.90`;
- `10 / 3` vale `3.333…`, y `toFixed(2)` conserva `3.33`;
- `toFixed(0)` no conserva ningún decimal: `2.5` se convierte en `3`;
- `typeof` muestra que el resultado de `toFixed` es un `string`, un texto.

> **Atención:** `toFixed` devuelve un **texto** (`string`), no un número. Úsalo al
> final del todo, solo para mostrar. No hagas más cálculos con él.

### Intl.NumberFormat: a la española

En España se escribe `1.234.567,89`: un punto entre los miles y una coma para los
decimales. `Intl.NumberFormat` conoce estas costumbres. Se presenta en detalle en
[el capítulo sobre el texto](texte.md#formatear-numeros-con-intl).

```typescript
const euros = new Intl.NumberFormat("es-ES", { style: "currency", currency: "EUR" });
const numero = new Intl.NumberFormat("es-ES", { maximumFractionDigits: 2 });

console.log(euros.format(1234.5));
console.log(numero.format(1234567.891));
console.log(numero.format(0.5));
```

```sortie
1234,50 €
1.234.567,89
0,5
```

Fíjate en la primera línea: en España, un número de solo cuatro cifras no lleva punto
de miles (`1234,50`). El punto aparece a partir de cinco cifras.

> **Nota:** el espacio antes de `€` no es un espacio corriente. Es un espacio
> **de no separación**: impide cortar el importe al final de una línea. Así que nunca
> compares este texto con un texto escrito con el teclado.

## Del texto al número

Un texto que contiene cifras sigue siendo un texto: `"42"` no es `42`. Cuando un valor
viene de un formulario, de un archivo o del teclado, es **siempre** un texto. Tres
herramientas lo transforman en número.

```typescript
console.log(Number("42"), parseInt("42"), parseFloat("42"));
console.log(Number("3.9"), parseInt("3.9"), parseFloat("3.9"));
console.log(Number("12px"), parseInt("12px"), parseFloat("12px"));
console.log(Number(""), parseInt(""), parseFloat(""));
console.log(Number(" 7 "), Number("3,5"));
```

```sortie
42 42 42
3.9 3 3.9
NaN 12 12
0 NaN NaN
7 NaN
```

`Number(" 7 ")` ignora los espacios de alrededor. `Number("3,5")` falla: en el código,
la coma decimal es un punto.

| Herramienta | Cómo lee | `"3.9"` | `"12px"` | `""` |
|---|---|---|---|---|
| `Number` | todo el texto, o nada | `3.9` | `NaN` | `0` |
| `parseInt` | el principio, hasta el primer «no dígito», sin decimales | `3` | `12` | `NaN` |
| `parseFloat` | el principio, con los decimales | `3.9` | `12` | `NaN` |

> **Recuerda:** usa `Number` por defecto: rechaza los textos que solo son números a
> medias. Cuidado con su trampa: un texto vacío da `0`, no `NaN`.

**Te toca:** dos campos de un formulario contienen los textos `"12"` y `"30"`. Escribe
un programa que los transforme en números y muestre su suma, `42`.

> **Solución**
>
> ```typescript
> const campo1 = "12";
> const campo2 = "30";
> console.log(Number(campo1) + Number(campo2));
> ```
>
> ```sortie
> 42
> ```

### NaN: «no es un número»

Cuando la conversión falla, obtienes `NaN`. Curiosamente, `NaN` es de tipo `number`. Y
no es igual a nada, ni siquiera a sí mismo. Para detectarlo, usa `Number.isNaN`:

```typescript
for (const entrada of ["12", "doce", "4.5"]) {
  const n = Number(entrada);
  if (Number.isNaN(n)) {
    console.log(`"${entrada}" no es un número`);
  } else {
    console.log(`${n} x 2 = ${n * 2}`);
  }
}
```

```sortie
12 x 2 = 24
"doce" no es un número
4.5 x 2 = 9
```

`NaN` es contagioso: `NaN + 1` da `NaN`. Basta un solo `NaN` al principio de un cálculo
para que todo el resultado sea `NaN`. Así que comprueba tus conversiones enseguida.

## 0.1 + 0.2: la sorpresa de los decimales

En cuanto calculas un precio o una media, usas números decimales. Esconden una pequeña
trampa: mejor conocerla antes de que te sorprenda.

```typescript
console.log(0.1 + 0.2);
console.log(0.1 + 0.2 === 0.3);
```

```sortie
0.30000000000000004
false
```

No es un error de TypeScript. El ordenador guarda los números en
[binario](../debuter/glossaire.md#binario) (con 0 y 1). Pero `0.1` no se puede escribir
exactamente en binario, igual que `1/3` no se puede escribir exactamente en decimal
(`0.3333…`). Queda un error minúsculo, que a veces aparece. Python, Java o JavaScript
hacen exactamente lo mismo.

Dos soluciones:

1. **redondear para mostrar**, con `toFixed` o `Intl`;
2. **contar en céntimos**, con enteros, para el dinero.

```typescript
const total = 0.1 + 0.2;
console.log(total.toFixed(2));
console.log(Math.round(total * 100) / 100);

const precioEnCentimos = 10 + 20;
console.log(precioEnCentimos / 100);
```

```sortie
0.30
0.3
0.3
```

Línea por línea:

- `total` vale `0.30000000000000004`, con el pequeño error;
- `total.toFixed(2)` lo muestra con dos decimales: `0.30`;
- `Math.round(total * 100) / 100` redondea al céntimo: `30.000…04` se convierte en
  `30` y, dividido entre 100, en `0.3`;
- `precioEnCentimos` suma dos enteros, `10 + 20`, sin ningún error. Solo se divide
  entre 100 para mostrar.

> **Recuerda:** no compares nunca dos números decimales con `===`. Para el dinero,
> calcula en céntimos (enteros) y divide entre 100 solo para mostrar.

## Los enteros muy grandes: bigint

> **Nota:** esta parte puede esperar. Léela cuando tengas las bases claras.

Un `number` es exacto hasta `Number.MAX_SAFE_INTEGER`, unos 9000 billones. Más allá, se
equivoca.

```typescript
console.log(Number.MAX_SAFE_INTEGER);
console.log(9007199254740992 + 1);
```

```sortie
9007199254740991
9007199254740992
```

¡`+ 1` no ha cambiado nada! Para enteros más grandes existe otro tipo: `bigint`. Se
escribe con una `n` al final.

```typescript
const grande: bigint = 9007199254740992n;
console.log(grande + 1n);
console.log(2n ** 100n);
console.log(typeof grande);
```

```sortie
9007199254740993n
1267650600228229401496703205376n
bigint
```

Un `bigint` no tiene decimales: `7n / 2n` da `3n`. Rara vez lo necesitarás al empezar.

## Los tipos literales de números

> **Nota:** esta parte puede esperar. Léela cuando tengas las bases claras.

Un tipo también puede ser **un número concreto**: `type Cara = 1 | 2 | 3 | 4 | 5 | 6;`
solo acepta esos seis valores, y `const c: Cara = 7;` se rechaza. Verás esta idea en
detalle en [los tipos literales](litteraux.md).

## Errores frecuentes

### Calcular con el resultado de toFixed

```typescript norun
const precio = 19.9;
const redondeado: number = precio.toFixed(2);
```

```text
main.ts:2:7 - error TS2322: Type 'string' is not assignable to type 'number'.

2 const redondeado: number = precio.toFixed(2);
        ~~~~~~~~~~


Found 1 error in main.ts:2
```

`toFixed` devuelve un texto. Para un número redondeado, usa
`Math.round(precio * 100) / 100`. Deja `toFixed` para mostrar.

### Comprobar NaN con ===

```typescript norun
const n = Number("doce");
if (n === NaN) {
  console.log("no es un número");
}
```

```text
main.ts:2:5 - error TS2845: This condition will always return 'false'.

2 if (n === NaN) {
      ~~~~~~~~~

  main.ts:2:5
    2 if (n === NaN) {
          ~
    Did you mean 'Number.isNaN(n)'?


Found 1 error in main.ts:2
```

«Esta condición siempre devolverá `false`». `NaN` no es igual a nada. TypeScript
propone la corrección: `if (Number.isNaN(n))`.

### Dar dos argumentos a Math.round

```typescript norun
console.log(Math.round(3.14159, 2));
```

```text
main.ts:1:33 - error TS2554: Expected 1 arguments, but got 2.

1 console.log(Math.round(3.14159, 2));
                                  ~


Found 1 error in main.ts:1
```

`Math.round` siempre redondea al entero. Para 2 decimales:
`Math.round(3.14159 * 100) / 100` da `3.14`.

### Mezclar bigint y number

```typescript norun
const grande = 10n;
console.log(grande + 1);
```

```text
main.ts:2:13 - error TS2365: Operator '+' cannot be applied to types '10n' and '1'.

2 console.log(grande + 1);
              ~~~~~~~~~~


Found 1 error in main.ts:2
```

Un `bigint` no se mezcla con un `number`. Escribe `grande + 1n`, o convierte con
`BigInt(1)`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto valen `17 % 5` y `Math.floor(17 / 5)`?

> **Respuesta**
>
> `2` y `3`. En 17 cabe 3 veces 5 (es decir, 15), y sobran 2.

**Pregunta 2:** ¿qué muestra `console.log(Number("8 kg"), parseInt("8 kg"));`?

> **Respuesta**
>
> `NaN 8`. `Number` lee todo el texto: `" kg"` lo hace fallar. `parseInt` lee el
> principio, `8`, y se detiene en el primer carácter que no es una cifra.

**Pregunta 3:** ¿por qué contar un precio en céntimos en vez de en euros?

> **Respuesta**
>
> Los céntimos son enteros, y los enteros son exactos. Los números decimales tienen
> errores minúsculos: `0.1 + 0.2` da `0.30000000000000004`.

## En resumen

- `number` sirve para todos los números, enteros o decimales; la coma se escribe `.`.
- `/` divide de verdad, `%` da el resto, `**` la potencia; `+=` y `++` modifican una
  variable `let`.
- `Math.round`, `Math.floor`, `Math.ceil` y `Math.trunc` redondean cada uno a su
  manera.
- `toFixed` e `Intl.NumberFormat` sirven para mostrar: devuelven texto.
- `Number(texto)` convierte; un fallo da `NaN`, que se comprueba con `Number.isNaN`.
- `0.1 + 0.2` no es exactamente `0.3`: redondea para mostrar, cuenta el dinero en
  céntimos.

Siguiente: [condiciones y bucles](conditions-boucles.md).
