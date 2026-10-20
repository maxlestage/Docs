# Tipos avanzados

Esta página explica cómo fabricar nuevos tipos a partir de tipos existentes, con los tipos utilitarios, `keyof`, `typeof` y algunas herramientas más potentes.

> **Objetivos:**
>
> - sabrás usar los tipos utilitarios habituales: `Partial`, `Pick`, `Omit`, `Record`…;
> - sabrás sacar un tipo de un valor con `typeof` y las claves con `keyof`;
> - sabrás leer el tipo de una propiedad con `Tipo["clave"]`;
> - sabrás leer un tipo mapeado (*mapped type*) y un tipo condicional sencillo;
> - sabrás construir tipos de texto con los *template literal types*.

## ¿Por qué fabricar tipos?

Imagina un tipo `Usuario`. Para el formulario de edición, necesitas el mismo tipo, pero
con todos los campos opcionales. Para la vista pública, necesitas el mismo tipo, sin la
contraseña. Copiar el tipo tres veces sería frágil: el día que añadas un campo, tendrías
que acordarte de las tres copias.

TypeScript permite **calcular** un tipo a partir de otro, como una fórmula de hoja de
cálculo calcula una celda a partir de otra. Cuando el original cambia, los tipos
calculados lo siguen.

Todo este capítulo se apoya en este tipo:

```typescript norun
interface Usuario {
  id: number;
  nombre: string;
  email: string;
  clave: string;
}
```

## Partial y Required

`Partial<T>` hace que **todas** las propiedades sean opcionales. Es ideal para una
actualización, en la que solo cambias algunos campos.

```typescript
interface Usuario {
  id: number;
  nombre: string;
  email: string;
  clave: string;
}

function modificar(u: Usuario, cambios: Partial<Usuario>): Usuario {
  return { ...u, ...cambios };
}

const alicia: Usuario = { id: 1, nombre: "Alicia", email: "a@ej.es", clave: "x" };
const nueva = modificar(alicia, { email: "alicia@ej.es" });
console.log(nueva.email);
```

```sortie
alicia@ej.es
```

`Required<T>` hace lo contrario: todas las propiedades pasan a ser obligatorias.

```typescript
interface Opciones {
  color?: string;
  tamano?: number;
}

const porDefecto: Required<Opciones> = { color: "negro", tamano: 12 };

function ajustar(o: Opciones): Required<Opciones> {
  return { ...porDefecto, ...o };
}

console.log(ajustar({ tamano: 16 }));
```

```sortie
{ color: 'negro', tamano: 16 }
```

## Readonly

`Readonly<T>` prohíbe modificar las propiedades. Útil para una configuración que no
debe cambiar.

```typescript
interface Config {
  puerto: number;
  host: string;
}

const config: Readonly<Config> = { puerto: 8080, host: "localhost" };
console.log(`${config.host}:${config.puerto}`);
```

```sortie
localhost:8080
```

Escribir `config.puerto = 3000` sería rechazado. Para los arrays existe
`ReadonlyArray<T>`, que también se escribe `readonly T[]`.

## Pick y Omit

`Pick<T, Claves>` conserva solo algunas propiedades; `Omit<T, Claves>` quita algunas.
Las claves se escriben como una unión de textos.

```typescript
interface Usuario {
  id: number;
  nombre: string;
  email: string;
  clave: string;
}

type Resumen = Pick<Usuario, "id" | "nombre">;
type Publico = Omit<Usuario, "clave">;

function resumir(u: Usuario): Resumen {
  return { id: u.id, nombre: u.nombre };
}

function hacerPublico(u: Usuario): Publico {
  const { clave, ...resto } = u;
  return resto;
}

const bob: Usuario = { id: 2, nombre: "Bob", email: "b@ej.es", clave: "secreto" };
console.log(resumir(bob));
console.log(hacerPublico(bob));
```

```sortie
{ id: 2, nombre: 'Bob' }
{ id: 2, nombre: 'Bob', email: 'b@ej.es' }
```

> **Consejo:** elige el que pida la lista más corta. Para conservar 2 campos de 10,
> `Pick`; para quitar 1 de 10, `Omit`.

## Record

`Record<Claves, Valor>` describe un objeto cuyas claves se conocen y cuyos valores
tienen todos el mismo tipo. Con una unión de literales como claves, TypeScript exige
**cada** clave.

```typescript
type Dia = "lun" | "mar" | "mie";

const horas: Record<Dia, number> = { lun: 7, mar: 8, mie: 6 };

let total = 0;
for (const dia of Object.keys(horas) as Dia[]) {
  total += horas[dia];
}
console.log(`Total: ${total} h`);

const stock: Record<string, number> = {};
stock["manzanas"] = 12;
stock["peras"] = 3;
console.log(stock);
```

```sortie
Total: 21 h
{ manzanas: 12, peras: 3 }
```

