# Números y cálculos

Esta página explica cómo calcula JavaScript: los operadores, el objeto `Math`, las sorpresas de los números decimales, `NaN`, `Infinity`, `BigInt` y cómo mostrar los números al estilo español.

> **Objetivos:**
>
> - sabrás hacer cálculos con `+ - * /`, el resto `%` y la potencia `**`;
> - sabrás redondear y usar las funciones de `Math`;
> - sabrás por qué `0.1 + 0.2` no da exactamente `0.3`;
> - sabrás convertir un texto en número y reconocer `NaN`;
> - sabrás mostrar un número en formato español con `Intl.NumberFormat`.

## Un solo tipo de número

Los programas calculan todo el tiempo: el total de una cesta de la compra, una
puntuación, la edad de una persona. Antes de calcular, veamos cómo es un número en
JavaScript.

Muchos lenguajes separan los [enteros](../debuter/glossaire.md#entero) (`3`) y los
[flotantes](../debuter/glossaire.md#flotante) (`3.5`). JavaScript, en cambio, tiene un
solo tipo para los dos: `number`.

```javascript
console.log(typeof 3);
console.log(typeof 3.5);
console.log(3 === 3.0);
console.log(Number.isInteger(5.0));
```

```sortie
number
number
true
true
```

Línea por línea:

- `typeof 3` pregunta el tipo del valor `3`: la respuesta es `number`;
- `typeof 3.5` también da `number`: el mismo tipo, con o sin decimales;
- `3 === 3.0` compara los dos valores: son iguales, así que `true`;
- `Number.isInteger(5.0)` responde `true`, porque `5.0` no tiene nada tras el punto.

`3` y `3.0` son exactamente el mismo valor. `Number.isInteger` dice si un número no
tiene parte decimal.

## Los operadores

Un programa suele hacer los cálculos por ti, como una calculadora: el precio de tres
cafés, la media de una clase. Los operadores son los signos que escriben esos
cálculos.

Los [operadores](../debuter/glossaire.md#operador) de cálculo son los del colegio, más
dos nuevos.

```javascript
console.log(7 + 2);
console.log(7 - 2);
console.log(7 * 2);
console.log(7 / 2);
console.log(7 % 2);
console.log(2 ** 10);
```

```sortie
9
5
14
3.5
1
1024
```

- `/` es una división de verdad: `7 / 2` da `3.5`, no `3`.
- `%` da el **resto** de la división entera: 7 = 2 × 3 + **1**.
- `**` es la potencia: `2 ** 10` significa 2 multiplicado por sí mismo 10 veces.

Las prioridades son las de las matemáticas: `**`, luego `*` `/` `%`, luego `+` `-`. Los
paréntesis cambian el orden.

```javascript
console.log(2 + 3 * 4);
console.log((2 + 3) * 4);
```

```sortie
14
20
```

### ¿Para qué sirve el resto?

El resto `%` responde a preguntas cotidianas: ¿un número es par? ¿Cuántos minutos
quedan después de las horas completas?

```javascript
const n = 17;
console.log(n % 2 === 0);

const minutos = 135;
console.log(`${Math.floor(minutos / 60)} h ${minutos % 60} min`);
```

```sortie
false
2 h 15 min
```

Un número es par si su resto al dividirlo por 2 vale 0.

### Los atajos

Como `+=`, que ya has visto, existen `-=`, `*=` y `/=`. Para sumar o restar 1, se
escribe `++` y `--`.

```javascript
let vidas = 3;
vidas -= 1;
vidas *= 10;
vidas++;
console.log(vidas);
```

```sortie
21
```

**Te toca:** escribe un programa que reparta 23 caramelos entre 5 amigos. Muestra
cuántos caramelos recibe cada uno y luego cuántos sobran.

> **Solución**
>
> ```javascript
> const caramelos = 23;
> const amigos = 5;
> console.log(Math.floor(caramelos / amigos));
> console.log(caramelos % amigos);
> ```
>
> ```sortie
> 4
> 3
> ```

## El objeto Math

Algunos cálculos se repiten sin parar: redondear un precio, encontrar la mejor nota de
una lista. En lugar de escribirlos tú, los tomas ya hechos.

`Math` es una caja de herramientas que viene con JavaScript. Contiene funciones de
cálculo y algunas constantes.

```javascript
console.log(Math.round(2.6));
console.log(Math.floor(2.6));
console.log(Math.ceil(2.1));
console.log(Math.trunc(-2.6));
console.log(Math.abs(-4));
console.log(Math.sqrt(16));
console.log(Math.max(3, 8, 1));
console.log(Math.min(3, 8, 1));
console.log(Math.PI);
```

```sortie
3
2
3
-2
4
4
8
1
3.141592653589793
```

| Función | Papel |
| --- | --- |
| `Math.round(x)` | redondea al más cercano |
| `Math.floor(x)` | redondea hacia abajo (el «suelo») |
| `Math.ceil(x)` | redondea hacia arriba (el «techo») |
| `Math.trunc(x)` | corta la parte decimal |
| `Math.abs(x)` | valor absoluto (quita el signo menos) |
| `Math.sqrt(x)` | raíz cuadrada |
| `Math.max(...)`, `Math.min(...)` | el mayor, el menor |

`Math.round` redondea al entero. Para conservar dos cifras decimales, un truco:
multiplica por 100, redondea y luego divide por 100.

```javascript
console.log(Math.round(3.456 * 100) / 100);
```

```sortie
3.46
```

### Un número al azar

`Math.random()` da un número al azar entre 0 (incluido) y 1 (no incluido). Para un dado
de seis caras, se estira y luego se redondea hacia abajo:

```javascript norun
const dado = Math.floor(Math.random() * 6) + 1;
console.log(dado);
```

```text
4
```

El resultado cambia en cada ejecución: el tuyo seguramente será distinto.

**Te toca:** escribe un programa que muestre la mejor de las tres notas 12, 17 y
9, y luego su media redondeada al entero más cercano.

> **Solución**
>
> ```javascript
> console.log(Math.max(12, 17, 9));
> console.log(Math.round((12 + 17 + 9) / 3));
> ```
>
> ```sortie
> 17
> 13
> ```

## Los números decimales no son exactos

Esta parte te evita una trampa clásica, por ejemplo el total de una compra con unas
quince cifras decimales. Mejor conocerla antes de encontrártela.

Aquí tienes una sorpresa famosa:

```javascript
console.log(0.1 + 0.2);
console.log(0.1 + 0.2 === 0.3);
```

```sortie
0.30000000000000004
false
```

El ordenador guarda los números en binario (con 0 y 1). Algunos números decimales
sencillos, como 0,1, no se pueden escribir exactamente en binario, igual que 1/3 no se
puede escribir exactamente en decimal (0,3333…). Queda un error muy pequeño.

No es un fallo de JavaScript: Python, Ruby, Java… hacen lo mismo. Bastan dos reglas:

- para **mostrar**, redondea (mira `toFixed` más abajo);
- para **comparar**, comprueba que la diferencia sea minúscula.

```javascript
const suma = 0.1 + 0.2;
console.log(Math.abs(suma - 0.3) < Number.EPSILON);
```

```sortie
true
```

`Number.EPSILON` es un número minúsculo, pensado para este tipo de comparación.

> **Consejo:** para el dinero, cuenta en **céntimos** con enteros: `1999` en lugar de
> `19.99`. Los enteros sí son exactos.

## Números y texto

Los números y los textos pasan a menudo de uno a otro. Lo que se teclea llega como
texto, incluso «42». Y para mostrar un precio bien, se convierte un número en texto.
Esta parte tiende el puente en los dos sentidos.

### Mostrar con toFixed

`toFixed(n)` devuelve un **texto** con exactamente `n` cifras decimales.

```javascript
const precio = 0.1 + 0.2;
console.log(precio.toFixed(2));
console.log((10).toFixed(2));
console.log(typeof precio.toFixed(2));
```

```sortie
0.30
10.00
string
```

Línea por línea:

- `precio` recibe `0.1 + 0.2`, es decir, `0.30000000000000004`;
- `precio.toFixed(2)` conserva dos cifras decimales y redondea: `0.30`;
- `(10).toFixed(2)` añade los ceros que faltan: `10.00`;
- `typeof` muestra que el resultado de `toFixed` es un `string`, es decir, un texto.

El resultado es una cadena: perfecto para mostrar, no para seguir calculando.

> **Nota:** `(10).toFixed(2)` necesita paréntesis. Sin ellos, `10.` se leería como el
> principio de un número decimal.

### Convertir un texto en número

Un texto que contiene cifras no es un número. Tres herramientas lo convierten:

```javascript
console.log(Number("42"));
console.log(Number("3.5"));
console.log(Number("12px"));
console.log(parseInt("12px"));
console.log(parseInt("3.9"));
console.log(parseFloat("3.5kg"));
```

```sortie
42
3.5
NaN
12
3
3.5
```

- `Number(texto)` es estricto: todo el texto debe ser un número.
- `parseInt(texto)` lee un entero al principio del texto y se detiene en el primer
  carácter que no es una cifra.
- `parseFloat(texto)` hace lo mismo, pero conserva la parte decimal.

> **Recuerda:** prefiere `Number(...)`. Rechaza los textos dudosos, en lugar de tomar
> un trozo en silencio.

**Te toca:** escribe un programa que convierta el texto `"19.99"` en número, le
sume 5 y luego muestre el resultado con dos cifras decimales.

> **Solución**
>
> ```javascript
> const texto = "19.99";
> const total = Number(texto) + 5;
> console.log(total.toFixed(2));
> ```
>
> ```sortie
> 24.99
> ```

## NaN: «no es un número»

Alguien puede teclear «treinta» en lugar de «30». Tu programa debe entonces saber
detectar que la conversión ha fallado.

Cuando un cálculo no tiene sentido, JavaScript no se detiene: devuelve `NaN` (*Not a
Number*, «no es un número»). Y `NaN` contamina todos los cálculos que lo usan.

```javascript
const edad = Number("treinta");
console.log(edad);
console.log(edad + 1);
console.log(typeof edad);
console.log(edad === NaN);
console.log(Number.isNaN(edad));
```

```sortie
NaN
NaN
number
false
true
```

Línea por línea:

- `Number("treinta")` no sabe leer esta palabra como número: `edad` vale `NaN`;
- `edad + 1` sigue dando `NaN`: un cálculo con `NaN` siempre da `NaN`;
- `typeof edad` responde `number`, lo que sorprende (mira más abajo);
- `edad === NaN` responde `false`, mientras que `Number.isNaN(edad)` sí responde `true`.

Dos rarezas:

- `typeof NaN` responde `"number"`: `NaN` es un valor especial del tipo número;
- `NaN` no es igual a nada, **ni siquiera a sí mismo**: `edad === NaN` siempre es falso.

Para comprobar `NaN`, usa `Number.isNaN(x)`. También existe un viejo `isNaN(x)`, que
primero convierte su argumento en número: `isNaN("abc")` responde `true` aunque `"abc"`
es un texto, no `NaN`.

```javascript
console.log(isNaN("abc"));
console.log(Number.isNaN("abc"));
```

```sortie
true
false
```

## Infinity

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Dividir por cero no provoca ningún error en JavaScript: se obtiene el infinito.

```javascript
console.log(1 / 0);
console.log(-1 / 0);
console.log(0 / 0);
console.log(Number.isFinite(1 / 0));
```

```sortie
Infinity
-Infinity
NaN
false
```

`Number.isFinite(x)` comprueba que un número es un número de verdad: ni `Infinity` ni
`NaN`.

## Los enteros muy grandes: BigInt

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Un `number` mantiene los enteros exactos hasta unos 9000 billones. Más allá, pierde
precisión.

```javascript
console.log(Number.MAX_SAFE_INTEGER);
console.log(2 ** 53);
console.log(2 ** 53 + 1);
```

```sortie
9007199254740991
9007199254740992
9007199254740992
```

`2 ** 53 + 1` debería terminar en 3. Para los enteros gigantes, añade una `n` al final:
es un `BigInt`, exacto sea cual sea su tamaño.

```javascript
console.log(2n ** 53n + 1n);
console.log(2n ** 64n);
console.log(7n / 2n);
console.log(BigInt(10) + 5n);
```

```sortie
9007199254740993n
18446744073709551616n
3n
15n
```

La división de dos `BigInt` corta la parte decimal: `7n / 2n` da `3n`.

## Mostrar al estilo español

En España se escribe `1.234.567,89`: un punto entre los millares y una coma antes de los
decimales. `Intl.NumberFormat` da formato a un número según un idioma.

```javascript
const es = new Intl.NumberFormat("es-ES");
console.log(es.format(1234567.891));

const euros = new Intl.NumberFormat("es-ES", { style: "currency", currency: "EUR" });
console.log(euros.format(12345.5));

const porcentaje = new Intl.NumberFormat("es-ES", { style: "percent" });
console.log(porcentaje.format(0.25));
```

```sortie
1.234.567,891
12.345,50 €
25 %
```

- `"es-ES"` significa «español de España».
- `style: "currency"` muestra una cantidad de dinero, con la moneda `EUR`.
- `style: "percent"` multiplica por 100 y añade `%`.

> **Atención:** el espacio antes de `€` y `%` no es un espacio normal: es un **espacio
> de no separación** (código Unicode U+00A0). Impide un salto de línea en medio de un
> número. Consecuencia: `euros.format(12345.5) === "12.345,50 €"` (tecleado) responde
> `false`. Además, según las normas del español, un número de cuatro cifras no lleva
> punto: `euros.format(1234.5)` da `1234,50 €`.

**Te toca:** escribe un programa que muestre el precio 12500 en euros, en formato
español.

> **Solución**
>
> ```javascript
> const euros = new Intl.NumberFormat("es-ES", { style: "currency", currency: "EUR" });
> console.log(euros.format(12500));
> ```
>
> ```sortie
> 12.500,00 €
> ```

## Errores frecuentes

### Sumar un texto y un número

```javascript norun
const cantidad = "5";
console.log(cantidad + 3);
```

```text
53
```

No hay mensaje de error, pero el resultado es incorrecto. Con un texto, `+` **pega** en
lugar de sumar: `"5" + 3` da el texto `"53"`. Convierte primero:
`Number(cantidad) + 3` da `8`.

### Llamar a toFixed sobre un texto

```javascript norun
const n = "3.14159";
console.log(n.toFixed(2));
```

```text
file:///home/alice/js/main.mjs:2
console.log(n.toFixed(2));
              ^

TypeError: n.toFixed is not a function
    at file:///home/alice/js/main.mjs:2:15

Node.js v22.22.0
```

`toFixed` existe para los números, no para los textos (*is not a function*, «no es una
función»). Convierte: `Number(n).toFixed(2)`.

### Mezclar BigInt y number

```javascript norun
const grande = 10n;
console.log(grande + 1);
```

```text
file:///home/alice/js/main.mjs:2
console.log(grande + 1);
                   ^

TypeError: Cannot mix BigInt and other types, use explicit conversions
    at file:///home/alice/js/main.mjs:2:20

Node.js v22.22.0
```

No se mezclan un `BigInt` y un `number` en un cálculo. Escribe `grande + 1n`, o
convierte: `Number(grande) + 1`.

### Comparar con NaN

```javascript norun
const edad = Number("abc");
if (edad === NaN) {
  console.log("Edad no válida");
}
```

```text
(no se muestra nada)
```

`NaN` no es igual a nada, ni siquiera a sí mismo: la condición siempre es falsa.
Escribe `if (Number.isNaN(edad))`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto valen `17 % 5` y `2 ** 3`?

> **Respuesta**
>
> ```javascript
> console.log(17 % 5);
> console.log(2 ** 3);
> ```
>
> ```sortie
> 2
> 8
> ```
>
> 17 = 5 × 3 + **2**, y 2 × 2 × 2 = 8.

**Pregunta 2:** ¿por qué `0.1 + 0.2 === 0.3` responde `false`?

> **Respuesta**
>
> Porque 0,1 y 0,2 no se pueden escribir exactamente en binario: la suma vale
> `0.30000000000000004`. Redondea para mostrar, y compara la diferencia con
> `Number.EPSILON`.

**Pregunta 3:** ¿cómo se muestra el precio `19.9` en la forma `19.90`?

> **Respuesta**
>
> ```javascript
> console.log((19.9).toFixed(2));
> ```
>
> ```sortie
> 19.90
> ```
>
> Para el formato español (`19,90 €`), usa `Intl.NumberFormat("es-ES", ...)`.

## En resumen

- JavaScript tiene un solo tipo `number` para los enteros y los números decimales.
- `%` da el resto, `**` la potencia; `Math` contiene `round`, `floor`, `ceil`, `max`,
  `min`…
- Los números decimales son aproximados: redondea con `toFixed` para mostrarlos.
- `Number("...")` convierte un texto; un cálculo imposible da `NaN`, que se comprueba
  con `Number.isNaN`.
- Dividir por cero da `Infinity`; los enteros gigantes se escriben como `BigInt` (`10n`).
- `Intl.NumberFormat("es-ES")` muestra los números al estilo español.

Siguiente: [condiciones y bucles](conditions-boucles.md).
