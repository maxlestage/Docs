# Map, Set y Record

Esta página explica cómo asociar claves a valores con `Map` y `Record`, y cómo guardar valores sin duplicados con `Set`.

> **Objetivos:**
>
> - sabrás asociar claves a valores con `Map<K, V>`;
> - sabrás manejar el `undefined` que devuelve `get`;
> - sabrás quitar los duplicados con `Set<T>`;
> - sabrás describir un objeto clave-valor con `Record` y elegir entre `Record` y
>   `Map`.

Este capítulo sigue a [arrays y tuplas](collections.md): volverás a encontrar
`for...of`, la desestructuración `[a, b]` y el reflejo `??` que viste con `find`.

## Map: asociar claves a valores

Un `Map` es un [diccionario](../debuter/glossaire.md#diccionario): asocia una
**clave** a un **valor**, como una guía telefónica asocia un nombre a un número. Su
tipo se escribe `Map<TipoDeClaves, TipoDeValores>`.

```typescript
const stock = new Map<string, number>();

stock.set("manzanas", 10);
stock.set("peras", 4);
stock.set("manzanas", 12);

console.log(stock.get("manzanas"));
console.log(stock.has("kiwis"), stock.size);

for (const [fruta, cantidad] of stock) {
  console.log(`${fruta}: ${cantidad}`);
}
```

```sortie
12
false 2
manzanas: 12
peras: 4
```

Línea por línea:

- `new Map<string, number>()` crea un `Map` vacío: claves de texto, valores numéricos;
- los `set` guardan `manzanas → 10` y `peras → 4`, y luego reemplazan 10 por 12;
- `get("manzanas")` da 12; la clave `"kiwis"` no existe, y el `Map` tiene 2 pares;
- el bucle `for...of` da cada par, en el orden en que se añadieron.

Los métodos que debes conocer:

- `set(clave, valor)` añade o reemplaza (una clave es única);
- `get(clave)` lee el valor, `has(clave)` comprueba si la clave existe;
- `size` da el número de pares; `delete(clave)` quita uno.

`get` devuelve `number | undefined`: puede que la clave no exista. El mismo reflejo
que con `find`:

```typescript
const stock = new Map<string, number>([["manzanas", 10]]);

const kiwis = stock.get("kiwis") ?? 0;
console.log(kiwis + 5);
```

```sortie
5
```

**Te toca:** escribe un programa que cree un `Map` de edades, guarde en él `"Alice"`
(30 años) y `"Bob"` (25 años), y luego muestre la edad de Bob y la de `"Zoé"`, con 0
si no está.

> **Solución**
>
> ```typescript
> const edades = new Map<string, number>();
> edades.set("Alice", 30);
> edades.set("Bob", 25);
> console.log(edades.get("Bob"));
> console.log(edades.get("Zoé") ?? 0);
> ```
>
> ```sortie
> 25
> 0
> ```

## Set: una colección sin duplicados

Piensa en la lista de invitados de una fiesta: escribir dos veces el mismo nombre no
sirve de nada. Un `Set` se encarga de ello y descarta los duplicados.

Un `Set` es un [conjunto](../debuter/glossaire.md#conjunto): cada valor aparece en él
una sola vez. Su tipo se escribe `Set<T>`.

```typescript
const visitantes = new Set<string>();

visitantes.add("Alice");
visitantes.add("Bob");
visitantes.add("Alice");

console.log(visitantes.size);
console.log(visitantes.has("Bob"));

const sinDuplicados = [...new Set([3, 1, 3, 2, 1])];
console.log(sinDuplicados);
```

```sortie
2
true
[ 3, 1, 2 ]
```

Línea por línea:

- `visitantes` empieza vacío;
- `add` añade `"Alice"` y luego `"Bob"`. El segundo `"Alice"` se ignora: ya está;
- por eso `size` vale 2, y `has("Bob")` responde `true`;
- el final crea un `Set` a partir del array y lo vuelve a convertir en array con
  `[...]`.

`new Set(array)` y luego `[...]` es el truco clásico para quitar los duplicados de un
array, conservando el orden de primera aparición.

## Record: un objeto clave-valor

Un `Record` se parece a una hoja de puntuaciones: un nombre, un número, un nombre, un
número. Lo escribes de una vez, como un objeto normal.

`Record<string, number>` describe un objeto simple cuyas claves son textos y cuyos
valores son números. Es más ligero que un `Map`, y se escribe directamente con llaves.

```typescript
const puntuaciones: Record<string, number> = {
  alice: 12,
  bob: 8,
};

puntuaciones.chloe = 15;
puntuaciones["bob"] += 1;

console.log(puntuaciones);
console.log(Object.keys(puntuaciones));

for (const [nombre, puntos] of Object.entries(puntuaciones)) {
  console.log(nombre.padEnd(6), puntos);
}
```

```sortie
{ alice: 12, bob: 9, chloe: 15 }
[ 'alice', 'bob', 'chloe' ]
alice  12
bob    9
chloe  15
```

Línea por línea:

- `puntuaciones` empieza con dos pares: `alice → 12` y `bob → 8`;
- `puntuaciones.chloe = 15` añade una clave; `puntuaciones["bob"] += 1` sube a Bob a 9;
- `Object.keys` lista las claves, en el orden en que se añadieron;
- el bucle da cada par; `padEnd(6)` completa el nombre con espacios, para alinear las
  columnas.

`Object.keys` da las claves, `Object.entries` los pares `[clave, valor]`.

Con una unión de tipos literales como claves, `Record` exige **todas** las claves:

```typescript
type Dia = "lunes" | "martes" | "miercoles";

const horas: Record<Dia, number> = {
  lunes: 7,
  martes: 8,
  miercoles: 4,
};

console.log(horas.martes);
```

```sortie
8
```

Olvidar `miercoles` sería un error.

> **Consejo:** usa `Record` para datos fijos escritos en el código, y `Map` cuando
> añades y quitas claves a menudo.

## Errores frecuentes

### Leer un Map con corchetes

```typescript norun
const stock = new Map<string, number>([["manzanas", 10]]);
console.log(stock["manzanas"]);
```

```text
main.ts:2:13 - error TS7052: Element implicitly has an 'any' type because type 'Map<string, number>' has no index signature. Did you mean to call 'stock.get'?

2 console.log(stock["manzanas"]);
              ~~~~~~~~~~~~~~~~~


Found 1 error in main.ts:2
```

Un `Map` no es un objeto: sus valores no se leen con corchetes. El mensaje lo sugiere:
escribe `stock.get("manzanas")`.

### Usar el resultado de get sin comprobarlo

```typescript norun
const stock = new Map<string, number>([["manzanas", 10]]);
const n = stock.get("peras");
console.log(n + 1);
```

```text
main.ts:3:13 - error TS18048: 'n' is possibly 'undefined'.

3 console.log(n + 1);
              ~


Found 1 error in main.ts:3
```

Puede que la clave `"peras"` no exista. Da un valor por defecto:
`const n = stock.get("peras") ?? 0;`.

### Olvidar una clave en un Record

```typescript norun
type Dia = "lunes" | "martes" | "miercoles";
const horas: Record<Dia, number> = { lunes: 7, martes: 8 };
```

```text
main.ts:2:7 - error TS2741: Property 'miercoles' is missing in type '{ lunes: number; martes: number; }' but required in type 'Record<Dia, number>'.

2 const horas: Record<Dia, number> = { lunes: 7, martes: 8 };
        ~~~~~


Found 1 error in main.ts:2
```

Con una unión como tipo de las claves, cada día necesita su valor. Añade la clave que
falta: `{ lunes: 7, martes: 8, miercoles: 4 }`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto vale `new Set(["a", "b", "a"]).size`?

> **Respuesta**
>
> `2`. Un `Set` guarda cada valor una sola vez: el segundo `"a"` se ignora.

**Pregunta 2:** `edades` es un `Map<string, number>`. ¿Cuál es el tipo de
`edades.get("Bob")`, y por qué?

> **Respuesta**
>
> `number | undefined`. Puede que la clave `"Bob"` no exista: en ese caso, `get`
> devuelve `undefined`. Trátalo con `if`, `?.` o `??`.

**Pregunta 3:** completa el código para mostrar el número de nombres distintos.

```typescript norun
const inscritos = ["Lea", "Tom", "Lea", "Inés", "Tom"];
```

> **Respuesta**
>
> ```typescript
> const inscritos = ["Lea", "Tom", "Lea", "Inés", "Tom"];
> console.log(new Set(inscritos).size);
> ```
>
> ```sortie
> 3
> ```
>
> El `Set` quita los duplicados; `size` cuenta lo que queda.

## En resumen

- `Map<K, V>` asocia claves a valores: `set`, `get`, `has`, `delete` y `size`.
- `get` devuelve `V | undefined`: trata el caso con `if`, `?.` o `??`.
- `Set<T>` guarda cada valor una sola vez; `[...new Set(array)]` quita los
  duplicados de un array.
- `Record<string, number>` describe un objeto clave-valor; con una unión de claves,
  las exige todas.
- `Record` para datos fijos escritos en el código, `Map` cuando las claves cambian a
  menudo.

Siguiente: [objetos, interfaces y alias de tipo](objets.md).
