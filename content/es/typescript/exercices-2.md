# Ejercicios resueltos: segunda serie

Esta página propone 12 nuevos ejercicios resueltos en TypeScript, del más fácil al más difícil, con pistas, soluciones explicadas y tablas de seguimiento.

## Cómo usar esta página

Estos ejercicios son **nuevos**: no repiten ni [los ejercicios para
empezar](exercices-faciles.md) ni [la primera serie](exercices.md). Cada uno tiene un
nivel:

- ★☆☆: fácil, solo con lo básico;
- ★★☆: medio, con arrays, objetos y tipos;
- ★★★: más difícil, con genéricos, clases y uniones.

Para cada ejercicio:

1. Lee el enunciado y la salida esperada.
2. Haz los pasos en papel, con una [tabla de seguimiento](pas-a-pas.md).
3. Escribe tu código en `ejercicio.ts`, compruébalo con
   `npx tsc --noEmit --strict ejercicio.ts` y luego ejecútalo con `npx tsx ejercicio.ts`.
4. ¿Atascado? Lee la pista. Abre la solución al final.

> **Consejo:** una solución distinta de la nuestra puede ser correcta. Lo que cuenta:
> la salida correcta, y ningún error de `tsc --strict`.

## Primeros pasos

### Ejercicio 1: De Celsius a Fahrenheit

Nivel: ★☆☆

Escribe una función `aFahrenheit(celsius: number): number`. La fórmula es:
Fahrenheit = Celsius × 9 / 5 + 32. Muestra la conversión de 0, 20, 37 y 100 grados.

Salida esperada:

```text
0 °C = 32 °F
20 °C = 68 °F
37 °C = 98.6 °F
100 °C = 212 °F
```

> **Pista:** escribe primero la función y pruébala con `0` (el resultado debe ser
> `32`). Luego recorre `[0, 20, 37, 100]` con `for...of`.

> **Solución**
>
> ```typescript
> function aFahrenheit(celsius: number): number {
>   return (celsius * 9) / 5 + 32;
> }
>
> for (const c of [0, 20, 37, 100]) {
>   console.log(`${c} °C = ${aFahrenheit(c)} °F`);
> }
> ```
>
> ```sortie
> 0 °C = 32 °F
> 20 °C = 68 °F
> 37 °C = 98.6 °F
> 100 °C = 212 °F
> ```
>
> Por qué funciona: la función es una pequeña máquina de convertir. El bucle le da
> cada temperatura, una a una.
>
> | Vuelta | c | c × 9 | ÷ 5 | + 32 |
> |---|---|---|---|---|
> | 1 | 0 | 0 | 0 | 32 |
> | 2 | 20 | 180 | 36 | 68 |
> | 3 | 37 | 333 | 66.6 | 98.6 |
> | 4 | 100 | 900 | 180 | 212 |

### Ejercicio 2: El mayor de los tres

Nivel: ★☆☆

Escribe una función `elMayor(a: number, b: number, c: number): number` que devuelva
el mayor de los tres números, **sin** usar `Math.max`. Pruébala con `(4, 9, 2)`,
`(7, 3, 5)` y `(1, 1, 8)`.

Salida esperada:

```text
9
7
8
```

> **Pista:** guarda `a` en una variable `max`. Luego compara `b` con `max`, y después
> `c` con `max`, con dos `if`.

> **Solución**
>
> ```typescript
> function elMayor(a: number, b: number, c: number): number {
>   let max = a;
>   if (b > max) max = b;
>   if (c > max) max = c;
>   return max;
> }
>
> console.log(elMayor(4, 9, 2));
> console.log(elMayor(7, 3, 5));
> console.log(elMayor(1, 1, 8));
> ```
>
> ```sortie
> 9
> 7
> 8
> ```
>
> Por qué funciona: `max` es el campeón del momento. Cada número nuevo lo desafía; si
> es mayor, ocupa su lugar.

### Ejercicio 3: Escribir una palabra al revés

Nivel: ★☆☆

Escribe una función `alReves(palabra: string): string` que devuelva la palabra escrita
de derecha a izquierda, con un bucle (sin `reverse`). Pruébala con `"sopas"`.

