# La programación asíncrona

Esta página explica cómo JavaScript hace varias cosas «a la vez» sin quedarse esperando sin hacer nada: el bucle de eventos, los callbacks, las promesas y `async`/`await`.

> **Objetivos:**
>
> - entenderás el principio del bucle de eventos;
> - sabrás prever el orden de lo que se muestra con `setTimeout`;
> - sabrás crear y usar una promesa con `then` y `catch`;
> - sabrás escribir código asíncrono legible con `async` y `await`;
> - sabrás lanzar varias tareas con `Promise.all`, `allSettled` y `race`.

## Esperar sin bloquear

Algunas operaciones son **lentas**: descargar una página web, leer un archivo grande,
consultar una base de datos. Si el programa se quedara congelado durante ese tiempo, no
podría pasar nada más: una página web dejaría de responder a los clics.

JavaScript hace **una sola cosa a la vez**. Pero nunca se queda parado esperando: lanza
la operación lenta, sigue con lo demás y vuelve cuando el resultado está listo. Es la
programación [asíncrona](../debuter/glossaire.md#async).

## El bucle de eventos

Imagina un camarero solo en un restaurante. Toma el pedido de la mesa 1, lo pasa a la
cocina y **no espera** delante de los fogones: va a tomar el pedido de la mesa 2. Cuando
la cocina toca el timbre, lleva el plato.

JavaScript funciona así:

1. ejecuta el código del programa, línea tras línea, hasta el final;
2. mientras tanto, las operaciones lentas (temporizadores, red, archivos) se hacen «en
   la cocina»;
3. cuando una operación termina, su continuación se coloca en una **cola de espera**;
4. en cuanto está libre, JavaScript toma la siguiente tarea de la cola y vuelve a
   empezar.

Esta vuelta sin fin se llama **bucle de eventos** (*event loop*). El programa termina
cuando ya no hay nada que ejecutar ni que esperar.

## setTimeout: programar para más tarde

`setTimeout(funcion, retraso)` pide ejecutar una función **más tarde**, tras un retraso
en milisegundos (1000 ms = 1 segundo). La función que le das se llama **callback**
(«devolución de llamada»): ya te llamarán.

```javascript
console.log("1. Pido");

setTimeout(() => {
  console.log("3. Llega el plato");
}, 100);

console.log("2. Charlo mientras espero");
```

```sortie
1. Pido
2. Charlo mientras espero
3. Llega el plato
```

`setTimeout` no bloquea: anota la petición y el programa sigue enseguida.

### Incluso con un retraso de 0

Un callback siempre espera a que el código en curso haya terminado **por completo**,
incluso con un retraso de 0: pasa por la cola de espera.

```javascript
setTimeout(() => console.log("C (retraso 200)"), 200);
setTimeout(() => console.log("B (retraso 0)"), 0);
setTimeout(() => console.log("D (retraso 300)"), 300);
console.log("A (enseguida)");
```

```sortie
A (enseguida)
B (retraso 0)
C (retraso 200)
D (retraso 300)
```

El orden en el código no cuenta: decide el retraso. Y el código «normal» siempre va
primero.

## Los callbacks y sus límites

Durante mucho tiempo, todas las operaciones lentas recibían un callback: «cuando
termines, llama a esta función». Simulemos una base de datos lenta.

```javascript
function cargarUsuario(id, callback) {
  setTimeout(() => callback({ id, nombre: "Ada" }), 50);
}

function cargarPedidos(usuario, callback) {
  setTimeout(() => callback(["libro", "bolígrafo"]), 50);
}

cargarUsuario(1, (usuario) => {
  console.log("Usuario:", usuario.nombre);
  cargarPedidos(usuario, (pedidos) => {
    console.log("Pedidos:", pedidos.join(", "));
  });
});
```

```sortie
Usuario: Ada
Pedidos: libro, bolígrafo
```

Funciona. Pero cada paso añade un nivel de sangría. Con cinco pasos y la gestión de
errores en cada nivel, el código se convierte en una «pirámide» ilegible, que se llama
el **infierno de los callbacks**. Las promesas resuelven este problema.

## Las promesas

Una **promesa** (`Promise`) es un objeto que representa un resultado **todavía no
disponible**. Es el tique que te dan en el mostrador: por ahora no hay bocadillo, pero el
tique promete que llegará… o que te dirán por qué no llegará.

Una promesa está en uno de tres estados:

| Estado | Significado |
|------|------|
| pendiente (*pending*) | el resultado todavía no está |
| cumplida (*fulfilled*) | el resultado ha llegado |
| rechazada (*rejected*) | se ha producido un error |

### Crear una promesa

Una promesa se crea con `new Promise`. Se le da una función que recibe dos herramientas:
`resolve` (cumplir la promesa, con un resultado) y `reject` (rechazarla, con un error).

```javascript
function lanzarDado(valor) {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      if (valor >= 1 && valor <= 6) {
        resolve(valor);
      } else {
        reject(new Error(`cara imposible: ${valor}`));
      }
    }, 50);
  });
}

console.log(lanzarDado(4));
```

```sortie
Promise { <pending> }
```

Justo después de crearla, la promesa está pendiente: el dado todavía está rodando.

### then y catch

`.then(funcion)` dice qué hacer **cuando** la promesa se cumple: la función recibe el
resultado. `.catch(funcion)` dice qué hacer si se rechaza: la función recibe el error.
`.finally(funcion)` se ejecuta en los dos casos. Con la misma función `lanzarDado`:

```javascript norun
lanzarDado(4)
  .then((cara) => console.log("Resultado:", cara))
  .catch((error) => console.log("Error:", error.message));

lanzarDado(9)
  .then((cara) => console.log("Resultado:", cara))
  .catch((error) => console.log("Error:", error.message))
  .finally(() => console.log("Tirada terminada"));
```

```text
Resultado: 4
Error: cara imposible: 9
Tirada terminada
```

Cada `then` devuelve una nueva promesa: puedes **encadenarlos** en lugar de anidarlos.
El `catch` final atrapa un error ocurrido en cualquier paso.

```javascript
const duplicar = (n) => new Promise((resolve) => setTimeout(() => resolve(n * 2), 20));

duplicar(5)
  .then((n) => duplicar(n))
  .then((n) => duplicar(n))
  .then((n) => console.log("Resultado final:", n))
  .catch((error) => console.log(error.message));
```

```sortie
Resultado final: 40
```

## async y await

`async` y `await` permiten escribir código asíncrono **como código normal**, de arriba
abajo. Es la forma recomendada hoy en día.

- `async` delante de una función la hace asíncrona: siempre devuelve una promesa.
- `await` delante de una promesa **pone la función en pausa** hasta tener el resultado,
  sin bloquear el resto del programa.

Este es el ejemplo de los callbacks, reescrito:

```javascript
const esperar = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function cargarUsuario(id) {
  await esperar(50);
  return { id, nombre: "Ada" };
}

async function cargarPedidos(usuario) {
  await esperar(50);
  return ["libro", "bolígrafo"];
}

async function mostrarPedidos() {
  const usuario = await cargarUsuario(1);
  console.log("Usuario:", usuario.nombre);
  const pedidos = await cargarPedidos(usuario);
  console.log("Pedidos:", pedidos.join(", "));
}

await mostrarPedidos();
console.log("Terminado");
```

```sortie
Usuario: Ada
Pedidos: libro, bolígrafo
Terminado
```

Se acabó la pirámide: cada paso es una línea. En un módulo (archivo `.mjs`, o
`"type": "module"`), también puedes escribir `await` **directamente** en el nivel
principal del archivo, como en la penúltima línea.

> **Recuerda:** `await` solo se usa dentro de una función `async`, o en el nivel
> principal de un módulo.

## Gestionar los errores con try y catch

Con `await`, una promesa rechazada **lanza un error**, como `throw`. Por eso se atrapa
con `try` y `catch`, exactamente como en [gestionar los errores](erreurs.md).

```javascript
const esperar = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function cargarPerfil(nombre) {
  await esperar(30);
  if (nombre === "") {
    throw new Error("nombre vacío");
  }
  return { nombre, nivel: 3 };
}

async function mostrar(nombre) {
  try {
    const perfil = await cargarPerfil(nombre);
    console.log(`${perfil.nombre} está en el nivel ${perfil.nivel}`);
  } catch (error) {
    console.log("Imposible:", error.message);
  } finally {
    console.log("--");
  }
}

await mostrar("Ada");
await mostrar("");
```

```sortie
Ada está en el nivel 3
--
Imposible: nombre vacío
--
```

## Varias tareas a la vez

Con `await` una tras otra, las tareas se suceden: 3 tareas de 100 ms tardan 300 ms. Si
no dependen unas de otras, puedes lanzarlas **todas a la vez**.

### Promise.all: todo o nada

`Promise.all(lista)` espera a que **todas** las promesas se cumplan y da la lista de
resultados, **en el orden de la lista** (no en el orden de llegada). Si una sola se
rechaza, todo se rechaza.

```javascript
const esperar = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function descargar(nombre, ms) {
  await esperar(ms);
  console.log(`  ${nombre} recibido`);
  return `${nombre} (${ms} ms)`;
}

const resultados = await Promise.all([
  descargar("imagen", 150),
  descargar("texto", 50),
  descargar("sonido", 100),
]);
console.log(resultados);
```

```sortie
  texto recibido
  sonido recibido
  imagen recibido
[ 'imagen (150 ms)', 'texto (50 ms)', 'sonido (100 ms)' ]
```

Las tres descargas se hacen en paralelo: el total tarda unos 150 ms, el tiempo de la más
lenta.

### Promise.allSettled: el balance de cada una

`Promise.allSettled(lista)` espera a que todas terminen, cumplidas **o** rechazadas, y da
un balance de cada una. Un error no detiene a las demás.

```javascript
const esperar = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function comprobar(sitio) {
  await esperar(20);
  if (sitio.includes("caido")) {
    throw new Error(`${sitio} no responde`);
  }
  return `${sitio} OK`;
}

const balances = await Promise.allSettled([
  comprobar("ejemplo.es"),
  comprobar("caido.es"),
  comprobar("demo.es"),
]);

for (const balance of balances) {
  if (balance.status === "fulfilled") {
    console.log("✓", balance.value);
  } else {
    console.log("✗", balance.reason.message);
  }
}
```

```sortie
✓ ejemplo.es OK
✗ caido.es no responde
✓ demo.es OK
```

### Promise.race: el primero que llega

`Promise.race(lista)` da el resultado de la **primera** promesa que termina. Se usa a
menudo para fijar un límite de tiempo.

```javascript
const esperar = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function respuestaServidor(ms) {
  await esperar(ms);
  return `respuesta en ${ms} ms`;
}

async function tiempoAgotado(ms) {
  await esperar(ms);
  throw new Error(`demasiado largo (más de ${ms} ms)`);
}

for (const duracion of [50, 300]) {
  try {
    const respuesta = await Promise.race([respuestaServidor(duracion), tiempoAgotado(150)]);
    console.log(respuesta);
  } catch (error) {
    console.log("Abandono:", error.message);
  }
}
```

```sortie
respuesta en 50 ms
Abandono: demasiado largo (más de 150 ms)
```

| Función | Espera | Resultado | Si hay un error |
|----------|--------|----------|---------------|
| `Promise.all` | todas | la lista de valores | todo se rechaza |
| `Promise.allSettled` | todas | un balance por promesa | anotado en el balance |
| `Promise.race` | la primera | su valor | si la primera falla |

## Unas palabras sobre fetch

`fetch(direccion)` descarga un recurso de la web. Es el ejemplo más común de operación
asíncrona: devuelve una promesa. Está disponible en Node y en todos los navegadores.

```javascript norun
try {
  const respuesta = await fetch("https://registry.npmjs.org/slugify/1.6.6");
  if (!respuesta.ok) {
    throw new Error(`HTTP ${respuesta.status}`);
  }
  const paquete = await respuesta.json();
  console.log(`${paquete.name} ${paquete.version}: ${paquete.description}`);
} catch (error) {
  console.log("Fallo:", error.message);
}
```

```text
slugify 1.6.6: Slugifies a String
```

Este código pide a npm la ficha del paquete `slugify` (ver [módulos y npm](modules.md)).
Si el servidor responde con un error, `respuesta.ok` vale `false`: con una versión que no
existe, el programa muestra `Fallo: HTTP 404`.

Dos `await`: uno para la respuesta del servidor y otro para leer su contenido en
[JSON](../debuter/glossaire.md#json). El capítulo sobre [la web](web.md) va más lejos.

## Errores frecuentes

### Olvidar await

```javascript norun
async function cargarNombre() {
  return "Ada";
}
const nombre = cargarNombre();
console.log(`Hola ${nombre}`);
console.log(nombre);
```

```text
Hola [object Promise]
Promise { 'Ada' }
```

No hay error, pero el resultado es falso: una función `async` devuelve una **promesa**,
no el valor. Escribe `const nombre = await cargarNombre();`.

### Usar await fuera de una función async

```javascript norun
function esperar(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
function empezar() {
  await esperar(100);
  console.log("Listo");
}
empezar();
```

```text
file:///home/vous/projet/main.mjs:5
  await esperar(100);
  ^^^^^

SyntaxError: Unexpected reserved word
```

«Palabra reservada inesperada»: `await` no está permitido en una función normal. Añade
`async`: `async function empezar() { ... }`.

### Olvidar gestionar una promesa rechazada

```javascript norun
function cargar() {
  return new Promise((resolve, reject) => {
    setTimeout(() => reject(new Error("servidor inaccesible")), 10);
  });
}
cargar().then((d) => console.log(d));
console.log("Resto del programa");
```

```text
Resto del programa
file:///home/vous/projet/main.mjs:3
    setTimeout(() => reject(new Error("servidor inaccesible")), 10);
                            ^

Error: servidor inaccesible
    at Timeout._onTimeout (file:///home/vous/projet/main.mjs:3:29)
```

Una promesa rechazada que nadie atrapa detiene el programa. Añade
`.catch((error) => ...)` a la cadena, o usa `await` dentro de un `try`/`catch`.

## Ponte a prueba

**Pregunta 1:** ¿en qué orden se muestran las letras?

```javascript norun
setTimeout(() => console.log("A"), 100);
console.log("B");
setTimeout(() => console.log("C"), 0);
console.log("D");
```

> **Respuesta**
>
> `B`, `D`, `C`, `A`. El código normal va primero (`B`, `D`). Después vienen los
> callbacks, del retraso más corto al más largo: `C` (0 ms) y luego `A` (100 ms).

**Pregunta 2:** ¿qué devuelve siempre una función `async`?

> **Respuesta**
>
> Una promesa. Si la función hace `return 5`, la promesa se cumple con `5`; si lanza un
> error, la promesa se rechaza.

**Pregunta 3:** escribe una función `async` `sumaLenta(a, b)` que espere 50 ms y luego
devuelva `a + b`, y lanza dos llamadas en paralelo con `Promise.all`.

> **Respuesta**
>
> ```javascript
> const esperar = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
>
> async function sumaLenta(a, b) {
>   await esperar(50);
>   return a + b;
> }
>
> const resultados = await Promise.all([sumaLenta(1, 2), sumaLenta(10, 20)]);
> console.log(resultados);
> ```
>
> ```sortie
> [ 3, 30 ]
> ```

## En resumen

- JavaScript hace una cosa a la vez, pero nunca espera bloqueando: el bucle de eventos
  retoma las tareas cuando están listas.
- `setTimeout` programa un callback; el código en curso siempre va primero.
- Una promesa representa un resultado futuro: `then` para el éxito, `catch` para el
  error.
- `async`/`await` escriben el código asíncrono de arriba abajo; los errores se atrapan
  con `try`/`catch`.
- `Promise.all`, `allSettled` y `race` lanzan varias tareas en paralelo.

Siguiente: [el DOM](dom.md). Compara con
[la concurrencia en Ruby](../ruby/concurrence.md).
