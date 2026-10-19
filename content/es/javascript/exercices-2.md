# Ejercicios resueltos: segunda serie

Esta página propone 12 nuevos ejercicios resueltos en JavaScript, del más fácil al más difícil, con tablas de traza para seguir las soluciones paso a paso.

## Cómo usar esta página

Estos ejercicios son **nuevos**: no repiten los de los
[ejercicios para empezar con buen pie](exercices-faciles.md) ni los de la
[primera serie](exercices.md). Cada ejercicio tiene un nivel:

- ★☆☆: fácil, una sola noción;
- ★★☆: medio, dos o tres nociones juntas;
- ★★★: más difícil, primero hay que pensar en papel.

Para cada ejercicio:

1. lee el enunciado y la salida esperada;
2. haz un pequeño plan en español, en papel;
3. escribe el código en un archivo `ejercicio.mjs` y ejecuta `node ejercicio.mjs`;
4. ¿atascado? lee la pista y vuelve a intentarlo;
5. abre la solución al final y sigue su tabla de traza con el dedo.

```text
 enunciado → plan en papel → código → ejecutar → comparar
                               ▲                    │
                               └── ¿distinto? ──────┘
```

> **Consejo:** para rellenar una tabla de traza, vuelve a leer
> [seguir un programa paso a paso](pas-a-pas.md).

## Primer nivel

### Ejercicio 1: El tique del aparcamiento

Nivel: ★☆☆

Un aparcamiento cuesta 2 € por hora, pero nunca más de 10 €. Escribe una función
`precioAparcamiento(horas)` que devuelva el precio. Muestra el precio para 1, 3 y 8
horas.

Salida esperada:

```text
1 h: 2 €
3 h: 6 €
8 h: 10 €
```

> **Pista:** calcula primero `horas * 2`. Después, con un `if`, devuelve `10` si ese
> precio supera 10 (mira [las funciones](fonctions.md)).

> **Solución**
>
> ```javascript
> function precioAparcamiento(horas) {
>   const precio = horas * 2;
>   if (precio > 10) {
>     return 10;
>   }
>   return precio;
> }
>
> for (const h of [1, 3, 8]) {
>   console.log(`${h} h: ${precioAparcamiento(h)} €`);
> }
> ```
>
> ```sortie
> 1 h: 2 €
> 3 h: 6 €
> 8 h: 10 €
> ```
>
> Por qué funciona: `return` detiene la función inmediatamente. Si el precio supera
> 10, devolvemos 10 y la última línea nunca se lee. `Math.min(horas * 2, 10)` hace lo
> mismo en una línea.

### Ejercicio 2: Los días de helada

Nivel: ★☆☆

Estas son las temperaturas de una semana: `[3, -2, 5, -7, 0, 4]`. Muestra cada día de
helada (temperatura bajo 0), con su número (el primer día es el día 1), y después el
número de días de helada.

Salida esperada:

```text
Helada el día 2: -2 °C
Helada el día 4: -7 °C
Días de helada: 2
```

> **Pista:** necesitas el número del día: usa un bucle con un índice,
> `for (let i = 0; i < temperaturas.length; i++)`. El número del día es `i + 1`.

> **Solución**
>
> ```javascript
> const temperaturas = [3, -2, 5, -7, 0, 4];
> let diasDeHelada = 0;
> for (let i = 0; i < temperaturas.length; i++) {
>   if (temperaturas[i] < 0) {
>     console.log(`Helada el día ${i + 1}: ${temperaturas[i]} °C`);
>     diasDeHelada = diasDeHelada + 1;
>   }
> }
> console.log(`Días de helada: ${diasDeHelada}`);
> ```
>
> ```sortie
> Helada el día 2: -2 °C
> Helada el día 4: -7 °C
> Días de helada: 2
> ```
>
> Por qué funciona: los índices empiezan en 0 y los días en 1, de ahí `i + 1`. El
> día 5 vale `0`: `0 < 0` es falso, no es helada.

### Ejercicio 3: La palabra más larga

Nivel: ★☆☆

En el array `["gato", "elefante", "ratón", "oso"]`, encuentra la palabra más larga y
muéstrala con su número de letras.