Salida esperada:

```text
sapos
```

> **Pista:** parte de un texto vacío `let resultado = "";`. Recorre las letras con
> `for (const letra of palabra)`.

> **Pista:** pega cada letra **delante** del resultado: `resultado = letra + resultado;`.

> **Solución**
>
> ```typescript
> function alReves(palabra: string): string {
>   let resultado = "";
>   for (const letra of palabra) {
>     resultado = letra + resultado;
>   }
>   return resultado;
> }
>
> console.log(alReves("sopas"));
> ```
>
> ```sortie
> sapos
> ```
>
> Por qué funciona: cada letra nueva pasa **delante** de las demás, como un plato
> puesto encima de una pila.
>
> | Vuelta | letra | resultado |
> |---|---|---|
> | inicio | | `""` |
> | 1 | s | `"s"` |
> | 2 | o | `"os"` |
> | 3 | p | `"pos"` |
> | 4 | a | `"apos"` |
> | 5 | s | `"sapos"` |

### Ejercicio 4: La suma de las cifras

Nivel: ★☆☆

Escribe una función `sumaCifras(n: number): number` que sume las cifras de un entero
positivo: para 4721, es 4 + 7 + 2 + 1 = 14. No uses texto: solo cálculos.

Salida esperada:

```text
14
```

> **Pista:** `n % 10` da la última cifra (4721 % 10 = 1).

> **Pista:** `Math.floor(n / 10)` quita la última cifra (4721 → 472). Repite con
> `while (n > 0)`.

> **Solución**
>
> ```typescript
> function sumaCifras(n: number): number {
>   let suma = 0;
>   while (n > 0) {
>     suma += n % 10;
>     n = Math.floor(n / 10);
>   }
>   return suma;
> }
>
> console.log(sumaCifras(4721));
> ```
>
> ```sortie
> 14
> ```
>
> Por qué funciona: nos «comemos» el número por la derecha, una cifra por vuelta.
>
> | Vuelta | n al inicio | n % 10 | suma | n al final |
> |---|---|---|---|---|
> | 1 | 4721 | 1 | 1 | 472 |
> | 2 | 472 | 2 | 3 | 47 |
> | 3 | 47 | 7 | 10 | 4 |
> | 4 | 4 | 4 | 14 | 0 |
>
> En la vuelta siguiente, la prueba `0 > 0` es falsa: el bucle se detiene.

## Arrays, objetos y tipos

### Ejercicio 5: El palíndromo

Nivel: ★★☆

Un palíndromo se lee igual en los dos sentidos, si ignoramos las mayúsculas y los
espacios: «Anita lava la tina». Escribe `esPalindromo(frase: string): boolean` y
pruébala con tres frases.

Salida esperada:

```text
kayak: sí
Anita lava la tina: sí
TypeScript: no
```

> **Pista:** limpia primero la frase: `toLowerCase()`, y luego `replaceAll(" ", "")`.

> **Pista:** para dar la vuelta a un texto: `[...texto].reverse().join("")`.

> **Solución**
>
> ```typescript
> function esPalindromo(frase: string): boolean {
>   const limpia = frase.toLowerCase().replaceAll(" ", "");
>   const inversa = [...limpia].reverse().join("");
>   return limpia === inversa;
> }
>
> for (const f of ["kayak", "Anita lava la tina", "TypeScript"]) {
>   console.log(`${f}: ${esPalindromo(f) ? "sí" : "no"}`);
> }
> ```
>
> ```sortie
> kayak: sí
> Anita lava la tina: sí
> TypeScript: no
> ```
>
> Por qué funciona: comparamos la frase limpia con su reflejo en un espejo.
> `[...limpia]` corta el texto en un array de letras, `reverse` le da la vuelta,
> `join("")` vuelve a pegar las letras.

### Ejercicio 6: Contar las letras

Nivel: ★★☆

Escribe una función `contarLetras(palabra: string): Record<string, number>` que cuente
cuántas veces aparece cada letra. Muestra el resultado para `"banana"`, una letra por
línea.

Salida esperada:

