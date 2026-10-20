# Tipos literales, enums y as const

Esta página explica cómo limitar un valor a una pequeña lista de opciones precisas, con los tipos literales, los enums y `as const`.

> **Objetivos:**
>
> - sabrás escribir un tipo literal y una unión de literales;
> - sabrás por qué `let` y `const` no dan el mismo tipo;
> - sabrás crear un `enum` y compararlo con una unión;
> - sabrás congelar un objeto con `as const` y sacar un tipo de él;
> - sabrás comprobar un valor sin perder su precisión con `satisfies`.

## Un tipo literal

Hasta ahora, un tipo designaba una gran familia de valores: `string` acepta todos los
textos. Un **tipo literal** solo acepta **un único** valor. El tipo `"rojo"` acepta el
texto `"rojo"`, y nada más.

Solo, no sirve de mucho. Pero en una [unión](unions.md), se convierte en una lista de
opciones, como un menú desplegable:

```typescript
type Color = "rojo" | "verde" | "azul";

function pintar(c: Color) {
  console.log(`Pinto de ${c}.`);
}

pintar("verde");
pintar("azul");
```

```sortie
Pinto de verde.
Pinto de azul.
```

Escribir `pintar("rosa")` se rechaza: `"rosa"` no está en el menú. Las erratas
(`"rjo"`) se detectan de la misma manera. Y tu editor te propone las tres opciones
mientras escribes.

Los números y los booleanos también tienen sus literales:

```typescript
type Dado = 1 | 2 | 3 | 4 | 5 | 6;
type Si = true;

const tirada: Dado = 4;
const aceptado: Si = true;
console.log(tirada, aceptado);
```

```sortie
4 true
```

## let, const y el ensanchamiento

TypeScript no da el mismo tipo a `let` y a `const`.

- `const x = "rojo"` nunca cambiará: su tipo es el literal `"rojo"`.
- `let y = "rojo"` podrá cambiar: TypeScript **ensancha** su tipo a `string`.

```typescript
type Color = "rojo" | "verde" | "azul";

function pintar(c: Color) {
  console.log(`Pinto de ${c}.`);
}

const fijo = "rojo";
pintar(fijo);

let variable: Color = "rojo";
variable = "azul";
pintar(variable);
```

```sortie
Pinto de rojo.
Pinto de azul.
```