Salida esperada:

```text
La palabra más larga: elefante (8 letras)
```

> **Pista:** guarda la mejor palabra encontrada en una caja `masLarga`, que empieza
> con la primera palabra. En cada vuelta, compara los `.length` (mira
> [el texto](texte.md)).

> **Solución**
>
> ```javascript
> const palabras = ["gato", "elefante", "ratón", "oso"];
> let masLarga = palabras[0];
> for (const palabra of palabras) {
>   if (palabra.length > masLarga.length) {
>     masLarga = palabra;
>   }
> }
> console.log(`La palabra más larga: ${masLarga} (${masLarga.length} letras)`);
> ```
>
> ```sortie
> La palabra más larga: elefante (8 letras)
> ```
>
> Por qué funciona: es el principio del campeón. Guardamos el mejor, y cada palabra
> nueva tiene que ganarle para ocupar su lugar.
>
> | Vuelta | palabra | longitud | ¿más larga que masLarga? | masLarga |
> | --- | --- | --- | --- | --- |
> | inicio | – | – | – | gato |
> | 1 | gato | 4 | 4 > 4 → no | gato |
> | 2 | elefante | 8 | 8 > 4 → sí | elefante |
> | 3 | ratón | 5 | 5 > 8 → no | elefante |
> | 4 | oso | 3 | 3 > 8 → no | elefante |

### Ejercicio 4: El triángulo de estrellas

Nivel: ★☆☆

Muestra un triángulo de 4 líneas: una estrella en la primera línea, dos en la
segunda, y así sucesivamente.

Salida esperada:

```text
*
**
***
****
```

> **Pista:** `"*".repeat(3)` da `"***"`. Haz un bucle de 1 a 4 y repite la estrella
> tantas veces como el número de la línea.

> **Solución**
>
> ```javascript
> for (let linea = 1; linea <= 4; linea++) {
>   console.log("*".repeat(linea));
> }
> ```
>
> ```sortie
> *
> **
> ***
> ****
> ```
>
> Por qué funciona: la variable del bucle sirve dos veces, para contar las líneas y
> para decir cuántas estrellas poner.
>
> | Vuelta | linea | ¿linea <= 4? | se muestra |
> | --- | --- | --- | --- |
> | 1 | 1 | sí | `*` |
> | 2 | 2 | sí | `**` |
> | 3 | 3 | sí | `***` |
> | 4 | 4 | sí | `****` |
> | fin | 5 | no | (el bucle se detiene) |

## Segundo nivel

### Ejercicio 5: Dar el cambio

Nivel: ★★☆

Una caja registradora debe devolver 470 céntimos con el menor número de monedas
posible. Las monedas valen 200, 100, 50, 20 y 10 céntimos. Escribe
`darCambio(importe)` que muestre cuántas monedas de cada tipo hacen falta (solo las
que se usan).

Salida esperada:

```text
2 × 200 c
1 × 50 c
1 × 20 c
```

> **Pista:** empieza por la moneda más grande. Mientras el resto sea lo bastante
> grande, da esa moneda y resta su valor del resto. Después pasa a la moneda
> siguiente.

> **Pista:** hacen falta dos bucles: un `for…of` sobre las monedas, y dentro un
> `while (resto >= moneda)`. Trabaja en céntimos para evitar los decimales
> (mira [los números](nombres.md)).