```text
b: 1
a: 3
n: 2
```

> **Pista:** un objeto `Record<string, number>` sirve de libreta: la letra es la
> clave, el número es el valor. Empieza con `{}`.

> **Pista:** `cuenta[letra] = (cuenta[letra] ?? 0) + 1;` gestiona el primer
> encuentro, cuando la casilla todavía no existe.

> **Solución**
>
> ```typescript
> function contarLetras(palabra: string): Record<string, number> {
>   const cuenta: Record<string, number> = {};
>   for (const letra of palabra) {
>     cuenta[letra] = (cuenta[letra] ?? 0) + 1;
>   }
>   return cuenta;
> }
>
> const resultado = contarLetras("banana");
> for (const [letra, numero] of Object.entries(resultado)) {
>   console.log(`${letra}: ${numero}`);
> }
> ```
>
> ```sortie
> b: 1
> a: 3
> n: 2
> ```
>
> Por qué funciona: la primera vez que vemos una letra, su casilla vale `undefined`,
> y `?? 0` la sustituye por 0.
>
> | Vuelta | letra | cuenta después de la vuelta |
> |---|---|---|
> | 1 | b | `{ b: 1 }` |
> | 2 | a | `{ b: 1, a: 1 }` |
> | 3 | n | `{ b: 1, a: 1, n: 1 }` |
> | 4 | a | `{ b: 1, a: 2, n: 1 }` |
> | 5 | n | `{ b: 1, a: 2, n: 2 }` |
> | 6 | a | `{ b: 1, a: 3, n: 2 }` |

### Ejercicio 7: El mínimo y el máximo

Nivel: ★★☆

Escribe una función `limites(numeros: readonly number[])` que devuelva un objeto
`{ min, max }` en **un solo** recorrido del array, o `undefined` si el array está
vacío. Muestra los límites de las temperaturas `[14, 9, 21, 17, 6]`, y luego los de un
array vacío.

Salida esperada:

```text
min = 6, max = 21
array vacío
```

> **Pista:** el tipo de retorno es `{ min: number; max: number } | undefined`.

> **Pista:** trata primero el caso vacío con un `return`. Después, parte de
> `numeros[0]` para `min` y `max`.

> **Solución**
>
> ```typescript
> type Limites = { min: number; max: number };
>
> function limites(numeros: readonly number[]): Limites | undefined {
>   if (numeros.length === 0) return undefined;
>   let min = numeros[0];
>   let max = numeros[0];
>   for (const n of numeros) {
>     if (n < min) min = n;
>     if (n > max) max = n;
>   }
>   return { min, max };
> }
>
> for (const lista of [[14, 9, 21, 17, 6], []]) {
>   const l = limites(lista);
>   console.log(l === undefined ? "array vacío" : `min = ${l.min}, max = ${l.max}`);
> }
> ```
>
> ```sortie
> min = 6, max = 21
> array vacío
> ```
>
> Por qué funciona: `min` y `max` son dos campeones que conservan su título mientras
> nadie los supere. El tipo `Limites | undefined` obliga a quien llama a comprobar el
> caso vacío: sin la prueba `l === undefined`, `tsc` rechazaría `l.min`.

### Ejercicio 8: Ordenar la compra por sección

Nivel: ★★☆

Una lista de la compra contiene artículos. Cada uno tiene un `nombre` y una
`seccion`: `"frescos"`, `"despensa"` o `"bebidas"`. Escribe una función `porSeccion`
que devuelva, para cada sección, la lista de nombres. Muestra una línea por sección.

Salida esperada:

```text
frescos: yogur, mantequilla
despensa: pasta, arroz
bebidas: zumo
```

> **Pista:** describe el tipo `Seccion` como una unión de textos, y el resultado como
> `Record<Seccion, string[]>`.

> **Pista:** crea el resultado con tres arrays vacíos y luego haz
> `resultado[articulo.seccion].push(articulo.nombre)` para cada artículo.