Con `let`, indica tú mismo el tipo (`let variable: Color`). Sin eso, la variable sería
un `string`, y `pintar(variable)` se rechazaría (mira los
[errores frecuentes](#errores-frecuentes)).

## Las uniones de literales en un objeto

Los literales brillan en los objetos. El estado de un pedido tiene unos pocos valores
posibles, no una infinidad:

```typescript
type Estado = "pendiente" | "enviado" | "entregado";

interface Pedido {
  numero: number;
  estado: Estado;
}

function mensaje(p: Pedido): string {
  switch (p.estado) {
    case "pendiente":
      return `Pedido ${p.numero}: lo estamos preparando.`;
    case "enviado":
      return `Pedido ${p.numero}: está en camino.`;
    case "entregado":
      return `Pedido ${p.numero}: ¡que aproveche!`;
  }
}

console.log(mensaje({ numero: 12, estado: "enviado" }));
```

```sortie
Pedido 12: está en camino.
```

Es el mismo principio que el campo `kind` de las
[uniones discriminadas](unions.md#las-uniones-discriminadas).

## Los enums

Un `enum` (una [enumeración](../debuter/glossaire.md#enumeration)) da un nombre a cada
opción de una lista. Existe en muchos lenguajes, y TypeScript también lo tiene.

```typescript
enum Direccion {
  Arriba,
  Abajo,
  Izquierda,
  Derecha,
}

function mover(d: Direccion): string {
  if (d === Direccion.Arriba) return "Subimos";
  if (d === Direccion.Abajo) return "Bajamos";
  return "Giramos";
}

console.log(mover(Direccion.Arriba));
console.log(Direccion.Izquierda);
console.log(Direccion[Direccion.Izquierda]);
```

```sortie
Subimos
2
Izquierda
```

Por defecto, los miembros valen `0`, `1`, `2`, `3`… en orden. Así que
`Direccion.Izquierda` vale `2`. No dice mucho al mostrarlo. Puedes darles valores de
texto:

```typescript
enum Nivel {
  Facil = "fácil",
  Medio = "medio",
  Dificil = "difícil",
}

const eleccion = Nivel.Medio;
console.log(`Nivel elegido: ${eleccion}`);
console.log(Object.values(Nivel));
```

```sortie
Nivel elegido: medio
[ 'fácil', 'medio', 'difícil' ]
```

> **Nota:** un `enum` no es solo un tipo: crea un objeto JavaScript real en el momento
> de la [compilación](../debuter/glossaire.md#compilation). Aquí no hay problema,
> porque compilamos con `tsc`. Pero algunas herramientas que simplemente «borran» los
> tipos (como la ejecución directa de archivos `.ts` con Node) rechazan los enums.

### ¿Enum o unión de literales?

Muchos proyectos TypeScript prefieren las uniones de literales:

| | Unión de literales | `enum` |
|---|---|---|
| Escritura de un valor | `"fácil"` | `Nivel.Facil` |
| Existe en tiempo de ejecución | no (no se genera nada) | sí (un objeto) |
| Lista de todos los valores | no, salvo con `as const` | sí, con `Object.values` |
| Funciona en todas partes | sí | hay que compilar |

Para empezar, elige la unión de literales. Si necesitas la lista de los valores en
tiempo de ejecución, usa la técnica siguiente.

## Congelar un objeto con as const

Un objeto normal puede cambiar: por eso TypeScript ensancha sus valores (`"fácil"` se
convierte en `string`). `as const` congela el objeto entero:

- todas las propiedades se vuelven `readonly`;
- todos los valores conservan su tipo literal.

```typescript
const NIVELES = {
  FACIL: "fácil",
  MEDIO: "medio",
  DIFICIL: "difícil",
} as const;

console.log(NIVELES.MEDIO);
console.log(Object.values(NIVELES).join(" < "));
```

```sortie
medio
fácil < medio < difícil
```

Sin `as const`, `NIVELES.MEDIO` tendría el tipo `string`. Con él, su tipo es
exactamente `"medio"`. También funciona con un array:
`const DIAS = ["lun", "mar"] as const` tiene el tipo `readonly ["lun", "mar"]`.

### Sacar un tipo de un objeto congelado

Esta es la receta que sustituye a un `enum`. Se escriben los valores **una sola vez**
en el objeto y luego se saca el tipo de él:

```typescript
const NIVELES = {
  FACIL: "fácil",
  MEDIO: "medio",
  DIFICIL: "difícil",
} as const;

type Nivel = (typeof NIVELES)[keyof typeof NIVELES];

function puntos(n: Nivel): number {
  switch (n) {
    case "fácil":
      return 1;
    case "medio":
      return 2;
    case "difícil":
      return 3;
  }
}

console.log(puntos(NIVELES.DIFICIL));
console.log(puntos("medio"));
```

```sortie
3
2
```

Leamos `(typeof NIVELES)[keyof typeof NIVELES]` desde dentro:

1. `typeof NIVELES`: el tipo del objeto,
   `{ readonly FACIL: "fácil"; readonly MEDIO: "medio"; … }`;
2. `keyof typeof NIVELES`: sus claves, `"FACIL" | "MEDIO" | "DIFICIL"`;
3. `(typeof NIVELES)[…]`: el tipo de los valores para esas claves,
   `"fácil" | "medio" | "difícil"`.

Es largo de escribir, pero lo escribes una vez. Estos operadores se explican en detalle
en [los tipos avanzados](types-avances.md).

Para un array congelado, la receta es más corta:

```typescript
const DIAS = ["lun", "mar", "mié"] as const;
type Dia = (typeof DIAS)[number];

const d: Dia = "mar";
console.log(DIAS.indexOf(d));
```

```sortie
1
```

`[number]` quiere decir «el tipo de un elemento cualquiera del array».

## Comprobar sin ensanchar con satisfies

Cuando anotas una variable (`const x: Tipo = …`), TypeScript comprueba el valor, pero
después **olvida** los detalles: solo recuerda `Tipo`. El operador `satisfies`
(«satisface») comprueba el valor **y** conserva su tipo preciso.

```typescript
type Color = "rojo" | "verde" | "azul";

const tema = {
  fondo: "azul",
  texto: "rojo",
} satisfies Record<string, Color>;

const acento: Color = tema.fondo;
console.log(acento, tema.texto);
```

```sortie
azul rojo
```

`Record<string, Color>` quiere decir «un objeto cuyos valores son `Color`». Gracias a
`satisfies`:

- una errata como `fondo: "azull"` se detectaría;
- `tema.fondo` conserva un tipo preciso, y TypeScript sabe que `tema.texto` existe.

Con la anotación `const tema: Record<string, Color>`, TypeScript aceptaría
`tema.titulo` (cualquier clave de texto), que valdría `undefined` en tiempo de
ejecución.

> **Consejo:** combina los dos: `{ … } as const satisfies Tipo` congela el objeto y
> comprueba su forma.

## Errores frecuentes

### Pasar una variable let a una unión de literales

```typescript norun
type Color = "rojo" | "verde" | "azul";
function pintar(c: Color) {
  console.log(`Pinto de ${c}`);
}
let eleccion = "rojo";
pintar(eleccion);
```

```text
main.ts(6,8): error TS2345: Argument of type 'string' is not assignable to parameter of type 'Color'.
```

`let` ensancha el tipo a `string`. Indica el tipo: `let eleccion: Color = "rojo";`, o
usa `const` si el valor no cambia.

### Un valor fuera del menú

```typescript norun
type Color = "rojo" | "verde" | "azul";
const c: Color = "rosa";
```

```text
main.ts(2,7): error TS2322: Type '"rosa"' is not assignable to type 'Color'.
```

`"rosa"` no está en la lista. Elige un valor permitido, o añade `"rosa"` al tipo
`Color`.

### Modificar un objeto congelado

```typescript norun
const CONFIG = { puerto: 8080, host: "localhost" } as const;
CONFIG.puerto = 3000;
```

```text
main.ts(2,8): error TS2540: Cannot assign to 'puerto' because it is a read-only property.
```

`as const` lo vuelve todo de solo lectura. Si el valor debe cambiar, quita `as const`
o crea un objeto nuevo: `const config2 = { ...CONFIG, puerto: 3000 };`.

### Dar un texto en lugar de un enum

```typescript norun
enum Direccion {
  Arriba,
  Abajo,
}
const d: Direccion = "Arriba";
```

```text
main.ts(5,7): error TS2322: Type '"Arriba"' is not assignable to type 'Direccion'.
```

Un valor de enum se escribe con el nombre del enum: `Direccion.Arriba`.

## Ponte a prueba

**Pregunta 1:** ¿cuál es el tipo de `a`? ¿Y el de `b`?

```typescript norun
const a = "gato";
let b = "gato";
```

> **Respuesta**
>
> `a` tiene el tipo literal `"gato"`, porque una constante no cambia. `b` tiene el tipo
> `string`, porque un `let` puede recibir otro texto más tarde.

**Pregunta 2:** ¿qué hace `as const` con un objeto?

> **Respuesta**
>
> Vuelve todas sus propiedades `readonly` y conserva el tipo literal de cada valor
> (`"fácil"` en lugar de `string`). Así se puede sacar de él una unión con
> `(typeof OBJETO)[keyof typeof OBJETO]`.

**Pregunta 3:** a partir de `const TALLAS = ["S", "M", "L"] as const;`, escribe un tipo
`Talla` y una función que muestre una talla.

> **Respuesta**
>
> ```typescript
> const TALLAS = ["S", "M", "L"] as const;
> type Talla = (typeof TALLAS)[number];
>
> function mostrar(t: Talla) {
>   console.log(`Talla ${t}`);
> }
>
> mostrar("M");
> ```
>
> ```sortie
> Talla M
> ```

## En resumen

- Un tipo literal solo acepta un valor; una unión de literales forma un menú de
  opciones.
- `const` conserva el tipo literal, `let` lo ensancha: anota tus `let`.
- Un `enum` nombra opciones y existe en tiempo de ejecución; requiere compilar.
- `as const` congela un objeto; `(typeof X)[keyof typeof X]` saca de él la unión de
  los valores.
- `satisfies` comprueba un valor sin perder su tipo preciso.

Siguiente: [los tipos avanzados](types-avances.md)
