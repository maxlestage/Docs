# Ejercicios para empezar con buen pie

Esta página propone 15 ejercicios muy guiados, todos fáciles, para practicar las bases de JavaScript: variables, números, condiciones, bucles, funciones, texto, arrays y objetos.

## Cómo hacer estos ejercicios

Hazlos en orden. Cada uno solo pide **una** idea nueva.

Para cada ejercicio:

1. **Lee** el enunciado y la salida esperada.
2. **Pruébalo en papel**: escribe en español los pasos («primero creo…, luego
   muestro…»).
3. **Escribe** el código en un archivo `ejercicio.mjs`.
4. **Ejecútalo** con `node ejercicio.mjs`.
5. **Compara** lo que se muestra con la salida esperada, carácter a carácter.
6. ¿Atascado? Lee **la primera pista** y vuelve a intentarlo. Luego la segunda. La
   solución, al final.

```text
 leer → papel → escribir → ejecutar → comparar
                   ▲                     │
                   └──── ¿distinto? ─────┘
```

> **Consejo:** un error no es un fracaso: es una pista más. Lee el mensaje de arriba
> abajo (mira [leer un error](../debuter/lire-une-erreur.md)).

## Variables y números

### Ejercicio 1: Una tarjeta de visita

Nivel: ★☆☆

Crea tres constantes: `nombre`, `ciudad` y `oficio`. Muéstralas en tres líneas, con una
etiqueta delante de cada una.

Salida esperada:

```text
Nombre: Alice
Ciudad: Lyon
Oficio: panadera
```

> **Pista:** una constante se crea con `const nombre = "Alice";`.

> **Pista:** para meter una variable dentro de un texto, usa una plantilla con acentos
> graves: `` `Nombre: ${nombre}` `` (mira [primeros pasos](premiers-pas.md)).

> **Solución**
>
> ```javascript
> const nombre = "Alice";
> const ciudad = "Lyon";
> const oficio = "panadera";
>
> console.log(`Nombre: ${nombre}`);
> console.log(`Ciudad: ${ciudad}`);
> console.log(`Oficio: ${oficio}`);
> ```
>
> ```sortie
> Nombre: Alice
> Ciudad: Lyon
> Oficio: panadera
> ```
>
> Por qué funciona: en una plantilla, `${nombre}` se sustituye por el valor guardado en
> la caja `nombre`.

### Ejercicio 2: El área de un rectángulo

Nivel: ★☆☆

Un rectángulo mide 7 m de largo y 4 m de ancho. Calcula su área (largo × ancho) y
muéstrala.

Salida esperada:

```text
Área: 28 m²
```

> **Pista:** guarda el largo y el ancho en dos constantes.

> **Pista:** la multiplicación se escribe `*`: `const area = largo * ancho;`.

> **Solución**
>
> ```javascript
> const largo = 7;
> const ancho = 4;
> const area = largo * ancho;
>
> console.log(`Área: ${area} m²`);
> ```
>
> ```sortie
> Área: 28 m²
> ```
>
> Por qué funciona: `7 * 4` se calcula una vez, se guarda en `area` y luego se muestra.

### Ejercicio 3: El precio con IVA

Nivel: ★☆☆

Un libro cuesta 12,50 € sin impuestos. El IVA añade un 5,5 %. Muestra el precio con
impuestos incluidos, con **dos** cifras decimales.

Salida esperada:

```text
Precio con IVA: 13.19 €
```

> **Pista:** añadir un 5,5 % es multiplicar por `1.055`. En JavaScript, los decimales se
> escriben con un **punto**.

