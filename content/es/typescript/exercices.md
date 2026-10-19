# Ejercicios resueltos

Esta página propone 12 ejercicios resueltos, del más fácil al más difícil, para practicar todo el recorrido de TypeScript.

## Cómo usar esta página

Cada ejercicio tiene un nivel:

- ★☆☆: fácil;
- ★★☆: medio;
- ★★★: más difícil.

Lee el enunciado y luego escribe tu solución en un archivo `.ts`. Comprueba los tipos
con `npx tsc --noEmit --strict ejercicio.ts` y luego ejecútalo con
`npx tsx ejercicio.ts`. Abre la solución solo después de haberlo intentado.

> **Consejo:** un ejercicio está superado cuando el programa muestra el resultado
> correcto **y** `tsc --strict` no señala ningún error. Todas las soluciones de esta
> página pasan este control.

## Lo básico

### Ejercicio 1: Presentarse

Nivel: ★☆☆

Crea dos constantes, `nombre` (una cadena) y `edad` (un número). Luego muestra la frase
`Me llamo Alice y tengo 30 años.` con tus constantes.

> **Pista:** una plantilla entre comillas invertidas: `` `Hola ${nombre}` `` (mira
> [los primeros pasos](premiers-pas.md)).

> **Solución**
>
> ```typescript
> const nombre = "Alice";
> const edad = 30;
> console.log(`Me llamo ${nombre} y tengo ${edad} años.`);
> ```
>
> ```sortie
> Me llamo Alice y tengo 30 años.
> ```
>
> No hace falta anotación: TypeScript infiere `string` y `number`.

### Ejercicio 2: Par o impar

Nivel: ★☆☆

Escribe una función `esPar(n: number): boolean`. Úsala para mostrar `7 es impar` y
`10 es par`.

> **Pista:** `n % 2` da el resto de la división entre 2.

> **Solución**
>
> ```typescript
> function esPar(n: number): boolean {
>   return n % 2 === 0;
> }
>
> for (const n of [7, 10]) {
>   console.log(`${n} es ${esPar(n) ? "par" : "impar"}`);
> }
> ```
>
> ```sortie
> 7 es impar
> 10 es par
> ```

### Ejercicio 3: FizzBuzz

Nivel: ★☆☆

Para los números del 1 al 15, muestra `Fizz` si el número es divisible entre 3, `Buzz`
si lo es entre 5, `FizzBuzz` si lo es entre los dos, y el número en caso contrario.
Muéstralo todo en una sola línea, separado por espacios.

> **Pista:** comprueba primero el caso «entre 15». Guarda las palabras en un array
> `string[]` y luego usa `join(" ")` (mira [condiciones y bucles](conditions-boucles.md)).

> **Solución**
>
> ```typescript
> const palabras: string[] = [];
> for (let i = 1; i <= 15; i++) {
>   if (i % 15 === 0) palabras.push("FizzBuzz");
>   else if (i % 3 === 0) palabras.push("Fizz");
>   else if (i % 5 === 0) palabras.push("Buzz");
>   else palabras.push(String(i));
> }
> console.log(palabras.join(" "));
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```
>
> La anotación `string[]` es útil aquí: el array está vacío al principio.

### Ejercicio 4: El área de un rectángulo

Nivel: ★☆☆

Escribe una función `area(ancho, alto)` que devuelva el área de un rectángulo. Si no se
da el alto, el rectángulo es un cuadrado: el alto vale lo mismo que el ancho.
Muestra `area(4, 3)` y `area(5)`.

> **Pista:** un parámetro puede tener un valor por defecto que depende de un parámetro
> anterior (mira [las funciones](fonctions.md)).

> **Solución**
>
> ```typescript
> function area(ancho: number, alto: number = ancho): number {
>   return ancho * alto;
> }
>
> console.log(area(4, 3));
> console.log(area(5));
> ```
>
> ```sortie
> 12
> 25
> ```

## Texto, arrays y objetos

### Ejercicio 5: Contar las vocales

Nivel: ★★☆

