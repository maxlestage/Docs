# Primeros pasos

Esta página presenta los ladrillos básicos de TypeScript: las variables, los tipos simples, la impresión en pantalla, los comentarios y la lectura de un error de tipo.

> **Objetivos:**
>
> - sabrás crear una variable con `let` o `const`;
> - sabrás anotar una variable con `number`, `string` o `boolean`;
> - sabrás cuándo TypeScript adivina el tipo por sí solo;
> - sabrás usar `null`, `undefined`, `any` y `unknown`;
> - sabrás leer un error de tipo línea por línea.

## Mostrar con console.log

Un programa que calcula sin mostrar nada no te sirve de mucho: no ves el resultado.
`console.log` es la ventana por la que te habla.

`console.log` muestra un valor en la terminal y luego pasa a la línea siguiente. Acepta
varios valores, separados por comas: los muestra con un espacio entre
ellos.

```typescript
console.log("¡Hola!");
console.log(2 + 3);
console.log("Total:", 2 + 3, "euros");
```

```sortie
¡Hola!
5
Total: 5 euros
```

Línea por línea:

- la primera línea muestra el texto entre comillas, tal cual;
- la segunda calcula `2 + 3` y luego muestra el resultado, `5`;
- la tercera recibe tres valores: un texto, un cálculo y otro texto. Los muestra en
  una sola línea, separados por un espacio.

Un texto se escribe entre comillas dobles `"..."` o simples `'...'`. Sin comillas,
TypeScript calcula: `2 + 3` da `5`.

## Las variables: let y const

Un programa a menudo necesita recordar cosas: el nombre de un jugador, sus puntos, un
precio. Una variable guarda un valor para usarlo más adelante, como un número anotado
en un pósit para no olvidarlo.

