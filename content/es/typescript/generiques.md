# Los genéricos

Esta página explica cómo escribir una función o una clase que funcione con cualquier tipo, sin perder la precisión de los tipos.

> **Objetivos:**
>
> - sabrás escribir una función genérica con `<T>`;
> - sabrás dejar que TypeScript adivine el tipo, o indicarlo tú mismo;
> - sabrás limitar un genérico con `extends`;
> - sabrás escribir interfaces y clases genéricas como `Caja<T>`;
> - sabrás relacionar dos parámetros con `K extends keyof T`.

## El problema

Imagina una función que devuelve el primer elemento de un array. Para los números,
escribirías:

```typescript
function primerNumero(lista: number[]): number | undefined {
  return lista[0];
}

console.log(primerNumero([10, 20, 30]));
```

```sortie
10
```

¿Y para los textos? Habría que copiar la función cambiando `number` por `string`.
Y otra vez para los booleanos… Con `any` bastaría una sola función, pero TypeScript
ya no sabría nada del resultado: ya no podría protegerte.

Los [genéricos](../debuter/glossaire.md#generique) resuelven este problema: una sola
función, y tipos que siguen siendo precisos.

## Una función genérica

Un genérico es un **parámetro de tipo**. Se escribe entre corchetes angulares `<T>`,
justo después del nombre de la función. `T` es un nombre de tipo «por rellenar más
tarde», igual que un parámetro normal es un valor por rellenar más tarde.

```typescript
function primero<T>(lista: T[]): T | undefined {
  return lista[0];
}

const n = primero([10, 20, 30]);
const t = primero(["manzana", "pera"]);

console.log(n);
console.log(t?.toUpperCase());
```

```sortie
10
MANZANA
```

Lee la firma así: «para un tipo `T` cualquiera, `primero` recibe un array de `T` y
devuelve un `T` (o `undefined`)». Es como una etiqueta en blanco en un tarro: escribes
en ella «mermelada» o «pepinillos» en el momento de llenarlo.

> **Nota:** `T` es una convención (por *Type*, tipo). Puedes elegir otro nombre, como
> `Elemento`. Con varios parámetros, se suelen usar `T`, `U`, `K`, `V`.

## La inferencia

En el ejemplo, nunca escribiste cuál era `T`. TypeScript lo ha **inferido**
(adivinado) a partir de los argumentos: `[10, 20, 30]` es un `number[]`, así que `T`
vale `number`, así que `n` es un `number | undefined`.

También puedes indicar `T` tú mismo, entre corchetes angulares en la llamada:

```typescript
function envolver<T>(valor: T): T[] {
  return [valor];
}

const a = envolver(5);
const b = envolver<string | number>("cinco");
b.push(5);

console.log(a, b);
```

```sortie
[ 5 ] [ 'cinco', 5 ]
```

Sin `<string | number>`, `b` sería un `string[]`, y `b.push(5)` sería rechazado.
En general, deja que TypeScript adivine; indícalo solo cuando adivine algo demasiado
estrecho.

## Varios parámetros de tipo

Una función puede tener varios genéricos. Aquí, `par` junta dos valores de tipos
distintos:

```typescript
function par<A, B>(a: A, b: B): [A, B] {
  return [a, b];
}

function transformar<T, U>(lista: T[], f: (x: T) => U): U[] {
  const resultado: U[] = [];
  for (const x of lista) {
    resultado.push(f(x));
  }
  return resultado;
}

console.log(par("edad", 12));
console.log(transformar(["a", "bb", "ccc"], (palabra) => palabra.length));
```

```sortie
[ 'edad', 12 ]
[ 1, 2, 3 ]
```

En `transformar`, TypeScript deduce `T = string` a partir del array, y luego
`U = number` a partir de lo que devuelve la función. Así es exactamente como está
tipado el método `map` de los arrays.

## Las restricciones con extends

A veces, `T` no puede ser **cualquier cosa**. Para leer `.length`, el valor tiene que
tener una longitud. `T extends Restriccion` quiere decir: «`T` puede ser cualquier
tipo, **siempre que** respete la restricción».

```typescript
function masLargo<T extends { length: number }>(a: T, b: T): T {
  return a.length >= b.length ? a : b;
}

console.log(masLargo("gato", "elefante"));
console.log(masLargo([1, 2, 3], [4]));
```

```sortie
elefante
[ 1, 2, 3 ]
```

Los textos y los arrays tienen todos un `length`: se aceptan. Un número sería
rechazado. Y el resultado conserva el tipo exacto: `string` en la primera llamada,
`number[]` en la segunda.

Una restricción también puede ser una interfaz:

```typescript
interface ConNombre {
  nombre: string;
}

function ordenarPorNombre<T extends ConNombre>(lista: T[]): T[] {
  return [...lista].sort((a, b) => a.nombre.localeCompare(b.nombre));
}

const alumnos = [
  { nombre: "Zoé", nota: 15 },
  { nombre: "Adam", nota: 12 },
];

for (const e of ordenarPorNombre(alumnos)) {
  console.log(`${e.nombre}: ${e.nota}`);
}
```

```sortie
Adam: 12
Zoé: 15
```

Fíjate en que `e.nota` está permitido: la función devuelve `T`, es decir, objetos con
`nombre` **y** `nota`, no solo `ConNombre`.

## Las interfaces y los tipos genéricos

Un tipo también puede tener un parámetro. `Caja<T>` describe una caja que contiene un
valor de tipo `T`:

```typescript
interface Caja<T> {
  contenido: T;
  etiqueta: string;
}

const cajaDeNumero: Caja<number> = { contenido: 42, etiqueta: "respuesta" };
const cajaDeLista: Caja<string[]> = { contenido: ["a", "b"], etiqueta: "letras" };

console.log(cajaDeNumero.contenido + 1);
console.log(cajaDeLista.contenido.join(", "));
```

```sortie
43
a, b
```

Ya los usas sin saberlo: `Array<number>` es la otra forma de escribir `number[]`, y
`Promise<string>` es una promesa que dará un texto (consulta
[la asincronía](asynchrone.md)).

## Las clases genéricas

Una clase puede tener un parámetro de tipo. Aquí tienes una **pila**: como una pila
de platos, se pone encima y se retira de encima.

```typescript
class Pila<T> {
  private elementos: T[] = [];

  apilar(x: T): void {
    this.elementos.push(x);
  }

  desapilar(): T | undefined {
    return this.elementos.pop();
  }

  get tamano(): number {
    return this.elementos.length;
  }
}

const platos = new Pila<string>();
platos.apilar("azul");
platos.apilar("rojo");
console.log(platos.desapilar());
console.log(platos.tamano);

const numeros = new Pila<number>();
numeros.apilar(7);
console.log(numeros.desapilar()! * 2);
```

```sortie
rojo
1
14
```

`Pila<string>` solo acepta textos, `Pila<number>` solo números. Una sola clase, y cada
pila está protegida contra las mezclas.

> **Nota:** el `!` después de `desapilar()` le dice a TypeScript «sé que no es
> `undefined`». Úsalo solo cuando estés seguro, como aquí.

## Los tipos por defecto

Un parámetro de tipo puede tener un valor por defecto, como un parámetro de función:
`<T = string>`. Si nadie indica `T`, vale `string`.

```typescript
interface Respuesta<T = string> {
  codigo: number;
  datos: T;
}

const texto: Respuesta = { codigo: 200, datos: "OK" };
const lista: Respuesta<number[]> = { codigo: 200, datos: [1, 2, 3] };

console.log(texto.datos.toLowerCase());
console.log(lista.datos.length);
```

```sortie
ok
3
```

## Relacionar dos parámetros con keyof

`keyof T` es la unión de los nombres de las propiedades de `T`. Para
`{ nombre: string; edad: number }`, es `"nombre" | "edad"`. Con `K extends keyof T`,
dices: «`K` tiene que ser una de las claves de `T`».

```typescript
function getProp<T, K extends keyof T>(objeto: T, clave: K): T[K] {
  return objeto[clave];
}

const gato = { nombre: "Misi", edad: 3, goloso: true };

const nombre = getProp(gato, "nombre");
const edad = getProp(gato, "edad");

console.log(nombre.toUpperCase());
console.log(edad + 1);
```

```sortie
MISI
4
```

El tipo de retorno `T[K]` se lee «el tipo de la propiedad `K` en `T`». Para
`"nombre"`, es `string`; para `"edad"`, es `number`. Y una clave que no existe, como
`"color"`, se rechaza antes incluso de la ejecución.

## Errores frecuentes

### Usar una propiedad sin restricción

```typescript norun
function longitud<T>(x: T): number {
  return x.length;
}
```

```text
main.ts(2,12): error TS2339: Property 'length' does not exist on type 'T'.
```

`T` puede ser cualquier cosa, incluso un número, que no tiene `length`. Añade una
restricción: `function longitud<T extends { length: number }>(x: T)`.

### Pedir una clave que no existe

```typescript norun
function getProp<T, K extends keyof T>(objeto: T, clave: K): T[K] {
  return objeto[clave];
}
const gato = { nombre: "Misi", edad: 3 };
console.log(getProp(gato, "color"));
```

```text
main.ts(5,27): error TS2345: Argument of type '"color"' is not assignable to parameter of type '"nombre" | "edad"'.
```

El mensaje da la lista de claves permitidas: `"nombre" | "edad"`. Corrige la errata,
o añade la propiedad al objeto.

### Mezclar tipos en una clase genérica

```typescript norun
class Pila<T> {
  private elementos: T[] = [];
  apilar(x: T) {
    this.elementos.push(x);
  }
}
const pila = new Pila<number>();
pila.apilar("diez");
```

```text
main.ts(8,13): error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.
```

La pila se creó para números: `T` vale `number` en todas partes. Apila `10`, o crea
una `Pila<string>` para los textos.

## Ponte a prueba

**Pregunta 1:** ¿cuál es el tipo de `x`?

```typescript norun
function ultimo<T>(lista: T[]): T | undefined {
  return lista[lista.length - 1];
}
const x = ultimo([true, false]);
```

> **Respuesta**
>
> `boolean | undefined`. TypeScript infiere `T = boolean` a partir de
> `[true, false]`.

**Pregunta 2:** ¿qué quiere decir `<T extends { id: number }>`?

> **Respuesta**
>
> `T` puede ser cualquier tipo, con la condición de tener una propiedad `id` de tipo
> `number`. Dentro de la función, puedes por tanto leer `x.id`, y el tipo exacto de
> `T` se conserva.

**Pregunta 3:** escribe una función genérica `repetir<T>(valor: T, n: number)` que
devuelva un array que contenga `n` veces el valor.

> **Respuesta**
>
> ```typescript
> function repetir<T>(valor: T, n: number): T[] {
>   const resultado: T[] = [];
>   for (let i = 0; i < n; i++) {
>     resultado.push(valor);
>   }
>   return resultado;
> }
>
> console.log(repetir("ja", 3));
> console.log(repetir(0, 2));
> ```
>
> ```sortie
> [ 'ja', 'ja', 'ja' ]
> [ 0, 0 ]
> ```

## En resumen

- Un genérico `<T>` es un parámetro de tipo: una sola función para todos los tipos.
- TypeScript infiere `T` a partir de los argumentos; también puedes escribirlo en la
  llamada.
- `T extends Restriccion` limita `T` a los tipos que tienen lo necesario.
- Las interfaces y las clases pueden ser genéricas: `Caja<T>`, `Pila<T>`.
- `<T = string>` da un tipo por defecto.
- `K extends keyof T` y `T[K]` relacionan una clave y el tipo de su valor.

Siguiente: [tipos literales, enums y as const](litteraux.md)
