# Las trampas del principiante en TypeScript

Esta página reúne doce trampas que sorprenden a casi todos los principiantes en TypeScript: para cada una, lo que esperas, lo que pasa de verdad, por qué, y cómo evitarla.

> **Objetivos:**
>
> - sabrás por qué `any` y `as` hacen callar a TypeScript, y cuándo es peligroso;
> - sabrás que los tipos desaparecen durante la ejecución;
> - sabrás desconfiar de las casillas de array vacías, de los textos que parecen
>   números y de `||` con `0`;
> - sabrás distinguir `object` y `{}`;
> - entenderás las sorpresas del tipado estructural y de los `enum`.

## Antes de empezar: TypeScript comprueba y luego se retira

Ten una imagen en la cabeza. TypeScript es un **inspector** a la entrada de una
fábrica. Revisa tu código **antes** de ejecutarlo. Después, `tsc` quita todos los
tipos, y lo que se ejecuta es JavaScript normal.

```text
 tu código .ts ──► [ tsc comprueba los tipos ] ──► código .js sin tipos ──► node lo ejecuta
                         el inspector                    el inspector ya no está
```

Muchas trampas vienen de ahí: el inspector solo ve lo que le enseñas, y ya no está
cuando el programa se ejecuta.

## Las trampas

### 1. any desconecta toda la comprobación

**Piensas:** «TypeScript me avisará si me equivoco».

```typescript norun
let valor: any = 42;
console.log(valor.toUpperCase());
```

`tsc` no dice nada. Pero el programa falla al ejecutarse (extracto del mensaje):

```text
TypeError: valor.toUpperCase is not a function
```

