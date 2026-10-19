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

## Los textos nunca cambian

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

- `toUpperCase()` y `toLowerCase()` cambian entre mayúsculas y minúsculas, incluidas
  las tildes y la ñ (`"año"` se convierte en `"AÑO"`).
- `trim()` quita los espacios del principio y del final; `trimStart()` y `trimEnd()`
  solo de un lado.

> **Consejo:** llama a `trim()` sobre todo lo que escribe un usuario: un espacio
> olvidado al final de una contraseña o de un correo electrónico es una fuente de
> errores clásica.

## Extraer un trozo

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

Piensa en los índices como **cortes entre las letras**: `slice(0, 3)` corta antes de la
letra 0 y antes de la letra 3.

`substring(inicio, fin)` hace casi lo mismo, pero no entiende los números negativos.
Quédate con `slice`, que también funciona con los arrays.

## Cortar y unir

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

`split("")` corta en caracteres. Los arrays se explican en
[la página siguiente](collections.md).

## Buscar en un texto

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

Para búsquedas más precisas («todas las cifras», «una palabra en mayúsculas»), existen
las [expresiones regulares](regex.md).

## Alinear y repetir

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
