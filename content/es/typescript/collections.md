# Arrays y tuplas

Esta página explica cómo guardar varios valores en arrays y tuplas, cómo transformarlos, y cómo los tipos mantienen coherentes estas listas.

> **Objetivos:**
>
> - sabrás crear un array tipado (`number[]`), leerlo y modificarlo;
> - sabrás proteger un array con `readonly` y describir una tupla
>   `[string, number]`;
> - sabrás transformar un array con `map`, `filter` y `reduce`;
> - sabrás manejar el `undefined` que devuelve `find`.

## Crear un array

Imagina una lista de la compra: una sola hoja, varios artículos, en un orden preciso.
Un array cumple ese papel en un programa: guarda varios valores bajo un solo nombre.

Un array es una [lista](../debuter/glossaire.md#lista) ordenada de valores, escrita
entre corchetes. Su tipo se escribe con el tipo de los elementos seguido de `[]`:

```typescript
const notas: number[] = [12, 15, 9];
const nombres: string[] = ["Alice", "Bob", "Chloé"];
const vacio: number[] = [];

console.log(notas);
console.log(nombres.length, vacio.length);
```

```sortie
[ 12, 15, 9 ]
3 0
```

Línea por línea:

- `notas` recibe tres números; `number[]` anuncia «solo números»;
- `nombres` recibe tres textos, y `vacio` un array sin ningún elemento;
- `console.log(notas)` muestra el array entero, entre corchetes;
- `.length` da el número de elementos: 3 para `nombres`, 0 para `vacio`.

`number[]` se lee «array de números». Existe otra forma de escribirlo, que significa
exactamente lo mismo: `Array<number>`. Los signos `< >` indican el tipo de los
elementos; los volverás a ver en [los genéricos](generiques.md).

```typescript
const ciudades: Array<string> = ["Lyon", "Nantes"];
console.log(ciudades);
```

```sortie
[ 'Lyon', 'Nantes' ]
```

Con valores iniciales, TypeScript adivina el tipo: `const notas = [12, 15, 9]`
también da un `number[]`. Escribe el tipo para un array vacío: sin él, TypeScript no
puede saber qué vas a guardar en él.

> **Recuerda:** todos los elementos de un array tienen el mismo tipo. Un array de
> números rechaza un texto: es la garantía de que cada elemento se maneja de la misma
> forma.

## Leer y modificar

Una lista está viva: miras un artículo, cambias uno, añades uno abajo, tachas el
último. Un array permite todo eso.

Cada elemento tiene un [índice](../debuter/glossaire.md#indice) que empieza en **0**.

```typescript
const frutas: string[] = ["manzana", "pera", "kiwi"];

console.log(frutas[0]);
console.log(frutas.at(-1));
frutas[1] = "mango";
frutas.push("fresa");
console.log(frutas);

const ultima = frutas.pop();
console.log(ultima, frutas);
console.log(frutas.includes("kiwi"), frutas.indexOf("kiwi"));
```

```sortie
manzana
kiwi
[ 'manzana', 'mango', 'kiwi', 'fresa' ]
fresa [ 'manzana', 'mango', 'kiwi' ]
true 2
```

Línea por línea:

- `frutas[0]` lee la primera casilla: `"manzana"`; `at(-1)` cuenta desde el final:
  `"kiwi"`;
- `frutas[1] = "mango"` reemplaza `"pera"`, y luego `push` añade `"fresa"`: 4
  elementos;
- `pop()` quita `"fresa"` y la guarda en `ultima`: quedan 3 elementos;
- `"kiwi"` sigue ahí, en la posición 2 (la tercera casilla, porque se cuenta desde 0).

Los métodos que debes conocer:

- `frutas[1] = ...` reemplaza un elemento;
- `push` añade al final, `pop` quita y devuelve el último;
- `includes` responde `true` o `false`, `indexOf` da la posición (`-1` si no está).

Incluso con `const`, el array puede cambiar: `const` solo impide reemplazar **todo** el
array por otro.

> **Atención:** leer un índice que no existe (`frutas[10]`) da `undefined`, pero
> TypeScript anuncia de todos modos un `string`. Comprueba antes la longitud, o
> prefiere `at()`, cuyo tipo `string | undefined` te recuerda el riesgo.

**Te toca:** escribe un programa que cree un array `compra` con `"pan"` y `"leche"`,
añada `"huevos"` al final y luego muestre el número de artículos y el array.

> **Solución**
>
> ```typescript
> const compra: string[] = ["pan", "leche"];
> compra.push("huevos");
> console.log(compra.length);
> console.log(compra);
> ```
>
> ```sortie
> 3
> [ 'pan', 'leche', 'huevos' ]
> ```

### Recorrer un array

`for...of` da cada elemento. Con `entries()`, obtienes también el índice.

```typescript
const podio = ["Alice", "Bob", "Chloé"];

for (const [i, nombre] of podio.entries()) {
  console.log(`${i + 1}. ${nombre}`);
}
```

```sortie
1. Alice
2. Bob
3. Chloé
```

`const [i, nombre]` separa cada par en dos variables: es la **desestructuración**,
explicada más abajo con las tuplas.

## Los arrays de solo lectura

Algunas listas no deben cambiar nunca: los días de la semana, los meses del año.
`readonly` te protege de modificarlas por error.

Añade `readonly` delante del tipo: el array ya no se puede modificar. `push`, `pop` o
`frutas[0] = ...` se rechazan.

```typescript
const dias: readonly string[] = ["lunes", "martes", "miércoles"];

console.log(dias[0], dias.length);
console.log(dias.includes("martes"));
```

```sortie
lunes 3
true
```

Solo quedan disponibles los métodos que **leen**. Es útil para una lista fija, o para
prometer que una función no tocará el array que le das:

```typescript
function media(valores: readonly number[]): number {
  let total = 0;
  for (const v of valores) {
    total += v;
  }
  return total / valores.length;
}

console.log(media([12, 15, 9]));
```

```sortie
12
```

## Las tuplas

A veces, dos o tres valores de tipos distintos van juntos, como una fila de una tabla
de puntuaciones: un nombre y unos puntos. La tupla los mantiene uno al lado del otro,
en un orden fijo.

Una [tupla](../debuter/glossaire.md#tupla) es un array de **longitud fija**, donde
cada posición tiene su propio tipo. `[string, number]` significa: «exactamente dos
elementos, un texto y luego un número».

```typescript
const jugador: [string, number] = ["Alice", 1200];

console.log(jugador[0].toUpperCase());
console.log(jugador[1] + 50);

const [nombre, puntos] = jugador;
console.log(`${nombre} tiene ${puntos} puntos.`);
```

```sortie
ALICE
1250
Alice tiene 1200 puntos.
```

Línea por línea:

- `jugador` contiene dos casillas: `"Alice"` (un texto) y luego `1200` (un número);
- `jugador[0].toUpperCase()` pone el texto en mayúsculas: `ALICE`;
- `jugador[1] + 50` calcula con el número: `1250`. `jugador` no cambia;
- el final guarda `"Alice"` en `nombre` y `1200` en `puntos`, y muestra la frase.

TypeScript sabe que `jugador[0]` es un `string` y `jugador[1]` un `number`. La línea
`const [nombre, puntos] = jugador;` guarda el primer elemento en `nombre` y el segundo
en `puntos`.

Una tupla es práctica para devolver dos valores desde una función:

```typescript
function minMax(valores: number[]): [number, number] {
  return [Math.min(...valores), Math.max(...valores)];
}

const [menor, mayor] = minMax([4, 9, 1, 7]);
console.log(menor, mayor);
```

```sortie
1 9
```

`...valores` (la **propagación**, *spread*) despliega el array en argumentos
separados: `Math.min(4, 9, 1, 7)`.

> **Consejo:** a partir de dos o tres elementos, una tupla se vuelve difícil de leer
> (¿qué significa `jugador[3]`?). Prefiere entonces un objeto con nombres de
> propiedades: ver [los objetos](objets.md).

**Te toca:** escribe un programa que guarde un producto en una tupla
`[string, number]`: `"bolígrafo"` y `2`. Sepáralo en dos variables y muestra
«bolígrafo cuesta 2 €».

> **Solución**
>
> ```typescript
> const producto: [string, number] = ["bolígrafo", 2];
> const [articulo, precio] = producto;
> console.log(`${articulo} cuesta ${precio} €`);
> ```
>
> ```sortie
> bolígrafo cuesta 2 €
> ```

## Transformar: map, filter, reduce

Muy a menudo, quieres hacer lo mismo con toda una lista: subir todos los precios,
quedarte con las buenas notas, calcular un total. Estos métodos lo hacen en una línea,
sin escribir un bucle.

Estos tres métodos reciben un *callback* (una función, ver
[las funciones](fonctions-avancees.md#los-callbacks-pasar-una-funcion-a-una-funcion)). No
modifican el array original: devuelven uno nuevo, o un valor.

### map: transformar cada elemento

```typescript
const precios = [10, 20, 35];
const preciosConIva = precios.map((p) => p * 1.2);
const etiquetas = precios.map((p) => `${p} €`);

console.log(preciosConIva);
console.log(etiquetas);
```

```sortie
[ 12, 24, 42 ]
[ '10 €', '20 €', '35 €' ]
```

Línea por línea:

- `precios` contiene tres números;
- `precios.map((p) => p * 1.2)` da cada precio a `p`, uno tras otro, y guarda cada
  resultado en un array nuevo: `[12, 24, 42]`;
- la línea siguiente hace lo mismo, pero el callback fabrica un texto: `"10 €"`…;
- `precios` no ha cambiado: sigue valiendo `[10, 20, 35]`.

TypeScript lo adivina todo: `p` es un `number` (un elemento de `precios`),
`preciosConIva` es un `number[]`, y `etiquetas` un `string[]`, porque el callback
devuelve un texto. Pasa el ratón sobre estos nombres en el editor para verlo.

### filter: quedarse con algunos elementos

```typescript
const notas = [12, 7, 15, 9, 18];
const aprobadas = notas.filter((n) => n >= 10);

console.log(aprobadas);
console.log(notas);
```

```sortie
[ 12, 15, 18 ]
[ 12, 7, 15, 9, 18 ]
```

El callback responde `true` (quedárselo) o `false` (descartarlo). `notas` no ha
cambiado.

### reduce: resumirlo todo en un valor

`reduce` recorre el array guardando un **acumulador**: un valor que crece en cada
vuelta, como una hucha. El segundo argumento es su valor inicial.

```typescript
const compras = [4.5, 12, 3.5];
const total = compras.reduce((suma, precio) => suma + precio, 0);
console.log(total);

const palabras = ["lobo", "elefante", "gato"];
const letras = palabras.reduce((acc, palabra) => acc + palabra.length, 0);
console.log(letras);
```

```sortie
20
16
```

En la primera vuelta, `suma` vale `0` (el valor inicial); después `4.5`, `16.5` y por
último `20`. El tipo del acumulador viene del valor inicial: `0` da un `number`.

**Te toca:** a partir de `const numeros = [2, 5, 8];`, escribe un programa que cree
el array de los dobles con `map` y luego calcule la suma de `numeros` con `reduce`.
Muestra los dos resultados.

> **Solución**
>
> ```typescript
> const numeros = [2, 5, 8];
> const dobles = numeros.map((n) => n * 2);
> const suma = numeros.reduce((acc, n) => acc + n, 0);
> console.log(dobles);
> console.log(suma);
> ```
>
> ```sortie
> [ 4, 10, 16 ]
> 15
> ```

### Encadenar

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Como cada método devuelve un array, se pueden encadenar:

```typescript
const notas = [12, 7, 15, 9, 18];

const totalConBonus = notas
  .filter((n) => n >= 10)
  .map((n) => n + 1)
  .reduce((acc, n) => acc + n, 0);

console.log(totalConBonus);
```

```sortie
48
```

Lee de arriba abajo: te quedas con las notas aprobadas (`12, 15, 18`), sumas un punto
extra a cada una (`13, 16, 19`) y luego calculas la suma.

### Ordenar

`sort` ordena el array **en el sitio**. Atención: sin callback, ordena como texto,
¡incluso los números! Da siempre una función de comparación `(a, b) => a - b`.

```typescript
const numeros = [10, 9, 1, 100];

console.log([...numeros].sort());
console.log([...numeros].sort((a, b) => a - b));
console.log([...numeros].sort((a, b) => b - a));
```

```sortie
[ 1, 10, 100, 9 ]
[ 1, 9, 10, 100 ]
[ 100, 10, 9, 1 ]
```

`[...numeros]` crea una copia, para no modificar el propio `numeros`.

## find: un resultado que puede faltar

Buscar en una lista es como buscar un nombre en una agenda: a veces no está.
TypeScript te obliga a prever ese caso.

`find` devuelve el **primer** elemento que cumple la prueba. Si no encuentra ninguno,
devuelve `undefined`. Su tipo lo dice claramente: para un `string[]`, es
`string | undefined`.

```typescript
const nombres = ["Alice", "Bob", "Chloé"];

const conC = nombres.find((p) => p.startsWith("C"));
const conZ = nombres.find((p) => p.startsWith("Z"));

console.log(conC, conZ);
```

```sortie
Chloé undefined
```

Línea por línea:

- `find` prueba cada nombre con la prueba `p.startsWith("C")` («¿empieza por C?»);
- `"Alice"` y `"Bob"` fallan, `"Chloé"` la supera: `conC` vale `"Chloé"`;
- ningún nombre empieza por Z: `conZ` vale `undefined`.

Aquí es donde TypeScript te protege: no puedes usar el resultado como un `string` sin
tratar el caso `undefined`. Tres formas de hacerlo:

```typescript
const nombres = ["Alice", "Bob", "Chloé"];
const encontrado = nombres.find((p) => p.startsWith("B"));

// 1. Comprobar antes de usar
if (encontrado !== undefined) {
  console.log(encontrado.toUpperCase());
} else {
  console.log("Nadie.");
}

// 2. Llamar solo si el valor existe
console.log(encontrado?.toLowerCase());

// 3. Dar un valor por defecto
const nombre = encontrado ?? "desconocido";
console.log(nombre.length);
```

```sortie
BOB
bob
3
```

Después de la prueba `encontrado !== undefined`, TypeScript sabe que `encontrado` es
un `string`.

> **Recuerda:** un tipo `T | undefined` es una pregunta que TypeScript te hace: «¿y si
> no se encuentra nada?». Respóndela, con `if`, `?.` o `??`.

## Errores frecuentes

### Guardar un valor del tipo incorrecto

```typescript norun
const notas: number[] = [12, 15];
notas.push("18");
```

```text
main.ts:2:12 - error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'.

2 notas.push("18");
             ~~~~


Found 1 error in main.ts:2
```

`"18"` es un texto. Un `number[]` solo acepta números: `notas.push(18)`.

### Modificar un array readonly

```typescript norun
const dias: readonly string[] = ["lunes", "martes"];
dias.push("miércoles");
```

```text
main.ts:2:6 - error TS2339: Property 'push' does not exist on type 'readonly string[]'.

2 dias.push("miércoles");
       ~~~~


Found 1 error in main.ts:2
```

Un array `readonly` no tiene método `push`. Crea un array nuevo en su lugar:
`const semana = [...dias, "miércoles"];`.

### Invertir el orden de una tupla

```typescript norun
const jugador: [string, number] = [42, "Alice"];
```

```text
main.ts:1:36 - error TS2322: Type 'number' is not assignable to type 'string'.

1 const jugador: [string, number] = [42, "Alice"];
                                     ~~

main.ts:1:40 - error TS2322: Type 'string' is not assignable to type 'number'.

1 const jugador: [string, number] = [42, "Alice"];
                                         ~~~~~~~


Found 2 errors in the same file, starting at: main.ts:1
```

En una tupla, el orden importa: primero el texto, luego el número.
Escribe `["Alice", 42]`.

## Ponte a prueba

**Pregunta 1:** ¿cuál es el tipo de `["a", "b"].map((s) => s.length)`?

> **Respuesta**
>
> `number[]`. El callback devuelve `s.length`, un número, para cada elemento.

**Pregunta 2:** ¿qué diferencia hay entre `string[]` y `[string, string]`?

> **Respuesta**
>
> `string[]` es un array de textos de cualquier longitud. `[string, string]` es una
> tupla: exactamente dos textos.

**Pregunta 3:** completa el código para mostrar el primer número par, o `ninguno`.

```typescript norun
const numeros = [3, 7, 8, 5];
const par = numeros.find((n) => n % 2 === 0);
```

> **Respuesta**
>
> ```typescript
> const numeros = [3, 7, 8, 5];
> const par = numeros.find((n) => n % 2 === 0);
> console.log(par ?? "ninguno");
> ```
>
> ```sortie
> 8
> ```
>
> `par` es de tipo `number | undefined`: `??` trata el caso en que no se encuentra
> nada.

## En resumen

- Un array se escribe `number[]` o `Array<number>`; todos sus elementos tienen el
  mismo tipo. `readonly number[]` impide que cambie.
- Una tupla `[string, number]` tiene una longitud fija y un tipo por posición.
- `map`, `filter` y `reduce` devuelven valores nuevos, cuyo tipo adivina TypeScript;
  ordena los números con `sort((a, b) => a - b)`.
- `find` devuelve `T | undefined`: trata el caso con `if`, `?.` o `??`.

Siguiente: [Map, Set y Record](map-set.md).
