# Las funciones

Esta página explica cómo escribir funciones en JavaScript, con `function` o con una flecha `=>`, darles parámetros y obtener su resultado.

> **Objetivos:**
>
> - sabrás declarar una función con `function` y llamarla;
> - sabrás devolver un valor con `return`;
> - sabrás usar parámetros por defecto y parámetros «rest»;
> - sabrás escribir una función flecha `=>`;
> - sabrás pasar una función a otra función (un *callback*).

## Declarar una función

Una [función](../debuter/glossaire.md#funcion) es un trozo de código con nombre que
puedes reutilizar. Es como una receta: la escribes una vez y luego la
sigues tantas veces como quieras.

```javascript
function saludar() {
  console.log("¡Hola!");
}

saludar();
saludar();
```

```sortie
¡Hola!
¡Hola!
```

- `function` anuncia una función, seguida de su nombre (`saludar`) y de paréntesis.
- El código de la función, su **cuerpo**, va entre llaves `{ }`.
- Escribir la función no ejecuta nada: guarda la receta. Escribir `saludar()`
  **llama** a la función, es decir, la ejecuta. Los paréntesis son obligatorios.

## Los parámetros

Una función que siempre hace exactamente lo mismo sirve de poco. Los parámetros la
hacen flexible: es como una receta «para N personas», donde eliges N cada vez.

Los [parámetros](../debuter/glossaire.md#parametro) son los valores que la función
recibe. Son casillas vacías que se rellenan en el momento de la llamada. Los valores que se dan
en la llamada se llaman [argumentos](../debuter/glossaire.md#argumento).

```javascript
function saludar(nombre) {
  console.log(`¡Hola, ${nombre}!`);
}

saludar("Alice");
saludar("Bob");
```

```sortie
¡Hola, Alice!
¡Hola, Bob!
```

Línea por línea:

- `function saludar(nombre)` guarda la función. Todavía no se muestra nada;
- `saludar("Alice")` llama a la función: `nombre` recibe `"Alice"` y luego
  `console.log` muestra la frase con ese nombre;
- `saludar("Bob")` vuelve a empezar, esta vez con `nombre` igual a `"Bob"`.

Aquí, `nombre` es el parámetro; `"Alice"` y `"Bob"` son los argumentos. Varios
parámetros se separan con comas, y los argumentos se dan en el mismo
orden.

**Te toca:** escribe una función `presentar(nombre, edad)` que muestre «Me llamo … y
tengo … años.», y luego llámala con `"Lea"` y `12`.

> **Solución**
>
> ```javascript
> function presentar(nombre, edad) {
>   console.log(`Me llamo ${nombre} y tengo ${edad} años.`);
> }
>
> presentar("Lea", 12);
> ```
>
> ```sortie
> Me llamo Lea y tengo 12 años.
> ```

## Devolver un valor con return

A menudo no quieres solo mostrar un resultado: quieres guardarlo para usarlo después.
Una calculadora te da un número, que puedes apuntar, reutilizar o sumar a otro.
`return` hace lo mismo.

`return` detiene la función y **devuelve** un valor al código que la llamó. El valor
devuelto sustituye a la llamada.

```javascript
function area(ancho, alto) {
  return ancho * alto;
}

console.log(area(3, 4));
const total = area(2, 5) + 1;
console.log(total);
```

```sortie
12
11
```

Línea por línea:

- `function area(ancho, alto)` guarda la función, sin calcular nada;
- `area(3, 4)` ejecuta la función: `ancho` vale 3, `alto` vale 4, y `return` devuelve
  12. `console.log` muestra ese 12;
- `area(2, 5)` devuelve 10, luego `+ 1` da 11, que se guarda en `total`;
- el último `console.log` muestra `total`, es decir, 11.

`area(2, 5) + 1` se convierte en `10 + 1`. Una función sin `return` devuelve `undefined`.

`return` también sirve para salir antes:

```javascript
function dividir(a, b) {
  if (b === 0) {
    return "división imposible";
  }
  return a / b;
}

console.log(dividir(10, 4));
console.log(dividir(10, 0));
```

```sortie
2.5
división imposible
```

### ¿Mostrar o devolver?

`console.log` **muestra** un valor en la pantalla. `return` lo **entrega** al código
que llama, que puede usarlo: guardarlo, volver a calcular con él, mostrarlo.

```javascript
function dobleMuestra(x) {
  console.log(x * 2);
}

function doble(x) {
  return x * 2;
}

const a = dobleMuestra(5);
const b = doble(5);
console.log(a, b);
```

```sortie
10
undefined 10
```

> **Recuerda:** prefiere las funciones que devuelven un valor. Después decidirás
> qué hacer con él.

**Te toca:** escribe una función `perimetro(lado)` que devuelva el perímetro de un
cuadrado (cuatro veces el lado). Muestra `perimetro(5)` y luego
`perimetro(5) + perimetro(2)`.

> **Solución**
>
> ```javascript
> function perimetro(lado) {
>   return lado * 4;
> }
>
> console.log(perimetro(5));
> console.log(perimetro(5) + perimetro(2));
> ```
>
> ```sortie
> 20
> 28
> ```

## Los parámetros por defecto

En un restaurante, el plato viene con patatas fritas, salvo si pides otra cosa. Un
valor por defecto funciona igual: te evita repetir el valor habitual en cada llamada.

Si falta un argumento, su parámetro vale `undefined`. Un **valor por defecto**, escrito
con `=`, lo sustituye.

```javascript
function saludar(nombre, saludo = "Hola") {
  return `¡${saludo}, ${nombre}!`;
}

console.log(saludar("Alice"));
console.log(saludar("Bob", "Buenas"));
```

```sortie
¡Hola, Alice!
¡Buenas, Bob!
```

Línea por línea:

- `saludo = "Hola"` prevé un valor de reserva para `saludo`;
- `saludar("Alice")` da un solo argumento: `nombre` vale `"Alice"` y `saludo` toma su
  valor por defecto, `"Hola"`;
- `saludar("Bob", "Buenas")` da los dos: `"Buenas"` sustituye al valor por defecto.

## Los parámetros rest

A veces no sabes de antemano cuántos valores recibirá la función: quieres sumar dos
números, o diez. Los parámetros rest aceptan cualquier cantidad de argumentos.

Con tres puntos `...` delante del último parámetro, la función recibe **todos los
argumentos restantes**, guardados en un array.

```javascript
function suma(...numeros) {
  let total = 0;
  for (const n of numeros) {
    total += n;
  }
  return total;
}

console.log(suma(1, 2));
console.log(suma(1, 2, 3, 4));
console.log(suma());
```

```sortie
3
10
0
```

Línea por línea:

- `...numeros` guarda todos los argumentos en un array: con `suma(1, 2)`, `numeros`
  vale `[1, 2]`;
- `total` empieza en 0;
- el bucle `for` toma cada número del array, uno a uno, y lo suma a `total`;
- `return total` devuelve la suma. Con `suma()`, el array está vacío: el bucle no da
  ninguna vuelta, y la función devuelve 0.

Es un poco como una bolsa: metes en ella tantos argumentos como quieras. Los arrays se
explican en [los arrays](collections.md).

## Las funciones flecha

Muchas funciones solo hacen un pequeño cálculo. Escribir `function`, llaves y `return`
para una sola línea es largo. La flecha dice lo mismo con menos palabras.

Una **función flecha** (*arrow function*) es una forma más corta de escribir una
función. Se guarda en una constante.

```javascript
const cuadrado = (x) => x * x;
const sumar = (a, b) => a + b;
const decirHola = () => "¡Hola!";

console.log(cuadrado(4));
console.log(sumar(2, 3));
console.log(decirHola());
```

```sortie
16
5
¡Hola!
```

- Los parámetros van antes de la flecha `=>`, el resultado después.
- En una línea, el resultado se **devuelve automáticamente**, sin `return`.
- Sin parámetros, se escriben paréntesis vacíos `()`.

Cuando el cuerpo ocupa varias líneas, se ponen llaves, y `return` vuelve a ser
obligatorio:

```javascript
const precioConIva = (precio, tasa = 0.2) => {
  const total = precio * (1 + tasa);
  return Math.round(total * 100) / 100;
};

console.log(precioConIva(50));
console.log(precioConIva(50, 0.055));
```

```sortie
60
52.75
```

> **Consejo:** usa `function` para las funciones principales de tu programa, y
> las flechas para las funciones pequeñas que se pasan a otras (mira más abajo). Las dos
> son correctas.

**Te toca:** escribe una función flecha `triple` que devuelva el triple de un número, y
una función flecha `bienvenida` que devuelva «¡Bienvenida, …!» con un nombre. Muestra
`triple(7)` y `bienvenida("Ana")`.

> **Solución**
>
> ```javascript
> const triple = (x) => x * 3;
> const bienvenida = (nombre) => `¡Bienvenida, ${nombre}!`;
>
> console.log(triple(7));
> console.log(bienvenida("Ana"));
> ```
>
> ```sortie
> 21
> ¡Bienvenida, Ana!
> ```

## Las funciones son valores

En JavaScript, una función es un valor, como un número o un texto. Se puede
guardar en una variable, meter en un array o dar a otra función.

¿Para qué sirve? Así puedes dar a una función no solo datos, sino también una acción
que hacer con esos datos. Es como dar a alguien una cesta de manzanas con la consigna
«pélalas».

```javascript
function doble(x) {
  return x * 2;
}

const operacion = doble;
console.log(operacion(21));
console.log(typeof doble);
```

```sortie
42
function
```

Línea por línea:

- `function doble(x)` guarda una función llamada `doble`;
- `const operacion = doble;` guarda la función misma en `operacion`, sin llamarla (sin
  paréntesis). `operacion` y `doble` nombran la misma receta;
- `operacion(21)` llama entonces a `doble` con 21, que devuelve 42;
- `typeof doble` da el tipo del valor: aquí, `function`.

Cuidado con la diferencia: `doble` es la función misma (la receta);
`doble(21)` es su resultado (el plato).

### Los callbacks

Una función que se da como argumento a otra se llama **callback** («devolución de llamada»):
la otra función la «volverá a llamar» cuando la necesite.

```javascript
function aplicarDosVeces(f, valor) {
  return f(f(valor));
}

console.log(aplicarDosVeces((x) => x + 10, 1));
console.log(aplicarDosVeces((x) => x * x, 3));
```

```sortie
21
81
```

Línea por línea:

- `aplicarDosVeces` recibe dos cosas: una función `f` y un valor;
- `f(valor)` aplica `f` una primera vez, y luego el `f( )` de fuera la aplica al
  resultado;
- primera llamada: `f` suma 10. 1 se convierte en 11, y luego en 21;
- segunda llamada: `f` eleva al cuadrado. 3 se convierte en 9, y luego en 81.

Los arrays usan mucho los callbacks. `map` aplica una función a cada
elemento:

```javascript
const precios = [10, 20, 30];
const preciosConIva = precios.map((p) => p * 1.2);
console.log(preciosConIva);
```

```sortie
[ 12, 24, 36 ]
```

Volverás a ver `map`, `filter` y sus amigos en [los arrays](collections.md).

**Te toca:** parte del array `const notas = [8, 12, 15];` y usa `map` para crear un
nuevo array donde cada nota gana 2 puntos. Muéstralo.

> **Solución**
>
> ```javascript
> const notas = [8, 12, 15];
> const notasExtra = notas.map((n) => n + 2);
> console.log(notasExtra);
> ```
>
> ```sortie
> [ 10, 14, 17 ]
> ```

## El hoisting

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Una función declarada con `function` se puede llamar **antes** del lugar donde está
escrita. JavaScript la «eleva» (*hoisting*) al principio del archivo antes de empezar.

```javascript
console.log(triple(5));

function triple(x) {
  return x * 3;
}
```

```sortie
15
```

**No** pasa lo mismo con una función flecha guardada en una `const`: la constante
solo existe a partir de su línea. Mira los errores frecuentes más abajo.

## Una función que se llama a sí misma

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Una función puede llamarse a sí misma: es la
[recursión](../debuter/glossaire.md#recursion). Hace falta un **caso base**, si no,
no se detiene nunca.

```javascript
function cuentaAtras(n) {
  if (n === 0) {
    console.log("¡Ya!");
    return;
  }
  console.log(n);
  cuentaAtras(n - 1);
}

cuentaAtras(3);
```

```sortie
3
2
1
¡Ya!
```

Un ejemplo clásico es el factorial: 4! = 4 × 3 × 2 × 1.

```javascript
function factorial(n) {
  if (n <= 1) {
    return 1;
  }
  return n * factorial(n - 1);
}

console.log(factorial(4));
```

```sortie
24
```

## Errores frecuentes

### Olvidar return en una flecha con llaves

```javascript norun
const doble = (x) => {
  x * 2;
};
console.log(doble(5));
```

```text
undefined
```

Con llaves, el resultado ya no se devuelve solo: la función devuelve
`undefined`. Añade `return x * 2;`, o quita las llaves: `(x) => x * 2`.

### Llamar a una flecha antes de definirla

```javascript norun
console.log(cuadrado(4));

const cuadrado = (x) => x * x;
```

```text
file:///home/alice/js/main.mjs:1
console.log(cuadrado(4));
            ^

ReferenceError: Cannot access 'cuadrado' before initialization
    at file:///home/alice/js/main.mjs:1:13

Node.js v22.22.0
```

Una `const` solo existe a partir de su línea. Coloca la definición antes de la llamada, o
usa una declaración `function cuadrado(x) { ... }`, que sí se eleva.

### Olvidar los paréntesis de la llamada

```javascript norun
function saludar() {
  return "Hola";
}
console.log(saludar);
```

```text
[Function: saludar]
```

Sin paréntesis, hablas de la función misma, sin ejecutarla. Escribe
`saludar()`.

### Olvidar el caso base

```javascript norun
function contar(n) {
  return contar(n - 1);
}
contar(3);
```

```text
file:///home/alice/js/main.mjs:2
  return contar(n - 1);
  ^

RangeError: Maximum call stack size exceeded
    at contar (file:///home/alice/js/main.mjs:2:3)
    at contar (file:///home/alice/js/main.mjs:2:10)
    at contar (file:///home/alice/js/main.mjs:2:10)
```

La función se llama sin fin, hasta llenar la memoria reservada para las llamadas (*maximum
call stack size exceeded*, «tamaño máximo de la pila de llamadas superado»). Añade un
caso base: `if (n === 0) return 0;`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```javascript norun
function f(x) {
  return x + 1;
  return x * 2;
}
console.log(f(5));
```

> **Respuesta**
>
> `6`. El primer `return` detiene la función: la línea siguiente nunca se ejecuta.

**Pregunta 2:** escribe una función flecha `media(a, b)` que devuelva la media de
dos números.

> **Respuesta**
>
> ```javascript
> const media = (a, b) => (a + b) / 2;
> console.log(media(3, 4));
> ```
>
> ```sortie
> 3.5
> ```
>
> Los paréntesis alrededor de `a + b` hacen la suma antes de la división.

**Pregunta 3:** ¿cuánto vale `aplicar((n) => n * 10, 4)` si
`function aplicar(f, x) { return f(x); }`?

> **Respuesta**
>
> `40`. `aplicar` llama al callback `f` con `4`, que devuelve `4 * 10`.

## En resumen

- `function nombre(parámetros) { ... }` declara una función; `nombre(argumentos)` la llama.
- `return` devuelve un valor y detiene la función; sin `return`, se obtiene
  `undefined`.
- `param = valor` da un valor por defecto; `...resto` reúne los argumentos
  restantes en un array.
- `(x) => x * 2` es una función flecha: sin llaves, devuelve su resultado
  sola.
- Una función es un valor: se puede pasar como argumento (un callback).
- Una declaración `function` se eleva (hoisting); una flecha en una `const`, no.

Siguiente: [trabajar con texto](texte.md).