> **Solución**
>
> ```typescript
> type Seccion = "frescos" | "despensa" | "bebidas";
>
> interface Articulo {
>   nombre: string;
>   seccion: Seccion;
> }
>
> function porSeccion(articulos: readonly Articulo[]): Record<Seccion, string[]> {
>   const resultado: Record<Seccion, string[]> = { frescos: [], despensa: [], bebidas: [] };
>   for (const articulo of articulos) {
>     resultado[articulo.seccion].push(articulo.nombre);
>   }
>   return resultado;
> }
>
> const compra: Articulo[] = [
>   { nombre: "yogur", seccion: "frescos" },
>   { nombre: "pasta", seccion: "despensa" },
>   { nombre: "zumo", seccion: "bebidas" },
>   { nombre: "mantequilla", seccion: "frescos" },
>   { nombre: "arroz", seccion: "despensa" },
> ];
>
> const ordenada = porSeccion(compra);
> for (const [seccion, nombres] of Object.entries(ordenada)) {
>   console.log(`${seccion}: ${nombres.join(", ")}`);
> }
> ```
>
> ```sortie
> frescos: yogur, mantequilla
> despensa: pasta, arroz
> bebidas: zumo
> ```
>
> Por qué funciona: el resultado es un mueble con tres cajones, uno por sección. Cada
> artículo cae en el cajón escrito en su etiqueta. Gracias al tipo `Seccion`, `tsc`
> rechaza una errata como `seccion: "frscos"`.

### Ejercicio 9: Comprobar una contraseña

Nivel: ★★☆

Escribe una función `problemas(contrasena: string): string[]` que devuelva la lista de
reglas que **no** se cumplen: al menos 8 caracteres, al menos una cifra, al menos una
mayúscula. Prueba tres contraseñas.

Salida esperada:

```text
gato: demasiado corta, ninguna cifra, ninguna mayúscula
gatito2024: ninguna mayúscula
Gatito2024: perfecta
```

> **Pista:** para saber si un texto contiene una cifra: `/[0-9]/.test(texto)`. Para
> una mayúscula: `/[A-Z]/.test(texto)`.

> **Pista:** parte de un array vacío `const lista: string[] = [];` y haz un `push` por
> cada regla que no se cumple.

> **Solución**
>
> ```typescript
> function problemas(contrasena: string): string[] {
>   const lista: string[] = [];
>   if (contrasena.length < 8) lista.push("demasiado corta");
>   if (!/[0-9]/.test(contrasena)) lista.push("ninguna cifra");
>   if (!/[A-Z]/.test(contrasena)) lista.push("ninguna mayúscula");
>   return lista;
> }
>
> for (const clave of ["gato", "gatito2024", "Gatito2024"]) {
>   const lista = problemas(clave);
>   console.log(`${clave}: ${lista.length === 0 ? "perfecta" : lista.join(", ")}`);
> }
> ```
>
> ```sortie
> gato: demasiado corta, ninguna cifra, ninguna mayúscula
> gatito2024: ninguna mayúscula
> Gatito2024: perfecta
> ```
>
> Por qué funciona: los tres `if` son independientes, como tres controles seguidos.
> Cada control fallido añade una observación. Una lista vacía quiere decir que todo
> está bien. La anotación `string[]` es necesaria: el array está vacío al principio.

## Para ir más lejos

### Ejercicio 10: Dar el cambio

Nivel: ★★★

Una máquina expendedora da el cambio con el menor número de monedas posible. Las
monedas valen, en céntimos: 200, 100, 50, 20, 10, 5, 2 y 1. Escribe
`darCambio(centimos: number): number[]` que devuelva las monedas que hay que dar.
Prueba con 386 céntimos.

Salida esperada:

```text
200 + 100 + 50 + 20 + 10 + 5 + 1
7 monedas
```

> **Pista:** recorre las monedas de la más grande a la más pequeña.

> **Pista:** para cada moneda, usa `while (resto >= moneda)`: se da la moneda y se
> resta del resto, mientras se pueda.