> **Solución**
>
> ```javascript
> function darCambio(importe) {
>   const monedas = [200, 100, 50, 20, 10];
>   let resto = importe;
>   for (const moneda of monedas) {
>     let cantidad = 0;
>     while (resto >= moneda) {
>       resto = resto - moneda;
>       cantidad = cantidad + 1;
>     }
>     if (cantidad > 0) {
>       console.log(`${cantidad} × ${moneda} c`);
>     }
>   }
> }
>
> darCambio(470);
> ```
>
> ```sortie
> 2 × 200 c
> 1 × 50 c
> 1 × 20 c
> ```
>
> Por qué funciona: vaciamos el resto empezando por las monedas grandes, como un
> cajero que primero coge del compartimento de 2 €.
>
> | Paso | moneda | ¿resto >= moneda? | resto | cantidad |
> | --- | --- | --- | --- | --- |
> | inicio | – | – | 470 | – |
> | 1 | 200 | 470 >= 200 → sí | 270 | 1 |
> | 2 | 200 | 270 >= 200 → sí | 70 | 2 |
> | 3 | 200 | 70 >= 200 → no | 70 | 2 (se muestra) |
> | 4 | 100 | 70 >= 100 → no | 70 | 0 |
> | 5 | 50 | 70 >= 50 → sí | 20 | 1 |
> | 6 | 50 | 20 >= 50 → no | 20 | 1 (se muestra) |
> | 7 | 20 | 20 >= 20 → sí | 0 | 1 |
> | 8 | 20 | 0 >= 20 → no | 0 | 1 (se muestra) |
> | 9 | 10 | 0 >= 10 → no | 0 | 0 |

### Ejercicio 6: Quitar los duplicados

Nivel: ★★☆

Escribe `sinDuplicados(valores)` que devuelva un **nuevo** array sin los valores
repetidos, en el orden de su primera aparición. Pruébala con `[3, 1, 3, 2, 1]`.

Salida esperada:

```text
[ 3, 1, 2 ]
[ 'a', 'b' ]
```

> **Pista:** crea un array vacío `resultado`. Para cada valor, añádelo solo si
> `resultado.includes(v)` es falso (mira [los arrays](collections.md)).

> **Solución**
>
> ```javascript
> function sinDuplicados(valores) {
>   const resultado = [];
>   for (const v of valores) {
>     if (!resultado.includes(v)) {
>       resultado.push(v);
>     }
>   }
>   return resultado;
> }
>
> console.log(sinDuplicados([3, 1, 3, 2, 1]));
> console.log(sinDuplicados(["a", "b", "a"]));
> ```
>
> ```sortie
> [ 3, 1, 2 ]
> [ 'a', 'b' ]
> ```
>
> Por qué funciona: `!` invierte la respuesta de `includes`. Así que solo añadimos un
> valor cuando todavía no está en `resultado`.
>
> | Vuelta | v | ¿ya está en resultado? | resultado |
> | --- | --- | --- | --- |
> | inicio | – | – | `[]` |
> | 1 | 3 | no | `[3]` |
> | 2 | 1 | no | `[3, 1]` |
> | 3 | 3 | sí | `[3, 1]` |
> | 4 | 2 | no | `[3, 1, 2]` |
> | 5 | 1 | sí | `[3, 1, 2]` |
>
> Más adelante, escribirás simplemente `[...new Set(valores)]`: un `Set` nunca guarda
> dos veces el mismo valor (mira [los objetos](objets.md)).

### Ejercicio 7: Agrupar por inicial

Nivel: ★★☆

Agrupa los nombres `["Alice", "Bob", "Anne", "Chloé", "Bruno"]` por su primera letra,
en un objeto. Después muestra una línea por letra.

Salida esperada:

```text
A: Alice, Anne
B: Bob, Bruno
C: Chloé
```

> **Pista:** el objeto `grupos` tiene una clave por letra y, como valor, un array de
> nombres. `nombre[0]` da la primera letra.

> **Pista:** si `grupos[inicial]` vale `undefined`, crea primero el cajón vacío
> `grupos[inicial] = []` y después haz `push`. Para mostrarlo, usa `Object.entries`
> (mira [los objetos](objets.md)).

> **Solución**
>
> ```javascript
> const nombres = ["Alice", "Bob", "Anne", "Chloé", "Bruno"];
> const grupos = {};
> for (const nombre of nombres) {
>   const inicial = nombre[0];
>   if (grupos[inicial] === undefined) {
>     grupos[inicial] = [];
>   }
>   grupos[inicial].push(nombre);
> }
>
> for (const [letra, lista] of Object.entries(grupos)) {
>   console.log(`${letra}: ${lista.join(", ")}`);
> }
> ```
>
> ```sortie
> A: Alice, Anne
> B: Bob, Bruno
> C: Chloé
> ```
>
> Por qué funciona: es un mueble con cajones. Abrimos el cajón de la letra; si no
> existe, lo creamos vacío; después guardamos el nombre en él.