**Por qué:** [`any`](../debuter/glossaire.md#any) significa «no compruebes nada». El
inspector cierra los ojos. Aquí, `valor` es un número, y un número no tiene
`toUpperCase`.

**La corrección:** usa `unknown`. Te obliga a comprobar antes de usar el valor.

```typescript
const valor: unknown = 42;

if (typeof valor === "string") {
  console.log(valor.toUpperCase());
} else {
  console.log("No es un texto:", valor);
}
```

```sortie
No es un texto: 42
```

> **Recuerda:** cada `any` es un agujero en la red. Prefiere `unknown`.

### 2. as le miente a TypeScript

**Piensas:** «`as Usuario` convierte mi objeto en un usuario».

```typescript norun
interface Usuario {
  nombre: string;
}
const u = {} as Usuario;
console.log(u.nombre.toUpperCase());
```

`tsc` lo acepta. Al ejecutarlo:

```text
TypeError: Cannot read properties of undefined (reading 'toUpperCase')
```

**Por qué:** `as` no transforma nada. Es una **promesa** hecha al inspector: «créeme,
es un `Usuario`». El inspector te cree. Pero el objeto está vacío: `u.nombre` vale
`undefined`.

**La corrección:** construye un objeto de verdad, completo. Así TypeScript comprobará
que no falta nada.

```typescript
interface Usuario {
  nombre: string;
}
const u: Usuario = { nombre: "Alicia" };
console.log(u.nombre.toUpperCase());
```

```sortie
ALICIA
```

> **Pruébalo:** quita `nombre: "Alicia"`. Esta vez, `tsc` lo rechaza: falta la
> propiedad `nombre`. Con `: Usuario`, el inspector trabaja. Con `as`, se calla.

### 3. Los tipos desaparecen durante la ejecución (JSON.parse)

**Piensas:** «He escrito `precio: number`, así que `precio` es un número».

```typescript
interface Producto {
  nombre: string;
  precio: number;
}

const texto = '{"nombre": "Café", "precio": "2.5"}';
const producto: Producto = JSON.parse(texto);

console.log(producto.precio + 1);
console.log(typeof producto.precio);
```

```sortie
2.51
string
```

**Por qué:** `JSON.parse` lee un texto en el momento de la ejecución. TypeScript no
puede saber lo que contiene: devuelve `any`. Tu anotación `: Producto` es, por tanto,
una promesa, igual que `as`. Aquí, el [JSON](../debuter/glossaire.md#json) contiene
`"2.5"` entre comillas: un texto. `+ 1` pega `"1"` en vez de sumar.

**La corrección:** comprueba los datos que vienen de fuera y luego conviértelos.

```typescript
const texto = '{"nombre": "Café", "precio": "2.5"}';
const bruto: unknown = JSON.parse(texto);

if (typeof bruto === "object" && bruto !== null && "precio" in bruto) {
  const precio = Number(bruto.precio);
  console.log(precio + 1);
}
```

```sortie
3.5
```

> **Recuerda:** los tipos no existen durante la ejecución. Todo lo que viene de un
> archivo, de un servidor o del teclado debe ser **comprobado**.

### 4. Lo que viene del teclado es un texto

**Piensas:** «El usuario ha escrito 5, le sumo 3, obtengo 8».

```typescript
const entrada = "5";
console.log(entrada + 3);
```

```sortie
53
```

**Por qué:** una entrada (formulario, teclado, línea de comandos) es **siempre** un
texto. Y `texto + número` está permitido en TypeScript: pega los dos. `"5" + 3` da
`"53"`.

**La corrección:** convierte primero con `Number`, mira
[números y cálculos](nombres.md).

```typescript
const entrada = "5";
const n = Number(entrada);
console.log(n + 3);
```

```sortie
8
```

### 5. Una casilla de array puede estar vacía

**Piensas:** «`frutas` es un `string[]`, así que `frutas[2]` es un `string`».

```typescript norun
const frutas = ["manzana", "pera"];
console.log(frutas[2].toUpperCase());
```

`tsc --strict` lo acepta. Al ejecutarlo:

```text
TypeError: Cannot read properties of undefined (reading 'toUpperCase')
```

**Por qué:** el array tiene dos casillas, numeradas 0 y 1. La casilla 2 no existe:
vale `undefined`. Pero para TypeScript, una casilla de un `string[]` es un `string`.
No cuenta las casillas.

```text
frutas:  [ "manzana" | "pera" ]
índice:       0          1       2 → nada (undefined)
```

**La corrección:** prueba la casilla antes de usarla, o usa `?.`.

```typescript
const frutas = ["manzana", "pera"];
const tercera = frutas[2];

if (tercera !== undefined) {
  console.log(tercera.toUpperCase());
} else {
  console.log("No hay tercera fruta.");
}
console.log(frutas[0]?.toUpperCase());
```

```sortie
No hay tercera fruta.
MANZANA
```

> **Nota:** una opción del archivo de configuración, `noUncheckedIndexedAccess`, hace
> que TypeScript sea más prudente: con ella, `frutas[2]` tiene el tipo
> `string | undefined`, y el olvido se convierte en un error (mira
> [Errores frecuentes](#errores-frecuentes)). No está incluida en `--strict`. El
> capítulo [configuración](configuration.md) explica cómo activarla.

### 6. || también reemplaza el 0

**Piensas:** «`stock || "desconocido"` muestra «desconocido» solo si no se indica
ningún stock».

```typescript
interface Articulo {
  nombre: string;
  stock?: number;
}

const articulos: Articulo[] = [
  { nombre: "bolígrafo", stock: 12 },
  { nombre: "goma", stock: 0 },
  { nombre: "regla" },
];

for (const a of articulos) {
  console.log(a.nombre, a.stock || "desconocido");
}
```

```sortie
bolígrafo 12
goma desconocido
regla desconocido
```

**Por qué:** `||` reemplaza todos los valores «falsos»: `undefined`, pero también
`0`, `""` y `false`. Y un stock de 0 es una información real.

**La corrección:** `??` solo reemplaza `null` y `undefined`.

```typescript
const stocks: (number | undefined)[] = [12, 0, undefined];

for (const s of stocks) {
  console.log(s ?? "desconocido");
}
```

```sortie
12
0
desconocido
```

### 7. const no impide modificar un array

**Piensas:** «Mi lista es `const`, ya no puede cambiar».

```typescript
const compra = ["pan"];
compra.push("leche");
compra[0] = "mantequilla";
console.log(compra);
```

```sortie
[ 'mantequilla', 'leche' ]
```

**Por qué:** `const` sella **la etiqueta**, no el contenido. No puedes poner otro
array en `compra`. Pero el array en sí sigue siendo modificable, como una caja pegada
a la pared cuyo contenido se puede cambiar.

**La corrección:** si el contenido no debe cambiar, dilo con `readonly`.

```typescript
const compra: readonly string[] = ["pan", "leche"];
console.log(compra.length);
```

```sortie
2
```

Con `readonly`, `compra.push(...)` se convierte en un error de tipo.

### 8. sort() ordena los números como palabras

**Piensas:** «`[10, 9, 1].sort()` da `[1, 9, 10]`».

```typescript
const notas = [10, 9, 1, 25];
notas.sort();
console.log(notas);
```

```sortie
[ 1, 10, 25, 9 ]
```

**Por qué:** sin indicaciones, `sort` convierte cada valor en texto y luego ordena
como en un diccionario. `"10"` va antes que `"9"`, porque `"1"` va antes que `"9"`.

**La corrección:** dale una función de comparación.

```typescript
const notas = [10, 9, 1, 25];
notas.sort((a, b) => a - b);
console.log(notas);
```

```sortie
[ 1, 9, 10, 25 ]
```

`a - b` es negativo si `a` es más pequeño: `a` va delante.

### 9. {} significa «casi cualquier cosa»

**Piensas:** «`{}` es un objeto vacío, así que solo acepta objetos».

```typescript
const a: {} = 42;
const b: {} = "hola";
const c: {} = { x: 1 };
console.log(a, b, c);
```

```sortie
42 hola { x: 1 }
```

**Por qué:** el tipo `{}` significa «un valor que no es ni `null` ni `undefined`». Un
número o un texto sirven. El tipo `object`, en cambio, rechaza los valores simples:

```typescript norun
const x: object = 42;
```

```text
main.ts:1:7 - error TS2322: Type 'number' is not assignable to type 'object'.

1 const x: object = 42;
        ~


Found 1 error in main.ts:1
```

**La corrección:** no uses ni `{}` ni `object` para describir tus datos. Escribe su
forma real con una `interface` (mira [los objetos](objets.md)).

### 10. Una errata que se cuela a través de una variable

**Piensas:** «TypeScript siempre detecta las propiedades desconocidas».

Escrito directamente, el objeto sí se comprueba:

```typescript norun
interface Contacto {
  nombre: string;
  email?: string;
}
const c: Contacto = { nombre: "Alicia", emial: "alicia@ejemplo.es" };
```

```text
main.ts:5:41 - error TS2561: Object literal may only specify known properties, but 'emial' does not exist in type 'Contacto'. Did you mean to write 'email'?

5 const c: Contacto = { nombre: "Alicia", emial: "alicia@ejemplo.es" };
                                          ~~~~~


Found 1 error in main.ts:5
```

Pero si el objeto pasa antes por una variable, la errata se esconde:

```typescript
interface Contacto {
  nombre: string;
  email?: string;
}

const datos = { nombre: "Alicia", emial: "alicia@ejemplo.es" };
const c: Contacto = datos;
console.log(c.email ?? "sin dirección");
```

```sortie
sin dirección
```

**Por qué:** la comprobación de propiedades de más solo se aplica a los objetos
escritos **en el sitio**. Un objeto guardado en una variable puede tener propiedades
de más. Y como `email` es opcional, que falte no molesta.

**La corrección:** anota el objeto desde que lo creas: `const datos: Contacto = { ... }`.

### 11. Dos tipos con la misma forma son intercambiables

**Piensas:** «`Euros` y `Dolares` son dos tipos diferentes».

```typescript
type Euros = number;
type Dolares = number;

function pagarEnEuros(importe: Euros): string {
  return `Pagado: ${importe} €`;
}

const precioAmericano: Dolares = 100;
console.log(pagarEnEuros(precioAmericano));
```

```sortie
Pagado: 100 €
```

**Por qué:** `type Euros = number` no crea un tipo nuevo. Le da **otro nombre** a
`number`. Y TypeScript compara las formas, no los nombres: es el
[tipado](../debuter/glossaire.md#tipado) estructural. Dos etiquetas, una misma caja.

**La corrección:** dales formas diferentes, por ejemplo un objeto con la moneda.

```typescript
interface Importe {
  valor: number;
  moneda: "EUR" | "USD";
}

function pagarEnEuros(m: Importe): string {
  if (m.moneda !== "EUR") {
    return "Rechazado: hacen falta euros.";
  }
  return `Pagado: ${m.valor} €`;
}

console.log(pagarEnEuros({ valor: 100, moneda: "USD" }));
```

```sortie
Rechazado: hacen falta euros.
```

### 12. Los enum numéricos guardan sorpresas

Una [enumeración](../debuter/glossaire.md#enumeracion) (`enum`) da un nombre a cada
valor de una lista pequeña.

**Piensas:** «`Color` contiene dos colores».

```typescript
enum Color {
  Rojo,
  Verde,
}

console.log(Color.Rojo);
console.log(Object.keys(Color));
```

```sortie
0
[ '0', '1', 'Rojo', 'Verde' ]
```

**Por qué:** un `enum` numérico numera sus valores (`Rojo` vale `0`). Y es un objeto
JavaScript de verdad, rellenado **en los dos sentidos**: `Color.Rojo` vale `0`, y
`Color[0]` vale `"Rojo"`. Si lo recorres, encuentras cuatro claves.

**La corrección:** para empezar, prefiere una unión de textos. Es sencilla, legible al
mostrarla, y no crea ningún objeto oculto.

```typescript
type Color = "rojo" | "verde";
const colores: Color[] = ["rojo", "verde"];

for (const c of colores) {
  console.log(c);
}
```

```sortie
rojo
verde
```

Verás estas uniones en [el capítulo sobre las uniones](unions.md).

## Errores frecuentes

### Una casilla de array quizá vacía, con noUncheckedIndexedAccess

Si tu proyecto activa `noUncheckedIndexedAccess` (trampa 5), este código se rechaza:

```typescript norun
const frutas = ["manzana", "pera"];
console.log(frutas[2].toUpperCase());
```

```text
main.ts:2:13 - error TS2532: Object is possibly 'undefined'.

2 console.log(frutas[2].toUpperCase());
              ~~~~~~~~~


Found 1 error in main.ts:2
```

«El objeto quizá sea `undefined`». Es una buena noticia: el fallo se encuentra antes
de ejecutar. Prueba la casilla (`if (f !== undefined)`) o escribe
`frutas[2]?.toUpperCase()`.

### Comparar un número y un texto

```typescript norun
const edad = 18;
const entrada = "18";
if (edad === entrada) {
  console.log("misma edad");
}
```

```text
main.ts:3:5 - error TS2367: This comparison appears to be unintentional because the types 'number' and 'string' have no overlap.

3 if (edad === entrada) {
      ~~~~~~~~~~~~~~~~


Found 1 error in main.ts:3
```

«Esta comparación parece involuntaria: un `number` y un `string` no tienen nada en
común». Siempre sería falsa. Convierte: `edad === Number(entrada)`.

### Usar as entre dos tipos sin relación

```typescript norun
const n = "42" as number;
```

```text
main.ts:1:11 - error TS2352: Conversion of type 'string' to type 'number' may be a mistake because neither type sufficiently overlaps with the other. If this was intentional, convert the expression to 'unknown' first.

1 const n = "42" as number;
            ~~~~~~~~~~~~~~


Found 1 error in main.ts:1
```

Incluso `as` tiene límites: un texto no se convierte en número cambiando de etiqueta.
El mensaje propone pasar por `unknown`… ¡no lo hagas! Convierte de verdad:
`const n = Number("42");`.

## Ponte a prueba

**Pregunta 1:** ¿por qué es arriesgado `const p: Producto = JSON.parse(texto)`?

> **Respuesta**
>
> `JSON.parse` devuelve `any`: TypeScript no comprueba el contenido. Si el texto no
> corresponde a `Producto`, el error solo aparecerá durante la ejecución. Hay que
> comprobar los datos antes de fiarse de ellos.

**Pregunta 2:** ¿qué muestra este código?

```typescript norun
const cantidad = 0;
console.log(cantidad || 10, cantidad ?? 10);
```

> **Respuesta**
>
> `10 0`. `||` reemplaza el `0`, porque `0` es un valor «falso». `??` solo reemplaza
> `null` y `undefined`: conserva el `0`.

**Pregunta 3:** ¿qué hay que escribir para ordenar `[3, 20, 100]` en el orden de los
números?

> **Respuesta**
>
> `lista.sort((a, b) => a - b)`. Sin función, `sort` compara textos y da
> `[100, 20, 3]`.

## En resumen

- `any` y `as` hacen callar al inspector: prefiere `unknown` y objetos de verdad,
  completos.
- Los tipos desaparecen durante la ejecución: comprueba los datos que vienen de
  `JSON.parse`, de un archivo o del teclado (que siempre da texto).
- Una casilla de array puede ser `undefined`; `noUncheckedIndexedAccess` te lo
  recuerda.
- `??` conserva el `0`, `||` no; `const` no impide modificar un array; `sort()` sin
  función ordena como palabras.
- `{}` acepta casi todo, `object` rechaza los valores simples: describe mejor la forma
  real.
- El tipado es estructural: dos tipos con la misma forma son intercambiables; los
  `enum` numéricos tienen claves ocultas.

Siguiente: [ejercicios para empezar con buen pie](exercices-faciles.md).
