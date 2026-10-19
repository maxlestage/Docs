# Gestionar los errores

Esta página explica cómo lanzar y capturar errores en TypeScript, cómo manejar un error de tipo `unknown` y cómo devolver un resultado en lugar de lanzar un error.

> **Objetivos:**
>
> - sabrás lanzar un error con `throw new Error(...)`;
> - sabrás capturarlo con `try`, `catch` y `finally`;
> - sabrás estrechar un error `unknown` con `instanceof Error`;
> - sabrás crear tus propias clases de error;
> - sabrás devolver un resultado `{ ok: true, … } | { ok: false, … }` y validar un
>   dato desconocido.

## Un error detiene el programa

Cuando un problema impide continuar, el código **lanza** una
[excepción](../debuter/glossaire.md#excepcion) con `throw`. «Lanzar» quiere decir:
hacer sonar la alarma. Si nadie la atiende, el programa se detiene.

```typescript norun
function dividir(a: number, b: number): number {
  if (b === 0) {
    throw new Error("división por cero");
  }
  return a / b;
}

console.log("Inicio");
console.log(dividir(10, 0));
console.log("Fin");
```

```text
Inicio
file:///home/alice/projet/main.js:3
        throw new Error("división por cero");
              ^

Error: división por cero
    at dividir (file:///home/alice/projet/main.js:3:15)
    at file:///home/alice/projet/main.js:8:13
    at ModuleJob.run (node:internal/modules/esm/module_job:343:25)
    at async onImport.tracePromise.__proto__ (node:internal/modules/esm/loader:665:26)
    at async asyncRunEntryPointWithESMLoader (node:internal/modules/run_main:117:5)

Node.js v22.22.0
```

`Fin` nunca se muestra. El mensaje se lee así:

- `Error: división por cero`: el tipo del error y su descripción;
- las líneas `at …` forman la **traza de llamadas**: el camino seguido hasta el
  error. Busca la primera línea que habla de **tu** archivo.

> **Nota:** Node ejecuta el archivo JavaScript compilado (`main.js`). Por eso los
> números de línea pueden diferir un poco de tu archivo `.ts`. La opción `sourceMap`
> lo corrige (consulta [la configuración](configuration.md)).

`new Error("…")` crea un objeto error. Tiene dos propiedades útiles: `message` (tu
texto) y `name` (`"Error"`). JavaScript también tiene `TypeError`, `RangeError`,
`SyntaxError`… que funcionan igual.

## try y catch

`try` («intentar») rodea el código arriesgado. Si se lanza un error, la ejecución
salta a `catch` («capturar»). Es como una red bajo un trapecista: la caída se
detiene y el espectáculo continúa.

```typescript
function dividir(a: number, b: number): number {
  if (b === 0) {
    throw new Error("división por cero");
  }
  return a / b;
}

try {
  console.log(dividir(10, 2));
  console.log(dividir(10, 0));
  console.log("nunca se muestra");
} catch (e) {
  console.log("Ha ocurrido un error.");
}
console.log("El programa continúa.");
```

```sortie
5
Ha ocurrido un error.
El programa continúa.
```

En cuanto `dividir(10, 0)` lanza el error, el resto del `try` se salta.

## El error capturado es unknown

En JavaScript, se puede lanzar **cualquier cosa**: `throw "uy"`, `throw 42`… Una
biblioteca puede hacerlo. Por eso TypeScript no puede saber qué contiene `e`. Le da
el tipo `unknown` («desconocido»).

Con `unknown`, TypeScript lo prohíbe todo mientras no hayas comprobado el tipo. Es el
[estrechamiento](unions.md#el-estrechamiento-con-instanceof) que viste en el capítulo
sobre las uniones: comprueba con `instanceof Error`.

```typescript
function leerNumero(texto: string): number {
  const n = Number(texto);
  if (Number.isNaN(n)) {
    throw new Error(`«${texto}» no es un número`);
  }
  return n;
}

for (const entrada of ["12", "doce"]) {
  try {
    console.log(leerNumero(entrada) * 2);
  } catch (e) {
    if (e instanceof Error) {
      console.log(`Error: ${e.message}`);
    } else {
      console.log("Error desconocido:", e);
    }
  }
}
```

```sortie
24
Error: «doce» no es un número
```

Dentro del `if`, `e` es un `Error`: `e.message` está permitido. El `else` trata el
caso poco frecuente en que alguien lanzó otra cosa que un error.

> **Consejo:** una pequeña función evita repetir esta comprobación por todas partes:
>
> ```typescript norun
> function mensajeDe(e: unknown): string {
>   return e instanceof Error ? e.message : String(e);
> }
> ```

## finally

El bloque `finally` («finalmente») se ejecuta **siempre**, haya un error o no. Sirve
para ordenar: cerrar un archivo, parar un temporizador, restablecer un estado.

```typescript
function tratar(valor: number) {
  console.log(`Apertura para ${valor}`);
  try {
    if (valor < 0) {
      throw new Error("valor negativo");
    }
    console.log("Tratamiento correcto");
  } catch (e) {
    if (e instanceof Error) console.log(`Problema: ${e.message}`);
  } finally {
    console.log("Cierre");
  }
}

tratar(5);
tratar(-1);
```

```sortie
Apertura para 5
Tratamiento correcto
Cierre
Apertura para -1
Problema: valor negativo
Cierre
```

## Tus propias clases de error

Un error es un objeto como cualquier otro: puedes crear tu propia [clase](types.md)
que **hereda** de `Error` con `extends`. Así puedes distinguir los errores con
`instanceof` y añadirles información.

```typescript
class SaldoInsuficienteError extends Error {
  constructor(public falta: number) {
    super(`saldo insuficiente (faltan ${falta} €)`);
    this.name = "SaldoInsuficienteError";
  }
}

function retirar(saldo: number, importe: number): number {
  if (importe > saldo) {
    throw new SaldoInsuficienteError(importe - saldo);
  }
  return saldo - importe;
}

try {
  retirar(50, 80);
} catch (e) {
  if (e instanceof SaldoInsuficienteError) {
    console.log(`Rechazado: ${e.message}`);
    console.log(`Importe que falta: ${e.falta}`);
  } else {
    throw e;
  }
}
```

```sortie
Rechazado: saldo insuficiente (faltan 30 €)
Importe que falta: 30
```

Tres detalles:

- `super(message)` llama al constructor de `Error`, que guarda el mensaje;
- `this.name` cambia el nombre que aparece en la traza de llamadas;
- `throw e` en el `else` **vuelve a lanzar** los errores que no sabes tratar. No te
  los tragues en silencio: ocultarías errores reales.

## Devolver un resultado en lugar de lanzar

Un error lanzado es invisible en la firma: nada en
`function leerEdad(t: string): number` avisa de que puede fallar. Es fácil olvidar el
`try`.

Otro enfoque: la función **devuelve** o un éxito, o un fallo, en una
[unión discriminada](unions.md#las-uniones-discriminadas). El campo `ok` sirve de
etiqueta.

```typescript
type Resultado<T> = { ok: true; value: T } | { ok: false; error: string };

function leerEdad(texto: string): Resultado<number> {
  const n = Number(texto);
  if (!Number.isInteger(n)) {
    return { ok: false, error: `«${texto}» no es un entero` };
  }
  if (n < 0 || n > 150) {
    return { ok: false, error: `${n} no es una edad realista` };
  }
  return { ok: true, value: n };
}

for (const entrada of ["34", "treinta", "200"]) {
  const r = leerEdad(entrada);
  if (r.ok) {
    console.log(`Edad aceptada: ${r.value}`);
  } else {
    console.log(`Rechazado: ${r.error}`);
  }
}
```

```sortie
Edad aceptada: 34
Rechazado: «treinta» no es un entero
Rechazado: 200 no es una edad realista
```

El tipo de retorno anuncia claramente el posible fallo. Y TypeScript te **obliga** a
comprobar `r.ok` antes de leer `r.value` (consulta los
[errores frecuentes](#errores-frecuentes)). Es la idea del tipo `Result` de Rust
(consulta [los errores en Rust](../rust/erreurs.md)).

> **Recuerda:** lanza un error para lo que es **anormal** (un bug, un archivo de
> configuración que falta). Devuelve un resultado para lo que es **previsible** (una
> entrada no válida, una búsqueda sin respuesta).

## Validar un dato desconocido

Los datos que vienen de fuera (un archivo [JSON](../debuter/glossaire.md#json), una
entrada del usuario, una respuesta web) no tienen ninguna garantía de forma.
`JSON.parse` devuelve `any`, lo que desactiva toda comprobación. Guarda mejor el
resultado en una variable `unknown` y luego comprueba su forma paso a paso.

```typescript
interface Jugador {
  nombre: string;
  puntos: number;
}

function validarJugador(dato: unknown): Resultado<Jugador> {
  if (typeof dato !== "object" || dato === null) {
    return { ok: false, error: "no es un objeto" };
  }
  if (!("nombre" in dato) || typeof dato.nombre !== "string") {
    return { ok: false, error: "nombre ausente o no válido" };
  }
  if (!("puntos" in dato) || typeof dato.puntos !== "number") {
    return { ok: false, error: "puntos ausentes o no válidos" };
  }
  return { ok: true, value: { nombre: dato.nombre, puntos: dato.puntos } };
}

type Resultado<T> = { ok: true; value: T } | { ok: false; error: string };

const textos = ['{"nombre": "Lina", "puntos": 42}', '{"nombre": "Tom"}', "[1, 2]"];

for (const texto of textos) {
  const dato: unknown = JSON.parse(texto);
  const r = validarJugador(dato);
  console.log(r.ok ? `${r.value.nombre} tiene ${r.value.puntos} puntos` : `No válido: ${r.error}`);
}
```

```sortie
Lina tiene 42 puntos
No válido: puntos ausentes o no válidos
No válido: nombre ausente o no válido
```

Cada comprobación estrecha `dato` un poco más: primero un objeto, luego un objeto con
un `nombre` de texto, luego con unos `puntos` numéricos. Al final, TypeScript queda
convencido.

> **Nota:** un array también es un `object`: `[1, 2]` pasa la primera comprobación y
> luego falla en el `nombre`. Para formatos grandes, bibliotecas como Zod escriben
> estas comprobaciones por ti (consulta [los módulos](modules.md)).

## Errores frecuentes

### Leer e.message sin comprobar

```typescript norun
try {
  JSON.parse("{uy");
} catch (e) {
  console.log(e.message);
}
```

```text
main.ts(4,15): error TS18046: 'e' is of type 'unknown'.
```

`e` puede ser cualquier cosa. Comprueba primero:
`if (e instanceof Error) console.log(e.message);`.

### Leer value sin comprobar ok

```typescript norun
type Resultado<T> = { ok: true; value: T } | { ok: false; error: string };
function dividir(a: number, b: number): Resultado<number> {
  if (b === 0) return { ok: false, error: "división por cero" };
  return { ok: true, value: a / b };
}
const r = dividir(10, 2);
console.log(r.value);
```

```text
main.ts(7,15): error TS2339: Property 'value' does not exist on type 'Resultado<number>'.
  Property 'value' does not exist on type '{ ok: false; error: string; }'.
```

En caso de fallo, no hay `value`. Comprueba la etiqueta:
`if (r.ok) console.log(r.value);`.

### Usar un dato unknown directamente

```typescript norun
const datos: unknown = JSON.parse('{"nombre": "Alice"}');
console.log(datos.nombre);
```

```text
main.ts(2,13): error TS18046: 'datos' is of type 'unknown'.
```

Es a propósito: `unknown` te obliga a comprobar. Valida la forma
(`typeof datos === "object" && datos !== null && "nombre" in datos`) o usa una función
de validación como `validarJugador` más arriba.

### Lanzar un texto en lugar de un Error

```typescript norun
function leerEdad(texto: string): number {
  const n = Number(texto);
  if (Number.isNaN(n)) {
    throw "edad no válida";
  }
  return n;
}
try {
  leerEdad("abc");
} catch (e) {
  if (e instanceof Error) console.log(e.message);
  else console.log("otra cosa:", e);
}
```

```text
otra cosa: edad no válida
```

TypeScript acepta `throw "…"`, pero ese texto no es un `Error`: no tiene `message` ni
traza de llamadas, y `instanceof Error` falla. Lanza siempre un objeto:
`throw new Error("edad no válida");`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```typescript norun
try {
  console.log("A");
  throw new Error("avería");
  console.log("B");
} catch (e) {
  console.log("C");
} finally {
  console.log("D");
}
```

> **Respuesta**
>
> `A`, `C` y luego `D`. `B` nunca se muestra: después de `throw`, la ejecución salta
> al `catch`. `finally` se ejecuta siempre.

**Pregunta 2:** ¿por qué el error capturado en `catch (e)` tiene el tipo `unknown`?

> **Respuesta**
>
> Porque en JavaScript se puede lanzar cualquier valor, no solo objetos `Error`.
> TypeScript no puede saber cuál: te obliga a comprobarlo, por ejemplo con
> `e instanceof Error`.

**Pregunta 3:** escribe una función `raiz(x: number)` que devuelva un
`Resultado<number>`: un fallo si `x` es negativo y, si no, la raíz cuadrada.

> **Respuesta**
>
> ```typescript
> type Resultado<T> = { ok: true; value: T } | { ok: false; error: string };
>
> function raiz(x: number): Resultado<number> {
>   if (x < 0) {
>     return { ok: false, error: "número negativo" };
>   }
>   return { ok: true, value: Math.sqrt(x) };
> }
>
> const r1 = raiz(16);
> const r2 = raiz(-4);
> console.log(r1.ok ? r1.value : r1.error);
> console.log(r2.ok ? r2.value : r2.error);
> ```
>
> ```sortie
> 4
> número negativo
> ```

## En resumen

- `throw new Error("…")` lanza un error; sin `catch`, el programa se detiene.
- `try` / `catch` captura el error, `finally` se ejecuta en todos los casos.
- El error capturado es `unknown`: estréchalo con `instanceof Error`.
- Tus errores heredan de `Error`; vuelve a lanzar los que no sabes tratar.
- Para los fallos previsibles, devuelve `{ ok: true, value } | { ok: false, error }`.
- Un dato que viene de fuera es `unknown`: valida su forma antes de usarlo.

Siguiente: [los módulos y npm](modules.md)
