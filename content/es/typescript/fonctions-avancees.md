# Más allá de las funciones

Esta página muestra cómo una función de TypeScript recibe un número variable de argumentos, cómo escribir el tipo de una función, pasarla a otra función, y qué son las sobrecargas.

> **Objetivos:**
>
> - sabrás escribir el tipo de una función: `(x: number) => number`;
> - sabrás dar un nombre a un tipo de función con `type`;
> - sabrás pasar una función como argumento (un *callback*) y tiparla.

Este capítulo sigue a [las funciones](fonctions.md): volverás a encontrar las funciones flecha, que aquí sirven como valores que se guardan y se pasan a otras funciones.

## Un número variable de argumentos

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

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

Línea por línea:

- `suma(1, 2, 3, 4)` guarda los cuatro argumentos en el array `numeros`:
  `[1, 2, 3, 4]`;
- `total` empieza en 0, y luego el bucle le suma cada número: 1, 3, 6, 10;
- `return total` devuelve 10;
- `suma()` recibe un array vacío: el bucle no da ninguna vuelta, y la función
  devuelve 0.

Cada argumento se comprueba: `suma(1, "2")` se rechazaría.

## El tipo de una función

Un enchufe solo acepta una forma de clavija. El tipo de una función cumple el mismo
papel: dice de antemano qué clase de función acepta una variable o un parámetro.

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

Línea por línea:

- la primera línea crea la variable `operacion`, todavía vacía, y dice qué podrá
  contener: una función que recibe dos números y devuelve un número;
- primero se guarda en ella una función que suma: `operacion(6, 3)` da 9;
- después se reemplaza por una función que multiplica: la misma llamada da 18.

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

Línea por línea:

- `repetir` recibe dos cosas: un número (`veces`) y una función (`accion`);
- en la llamada, `veces` vale 3 y `accion` es la función flecha escrita entre los
  paréntesis;
- el bucle da a `i` los valores 1, 2 y 3, y llama a `accion(i)` en cada vuelta;
- cada llamada guarda `i` en `vuelta` y muestra la frase.

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

**Te toca:** escribe una función `aplicar` que reciba un número y una función de tipo
`(n: number) => number`, y devuelva el resultado de esa función sobre el número.
Muestra `aplicar(5, (n) => n + 100)`.

> **Solución**
>
> ```typescript
> function aplicar(x: number, f: (n: number) => number): number {
>   return f(x);
> }
>
> console.log(aplicar(5, (n) => n + 100));
> ```
>
> ```sortie
> 105
> ```

## Las sobrecargas, en pocas palabras

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

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

### Poner el parámetro rest antes de otro

```typescript norun
function invitar(...nombres: string[], sala: string): void {
  console.log(`${nombres.join(", ")}: sala ${sala}`);
}
invitar("Alice", "Bob", "B12");
```

```text
main.ts:1:18 - error TS1014: A rest parameter must be last in a parameter list.

1 function invitar(...nombres: string[], sala: string): void {
                   ~~~

main.ts:4:25 - error TS2554: Expected 2 arguments, but got 3.

4 invitar("Alice", "Bob", "B12");
                          ~~~~~


Found 2 errors in the same file, starting at: main.ts:1
```

«Un parámetro rest debe ser el último de la lista.» Se lleva todos los argumentos
restantes: no quedaría ninguno para `sala`. El segundo error viene del primero.
Ponlo al final: `function invitar(sala: string, ...nombres: string[])`, y luego llama a
`invitar("B12", "Alice", "Bob")`.

### Pasar un callback del tipo incorrecto

```typescript norun
function repetir(veces: number, accion: (vuelta: number) => void): void {
  for (let i = 1; i <= veces; i++) {
    accion(i);
  }
}

repetir(3, (vuelta: string) => {
  console.log(`Vuelta número ${vuelta}`);
});
```

```text
main.ts:7:12 - error TS2345: Argument of type '(vuelta: string) => void' is not assignable to parameter of type '(vuelta: number) => void'.
  Types of parameters 'vuelta' and 'vuelta' are incompatible.
    Type 'number' is not assignable to type 'string'.

7 repetir(3, (vuelta: string) => {
             ~~~~~~~~~~~~~~~~~~~~~


Found 1 error in main.ts:7
```

`repetir` llama a `accion` con un número: una función que espera un texto no sirve
(*types of parameters are incompatible*). Quita el tipo del parámetro: con
`(vuelta) => { ... }`, TypeScript adivina solo que `vuelta` es un `number`.

### Llamar a una sobrecarga que no existe

```typescript norun
function duplicar(x: number): number;
function duplicar(x: string): string;
function duplicar(x: number | string): number | string {
  return typeof x === "number" ? x * 2 : x + x;
}

console.log(duplicar(true));
```

```text
main.ts:7:22 - error TS2769: No overload matches this call.
  Overload 1 of 2, '(x: number): number', gave the following error.
    Argument of type 'boolean' is not assignable to parameter of type 'number'.
  Overload 2 of 2, '(x: string): string', gave the following error.
    Argument of type 'boolean' is not assignable to parameter of type 'string'.

7 console.log(duplicar(true));
                       ~~~~


Found 1 error in main.ts:7
```

«Ninguna sobrecarga corresponde a esta llamada.» TypeScript prueba cada forma permitida
y dice por qué falla cada una: `true` no es ni un `number` ni un `string`. Pasa un
número o un texto: `duplicar(21)`.

## Ponte a prueba

**Pregunta 1:** ¿cuántos argumentos se pueden dar a `function f(...palabras: string[])`? ¿Qué
vale `palabras` en la llamada `f()`?

> **Respuesta**
>
> Tantos como quieras, incluso ninguno: cada uno debe ser un `string`. Con `f()`,
> `palabras` es un array vacío, `[]`.

**Pregunta 2:** ¿qué quiere decir el tipo `(texto: string) => boolean`?

> **Respuesta**
>
> Es el tipo de una función que recibe un texto y devuelve `true` o `false`. Por
> ejemplo: `(texto) => texto.length > 3`.

**Pregunta 3:** escribe una función `dosVeces` que reciba un callback de tipo `() => void` y
lo llame dos veces.

> **Respuesta**
>
> ```typescript
> function dosVeces(accion: () => void): void {
>   accion();
>   accion();
> }
>
> dosVeces(() => console.log("Hola"));
> ```
>
> ```sortie
> Hola
> Hola
> ```

## En resumen

- `...nombre: T[]` recibe todos los argumentos restantes en un array; va al final.
- El tipo de una función se escribe `(x: number) => number`; `type` le da un nombre.
- Un callback es una función pasada como argumento; gracias al tipo del parámetro, sus
  propios parámetros se adivinan.
- Las sobrecargas enumeran varias formas de llamada; una unión suele bastar.

Siguiente: [trabajar con texto](texte.md).
