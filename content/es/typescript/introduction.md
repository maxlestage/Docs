# TypeScript: introducción e instalación

Esta página presenta TypeScript, explica cómo instalarlo y te guía para escribir, comprobar y ejecutar tu primer programa.

> **Objetivos:**
>
> - sabrás explicar qué es TypeScript y qué le añade a JavaScript;
> - sabrás instalar Node.js y TypeScript, y comprobar que funcionan;
> - sabrás escribir un archivo `.ts`, comprobarlo con `tsc` y ejecutarlo con `node`;
> - sabrás ejecutar un archivo `.ts` directamente, sin paso intermedio;
> - sabrás leer tu primer error de tipo.

## ¿Qué es TypeScript?

TypeScript es un lenguaje de programación creado por Microsoft en 2012. Es
**JavaScript con tipos**.

- **JavaScript** es el lenguaje de las páginas web. Funciona en todos los navegadores,
  y también en tu ordenador gracias a Node.js.
- Un [tipo](../debuter/glossaire.md#tipo) es la *clase* de un valor: un número, un
  texto, un verdadero/falso…

En TypeScript, puedes escribir la clase de cada valor. Aquí tienes un primer ejemplo:

```typescript
const nombre: string = "Alice";
const edad: number = 30;

console.log(nombre);
console.log(edad + 1);
```

```sortie
Alice
31
```

- `const nombre` crea una [variable](../debuter/glossaire.md#variable) llamada `nombre`.
- `: string` es la **anotación de tipo**: «esta variable contiene un texto».
- `: number` quiere decir «esta variable contiene un número».
- `console.log(...)` muestra un valor en la terminal.

Sin los `: string` y `: number`, sería JavaScript normal. Así que casi todo el código
JavaScript ya es TypeScript: no necesitas saber JavaScript para empezar, este
recorrido te enseña los dos a la vez.

> **Nota:** el recorrido de [JavaScript](../javascript/introduction.md) presenta el mismo
> lenguaje sin los tipos. Puedes leerlo más adelante, por curiosidad.

## ¿Para qué sirven los tipos?

Los tipos sirven para encontrar los [bugs](../debuter/glossaire.md#bug) **antes** de
ejecutar el programa.

Piensa en una analogía: un corrector ortográfico subraya las faltas mientras
escribes, antes de enviar la carta. TypeScript hace lo mismo con tu código. Lo
relee todo y subraya lo que no puede funcionar.

Aquí tienes un programa JavaScript con un error. Calcula un precio total:

```javascript norun
const precio = "10";
const total = precio * 2 + precio;
console.log(total);
```

```text
2010
```

Esperábamos `30`. Pero `precio` es un texto (`"10"`, entre comillas). `precio * 2`
da `20`, y luego `20 + "10"` pega los dos textos: `"2010"`. JavaScript no dice nada,
muestra un resultado incorrecto.

Con TypeScript, declaramos que `precio` es un número. El error salta a la vista
enseguida, antes de ejecutar nada:

```typescript norun
const precio: number = "10";
```

```text
main.ts:1:7 - error TS2322: Type 'string' is not assignable to type 'number'.

1 const precio: number = "10";
        ~~~~~~


Found 1 error in main.ts:1
```

Lo corriges con `const precio: number = 10;` y el programa muestra `30`.

> **Recuerda:** JavaScript descubre los errores sobre la marcha, a veces nunca.
> TypeScript los encuentra al releer el código, antes de ejecutarlo.

Los tipos tienen otras ventajas:

- tu [editor de código](../debuter/editeur.md) sabe qué contiene cada variable:
  te propone las acciones correctas y completa los nombres;
- el código se documenta solo: `edad: number` dice claramente lo que se espera;
- cuando modificas un programa, TypeScript te muestra todos los sitios que hay que adaptar.

## Cómo funciona: tsc y node

El ordenador no sabe ejecutar TypeScript directamente. Pasa por dos etapas:

1. **`tsc`** (*TypeScript compiler*) comprueba los tipos y luego traduce el archivo `.ts`
   a un archivo `.js`: simplemente quita las anotaciones. Es un
   [compilador](../debuter/glossaire.md#compilador).
2. **`node`** ejecuta el archivo `.js` obtenido.

```text
hola.ts  --(tsc: comprueba y traduce)-->  hola.js  --(node: ejecuta)-->  resultado
```

Así que los tipos solo existen durante la comprobación. En el momento de la ejecución,
han desaparecido: solo queda JavaScript.

## Instalar Node.js

TypeScript se instala con **npm**, el gestor de paquetes que viene con Node.js.
Así que instala primero Node.js. Este sitio usa **Node 22**; una versión más reciente
también funciona.

### En Windows y macOS

Descarga el instalador «LTS» (la versión estable) del sitio nodejs.org y
ejecútalo. Deja las opciones que vienen por defecto.

### En Linux

Usa el gestor de paquetes de tu sistema, o la herramienta `nvm`, que
instala la versión que elijas:

```sh
sudo apt install nodejs npm    # Debian, Ubuntu (versión a veces antigua)
nvm install 22                 # con nvm: Node 22
```

### Comprobar Node.js

Abre una terminal y escribe estos dos comandos. Si la terminal es nueva para ti,
lee primero [la terminal](../debuter/terminal.md).

```sh
node --version
npm --version
```

Debes ver dos números, por ejemplo `v22.22.0` y `10.9.4`. Si la terminal responde
«command not found» (comando no encontrado), ciérrala y vuelve a abrirla después de
la instalación.

## Instalar TypeScript

Basta con un solo comando:

```sh
npm install -g typescript
```

- `npm install` descarga e instala un paquete;
- `-g` (*global*) lo instala para todo el ordenador: el comando `tsc` funcionará en
  cualquier carpeta.

Comprueba la instalación:

```sh
tsc --version
```

```text
Version 5.8.3
```

Este sitio comprueba sus ejemplos con TypeScript 5.8; una versión más reciente también
funciona.

> **Consejo:** en Linux o macOS, si `npm install -g` responde *permission denied*
> (permiso denegado), instala Node.js con `nvm`: lo guarda todo en tu carpeta
> personal, sin necesidad de permisos de administrador.

## Tu primer programa

Abre tu [editor de código](../debuter/editeur.md). Crea una carpeta `pruebas`, y
dentro un archivo `hola.ts` con este contenido:

```typescript
const nombre: string = "Alice";
console.log(`Hola, ${nombre}!`);
```

```sortie
Hola, Alice!
```

- El texto entre comillas invertidas `` ` `` puede contener `${...}`: el valor de
  `nombre` se inserta en ese lugar.
- Cada instrucción termina con un punto y coma `;`.

### Compilar con tsc

En la terminal, sitúate en la carpeta `pruebas` (con `cd`) y escribe:

```sh
tsc hola.ts
```

Si todo va bien, `tsc` no muestra **nada**: si no hay noticias, son buenas noticias.
Ha aparecido un archivo `hola.js` al lado. Ábrelo: es el mismo programa, sin la
anotación `: string`.

```javascript norun
"use strict";
const nombre = "Alice";
console.log(`Hola, ${nombre}!`);
```

La línea `"use strict";` le pide a JavaScript que sea más estricto; puedes
ignorarla.

### Ejecutar con node

```sh
node hola.js
```

```text
Hola, Alice!
```

> **Recuerda:** `tsc archivo.ts` comprueba y produce `archivo.js`;
> `node archivo.js` lo ejecuta.

## Ejecutar un archivo .ts directamente

Compilar y luego ejecutar son dos comandos en cada prueba. Existen dos atajos.

### Con Node 22

Node sabe ejecutar por sí mismo un archivo `.ts`: borra las anotaciones sobre la marcha.

```sh
node --experimental-strip-types hola.ts
```

Desde Node 22.18, la opción ya ni siquiera es necesaria: basta con `node hola.ts`.

### Con tsx

`tsx` es una pequeña herramienta que ejecuta un archivo TypeScript con un solo comando.
`npx` la descarga la primera vez que la usas:

```sh
npx tsx hola.ts
```

> **Atención:** estos dos atajos **no comprueban los tipos**. Quitan las
> anotaciones y ejecutan el programa, aunque contenga un error de tipo. Ejecuta
> `tsc --noEmit hola.ts` para comprobar sin crear un archivo `.js`.

## Probar en línea: el Playground

El sitio **typescriptlang.org/play** (el «Playground», el patio de juegos) permite
probar TypeScript en el navegador, sin instalar nada.

- A la izquierda, escribes tu código; los errores se subrayan en rojo.
- A la derecha, la pestaña «.JS» muestra el JavaScript producido.
- El botón «Run» ejecuta el código; la pestaña «Logs» muestra lo que imprime
  `console.log`.

> **Consejo:** pasa el ratón por encima de una variable: el Playground muestra su tipo.
> Es la mejor forma de ver lo que TypeScript ha entendido.

## Leer un error de tipo

Cuando `tsc` encuentra un problema, muestra un error. Tomemos este archivo, con una
errata en el nombre de la variable:

```typescript norun
const nombre: string = "Alice";
console.log(nombr);
```

```text
main.ts:2:13 - error TS2552: Cannot find name 'nombr'. Did you mean 'nombre'?

2 console.log(nombr);
              ~~~~~

  main.ts:1:7
    1 const nombre: string = "Alice";
            ~~~~~~
    'nombre' is declared here.


Found 1 error in main.ts:2
```

Léelo trozo a trozo:

- `main.ts:2:13`: el archivo, la línea 2, la columna 13;
- `error TS2552`: el número del error (útil para buscar en la web);
- `Cannot find name 'nombr'`: «no encuentro el nombre `nombr`»;
- `Did you mean 'nombre'?`: «¿querías decir `nombre`?»;
- las olas `~~~` subrayan el lugar exacto;
- la parte con sangría muestra dónde se declaró `nombre` (*declared here*).

Los mensajes están en inglés. Algunas palabras aparecen a menudo: *type* (tipo),
*assignable* (que se puede guardar en), *property* (propiedad), *argument* (argumento),
*expected* (esperado).

## Errores frecuentes

### Ejecutar node sobre el archivo .ts con un Node antiguo

```sh
node hola.ts
```

```text
hola.ts:1
const nombre: string = "Alice";
      ^^^^^^

SyntaxError: Missing initializer in const declaration
    at wrapSafe (node:internal/modules/cjs/loader:1638:18)
```

El mensaje sigue con otras líneas `at ...`, sin interés aquí. Antes de Node 22.18, `node` no entiende las anotaciones como `: string`. Compila
primero (`tsc hola.ts`) y luego ejecuta `node hola.js`. O usa
`node --experimental-strip-types hola.ts`.

### Creer que tsc ha bloqueado el programa

```typescript norun
const edad: number = "treinta";
console.log(edad);
```

```text
main.ts:1:7 - error TS2322: Type 'string' is not assignable to type 'number'.

1 const edad: number = "treinta";
        ~~~~


Found 1 error in main.ts:1
```

Por defecto, `tsc` muestra el error **y** crea de todas formas el archivo `.js`. No ejecutes
`node` mientras `tsc` señale errores: corrige primero, aquí con
`const edad: number = 30;`.

### Olvidar instalar TypeScript

```sh
tsc hola.ts
```

```text
tsc: command not found
```

TypeScript no está instalado, o la terminal se abrió antes de la instalación.
Escribe `npm install -g typescript` y abre una terminal nueva.

## Ponte a prueba

**Pregunta 1:** ¿qué hace `tsc hola.ts`?

> **Respuesta**
>
> Comprueba los tipos del archivo y luego crea `hola.js`: el mismo programa, sin
> las anotaciones. Después se ejecuta con `node hola.js`.

**Pregunta 2:** ¿por qué es útil TypeScript, si el programa final es
JavaScript?

> **Respuesta**
>
> Porque encuentra los errores **antes** de la ejecución: un texto en lugar de un
> número, una errata en un nombre… El programa que se ejecuta es JavaScript, pero
> un JavaScript ya revisado.

**Pregunta 3:** `npx tsx juego.ts` ejecuta tu programa sin errores. ¿Está bien tipado?

> **Respuesta**
>
> No necesariamente: `tsx` no comprueba los tipos. Ejecuta `tsc --noEmit juego.ts` para
> estar seguro.

## En resumen

- TypeScript es JavaScript con tipos: `const edad: number = 30;`.
- Los tipos encuentran los errores antes de la ejecución, como un corrector ortográfico.
- Se instala Node.js y luego `npm install -g typescript`; `tsc --version` lo comprueba.
- `tsc hola.ts` comprueba y produce `hola.js`, que se ejecuta con `node hola.js`.
- `node --experimental-strip-types` y `npx tsx` ejecutan un `.ts` directamente, sin
  comprobar los tipos.
- El Playground (typescriptlang.org/play) permite probar sin instalar nada.

Siguiente: [primeros pasos](premiers-pas.md).
