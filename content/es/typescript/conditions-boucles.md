# Condiciones y bucles

Esta página explica cómo tomar decisiones con `if`, `switch` y el operador ternario, cómo repetir acciones con bucles, y lo que aportan los tipos en cada caso.

> **Objetivos:**
>
> - sabrás escribir una condición con `if`, `else if` y `else`;
> - sabrás comparar con `===` y reconocer los valores «falsos»;
> - sabrás manejar un valor ausente con `??` y `?.`;
> - sabrás escribir un `switch` que no olvida ningún caso gracias a `never`;
> - sabrás repetir con `for`, `while` y `for...of`.

## if y else

Una [condición](../debuter/glossaire.md#condicion) elige qué código ejecutar. `if`
(«si») ejecuta su bloque `{ ... }` solo si la prueba entre paréntesis es verdadera.
`else` («si no») da el bloque que se ejecuta en caso contrario.

```typescript
const temperatura: number = 28;

if (temperatura > 25) {
  console.log("Hace calor.");
} else {
  console.log("Hace buen tiempo.");
}
```

```sortie
Hace calor.
```

Para más de dos casos, encadena con `else if` («si no, si»). Las pruebas se leen
en orden; la primera verdadera gana, las demás se ignoran.

```typescript
const nota: number = 13;

if (nota >= 16) {
  console.log("Sobresaliente");
} else if (nota >= 12) {
  console.log("Notable");
} else if (nota >= 10) {
  console.log("Aprobado");
} else {
  console.log("Suspenso");
}
```

```sortie
Notable
```

## Comparar con ===

Los operadores de comparación dan un `boolean` (`true` o `false`):

| Operador | Significado |
|-----------|------|
| `===` | igual a |
| `!==` | distinto de |
| `<`, `<=` | menor, menor o igual |
| `>`, `>=` | mayor, mayor o igual |

Se combinan con `&&` («y»), `||` («o») y `!` («no»):

```typescript
const edad: number = 20;
const tieneEntrada: boolean = true;

console.log(edad === 20, edad !== 20);
console.log(edad >= 18 && tieneEntrada);
console.log(edad < 12 || edad > 65);
console.log(!tieneEntrada);
```

```sortie
true false
true
false
false
```

> **Atención:** escribe siempre `===` (tres signos), nunca `==` (dos). `==`
> transforma los valores antes de comparar: ¡`0 == ""` vale `true` en JavaScript!
> Y un solo `=` no compara: guarda un valor en una variable.

TypeScript vigila tus comparaciones. Comparar un número y un texto siempre da
falso: seguramente es un error, y te lo dice (ver los
[errores frecuentes](#errores-frecuentes)).

## Los valores «verdaderos» y «falsos»

Un `if` acepta cualquier valor, no solo un `boolean`. Seis valores
cuentan como falsos (*falsy*):

- `false`, `0`, `NaN` («no es un número»), `""` (el texto vacío), `null` y `undefined`.

Todos los demás cuentan como verdaderos (*truthy*), incluso `"0"`, `" "` o `[]`.

```typescript
const texto = "";

if (texto) {
  console.log("El texto está lleno.");
} else {
  console.log("El texto está vacío.");
}

console.log(Boolean(0), Boolean(""), Boolean(null));
console.log(Boolean("0"), Boolean(" "), Boolean([]));
```

```sortie
El texto está vacío.
false false false
true true true
```

`Boolean(x)` muestra cómo ve un `if` el valor `x`.

> **Consejo:** cuando sea posible, escribe la prueba completa: `texto !== ""` o
> `numero !== 0`. Quien lee entiende enseguida qué estás probando.

## El operador ternario

El ternario `prueba ? siVerdadero : siFalso` es un `if/else` que **da un valor**.
Cabe en una línea.

```typescript
const edad: number = 15;
const estado = edad >= 18 ? "mayor de edad" : "menor de edad";
console.log(estado);

const puntos: number = 1;
console.log(`${puntos} punto${puntos > 1 ? "s" : ""}`);
```

```sortie
menor de edad
1 punto
```

Léelo como una pregunta: «¿edad ≥ 18? entonces "mayor de edad", si no "menor de edad"».

> **Consejo:** reserva el ternario para las elecciones simples. Si hay que anidarlo, un
> `if/else` es más legible.

## switch: elegir entre varios valores

`switch` compara un valor con una lista de casos (`case`). Ejecuta el código del caso que
coincide. `break` sale del `switch`; `default` trata todos los demás casos.

```typescript
const dia: string = "sábado";

switch (dia) {
  case "sábado":
  case "domingo":
    console.log("¡Fin de semana!");
    break;
  case "miércoles":
    console.log("Mitad de semana.");
    break;
  default:
    console.log("Día de trabajo.");
}
```

```sortie
¡Fin de semana!
```

Dos `case` seguidos comparten el mismo código: `sábado` y `domingo` muestran
«¡Fin de semana!».

> **Atención:** sin `break`, la ejecución continúa en el caso siguiente. Olvidar
> `break` es una trampa clásica.

## Los bucles

Un [bucle](../debuter/glossaire.md#bucle) repite un bloque de código.

### for: contar

El bucle `for` tiene tres partes, separadas por puntos y coma: el inicio, la
condición para continuar, y el paso.

```typescript
for (let i = 1; i <= 3; i++) {
  console.log(`Vuelta ${i}`);
}
```

```sortie
Vuelta 1
Vuelta 2
Vuelta 3
```

- `let i = 1`: el contador empieza en 1;
- `i <= 3`: se continúa mientras `i` valga como mucho 3;
- `i++`: después de cada vuelta, se suma 1 a `i`.

### while: mientras

`while` («mientras») repite mientras su condición sea verdadera. Úsalo cuando
no sabes de antemano cuántas vueltas hacen falta.

```typescript
let dinero = 100;
let anios = 0;

while (dinero < 150) {
  dinero = dinero * 1.1;
  anios++;
}

console.log(`${anios} años, ${dinero.toFixed(2)} euros`);
```

```sortie
5 años, 161.05 euros
```

> **Atención:** si la condición sigue siendo siempre verdadera, el bucle no se detiene nunca.
> Comprueba que algo cambia en cada vuelta. Para detener un programa bloqueado,
> pulsa `Ctrl+C` en la terminal.

### for...of: recorrer una lista

`for...of` toma los elementos de una lista uno a uno. La lista se escribe entre corchetes;
los [arrays](collections.md) se explican más adelante.

```typescript
const frutas = ["manzana", "pera", "kiwi"];

for (const fruta of frutas) {
  console.log(fruta.toUpperCase());
}
```

```sortie
MANZANA
PERA
KIWI
```

TypeScript sabe que `frutas` contiene `string`: por tanto `fruta` es un `string`, y
`toUpperCase()` está permitido. Basta con un `const`: cada vuelta crea una nueva `fruta`.

### break y continue

- `break` sale del bucle de inmediato;
- `continue` pasa directamente a la vuelta siguiente.

```typescript
for (let i = 1; i <= 10; i++) {
  if (i % 2 === 0) {
    continue;
  }
  if (i > 7) {
    break;
  }
  console.log(i);
}
```

```sortie
1
3
5
7
```

Los números pares se saltan (`continue`), y el bucle se detiene en 9 (`break`).

## El estrechamiento: los tipos siguen tus pruebas

Esto es lo que aportan los tipos a las condiciones. Cuando una variable puede tener
varios tipos, una prueba **estrecha** su tipo dentro del bloque (*narrowing*). Es como
un portero a la entrada de una sala: una vez pasada la puerta, se sabe quién está dentro.

```typescript
const respuestas: (string | number)[] = ["sí", 3.14159];

for (const respuesta of respuestas) {
  if (typeof respuesta === "string") {
    console.log(respuesta.toUpperCase(), "es un texto");
  } else {
    console.log(respuesta.toFixed(2), "es un número");
  }
}
```

```sortie
SÍ es un texto
3.14 es un número
```

`(string | number)[]` es una lista en la que cada elemento es un texto o un número.

En el primer bloque, TypeScript sabe que `respuesta` es un `string`: permite
`toUpperCase()`. En el `else`, sabe que es un `number`: permite
`toFixed()`. No hace falta repetírselo.

## Los valores ausentes: ?? y ?.

Un valor de tipo `string | undefined` puede faltar. Dos operadores ayudan a
manejarlo sin riesgo.

### ??: un valor por defecto

`a ?? b` da `a`, salvo si `a` vale `null` o `undefined`: entonces da `b`.

```typescript
const volumenes: (number | null)[] = [7, 0, null];

for (const volumen of volumenes) {
  console.log(volumen ?? 5, volumen || 5);
}
```

```sortie
7 7
0 5
5 5
```

Compara las dos columnas: `||` reemplaza **todos** los valores falsos, incluido
`0`. `??` solo reemplaza `null` y `undefined`. Un volumen a `0` es un ajuste de verdad:
`??` lo conserva.

> **Recuerda:** para un valor por defecto, prefiere `??` a `||`.

### ?.: llamar solo si el valor existe

`x?.accion()` llama a `accion()` solo si `x` existe. Si no, el resultado es
`undefined`, sin error.

```typescript
const entradas: (string | undefined)[] = ["alice", undefined];

for (const entrada of entradas) {
  console.log(entrada?.toUpperCase());
  console.log(entrada?.toUpperCase() ?? "(nada)");
}
```

```sortie
ALICE
ALICE
undefined
(nada)
```

Sin el `?.`, TypeScript rechazaría `entrada.toUpperCase()`: `entrada` quizá sea
`undefined`. A menudo se combinan `?.` y `??`: «el nombre en mayúsculas, o "(nada)"».

## Un switch que no olvida ningún caso

Esta es una técnica que solo permite un lenguaje tipado. Un tipo puede enumerar sus valores
posibles: `"rojo" | "ámbar" | "verde"` (un tipo que solo acepta esos tres textos).
La palabra `type` le da un nombre a ese tipo.

```typescript
type Semaforo = "rojo" | "ámbar" | "verde";

const semaforos: Semaforo[] = ["verde", "ámbar", "rojo"];

for (const luz of semaforos) {
  switch (luz) {
    case "rojo":
      console.log("Para");
      break;
    case "ámbar":
      console.log("Frena");
      break;
    case "verde":
      console.log("Pasa");
      break;
    default: {
      const imposible: never = luz;
      console.log(imposible);
    }
  }
}
```

```sortie
Pasa
Frena
Para
```

El tipo `never` quiere decir «ningún valor posible». En el `default`, todos los casos
ya se han tratado: `luz` ya no puede valer nada, así que su tipo es `never`, y la
línea se acepta.

La ventaja llega el día en que añades un color, por ejemplo
`"intermitente"`, sin acordarte de modificar el `switch`. Ese caso cae entonces en el
`default`, y `tsc` se niega a guardar `"intermitente"` en una variable `never`:

```text
main.ts:15:13 - error TS2322: Type '"intermitente"' is not assignable to type 'never'.

15       const imposible: never = luz;
               ~~~~~~~~~


Found 1 error in main.ts:15
```

El error nombra exactamente el caso olvidado. Es una alarma: «has añadido un
valor, acuérdate de tratarlo en todas partes».

## Errores frecuentes

### Comparar un número y un texto

```typescript norun
const edad: number = 18;
if (edad === "18") {
  console.log("Mayor de edad");
}
```

```text
main.ts:2:5 - error TS2367: This comparison appears to be unintentional because the types 'number' and 'string' have no overlap.

2 if (edad === "18") {
      ~~~~~~~~~~~~~


Found 1 error in main.ts:2
```

«Esta comparación parece involuntaria: los tipos `number` y `string` no tienen nada en
común.» Un número nunca es igual a un texto: el `if` siempre sería falso.
Compara con el número: `edad === 18`.

### Usar un valor que quizá falte

```typescript norun
const nombres = ["Alice", "Bob"];
const encontrado = nombres.find((n) => n.startsWith("Z"));
console.log(encontrado.toUpperCase());
```

```text
main.ts:3:13 - error TS18048: 'encontrado' is possibly 'undefined'.

3 console.log(encontrado.toUpperCase());
              ~~~~~~~~~~


Found 1 error in main.ts:3
```

`find` busca un elemento y devuelve `undefined` si no encuentra nada (se verá en detalle
en [los arrays](collections.md)). TypeScript te obliga a prever este caso. Escribe
`encontrado?.toUpperCase()`, o prueba primero: `if (encontrado !== undefined) { ... }`.

### Usar el contador después del bucle

```typescript norun
for (let i = 0; i < 3; i++) {
  console.log(i);
}
console.log(i);
```

```text
main.ts:4:13 - error TS2304: Cannot find name 'i'.

4 console.log(i);
              ~


Found 1 error in main.ts:4
```

Una variable creada con `let` solo existe dentro de su bloque: es su
[ámbito](../debuter/glossaire.md#ambito). Si necesitas el valor después del
bucle, crea la variable antes: `let i = 0; for (; i < 3; i++) { ... }`.

### Olvidar un caso en un switch exhaustivo

Has añadido `"intermitente"` al tipo `Semaforo`, pero no al `switch`:

```typescript norun
type Semaforo = "rojo" | "ámbar" | "verde" | "intermitente";
const semaforos: Semaforo[] = ["rojo", "ámbar", "verde"];
for (const luz of semaforos) {
  switch (luz) {
    case "rojo":
      console.log("Para");
      break;
    case "ámbar":
      console.log("Frena");
      break;
    case "verde":
      console.log("Pasa");
      break;
    default: {
      const imposible: never = luz;
      console.log(imposible);
    }
  }
}
```

```text
main.ts:15:13 - error TS2322: Type '"intermitente"' is not assignable to type 'never'.

15       const imposible: never = luz;
               ~~~~~~~~~


Found 1 error in main.ts:15
```

Añade el caso que falta: `case "intermitente": console.log("Precaución"); break;`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```typescript norun
const stock: number | null = 0;
console.log(stock ?? 10, stock || 10);
```

> **Respuesta**
>
> `0 10`. `??` solo reemplaza `null` y `undefined`: conserva `0`. `||` reemplaza
> todos los valores falsos, entre ellos `0`.

**Pregunta 2:** escribe un bucle que muestre los números de 10 a 0, de 2 en 2.

> **Respuesta**
>
> ```typescript
> for (let i = 10; i >= 0; i -= 2) {
>   console.log(i);
> }
> ```
>
> ```sortie
> 10
> 8
> 6
> 4
> 2
> 0
> ```
>
> `i -= 2` es un atajo de `i = i - 2`.

**Pregunta 3:** ¿para qué sirve `const imposible: never = luz;` en el `default` de un
`switch`?

> **Respuesta**
>
> Para comprobar que todos los casos están tratados. Si se olvida un valor del tipo,
> llega al `default`, y `tsc` se niega a guardarlo en una variable `never`:
> el error nombra el caso olvidado.

## En resumen

- `if`, `else if` y `else` eligen un bloque; el ternario `a ? b : c` elige un
  valor.
- Compara con `===` y `!==`, nunca con `==`.
- `false`, `0`, `NaN`, `""`, `null` y `undefined` cuentan como falsos en un `if`.
- Una prueba como `typeof x === "string"` estrecha el tipo de `x` dentro del bloque.
- `??` da un valor por defecto, `?.` llama a una acción solo si el valor
  existe.
- `for` cuenta, `while` repite mientras, `for...of` recorre una lista; un `switch`
  con `never` en `default` no olvida ningún caso.

Siguiente: [las funciones](fonctions.md).