> **Solución**
>
> ```typescript
> const MONEDAS = [200, 100, 50, 20, 10, 5, 2, 1] as const;
>
> function darCambio(centimos: number): number[] {
>   const cambio: number[] = [];
>   let resto = centimos;
>   for (const moneda of MONEDAS) {
>     while (resto >= moneda) {
>       cambio.push(moneda);
>       resto -= moneda;
>     }
>   }
>   return cambio;
> }
>
> const monedas = darCambio(386);
> console.log(monedas.join(" + "));
> console.log(`${monedas.length} monedas`);
> ```
>
> ```sortie
> 200 + 100 + 50 + 20 + 10 + 5 + 1
> 7 monedas
> ```
>
> Por qué funciona: es lo que hace un cajero. Siempre toma la moneda más grande que
> «cabe» en lo que queda por devolver.
>
> | moneda | resto antes | monedas dadas | resto después |
> |---|---|---|---|
> | 200 | 386 | 200 | 186 |
> | 100 | 186 | 100 | 86 |
> | 50 | 86 | 50 | 36 |
> | 20 | 36 | 20 | 16 |
> | 10 | 16 | 10 | 6 |
> | 5 | 6 | 5 | 1 |
> | 2 | 1 | (ninguna: 1 < 2) | 1 |
> | 1 | 1 | 1 | 0 |
>
> Trabajamos en céntimos enteros a propósito: con `3.86` €, los cálculos con decimales
> darían restos como `0.009999999`.

### Ejercicio 11: Una cola genérica

Nivel: ★★★

Escribe una clase **genérica** `Cola<T>`, como la cola de una ventanilla: se entra por
detrás, se sale por delante. Tiene tres métodos: `entrar(elemento: T)`,
`salir(): T | undefined` y `cantidad(): number`. Haz entrar a Alice, Bob y Chloé, y
luego haz salir a cuatro personas.

Salida esperada:

```text
3 personas esperando
Turno de Alice
Turno de Bob
Turno de Chloé
No hay nadie esperando
```

> **Pista:** guarda los elementos en un array privado `#elementos: T[] = []`. `push`
> añade al final; `shift` quita el primero y devuelve `undefined` si el array está
> vacío (mira [los genéricos](generiques.md)).

> **Solución**
>
> ```typescript
> class Cola<T> {
>   #elementos: T[] = [];
>
>   entrar(elemento: T): void {
>     this.#elementos.push(elemento);
>   }
>
>   salir(): T | undefined {
>     return this.#elementos.shift();
>   }
>
>   cantidad(): number {
>     return this.#elementos.length;
>   }
> }
>
> const ventanilla = new Cola<string>();
> ventanilla.entrar("Alice");
> ventanilla.entrar("Bob");
> ventanilla.entrar("Chloé");
> console.log(`${ventanilla.cantidad()} personas esperando`);
>
> for (let i = 0; i < 4; i++) {
>   const siguiente = ventanilla.salir();
>   console.log(siguiente === undefined ? "No hay nadie esperando" : `Turno de ${siguiente}`);
> }
> ```
>
> ```sortie
> 3 personas esperando
> Turno de Alice
> Turno de Bob
> Turno de Chloé
> No hay nadie esperando
> ```
>
> Por qué funciona: `T` es una casilla vacía que se rellena al crear la cola
> (`Cola<string>`). Después, `tsc` rechazaría `ventanilla.entrar(42)`.
>
> | Paso | Acción | cola (delante → detrás) | Muestra |
> |---|---|---|---|
> | 1 | entrar Alice, Bob, Chloé | Alice, Bob, Chloé | `3 personas esperando` |
> | 2 | salir | Bob, Chloé | `Turno de Alice` |
> | 3 | salir | Chloé | `Turno de Bob` |
> | 4 | salir | (vacía) | `Turno de Chloé` |
> | 5 | salir → `undefined` | (vacía) | `No hay nadie esperando` |

### Ejercicio 12: Leer órdenes de stock

Nivel: ★★★

Una tienda recibe líneas de texto: `"añadir 10"`, `"quitar 3"`… Escribe:

- un tipo `Orden`: `{ tipo: "añadir"; cantidad: number }` o
  `{ tipo: "quitar"; cantidad: number }`;
- una función `leer(linea: string): Orden | string` que devuelva la orden, o un
  mensaje de error (un `string`) si la línea es incorrecta;