### Ejercicio 8: Comprobar una contraseña

Nivel: ★★☆

Escribe `problemasContrasena(clave)` que devuelva la lista de problemas de una
contraseña: menos de 8 caracteres, ningún dígito, ninguna mayúscula. Pruébala con
`"gato"`, `"verano2024"` y `"Verano2024"`.

Salida esperada:

```text
gato: menos de 8 caracteres, ningún dígito, ninguna mayúscula
verano2024: ninguna mayúscula
Verano2024: correcta
```

> **Pista:** tres `if` separados (sin `else`), porque pueden existir varios problemas
> a la vez. Cada `if` añade un texto a un array `problemas`.

> **Pista:** un texto sin mayúsculas no cambia con `toLowerCase()`. Para los dígitos,
> busca si un carácter está en `"0123456789"`.

> **Solución**
>
> ```javascript
> function problemasContrasena(clave) {
>   const problemas = [];
>   if (clave.length < 8) {
>     problemas.push("menos de 8 caracteres");
>   }
>   if (![...clave].some((c) => "0123456789".includes(c))) {
>     problemas.push("ningún dígito");
>   }
>   if (clave === clave.toLowerCase()) {
>     problemas.push("ninguna mayúscula");
>   }
>   return problemas;
> }
>
> for (const clave of ["gato", "verano2024", "Verano2024"]) {
>   const problemas = problemasContrasena(clave);
>   if (problemas.length === 0) {
>     console.log(`${clave}: correcta`);
>   } else {
>     console.log(`${clave}: ${problemas.join(", ")}`);
>   }
> }
> ```
>
> ```sortie
> gato: menos de 8 caracteres, ningún dígito, ninguna mayúscula
> verano2024: ninguna mayúscula
> Verano2024: correcta
> ```
>
> Por qué funciona: con `else if`, el programa se detendría en el primer problema.
> Tres `if` independientes lo comprueban todo, como una lista de control en la que
> marcas cada casilla.

### Ejercicio 9: El cifrado César

Nivel: ★★☆

Julio César ocultaba sus mensajes desplazando cada letra 3 posiciones en el alfabeto:
`a` se convierte en `d`, `z` se convierte en `c`. Escribe `cesar(texto, desplazamiento)`
para textos en minúsculas sin acentos; los demás caracteres no cambian. Cifra
`"hola zoe"` y después descifra el resultado con un desplazamiento de 23.

Salida esperada:

```text
krod crh
hola zoe
```

> **Pista:** `ALFABETO.indexOf(c)` da la posición de una letra (`-1` si no es una
> letra). La nueva posición es `(posicion + desplazamiento) % 26`.

> **Solución**
>
> ```javascript
> const ALFABETO = "abcdefghijklmnopqrstuvwxyz";
>
> function cesar(texto, desplazamiento) {
>   let resultado = "";
>   for (const c of texto) {
>     const posicion = ALFABETO.indexOf(c);
>     if (posicion === -1) {
>       resultado += c;
>     } else {
>       resultado += ALFABETO[(posicion + desplazamiento) % 26];
>     }
>   }
>   return resultado;
> }
>
> const secreto = cesar("hola zoe", 3);
> console.log(secreto);
> console.log(cesar(secreto, 23));
> ```
>
> ```sortie
> krod crh
> hola zoe
> ```
>
> Por qué funciona: `% 26` hace girar el alfabeto en círculo, como las agujas de un
> reloj. Para `z`: posición 25, 25 + 3 = 28, 28 % 26 = 2, es la `c`. Desplazar 3 y
> después 23 da 26: una vuelta completa, volvemos al principio.
>
> ```text
>  a b c d e … x y z
>  ▲               │
>  └───────────────┘
>   después de z, se vuelve a la a
> ```

## Tercer nivel

### Ejercicio 10: La ordenación de burbuja

Nivel: ★★★

