# Los objetos

Esta página explica cómo agrupar información en un objeto, leer y cambiar sus propiedades, recorrerlo, copiarlo, y cuándo elegir mejor un `Map` o un `Set`.

> **Objetivos:**
>
> - sabrás crear un objeto y leer sus propiedades con `.` y `[ ]`;
> - sabrás añadir, cambiar y borrar una propiedad;
> - sabrás escribir un método que usa `this`;
> - sabrás recorrer un objeto con `Object.keys`, `Object.values` y `Object.entries`;
> - sabrás copiar un objeto y elegir entre un objeto, un `Map` y un `Set`.

## Crear un objeto

Un [objeto](../debuter/glossaire.md#objeto) agrupa varios datos sobre una misma cosa. Es
como un documento de identidad: cada línea tiene una etiqueta (la **clave**) y un valor.
Un par clave-valor se llama **propiedad**.

```javascript
const gato = {
  nombre: "Félix",
  edad: 3,
  color: "pelirrojo",
  vacunado: true,
};

console.log(gato);
```

```sortie
{ nombre: 'Félix', edad: 3, color: 'pelirrojo', vacunado: true }
```

Línea por línea:

- `const gato = {` abre el objeto y le da un nombre: `gato`;
- las cuatro líneas siguientes son sus propiedades: la clave `nombre` vale
  `"Félix"`, la clave `edad` vale `3`, y así sucesivamente;
- `};` cierra el objeto;
- `console.log(gato)` muestra el objeto entero, con todas sus propiedades.

- El objeto se escribe entre llaves `{ }`.
- Cada propiedad se escribe `clave: valor`, y las propiedades se separan con comas.
- La coma después de la última propiedad está permitida: facilita añadir una línea.

Un array guarda valores **en orden**, por número. Un objeto los guarda **por nombre**. En
otros lenguajes se habla de [diccionario](../debuter/glossaire.md#diccionario).

## Leer una propiedad

Una ficha solo sirve si puedes consultarla: encontrar el nombre del gato, o su edad.
Leer una propiedad es mirar una línea concreta de la ficha.

Lees una propiedad con un punto `.` o con corchetes `[ ]`.

```javascript
const gato = { nombre: "Félix", edad: 3 };

console.log(gato.nombre);
console.log(gato["edad"]);
console.log(gato.peso);

const clave = "nombre";
console.log(gato[clave]);
```

```sortie
Félix
3
undefined
Félix
```

Línea por línea:

- `gato.nombre` busca el valor guardado bajo la clave `nombre`: `"Félix"`;
- `gato["edad"]` hace lo mismo con corchetes, y da `3`;
- `gato.peso` busca una clave que no existe: el resultado es `undefined`;
- `clave` contiene el texto `"nombre"`, así que `gato[clave]` equivale a escribir
  `gato["nombre"]`.

- El punto es lo más habitual: `gato.nombre`.
- Los corchetes toman un **texto**: útil cuando la clave está en una variable
  (`gato[clave]`) o contiene un espacio (`objeto["fecha de nacimiento"]`).
- Una propiedad que no existe da `undefined`, sin error.

## Añadir, cambiar, borrar

La información cambia con el tiempo: el gato cumple un año, descubres su color. Como
en una ficha escrita a lápiz, puedes corregir una línea, añadir una o borrar una.

Un objeto se puede cambiar después de crearlo, aunque esté en una `const` (como un
array: `const` solo impide reemplazar el objeto entero).

```javascript
const gato = { nombre: "Félix", edad: 3 };

gato.edad = 4;
gato.color = "pelirrojo";
delete gato.nombre;

console.log(gato);
console.log("color" in gato);
console.log("nombre" in gato);
```

```sortie
{ edad: 4, color: 'pelirrojo' }
true
false
```

Línea por línea:

- `gato.edad = 4;` reemplaza 3 por 4: la clave `edad` ya existía;
- `gato.color = "pelirrojo";` añade una línea a la ficha, porque `color` no existía;
- `delete gato.nombre;` borra la propiedad `nombre`;
- `"color" in gato` responde `true`, y `"nombre" in gato` responde `false`: `nombre` se
  ha borrado.

- Asignar una clave que existe la **cambia**; una clave nueva se **añade**.
- `delete` borra una propiedad.
- `"clave" in objeto` comprueba si una propiedad existe.

**Te toca:** escribe un programa que cree un objeto `libro` con un `titulo`
(«El principito») y un número de `paginas` (96). Después añade una propiedad `leido`
que valga `true`, y muestra el objeto.

> **Solución**
>
> ```javascript
> const libro = { titulo: "El principito", paginas: 96 };
> libro.leido = true;
> console.log(libro);
> ```
>
> ```sortie
> { titulo: 'El principito', paginas: 96, leido: true }
> ```

## Las propiedades abreviadas

Cuando una variable tiene el mismo nombre que la clave, basta con escribir el nombre una
vez.

```javascript
const nombre = "Alice";
const edad = 30;

const persona = { nombre, edad };
console.log(persona);
```

```sortie
{ nombre: 'Alice', edad: 30 }
```

`{ nombre, edad }` es una abreviatura de `{ nombre: nombre, edad: edad }`.

## Los métodos y this

Un objeto también puede hacer cosas, no solo guardar información. Una cuenta bancaria
tiene un saldo, pero también quieres depositar dinero en ella: un método guarda esa
acción en el objeto, junto a los datos que usa.

Una propiedad puede contener una función: entonces se llama
[método](../debuter/glossaire.md#metodo). Dentro, la palabra `this` («esto») designa el
objeto sobre el que se llama el método.

```javascript
const cuenta = {
  titular: "Alice",
  saldo: 100,
  depositar(cantidad) {
    this.saldo += cantidad;
  },
  resumen() {
    return `${this.titular} tiene ${this.saldo} €`;
  },
};

cuenta.depositar(50);
console.log(cuenta.resumen());
```

```sortie
Alice tiene 150 €
```

Línea por línea:

- `cuenta` tiene dos propiedades simples, `titular` y `saldo`, y dos métodos,
  `depositar` y `resumen`;
- `cuenta.depositar(50)` llama al método con `cantidad` igual a 50:
  `this.saldo += cantidad` hace pasar el saldo de 100 a 150;
- `cuenta.resumen()` construye un texto con `this.titular` y `this.saldo`;
- `console.log` muestra ese texto: «Alice tiene 150 €».

`depositar(cantidad) { ... }` es una abreviatura de
`depositar: function (cantidad) { ... }`. En `cuenta.depositar(50)`, `this` es el objeto
antes del punto: `cuenta`.

> **Atención:** no escribas un método con una flecha `=>` si usa `this`: una función
> flecha no tiene su propio `this`. Mantén la forma `nombre() { ... }`.

Los objetos que comparten los mismos métodos se fabrican con **clases**: eso será para
[la página sobre la herencia](heritage.md).

**Te toca:** escribe un programa que cree un objeto `contador` con una propiedad `valor`
que valga 0 y un método `aumentar()` que sume 1 a `valor`. Llama a `aumentar()` dos
veces, y luego muestra `contador.valor`.

> **Solución**
>
> ```javascript
> const contador = {
>   valor: 0,
>   aumentar() {
>     this.valor += 1;
>   },
> };
>
> contador.aumentar();
> contador.aumentar();
> console.log(contador.valor);
> ```
>
> ```sortie
> 2
> ```

## Recorrer un objeto

A veces no quieres leer una sola propiedad, sino todas: hacer el inventario de un
almacén, o mostrar cada línea de una ficha. Necesitas entonces la lista de lo que
contiene el objeto.

Tres funciones convierten un objeto en un array:

- `Object.keys(objeto)`: las claves;
- `Object.values(objeto)`: los valores;
- `Object.entries(objeto)`: pares `[clave, valor]`.

```javascript
const stock = { manzanas: 12, peras: 5, kiwis: 0 };

console.log(Object.keys(stock));
console.log(Object.values(stock));
console.log(Object.entries(stock));
```

```sortie
[ 'manzanas', 'peras', 'kiwis' ]
[ 12, 5, 0 ]
[ [ 'manzanas', 12 ], [ 'peras', 5 ], [ 'kiwis', 0 ] ]
```

Línea por línea:

- `stock` guarda tres frutas, cada una con su cantidad;
- `Object.keys(stock)` da el array de las claves: los nombres de las frutas;
- `Object.values(stock)` da el array de los valores: las cantidades;
- `Object.entries(stock)` da un array de pares: cada par contiene una clave y su valor.

Con `for...of` y una desestructuración, recorres claves y valores a la vez:

```javascript
const stock = { manzanas: 12, peras: 5, kiwis: 0 };

for (const [fruta, cantidad] of Object.entries(stock)) {
  console.log(`${fruta}: ${cantidad}`);
}

const total = Object.values(stock).reduce((s, n) => s + n, 0);
console.log(`Total: ${total}`);
```

```sortie
manzanas: 12
peras: 5
kiwis: 0
Total: 17
```

Las claves se recorren en el orden en que se añadieron (salvo las claves que son números
enteros, que van primero en orden creciente).

**Te toca:** escribe un programa que guarde el objeto `{ mates: 15, historia: 12 }` en
`notas`, y luego muestre cada asignatura y su nota con la forma «mates: 15», usando
`Object.entries` y `for...of`.

> **Solución**
>
> ```javascript
> const notas = { mates: 15, historia: 12 };
>
> for (const [asignatura, nota] of Object.entries(notas)) {
>   console.log(`${asignatura}: ${nota}`);
> }
> ```
>
> ```sortie
> mates: 15
> historia: 12
> ```

## La desestructuración de objetos

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

Como con los arrays, puedes guardar varias propiedades en variables de una vez. Aquí lo
que cuenta es el **nombre**, no la posición.

```javascript
const pelicula = { titulo: "Amélie", anio: 2001, duracion: 122 };

const { titulo, anio } = pelicula;
console.log(titulo, anio);

const { duracion: minutos, nota = "sin nota" } = pelicula;
console.log(minutos, nota);
```

```sortie
Amélie 2001
122 sin nota
```

- `duracion: minutos` lee la clave `duracion` y la guarda en una variable `minutos`.
- `nota = "sin nota"` da un valor por defecto si la clave no está.

Es muy práctico en los parámetros de una función:

```javascript
function presentar({ nombre, edad }) {
  return `${nombre}, ${edad} años`;
}

console.log(presentar({ nombre: "Alice", edad: 30 }));
```

```sortie
Alice, 30 años
```

## Copiar un objeto

Copiar sirve para cambiar una versión sin estropear el original, como fotocopiar un
formulario antes de rellenarlo.

Como los arrays, un objeto se comparte por **referencia**: `b = a` no copia nada, los
dos nombres designan el mismo objeto. Para copiar, usa el spread `...`.

```javascript
const original = { nombre: "Alice", edad: 30 };
const copia = { ...original };
copia.edad = 31;

const ampliado = { ...original, ciudad: "Lyon", edad: 40 };

console.log(original);
console.log(copia);
console.log(ampliado);
```

```sortie
{ nombre: 'Alice', edad: 30 }
{ nombre: 'Alice', edad: 31 }
{ nombre: 'Alice', edad: 40, ciudad: 'Lyon' }
```

Línea por línea:

- `copia` recibe un objeto nuevo, lleno con las propiedades de `original`;
- `copia.edad = 31` solo cambia la copia: `original` conserva 30;
- `ampliado` también copia `original`, añade `ciudad`, y reemplaza `edad` por 40;
- los tres `console.log` muestran tres objetos distintos.

En `ampliado`, las propiedades escritas **después** del spread reemplazan las del
original.

### Una copia superficial

El spread hace una copia **superficial**: solo copia el primer nivel. Un objeto o un
array guardado dentro sigue compartido.

```javascript
const original = { nombre: "Alice", aficiones: ["bici"] };
const copia = { ...original };
copia.aficiones.push("piano");
console.log(original.aficiones);

const profunda = structuredClone(original);
profunda.aficiones.push("canto");
console.log(original.aficiones);
console.log(profunda.aficiones);
```

```sortie
[ 'bici', 'piano' ]
[ 'bici', 'piano' ]
[ 'bici', 'piano', 'canto' ]
```

`structuredClone(objeto)` hace una copia **profunda**: se copian todos los niveles.

## Mostrar un objeto con JSON.stringify

En una plantilla, un objeto se muestra mal: `[object Object]`. `JSON.stringify` lo
convierte en un texto legible, en formato [JSON](../debuter/glossaire.md#json).

```javascript
const persona = { nombre: "Alice", edad: 30, idiomas: ["fr", "es"] };

console.log(`Persona: ${persona}`);
console.log(`Persona: ${JSON.stringify(persona)}`);
console.log(JSON.stringify(persona, null, 2));
```

```sortie
Persona: [object Object]
Persona: {"nombre":"Alice","edad":30,"idiomas":["fr","es"]}
{
  "nombre": "Alice",
  "edad": 30,
  "idiomas": [
    "fr",
    "es"
  ]
}
```

Línea por línea:

- el primer `console.log` pone el objeto tal cual en la plantilla: obtienes
  `[object Object]`, que no sirve de mucho;
- el segundo pasa antes por `JSON.stringify`: el objeto se convierte en un texto de una
  línea;
- el tercero añade `null, 2`: el texto se reparte en varias líneas, más fáciles de
  leer.

El tercer argumento (`2`) sangra el resultado con 2 espacios. El JSON también sirve para
guardar datos en un archivo: mira [los formatos de datos](formats.md).

## El encadenamiento opcional

> **Nota:** esta parte puede esperar. Léela cuando las bases estén firmes.

En objetos anidados, puede faltar una propiedad intermedia. `?.` se detiene limpiamente
y devuelve `undefined` en lugar de fallar; `??` da entonces un valor de reserva (mira
[las condiciones](conditions-boucles.md#los-valores-ausentes-y)).

```javascript
const clientes = [
  { nombre: "Alice", direccion: { ciudad: "Lyon" } },
  { nombre: "Bob" },
];

for (const cliente of clientes) {
  const ciudad = cliente.direccion?.ciudad ?? "desconocida";
  console.log(`${cliente.nombre}: ${ciudad}`);
}
```

```sortie
Alice: Lyon
Bob: desconocida
```

## Map y Set

Un objeto es perfecto para describir una cosa. Pero para contar palabras, o para
apuntar quién ya ha pasado, JavaScript ofrece dos herramientas más adecuadas: `Map` y
`Set`.

### Map: un diccionario a medida

Un `Map` también guarda pares clave-valor, con métodos propios: `set`, `get`, `has`,
`delete`, y `size` para el número de pares.

```javascript
const contador = new Map();

for (const palabra of ["gato", "perro", "gato", "gato"]) {
  contador.set(palabra, (contador.get(palabra) ?? 0) + 1);
}

console.log(contador);
console.log(contador.get("gato"));
console.log(contador.has("pez"));
console.log(contador.size);

for (const [palabra, n] of contador) {
  console.log(palabra, n);
}
```

```sortie
Map(2) { 'gato' => 3, 'perro' => 1 }
3
false
2
gato 3
perro 1
```

Línea por línea:

- `new Map()` crea un `Map` vacío, guardado en `contador`;
- el bucle lee las palabras una a una. `contador.get(palabra) ?? 0` da la cuenta actual
  de la palabra (0 si es nueva), y `set` guarda esa cuenta más 1;
- después del bucle, `"gato"` vale 3 y `"perro"` vale 1;
- `get`, `has` y `size` consultan el `Map`, y el último bucle muestra cada par.

### Set: un conjunto sin duplicados

Un `Set` es un [conjunto](../debuter/glossaire.md#conjunto): una colección de valores
**únicos**. Añadir un valor que ya está no hace nada.

```javascript
const visitantes = new Set();
visitantes.add("Alice");
visitantes.add("Bob");
visitantes.add("Alice");

console.log(visitantes);
console.log(visitantes.size);
console.log(visitantes.has("Bob"));

const sinDuplicados = [...new Set([3, 1, 3, 2, 1])];
console.log(sinDuplicados);
```

```sortie
Set(2) { 'Alice', 'Bob' }
2
true
[ 3, 1, 2 ]
```

`[...new Set(array)]` es el truco clásico para quitar los duplicados de un array.

**Te toca:** escribe un programa que cree un `Set` vacío llamado `colores`, le añada
`"rojo"`, luego `"azul"`, luego otra vez `"rojo"`, y muestre el número de colores.

> **Solución**
>
> ```javascript
> const colores = new Set();
> colores.add("rojo");
> colores.add("azul");
> colores.add("rojo");
> console.log(colores.size);
> ```
>
> ```sortie
> 2
> ```

### ¿Qué elegir?

| Necesidad | Elección |
| --- | --- |
| Describir **una cosa** con campos conocidos (nombre, edad…) | un objeto `{ }` |
| Claves que se añaden durante el programa (contar palabras, una caché) | un `Map` |
| Claves que no son textos (números, objetos) | un `Map` |
| Una lista de valores **sin duplicados**, y comprobar si uno está | un `Set` |
| Enviar o guardar en JSON | un objeto (JSON no conoce ni `Map` ni `Set`) |

## Errores frecuentes

### Olvidar una coma entre dos propiedades

```javascript norun
const gato = {
  nombre: "Félix",
  edad: 3
  color: "pelirrojo",
};
```

```text
file:///home/alice/js/main.mjs:4
  color: "pelirrojo",
  ^^^^^

SyntaxError: Unexpected identifier 'color'

Node.js v22.22.0
```

Falta la coma después de `edad: 3`. JavaScript no esperaba encontrar `color` justo
después (*unexpected identifier*: identificador inesperado). Añade la coma.

### Leer dentro de una propiedad que no existe

```javascript norun
const usuario = { nombre: "Alice" };
console.log(usuario.direccion.ciudad);
```

```text
file:///home/alice/js/main.mjs:2
console.log(usuario.direccion.ciudad);
                              ^

TypeError: Cannot read properties of undefined (reading 'ciudad')
    at file:///home/alice/js/main.mjs:2:31

Node.js v22.22.0
```

`usuario.direccion` vale `undefined`, y no se puede leer nada dentro de `undefined`.
Usa el encadenamiento opcional: `usuario.direccion?.ciudad`.

### Usar el punto con una clave guardada en una variable

```javascript norun
const notas = { mates: 15, lengua: 12 };
const asignatura = "mates";
console.log(notas.asignatura);
```

```text
undefined
```

`notas.asignatura` busca una clave que se llama literalmente `"asignatura"`. Para usar el
**contenido** de la variable, escribe `notas[asignatura]`.

### Mostrar un objeto en una plantilla

```javascript norun
const cesta = { manzanas: 3 };
console.log(`Cesta: ${cesta}`);
```

```text
Cesta: [object Object]
```

Una plantilla convierte el objeto en texto de la forma más pobre. Escribe
`${JSON.stringify(cesta)}`, o pasa el objeto aparte: `console.log("Cesta:", cesta)`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```javascript norun
const p = { x: 1, y: 2 };
const q = p;
q.x = 10;
console.log(p.x);
```

> **Respuesta**
>
> `10`. `q` y `p` designan el mismo objeto: cambiar uno cambia el otro. Para hacer una
> copia, escribe `const q = { ...p };`.

**Pregunta 2:** ¿cómo muestras cada clave y cada valor de `{ a: 1, b: 2 }`?

> **Respuesta**
>
> ```javascript
> for (const [clave, valor] of Object.entries({ a: 1, b: 2 })) {
>   console.log(clave, valor);
> }
> ```
>
> ```sortie
> a 1
> b 2
> ```

**Pregunta 3:** quieres la lista de las ciudades visitadas, sin duplicados. ¿Objeto,
`Map` o `Set`?

> **Respuesta**
>
> Un `Set`: guarda cada valor una sola vez, y `has(ciudad)` dice si una ciudad ya está en
> la lista.

## En resumen

- Un objeto `{ clave: valor }` agrupa propiedades; se leen con `objeto.clave` u
  `objeto["clave"]`.
- Se añade o se cambia con una asignación, se borra con `delete`; `{ nombre, edad }` es
  una abreviatura.
- Un método es una función dentro de un objeto; `this` designa el objeto antes del punto.
- `Object.keys`, `Object.values` y `Object.entries` recorren un objeto;
  `const { a, b } = objeto` lo desestructura.
- `{ ...objeto }` hace una copia superficial, `structuredClone` una copia profunda;
  `JSON.stringify` lo muestra de forma legible.
- `Map` para claves que se añaden durante el programa, `Set` para valores únicos.

Siguiente: [seguir un programa paso a paso en JavaScript](pas-a-pas.md).
