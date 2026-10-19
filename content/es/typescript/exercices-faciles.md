# Ejercicios para empezar bien

Esta página propone 15 pequeños ejercicios muy guiados, todos fáciles, para practicar las bases de TypeScript: variables, números, condiciones, bucles, funciones, texto, arrays y objetos.

## Cómo hacer estos ejercicios

Cada ejercicio se hace en cinco pasos:

1. **Lee** el enunciado y la salida esperada.
2. **Prueba en papel**: escribe los pasos en español, como una receta.
3. **Escribe** el código en un archivo `ejercicio.ts` y luego **ejecútalo**
   (`npx tsx ejercicio.ts`, mira [la introducción](introduction.md)).
4. **Compara** lo que se muestra con la salida esperada, carácter por carácter.
5. ¿Atascado? Lee la **primera pista** y luego la segunda. Abre la solución al final.

> **Consejo:** un error de `tsc` no es un fracaso. Es un consejo. Léelo línea por
> línea, como en [los primeros pasos](premiers-pas.md#leer-un-error-de-tipo-linea-por-linea).

Todos los ejercicios son de nivel ★☆☆. Las soluciones pasan `tsc --strict`.

## Variables y números

### Ejercicio 1: Una tarjeta de visita

Nivel: ★☆☆

Crea tres constantes: `nombre`, `ciudad` y `edad`. Muestra una frase con las
tres.

Salida esperada:

```text
Me llamo Alice, vivo en Lyon y tengo 30 años.
```

> **Pista:** crea una variable con `const nombre = "Alice";`.

> **Pista:** para insertar una variable en un texto, usa una plantilla entre
> acentos graves: `` `Hola ${nombre}` ``.

> **Solución**
>
> ```typescript
> const nombre = "Alice";
> const ciudad = "Lyon";
> const edad = 30;
> console.log(`Me llamo ${nombre}, vivo en ${ciudad} y tengo ${edad} años.`);
> ```
>
> ```sortie
> Me llamo Alice, vivo en Lyon y tengo 30 años.
> ```
>
> Por qué funciona: cada `${...}` se sustituye por el valor de la variable.
> TypeScript adivina solo que `edad` es un `number`.

### Ejercicio 2: El área de un rectángulo

Nivel: ★☆☆

Un rectángulo mide 8 metros de largo y 5 metros de ancho. Calcula su área
(largo × ancho) y su perímetro (2 × (largo + ancho)).

Salida esperada:

```text
Área: 40 m²
Perímetro: 26 m
```

> **Pista:** la multiplicación se escribe `*`.

> **Pista:** para el perímetro, pon la suma entre paréntesis:
> `2 * (largo + ancho)`.

> **Solución**
>
> ```typescript
> const largo = 8;
> const ancho = 5;
> const area = largo * ancho;
> const perimetro = 2 * (largo + ancho);
> console.log(`Área: ${area} m²`);
> console.log(`Perímetro: ${perimetro} m`);
> ```
>
> ```sortie
> Área: 40 m²
> Perímetro: 26 m
> ```
>
> Por qué funciona: los paréntesis hacen que se calcule primero `8 + 5` (13), y luego
> `2 * 13`. Sin ellos, tendrías `2 * 8 + 5`, es decir, 21.

### Ejercicio 3: De minutos a horas

Nivel: ★☆☆

Una película dura 137 minutos. Muestra su duración en horas y minutos.

Salida esperada:

```text
137 minutos = 2 h 17 min
```

> **Pista:** el número de horas es `137 / 60` redondeado hacia abajo:
> `Math.floor(...)`.

> **Pista:** los minutos que sobran son el resto de la división: `137 % 60`.

> **Solución**
>
> ```typescript
> const duracion = 137;
> const horas = Math.floor(duracion / 60);
> const minutos = duracion % 60;
> console.log(`${duracion} minutos = ${horas} h ${minutos} min`);
> ```
>
> ```sortie
> 137 minutos = 2 h 17 min
> ```
>
> Por qué funciona: `137 / 60` vale `2.28…`, redondeado a `2`. Y `2 × 60 = 120`, así
> que quedan `137 - 120 = 17` minutos. Mira [números y cálculos](nombres.md).

### Ejercicio 4: El precio con IVA

Nivel: ★☆☆

Un libro cuesta 12.5 € sin impuestos. El IVA es del 5.5 %. Muestra el precio con IVA
con dos cifras después del punto decimal.

Salida esperada:

```text
Precio con IVA: 13.19 €
```

> **Pista:** 5.5 % se escribe `0.055` en un cálculo. El precio con IVA es
> `precioSinIva * (1 + 0.055)`.

> **Pista:** `numero.toFixed(2)` da un texto con dos decimales.

> **Solución**
>
> ```typescript
> const precioSinIva = 12.5;
> const iva = 0.055;
> const precioConIva = precioSinIva * (1 + iva);
> console.log(`Precio con IVA: ${precioConIva.toFixed(2)} €`);
> ```
>
> ```sortie
> Precio con IVA: 13.19 €
> ```
>
> Por qué funciona: el cálculo da `13.1875`. `toFixed(2)` lo redondea a `13.19`
> para mostrarlo.

## Condiciones y bucles

### Ejercicio 5: Mayor o menor de edad

Nivel: ★☆☆

Para cada edad de la lista `[12, 18, 45]`, muestra si se trata de una persona mayor de
edad (18 años o más) o menor.

Salida esperada:

```text
12 años: menor
18 años: mayor
45 años: mayor
```

> **Pista:** recorre la lista con `for (const edad of edades) { ... }`.

> **Pista:** dentro del bucle, prueba `if (edad >= 18)`. Atención: `>=` y no `>`,
> porque con 18 años ya eres mayor de edad.

> **Solución**
>
> ```typescript
> const edades = [12, 18, 45];
> for (const edad of edades) {
>   if (edad >= 18) {
>     console.log(`${edad} años: mayor`);
>   } else {
>     console.log(`${edad} años: menor`);
>   }
> }
> ```
>
> ```sortie
> 12 años: menor
> 18 años: mayor
> 45 años: mayor
> ```
>
> Por qué funciona: el bucle toma cada edad por turno. El `if` elige una de las
> dos frases.

### Ejercicio 6: La cuenta atrás

Nivel: ★☆☆

Muestra una cuenta atrás de 5 a 1 y luego `¡Ya!`.

Salida esperada:

```text
5
4
3
2
1
¡Ya!
```

> **Pista:** un bucle `for` puede ir hacia abajo: `for (let i = 5; i >= 1; i--)`.

> **Solución**
>
> ```typescript
> for (let i = 5; i >= 1; i--) {
>   console.log(i);
> }
> console.log("¡Ya!");
> ```
>
> ```sortie
> 5
> 4
> 3
> 2
> 1
> ¡Ya!
> ```
>
> Por qué funciona: `i` empieza en 5. Después de cada vuelta, `i--` le resta 1. El
> bucle se detiene cuando `i >= 1` se vuelve falso, es decir, cuando `i` vale 0. El
> último `console.log` está **después** del bucle: solo se muestra una vez.

### Ejercicio 7: La tabla del 7

Nivel: ★☆☆

Muestra la tabla de multiplicar del 7, del 1 al 5.

Salida esperada:

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
```

> **Pista:** un bucle `for (let i = 1; i <= 5; i++)` da `i` = 1, 2, 3, 4, 5.

> **Pista:** dentro del bucle, muestra `` `7 x ${i} = ${7 * i}` ``.

> **Solución**
>
> ```typescript
> const tabla = 7;
> for (let i = 1; i <= 5; i++) {
>   console.log(`${tabla} x ${i} = ${tabla * i}`);
> }
> ```
>
> ```sortie
> 7 x 1 = 7
> 7 x 2 = 14
> 7 x 3 = 21
> 7 x 4 = 28
> 7 x 5 = 35
> ```
>
> Por qué funciona: `<=` incluye el 5. Con `<`, el bucle se detendría en 4. Cambia
> `tabla` para obtener otra tabla.

### Ejercicio 8: Sumar una lista

Nivel: ★☆☆

Calcula la suma de los gastos `[12, 5, 30, 8]` con un bucle.

Salida esperada:

```text
Total: 55 €
```

> **Pista:** crea una variable `let total = 0;` antes del bucle.

> **Pista:** dentro del bucle `for...of`, suma cada gasto: `total += gasto;`.

> **Solución**
>
> ```typescript
> const gastos = [12, 5, 30, 8];
> let total = 0;
> for (const gasto of gastos) {
>   total += gasto;
> }
> console.log(`Total: ${total} €`);
> ```
>
> ```sortie
> Total: 55 €
> ```
>
> Por qué funciona: `total` es una hucha que crece en cada vuelta.
>
> | Vuelta | gasto | total |
> |---|---|---|
> | inicio | | 0 |
> | 1 | 12 | 12 |
> | 2 | 5 | 17 |
> | 3 | 30 | 47 |
> | 4 | 8 | 55 |

## Funciones

### Ejercicio 9: El cuadrado de un número

Nivel: ★☆☆

Escribe una función `cuadrado` que recibe un número y devuelve ese número multiplicado
por sí mismo. Muestra el cuadrado de 3 y de 12.

Salida esperada:

```text
9
144
```

> **Pista:** la forma de una función tipada:
> `function cuadrado(n: number): number { ... }`.

> **Pista:** dentro de la función, escribe `return n * n;`.

> **Solución**
>
> ```typescript
> function cuadrado(n: number): number {
>   return n * n;
> }
> console.log(cuadrado(3));
> console.log(cuadrado(12));
> ```
>
> ```sortie
> 9
> 144
> ```
>
> Por qué funciona: `cuadrado(3)` pone `3` en `n`, calcula `3 * 3` y devuelve `9`.
> `console.log` muestra lo que se devuelve.

### Ejercicio 10: Saludar con un valor por defecto

Nivel: ★☆☆

Escribe una función `saludar` que recibe un nombre y devuelve `¡Hola, <nombre>!`.
Sin nombre, debe decir `¡Hola, desconocido!`.

Salida esperada:

```text
¡Hola, Léa!
¡Hola, desconocido!
```

> **Pista:** un valor por defecto se escribe con `=` en los parámetros:
> `function saludar(nombre = "desconocido")`.

> **Solución**
>
> ```typescript
> function saludar(nombre: string = "desconocido"): string {
>   return `¡Hola, ${nombre}!`;
> }
> console.log(saludar("Léa"));
> console.log(saludar());
> ```
>
> ```sortie
> ¡Hola, Léa!
> ¡Hola, desconocido!
> ```
>
> Por qué funciona: cuando llamas a `saludar()` sin argumento, `nombre` toma el
> valor por defecto. Mira [las funciones](fonctions.md).

### Ejercicio 11: Par o impar

Nivel: ★☆☆

Escribe una función `esPar` que devuelve `true` si un número es par, y `false` si
no. Pruébala con 4 y 7.

Salida esperada:

```text
4 es par: true
7 es par: false
```

> **Pista:** un número es par si el resto de su división entre 2 vale 0: `n % 2`.

> **Pista:** la comparación `n % 2 === 0` ya da `true` o `false`. Puedes
> devolverla directamente.

> **Solución**
>
> ```typescript
> function esPar(n: number): boolean {
>   return n % 2 === 0;
> }
> console.log(`4 es par: ${esPar(4)}`);
> console.log(`7 es par: ${esPar(7)}`);
> ```
>
> ```sortie
> 4 es par: true
> 7 es par: false
> ```
>
> Por qué funciona: `4 % 2` vale `0`, así que `0 === 0` es verdadero. `7 % 2` vale
> `1`, así que la comparación es falsa.

## Texto

### Ejercicio 12: Las iniciales

Nivel: ★☆☆

A partir de `nombre = "ada"` y `apellido = "lovelace"`, muestra las iniciales en
mayúsculas, seguidas de puntos.

Salida esperada:

```text
A.L.
```

> **Pista:** el primer carácter de un texto se obtiene con `texto[0]`, o mejor
> `texto.charAt(0)`, que siempre da un `string`.

> **Pista:** `.toUpperCase()` pone un texto en mayúsculas.

> **Solución**
>
> ```typescript
> const nombre = "ada";
> const apellido = "lovelace";
> const iniciales = `${nombre.charAt(0)}.${apellido.charAt(0)}.`.toUpperCase();
> console.log(iniciales);
> ```
>
> ```sortie
> A.L.
> ```
>
> Por qué funciona: primero se construye `"a.l."` y luego `toUpperCase` lo transforma
> en `"A.L."`. Los puntos no cambian.

### Ejercicio 13: Contar las palabras

Nivel: ★☆☆

Cuenta las palabras de la frase `"el gato está en el sillón"` y muestra la más larga.

Salida esperada:

```text
6 palabras
La más larga: sillón
```

> **Pista:** `frase.split(" ")` corta la frase en cada espacio y da un array de
> palabras. Su longitud es `.length`.

> **Pista:** para la más larga, guarda una variable `let masLarga = "";` y
> sustitúyela en un bucle cuando una palabra sea más larga que ella.

> **Solución**
>
> ```typescript
> const frase = "el gato está en el sillón";
> const palabras = frase.split(" ");
> console.log(`${palabras.length} palabras`);
>
> let masLarga = "";
> for (const palabra of palabras) {
>   if (palabra.length > masLarga.length) {
>     masLarga = palabra;
>   }
> }
> console.log(`La más larga: ${masLarga}`);
> ```
>
> ```sortie
> 6 palabras
> La más larga: sillón
> ```
>
> Por qué funciona: `masLarga` guarda a la campeona del momento. `"el"` sustituye a
> `""`, luego `"gato"` sustituye a `"el"`, y después solo `"sillón"` (6 letras) lo
> hace mejor.

## Arrays y objetos

### Ejercicio 14: Quedarse con las buenas notas

Nivel: ★☆☆

A partir de las notas `[8, 15, 11, 6, 19]`, quédate solo con las que valen al menos
10 y muéstralas con su cantidad.

Salida esperada:

```text
[ 15, 11, 19 ]
3 notas por encima de la media
```

> **Pista:** `filter` guarda los elementos para los que una función responde `true`:
> `notas.filter((n) => ...)`.

> **Pista:** la condición es `n >= 10`.

> **Solución**
>
> ```typescript
> const notas = [8, 15, 11, 6, 19];
> const buenas = notas.filter((n) => n >= 10);
> console.log(buenas);
> console.log(`${buenas.length} notas por encima de la media`);
> ```
>
> ```sortie
> [ 15, 11, 19 ]
> 3 notas por encima de la media
> ```
>
> Por qué funciona: `filter` le hace la pregunta `n >= 10` a cada nota y guarda
> las que responden que sí. El array `notas` no se modifica. Mira
> [los arrays](collections.md).

### Ejercicio 15: La ficha de un libro

Nivel: ★☆☆

Escribe una interfaz `Libro` con un `titulo` (texto) y un número de `paginas`. Crea
dos libros en un array y luego muestra una línea por libro.

Salida esperada:

```text
Dune: 600 páginas
El principito: 96 páginas
```

> **Pista:** una interfaz se escribe
> `interface Libro { titulo: string; paginas: number; }`.

> **Pista:** el array tiene el tipo `Libro[]`. Recórrelo con `for...of`.

> **Solución**
>
> ```typescript
> interface Libro {
>   titulo: string;
>   paginas: number;
> }
>
> const libros: Libro[] = [
>   { titulo: "Dune", paginas: 600 },
>   { titulo: "El principito", paginas: 96 },
> ];
>
> for (const libro of libros) {
>   console.log(`${libro.titulo}: ${libro.paginas} páginas`);
> }
> ```
>
> ```sortie
> Dune: 600 páginas
> El principito: 96 páginas
> ```
>
> Por qué funciona: la interfaz es el modelo de la ficha. TypeScript comprueba que
> cada libro tenga de verdad un `titulo` y unas `paginas`. Mira [los objetos](objets.md).

¡Bravo! Para ir más lejos, prueba los [ejercicios resueltos](exercices.md) de nivel
★★☆.

Siguiente: [las uniones](unions.md).