Ordena `[5, 2, 4, 1]` de menor a mayor **sin** usar `sort`. Método: recorre el array
e intercambia dos vecinos si están en el orden equivocado. Vuelve a empezar hasta que
todo esté ordenado. Muestra el array después de cada pasada. El array de partida no
debe cambiar.

Salida esperada:

```text
Tras la pasada 1: 2 4 1 5
Tras la pasada 2: 2 1 4 5
Tras la pasada 3: 1 2 4 5
Ordenado: 1 2 4 5, original: 5 2 4 1
```

> **Pista:** trabaja sobre una copia: `const t = [...array];`. Para intercambiar
> `t[i]` y `t[i + 1]`, guarda uno de ellos en una caja `temp` (mira
> [seguir un programa paso a paso](pas-a-pas.md)).

> **Pista:** después de cada pasada, el número más grande que queda ha llegado al
> final, como una burbuja que sube. Para un array de 4 números hacen falta 3 pasadas,
> y la pasada número `p` se detiene antes de las `p` últimas casillas.

> **Solución**
>
> ```javascript
> function ordenarBurbuja(array) {
>   const t = [...array];
>   for (let pasada = 1; pasada < t.length; pasada++) {
>     for (let i = 0; i < t.length - pasada; i++) {
>       if (t[i] > t[i + 1]) {
>         const temp = t[i];
>         t[i] = t[i + 1];
>         t[i + 1] = temp;
>       }
>     }
>     console.log(`Tras la pasada ${pasada}: ${t.join(" ")}`);
>   }
>   return t;
> }
>
> const numeros = [5, 2, 4, 1];
> const ordenados = ordenarBurbuja(numeros);
> console.log(`Ordenado: ${ordenados.join(" ")}, original: ${numeros.join(" ")}`);
> ```
>
> ```sortie
> Tras la pasada 1: 2 4 1 5
> Tras la pasada 2: 2 1 4 5
> Tras la pasada 3: 1 2 4 5
> Ordenado: 1 2 4 5, original: 5 2 4 1
> ```
>
> Por qué funciona: cada pasada empuja el número más grande que queda hacia la
> derecha. Esta es la primera pasada en detalle:
>
> | pasada | i | compara | ¿intercambio? | t |
> | --- | --- | --- | --- | --- |
> | 1 | 0 | 5 > 2 | sí | `[2, 5, 4, 1]` |
> | 1 | 1 | 5 > 4 | sí | `[2, 4, 5, 1]` |
> | 1 | 2 | 5 > 1 | sí | `[2, 4, 1, 5]` |
>
> El 5 está al final: está en su sitio. La pasada 2 solo mira las 3 primeras casillas,
> la pasada 3 las 2 primeras.

### Ejercicio 11: Adivinar un número en pocos intentos

Nivel: ★★★

Un número secreto está entre 1 y 100. En cada intento te dicen «demasiado bajo» o
«demasiado alto». Escribe `adivinar(secreto)` que pruebe siempre el **centro** del
intervalo todavía posible y muestre cada intento. Pruébala con `71`.

Salida esperada:

```text
50: demasiado bajo
75: demasiado alto
62: demasiado bajo
68: demasiado bajo
71: encontrado en 5 intentos
```

> **Pista:** guarda dos límites, `bajo = 1` y `alto = 100`. El centro es
> `Math.floor((bajo + alto) / 2)`.

> **Pista:** si el centro es demasiado bajo, el secreto está por encima:
> `bajo = centro + 1`. Si es demasiado alto: `alto = centro - 1`. Repite con
> `while (bajo <= alto)`.

