# Trabajar con texto

Esta página muestra cómo manejar texto en JavaScript: leer sus caracteres, cortarlo, transformarlo, buscar en él y ordenarlo según el orden alfabético español.

> **Objetivos:**
>
> - sabrás leer la longitud de un texto y el carácter que está en una posición dada;
> - sabrás extraer un trozo con `slice` y limpiar un texto con `trim`;
> - sabrás cortar con `split` y volver a unir con `join`;
> - sabrás buscar y reemplazar en un texto;
> - sabrás ordenar palabras en español correctamente con `localeCompare`.

## Las cadenas de caracteres

Casi todos los programas manejan texto: un nombre, un mensaje, un correo electrónico,
una línea leída de un archivo. Antes de transformarlo, hay que saber escribirlo.

Un texto se llama [cadena de caracteres](../debuter/glossaire.md#cadena-de-caracteres)
(*string*): una sucesión de letras, cifras, espacios y símbolos, como las perlas de un
collar. Se escribe entre comillas dobles, simples o comillas invertidas.

```javascript
const a = "Hola";
const b = 'Buenas';
const c = `Qué tal`;
const cita = "Dijo: 'sí'";

console.log(a, b, c);
console.log(cita);
console.log("L'Hospitalet");
```

```sortie
Hola Buenas Qué tal
Dijo: 'sí'
L'Hospitalet
```

Línea por línea:

- las tres primeras líneas guardan un texto en `a`, `b` y `c`, con tres tipos de
  comillas;
- `cita` contiene comillas simples: por eso se rodea con comillas dobles;
- `console.log(a, b, c)` muestra los tres textos en una línea, separados por un espacio;
- las comillas que rodean un texto nunca se muestran: solo marcan dónde empieza y dónde
  termina.

Elige las comillas que no aparecen en el texto: `"L'Hospitalet"` contiene un apóstrofo,
así que se rodea con comillas dobles.

### Los caracteres especiales

Una barra invertida `\` introduce un carácter especial: `\n` para un salto de línea,
`\t` para una tabulación, `\"` para unas comillas dentro de un texto entre comillas.

```javascript
console.log("Línea 1\nLínea 2");
console.log("Ella dice \"hola\".");
```

```sortie
Línea 1
Línea 2
Ella dice "hola".
```

## Longitud y posición

Comprobar que una contraseña es bastante larga, tomar la primera letra de un nombre:
para eso, necesitas conocer el tamaño de un texto y saber señalar una letra concreta,
como se señala una casa por su número en una calle.

`length` da el número de caracteres. Cada carácter tiene una posición, su
[índice](../debuter/glossaire.md#indice), que **empieza en 0**.

```javascript
const palabra = "Ventana";

console.log(palabra.length);
console.log(palabra[0]);
console.log(palabra[3]);
console.log(palabra[palabra.length - 1]);
console.log(palabra[10]);
```

```sortie
7
V
t
a
undefined
```

Línea por línea:

- `palabra.length` vale 7: «Ventana» tiene siete letras;
- `palabra[0]` es la primera letra, `"V"`, porque se cuenta desde 0;
- `palabra[3]` es por tanto la cuarta letra, `"t"`;
- `palabra.length - 1` vale 6: `palabra[6]` es la última letra, `"a"`;
- `palabra[10]` señala una casilla que no existe: el resultado es `undefined`.

| Carácter | V | e | n | t | a | n | a |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Índice | 0 | 1 | 2 | 3 | 4 | 5 | 6 |

El último carácter está en el índice `length - 1`. Una posición que no existe da
`undefined`.

### Contar desde el final con at

`at(i)` funciona como `[i]`, pero acepta un número negativo: `-1` es el último
carácter, `-2` el penúltimo.

```javascript
const palabra = "Ventana";
console.log(palabra.at(0));
console.log(palabra.at(-1));
console.log(palabra.at(-2));
```

```sortie
V
a
n
```

**Te toca:** escribe un programa que guarde `"Lucía"` en una variable `nombre` y
luego muestre su longitud, su primera letra y su última letra.

> **Solución**
>
> ```javascript
> const nombre = "Lucía";
> console.log(nombre.length);
> console.log(nombre[0]);
> console.log(nombre.at(-1));
> ```
>
> ```sortie
> 5
> L
> a
> ```

## Los textos nunca cambian

Conviene conocer esta regla pronto, porque causa muchos errores de principiante: crees
que has cambiado un texto, cuando en realidad has fabricado uno nuevo y lo has
olvidado.

Una cadena es [inmutable](../debuter/glossaire.md#inmutable): no se puede modificar uno
de sus caracteres. Todos los métodos de esta página **devuelven un texto nuevo** y dejan
el original intacto.

```javascript
const nombre = "alicia";
const mayusculas = nombre.toUpperCase();

console.log(nombre);
console.log(mayusculas);
```

```sortie
alicia
ALICIA
```

Para conservar el resultado, guárdalo en una variable (con `let`, puedes reutilizar la
misma: `texto = texto.trim()`).

## Mayúsculas y espacios

Lo que escribe un usuario rara vez está limpio: espacios de más, mayúsculas al azar.
Estos métodos ordenan el texto antes de usarlo, como se ordena un cajón antes de buscar
algo en él.

```javascript
const entrada = "   Marie Curie   ";

console.log(`[${entrada.toUpperCase()}]`);
console.log(`[${entrada.toLowerCase()}]`);
console.log(`[${entrada.trim()}]`);
console.log(`[${entrada.trimStart()}]`);
console.log(`[${entrada.trimEnd()}]`);
```

```sortie
[   MARIE CURIE   ]
[   marie curie   ]
[Marie Curie]
[Marie Curie   ]
[   Marie Curie]
```

Línea por línea:

- `entrada` contiene «Marie Curie» con tres espacios a cada lado;
- los corchetes `[` y `]` solo sirven para ver dónde empiezan y terminan los espacios;
- cada línea llama a un método sobre `entrada` y muestra el texto nuevo que obtiene;
- `entrada` nunca cambia: cada línea parte del texto original, con sus espacios.

- `toUpperCase()` y `toLowerCase()` cambian entre mayúsculas y minúsculas, incluidas
  las tildes y la ñ (`"año"` se convierte en `"AÑO"`).
- `trim()` quita los espacios del principio y del final; `trimStart()` y `trimEnd()`
  solo de un lado.

> **Consejo:** llama a `trim()` sobre todo lo que escribe un usuario: un espacio
> olvidado al final de una contraseña o de un correo electrónico es una fuente de
> errores clásica.

## Extraer un trozo

A menudo solo te interesa una parte de un texto: el año de una fecha, las primeras
letras de una palabra. `slice` recorta ese trozo, como unas tijeras.

`slice(inicio, fin)` devuelve el trozo que va de `inicio` (incluido) a `fin` (no
incluido). Sin `fin`, llega hasta el final. Los números negativos cuentan desde el final.

```javascript
const palabra = "Ventana";

console.log(palabra.slice(0, 3));
console.log(palabra.slice(3));
console.log(palabra.slice(-4));
console.log(palabra.slice(1, -1));
```

```sortie
Ven
tana
tana
entan
```

Línea por línea:

- `slice(0, 3)` toma los índices 0, 1 y 2: `"Ven"`;
- `slice(3)` empieza en el índice 3 y llega hasta el final: `"tana"`;
- `slice(-4)` toma los cuatro últimos caracteres: otra vez `"tana"`;
- `slice(1, -1)` quita la primera y la última letra: `"entan"`.

Piensa en los índices como **cortes entre las letras**: `slice(0, 3)` corta antes de la
letra 0 y antes de la letra 3.

`substring(inicio, fin)` hace casi lo mismo, pero no entiende los números negativos.
Quédate con `slice`, que también funciona con los arrays.

**Te toca:** escribe un programa que guarde `"2026-10-19"` en una variable `fecha` y
luego muestre el año (los cuatro primeros caracteres) y el día (los dos últimos).

> **Solución**
>
> ```javascript
> const fecha = "2026-10-19";
> console.log(fecha.slice(0, 4));
> console.log(fecha.slice(-2));
> ```
>
> ```sortie
> 2026
> 19
> ```

## Cortar y unir

Un texto suele contener una lista escondida: frutas separadas por comas, palabras
separadas por espacios. `split` saca esa lista del texto, para tratar los trozos uno a
uno.

`split(separador)` corta un texto en un array de trozos. `join(separador)` hace lo
contrario: vuelve a unir los trozos de un array.

```javascript
const linea = "manzana,pera,kiwi";
const frutas = linea.split(",");
console.log(frutas);
console.log(frutas.length);

console.log(frutas.join(" - "));
console.log("Hola a todos".split(" "));
console.log("abc".split(""));
```

```sortie
[ 'manzana', 'pera', 'kiwi' ]
3
manzana - pera - kiwi
[ 'Hola', 'a', 'todos' ]
[ 'a', 'b', 'c' ]
```

Línea por línea:

- `linea.split(",")` corta el texto en cada coma: `frutas` es un array de tres textos;
- `frutas.length` vale 3: el array tiene tres casillas;
- `frutas.join(" - ")` vuelve a unir los tres trozos poniendo `" - "` entre cada uno;
- `split(" ")` corta en cada espacio: se obtienen las palabras de la frase.

`split("")` corta en caracteres. Los arrays se explican en
[la página siguiente](collections.md).

**Te toca:** escribe un programa que cuente las palabras de la frase
`"El gato duerme en el sofá"` y muestre ese número.

> **Solución**
>
> ```javascript
> const frase = "El gato duerme en el sofá";
> const palabras = frase.split(" ");
> console.log(palabras.length);
> ```
>
> ```sortie
> 6
> ```

## Buscar en un texto

Un programa necesita a menudo comprobar un texto: ¿contiene una dirección una `@`?
¿Termina un nombre de archivo en `.jpg`? Estos métodos responden con `true` (verdadero)
o `false` (falso), o dan una posición.

```javascript
const frase = "El gato duerme en el sofá";

console.log(frase.includes("gato"));
console.log(frase.includes("Gato"));
console.log(frase.startsWith("El"));
console.log(frase.endsWith("sofá"));
console.log(frase.indexOf("duerme"));
console.log(frase.indexOf("perro"));
```

```sortie
true
false
true
true
8
-1
```

Línea por línea:

- `includes("gato")` da `true`, pero `includes("Gato")` da `false`: la mayúscula
  cuenta;
- `startsWith("El")` y `endsWith("sofá")` dan `true`: la frase empieza y termina así;
- `indexOf("duerme")` da 8: «duerme» empieza en el índice 8, contando los espacios;
- `indexOf("perro")` da `-1`: «perro» no está en la frase.

- `includes(x)`: ¿contiene el texto `x`?
- `startsWith(x)` y `endsWith(x)`: ¿empieza o termina por `x`?
- `indexOf(x)`: la posición de la primera aparición de `x`, o `-1` si no está.

Todas estas búsquedas distinguen entre mayúsculas y minúsculas. Para ignorarlas, pasa
primero los dos textos a minúsculas:

```javascript
const frase = "El Gato duerme";
console.log(frase.toLowerCase().includes("gato"));
```

```sortie
true
```

## Reemplazar

Corregir una palabra en todo un texto, quitar los espacios de un número de teléfono:
es el «buscar y reemplazar» de un procesador de textos.

`replace(viejo, nuevo)` reemplaza la **primera** aparición. `replaceAll` las reemplaza
**todas**.

```javascript
const frase = "el gato y el perro";

console.log(frase.replace("el", "un"));
console.log(frase.replaceAll("el", "un"));
console.log("612 34 56 78".replaceAll(" ", ""));
```

```sortie
un gato y el perro
un gato y un perro
612345678
```

Línea por línea:

- `replace("el", "un")` solo cambia el primer «el»;
- `replaceAll("el", "un")` cambia los dos;
- la última línea reemplaza cada espacio por un texto vacío `""`: los espacios
  desaparecen.

Para búsquedas más precisas («todas las cifras», «una palabra en mayúsculas»), existen
las [expresiones regulares](regex.md).

**Te toca:** escribe un programa que guarde `"19/10/2026"` en una variable y luego la
muestre con guiones en lugar de barras.

> **Solución**
>
> ```javascript
> const fecha = "19/10/2026";
> console.log(fecha.replaceAll("/", "-"));
> ```
>
> ```sortie
> 19-10-2026
> ```

## Alinear y repetir

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

`padStart(longitud, relleno)` completa un texto por el principio hasta la longitud
deseada; `padEnd` completa por el final. `repeat(n)` repite un texto `n` veces.

```javascript
console.log("7".padStart(3, "0"));
console.log(String(42).padStart(5, " "));

const lineas = [["Café", 2.5], ["Cruasán", 1.2]];
for (const [articulo, precio] of lineas) {
  console.log(articulo.padEnd(12, ".") + precio.toFixed(2).padStart(6));
}
console.log("=".repeat(18));
```

```sortie
007
   42
Café........  2.50
Cruasán.....  1.20
==================
```

`padStart(6)` sin segundo argumento completa con espacios. Con un número, primero hay
que convertirlo en texto: `String(42)`.
Estos métodos son perfectos para dibujar tablas alineadas en la terminal.

## Las plantillas de varias líneas

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Una plantilla entre comillas invertidas puede ocupar varias líneas: los saltos de línea
forman parte del texto.

```javascript
const nombre = "Alicia";
const total = 42.5;

const ticket = `Cliente: ${nombre}
Total:   ${total.toFixed(2)} €
¡Gracias por su visita!`;

console.log(ticket);
```

```sortie
Cliente: Alicia
Total:   42.50 €
¡Gracias por su visita!
```

> **Atención:** los espacios de sangría también forman parte del texto. Si desplazas
> las líneas 2 y 3 en tu código, aparecerán desplazadas al mostrarse.

## Ordenar palabras en español

Poner nombres en orden alfabético, como en una guía telefónica, parece sencillo. Pero
el ordenador no conoce el alfabeto español: ordena las letras según unos números.

Comparar dos textos con `<` u ordenar con `sort()` sigue el orden de los **códigos de
los caracteres** (el estándar [Unicode](../debuter/glossaire.md#unicode)). Las
mayúsculas van antes que las minúsculas, y las letras con tilde y la ñ, al final.

```javascript
const palabras = ["zorro", "Ñandú", "árbol", "nube", "Barco", "ñu"];

console.log([...palabras].sort());
console.log([...palabras].sort((a, b) => a.localeCompare(b, "es")));
```

```sortie
[ 'Barco', 'nube', 'zorro', 'Ñandú', 'árbol', 'ñu' ]
[ 'árbol', 'Barco', 'nube', 'Ñandú', 'ñu', 'zorro' ]
```

Línea por línea:

- `palabras` contiene seis palabras, con mayúsculas, tildes y eñes;
- `[...palabras].sort()` ordena una copia según los códigos de los caracteres:
  `"Barco"` va antes que `"nube"`, y las palabras que empiezan por `Ñ`, `á` o `ñ` acaban
  al final;
- la línea siguiente ordena otra copia con `localeCompare`: el orden es el del
  diccionario.

`a.localeCompare(b, "es")` compara dos textos según las reglas del español: devuelve un
número negativo si `a` va antes que `b`, positivo si va después, `0` si son iguales.
`sort` lo usa para ordenar el array. `[...palabras]` crea una copia, para no modificar
el original (ver [los arrays](collections.md)).

> **Recuerda:** para ordenar palabras que va a leer una persona, usa siempre
> `localeCompare` con `"es"`.

## Errores frecuentes

### Modificar un carácter

```javascript norun
const palabra = "gato";
palabra[0] = "G";
```

```text
file:///home/alice/js/main.mjs:2
palabra[0] = "G";
           ^

TypeError: Cannot assign to read only property '0' of string 'gato'
    at file:///home/alice/js/main.mjs:2:12

Node.js v22.22.0
```

Un texto es inmutable: sus caracteres son de solo lectura (*read only*). Crea un texto
nuevo: `const nueva = "G" + palabra.slice(1);`.

### Escribir length con paréntesis

```javascript norun
const nombre = "Alicia";
console.log(nombre.length());
```

```text
file:///home/alice/js/main.mjs:2
console.log(nombre.length());
                   ^

TypeError: nombre.length is not a function
    at file:///home/alice/js/main.mjs:2:20

Node.js v22.22.0
```

`length` es una **propiedad** (un valor), no un método: no se llama. Escribe
`nombre.length`.

### Equivocarse de mayúscula en el nombre de un método

```javascript norun
const nombre = "alicia";
console.log(nombre.toUppercase());
```

```text
file:///home/alice/js/main.mjs:2
console.log(nombre.toUppercase());
                   ^

TypeError: nombre.toUppercase is not a function
    at file:///home/alice/js/main.mjs:2:20

Node.js v22.22.0
```

El método se llama `toUpperCase`, con una `C` mayúscula. Los nombres de JavaScript
están en camelCase: cada palabra nueva empieza por mayúscula.

### Olvidar guardar el resultado

```javascript norun
let entrada = "  hola  ";
entrada.trim();
console.log(`[${entrada}]`);
```

```text
[  hola  ]
```

`trim()` no modifica `entrada`: devuelve un texto nuevo, que aquí se pierde. Escribe
`entrada = entrada.trim();`.

## Ponte a prueba

**Pregunta 1:** ¿qué da `"JavaScript".slice(0, 4)`?

> **Respuesta**
>
> ```javascript
> console.log("JavaScript".slice(0, 4));
> ```
>
> ```sortie
> Java
> ```
>
> Los índices 0, 1, 2 y 3: el final (4) no está incluido.

**Pregunta 2:** ¿cómo transformar `"a;b;c"` en `"a, b, c"`?

> **Respuesta**
>
> ```javascript
> console.log("a;b;c".split(";").join(", "));
> ```
>
> ```sortie
> a, b, c
> ```
>
> `replaceAll(";", ", ")` también funciona.

**Pregunta 3:** ¿por qué ordenar con `localeCompare(b, "es")` en vez de con `sort()`
solo?

> **Respuesta**
>
> `sort()` solo ordena según los códigos de los caracteres: las mayúsculas antes que
> las minúsculas y `"ñ"` después de `"z"`. `localeCompare` sigue el orden alfabético
> español, con la ñ entre la n y la o.

## En resumen

- Un texto se escribe entre `"..."`, `'...'` o `` `...` ``; es inmutable, cada método
  devuelve un texto nuevo.
- `length` da la longitud; `texto[i]` y `texto.at(i)` un carácter, a partir de 0.
- `slice`, `trim`, `toUpperCase` y `toLowerCase` extraen y limpian.
- `split` corta en un array, `join` vuelve a unir; `includes`, `startsWith` e `indexOf`
  buscan; `replace` y `replaceAll` reemplazan.
- `padStart`, `padEnd` y `repeat` alinean; una plantilla puede ocupar varias líneas.
- Ordena palabras en español con `localeCompare(b, "es")`.

Siguiente: [los arrays](collections.md).