Escribe una función `vocales(texto: string): number` que cuente las vocales (a, e, i,
o, u, y) de un texto, tanto en mayúsculas como en minúsculas. Pruébala con
`"TypeScript es genial"`.

> **Pista:** `toLowerCase()`, luego recorre las letras con `for (const c of texto)` y
> comprueba `"aeiouy".includes(c)` (mira [el texto](texte.md)).

> **Solución**
>
> ```typescript
> function vocales(texto: string): number {
>   let total = 0;
>   for (const c of texto.toLowerCase()) {
>     if ("aeiouy".includes(c)) total++;
>   }
>   return total;
> }
>
> console.log(vocales("TypeScript es genial"));
> ```
>
> ```sortie
> 7
> ```

### Ejercicio 6: La media

Nivel: ★★☆

Escribe una función `media(notas: readonly number[])` que devuelva la media, o
`undefined` si el array está vacío. Muestra la media de `[12, 15, 9, 14]` y luego la
de `[]`, escribiendo `ninguna nota` en ese caso.

> **Pista:** el tipo de retorno es `number | undefined`. En la llamada, `??` da un
> valor de reemplazo.

> **Solución**
>
> ```typescript
> function media(notas: readonly number[]): number | undefined {
>   if (notas.length === 0) return undefined;
>   return notas.reduce((a, b) => a + b, 0) / notas.length;
> }
>
> console.log(media([12, 15, 9, 14]) ?? "ninguna nota");
> console.log(media([]) ?? "ninguna nota");
> ```
>
> ```sortie
> 12.5
> ninguna nota
> ```
>
> El tipo de retorno obliga a quien llama a prever el caso vacío:
> `media([]).toFixed(1)` sería rechazado por `tsc`.

### Ejercicio 7: La cesta

Nivel: ★★☆

Define una interfaz `Articulo` con un `nombre`, un `precio` unitario y una `cantidad`.
Escribe `total(cesta: readonly Articulo[]): number` y luego muestra el total de esta
cesta con dos decimales: 3 cruasanes a 1,20 €, 1 barra de pan a 1,10 €.

> **Pista:** `reduce` suma `precio * cantidad`; `toFixed(2)` da formato (mira
> [los objetos](objets.md)).

> **Solución**
>
> ```typescript
> interface Articulo {
>   nombre: string;
>   precio: number;
>   cantidad: number;
> }
>
> function total(cesta: readonly Articulo[]): number {
>   return cesta.reduce((suma, a) => suma + a.precio * a.cantidad, 0);
> }
>
> const cesta: Articulo[] = [
>   { nombre: "cruasán", precio: 1.2, cantidad: 3 },
>   { nombre: "barra de pan", precio: 1.1, cantidad: 1 },
> ];
> console.log(`Total: ${total(cesta).toFixed(2)} €`);
> ```
>
> ```sortie
> Total: 4.70 €
> ```

### Ejercicio 8: Las formas

Nivel: ★★☆

Una forma es o bien un círculo (con un `radio`), o bien un rectángulo (con un `ancho`
y un `alto`). Escribe el tipo `Forma` y una función `area(f: Forma): number`. Muestra
el área de un círculo de radio 1 (dos decimales) y de un rectángulo de 2 × 3.

