# Las funciones

Esta página explica cómo escribir funciones en TypeScript, tipar sus parámetros y su valor de retorno, y pasar una función a otra.

> **Objetivos:**
>
> - sabrás escribir una función con parámetros y un tipo de retorno;
> - sabrás hacer que un parámetro sea opcional o darle un valor por defecto;
> - sabrás escribir una función flecha `(x) => ...`;
> - sabrás escribir el tipo de una función: `(x: number) => number`;
> - sabrás pasar una función como argumento (un *callback*) y tiparla.

## Definir y llamar a una función

Una [función](../debuter/glossaire.md#funcion) es un trozo de código con nombre, que
puedes reutilizar. Es como una receta: la escribes una vez y luego la
sigues tantas veces como quieras.

```typescript
function saludar(): void {
  console.log("¡Hola!");
}

saludar();
saludar();
```

```sortie
¡Hola!
¡Hola!
```

- `function` anuncia una función, `saludar` es su nombre;
- los paréntesis `()` contienen sus parámetros (aquí, ninguno);
- `: void` dice que la función no devuelve nada útil (*void*: «vacío»);
- el bloque `{ ... }` es el cuerpo de la función.

Escribir `function saludar` no ejecuta nada: guarda la receta. Escribir `saludar()`
**llama** a la función, es decir, la ejecuta. Los paréntesis son obligatorios.

## Los parámetros y su tipo

Los [parámetros](../debuter/glossaire.md#parametro) son los valores que la función
recibe. Son casillas vacías, que se rellenan en el momento de la llamada con los
[argumentos](../debuter/glossaire.md#argumento). En TypeScript, cada parámetro tiene un
tipo.

```typescript
function presentar(nombre: string, edad: number): void {
  console.log(`${nombre} tiene ${edad} años.`);
}

presentar("Alice", 30);
presentar("Bob", 25);
```

```sortie
Alice tiene 30 años.
Bob tiene 25 años.
```

Aquí, `nombre` y `edad` son los parámetros; `"Alice"` y `30` son los argumentos.

El tipo de un parámetro es un contrato: «dame un número». TypeScript comprueba
**cada llamada**. `presentar("Alice", "treinta")` se rechaza antes incluso de ejecutar el
programa.

> **Recuerda:** TypeScript no adivina el tipo de un parámetro. Escríbelo siempre;
> si no, `tsc --strict` muestra un error (ver los
> [errores frecuentes](#errores-frecuentes)).

## Devolver un valor con return

`return` devuelve un valor al código que ha llamado a la función, y detiene la función.
El tipo de ese valor se escribe después de los paréntesis.

```typescript
function area(ancho: number, alto: number): number {
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

El valor devuelto reemplaza a la llamada: `area(2, 5) + 1` se convierte en `10 + 1`.

El tipo de retorno `: number` se puede omitir: TypeScript lo adivina a partir del
`return`. Escribirlo sigue siendo una buena costumbre: si te equivocas en el cuerpo de la
función, el error aparece en la función, no más lejos, en el código que la llama.

### ¿Mostrar o devolver?

`console.log` **muestra** un valor en la pantalla. `return` lo **entrega** al código que ha
llamado a la función, que puede usarlo.

```typescript
function mostrarDoble(x: number): void {
  console.log(x * 2);
}

function doble(x: number): number {
  return x * 2;
}

mostrarDoble(5);
console.log(doble(5) + 1);
```

```sortie
10
11
```

`mostrarDoble(5) + 1` se rechazaría: una función `void` no da ningún valor
utilizable.

> **Consejo:** prefiere las funciones que devuelven un valor. Después decidirás
> si mostrarlo, guardarlo o reutilizarlo.

### Salir antes

Un `return` en medio de la función la detiene en el acto:

```typescript
function dividir(a: number, b: number): string {
  if (b === 0) {
    return "división imposible";
  }
  return (a / b).toString();
}

console.log(dividir(10, 4));
console.log(dividir(10, 0));
```

```sortie
2.5
división imposible
```

TypeScript comprueba que **todos los caminos** devuelven un valor del tipo correcto. Si
falta un `return`, lo señala.

## Parámetros opcionales y valores por defecto

### Un parámetro opcional: ?

Un `?` después del nombre hace que el parámetro sea opcional. Si no se da, vale
`undefined`: su tipo pasa a ser `string | undefined`.

```typescript
function saludar(nombre: string, titulo?: string): string {
  if (titulo === undefined) {
    return `Hola, ${nombre}!`;
  }
  return `Hola, ${titulo} ${nombre}!`;
}

console.log(saludar("Alice"));
console.log(saludar("Martín", "doctor"));
```

```sortie
Hola, Alice!
Hola, doctor Martín!
```

TypeScript te obliga a tratar el caso `undefined` antes de usar `titulo` como un
texto.

### Un valor por defecto: =

Más sencillo: da un valor por defecto. Se usa si falta el argumento. El
tipo se adivina a partir de ese valor.

```typescript
function precioConIva(precio: number, tasa = 0.2): number {
  return precio * (1 + tasa);
}

console.log(precioConIva(100));
console.log(precioConIva(100, 0.055));
```

```sortie
120
105.5
```

> **Atención:** los parámetros opcionales se colocan **después** de los obligatorios.
> `function f(a?: number, b: number)` se rechaza.

## Un número variable de argumentos

Tres puntos `...` delante del último parámetro lo convierten en **parámetro
rest**: recibe todos los argumentos restantes, guardados en un array. Su tipo es
un array, aquí `number[]` («lista de números»).

```typescript
function suma(...numeros: number[]): number {
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

Cada argumento se comprueba: `suma(1, "2")` se rechazaría.

## Las funciones flecha

Una **función flecha** (*arrow function*) es una forma corta de escribir, con `=>`. A
menudo se guarda en una constante.

```typescript
const cuadrado = (x: number): number => x * x;

const presentar = (nombre: string): string => {
  const mayusculas = nombre.toUpperCase();
  return `Soy ${mayusculas}.`;
};

console.log(cuadrado(4));
console.log(presentar("lucía"));
```

```sortie
16
Soy LUCÍA.
```

- Con una sola expresión después de `=>`, no hacen falta llaves ni `return`: el
  valor se devuelve automáticamente.
- Con varias líneas, se ponen llaves y un `return`.

Una función sin nombre, como esta, también se llama
[lambda](../debuter/glossaire.md#lambda). Las funciones flecha están por todas partes en
TypeScript, sobre todo como *callbacks* (más abajo).

## El tipo de una función

Una función es un valor, como un número o un texto: se puede guardar en una
variable. Su tipo se escribe como una función flecha: los parámetros, luego `=>`, luego
el tipo de retorno.

```typescript
let operacion: (a: number, b: number) => number;

operacion = (a, b) => a + b;
console.log(operacion(6, 3));

operacion = (a, b) => a * b;
console.log(operacion(6, 3));
```

```sortie
9
18
```

`(a: number, b: number) => number` se lee: «una función que recibe dos números y
devuelve un número». Fíjate: en `(a, b) => a + b`, ya no hace falta escribir los
tipos de los parámetros. TypeScript ya los conoce gracias al tipo de `operacion`.

Para reutilizar un tipo de función, dale un nombre con `type`:

```typescript
type Operacion = (a: number, b: number) => number;

const mas: Operacion = (a, b) => a + b;
const menos: Operacion = (a, b) => a - b;

console.log(mas(10, 4), menos(10, 4));
```

```sortie
14 6
```

## Los callbacks: pasar una función a una función

Un **callback** («devolución de llamada») es una función que se pasa como argumento a otra
función, que la llamará más tarde. Es como dejarle tu número al panadero:
te llamará cuando el pan esté listo.

```typescript
function repetir(veces: number, accion: (vuelta: number) => void): void {
  for (let i = 1; i <= veces; i++) {
    accion(i);
  }
}

repetir(3, (vuelta) => {
  console.log(`Vuelta número ${vuelta}`);
});
```

```sortie
Vuelta número 1
Vuelta número 2
Vuelta número 3
```

El parámetro `accion` tiene un tipo de función: `(vuelta: number) => void`. Gracias a él,
TypeScript sabe que `vuelta` es un `number` dentro de la función flecha pasada en la llamada.

Aquí tienes un ejemplo más útil: una función que transforma cada número de una lista.

```typescript
function transformar(numeros: number[], f: (n: number) => number): number[] {
  const resultado: number[] = [];
  for (const n of numeros) {
    resultado.push(f(n));
  }
  return resultado;
}

console.log(transformar([1, 2, 3], (n) => n * 10));
console.log(transformar([1, 2, 3], (n) => n - 1));
```

```sortie
[ 10, 20, 30 ]
[ 0, 1, 2 ]
```

`push` añade un elemento al final de un array. Los arrays ya tienen un método
`map` que hace exactamente eso: lo verás en [los arrays](collections.md).

## El ámbito de las variables

El [ámbito](../debuter/glossaire.md#ambito) de una variable es la zona del código donde
existe. Los parámetros y las variables creadas en una función solo existen dentro de
esa función.

```typescript
const tasaIva = 0.2;

function precioConIva(precio: number): number {
  const total = precio * (1 + tasaIva);
  return Math.round(total * 100) / 100;
}

console.log(precioConIva(19.99));
```

```sortie
23.99
```

- La función ve `tasaIva`, creada **fuera**, por encima de ella.
- Fuera, `total` no existe: `console.log(total)` se rechazaría.

> **Consejo:** una función es más fácil de entender cuando recibe todo lo que
> necesita como parámetros.

## Las sobrecargas, en pocas palabras

A veces, una función acepta varias formas de llamada, con un tipo de retorno
diferente para cada una. TypeScript permite enumerarlas antes del cuerpo: son las
**sobrecargas** (*overloads*).

```typescript
function duplicar(x: number): number;
function duplicar(x: string): string;
function duplicar(x: number | string): number | string {
  return typeof x === "number" ? x * 2 : x + x;
}

console.log(duplicar(21));
console.log(duplicar("ja"));
```

```sortie
42
jaja
```

Las dos primeras líneas son las formas permitidas; la tercera es el código de verdad.
Gracias a ellas, se sabe que `duplicar(21)` es un `number`, y `duplicar("ja")` un
`string`. Las encontrarás en las bibliotecas; para tus comienzos, una unión
(`number | string`) suele bastar.

## Errores frecuentes

### Olvidar el tipo de un parámetro

```typescript norun
function doble(x) {
  return x * 2;
}
console.log(doble(4));
```

```text
main.ts:1:16 - error TS7006: Parameter 'x' implicitly has an 'any' type.

1 function doble(x) {
                 ~


Found 1 error in main.ts:1
```

«El parámetro `x` tiene implícitamente el tipo `any`.» Sin tipo, `x` podría ser
cualquier cosa: el modo estricto lo rechaza. Escribe `function doble(x: number)`.

### Olvidar un argumento

```typescript norun
function area(ancho: number, alto: number): number {
  return ancho * alto;
}
console.log(area(3));
```

```text
main.ts:4:13 - error TS2554: Expected 2 arguments, but got 1.

4 console.log(area(3));
              ~~~~

  main.ts:1:30
    1 function area(ancho: number, alto: number): number {
                                   ~~~~~~~~~~~~
    An argument for 'alto' was not provided.


Found 1 error in main.ts:4
```

La función espera 2 argumentos (*expected 2*), recibe 1 (*got 1*). El mensaje
incluso precisa cuál falta: `alto`. Dalos todos: `area(3, 4)`. O haz que el
parámetro sea opcional con `?` o un valor por defecto.

### Pasar un argumento del tipo incorrecto

```typescript norun
function area(ancho: number, alto: number): number {
  return ancho * alto;
}
console.log(area(3, "4"));
```

```text
main.ts:4:21 - error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.

4 console.log(area(3, "4"));
                      ~~~


Found 1 error in main.ts:4
```

`"4"` es un texto, no un número. Escribe `area(3, 4)`, o convierte el texto con
`Number("4")`.

### Olvidar un return en un camino

```typescript norun
function resultado(nota: number): string {
  if (nota >= 10) {
    return "aprobado";
  } else if (nota < 10) {
    return "suspendido";
  }
}
console.log(resultado(12));
```

```text
main.ts:1:35 - error TS2366: Function lacks ending return statement and return type does not include 'undefined'.

1 function resultado(nota: number): string {
                                    ~~~~~~


Found 1 error in main.ts:1
```

«Falta un `return` al final.» TypeScript no ve que las dos pruebas cubren
todos los casos. Reemplaza `else if (nota < 10)` por un simple `else`: así todos los caminos
devuelven un texto.

## Ponte a prueba

**Pregunta 1:** escribe una función flecha `media` que reciba dos números y
devuelva su media.

> **Respuesta**
>
> ```typescript
> const media = (a: number, b: number): number => (a + b) / 2;
>
> console.log(media(3, 4));
> ```
>
> ```sortie
> 3.5
> ```

**Pregunta 2:** ¿qué quiere decir el tipo `(texto: string) => boolean`?

> **Respuesta**
>
> Es el tipo de una función que recibe un texto y devuelve `true` o `false`. Por
> ejemplo: `(texto) => texto.length > 3`.

**Pregunta 3:** ¿qué diferencia hay entre `titulo?: string` y `titulo = "Sr."`?

> **Respuesta**
>
> Los dos hacen que el parámetro sea opcional. Con `?`, vale `undefined` cuando
> falta: hay que comprobar ese caso antes de usarlo. Con `= "Sr."`, vale `"Sr."`
> cuando falta: siempre es un `string`.

## En resumen

- Una función se escribe `function nombre(param: tipo): tipoRetorno { ... }` y se llama
  con `nombre(...)`.
- Los parámetros siempre deben estar tipados; el tipo de retorno se puede adivinar,
  pero escribirlo ayuda.
- `?` hace que un parámetro sea opcional (entonces vale `undefined`), `= valor` le da
  un valor por defecto, `...nombre: T[]` recibe todos los argumentos restantes.
- Una función flecha se escribe `(x: number) => x * 2`; `void` quiere decir «no devuelve
  nada».
- El tipo de una función se escribe `(x: number) => number`: permite tipar los
  callbacks, cuyos parámetros se adivinan entonces.

Siguiente: [trabajar con texto](texte.md).
