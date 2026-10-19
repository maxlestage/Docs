# Objetos, interfaces y alias de tipo

Esta página explica cómo agrupar valores en objetos, describir su forma con `interface` y `type`, y cómo TypeScript compara los objetos según su forma.

> **Objetivos:**
>
> - sabrás crear un objeto y leer sus propiedades;
> - sabrás describir la forma de un objeto con `interface` o `type`;
> - sabrás usar las propiedades opcionales (`?`) y de solo lectura
>   (`readonly`);
> - sabrás extender una interfaz y escribir una firma de índice;
> - sabrás explicar el tipado estructural y la comprobación de las propiedades de
>   más.

## Crear un objeto

Un [objeto](../debuter/glossaire.md#objeto) agrupa varios valores bajo un mismo nombre.
Cada valor tiene una etiqueta: su **propiedad**. Se escribe entre llaves, con pares
`nombre: valor`.

```typescript
const libro = {
  titulo: "Dune",
  autor: "Frank Herbert",
  paginas: 600,
};

console.log(libro.titulo);
console.log(libro["paginas"]);
libro.paginas = 612;
console.log(libro);
```

```sortie
Dune
600
{ titulo: 'Dune', autor: 'Frank Herbert', paginas: 612 }
```

- `libro.titulo` lee una propiedad con un punto;
- `libro["paginas"]` hace lo mismo con corchetes y un texto;
- `libro.paginas = 612` modifica una propiedad.

Un objeto es como una ficha de inscripción: cada casilla tiene una etiqueta
(`titulo`, `paginas`) y un contenido.

TypeScript adivina el tipo de este objeto: `{ titulo: string; autor: string; paginas:
number }`. Así que conoce cada propiedad y su tipo. `libro.paginas = "muchas"` sería
rechazado, y también una errata como `libro.titulos`.

## Describir un objeto: el tipo objeto

Para una función que recibe un objeto, hay que escribir su forma. Se describe entre
llaves, como un objeto, pero con tipos en lugar de valores:

```typescript
function presentar(persona: { nombre: string; edad: number }): string {
  return `${persona.nombre}, ${persona.edad} años`;
}

console.log(presentar({ nombre: "Alicia", edad: 30 }));
```

```sortie
Alicia, 30 años
```

Escribir esta forma cada vez sería pesado. Por eso se le da un nombre, de dos maneras
posibles: `interface` o `type`.

## interface y type

### interface

Una **interfaz** da un nombre a una forma de objeto. Es un plano, un modelo de ficha:
dice qué casillas deben existir y qué contienen.

```typescript
interface Persona {
  nombre: string;
  edad: number;
}

const alicia: Persona = { nombre: "Alicia", edad: 30 };
const bruno: Persona = { nombre: "Bruno", edad: 25 };

function presentar(p: Persona): string {
  return `${p.nombre}, ${p.edad} años`;
}

console.log(presentar(alicia));
console.log(presentar(bruno));
```

```sortie
Alicia, 30 años
Bruno, 25 años
```

Por convención, el nombre de una interfaz empieza por mayúscula.

### type: el alias de tipo

La palabra `type` crea un **alias de tipo**: un nombre para cualquier tipo. Ya lo has
visto para las uniones (`type Respuesta = "sí" | "no"`). También sirve para un objeto:

```typescript
type Producto = {
  nombre: string;
  precio: number;
};

const cafe: Producto = { nombre: "Café", precio: 2.5 };
console.log(`${cafe.nombre}: ${cafe.precio} €`);
```

```sortie
Café: 2.5 €
```

### ¿Cuál elegir?

Para describir un objeto, los dos valen casi siempre lo mismo. La diferencia:

| | `interface` | `type` |
|---|---|---|
| Describir un objeto | sí | sí |
| Describir una unión, una tupla, una función | no | sí |
| Extender con `extends` | sí | con `&` |

> **Recuerda:** una regla sencilla para empezar: `interface` para los objetos, `type`
> para todo lo demás (uniones, tuplas, funciones). Sobre todo, sé coherente dentro de
> un mismo proyecto.

## Las propiedades opcionales

Un `?` después del nombre hace que una propiedad sea opcional: el objeto puede tenerla
o no. Su tipo pasa a ser `string | undefined`.

```typescript
interface Usuario {
  nombre: string;
  email?: string;
}

const alicia: Usuario = { nombre: "Alicia", email: "alicia@ejemplo.es" };
const bruno: Usuario = { nombre: "Bruno" };

for (const u of [alicia, bruno]) {
  console.log(`${u.nombre}: ${u.email ?? "sin dirección"}`);
}
```

```sortie
Alicia: alicia@ejemplo.es
Bruno: sin dirección
```

Antes de usar `u.email` como un texto, TypeScript te obliga a tratar el caso en que
falte: con `??`, `?.` o un `if`.

## Las propiedades de solo lectura

`readonly` delante de una propiedad impide modificarla después de crear el objeto.

```typescript
interface Punto {
  readonly x: number;
  readonly y: number;
}

const salida: Punto = { x: 0, y: 0 };
const llegada: Punto = { x: salida.x + 3, y: salida.y + 4 };

console.log(llegada);
```

```sortie
{ x: 3, y: 4 }
```

Para «mover» un punto, se crea uno nuevo. Esta comprobación solo existe en
TypeScript: al ejecutar el programa, el objeto es un objeto JavaScript normal.

> **Consejo:** marca como `readonly` las propiedades que nunca deben cambiar, como un
> identificador. TypeScript las vigilará por ti.

## Los objetos anidados y los métodos

Una propiedad puede contener otro objeto, un array o una función. Una función guardada
en un objeto se llama [método](../debuter/glossaire.md#metodo).

```typescript
interface Direccion {
  ciudad: string;
  codigoPostal: string;
}

interface Cliente {
  nombre: string;
  direccion: Direccion;
  pedidos: number[];
  saludar(): string;
}

const cliente: Cliente = {
  nombre: "Clara",
  direccion: { ciudad: "Sevilla", codigoPostal: "41001" },
  pedidos: [25, 40],
  saludar() {
    return `¡Hola, ${this.nombre}!`;
  },
};

console.log(cliente.direccion.ciudad);
console.log(cliente.pedidos.length);
console.log(cliente.saludar());
```

```sortie
Sevilla
2
¡Hola, Clara!
```

- `saludar(): string` en la interfaz: un método sin parámetros que devuelve un texto;
- en el objeto, `this` designa al propio objeto: `this.nombre` vale `"Clara"`.

## Extender una interfaz

`extends` («extiende») crea una interfaz que toma todas las propiedades de otra y
añade algunas. Es como una ficha «Alumno» que toma la ficha «Persona» y añade el
curso.

```typescript
interface Persona {
  nombre: string;
  edad: number;
}

interface Alumno extends Persona {
  curso: string;
}

const lea: Alumno = { nombre: "Lea", edad: 15, curso: "4.º B" };

function presentar(p: Persona): string {
  return `${p.nombre} (${p.edad} años)`;
}

console.log(presentar(lea));
console.log(lea.curso);
```

```sortie
Lea (15 años)
4.º B
```

Un `Alumno` tiene todo lo que tiene una `Persona`: por eso se le puede pasar a
`presentar`.

Con `type`, se obtiene el mismo resultado con `&` («y»):

```typescript
type Animal = { nombre: string };
type Perro = Animal & { raza: string };

const rex: Perro = { nombre: "Rex", raza: "pastor" };
console.log(rex.nombre, rex.raza);
```

```sortie
Rex pastor
```

## Las firmas de índice

A veces no se conocen de antemano los nombres de las propiedades: notas por
asignatura, precios por artículo… Una **firma de índice** describe «cualquier clave de
este tipo, con un valor de este tipo».

```typescript
interface Notas {
  [asignatura: string]: number;
}

const notas: Notas = { mates: 15, lengua: 12 };
notas.ingles = 14;

for (const [asignatura, nota] of Object.entries(notas)) {
  console.log(`${asignatura}: ${nota}`);
}
```

```sortie
mates: 15
lengua: 12
ingles: 14
```

`[asignatura: string]: number` se lee: «para cualquier clave de texto, un valor
numérico». La palabra `asignatura` es solo un nombre para la documentación. Es lo
mismo que `Record<string, number>`, visto en [los arrays](collections.md#record-un-objeto-clave-valor).

> **Atención:** TypeScript anuncia `notas.historia` como un `number`, pero la clave no
> existe: el valor es `undefined`. Con una firma de índice, comprueba que una clave
> existe (`"historia" in notas`) antes de fiarte de su valor.

## El tipado estructural: solo cuenta la forma

Esta es una idea importante, propia de TypeScript. Para saber si un objeto sirve,
TypeScript no mira **su nombre**, sino **su forma**: sus propiedades y sus tipos. Se
habla de **tipado estructural**.

Piensa en una analogía: un enchufe. Acepta cualquier clavija que tenga la forma
correcta, sea cual sea la marca del aparato. Del mismo modo, una función que espera un
`Punto` acepta cualquier objeto que tenga un `x` y un `y` numéricos.

```typescript
interface Punto {
  x: number;
  y: number;
}

function distanciaAlOrigen(p: Punto): number {
  return Math.sqrt(p.x ** 2 + p.y ** 2);
}

const pixel = { x: 3, y: 4, color: "rojo" };

console.log(distanciaAlOrigen(pixel));
console.log(distanciaAlOrigen({ x: 6, y: 8 }));
```

```sortie
5
10
```

`pixel` nunca se ha declarado como un `Punto`. Incluso tiene una propiedad de más,
`color`. Pero tiene `x` e `y`, que son números: tiene la forma de un `Punto`, así que
se acepta. La clavija entra en el enchufe.

> **Nota:** otros lenguajes, como Java, piden declarar explícitamente que un objeto
> «es un» `Punto`. TypeScript no: basta con tener la forma correcta. A veces se dice
> «si camina como un pato y grazna como un pato, es un pato».

## La comprobación de las propiedades de más

Una excepción a la regla anterior: cuando escribes un objeto **directamente** en el
lugar donde se espera un tipo, TypeScript rechaza las propiedades de más.

```typescript norun
interface Libro {
  titulo: string;
  paginas: number;
}
const libro: Libro = { titulo: "Dune", paginas: 600, autor: "Herbert" };
```

```text
main.ts:5:54 - error TS2353: Object literal may only specify known properties, and 'autor' does not exist in type 'Libro'.

5 const libro: Libro = { titulo: "Dune", paginas: 600, autor: "Herbert" };
                                                       ~~~~~


Found 1 error in main.ts:5
```

«Un objeto escrito así (*object literal*) solo puede tener propiedades conocidas, y
`autor` no existe en `Libro`.»

¿Por qué esta diferencia con `pixel`? Un objeto escrito en el sitio solo sirve en ese
lugar: una propiedad desconocida ahí es casi siempre una errata (`titulos` en lugar de
`titulo`) o una propiedad olvidada en la interfaz. TypeScript prefiere avisarte. Un
objeto ya guardado en una variable, en cambio, puede servir en otro sitio con sus
otras propiedades: se acepta.

Las dos correcciones posibles:

- quita la propiedad de más;
- o añádela a la interfaz, si tiene sentido: `autor?: string;`.

```typescript
interface Libro {
  titulo: string;
  paginas: number;
  autor?: string;
}

const libro: Libro = { titulo: "Dune", paginas: 600, autor: "Herbert" };
console.log(`${libro.titulo}, de ${libro.autor ?? "autor desconocido"}`);
```

```sortie
Dune, de Herbert
```

## Errores frecuentes

### Olvidar una propiedad obligatoria

```typescript norun
interface Libro {
  titulo: string;
  paginas: number;
}
const libro: Libro = { titulo: "Dune" };
```

```text
main.ts:5:7 - error TS2741: Property 'paginas' is missing in type '{ titulo: string; }' but required in type 'Libro'.

5 const libro: Libro = { titulo: "Dune" };
        ~~~~~

  main.ts:3:3
    3   paginas: number;
        ~~~~~~~
    'paginas' is declared here.


Found 1 error in main.ts:5
```

«Falta la propiedad `paginas`, pero `Libro` la exige.» Añádela
(`{ titulo: "Dune", paginas: 600 }`), o hazla opcional en la interfaz
(`paginas?: number`).

### Equivocarse de nombre de propiedad

```typescript norun
interface Libro {
  titulo: string;
  paginas: number;
}
const libro: Libro = { titulo: "Dune", paginas: 600 };
console.log(libro.titulos);
```

```text
main.ts:6:19 - error TS2551: Property 'titulos' does not exist on type 'Libro'. Did you mean 'titulo'?

6 console.log(libro.titulos);
                    ~~~~~~~

  main.ts:2:3
    2   titulo: string;
        ~~~~~~
    'titulo' is declared here.


Found 1 error in main.ts:6
```

TypeScript conoce todas las propiedades de `Libro` y propone la correcta:
`libro.titulo`. En JavaScript solo, `libro.titulos` habría dado `undefined` sin decir
nada.

### Usar una propiedad opcional sin comprobarla

```typescript norun
interface Usuario {
  nombre: string;
  email?: string;
}
const u: Usuario = { nombre: "Alicia" };
console.log(u.email.toLowerCase());
```

```text
main.ts:6:13 - error TS18048: 'u.email' is possibly 'undefined'.

6 console.log(u.email.toLowerCase());
              ~~~~~~~


Found 1 error in main.ts:6
```

`email` es opcional: puede faltar. Escribe `u.email?.toLowerCase()`, o comprueba
`if (u.email !== undefined)` antes.

### Modificar una propiedad readonly

```typescript norun
interface Punto {
  readonly x: number;
  readonly y: number;
}
const p: Punto = { x: 1, y: 2 };
p.x = 5;
```

```text
main.ts:6:3 - error TS2540: Cannot assign to 'x' because it is a read-only property.

6 p.x = 5;
    ~


Found 1 error in main.ts:6
```

`x` es de solo lectura. Crea un objeto nuevo: `const q: Punto = { ...p, x: 5 };`.
Los `...p` copian todas las propiedades de `p`, y luego `x: 5` reemplaza `x`.

## Ponte a prueba

**Pregunta 1:** escribe una interfaz `Pelicula` con un `titulo` (texto), un `anio`
(número) y una `nota` opcional (número), y luego una película sin nota.

> **Respuesta**
>
> ```typescript
> interface Pelicula {
>   titulo: string;
>   anio: number;
>   nota?: number;
> }
>
> const pelicula: Pelicula = { titulo: "El viaje de Chihiro", anio: 2001 };
> console.log(pelicula.nota ?? "todavía sin nota");
> ```
>
> ```sortie
> todavía sin nota
> ```

**Pregunta 2:** una función espera un `{ nombre: string }`. ¿Se le puede pasar una
variable `const gato = { nombre: "Félix", edad: 3 }`?

> **Respuesta**
>
> Sí. Gracias al tipado estructural, solo cuenta la forma: `gato` tiene un `nombre`
> de texto. La propiedad `edad` de más no molesta, porque el objeto se pasa a través
> de una variable y no se escribe directamente en la llamada.

**Pregunta 3:** ¿qué diferencia hay entre `interface Alumno extends Persona` y
`type Alumno = Persona & { curso: string }`?

> **Respuesta**
>
> El resultado es el mismo: un `Alumno` tiene todas las propiedades de `Persona`, más
> `curso`. `extends` se usa con `interface`, `&` con `type`.

## En resumen

- Un objeto agrupa valores con nombre: `{ nombre: "Alicia", edad: 30 }`; una propiedad
  se lee con `objeto.nombre`.
- `interface` y `type` dan un nombre a la forma de un objeto; `interface` para los
  objetos y `type` para el resto es una buena regla para empezar.
- `?` hace que una propiedad sea opcional (compruébala antes de usarla), `readonly`
  impide que cambie.
- `extends` (o `&`) extiende una forma; `[clave: string]: number` acepta cualquier
  clave.
- El tipado es estructural: un objeto sirve si tiene la forma correcta, incluso con
  propiedades de más, salvo si se escribe directamente en ese lugar.

Siguiente: [seguir un programa paso a paso](pas-a-pas.md).
