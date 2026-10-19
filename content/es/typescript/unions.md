# Uniones y estrechamiento

Esta página explica cómo decir que un valor puede tener varios tipos, y cómo TypeScript deduce el tipo exacto gracias a tus comprobaciones.

> **Objetivos:**
>
> - sabrás escribir un tipo unión con `|`;
> - sabrás estrechar una unión con `typeof`, `in`, `instanceof` y `===`;
> - sabrás construir una unión discriminada con un campo `kind`;
> - sabrás comprobar que un `switch` trata todos los casos gracias a `never`;
> - sabrás escribir tu propia guarda de tipo `x is T`.

## Un tipo unión

A veces, un valor puede ser de dos [tipos](../debuter/glossaire.md#tipo) distintos.
Un identificador puede ser un número (`42`) o un texto (`"A-42"`). El **tipo unión**
se escribe con una barra vertical `|`, que se lee «o».

```typescript
let identificador: number | string;

identificador = 42;
console.log(identificador);

identificador = "A-42";
console.log(identificador);
```

```sortie
42
A-42
```

Una unión es como un buzón que acepta cartas **o** paquetes. Rechaza todo lo demás:
`identificador = true` sería un error.

Puedes darle un nombre a una unión con `type`:

```typescript
type Identificador = number | string;

function mostrar(id: Identificador) {
  console.log(`Identificador: ${id}`);
}

mostrar(7);
mostrar("B-7");
```

```sortie
Identificador: 7
Identificador: B-7
```

### Lo que se puede hacer con una unión

Con una unión, TypeScript solo permite lo que funciona para **todos** los tipos de la
unión. `toString()` existe en los números y en los textos: está permitido.
`toUpperCase()` solo existe en los textos: se rechaza (mira los
[errores frecuentes](#errores-frecuentes)).

```typescript
function aTexto(x: number | string): string {
  return x.toString();
}

console.log(aTexto(3.5) + " / " + aTexto("tres"));
```

```sortie
3.5 / tres
```

Para ir más lejos, primero hay que **saber** cuál de los tipos tienes.

## El estrechamiento con typeof

Cuando compruebas el tipo de un valor en un `if`, TypeScript lo recuerda. Dentro del
bloque del `if`, **estrecha** la unión: sabe que el valor tiene el tipo comprobado. Es
como un detective que descarta a los sospechosos uno a uno.

`typeof x` da el nombre del tipo en forma de texto: `"string"`, `"number"`,
`"boolean"`, `"object"`, `"undefined"`, `"function"`…

```typescript
function gritar(x: string | number): string {
  if (typeof x === "string") {
    // aquí, x es un string
    return x.toUpperCase();
  }
  // aquí, solo queda number
  return x.toFixed(1);
}

console.log(gritar("hola"));
console.log(gritar(2));
```

```sortie
HOLA
2.0
```

Después del `return`, TypeScript sabe que el caso `string` está tratado. Solo queda
`number`. No hace falta `else`.

## El estrechamiento por veracidad

Un valor opcional tiene a menudo el tipo `string | undefined`. Un simple `if (x)`
descarta `undefined` (y también `null`, `""`, `0`): se habla de prueba de
**veracidad**.

```typescript
function saludar(nombre?: string) {
  if (nombre) {
    console.log(`¡Hola, ${nombre}!`);
  } else {
    console.log("¡Hola, desconocido!");
  }
}

saludar("Alice");
saludar();
```

```sortie
¡Hola, Alice!
¡Hola, desconocido!
```

> **Atención:** `if (x)` también rechaza `0` y la cadena vacía `""`. Para un número,
> comprueba mejor `x !== undefined`; si no, `0` se tomará por «sin valor».

## El estrechamiento por igualdad

Comparar con `===` también estrecha. Si `a === b`, los dos tienen por fuerza un tipo
en común. Es muy útil con `null`:

```typescript
function longitud(texto: string | null): number {
  if (texto === null) {
    return 0;
  }
  return texto.length;
}

console.log(longitud("gato"));
console.log(longitud(null));
```

```sortie
4
0
```

## El estrechamiento con in

Con objetos, `typeof` siempre responde `"object"`. El operador `in` comprueba si un
objeto tiene una propiedad: `"volar" in animal`.

```typescript
type Pajaro = { nombre: string; volar(): string };
type Pez = { nombre: string; nadar(): string };

function mover(animal: Pajaro | Pez): string {
  if ("volar" in animal) {
    return animal.volar();
  }
  return animal.nadar();
}

const urraca: Pajaro = { nombre: "urraca", volar: () => "La urraca vuela." };
const trucha: Pez = { nombre: "trucha", nadar: () => "La trucha nada." };

console.log(mover(urraca));
console.log(mover(trucha));
```

```sortie
La urraca vuela.
La trucha nada.
```

## El estrechamiento con instanceof

Para los objetos creados con `new` (las [clases](types.md)), `instanceof` comprueba la
clase del objeto. `Date` es una clase que proporciona JavaScript.

```typescript
function formatear(valor: Date | string): string {
  if (valor instanceof Date) {
    return valor.toISOString().slice(0, 10);
  }
  return valor;
}

console.log(formatear(new Date(Date.UTC(2024, 0, 15))));
console.log(formatear("mañana"));
```

```sortie
2024-01-15
mañana
```

> **Nota:** en `Date.UTC(2024, 0, 15)`, los meses empiezan en 0: `0` significa enero.

## Las uniones discriminadas

Esta es la herramienta más útil de este capítulo. Cada tipo de la unión lleva un campo
común, a menudo llamado `kind` («clase», «tipo»), con un valor **fijo** y distinto.
Este campo sirve de etiqueta: basta con leerlo para saber con qué tipo estás tratando.

```typescript
type Circulo = { kind: "circulo"; radio: number };
type Rectangulo = { kind: "rectangulo"; ancho: number; alto: number };
type Forma = Circulo | Rectangulo;

function area(f: Forma): number {
  if (f.kind === "circulo") {
    return Math.PI * f.radio ** 2;
  }
  return f.ancho * f.alto;
}

console.log(area({ kind: "rectangulo", ancho: 3, alto: 4 }));
console.log(area({ kind: "circulo", radio: 1 }).toFixed(2));
```

```sortie
12
3.14
```

Dentro del `if`, TypeScript sabe que `f` es un `Circulo`: `f.radio` está permitido.
Después, solo queda `Rectangulo`. El tipo `"circulo"` es un **tipo literal**: solo
acepta un único valor (mira [los tipos literales](litteraux.md)).

### Con un switch

Con varios casos, un `switch` sobre `kind` se lee mejor que una serie de `if`:

```typescript
type Evento =
  | { kind: "clic"; x: number; y: number }
  | { kind: "tecla"; tecla: string }
  | { kind: "cierre" };

function describir(e: Evento): string {
  switch (e.kind) {
    case "clic":
      return `Clic en (${e.x}, ${e.y})`;
    case "tecla":
      return `Tecla ${e.tecla}`;
    case "cierre":
      return "Ventana cerrada";
  }
}

console.log(describir({ kind: "clic", x: 10, y: 20 }));
console.log(describir({ kind: "tecla", tecla: "Intro" }));
console.log(describir({ kind: "cierre" }));
```

```sortie
Clic en (10, 20)
Tecla Intro
Ventana cerrada
```

## Comprobar todos los casos con never

El tipo `never` significa «imposible»: ningún valor puede tenerlo. Sirve de red de
seguridad. En el `default` de un `switch`, si se han tratado todos los casos, el valor
tiene el tipo `never`. Si olvidas un caso, tiene otro tipo, y TypeScript protesta.

```typescript
type Semaforo = { kind: "verde" } | { kind: "ambar" } | { kind: "rojo" };

function indicacion(semaforo: Semaforo): string {
  switch (semaforo.kind) {
    case "verde":
      return "Pase";
    case "ambar":
      return "Reduzca la velocidad";
    case "rojo":
      return "Deténgase";
    default: {
      const imposible: never = semaforo;
      return imposible;
    }
  }
}

console.log(indicacion({ kind: "ambar" }));
```

```sortie
Reduzca la velocidad
```

El día que añadas `{ kind: "intermitente" }` al tipo `Semaforo`, la línea
`const imposible: never = semaforo` se convierte en un error. TypeScript te muestra el
lugar exacto que hay que completar. Es un detector de olvidos gratuito.

## Tus propias guardas de tipo

A veces, la comprobación es demasiado complicada para un simple `typeof`. Puedes
escribir una función que haga la comprobación y devuelva un booleano, con un tipo de
retorno especial: `x is T` («x es un T»). Es una **guarda de tipo**.

```typescript
type Gato = { nombre: string; maullar(): string };
type Perro = { nombre: string; ladrar(): string };

function esUnGato(animal: Gato | Perro): animal is Gato {
  return "maullar" in animal;
}

const animales: (Gato | Perro)[] = [
  { nombre: "Michi", maullar: () => "Miau" },
  { nombre: "Rex", ladrar: () => "Guau" },
];

for (const a of animales) {
  if (esUnGato(a)) {
    console.log(`${a.nombre}: ${a.maullar()}`);
  } else {
    console.log(`${a.nombre}: ${a.ladrar()}`);
  }
}
```

```sortie
Michi: Miau
Rex: Guau
```

Una guarda de tipo también funciona con `filter`: el array obtenido tiene el tipo
correcto.

```typescript
function esUnNumero(x: number | string): x is number {
  return typeof x === "number";
}

const mezcla = [3, "a", 7, "b"];
const numeros: number[] = mezcla.filter(esUnNumero);
console.log(numeros);
```

```sortie
[ 3, 7 ]
```

> **Atención:** TypeScript se fía de tu guarda sin comprobarla. Si la función devuelve
> `true` por error, el tipo será falso. Mantén la comprobación simple y correcta.

## Errores frecuentes

### Usar un método sin estrechar

```typescript norun
function gritar(x: string | number) {
  return x.toUpperCase();
}
```

```text
main.ts(2,12): error TS2339: Property 'toUpperCase' does not exist on type 'string | number'.
  Property 'toUpperCase' does not exist on type 'number'.
```

`toUpperCase` no existe en los números. Comprueba primero el tipo:
`if (typeof x === "string") return x.toUpperCase();`.

### Olvidar que el valor puede faltar

```typescript norun
function inicial(nombre?: string) {
  return nombre[0];
}
```

```text
main.ts(2,10): error TS18048: 'nombre' is possibly 'undefined'.
```

Un parámetro opcional tiene el tipo `string | undefined`. Trata el caso en que falta:
`if (nombre === undefined) return "?";`, o da un valor por defecto
`(nombre = "?")`.

### Leer un campo que solo existe en un caso

```typescript norun
type Circulo = { kind: "circulo"; radio: number };
type Cuadrado = { kind: "cuadrado"; lado: number };

function describir(f: Circulo | Cuadrado) {
  return f.radio;
}
```

```text
main.ts(5,12): error TS2339: Property 'radio' does not exist on type 'Circulo | Cuadrado'.
  Property 'radio' does not exist on type 'Cuadrado'.
```

Un cuadrado no tiene radio. Comprueba primero la etiqueta:
`if (f.kind === "circulo") return f.radio;`.

### Olvidar un caso en el switch

```typescript norun
type Forma =
  | { kind: "circulo"; radio: number }
  | { kind: "cuadrado"; lado: number }
  | { kind: "triangulo"; base: number; altura: number };

function area(f: Forma): number {
  switch (f.kind) {
    case "circulo":
      return Math.PI * f.radio ** 2;
    case "cuadrado":
      return f.lado ** 2;
    default: {
      const imposible: never = f;
      return imposible;
    }
  }
}
```

```text
main.ts(13,13): error TS2322: Type '{ kind: "triangulo"; base: number; altura: number; }' is not assignable to type 'never'.
```

El mensaje nombra el caso olvidado: el triángulo. Añade
`case "triangulo": return (f.base * f.altura) / 2;`.

## Ponte a prueba

**Pregunta 1:** en este código, ¿cuál es el tipo de `x` dentro del `if`?

```typescript norun
function f(x: string | number | boolean) {
  if (typeof x === "number") {
    // ?
  }
}
```

> **Respuesta**
>
> `number`. La comprobación `typeof x === "number"` estrecha la unión al único tipo
> `number`. Después del `if`, `x` todavía puede ser `string | number | boolean`, porque
> el bloque no termina con un `return`.

**Pregunta 2:** ¿para qué sirve el campo `kind` en una unión discriminada?

> **Respuesta**
>
> Es una etiqueta. Cada tipo de la unión tiene un valor distinto para `kind`. Al
> comprobar `kind` (con `if` o `switch`), TypeScript sabe exactamente qué tipo tienes,
> y por tanto qué campos están disponibles.

**Pregunta 3:** escribe una guarda de tipo `esTexto` que diga si un valor `unknown`
es un `string`.

> **Respuesta**
>
> ```typescript
> function esTexto(x: unknown): x is string {
>   return typeof x === "string";
> }
>
> const valor: unknown = "hola";
> if (esTexto(valor)) {
>   console.log(valor.toUpperCase());
> }
> ```
>
> ```sortie
> HOLA
> ```

## En resumen

- Un tipo unión `A | B` acepta un valor de tipo `A` o de tipo `B`.
- Con una unión, solo se permite lo que funciona para todos los tipos.
- `typeof`, `in`, `instanceof`, `===` y la prueba de veracidad estrechan la unión.
- Una unión discriminada lleva una etiqueta común (`kind`) que se comprueba con
  `switch`.
- `const imposible: never = x` en el `default` señala cualquier caso olvidado.
- Una guarda de tipo `x is T` encierra una comprobación en una función reutilizable.

Siguiente: [las clases](types.md)
