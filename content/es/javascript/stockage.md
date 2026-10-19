# Guardar datos en el navegador

Esta página muestra cómo una página web guarda datos de una visita a otra con `localStorage`, `sessionStorage` e IndexedDB, y lo que nunca hay que guardar ahí.

> **Objetivos:**
>
> - sabrás guardar, releer y borrar datos con `localStorage`;
> - sabrás guardar objetos y listas gracias a `JSON.stringify` y `JSON.parse`;
> - sabrás elegir entre `localStorage` y `sessionStorage`;
> - conocerás los límites del almacenamiento y el papel de IndexedDB;
> - sabrás lo que **nunca** hay que guardar en el navegador.

## Por qué guardar datos

Las [variables](../debuter/glossaire.md#variable) de una página están escritas en una
**pizarra blanca**: cuando recargas la página, todo se borra. El contador vuelve a 0,
la lista de tareas está vacía.

El navegador también ofrece un **cuaderno**: lo que escribes en él se queda, incluso
después de cerrar el navegador y volver a encender el ordenador.

```text
  Variables (pizarra)                localStorage (cuaderno)
  ┌──────────────────────┐           ┌──────────────────────┐
  │ contador = 3         │           │ "contador" → "3"     │
  └──────────────────────┘           └──────────────────────┘
        recarga ▼                          recarga ▼
  ┌──────────────────────┐           ┌──────────────────────┐
  │ contador = 0 perdido │           │ "contador" → "3"     │
  └──────────────────────┘           └──────────────────────┘
```

El capítulo [el DOM](dom.md) dio un adelanto. Aquí nos tomamos el tiempo de verlo todo.

> **Recuerda:** las variables desaparecen al recargar. El almacenamiento del navegador,
> en cambio, se queda.

## localStorage: casilleros con etiqueta

`localStorage` se parece a una pared de casilleros. Cada casillero tiene una
**etiqueta** (la clave) y contiene **un texto** (el valor).

```text
  ┌────────────┬────────────┬────────────┐
  │ "tema"     │ "nombre"   │ "visitas"  │   ← las claves (etiquetas)
  │  "oscuro"  │  "Léa"     │  "4"       │   ← los valores (siempre texto)
  └────────────┴────────────┴────────────┘
```

Bastan cuatro métodos:

```javascript norun
localStorage.setItem("tema", "oscuro");       // guardar (o reemplazar)
const tema = localStorage.getItem("tema");    // releer: "oscuro"
localStorage.removeItem("tema");              // vaciar un casillero
localStorage.clear();                         // vaciar todos los casilleros
```

- `getItem` devuelve `null` si el casillero no existe;
- `setItem` sobre una clave existente **reemplaza** el valor anterior;
- `localStorage.length` da el número de casilleros.

Este código solo funciona en un navegador: Node no tiene `localStorage`. Para probarlo
en el navegador, abre la consola (`F12`, pestaña **Consola**) y escribe estas líneas una
a una. La pestaña **Aplicación** (Chrome) o **Almacenamiento** (Firefox) muestra los
casilleros.

> **Recuerda:** `setItem(clave, texto)` guarda, `getItem(clave)` relee (o `null`),
> `removeItem(clave)` borra.

## Probar la API en Node

Para entender y probar sin navegador, fabriquemos un `localStorage` **falso**: una
[clase](../debuter/glossaire.md#classe) que tiene los mismos métodos. Lo guarda todo en
un `Map` (mira [las colecciones](collections.md)).

```javascript
class AlmacenFalso {
  #casilleros = new Map();

  get length() {
    return this.#casilleros.size;
  }
  setItem(clave, valor) {
    this.#casilleros.set(String(clave), String(valor)); // todo se vuelve texto
  }
  getItem(clave) {
    return this.#casilleros.has(clave) ? this.#casilleros.get(clave) : null;
  }
  removeItem(clave) {
    this.#casilleros.delete(clave);
  }
  clear() {
    this.#casilleros.clear();
  }
  key(i) {
    return [...this.#casilleros.keys()][i] ?? null;
  }
}

const cuaderno = new AlmacenFalso();
cuaderno.setItem("tema", "claro");
cuaderno.setItem("nombre", "Léa");
cuaderno.setItem("tema", "oscuro");
console.log(cuaderno.getItem("tema"), cuaderno.length);
cuaderno.removeItem("nombre");
console.log(cuaderno.getItem("nombre"), cuaderno.length);
console.log(cuaderno.key(0));
```

```sortie
oscuro 2
null 1
tema
```

Sigamos el contenido del cuaderno línea a línea:

| Paso | Instrucción | Contenido del cuaderno | length |
|---|---|---|---|
| 1 | `setItem("tema", "claro")` | tema → "claro" | 1 |
| 2 | `setItem("nombre", "Léa")` | tema → "claro", nombre → "Léa" | 2 |
| 3 | `setItem("tema", "oscuro")` | tema → "oscuro", nombre → "Léa" | 2 |
| 4 | `removeItem("nombre")` | tema → "oscuro" | 1 |

En el paso 3, `length` se queda en 2: la clave `tema` ya existía, su valor se reemplaza.

Prueba: añade `cuaderno.clear();` y luego `console.log(cuaderno.length, cuaderno.key(0));`.
Obtienes `0 null`.

> **Recuerda:** el verdadero `localStorage` se comporta como esta pequeña clase: una
> tabla de claves y textos.

## Todo se vuelve texto

El cuaderno solo guarda [cadenas de caracteres](../debuter/glossaire.md#chaine-de-caracteres).
Si guardas un número, se convierte en texto. A la vuelta, ya no es un número.

```javascript
// la misma conversión que setItem: String(valor)
const guardado = String(5);
console.log(guardado, typeof guardado);
console.log(guardado + 1);          // ¡textos pegados!
console.log(Number(guardado) + 1);  // el cálculo correcto

console.log(String(true));
console.log(String([1, 2, 3]));
console.log(String({ nombre: "Léa" }));
```

```sortie
5 string
51
6
true
1,2,3
[object Object]
```

| Valor guardado | Texto conservado | Problema |
|---|---|---|
| `5` | `"5"` | `"5" + 1` da `"51"` |
| `true` | `"true"` | `"true"` no es `true` |
| `[1, 2, 3]` | `"1,2,3"` | ya no es un array |
| `{ nombre: "Léa" }` | `"[object Object]"` | los datos se **pierden** |

Para un número, conviértelo a la vuelta con `Number(...)`. Para todo lo demás, usa JSON
(sección siguiente).

> **Recuerda:** lo que entra en el cuaderno sale en forma de texto.

## Guardar objetos con JSON

[JSON](../debuter/glossaire.md#json) es una forma de escribir un objeto o una lista como
texto, y luego volver a leerlo. Es como desmontar un mueble para meterlo en una caja, y
volver a montarlo al llegar (se habla de
[serialización](../debuter/glossaire.md#serialisation)).

- `JSON.stringify(valor)`: objeto → texto (lo metemos en la caja);
- `JSON.parse(texto)`: texto → objeto (lo volvemos a montar).

```javascript
const ajustes = { tema: "oscuro", tamano: 18, sonidos: false };

const texto = JSON.stringify(ajustes);
console.log(texto, typeof texto);

const releido = JSON.parse(texto);
console.log(releido.tamano + 2, typeof releido.sonidos);
```

```sortie
{"tema":"oscuro","tamano":18,"sonidos":false} string
20 boolean
```

Los números y los booleanos vuelven intactos. Mira [JSON y CSV](formats.md) para más
detalles.

### Dos funciones, guardar y cargar

Escribe una vez dos pequeñas funciones y úsalas en todas partes. `cargar` recibe un
**valor por defecto** para el caso en que el casillero esté vacío, o dañado.

```javascript
class AlmacenFalso {
  #casilleros = new Map();
  setItem(clave, valor) { this.#casilleros.set(clave, String(valor)); }
  getItem(clave) { return this.#casilleros.get(clave) ?? null; }
}
const cuaderno = new AlmacenFalso();

function guardar(clave, valor) {
  cuaderno.setItem(clave, JSON.stringify(valor));
}

function cargar(clave, porDefecto) {
  const texto = cuaderno.getItem(clave);
  if (texto === null) return porDefecto;   // casillero vacío
  try {
    return JSON.parse(texto);
  } catch {
    return porDefecto;                     // texto dañado
  }
}

console.log(cargar("tareas", []));
guardar("tareas", ["pan", "leche"]);
console.log(cargar("tareas", []));
cuaderno.setItem("tareas", "{no es json");
console.log(cargar("tareas", []));
```

```sortie
[]
[ 'pan', 'leche' ]
[]
```

| Llamada | `getItem` devuelve | Camino seguido | Resultado |
|---|---|---|---|
| 1.ª `cargar` | `null` | casillero vacío | `[]` (por defecto) |
| 2.ª `cargar` | `'["pan","leche"]'` | `JSON.parse` funciona | `[ 'pan', 'leche' ]` |
| 3.ª `cargar` | `'{no es json'` | `JSON.parse` falla, `catch` | `[]` (por defecto) |

En el navegador, basta con reemplazar `cuaderno` por `localStorage`.

> **Recuerda:** `JSON.stringify` al guardar, `JSON.parse` al releer, y siempre un valor
> por defecto.

## Un ejemplo: contar las visitas

Simulemos tres visitas a una página. En cada visita, las variables vuelven a empezar de
cero (nueva llamada a la función), pero el cuaderno es el mismo.

```javascript
class AlmacenFalso {
  #casilleros = new Map();
  setItem(clave, valor) { this.#casilleros.set(clave, String(valor)); }
  getItem(clave) { return this.#casilleros.get(clave) ?? null; }
}
const cuaderno = new AlmacenFalso();  // sobrevive a las visitas

function visita() {
  const antes = Number(cuaderno.getItem("visitas") ?? "0");
  const visitas = antes + 1;
  cuaderno.setItem("visitas", visitas);
  console.log(visitas === 1 ? "¡Te damos la bienvenida!" : `Visita número ${visitas}`);
}

visita();
visita();
visita();
```

```sortie
¡Te damos la bienvenida!
Visita número 2
Visita número 3
```

| Visita | `getItem("visitas")` | antes | visitas | Cuaderno después |
|---|---|---|---|---|
| 1 | `null` → `"0"` | 0 | 1 | "1" |
| 2 | `"1"` | 1 | 2 | "2" |
| 3 | `"2"` | 2 | 3 | "3" |

Prueba: quita `Number(...)` (deja `cuaderno.getItem("visitas") ?? "0"`). La primera
visita muestra `Visita número 01`, la tercera `Visita número 0111`: se pegan textos en
lugar de sumar.

> **Recuerda:** lee, calcula, vuelve a escribir. Y convierte el texto antes de calcular.

## sessionStorage: el cuaderno de una sola pestaña

`sessionStorage` tiene **exactamente** los mismos métodos. La diferencia es su duración:
se borra cuando cierras la pestaña. Es una nota adhesiva pegada en la pestaña, mientras
que `localStorage` es un cuaderno guardado en el cajón.

```javascript norun
sessionStorage.setItem("paso", "2");         // el paso de un formulario en curso
console.log(sessionStorage.getItem("paso")); // "2", incluso después de recargar
```

| | `localStorage` | `sessionStorage` |
|---|---|---|
| Sobrevive a la recarga | sí | sí |
| Sobrevive al cierre de la pestaña | sí | **no** |
| Compartido entre pestañas del mismo sitio | sí | no (un cuaderno por pestaña) |
| Uso típico | tema, idioma, borrador | paso de un formulario, filtro |

Las dos funciones `guardar` y `cargar` funcionan tanto con uno como con el otro.

> **Recuerda:** la misma API; `sessionStorage` vive lo que dura una pestaña,
> `localStorage` se queda.

## Los límites

El cuaderno es práctico, pero pequeño y sencillo.

- **El tamaño**: unos 5 MB por sitio, según el navegador. Por encima, `setItem` lanza
  un error.
- **Solo texto**, de ahí JSON.
- **Un cuaderno por sitio**: un sitio no ve el cuaderno de otro. «Sitio» quiere decir
  aquí el protocolo, el nombre y el puerto: `http://localhost:3000` y
  `http://localhost:5173` tienen dos cuadernos distintos.
- **El usuario decide**: puede borrarlo todo («borrar datos de navegación»), y la
  navegación privada lo vacía todo al cerrarse.
- **Es síncrono**: la página espera durante la lectura. Con textos grandes, puede ir
  más lenta.

Para estimar el espacio ocupado, mide la longitud del texto JSON:

```javascript
const tareas = Array.from({ length: 1000 }, (_, i) => ({ texto: `Tarea ${i}`, hecha: false }));
const texto = JSON.stringify(tareas);
console.log(texto.length, "caracteres");
console.log((texto.length / 5_000_000 * 100).toFixed(1), "% de 5 millones");
```

```sortie
35891 caracteres
0.7 % de 5 millones
```

Mil tareas caben de sobra. Unas fotos, no.

Cuando el cuaderno está lleno, el navegador lanza un error (aquí en Chrome):

```text
Uncaught DOMException: Failed to execute 'setItem' on 'Storage': Setting the value of 'fotos' exceeded the quota.
```

> **Recuerda:** `localStorage` sirve para datos pequeños (ajustes, borradores, listas
> cortas). Para muchos datos, usa IndexedDB.

## IndexedDB: una verdadera base pequeña

IndexedDB es una **base de datos** en el navegador (mira
[las bases de datos](../debuter/bases-de-donnees.md)). Guarda **objetos** (no solo
texto), en gran cantidad, y no bloquea la página. A cambio, su API es más larga de
escribir. Es el almacén, al lado del pequeño cuaderno.

```text
 base "notas"
 └── almacén "tareas"   (como una tabla)
      ├── { id: 1, texto: "pan", hecha: false }
      └── { id: 2, texto: "leche", hecha: true }
```

### Abrir la base

La base se abre por su nombre y su **versión**. La primera vez (o cuando la versión
aumenta), el evento `upgradeneeded` permite crear los **almacenes**.

```javascript norun
function abrir() {
  return new Promise((resolve, reject) => {
    const peticion = indexedDB.open("notas", 1);       // nombre, versión

    peticion.onupgradeneeded = () => {
      const base = peticion.result;
      base.createObjectStore("tareas", { keyPath: "id" }); // la clave es el campo id
    };
    peticion.onsuccess = () => resolve(peticion.result);
    peticion.onerror = () => reject(peticion.error);
  });
}
```

IndexedDB avisa mediante **eventos** (`onsuccess`, `onerror`). Lo envolvemos en una
[promesa](../debuter/glossaire.md#promesse) para poder escribir `await` (mira
[la asincronía](asynchrone.md)).

### Añadir y leer

Cada lectura o escritura pasa por una **transacción**: un pequeño pedido que se cumple
entero o nada en absoluto.

```javascript norun
function registrar(base, tarea) {
  return new Promise((resolve, reject) => {
    const tx = base.transaction("tareas", "readwrite");
    tx.objectStore("tareas").put(tarea);       // añade o reemplaza
    tx.oncomplete = () => resolve();
    tx.onerror = () => reject(tx.error);
  });
}

function leer(base, id) {
  return new Promise((resolve, reject) => {
    const peticion = base.transaction("tareas").objectStore("tareas").get(id);
    peticion.onsuccess = () => resolve(peticion.result);  // undefined si no existe
    peticion.onerror = () => reject(peticion.error);
  });
}

const base = await abrir();
await registrar(base, { id: 1, texto: "pan", hecha: false });
console.log(await leer(base, 1));   // { id: 1, texto: "pan", hecha: false }
```

Fíjate: aquí no hay `JSON.stringify`. IndexedDB guarda los objetos tal cual.

> **Recuerda:** IndexedDB = muchos datos, objetos, sin bloquear la página. Más código:
> envuélvelo en unas pocas funciones con promesas.

## Lo que nunca hay que guardar ahí

El almacenamiento del navegador es una **postal**, no una caja fuerte:

- cualquier script que se ejecute en la página puede leerlo, incluido un script
  malicioso (un ataque XSS, mira [la seguridad](../debuter/securite.md));
- cualquier persona que use el ordenador puede abrirlo con `F12`;
- nada está cifrado.

**Nunca** guardes ahí:

- una contraseña, ni siquiera «temporalmente»;
- un número de tarjeta bancaria;
- un token de sesión que da acceso a una cuenta (mejor: una cookie `HttpOnly`, puesta
  por el [servidor](../debuter/glossaire.md#serveur), que JavaScript no puede leer);
- datos personales sensibles (salud, documentos de identidad).

Guarda ahí sin miedo: el tema, el idioma, un borrador, el estado de un juego, una lista
de la compra.

> **Recuerda:** si te daría vergüenza verlo escrito en una postal, no lo pongas en
> `localStorage`.

## Errores frecuentes

### Releer un casillero vacío

```javascript norun
const tareas = JSON.parse(localStorage.getItem("tareas"));
console.log(tareas.length);
```

```text
Uncaught TypeError: Cannot read properties of null (reading 'length')
```

En la primera visita, el casillero no existe: `getItem` devuelve `null`, y
`JSON.parse(null)` da `null`. Prevé un valor por defecto:
`JSON.parse(localStorage.getItem("tareas") ?? "[]")`, o la función `cargar`.

### Guardar texto que no es JSON

```javascript norun
localStorage.setItem("ajustes", "{tema: oscuro}");   // escrito a mano
const ajustes = JSON.parse(localStorage.getItem("ajustes"));
```

```text
Uncaught SyntaxError: Expected property name or '}' in JSON at position 1 (line 1 column 2)
```

En JSON, las claves y los textos van entre comillas dobles. No fabriques el texto a
mano: pasa siempre por `JSON.stringify`, y rodea `JSON.parse` con un `try...catch`.

### Calcular con un número releído

```javascript norun
localStorage.setItem("puntos", 10);
const puntos = localStorage.getItem("puntos") + 5;
console.log(puntos);
```

```text
105
```

No hay error, pero sí un resultado falso: `"10" + 5` pega los textos. Escribe
`Number(localStorage.getItem("puntos")) + 5`.

## Ponte a prueba

**Pregunta 1:** ¿qué devuelve `localStorage.getItem("ausente")` si ese casillero nunca
se ha llenado?

> **Respuesta**
>
> `null`. Hay que prever este caso, por ejemplo con `?? "[]"` antes de `JSON.parse`.

**Pregunta 2:** un usuario cierra su pestaña y luego vuelve a abrir el sitio. ¿Qué
queda en `localStorage`? ¿Y en `sessionStorage`?

> **Respuesta**
>
> `localStorage` lo ha guardado todo. `sessionStorage` está vacío: solo vive lo que
> dura una pestaña.

**Pregunta 3:** con el `AlmacenFalso` (o un `Map`), guarda la lista `[3, 1, 2]` con la
clave `"notas"`, reléela y muestra su suma.

> **Respuesta**
>
> ```javascript
> const cuaderno = new Map();
> cuaderno.set("notas", JSON.stringify([3, 1, 2]));
>
> const notas = JSON.parse(cuaderno.get("notas") ?? "[]");
> console.log(notas.reduce((a, b) => a + b, 0));
> ```
>
> ```sortie
> 6
> ```

## En resumen

- `localStorage` guarda textos por clave, incluso después de cerrar: `setItem`, `getItem`, `removeItem`.
- Todo se vuelve texto: `JSON.stringify` para guardar, `JSON.parse` para releer, `Number` para un número.
- `getItem` devuelve `null` para un casillero vacío: prevé un valor por defecto.
- `sessionStorage` tiene la misma API pero vive lo que dura una pestaña.
- Para muchos datos u objetos, IndexedDB; nunca un secreto en el navegador.

Siguiente: [leer y escribir archivos con Node](fichiers.md)