Una [variable](../debuter/glossaire.md#variable) es una caja con una etiqueta:
la etiqueta es el nombre, el contenido es el valor. Dos palabras permiten crear una:

- `const`: una caja **sellada**, su valor ya no cambia;
- `let`: una caja **abierta**, se puede reemplazar su valor.

```typescript
const nombre = "Alice";
let puntos = 10;
console.log(nombre, puntos);

puntos = 15;
puntos = puntos + 5;
console.log(nombre, puntos);
```

```sortie
Alice 10
Alice 20
```

Línea por línea:

- `const nombre = "Alice"` guarda el texto `"Alice"` en la caja `nombre`, para siempre;
- `let puntos = 10` crea una caja abierta, `puntos`, que contiene 10;
- el primer `console.log` muestra el contenido de las dos cajas: `Alice 10`;
- `puntos = 15` reemplaza 10 por 15, y luego `puntos = puntos + 5` lo sube a 20;
- por eso el segundo `console.log` muestra `Alice 20`.

`puntos = puntos + 5` se lee de derecha a izquierda: se calcula `puntos + 5` (o sea `20`),
y luego se guarda el resultado en `puntos`.

> **Recuerda:** usa `const` por defecto. Pasa a `let` solo si el valor
> tiene que cambiar. Así quien lee sabe enseguida qué se mueve en el programa.

Un nombre de variable empieza por una letra, sin espacios y mejor sin tildes. Los
nombres compuestos se escriben en `camelCase`: `precioTotal`, `numeroDeJugadores`.

> **Nota:** a veces verás `var` en programas antiguos. Es el antepasado de
> `let`, con reglas más traicioneras. No lo uses.

**Te toca:** escribe un programa que cree una constante `fruta` que valga
`"manzana"` y una variable `cantidad` que valga 3, sume 2 a `cantidad` y luego muestre
las dos.

> **Solución**
>
> ```typescript
> const fruta = "manzana";
> let cantidad = 3;
> cantidad = cantidad + 2;
> console.log(fruta, cantidad);
> ```
>
> ```sortie
> manzana 5
> ```

## Los tipos simples

Un número y un texto no se manejan igual: puedes multiplicar 30 por 2, pero no
«Sevilla». El tipo dice qué clase de valor contiene una caja. TypeScript lo usa para
detenerte antes de que cometas un error.

Cada valor tiene un [tipo](../debuter/glossaire.md#tipo). Los tres tipos básicos son:

| Tipo | Contenido | Ejemplos |
|------|---------|----------|
| `number` | un número, entero o decimal | `42`, `-3`, `3.14` |
| `string` | un texto (una [cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres)) | `"Alice"`, `'sí'` |
| `boolean` | verdadero o falso (un [booleano](../debuter/glossaire.md#booleano)) | `true`, `false` |

Puedes escribir el tipo después del nombre, con dos puntos: es una **anotación**.

```typescript
const edad: number = 30;
const ciudad: string = "Sevilla";
const esMayor: boolean = true;

console.log(edad, ciudad, esMayor);
console.log(typeof edad, typeof ciudad, typeof esMayor);
```

```sortie
30 Sevilla true
number string boolean
```

Línea por línea:

- `const edad: number = 30` crea una caja `edad` que solo puede contener un número, y
  guarda 30 en ella;
- las dos líneas siguientes hacen lo mismo con un texto y un booleano;
- el primer `console.log` muestra los tres valores;
- el segundo muestra el tipo de cada uno, gracias a `typeof`.

`typeof` da el tipo de un valor durante la ejecución, en forma de texto.

### Calcular con números

Solo hay un tipo para todos los números: `number`. Las operaciones habituales:

```typescript
console.log(7 + 2, 7 - 2, 7 * 2);
console.log(7 / 2);
console.log(7 % 2);
console.log(2 ** 10);
console.log(Math.round(3.6), Math.floor(3.6));
console.log((10 / 3).toFixed(2));
```

```sortie
9 5 14
3.5
1
1024
4 3
3.33
```

- `/` es una división de verdad: `7 / 2` da `3.5`.
- `%` da el resto de la división (`7 = 2 × 3 + 1`).
- `**` es la potencia: `2 ** 10` vale 1024.
- `Math.round` redondea, `Math.floor` redondea hacia abajo.
- `toFixed(2)` da un texto con 2 cifras después del punto decimal.

**Te toca:** 17 caramelos se reparten entre 5 amigos. Escribe un programa que guarde
esos dos números en `caramelos` y `amigos`, y luego muestre cuántos caramelos recibe
cada amigo (con `Math.floor`) y cuántos sobran (con `%`).

> **Solución**
>
> ```typescript
> const caramelos = 17;
> const amigos = 5;
> console.log("Cada uno:", Math.floor(caramelos / amigos));
> console.log("Sobran:", caramelos % amigos);
> ```
>
> ```sortie
> Cada uno: 3
> Sobran: 2
> ```

## La inferencia: TypeScript adivina

Escribir el tipo de cada variable sería pronto pesado, sobre todo cuando salta a la
vista: `10` es claramente un número. TypeScript puede hacer ese trabajo por ti.

No estás obligado a escribir el tipo. Cuando una variable recibe un valor desde su
creación, TypeScript **adivina** su tipo: es la **inferencia**.

```typescript
let puntos = 10;
const mensaje = "Ganado";

puntos = puntos * 2;
console.log(puntos, mensaje);
```

```sortie
20 Ganado
```

Línea por línea:

- `let puntos = 10`: TypeScript ve un número, así que anota que `puntos` es un `number`;
- `const mensaje = "Ganado"`: ve un texto, así que `mensaje` es un `string`;
- `puntos = puntos * 2` guarda un número en `puntos`: está permitido, ahora vale 20.

TypeScript ha deducido que `puntos` es un `number` y `mensaje` un `string`. En tu
editor, pasa el ratón por encima de `puntos`: muestra `let puntos: number`.

El tipo adivinado es tan sólido como un tipo escrito a mano. Intentar guardar un texto
en `puntos` da un error:

```typescript norun
let puntos = 10;
puntos = "veinte";
```

```text
main.ts:2:1 - error TS2322: Type 'string' is not assignable to type 'number'.

2 puntos = "veinte";
  ~~~~~~


Found 1 error in main.ts:2
```

> **Consejo:** deja que TypeScript adivine cuando el valor es evidente
> (`let puntos = 10`). Escribe el tipo cuando la variable se crea sin valor, o
> cuando quieras dejar clara tu intención.

### const y los tipos precisos

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Con `const`, TypeScript va más lejos. Como el valor nunca cambiará, retiene
el valor exacto como tipo:

```typescript
const color = "rojo";
let otroColor = "rojo";

console.log(color, otroColor);
```

```sortie
rojo rojo
```

Pasa el ratón por encima de los dos nombres en tu editor: `color` tiene el tipo `"rojo"` (ese texto
y ningún otro), `otroColor` tiene el tipo `string`. Estos tipos precisos serán muy útiles
en [el capítulo sobre el texto](texte.md).

## Las plantillas de texto

A menudo querrás construir una frase con valores dentro: «Alice tiene 30 años.». Una
plantilla lo hace de forma sencilla, como un formulario con huecos que se rellenan.

Un texto entre comillas invertidas `` ` `` es una **plantilla** (*template literal*). Puede
contener `${...}`: la expresión entre las llaves se calcula y se inserta en el
texto.

```typescript
const nombre = "Alice";
const edad = 30;

console.log(`${nombre} tiene ${edad} años.`);
console.log(`Dentro de 5 años, tendrá ${edad + 5} años.`);
console.log(`Línea 1
Línea 2`);
```

```sortie
Alice tiene 30 años.
Dentro de 5 años, tendrá 35 años.
Línea 1
Línea 2
```

Línea por línea:

- las dos primeras líneas guardan un nombre y una edad;
- en la primera plantilla, `${nombre}` se reemplaza por `Alice` y `${edad}` por `30`;
- en la segunda, `${edad + 5}` se calcula primero (35) y luego se inserta;
- la tercera plantilla contiene un salto de línea de verdad: se muestra en dos líneas.

Una plantilla también puede ocupar varias líneas.

Con comillas normales, hay que pegar los trozos con `+`:

```typescript
const nombre = "Alice";
console.log("Hola, " + nombre + "!");
```

```sortie
Hola, Alice!
```

**Te toca:** una noche de hotel cuesta 80 euros. Escribe un programa que guarde
`"Sevilla"` en `ciudad` y `2` en `noches`, y luego muestre con una plantilla:
«Estancia en Sevilla: 2 noches, o sea 160 euros.».

> **Solución**
>
> ```typescript
> const ciudad = "Sevilla";
> const noches = 2;
> console.log(`Estancia en ${ciudad}: ${noches} noches, o sea ${noches * 80} euros.`);
> ```
>
> ```sortie
> Estancia en Sevilla: 2 noches, o sea 160 euros.
> ```

## Los comentarios

Dentro de unas semanas, habrás olvidado por qué escribiste tal línea. Un comentario es
una nota que dejas a tu yo del futuro, o a un compañero.

Un [comentario](../debuter/glossaire.md#comentario) es una nota para los humanos:
TypeScript lo ignora.

```typescript
// Un comentario en una línea.
const iva = 0.2; // También puede ir después del código.

/*
  Un comentario
  en varias líneas.
*/

/** El precio sin impuestos, en euros. */
const precioSinIva = 100;

console.log(precioSinIva * (1 + iva));
```

```sortie
120
```

Línea por línea:

- todo lo que sigue a `//` se ignora. En la segunda línea, `const iva = 0.2;` sí se
  ejecuta: solo se ignora el texto después de `//`;
- todo lo que está entre `/*` y `*/` se ignora, aunque ocupe varias líneas;
- quedan tres instrucciones de verdad: `iva` vale 0.2, `precioSinIva` vale 100, y
  `console.log` muestra `100 * 1.2`, o sea 120.

El comentario `/** ... */` es un comentario de documentación: tu editor
lo muestra cuando pasas el ratón por encima de `precioSinIva`.

> **Consejo:** explica **por qué** haces algo, no lo que hace el
> código. `// IVA francés` es útil; `// multiplica por 0.2` no lo es.

## null y undefined: la ausencia de valor

Un dato puede faltar: un cliente no siempre tiene apodo, un formulario todavía no
está rellenado. Hace falta una forma de decir «aquí no hay nada».

Dos valores especiales representan «nada» (ver [null](../debuter/glossaire.md#null)):

- `undefined`: «todavía sin valor». Es lo que contiene una variable `let`
  creada sin valor;
- `null`: «vacío a propósito». Lo escribes tú mismo.

En modo estricto (`--strict`, el de este sitio), un `string` **no** puede contener
`null`. Si un valor puede faltar, hay que decirlo con una barra `|`, que quiere decir
«o»:

```typescript
let apodo: string | null = null;
console.log(apodo);

apodo = "Ali";
console.log(apodo);

let respuesta: number | undefined;
console.log(respuesta);
```

```sortie
null
Ali
undefined
```

Línea por línea:

- `apodo` puede contener un texto o `null`. Al principio vale `null`: no hay apodo;
- luego recibe el texto `"Ali"`, y la segunda impresión lo muestra;
- `respuesta` se crea sin valor: contiene `undefined`.

`string | null` se lee «un texto o null». Es una **unión** de tipos: verás
muchas en [el capítulo sobre las uniones](unions.md).

> **Recuerda:** en TypeScript estricto, la ausencia de valor se ve en el tipo. No
> puedes olvidarla: es la causa de muchísimos bugs en otros
> lenguajes.

**Te toca:** escribe un programa que cree una variable `telefono` de tipo
`string | null`, que valga `null` al principio. Muéstrala, dale el valor
`"0601020304"` y vuelve a mostrarla.

> **Solución**
>
> ```typescript
> let telefono: string | null = null;
> console.log(telefono);
>
> telefono = "0601020304";
> console.log(telefono);
> ```
>
> ```sortie
> null
> 0601020304
> ```

## any y unknown: cuando no se sabe

A veces, no conoces el tipo de un valor (un dato que viene de un archivo,
por ejemplo). Existen dos tipos para eso, y son muy diferentes.

### any: se desconecta la comprobación

`any` quiere decir «cualquier cosa, no compruebes nada». TypeScript lo acepta todo, incluso los
errores:

```typescript norun
let valor: any = "hola";
console.log(valor.toFixed(2));
```

`tsc` no dice nada. Pero al ejecutarlo, el programa falla (extracto del mensaje):

```text
TypeError: valor.toFixed is not a function
```

`toFixed` no existe en un texto. Con `any`, has perdido toda la protección
de TypeScript.

### unknown: se comprueba antes de usar

`unknown` quiere decir «todavía no lo sé». Puedes guardar cualquier cosa
dentro, pero para usarlo, primero debes **comprobar** qué es:

```typescript
const dato: unknown = "hola";

if (typeof dato === "string") {
  console.log(dato.toUpperCase());
}
```

```sortie
HOLA
```

Línea por línea:

- `dato` es de tipo `unknown`: TypeScript no supone nada sobre su contenido;
- `typeof dato === "string"` hace la pregunta «¿es un texto?». Aquí, sí;
- dentro del bloque, `toUpperCase()` pasa el texto a mayúsculas: `HOLA`.

Dentro del bloque `if`, TypeScript sabe que `dato` es un `string`, y permite
`toUpperCase()`. Las condiciones se explican en
[el capítulo siguiente](conditions-boucles.md).

> **Recuerda:** evita `any`. Si no conoces el tipo, usa `unknown`:
> es un paquete cerrado que se abre con cuidado, no un paquete que se tira sin
> mirar.

## Leer un error de tipo, línea por línea

Un error de tipo no es un castigo: es TypeScript avisándote de un bug antes incluso de
ejecutar el programa. Saber leerlo te ahorra mucho tiempo.

Aquí tienes un programa que suma un número y un texto:

```typescript norun
let total: number = 0;
const entrada = "42";
total = total + entrada;
```

```text
main.ts:3:1 - error TS2322: Type 'string' is not assignable to type 'number'.

3 total = total + entrada;
  ~~~~~


Found 1 error in main.ts:3
```

Leámoslo en orden:

1. `main.ts:3:1`: el problema está en `main.ts`, línea 3, columna 1.
2. `error TS2322`: el código del error. TS2322 es el más habitual: «tipo incorrecto
   para este sitio».
3. `Type 'string' is not assignable to type 'number'`: «el tipo `string` no se puede
   guardar en el tipo `number`».
4. La línea copiada, con `~~~~~` debajo de `total`: es la caja que rechaza el valor.
5. `Found 1 error`: un solo error en total.

Repasemos el razonamiento: `entrada` es un texto (`"42"`). `total + entrada` pega por tanto
un número y un texto, lo que da un **texto** (`"042"`). Y `total` solo acepta
números. La corrección: transformar el texto en número con `Number(...)`.

```typescript
let total: number = 0;
const entrada = "42";
total = total + Number(entrada);
console.log(total);
```

```sortie
42
```

> **Consejo:** cuando el mensaje es largo, busca la última frase en inglés: es la
> que describe la causa más precisa.

## Errores frecuentes

### Modificar una constante

```typescript norun
const puntos = 10;
puntos = 20;
```

```text
main.ts:2:1 - error TS2588: Cannot assign to 'puntos' because it is a constant.

2 puntos = 20;
  ~~~~~~


Found 1 error in main.ts:2
```

«No se puede modificar `puntos`, es una constante.» Si el valor tiene que cambiar,
crea la variable con `let puntos = 10;`.

### Usar una variable antes de darle un valor

```typescript norun
let ciudad: string;
console.log(ciudad);
```

```text
main.ts:2:13 - error TS2454: Variable 'ciudad' is used before being assigned.

2 console.log(ciudad);
              ~~~~~~


Found 1 error in main.ts:2
```

`ciudad` está declarada, pero vacía. Dale un valor antes de usarla:
`let ciudad: string = "Sevilla";`.

### Guardar null en un string

```typescript norun
let apodo: string = null;
```

```text
main.ts:1:5 - error TS2322: Type 'null' is not assignable to type 'string'.

1 let apodo: string = null;
      ~~~~~


Found 1 error in main.ts:1
```

En modo estricto, `string` quiere decir «siempre un texto». Si el valor puede faltar,
dilo: `let apodo: string | null = null;`.

### Usar un valor unknown sin comprobar

```typescript norun
const dato: unknown = "hola";
console.log(dato.toUpperCase());
```

```text
main.ts:2:13 - error TS18046: 'dato' is of type 'unknown'.

2 console.log(dato.toUpperCase());
              ~~~~


Found 1 error in main.ts:2
```

TypeScript se niega: `dato` podría ser un número. Comprueba primero con
`if (typeof dato === "string") { ... }`.

## Ponte a prueba

**Pregunta 1:** ¿cuál es el tipo de `x` después de `let x = 3.5;`?

> **Respuesta**
>
> `number`. TypeScript lo adivina (inferencia): solo hay un tipo para todos los
> números, enteros o decimales.

**Pregunta 2:** ¿qué muestra este código?

```typescript norun
const a = 4;
console.log(`${a} x 2 = ${a * 2}`);
```

> **Respuesta**
>
> `4 x 2 = 8`. Cada `${...}` se calcula y se inserta en la plantilla.

**Pregunta 3:** ¿cuál es la diferencia entre `any` y `unknown`?

> **Respuesta**
>
> Los dos aceptan cualquier valor. Pero `any` desconecta la comprobación:
> todo está permitido, incluso los errores. `unknown` obliga a comprobar el tipo (con
> `typeof`, por ejemplo) antes de usar el valor.

## En resumen

- `const` crea una variable fija, `let` una variable modificable; prefiere `const`.
- Los tipos básicos son `number`, `string` y `boolean`; se escriben después de `:`.
- TypeScript adivina el tipo de una variable que recibe un valor: es la inferencia.
- En modo estricto, `null` y `undefined` deben aparecer en el tipo:
  `string | null`.
- Prefiere `unknown` a `any`: obliga a comprobar antes de usar.
- Una plantilla `` `...${x}...` `` inserta valores en un texto; `//` empieza un
  comentario.

Siguiente: [Nombres et calculs](nombres.md)