- un bucle que parta de un stock de 0, aplique las órdenes válidas y señale las
  demás. Una retirada mayor que el stock se rechaza.

Las líneas: `"añadir 10"`, `"quitar 3"`, `"vender 2"`, `"quitar 20"`,
`"añadir cinco"`, `"añadir 5"`.

Salida esperada:

```text
añadir 10 -> stock 10
quitar 3 -> stock 7
vender 2 -> error: acción desconocida «vender»
quitar 20 -> rechazado: solo hay 7 en stock
añadir cinco -> error: cantidad no válida «cinco»
añadir 5 -> stock 12
```

> **Pista:** `linea.split(" ")` da `["añadir", "10"]`. `Number("cinco")` da `NaN`:
> compruébalo con `Number.isNaN`.

> **Pista:** después de la llamada, `typeof resultado === "string"` separa los errores
> de las órdenes. Luego, `resultado.tipo` dice qué orden aplicar (mira
> [las uniones](unions.md)).

> **Solución**
>
> ```typescript
> type Orden =
>   | { tipo: "añadir"; cantidad: number }
>   | { tipo: "quitar"; cantidad: number };
>
> function leer(linea: string): Orden | string {
>   const [accion, texto] = linea.split(" ");
>   const cantidad = Number(texto);
>   if (Number.isNaN(cantidad) || cantidad <= 0) {
>     return `cantidad no válida «${texto}»`;
>   }
>   if (accion === "añadir" || accion === "quitar") {
>     return { tipo: accion, cantidad };
>   }
>   return `acción desconocida «${accion}»`;
> }
>
> const lineas = ["añadir 10", "quitar 3", "vender 2", "quitar 20", "añadir cinco", "añadir 5"];
> let stock = 0;
>
> for (const linea of lineas) {
>   const resultado = leer(linea);
>   if (typeof resultado === "string") {
>     console.log(`${linea} -> error: ${resultado}`);
>   } else if (resultado.tipo === "añadir") {
>     stock += resultado.cantidad;
>     console.log(`${linea} -> stock ${stock}`);
>   } else if (resultado.cantidad > stock) {
>     console.log(`${linea} -> rechazado: solo hay ${stock} en stock`);
>   } else {
>     stock -= resultado.cantidad;
>     console.log(`${linea} -> stock ${stock}`);
>   }
> }
> ```
>
> ```sortie
> añadir 10 -> stock 10
> quitar 3 -> stock 7
> vender 2 -> error: acción desconocida «vender»
> quitar 20 -> rechazado: solo hay 7 en stock
> añadir cinco -> error: cantidad no válida «cinco»
> añadir 5 -> stock 12
> ```
>
> Por qué funciona: `leer` es una ventanilla que clasifica el correo. Devuelve o bien
> una orden limpia, o bien una carta de error. Después de
> `typeof resultado === "string"`, TypeScript sabe que `resultado` es una `Orden` en
> las otras ramas: permite `resultado.tipo` y `resultado.cantidad`.
>
> | Línea | leer devuelve | stock después |
> |---|---|---|
> | añadir 10 | `{ tipo: "añadir", cantidad: 10 }` | 10 |
> | quitar 3 | `{ tipo: "quitar", cantidad: 3 }` | 7 |
> | vender 2 | `"acción desconocida «vender»"` | 7 |
> | quitar 20 | `{ tipo: "quitar", cantidad: 20 }`, pero 20 > 7 | 7 |
> | añadir cinco | `"cantidad no válida «cinco»"` | 7 |
> | añadir 5 | `{ tipo: "añadir", cantidad: 5 }` | 12 |

## En resumen

- Los ejercicios 1 a 4 repasan lo básico: funciones, condiciones, bucles y cálculos.
- Los ejercicios 5 a 9 trabajan el texto, los objetos `Record`, las uniones de textos
  y los retornos `| undefined`.
- Los ejercicios 10 a 12 usan bucles anidados, una clase genérica y una unión
  discriminada.
- Cuando una solución te sorprenda, rehaz su tabla de seguimiento en papel.

Siguiente: [chuleta](aide-memoire.md)