> **Solución**
>
> ```javascript
> function adivinar(secreto) {
>   let bajo = 1;
>   let alto = 100;
>   let intentos = 0;
>   while (bajo <= alto) {
>     const centro = Math.floor((bajo + alto) / 2);
>     intentos = intentos + 1;
>     if (centro === secreto) {
>       console.log(`${centro}: encontrado en ${intentos} intentos`);
>       return intentos;
>     } else if (centro < secreto) {
>       console.log(`${centro}: demasiado bajo`);
>       bajo = centro + 1;
>     } else {
>       console.log(`${centro}: demasiado alto`);
>       alto = centro - 1;
>     }
>   }
>   return -1;
> }
>
> adivinar(71);
> ```
>
> ```sortie
> 50: demasiado bajo
> 75: demasiado alto
> 62: demasiado bajo
> 68: demasiado bajo
> 71: encontrado en 5 intentos
> ```
>
> Por qué funciona: cada intento corta el intervalo en dos, como cuando abres un
> diccionario por la mitad para buscar una palabra. Nunca hacen falta más de 7
> intentos para 100 números.
>
> | Intento | bajo | alto | centro | respuesta |
> | --- | --- | --- | --- | --- |
> | 1 | 1 | 100 | 50 | demasiado bajo |
> | 2 | 51 | 100 | 75 | demasiado alto |
> | 3 | 51 | 74 | 62 | demasiado bajo |
> | 4 | 63 | 74 | 68 | demasiado bajo |
> | 5 | 69 | 74 | 71 | encontrado |

### Ejercicio 12: Los paréntesis equilibrados

Nivel: ★★★

Escribe `equilibrado(texto)` que diga si los paréntesis `()`, corchetes `[]` y llaves
`{}` de un texto están bien cerrados, en el orden correcto. Los demás caracteres se
ignoran. Pruébala con `"(a[b]{c})"`, `"(]"`, `"((a)"` y `"f(x[0])"`.

Salida esperada:

```text
(a[b]{c}): true
(]: false
((a): false
f(x[0]): true
```

> **Pista:** usa un array como una **pila** de platos: `push` pone un signo de
> apertura encima, `pop` retira el de encima.

> **Pista:** en cada signo de cierre, el último signo de apertura colocado debe ser su
> pareja. Un objeto `{ ")": "(", "]": "[", "}": "{" }` da la pareja. Al final, la pila
> debe estar vacía.

> **Solución**
>
> ```javascript
> function equilibrado(texto) {
>   const aperturas = { ")": "(", "]": "[", "}": "{" };
>   const pila = [];
>   for (const c of texto) {
>     if (c === "(" || c === "[" || c === "{") {
>       pila.push(c);
>     } else if (c in aperturas) {
>       if (pila.pop() !== aperturas[c]) {
>         return false;
>       }
>     }
>   }
>   return pila.length === 0;
> }
>
> for (const texto of ["(a[b]{c})", "(]", "((a)", "f(x[0])"]) {
>   console.log(`${texto}: ${equilibrado(texto)}`);
> }
> ```
>
> ```sortie
> (a[b]{c}): true
> (]: false
> ((a): false
> f(x[0]): true
> ```
>
> Por qué funciona: el último signo de apertura colocado es el primero en cerrarse,
> igual que el último plato colocado es el primero que se retira. Esta es la traza de
> `"(a[b]{c})"`:
>
> | Vuelta | c | acción | pila |
> | --- | --- | --- | --- |
> | 1 | `(` | poner | `["("]` |
> | 2 | `a` | ignorar | `["("]` |
> | 3 | `[` | poner | `["(", "["]` |
> | 4 | `b` | ignorar | `["(", "["]` |
> | 5 | `]` | retirar `[`: pareja, ok | `["("]` |
> | 6 | `{` | poner | `["(", "{"]` |
> | 7 | `c` | ignorar | `["(", "{"]` |
> | 8 | `}` | retirar `{`: pareja, ok | `["("]` |
> | 9 | `)` | retirar `(`: pareja, ok | `[]` |
>
> La pila está vacía al final: `true`. Para `"((a)"`, queda un `(` en la pila:
> `false`. Para `"(]"`, retiramos `(` cuando hacía falta `[`: `false` enseguida.

## En resumen

- Los ejercicios 1 a 4 trabajan una noción cada vez: función, índice, campeón,
  repetición.
- Los ejercicios 5 a 9 combinan bucles, condiciones, arrays, objetos y texto.
- Los ejercicios 10 a 12 son verdaderos pequeños algoritmos: ordenar, buscar,
  comprobar.
- Cuando una solución se te escape, rellena su tabla de traza con un lápiz.

Siguiente: ten [la chuleta](aide-memoire.md) siempre a mano.
