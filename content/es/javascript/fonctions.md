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

Aquí, `nombre` es el parámetro; `"Alice"` y `"Bob"` son los argumentos. Varios
parámetros se separan con comas, y los argumentos se dan en el mismo
orden.

## Devolver un valor con return

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

## Los parámetros por defecto

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

## Los parámetros rest

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

Es un poco como una bolsa: metes en ella tantos argumentos como quieras. Los arrays se
explican en [los arrays](collections.md).

## Las funciones flecha

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

## Las funciones son valores

En JavaScript, una función es un valor, como un número o un texto. Se puede
guardar en una variable, meter en un array o dar a otra función.

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

## El hoisting

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
