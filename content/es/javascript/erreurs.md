# Gestionar los errores

Esta página explica cómo leer un mensaje de error de Node, cómo capturar un error con `try` y `catch`, y cómo lanzar tus propios errores con `throw`.

> **Objetivos:**
>
> - sabrás leer un mensaje de error y su traza de llamadas;
> - sabrás capturar un error con `try`, `catch` y `finally`;
> - sabrás lanzar un error con `throw new Error(...)`;
> - sabrás crear tus propias clases de error y encadenar una causa;
> - sabrás comprobar los datos recibidos antes de usarlos.

## Un error detiene el programa

Cuando JavaScript se encuentra con un problema que no sabe resolver, **lanza** una
[excepción](../debuter/glossaire.md#excepcion): un objeto que describe el
[error](../debuter/glossaire.md#error). «Lanzar» quiere decir: hacer sonar la alarma.
Si nadie se ocupa de ella, el programa se detiene en seco.

Aquí tienes un programa que calcula la media de un alumno… al que le olvidamos las notas.

```javascript norun
function media(notas) {
  const total = notas.reduce((a, b) => a + b, 0);
  return total / notas.length;
}

function boletin(alumno) {
  return `${alumno.nombre}: ${media(alumno.notas)}`;
}

console.log(boletin({ nombre: "Lea" }));
```

```text
file:///home/vous/projet/main.mjs:2
  const total = notas.reduce((a, b) => a + b, 0);
                      ^

TypeError: Cannot read properties of undefined (reading 'reduce')
    at media (file:///home/vous/projet/main.mjs:2:23)
    at boletin (file:///home/vous/projet/main.mjs:7:31)
    at file:///home/vous/projet/main.mjs:10:13
    at ModuleJob.run (node:internal/modules/esm/module_job:343:25)
    at async onImport.tracePromise.__proto__ (node:internal/modules/esm/loader:665:26)
    at async asyncRunEntryPointWithESMLoader (node:internal/modules/run_main:117:5)

Node.js v22.22.0
```

### Leer el mensaje

Este bloque de texto da miedo, pero se lee en cuatro partes:

1. **¿Dónde?** La primera línea da el archivo y la línea: `main.mjs:2`. Node copia la
   línea culpable y coloca un `^` debajo del lugar exacto.
2. **¿Qué?** `TypeError` es el **tipo** del error, su apellido.
3. **¿Por qué?** El mensaje: «no se pueden leer las propiedades de `undefined` (al leer
   `reduce`)». Dicho de otro modo: `notas` vale `undefined`, y `undefined` no tiene
   método `reduce`.
4. **¿Cómo hemos llegado aquí?** Las líneas `at ...` forman la **traza de llamadas** (en
   inglés *stack trace*): el camino que siguió el programa, desde la función más
   profunda hasta el programa principal.

La traza se lee de arriba abajo: el error está en `media` (línea 2), llamada por
`boletin` (línea 7), llamada por el programa (línea 10). Las líneas `node:internal`
vienen del propio Node: ignóralas y busca **tus** archivos.

Aquí, el verdadero culpable es la línea 10: el alumno no tiene propiedad `notas`.

## try y catch

`try` («intentar») y `catch` («atrapar») forman una red de seguridad, como la que hay
bajo un trapecista: si cae un error, se recoge, y el espectáculo continúa.

- El código arriesgado va en el bloque `try`.
- El código de emergencia va en el bloque `catch`. Recibe el error como parámetro.

```javascript
const textos = ['{"nombre": "Lea"}', "{nombre: Lea}"];

for (const texto of textos) {
  try {
    const datos = JSON.parse(texto);
    console.log("Leído:", datos.nombre);
  } catch (error) {
    console.log("Ilegible:", error.message);
  }
}
console.log("El programa continúa.");
```

```sortie
Leído: Lea
Ilegible: Expected property name or '}' in JSON at position 1 (line 1 column 2)
El programa continúa.
```

En cuanto se produce un error en `try`, JavaScript salta **inmediatamente** al `catch`:
la línea `console.log("Leído:", ...)` no se ejecuta para el segundo texto.

Un objeto error tiene tres propiedades útiles:

| Propiedad | Contenido |
|-----------|-----------|
| `name` | el tipo del error: `"SyntaxError"` |
| `message` | la descripción |
| `stack` | la traza de llamadas completa, en texto |

> **Consejo:** si no necesitas el error, puedes escribir `catch { ... }` sin paréntesis.

## finally

El bloque `finally` («para terminar») se ejecuta **siempre**: después de `try` si todo
va bien, después de `catch` si hubo un error. Sirve para ordenar, cerrar, dejar las
cosas como estaban.

```javascript
function cargar(texto) {
  console.log("Apertura");
  try {
    return JSON.parse(texto);
  } catch (error) {
    console.log("Problema:", error.name);
    return null;
  } finally {
    console.log("Cierre");
  }
}

console.log(cargar("[1, 2]"));
console.log(cargar("[1, 2"));
```

```sortie
Apertura
Cierre
[ 1, 2 ]
Apertura
Problema: SyntaxError
Cierre
null
```

Ni siquiera un `return` en `try` impide que `finally` se ejecute.

## Lanzar un error con throw

`throw` («lanzar») te permite lanzar un error tú mismo, por ejemplo para rechazar un
valor imposible. Lanza siempre un objeto `Error`, con un mensaje claro.

```javascript
function retirar(saldo, cantidad) {
  if (cantidad > saldo) {
    throw new Error("saldo insuficiente");
  }
  return saldo - cantidad;
}

console.log(retirar(100, 30));

try {
  retirar(100, 500);
} catch (error) {
  console.log(`Rechazado: ${error.message}`);
}
```

```sortie
70
Rechazado: saldo insuficiente
```

`throw` detiene la función en el acto, como `return`. Después, el error sube por la
pila de llamadas hasta el primer `catch` que encuentra. Si no hay ninguno, el programa
se detiene con el mensaje, como al principio de esta página.

## Los tipos de errores

JavaScript tiene varias clases de errores predefinidas. El nombre te da una pista.

| Tipo | ¿Cuándo? | Ejemplo de mensaje |
|------|----------|--------------------|
| `TypeError` | tipo de valor equivocado: propiedad de `undefined`, llamada a algo que no es una función | `f is not a function` |
| `ReferenceError` | variable desconocida (a menudo una errata) | `desconocida is not defined` |
| `RangeError` | número fuera de los límites permitidos | `Invalid array length` |
| `SyntaxError` | código o JSON mal escrito | `missing ) after argument list` |

Puedes lanzar estos tipos tú mismo: `throw new RangeError("nota entre 0 y 20")`.
Comprueba el tipo de un error capturado con `instanceof`.

```javascript
const pruebas = [
  () => null.longitud,
  () => variableDesconocida,
  () => "ab".repeat(-1),
  () => JSON.parse("{"),
];

for (const prueba of pruebas) {
  try {
    prueba();
  } catch (error) {
    const tipo = error instanceof TypeError ? "tipo" : "otro";
    console.log(`${error.name} (${tipo}): ${error.message}`);
  }
}
```

```sortie
TypeError (tipo): Cannot read properties of null (reading 'longitud')
ReferenceError (otro): variableDesconocida is not defined
RangeError (otro): Invalid count value: -1
SyntaxError (otro): Expected property name or '}' in JSON at position 1 (line 1 column 2)
```

> **Atención:** un `SyntaxError` en **tu código** (un paréntesis olvidado) no se puede
> capturar: Node rechaza el archivo entero antes de ejecutarlo. Solo `JSON.parse` lanza
> un `SyntaxError` que se pueda capturar.

## Tus propias clases de error

Un error es un objeto como cualquier otro. Para distinguir tus errores, crea una clase
que herede de `Error` (mira [la herencia](heritage.md)). Dale un `name`: aparecerá en
los mensajes.

```javascript
class ValidationError extends Error {
  constructor(message, campo) {
    super(message);
    this.name = "ValidationError";
    this.campo = campo;
  }
}

function registrar(usuario) {
  if (!usuario.email.includes("@")) {
    throw new ValidationError("dirección no válida", "email");
  }
  return "¡Registrado!";
}

try {
  registrar({ email: "ada.ejemplo.es" });
} catch (error) {
  if (error instanceof ValidationError) {
    console.log(`Campo ${error.campo}: ${error.message}`);
    console.log(String(error));
  } else {
    throw error;
  }
}
```

```sortie
Campo email: dirección no válida
ValidationError: dirección no válida
```

Fíjate en el `else { throw error; }`: solo tratas los errores que conoces, y **vuelves a
lanzar** los demás. Así, un verdadero bug no queda oculto en silencio.

## Encadenar una causa con error.cause

A menudo, un error técnico («JSON mal formado») esconde un problema más claro para el
usuario («la configuración es ilegible»). Entonces lanzas un nuevo error y guardas el
anterior en la opción `cause`.

```javascript
function leerConfiguracion(texto) {
  try {
    return JSON.parse(texto);
  } catch (error) {
    throw new Error("Configuración ilegible", { cause: error });
  }
}

try {
  leerConfiguracion("{uy");
} catch (error) {
  console.log(error.message);
  console.log("Causa:", error.cause.name);
}
```

```sortie
Configuración ilegible
Causa: SyntaxError
```

Si el error no se captura, Node muestra los dos, con la causa bajo `[cause]`: se
conserva toda la historia.

## Comprobar los datos recibidos

El mejor error es el que se detecta pronto. Cuando una función recibe datos del
exterior (un formulario, un archivo), compruébalos **al principio** y rechaza con
claridad lo que no está bien. Es el control de billetes a la entrada del tren, en lugar
de en pleno viaje.

```javascript
function leerNota(texto) {
  const nota = Number(texto);
  if (texto.trim() === "" || Number.isNaN(nota)) {
    throw new TypeError(`"${texto}" no es un número`);
  }
  if (nota < 0 || nota > 20) {
    throw new RangeError(`${nota} no está entre 0 y 20`);
  }
  return nota;
}

for (const entrada of ["15", "doce", "25", ""]) {
  try {
    console.log("Nota:", leerNota(entrada));
  } catch (error) {
    console.log(`${error.name}: ${error.message}`);
  }
}
```

```sortie
Nota: 15
TypeError: "doce" no es un número
RangeError: 25 no está entre 0 y 20
TypeError: "" no es un número
```

> **Recuerda:** `Number("doce")` no lanza ningún error: devuelve `NaN`, sin decir nada.
> Te toca a ti comprobarlo con `Number.isNaN`. Del mismo modo, `Number("")` vale `0`:
> de ahí la prueba con el texto vacío.

## Errores frecuentes

### Lanzar un texto en lugar de un Error

```javascript norun
try {
  throw "archivo no encontrado";
} catch (error) {
  console.log(error.message);
}
```

```text
undefined
```

No hay mensaje de error, pero el resultado es incorrecto: un texto no tiene propiedad
`message`, ni traza de llamadas. Lanza siempre un objeto:
`throw new Error("archivo no encontrado")`.

### Usar una variable declarada en try

```javascript norun
try {
  const edad = Number.parseInt("42");
} catch (e) {
  console.log("Error");
}
console.log(edad + 1);
```

```text
file:///home/vous/projet/main.mjs:6
console.log(edad + 1);
            ^

ReferenceError: edad is not defined
    at file:///home/vous/projet/main.mjs:6:13
```

El bloque `try` tiene su propio [ámbito](closures.md): `edad` desaparece en la llave de
cierre. Declara `let edad;` antes del `try`, o usa `edad` dentro del bloque.

### Creer que try captura un error de sintaxis

```javascript norun
console.log("inicio");
console.log("hola";
```

```text
file:///home/vous/projet/main.mjs:2
console.log("hola";
            ^^^^^^

SyntaxError: missing ) after argument list
```

Ni siquiera se muestra `inicio`: Node lee todo el archivo antes de ejecutarlo, y rechaza
un archivo mal escrito. Ningún `try` puede capturarlo. Corrige el código: aquí falta el
paréntesis de cierre.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```javascript norun
try {
  console.log("A");
  throw new Error("avería");
  console.log("B");
} catch {
  console.log("C");
} finally {
  console.log("D");
}
```

> **Respuesta**
>
> `A`, `C` y luego `D`. `B` nunca se muestra: después de `throw`, JavaScript salta
> directamente al `catch`. `finally` se ejecuta siempre.

**Pregunta 2:** en una traza de llamadas, ¿qué línea hay que mirar primero?

> **Respuesta**
>
> La primera línea `at ...` que habla de **tu** archivo. Las líneas `node:internal`
> pertenecen a Node.

**Pregunta 3:** escribe una función `dividir(a, b)` que lance un `RangeError` si `b`
vale `0`, y llámala dentro de un `try`.

> **Respuesta**
>
> ```javascript
> function dividir(a, b) {
>   if (b === 0) {
>     throw new RangeError("división por cero");
>   }
>   return a / b;
> }
>
> try {
>   console.log(dividir(10, 2));
>   console.log(dividir(1, 0));
> } catch (error) {
>   console.log(`${error.name}: ${error.message}`);
> }
> ```
>
> ```sortie
> 5
> RangeError: división por cero
> ```

## En resumen

- Un mensaje de error da el archivo, la línea, el tipo y la traza de llamadas: busca la
  primera línea de tu código.
- `try` / `catch` capturan un error; `finally` se ejecuta siempre.
- `throw new Error("...")` lanza un error; sube hasta el primer `catch`.
- Tus errores heredan de `Error`; `{ cause }` guarda el error original.
- Comprueba los datos al principio de una función, y recházalos con un mensaje claro.

Siguiente: [módulos y npm](modules.md). Compara con
[los errores en Ruby](../ruby/erreurs.md).