`Record<string, number>` sirve de [diccionario](../debuter/glossaire.md#diccionario)
sencillo: cualquier clave de texto, valores numéricos.

> **Nota:** `Object.keys` siempre devuelve un `string[]`. El `as Dia[]` le dice a
> TypeScript que son de verdad valores `Dia`, lo cual es cierto aquí.

## typeof y keyof

Ya los has visto en los capítulos anteriores. Recordatorio:

- `typeof valor`, **en una posición de tipo**, da el tipo de un valor existente;
- `keyof Tipo` da la unión de los nombres de propiedades de un tipo.

```typescript
const ajustes = { volumen: 5, silencio: false, idioma: "es" };

type Ajustes = typeof ajustes;
type NombreAjuste = keyof Ajustes;

function leer(nombre: NombreAjuste): string {
  return `${nombre} = ${ajustes[nombre]}`;
}

console.log(leer("volumen"));
console.log(leer("idioma"));
```

```sortie
volumen = 5
idioma = es
```

Aquí, `Ajustes` vale `{ volumen: number; silencio: boolean; idioma: string }` y
`NombreAjuste` vale `"volumen" | "silencio" | "idioma"`. No has escrito ningún tipo a
mano: siguen al objeto.

## Los tipos de acceso indexado

`Tipo["clave"]` da el tipo de **una** propiedad. Es la misma escritura que para leer un
valor en un objeto, pero aplicada a los tipos.

```typescript
interface Pedido {
  numero: number;
  cliente: { nombre: string; ciudad: string };
  lineas: { producto: string; cantidad: number }[];
}

type Cliente = Pedido["cliente"];
type Linea = Pedido["lineas"][number];

const c: Cliente = { nombre: "Lucía", ciudad: "Sevilla" };
const l: Linea = { producto: "Cuaderno", cantidad: 3 };
console.log(`${c.nombre} pide ${l.cantidad} × ${l.producto}`);
```

```sortie
Lucía pide 3 × Cuaderno
```

`[number]` después de un tipo array da el tipo de un elemento. No hace falta declarar
`Cliente` y `Linea` por separado: se extraen.

## ReturnType, Parameters y Awaited

Estos tres utilitarios leen el tipo de una **función**:

- `ReturnType<F>`: el tipo de lo que devuelve;
- `Parameters<F>`: los tipos de sus parámetros, en forma de tupla;
- `Awaited<P>`: el tipo que se obtiene tras `await` sobre una promesa.

```typescript
function crearJugador(nombre: string, nivel: number) {
  return { nombre, nivel, vidas: 3 };
}

type Jugador = ReturnType<typeof crearJugador>;
type Argumentos = Parameters<typeof crearJugador>;

const args: Argumentos = ["Sam", 2];
const j: Jugador = crearJugador(...args);
console.log(j);

async function cargarPuntos(): Promise<number> {
  return 1200;
}

type Puntos = Awaited<ReturnType<typeof cargarPuntos>>;
const p: Puntos = await cargarPuntos();
console.log(p + 1);
```

```sortie
{ nombre: 'Sam', nivel: 2, vidas: 3 }
1201
```

`Jugador` vale `{ nombre: string; nivel: number; vidas: number }`, sin que lo hayas
escrito. Si la función cambia, el tipo la sigue.

## NonNullable

`NonNullable<T>` quita `null` y `undefined` de una unión.

```typescript
type PuedeFaltar = string | null | undefined;
type Presente = NonNullable<PuedeFaltar>;

const nombres: PuedeFaltar[] = ["Ana", null, "Leo", undefined];
const presentes: Presente[] = nombres.filter((n): n is Presente => n != null);
console.log(presentes);
```

```sortie
[ 'Ana', 'Leo' ]
```

`n != null` (con `!=`, no `!==`) elimina a la vez `null` y `undefined`. La guarda de
tipo `n is Presente` viene del capítulo sobre [las uniones](unions.md#tus-propias-guardas-de-tipo).

## Los tipos mapeados

¿Cómo está escrito `Partial`? Con un **tipo mapeado** (*mapped type*). Recorre las
claves de un tipo, como un bucle `for`, y construye un tipo nuevo.

```typescript
type Marcas<T> = {
  [K in keyof T]: boolean;
};

interface Formulario {
  nombre: string;
  edad: number;
  email: string;
}

const modificados: Marcas<Formulario> = { nombre: true, edad: false, email: false };
const campos = Object.entries(modificados)
  .filter(([, v]) => v)
  .map(([k]) => k);
console.log(`Campos modificados: ${campos.join(", ")}`);
```

```sortie
Campos modificados: nombre
```

`[K in keyof T]: boolean` se lee: «para cada clave `K` de `T`, una propiedad de tipo
`boolean`». El resultado tiene las mismas claves que `Formulario`, con booleanos.
También puedes añadir `?` o `readonly`: `Partial<T>` es simplemente
`{ [K in keyof T]?: T[K] }`.

## Los tipos condicionales

Un **tipo condicional** elige un tipo según una condición. Se parece al operador
ternario `a ? b : c`, pero para los tipos: `T extends U ? Si : No`.

```typescript
type Descripcion<T> = T extends string ? "texto" : T extends number ? "número" : "otro";

function describir<T>(valor: T, descripcion: Descripcion<T>): string {
  return `${String(valor)}: ${descripcion}`;
}

console.log(describir("abc", "texto"));
console.log(describir(42, "número"));
console.log(describir(true, "otro"));
```

```sortie
abc: texto
42: número
true: otro
```

Aquí, `describir(42, "texto")` sería rechazado: para un número,
`Descripcion<number>` vale `"número"`. `NonNullable` y `Awaited` están escritos con
tipos condicionales.

> **Nota:** a menudo leerás tipos condicionales en las bibliotecas. Al principio
> escribirás pocos: los tipos utilitarios bastan en la mayoría de los casos.

## Los template literal types

Las plantillas de texto `` `Hola ${nombre}` `` también existen para los tipos.
Combinadas con uniones, producen todas las combinaciones.

```typescript
type Tamano = "chico" | "grande";
type Color = "rojo" | "azul";
type Variante = `${Tamano}-${Color}`;

const v1: Variante = "chico-rojo";
const v2: Variante = "grande-azul";
console.log(v1, v2);

type Evento = "clic" | "pasar";
type Manejador = `on${Capitalize<Evento>}`;

const manejadores: Record<Manejador, () => string> = {
  onClic: () => "clicado",
  onPasar: () => "sobrevolado",
};
console.log(manejadores.onClic());
```

```sortie
chico-rojo grande-azul
clicado
```

`Variante` vale `"chico-rojo" | "chico-azul" | "grande-rojo" | "grande-azul"`.
`Capitalize` pone la primera letra en mayúscula: `Manejador` vale
`"onClic" | "onPasar"`. También existen `Uppercase`, `Lowercase` y `Uncapitalize`.

## Errores frecuentes

### Leer una propiedad quitada por Omit

```typescript norun
type Usuario = { nombre: string; email: string; clave: string };
type Publico = Omit<Usuario, "clave">;
function mostrar(u: Publico) {
  console.log(u.clave);
}
```

```text
main.ts(4,17): error TS2339: Property 'clave' does not exist on type 'Publico'.
```

Es el objetivo de `Omit`: ese campo ya no existe. Si lo necesitas aquí, usa el tipo
completo `Usuario`.

### Olvidar una clave en un Record

```typescript norun
type Dia = "lun" | "mar" | "mie";
const horas: Record<Dia, number> = { lun: 7, mar: 8 };
```

```text
main.ts(2,7): error TS2741: Property 'mie' is missing in type '{ lun: number; mar: number; }' but required in type 'Record<Dia, number>'.
```

Con claves literales, cada clave es obligatoria. Añade `mie`, o usa
`Partial<Record<Dia, number>>` si algunas pueden faltar.

### Escribir Tipo.clave en lugar de Tipo["clave"]

```typescript norun
type Producto = { nombre: string; precio: number };
type Precio = Producto.precio;
```

```text
main.ts(2,15): error TS2713: Cannot access 'Producto.precio' because 'Producto' is a type, but not a namespace. Did you mean to retrieve the type of the property 'precio' in 'Producto' with 'Producto["precio"]'?
```

Para los tipos, el punto no funciona. Usa los corchetes: `Producto["precio"]`. El
mensaje incluso propone la corrección.

## Ponte a prueba

**Pregunta 1:** ¿qué tipo utilitario elegir para una función de actualización que
recibe solo los campos que hay que cambiar?

> **Respuesta**
>
> `Partial<T>`. Todas las propiedades pasan a ser opcionales: puedes enviar
> `{ email: "…" }` solo.

**Pregunta 2:** ¿cuánto vale `keyof { x: number; y: number }`?

> **Respuesta**
>
> La unión de las claves: `"x" | "y"`.

**Pregunta 3:** a partir de `interface Articulo { titulo: string; precio: number; stock: number }`,
crea un tipo `Etiqueta` que contenga solo `titulo` y `precio`, y úsalo.

> **Respuesta**
>
> ```typescript
> interface Articulo {
>   titulo: string;
>   precio: number;
>   stock: number;
> }
>
> type Etiqueta = Pick<Articulo, "titulo" | "precio">;
>
> const e: Etiqueta = { titulo: "Goma", precio: 1.5 };
> console.log(`${e.titulo}: ${e.precio} €`);
> ```
>
> ```sortie
> Goma: 1.5 €
> ```
>
> `Omit<Articulo, "stock">` da el mismo resultado.

## En resumen

- Los tipos utilitarios calculan un tipo a partir de otro: cuando el original cambia,
  lo siguen.
- `Partial`, `Required`, `Readonly` cambian todas las propiedades; `Pick` y `Omit`
  eligen algunas; `Record` crea un objeto clave → valor.
- `typeof` da el tipo de un valor, `keyof` la unión de las claves, `T["clave"]` el tipo
  de una propiedad.
- `ReturnType`, `Parameters`, `Awaited` y `NonNullable` leen funciones y uniones.
- Los tipos mapeados (`[K in keyof T]`), condicionales (`A extends B ? X : Y`) y las
  plantillas (`` `on${Nombre}` ``) son las piezas de los tipos utilitarios.

Siguiente: [los decoradores](decorateurs.md)