> **Pista:** `numero.toFixed(2)` redondea y conserva dos cifras decimales
> (mira [los números](nombres.md#mostrar-con-tofixed)).

> **Solución**
>
> ```javascript
> const precioSinIva = 12.5;
> const precioConIva = precioSinIva * 1.055;
>
> console.log(`Precio con IVA: ${precioConIva.toFixed(2)} €`);
> ```
>
> ```sortie
> Precio con IVA: 13.19 €
> ```
>
> Por qué funciona: `12.5 * 1.055` vale `13.1875`. `toFixed(2)` lo redondea a
> `"13.19"`.

### Ejercicio 4: De minutos a horas

Nivel: ★☆☆

Una película dura 135 minutos. Muestra su duración en horas y minutos.

Salida esperada:

```text
2 h 15 min
```

> **Pista:** una hora son 60 minutos. ¿Cuántas veces «cabe» 60 en 135? Es
> `Math.floor(135 / 60)`.

> **Pista:** lo que queda después de las horas es el **resto** de la división:
> `135 % 60`.

> **Solución**
>
> ```javascript
> const duracion = 135;
> const horas = Math.floor(duracion / 60);
> const minutos = duracion % 60;
>
> console.log(`${horas} h ${minutos} min`);
> ```
>
> ```sortie
> 2 h 15 min
> ```
>
> Por qué funciona: `135 / 60` vale `2.25`; `Math.floor` se queda con `2`. Y
> `135 - 2 × 60` deja `15`, que es lo que da `135 % 60`.

## Condiciones y bucles

### Ejercicio 5: Mayor o menor de edad

Nivel: ★☆☆

Para cada edad, `15` y `18`, muestra si se trata de una persona mayor o menor de edad.

Salida esperada:

```text
15 años: menor de edad
18 años: mayor de edad
```

> **Pista:** empieza con una sola edad: `const edad = 15;` y un `if (edad >= 18)`.

> **Pista:** para tratar las dos edades, mete el `if` dentro de un bucle
> `for (const edad of [15, 18])` (mira [las condiciones](conditions-boucles.md)).

> **Solución**
>
> ```javascript
> for (const edad of [15, 18]) {
>   if (edad >= 18) {
>     console.log(`${edad} años: mayor de edad`);
>   } else {
>     console.log(`${edad} años: menor de edad`);
>   }
> }
> ```
>
> ```sortie
> 15 años: menor de edad
> 18 años: mayor de edad
> ```
>
> Por qué funciona: `>=` quiere decir «mayor o igual que». `18 >= 18` es verdadero, así
> que con 18 años se es mayor de edad.

### Ejercicio 6: El mayor de los dos

Nivel: ★☆☆

Con `a = 8` y `b = 13`, muestra el mayor de los dos números, con una sola línea de
cálculo.

Salida esperada:

```text
El mayor: 13
```

> **Pista:** el operador ternario se escribe `condicion ? siVerdadero : siFalso`.

> **Pista:** la condición es `a > b`.

> **Solución**
>
> ```javascript
> const a = 8;
> const b = 13;
> const mayor = a > b ? a : b;
>
> console.log(`El mayor: ${mayor}`);
> ```
>
> ```sortie
> El mayor: 13
> ```
>
> Por qué funciona: `8 > 13` es falso, así que se toma el valor que va después de los
> `:`, `b`. `Math.max(a, b)` también da el resultado correcto.

### Ejercicio 7: La cuenta atrás

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

> **Pista:** un bucle `for` puede **bajar**: se empieza en 5 y se resta 1 en cada vuelta
> con `i--`.

> **Pista:** el bucle continúa mientras `i >= 1`. El mensaje final va **después** de la
> llave de cierre del bucle.

> **Solución**
>
> ```javascript
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
> Por qué funciona: cuando `i` vale 0, `0 >= 1` es falso y el bucle se detiene. La
> línea siguiente se ejecuta una sola vez.

### Ejercicio 8: La tabla del 7

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

> **Pista:** dentro de la plantilla, el cálculo se puede hacer directamente:
> `${7 * i}`.

> **Solución**
>
> ```javascript
> for (let i = 1; i <= 5; i++) {
>   console.log(`7 x ${i} = ${7 * i}`);
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
> Por qué funciona: en cada vuelta, `i` cambia, y la misma línea muestra un cálculo
> nuevo.

### Ejercicio 9: La suma del 1 al 10

Nivel: ★☆☆

Suma todos los números del 1 al 10 con un bucle y luego muestra el total.

Salida esperada:

```text
Total: 55
```

> **Pista:** crea una caja `total` que valga `0` **antes** del bucle. Tiene que poder
> cambiar: usa `let`.

> **Pista:** en cada vuelta, añade `i` a la caja: `total = total + i;` (o
> `total += i;`). Estas son las primeras vueltas:
>
> | Paso | i | total |
> | --- | --- | --- |
> | inicio | – | 0 |
> | 1 | 1 | 1 |
> | 2 | 2 | 3 |
> | 3 | 3 | 6 |

> **Solución**
>
> ```javascript
> let total = 0;
> for (let i = 1; i <= 10; i++) {
>   total += i;
> }
> console.log(`Total: ${total}`);
> ```
>
> ```sortie
> Total: 55
> ```
>
> Por qué funciona: `total` se crea fuera del bucle, así que conserva su valor de una
> vuelta a otra. Si se creara dentro, volvería a empezar desde 0 en cada vuelta.

## Funciones

### Ejercicio 10: Saludar

Nivel: ★☆☆

Escribe una función `saludar(nombre)` que **devuelva** `¡Hola, <nombre>!`. Llámala con
`"Alice"` y luego con `"Bob"`, y muestra los resultados.

Salida esperada:

```text
¡Hola, Alice!
¡Hola, Bob!
```

> **Pista:** una función se escribe `function saludar(nombre) { ... }`.

> **Pista:** dentro, `return` devuelve el texto. Fuera,
> `console.log(saludar("Alice"))` lo muestra (mira [las funciones](fonctions.md)).

> **Solución**
>
> ```javascript
> function saludar(nombre) {
>   return `¡Hola, ${nombre}!`;
> }
>
> console.log(saludar("Alice"));
> console.log(saludar("Bob"));
> ```
>
> ```sortie
> ¡Hola, Alice!
> ¡Hola, Bob!
> ```
>
> Por qué funciona: el parámetro `nombre` recibe `"Alice"` en la primera llamada y
> luego `"Bob"` en la segunda. La misma receta sirve dos veces.

### Ejercicio 11: El cuadrado de un número

Nivel: ★☆☆

Escribe una función flecha `cuadrado(n)` que devuelva `n × n`. Muestra el cuadrado de 3
y luego la suma de los cuadrados de 3 y de 4.

Salida esperada:

```text
9
25
```

> **Pista:** una función flecha corta se escribe `const cuadrado = (n) => n * n;`.

> **Pista:** el resultado de una función es un valor: se puede sumar
> `cuadrado(3) + cuadrado(4)`.

> **Solución**
>
> ```javascript
> const cuadrado = (n) => n * n;
>
> console.log(cuadrado(3));
> console.log(cuadrado(3) + cuadrado(4));
> ```
>
> ```sortie
> 9
> 25
> ```
>
> Por qué funciona: sin llaves, una flecha devuelve automáticamente su cálculo.
> `cuadrado(3) + cuadrado(4)` se convierte en `9 + 16`.

## Texto, arrays y objetos

### Ejercicio 12: Gritar una palabra

Nivel: ★☆☆

Con la palabra `"hola"`, muéstrala en mayúsculas, seguida de su número de letras.

Salida esperada:

```text
HOLA (4 letras)
```

> **Pista:** `palabra.toUpperCase()` la pone en mayúsculas.

> **Pista:** `palabra.length` da el número de caracteres. Atención: **sin** paréntesis
> (mira [trabajar con texto](texte.md)).

> **Solución**
>
> ```javascript
> const palabra = "hola";
> console.log(`${palabra.toUpperCase()} (${palabra.length} letras)`);
> ```
>
> ```sortie
> HOLA (4 letras)
> ```
>
> Por qué funciona: `toUpperCase()` devuelve un texto **nuevo** en mayúsculas;
> `palabra` en sí no cambia.

### Ejercicio 13: Las iniciales

Nivel: ★☆☆

Con el nombre completo `"Ada Lovelace"`, muestra sus iniciales.

Salida esperada:

```text
A.L.
```

> **Pista:** `nombreCompleto.split(" ")` corta el texto en los espacios y da un array:
> `["Ada", "Lovelace"]`.

> **Pista:** la primera letra de una palabra `p` es `p[0]`.

> **Solución**
>
> ```javascript
> const nombreCompleto = "Ada Lovelace";
> const [nombre, apellido] = nombreCompleto.split(" ");
>
> console.log(`${nombre[0]}.${apellido[0]}.`);
> ```
>
> ```sortie
> A.L.
> ```
>
> Por qué funciona: `split` da dos palabras, guardadas en `nombre` y `apellido`. El
> índice `[0]` toma el primer carácter de cada una.

### Ejercicio 14: La media de las notas

Nivel: ★☆☆

Con el array `[12, 15, 9, 16]`, calcula la media de las notas y muéstrala.

Salida esperada:

```text
Media: 13
```

> **Pista:** primero la suma, como en el ejercicio 9, pero con
> `for (const nota of notas)`.

> **Pista:** la media es la suma dividida entre el número de notas: `notas.length`
> (mira [los arrays](collections.md)).

> **Solución**
>
> ```javascript
> const notas = [12, 15, 9, 16];
>
> let suma = 0;
> for (const nota of notas) {
>   suma += nota;
> }
>
> console.log(`Media: ${suma / notas.length}`);
> ```
>
> ```sortie
> Media: 13
> ```
>
> Por qué funciona: la suma vale `52`, hay `4` notas, y `52 / 4` da `13`.

### Ejercicio 15: La ficha de un libro

Nivel: ★☆☆

Crea un objeto `libro` con un `titulo` (`"El principito"`), un `autor`
(`"Saint-Exupéry"`) y un `anio` (`1943`). Muestra cada propiedad en una línea.

Salida esperada:

```text
titulo: El principito
autor: Saint-Exupéry
anio: 1943
```

> **Pista:** un objeto se escribe entre llaves: `{ titulo: "...", autor: "...", ... }`.

> **Pista:** `Object.entries(libro)` da la lista de los pares `[clave, valor]`.
> Recórrela con `for (const [clave, valor] of ...)` (mira
> [los objetos](objets.md#recorrer-un-objeto)).

> **Solución**
>
> ```javascript
> const libro = {
>   titulo: "El principito",
>   autor: "Saint-Exupéry",
>   anio: 1943,
> };
>
> for (const [clave, valor] of Object.entries(libro)) {
>   console.log(`${clave}: ${valor}`);
> }
> ```
>
> ```sortie
> titulo: El principito
> autor: Saint-Exupéry
> anio: 1943
> ```
>
> Por qué funciona: `Object.entries` convierte el objeto en un array de pares, en el
> orden en que se escribieron las propiedades. Cada par se descompone en `clave` y
> `valor`.

## ¿Y después?

¡Bravo! Si has hecho estos 15 ejercicios, dominas las bases. Otros ejercicios, más
variados, te esperan en [los ejercicios resueltos](exercices.md).

Siguiente: [las clases](types.md).
