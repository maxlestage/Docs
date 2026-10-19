# Condiciones y bucles

Esta página muestra cómo tomar decisiones con `if`, `switch` y el operador ternario, y luego cómo repetir acciones con `while`, `for` y `for...of`.

> **Objetivos:**
>
> - sabrás escribir una condición con `if`, `else if` y `else`;
> - sabrás combinar comparaciones con `&&`, `||` y `!`;
> - sabrás qué son los valores «truthy» y «falsy», y usar `??` y `?.`;
> - sabrás repetir código con `while`, `for` y `for...of`;
> - sabrás salir de un bucle con `break` o saltarte una vuelta con `continue`.

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

- La pregunta va entre paréntesis `( )`.
- El código que se ejecuta si la respuesta es verdadera va entre llaves `{ }`. Esto se
  llama un **bloque**.
- El bloque está desplazado dos espacios: es la **sangría**. JavaScript no la exige,
  pero hace que el código se lea bien.

### else y else if

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

## Los operadores de comparación

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

Los textos se comparan por orden alfabético (más exactamente, por el orden de los códigos
de los caracteres). Recuerda lo visto en [la página anterior](premiers-pas.md): usa `===` y
`!==`, nunca `==` ni `!=`.

## Los operadores lógicos

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

> **Consejo:** JavaScript se detiene en cuanto conoce la respuesta. En `a && b`, si `a`
> es falso, ni siquiera mira `b`.

## Truthy y falsy

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

## El bucle while

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

> **Atención:** si la condición sigue siendo siempre verdadera, el bucle no se detiene nunca.
> Aquí, `contador--` acerca el contador a 0 en cada vuelta. Para detener un programa
> bloqueado, pulsa **Ctrl + C** en el terminal.

## El bucle for

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

## for...of y for...in

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

- `if`, `else if` y `else` eligen un camino; `switch` compara un valor con
  varios casos, sin olvidar `break`.
- `&&`, `||` y `!` combinan condiciones; compara con `===` y `!==`.
- `false`, `0`, `""`, `null`, `undefined` y `NaN` son falsy; todo lo demás es truthy.
- `cond ? a : b` elige un valor; `??` da un valor de reserva; `?.` lee sin
  que falle.
- `while` repite mientras una condición sea verdadera, `for` cuenta, `for...of` recorre los
  valores, `for...in` las claves de un objeto.
- `break` sale del bucle, `continue` pasa a la vuelta siguiente.

Siguiente: [las funciones](fonctions.md).