> **Pista:** una unión discriminada, con una propiedad `tipo` común, y un `switch`
> (mira [las uniones](unions.md#las-uniones-discriminadas)).

> **Solución**
>
> ```typescript
> type Forma =
>   | { tipo: "circulo"; radio: number }
>   | { tipo: "rectangulo"; ancho: number; alto: number };
>
> function area(f: Forma): number {
>   switch (f.tipo) {
>     case "circulo":
>       return Math.PI * f.radio ** 2;
>     case "rectangulo":
>       return f.ancho * f.alto;
>   }
> }
>
> console.log(area({ tipo: "circulo", radio: 1 }).toFixed(2));
> console.log(area({ tipo: "rectangulo", ancho: 2, alto: 3 }));
> ```
>
> ```sortie
> 3.14
> 6
> ```

## Tipos más ricos

### Ejercicio 9: El último elemento

Nivel: ★★☆

Escribe una función **genérica** `ultimo` que devuelva el último elemento de un array
de cualquier tipo, o `undefined` si está vacío. El tipo de retorno debe seguir el tipo
del array: `ultimo(["a", "b"])` es un `string | undefined`.

> **Pista:** `function ultimo<T>(lista: readonly T[]): T | undefined` (mira
> [los genéricos](generiques.md)).

> **Solución**
>
> ```typescript
> function ultimo<T>(lista: readonly T[]): T | undefined {
>   return lista[lista.length - 1];
> }
>
> const palabra = ultimo(["manzana", "pera", "kiwi"]);
> const numero = ultimo([1, 2, 3]);
> console.log(palabra?.toUpperCase());
> console.log(numero);
> console.log(ultimo([]));
> ```
>
> ```sortie
> KIWI
> 3
> undefined
> ```
>
> Gracias a `T`, TypeScript sabe que `palabra` es una cadena: `toUpperCase` está
> permitido. El operador `?.` gestiona el caso `undefined`.

### Ejercicio 10: El semáforo

Nivel: ★★★

Define el tipo `Luz`: `"verde"`, `"amarillo"` o `"rojo"`. Escribe
`siguiente(luz: Luz): Luz` (verde → amarillo → rojo → verde). Añade una comprobación de
exhaustividad: si algún día se añade un color al tipo, `tsc` debe señalar el olvido.
Muestra cuatro cambios a partir de `"verde"`.

> **Pista:** en el `default` del `switch`, guarda el valor en una variable de tipo
> `never` (mira [los tipos literales](litteraux.md) y
> [las uniones](unions.md#comprobar-todos-los-casos-con-never)).

> **Solución**
>
> ```typescript
> type Luz = "verde" | "amarillo" | "rojo";
>
> function siguiente(luz: Luz): Luz {
>   switch (luz) {
>     case "verde":
>       return "amarillo";
>     case "amarillo":
>       return "rojo";
>     case "rojo":
>       return "verde";
>     default: {
>       const imposible: never = luz;
>       throw new Error(`Luz desconocida: ${imposible}`);
>     }
>   }
> }
>
> let luz: Luz = "verde";
> const etapas: Luz[] = [luz];
> for (let i = 0; i < 4; i++) {
>   luz = siguiente(luz);
>   etapas.push(luz);
> }
> console.log(etapas.join(" -> "));
> ```
>
> ```sortie
> verde -> amarillo -> rojo -> verde -> amarillo
> ```
>
> Añade `"intermitente"` al tipo `Luz`: la línea `const imposible: never = luz;` se
> convierte en un error, porque `luz` todavía puede valer `"intermitente"` en ese punto.

### Ejercicio 11: La cuenta bancaria

Nivel: ★★★

Escribe una clase `Cuenta` con un `titular` de solo lectura y un saldo **privado**, que
empieza en 0. Tiene tres métodos: `depositar(cantidad)`, `retirar(cantidad)` y
`saldo()`. Un depósito o una retirada negativos, o una retirada mayor que el saldo,
lanzan un `Error`. Haz un depósito de 100, una retirada de 30 y luego intenta una
retirada de 500 capturando el error.

> **Pista:** `private` o `#saldo` oculta el saldo; `throw new Error("…")` lanza un
> error; en el `catch`, comprueba `e instanceof Error` antes de leer `e.message`
> (mira [los errores](erreurs.md)).

> **Solución**
>
> ```typescript
> class Cuenta {
>   #saldo = 0;
>
>   constructor(readonly titular: string) {}
>
>   depositar(cantidad: number): void {
>     if (cantidad <= 0) throw new Error("El depósito debe ser positivo");
>     this.#saldo += cantidad;
>   }
>
>   retirar(cantidad: number): void {
>     if (cantidad <= 0) throw new Error("La retirada debe ser positiva");
>     if (cantidad > this.#saldo) throw new Error(`Saldo insuficiente (${this.#saldo} €)`);
>     this.#saldo -= cantidad;
>   }
>
>   saldo(): number {
>     return this.#saldo;
>   }
> }
>
> const cuenta = new Cuenta("Alice");
> cuenta.depositar(100);
> cuenta.retirar(30);
> console.log(`${cuenta.titular}: ${cuenta.saldo()} €`);
>
> try {
>   cuenta.retirar(500);
> } catch (e) {
>   console.log(e instanceof Error ? `Rechazado: ${e.message}` : "Error desconocido");
> }
> console.log(`${cuenta.titular}: ${cuenta.saldo()} €`);
> ```
>
> ```sortie
> Alice: 70 €
> Rechazado: Saldo insuficiente (70 €)
> Alice: 70 €
> ```
>
> `#saldo` es realmente privado, incluso en tiempo de ejecución: `cuenta.#saldo` es
> imposible fuera de la clase. El saldo solo cambia a través de los métodos, que lo
> comprueban todo.

### Ejercicio 12: Cargar perfiles

Nivel: ★★★

Aquí tienes una función asíncrona que simula un servidor lento:

```typescript norun
const PERFILES: Record<number, string> = { 1: "Alice", 2: "Bob" };

function cargarNombre(id: number): Promise<string> {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      const nombre = PERFILES[id];
      if (nombre === undefined) reject(new Error(`Perfil ${id} no encontrado`));
      else resolve(nombre);
    }, 10);
  });
}
```

Escribe una función `async function cargarTodos(ids: number[])` que cargue todos los
perfiles **a la vez**, y devuelva para cada uno un resultado:
`{ ok: true; nombre: string }` o `{ ok: false; error: string }`. Muestra los resultados
para los números 1, 3 y 2, en este orden.

> **Pista:** `Promise.allSettled` espera todas las promesas, cumplidas o no, y conserva
> el orden. Cada respuesta tiene un `status`: `"fulfilled"` (con `value`) o
> `"rejected"` (con `reason`) (mira [la asincronía](asynchrone.md)).

> **Solución**
>
> ```typescript
> type Resultado = { ok: true; nombre: string } | { ok: false; error: string };
>
> const PERFILES: Record<number, string> = { 1: "Alice", 2: "Bob" };
>
> function cargarNombre(id: number): Promise<string> {
>   return new Promise((resolve, reject) => {
>     setTimeout(() => {
>       const nombre = PERFILES[id];
>       if (nombre === undefined) reject(new Error(`Perfil ${id} no encontrado`));
>       else resolve(nombre);
>     }, 10);
>   });
> }
>
> async function cargarTodos(ids: number[]): Promise<Resultado[]> {
>   const respuestas = await Promise.allSettled(ids.map(cargarNombre));
>   return respuestas.map((r): Resultado => {
>     if (r.status === "fulfilled") return { ok: true, nombre: r.value };
>     const mensaje = r.reason instanceof Error ? r.reason.message : String(r.reason);
>     return { ok: false, error: mensaje };
>   });
> }
>
> for (const r of await cargarTodos([1, 3, 2])) {
>   console.log(r.ok ? `OK: ${r.nombre}` : `Fallo: ${r.error}`);
> }
> ```
>
> ```sortie
> OK: Alice
> Fallo: Perfil 3 no encontrado
> OK: Bob
> ```
>
> Con `Promise.all`, el primer fallo lo habría detenido todo. `allSettled` deja que
> cada carga llegue hasta el final. El tipo de retorno anotado `(r): Resultado` ayuda a
> TypeScript a reconocer la unión discriminada.

## En resumen

- Empieza por los ejercicios ★☆☆ y luego sube de dificultad.
- Inténtalo siempre por tu cuenta, y comprueba tus tipos con `tsc --strict` antes de
  abrir la solución.
- Los ejercicios 1 a 4 repasan lo básico: constantes, condiciones, bucles, funciones.
- Los ejercicios 5 a 8 trabajan el texto, los arrays, las interfaces y las uniones.
- Los ejercicios 9 a 12 usan los genéricos, los literales, las clases, los errores y la
  asincronía.

Siguiente: [ejercicios resueltos: segunda serie](exercices-2.md)
