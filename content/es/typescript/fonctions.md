# Las funciones

Esta página explica cómo escribir funciones en TypeScript, tipar sus parámetros y su valor de retorno, y saber dónde existen sus variables.

> **Objetivos:**
>
> - sabrás escribir una función con parámetros y un tipo de retorno;
> - sabrás hacer que un parámetro sea opcional o darle un valor por defecto;
> - sabrás escribir una función flecha `(x) => ...`;
> - sabrás decir dónde existe una variable: su ámbito.

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

Una receta de crepes sirve para 4 o para 10 personas: solo cambian las cantidades.
Del mismo modo, una función puede hacer el mismo trabajo con valores distintos en cada
llamada.

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

Línea por línea:

- las tres primeras líneas guardan la función: todavía no se muestra nada;
- `presentar("Alice", 30)` ejecuta la función: `nombre` recibe `"Alice"` y `edad`
  recibe `30`, y luego el cuerpo muestra la frase;
- `presentar("Bob", 25)` la vuelve a ejecutar con otros valores: esta vez, `nombre`
  vale `"Bob"` y `edad` vale `25`.

Aquí, `nombre` y `edad` son los parámetros; `"Alice"` y `30` son los argumentos.

El tipo de un parámetro es un contrato: «dame un número». TypeScript comprueba
**cada llamada**. `presentar("Alice", "treinta")` se rechaza antes incluso de ejecutar el
programa.

> **Recuerda:** TypeScript no adivina el tipo de un parámetro. Escríbelo siempre;
> si no, `tsc --strict` muestra un error (ver los
> [errores frecuentes](#errores-frecuentes)).

## Devolver un valor con return

Una calculadora no se limita a mostrar: te **entrega** un resultado, que puedes
reutilizar en otro cálculo. `return` permite que una función haga lo mismo.

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

Línea por línea:

- `area(3, 4)` ejecuta la función con `ancho` a 3 y `alto` a 4;
- `return ancho * alto` calcula 12 y lo devuelve: `console.log` muestra 12;
- `area(2, 5)` devuelve 10, y luego se suma 1: `total` vale 11;
- la última línea muestra `total`.

El valor devuelto reemplaza a la llamada: `area(2, 5) + 1` se convierte en `10 + 1`.

El tipo de retorno `: number` se puede omitir: TypeScript lo adivina a partir del
`return`. Escribirlo sigue siendo una buena costumbre: si te equivocas en el cuerpo de la
función, el error aparece en la función, no más lejos, en el código que la llama.

**Te toca:** escribe una función `perimetro` que reciba el ancho y el alto de un
rectángulo (dos números) y devuelva su perímetro. Muestra `perimetro(3, 4)`.

> **Solución**
>
> ```typescript
> function perimetro(ancho: number, alto: number): number {
>   return 2 * (ancho + alto);
> }
>
> console.log(perimetro(3, 4));
> ```
>
> ```sortie
> 14
> ```

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

Algunos datos no siempre hacen falta. En la cafetería, puedes pedir «un café» sin
decir el tamaño: el camarero te trae el tamaño normal. Una función también puede
prescindir de algunos argumentos.

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

Línea por línea:

- `saludar("Alice")` da un solo argumento: `nombre` vale `"Alice"` y `titulo` vale
  `undefined`;
- por eso la prueba `titulo === undefined` es verdadera: la función devuelve la frase
  sin título;
- `saludar("Martín", "doctor")` da los dos: la prueba es falsa, y la función devuelve
  la frase con el título.

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

**Te toca:** escribe una función `pedir` que reciba el nombre de una bebida y un
tamaño, con `"mediano"` como tamaño por defecto. Devuelve, por ejemplo,
`"Un café mediano"`. Muestra `pedir("café")` y `pedir("té", "grande")`.

> **Solución**
>
> ```typescript
> function pedir(bebida: string, tamano = "mediano"): string {
>   return `Un ${bebida} ${tamano}`;
> }
>
> console.log(pedir("café"));
> console.log(pedir("té", "grande"));
> ```
>
> ```sortie
> Un café mediano
> Un té grande
> ```

## Las funciones flecha

Muchas funciones son muy pequeñas: una sola línea de cálculo. Escribir `function`,
llaves y `return` para tan poco es largo. La función flecha es un atajo para estos
casos.

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

Línea por línea:

- `cuadrado` es una constante que contiene una función: recibe `x` y devuelve `x * x`;
- `presentar` contiene una función más larga, entre llaves: pone el nombre en
  mayúsculas y luego devuelve la frase;
- `cuadrado(4)` se llama como una función normal y devuelve 16;
- `presentar("lucía")` devuelve `"Soy LUCÍA."`.

- Con una sola expresión después de `=>`, no hacen falta llaves ni `return`: el
  valor se devuelve automáticamente.
- Con varias líneas, se ponen llaves y un `return`.

Una función sin nombre, como esta, también se llama
[lambda](../debuter/glossaire.md#lambda). Las funciones flecha están por todas partes en
TypeScript, sobre todo como *callbacks* (en el capítulo siguiente).

**Te toca:** escribe una función flecha `triple`, guardada en una constante, que
reciba un número y devuelva ese número multiplicado por 3. Muestra `triple(7)`.

> **Solución**
>
> ```typescript
> const triple = (x: number): number => x * 3;
>
> console.log(triple(7));
> ```
>
> ```sortie
> 21
> ```

## El ámbito de las variables

Cada función tiene su propia mesa de trabajo. Lo que pone en ella se queda con ella:
así una función no molesta por error a las variables del resto del programa.

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

Línea por línea:

- `tasaIva` se crea arriba del todo, fuera de la función: vale 0.2;
- `precioConIva(19.99)` ejecuta la función: `precio` vale 19.99;
- `total` se crea **dentro** de la función y vale unos 23.988;
- `Math.round(total * 100) / 100` redondea a dos decimales: la función devuelve
  23.99.

- La función ve `tasaIva`, creada **fuera**, por encima de ella.
- Fuera, `total` no existe: `console.log(total)` se rechazaría.

> **Consejo:** una función es más fácil de entender cuando recibe todo lo que
> necesita como parámetros.

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

**Pregunta 2:** ¿qué diferencia hay entre `titulo?: string` y `titulo = "Sr."`?

> **Respuesta**
>
> Los dos hacen que el parámetro sea opcional. Con `?`, vale `undefined` cuando
> falta: hay que comprobar ese caso antes de usarlo. Con `= "Sr."`, vale `"Sr."`
> cuando falta: siempre es un `string`.

**Pregunta 3:** una variable `total` se crea dentro de una función. Después de la llamada,
¿se puede escribir `console.log(total)` fuera de la función?

> **Respuesta**
>
> No. El ámbito de `total` es la función: fuera de ella no existe, y TypeScript
> lo rechaza (*Cannot find name 'total'*). Para recuperar el valor, la función debe
> devolverlo con `return`.

## En resumen

- Una función se escribe `function nombre(param: tipo): tipoRetorno { ... }` y se llama
  con `nombre(...)`.
- Los parámetros siempre deben estar tipados; el tipo de retorno se puede adivinar,
  pero escribirlo ayuda.
- `?` hace que un parámetro sea opcional (entonces vale `undefined`), `= valor` le da
  un valor por defecto.
- Una función flecha se escribe `(x: number) => x * 2`; `void` quiere decir «no devuelve
  nada».
- Los parámetros y las variables creadas dentro de una función solo existen en esa
  función: es su ámbito.

Siguiente: [más allá de las funciones](fonctions-avancees.md).
