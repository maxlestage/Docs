# Las condiciones

Esta página muestra cómo tomar decisiones con `if`, `switch` y el operador ternario, y cómo combinar comparaciones.

> **Objetivos:**
>
> - sabrás escribir una condición con `if`, `else if` y `else`;
> - sabrás combinar comparaciones con `&&`, `||` y `!`;
> - sabrás qué son los valores «truthy» y «falsy», y usar `??` y `?.`.

## La condición if

Una [condición](../debuter/glossaire.md#condicion) permite al programa elegir. Es
un cambio de agujas: según la respuesta a una pregunta, el tren sale por un lado o por el otro.

```javascript
const temperatura = 28;

if (temperatura > 25) {
  console.log("Hace calor.");
}
console.log("Fin del programa.");
```

```sortie
Hace calor.
Fin del programa.
```

Línea por línea:

- `const temperatura = 28;` guarda 28 en la constante `temperatura`;
- `if (temperatura > 25)` hace una pregunta: «¿28 es mayor que 25?». La respuesta es
  sí;
- por eso se ejecuta el `console.log` que está entre las llaves;
- el último `console.log` está fuera de las llaves: se ejecuta en todos los casos,
  aunque hiciera frío.

- La pregunta va entre paréntesis `( )`.
- El código que se ejecuta si la respuesta es verdadera va entre llaves `{ }`. Esto se
  llama un **bloque**.
- El bloque está desplazado dos espacios: es la **sangría**. JavaScript no la exige,
  pero hace que el código se lea bien.

### else y else if

A menudo no hay un solo caso. Una nota puede ser muy buena, regular o mala, y quieres
un mensaje distinto para cada una, como en un boletín de notas.

`else` («si no») da el camino que seguir cuando la condición es falsa. `else if`
añade otras preguntas, que se hacen en orden.

```javascript
const nota = 7;

if (nota >= 9) {
  console.log("Sobresaliente");
} else if (nota >= 7) {
  console.log("Notable");
} else if (nota >= 5) {
  console.log("Aprobado");
} else {
  console.log("Suspenso");
}
```

```sortie
Notable
```

En cuanto una pregunta responde verdadero, su bloque se ejecuta y las siguientes se ignoran.
Aquí, `nota >= 5` también es verdadero, pero nunca se comprueba.

**Te toca:** escribe un programa que guarde `15` en una constante `hora` y luego
muestre «Buenos días» si la hora es antes de las 12, «Buenas tardes» si es antes de
las 20, y «Buenas noches» si no.

> **Solución**
>
> ```javascript
> const hora = 15;
>
> if (hora < 12) {
>   console.log("Buenos días");
> } else if (hora < 20) {
>   console.log("Buenas tardes");
> } else {
>   console.log("Buenas noches");
> }
> ```
>
> ```sortie
> Buenas tardes
> ```

## Los operadores de comparación

Para decidir, un programa primero hace una pregunta: «¿la edad es 18?», «¿la
temperatura pasa de 25?». Los operadores de comparación sirven para escribir esas
preguntas.

Una comparación responde con un [booleano](../debuter/glossaire.md#booleano): `true` o
`false`.

| Operador | Significado | Ejemplo verdadero |
| --- | --- | --- |
| `===` | igual a | `3 === 3` |
| `!==` | distinto de | `3 !== 4` |
| `<`, `>` | menor, mayor | `2 < 5` |
| `<=`, `>=` | menor o igual, mayor o igual | `5 >= 5` |

```javascript
const edad = 17;
console.log(edad >= 18);
console.log(edad !== 18);
console.log("abc" < "abd");
```

```sortie
false
true
true
```

Línea por línea:

- `edad` vale 17;
- `edad >= 18` pregunta «¿17 es mayor o igual que 18?». No: `false`;
- `edad !== 18` pregunta «¿17 es distinto de 18?». Sí: `true`;
- `"abc" < "abd"`: los dos textos empiezan por «ab», y la «c» va antes que la «d» en
  el alfabeto: `true`.

Los textos se comparan por orden alfabético (más exactamente, por el orden de los códigos
de los caracteres). Recuerda lo visto en [la página anterior](premiers-pas.md): usa `===` y
`!==`, nunca `==` ni `!=`.

## Los operadores lógicos

Una decisión depende a menudo de varias cosas a la vez. Para entrar al cine, hay que
tener la edad **y** una entrada. Estos tres operadores unen varias preguntas en una
sola.

Para combinar varias preguntas:

- `&&` («y»): verdadero si **los dos** lados son verdaderos;
- `||` («o»): verdadero si **al menos un** lado es verdadero;
- `!` («no»): invierte la respuesta.

```javascript
const edad = 25;
const tieneEntrada = true;

if (edad >= 18 && tieneEntrada) {
  console.log("Entrada permitida");
}

const dia = "domingo";
if (dia === "sábado" || dia === "domingo") {
  console.log("Es fin de semana");
}

console.log(!tieneEntrada);
```

```sortie
Entrada permitida
Es fin de semana
false
```

Línea por línea:

- `edad` vale 25 y `tieneEntrada` vale `true`;
- primer `if`: `edad >= 18` es verdadero **y** `tieneEntrada` es verdadero, así que la
  entrada está permitida;
- segundo `if`: `dia` vale `"domingo"`. La primera prueba es falsa, la segunda es
  verdadera, y con `||` basta;
- el último `console.log` muestra `!tieneEntrada`: `true` invertido se convierte en
  `false`.

> **Consejo:** JavaScript se detiene en cuanto conoce la respuesta. En `a && b`, si `a`
> es falso, ni siquiera mira `b`.

**Te toca:** escribe un programa que guarde `70` en una constante `edad` y luego
muestre «Tarifa reducida» si la edad es menor que 18 o mayor o igual que 65, y
«Tarifa completa» si no.

> **Solución**
>
> ```javascript
> const edad = 70;
>
> if (edad < 18 || edad >= 65) {
>   console.log("Tarifa reducida");
> } else {
>   console.log("Tarifa completa");
> }
> ```
>
> ```sortie
> Tarifa reducida
> ```

## Truthy y falsy

A menudo quieres saber si una casilla está rellena: un apodo escrito, un número
distinto de cero. JavaScript te deja hacer esta pregunta de forma muy sencilla,
poniendo el valor solo dentro del `if`.

En un `if`, puedes poner cualquier valor, no solo un booleano.
JavaScript lo convierte entonces en `true` o `false`. Seis valores se convierten en `false`: se
llaman **falsy** («más o menos falsos»).

- `false`
- `0` (y `-0`, `0n`)
- `""` (el texto vacío)
- `null`
- `undefined`
- `NaN`

**Todos** los demás valores son **truthy**: se convierten en `true`. Incluso `"0"`,
`"false"`, un array vacío `[]` o un objeto vacío `{}`.

```javascript
const valores = [0, 1, "", "hola", null, undefined, NaN, "0", []];

for (const v of valores) {
  console.log(v, "→", Boolean(v));
}
```

```sortie
0 → false
1 → true
 → false
hola → true
null → false
undefined → false
NaN → false
0 → true
[] → true
```

Línea por línea:

- `valores` es una lista de nueve valores de todo tipo;
- `for (const v of valores)` toma esos valores uno a uno y guarda cada uno en `v`. Este
  bucle se explica en
  [el capítulo sobre los bucles](boucles.md#for-of-recorrer-los-valores);
- `Boolean(v)` convierte el valor en `true` o `false`, y `console.log` muestra los dos
  uno al lado del otro;
- la tercera línea de la salida empieza por un espacio: es el texto vacío `""`, que no
  muestra nada.

`Boolean(v)` muestra en qué se convierte un valor. Esta regla sirve a menudo para comprobar
si un texto está relleno:

```javascript
const apodo = "";

if (apodo) {
  console.log(`Hola, ${apodo}`);
} else {
  console.log("¡Apodo vacío!");
}
```

```sortie
¡Apodo vacío!
```

## El operador ternario

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Para elegir entre **dos valores**, el ternario es más corto que un `if`:
`condicion ? valorSiVerdadero : valorSiFalso`.

```javascript
const edad = 20;
const estado = edad >= 18 ? "mayor de edad" : "menor de edad";
console.log(estado);

const n = 3;
console.log(`${n} artículo${n > 1 ? "s" : ""}`);
```

```sortie
mayor de edad
3 artículos
```

Léelo como una pregunta: «¿edad ≥ 18? si sí, mayor de edad; si no, menor de edad».

> **Atención:** reserva el ternario para elecciones sencillas. Dos ternarios anidados se
> leen muy mal: en ese caso, mejor un `if`.

## Los valores ausentes: ?? y ?.

A veces falta un valor: un ajuste que el usuario no ha elegido, una dirección que aún
no está rellena. Estos dos operadores gestionan ese hueco sin escribir un `if` cada
vez.

### ??: un valor de reserva

`a ?? b` da `a`, salvo si `a` vale `null` o `undefined`: entonces da `b`.

```javascript
let color;
console.log(color ?? "azul");

const volumen = 0;
console.log(volumen ?? 50);
console.log(volumen || 50);
```

```sortie
azul
0
50
```

Línea por línea:

- `let color;` crea una variable sin valor: vale `undefined`. Por eso `??` da el valor
  de reserva `"azul"`;
- `volumen` vale `0`, que no es ni `null` ni `undefined`: `volumen ?? 50` conserva `0`;
- `volumen || 50` da `50`, porque `0` es falsy.

Compara con `||`, que se ve a menudo en código antiguo: `||` sustituye **todos** los
valores falsy, incluido `0`. El volumen `0` es un valor de verdad: `??` lo conserva, `||`
lo aplasta.

### ?.: preguntar sin que falle

`?.` (el **encadenamiento opcional**) lee una propiedad solo si el valor existe. Si no,
devuelve `undefined` en lugar de provocar un error.

```javascript
const usuario = { nombre: "Alice", direccion: null };

console.log(usuario.nombre);
console.log(usuario.direccion?.ciudad);
console.log(usuario.direccion?.ciudad ?? "ciudad desconocida");
```

```sortie
Alice
undefined
ciudad desconocida
```

Los objetos `{ ... }` se explican en [la página sobre los objetos](objets.md).

## La elección múltiple: switch

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Cuando se compara **un mismo valor** con varias posibilidades, `switch` es más claro
que una larga serie de `else if`.

```javascript
const dia = "martes";

switch (dia) {
  case "sábado":
  case "domingo":
    console.log("Descanso");
    break;
  case "miércoles":
    console.log("Media jornada");
    break;
  default:
    console.log("Trabajo");
}
```

```sortie
Trabajo
```

- Cada `case` compara con `===`.
- `break` sale del `switch`. Sin él, la ejecución **continúa** en el `case` siguiente.
- Dos `case` seguidos comparten el mismo código (aquí sábado y domingo).
- `default` sirve cuando ningún `case` coincide.

## Errores frecuentes

### Escribir = en lugar de ===

```javascript norun
const nota = 7;
if (nota = 10) {
  console.log("¡Perfecto!");
}
```

```text
file:///home/alice/js/main.mjs:2
if (nota = 10) {
         ^

TypeError: Assignment to constant variable.
    at file:///home/alice/js/main.mjs:2:10

Node.js v22.22.0
```

Un solo `=` **guarda** un valor, no compara. Aquí, JavaScript intenta meter 10
en `nota`. Con `let`, no habría error, pero la condición sería siempre
verdadera: es peor. Compara con `===`: `if (nota === 10)`.

### Olvidar una llave de cierre

```javascript norun
const edad = 20;
if (edad >= 18) {
  console.log("Mayor de edad");
```

```text
file:///home/alice/js/main.mjs:4



SyntaxError: Unexpected end of input

Node.js v22.22.0
```

El archivo termina (*end of input*) mientras un bloque `{` sigue abierto. Cada
`{` debe tener su `}`. Un buen editor colorea las parejas de llaves para ayudarte.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```javascript norun
const x = 7;
if (x > 10) {
  console.log("grande");
} else if (x > 5) {
  console.log("mediano");
} else {
  console.log("pequeño");
}
```

> **Respuesta**
>
> `mediano`. `x > 10` es falso y luego `x > 5` es verdadero: su bloque se ejecuta y se detiene ahí.

**Pregunta 2:** ¿qué dan `0 ?? 10` y `0 || 10`?

> **Respuesta**
>
> ```javascript
> console.log(0 ?? 10);
> console.log(0 || 10);
> ```
>
> ```sortie
> 0
> 10
> ```
>
> `??` solo sustituye `null` y `undefined`. `||` sustituye cualquier valor falsy, incluido `0`.

**Pregunta 3:** entre `0`, `"0"`, `[]` y `""`, ¿qué valores son falsy?

> **Respuesta**
>
> ```javascript
> console.log(Boolean(0), Boolean("0"), Boolean([]), Boolean(""));
> ```
>
> ```sortie
> false true true false
> ```
>
> Solo `0` y `""` son falsy. `"0"` es un texto no vacío, y un array vacío `[]` es
> truthy.

## En resumen

- `if`, `else if` y `else` eligen un camino; `switch` compara un valor con
  varios casos, sin olvidar `break`.
- `&&`, `||` y `!` combinan condiciones; compara con `===` y `!==`.
- `false`, `0`, `""`, `null`, `undefined` y `NaN` son falsy; todo lo demás es truthy.
- `cond ? a : b` elige un valor; `??` da un valor de reserva; `?.` lee sin
  que falle.

Siguiente: [los bucles](boucles.md).
