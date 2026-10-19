# JavaScript: introducción e instalación

Esta página presenta JavaScript, explica cómo instalar Node.js y te guía para escribir tu primer programa.

> **Objetivos:**
>
> - sabrás decir qué es JavaScript y dónde se ejecuta;
> - sabrás instalar Node.js y comprobar que funciona;
> - sabrás probar código en la consola del navegador y en Node;
> - sabrás escribir un archivo `.js` y ejecutarlo con `node`;
> - sabrás qué es un módulo ES y por qué este sitio los usa.

## ¿Qué es JavaScript?

JavaScript es **el lenguaje de la web**. Lo creó Brendan Eich en 1995, en diez
días, para dar vida a las páginas web: un botón que reacciona, un menú que se abre,
un formulario que comprueba lo que escribes.

Un **lenguaje de programación** es un idioma preciso que el ordenador sabe seguir.
Un [programa](../debuter/glossaire.md#programa) es un texto escrito en ese idioma:
una serie de órdenes, como una receta de cocina.

```javascript
console.log("¡Hola!");
console.log(2 + 3);
```

```sortie
¡Hola!
5
```

`console.log(...)` muestra un valor. Léelo como «en la consola, apunta esto».
El punto y coma `;` marca el final de una instrucción, como el punto al final de una
frase.

> **Nota:** a pesar de su nombre, JavaScript no tiene casi nada que ver con el lenguaje Java.
> El nombre se eligió en 1995 para aprovechar la moda de Java.

## ¿Dónde se ejecuta JavaScript?

JavaScript es un lenguaje **interpretado**: un programa llamado
[intérprete](../debuter/glossaire.md#interprete) lee tu código y lo ejecuta
directamente. Hay dos grandes lugares donde vive ese intérprete.

- **En el navegador** (Firefox, Chrome, Safari…). Cada navegador contiene un
  motor de JavaScript. Es él quien hace que las páginas web se muevan.
- **En Node.js**, un programa que ejecuta JavaScript **fuera del navegador**:
  en el [terminal](../debuter/terminal.md), en un servidor, para scripts.

Es como un mismo idioma hablado en dos países: las palabras son las mismas, pero
algunas cosas solo existen en un país. El navegador sabe manejar una página
web; Node sabe leer archivos de tu disco.

Este sitio usa **Node.js** para casi todos los ejemplos. La página sobre
[el DOM](dom.md) muestra lo que es propio del navegador.

## Instalar Node.js

Este sitio usa **Node.js 22**. Una versión más reciente también funciona.

### Desde el sitio oficial

Ve a **nodejs.org** y descarga la versión **LTS** (*Long Term Support*,
«soporte a largo plazo»): es la versión estable, recomendada para todo el mundo.
Ejecuta el instalador y deja las opciones propuestas. Funciona en Windows, macOS y
Linux.

### Con nvm

`nvm` (*Node Version Manager*) instala varias versiones de Node y permite pasar de una a
otra. Una vez instalado nvm (mira su página en GitHub), escribe:

```sh
nvm install 22
nvm use 22
```

> **Consejo:** usa nvm si trabajas en varios proyectos que piden
> versiones distintas de Node. En Windows, el equivalente se llama **nvm-windows**.

### Con un gestor de paquetes

En Linux y macOS, también puedes usar el gestor de paquetes del sistema.

```sh
sudo apt install nodejs      # Debian, Ubuntu
brew install node            # macOS con Homebrew
```

> **Atención:** los paquetes de algunas distribuciones de Linux son antiguos. Comprueba
> la versión; si es inferior a 22, mejor usa nodejs.org o nvm.

## Comprobar la instalación

Abre un terminal y escribe el siguiente comando. Si el terminal es nuevo para ti,
lee primero [el terminal](../debuter/terminal.md).

```sh
node --version
```

Deberías ver un número de versión, por ejemplo `v22.22.0`. Si el terminal responde
«command not found» (comando no encontrado), Node no está instalado, o el terminal se
abrió antes de la instalación: ciérralo y vuelve a abrirlo.

Node viene con **npm** (*Node Package Manager*), la herramienta que instala
[bibliotecas](../debuter/glossaire.md#biblioteca). `npm --version` comprueba que también
está. No lo necesitarás hasta la página sobre [los módulos](modules.md).

## Probar JavaScript enseguida

### En la consola del navegador

Ya tienes JavaScript: está en tu navegador. Abre cualquier página
y pulsa **F12** (o **Cmd + Option + I** en Mac). Elige la pestaña
**Console**. Escribe una línea y pulsa Intro: el navegador responde enseguida.

```text
> 2 + 3
5
> "Ho" + "la"
'Hola'
```

Es una [consola interactiva](../debuter/glossaire.md#repl): perfecta para probar una
pequeña idea sin crear un archivo.

### En Node

Escribe `node` solo en el terminal. Obtienes la misma consola interactiva, esta vez en el
terminal.

```text
$ node
Welcome to Node.js v22.22.0.
Type ".help" for more information.
> 1 + 2
3
> "Ho" + "la"
'Hola'
> .exit
```

- El signo `>` espera tu línea. La línea siguiente muestra el resultado.
- `.exit` (o dos veces **Ctrl + C**) sale de la consola.

## Tu primer programa

Abre tu [editor de código](../debuter/editeur.md). Crea una carpeta `js` y luego un
archivo `hola.js` con este contenido:

```javascript
console.log("¡Hola, mundo!");
```

```sortie
¡Hola, mundo!
```

- `console.log` muestra un valor y luego pasa a la línea siguiente.
- El texto va entre comillas `"..."`. Sin comillas, JavaScript creería que es
  un nombre de variable.

Un programa puede tener varias líneas. JavaScript las ejecuta en orden, de arriba
abajo.

```javascript
console.log("Línea 1");
console.log("Línea 2");
console.log(2 + 3);
console.log("2 + 3");
```

```sortie
Línea 1
Línea 2
5
2 + 3
```

Sin comillas, JavaScript calcula. Entre comillas, muestra el texto tal cual.

## Ejecutar un programa

En el terminal, ve a la carpeta del archivo (con el comando `cd`) y luego
escribe `node` seguido del nombre del archivo:

```sh
cd js
node hola.js
```

```text
¡Hola, mundo!
```

> **Recuerda:** un programa de JavaScript es un archivo `.js`. Lo ejecutas con
> `node nombre_del_archivo.js`.

## Los módulos ES

Un **módulo** es un archivo de código que puede compartir cosas con otros
archivos. JavaScript tiene dos formas de escribir módulos:

- los **módulos ES** (*ECMAScript modules*), la forma moderna y estándar, con las
  palabras `import` y `export`;
- **CommonJS**, la forma antigua propia de Node, con `require(...)`.

Este sitio usa **siempre los módulos ES**. Para que Node lo sepa, hay dos soluciones:

1. llamar a tus archivos **`.mjs`** en lugar de `.js` (`hola.mjs`);
2. o añadir un archivo `package.json` en la carpeta, con este contenido:

```json
{
  "type": "module"
}
```

Con este archivo, todos los `.js` de la carpeta son módulos ES. Aquí tienes un programa que
usa `import` para leer información de Node:

```javascript
import { platform } from "node:process";

console.log(typeof platform);
```

```sortie
string
```

`import` va a buscar `platform` en el módulo `node:process`, que viene con Node.
No intentes entenderlo todo: [los módulos](modules.md) tienen su propia página.

> **Nota:** Node 22 a menudo adivina solo que un `.js` es un módulo ES cuando ve
> `import`. Pero decir las cosas claramente (`.mjs` o `"type": "module"`) evita
> sorpresas.

## Leer un mensaje de error

Cuando JavaScript no entiende tu código, se detiene y muestra un
[error](../debuter/glossaire.md#error). No pasa nada: el mensaje te ayuda.

```text
/home/alice/js/hola.js:1
console.log(Hola);
            ^

ReferenceError: Hola is not defined
    at Object.<anonymous> (/home/alice/js/hola.js:1:13)

Node.js v22.22.0
```

Léelo de arriba abajo:

- `hola.js:1`: el archivo y el número de línea del problema;
- la línea de código, con un `^` bajo el lugar exacto;
- `ReferenceError`: el tipo de error; `Hola is not defined`: la causa
  («`Hola` no está definido»);
- las líneas `at ...`: el camino que siguió el programa (aquí `1:13` significa línea 1,
  columna 13).

Los mensajes están en inglés. Algunas palabras aparecen a menudo: *not defined* (no
definido), *unexpected* (inesperado), *cannot* (no se puede), *is not a function* (no es
una función).

## Errores frecuentes

### Una mayúscula en console

```javascript norun
Console.log("Hola");
```

```text
/home/alice/js/hola.js:1
Console.log("Hola");
^

ReferenceError: Console is not defined
    at Object.<anonymous> (/home/alice/js/hola.js:1:1)

Node.js v22.22.0
```

JavaScript distingue entre mayúsculas y minúsculas. `Console` y `console`
son dos nombres distintos, y solo existe el segundo. Escribe `console.log("Hola");`.

### Olvidar la comilla final

```javascript norun
console.log("Hola);
```

```text
/home/alice/js/hola.js:1
console.log("Hola);
            ^^^^^^^

SyntaxError: Invalid or unexpected token

Node.js v22.22.0
```

El texto nunca se cierra: JavaScript ya no sabe dónde termina (*invalid or
unexpected token*, «trozo no válido o inesperado»). Añade la comilla:
`console.log("Hola");`.

### Ejecutar el archivo desde la carpeta equivocada

```sh
node hola.js
```

```text
node:internal/modules/cjs/loader:1386
  throw err;
  ^

Error: Cannot find module '/home/alice/hola.js'
```

Node busca `hola.js` en la carpeta donde está el terminal (aquí `/home/alice`),
no en otro sitio. Ve a la carpeta correcta con `cd js`, o comprueba el nombre del archivo
con `ls`.

### Escribir el comando en la consola de Node

```text
> node hola.js
     ^^^^

Uncaught SyntaxError: Unexpected identifier 'hola'
```

El prompt `>` muestra que estás **dentro** de Node, que espera JavaScript, no un
comando del terminal. Sal con `.exit` y luego escribe `node hola.js`.

## Ponte a prueba

**Pregunta 1:** ¿qué comando comprueba que Node.js está instalado?

> **Respuesta**
>
> `node --version`. Muestra el número de versión, por ejemplo `v22.22.0`.

**Pregunta 2:** ¿qué muestra este programa?

```javascript norun
console.log("4 * 2");
console.log(4 * 2);
```

> **Respuesta**
>
> `4 * 2` y luego `8`. Entre comillas, es un texto que se muestra tal cual. Sin
> comillas, JavaScript calcula.

**Pregunta 3:** quieres usar `import` en `juego.js`. ¿Qué haces?

> **Respuesta**
>
> Cambia el nombre del archivo a `juego.mjs`, o añade un `package.json` que contenga
> `"type": "module"`. Después ejecútalo con `node juego.mjs` (o `node juego.js`).

## En resumen

- JavaScript es el lenguaje de la web: funciona en el navegador y, con Node.js,
  en el terminal.
- Instala Node desde nodejs.org (versión LTS) o con nvm; `node --version` lo comprueba.
- La consola del navegador (F12) y el comando `node` solo permiten probar
  código enseguida.
- Un programa es un archivo `.js` que se ejecuta con `node nombre_del_archivo.js`;
  `console.log` muestra un valor.
- Este sitio usa los módulos ES: archivos `.mjs` o `"type": "module"`.
- Un mensaje de error da el archivo, la línea, el tipo de error y la causa.

Siguiente: [primeros pasos](premiers-pas.md).
