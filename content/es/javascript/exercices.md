# Ejercicios resueltos

Esta página propone 12 ejercicios resueltos, del más fácil al más difícil, para practicar todo el recorrido de JavaScript.

## Cómo usar esta página

Cada ejercicio tiene un nivel:

- ★☆☆: fácil;
- ★★☆: medio;
- ★★★: más difícil.

Lee el enunciado, luego escribe tu solución en un archivo `.mjs` y ejecútalo con
`node archivo.mjs`. Abre la solución solo después de haberlo intentado.

> **Consejo:** a menudo hay varias soluciones buenas. Si tu programa muestra el
> resultado correcto, ¡lo has conseguido!

## Lo básico

### Ejercicio 1: Presentarse

Nivel: ★☆☆

Crea dos constantes, `nombre` y `edad`. Después muestra la frase
`Me llamo Alice y tengo 30 años.`, con tus constantes.

> **Pista:** usa una plantilla `` `…${nombre}…` `` (mira
> [los primeros pasos](premiers-pas.md)).

> **Solución**
>
> ```javascript
> const nombre = "Alice";
> const edad = 30;
> console.log(`Me llamo ${nombre} y tengo ${edad} años.`);
> ```
>
> ```sortie
> Me llamo Alice y tengo 30 años.
> ```

### Ejercicio 2: Par o impar

Nivel: ★☆☆

Para cada uno de los números `7` y `10`, muestra `7 es impar` o `10 es par`.

> **Pista:** `n % 2` da el resto de la división entre 2: `0` para un número par.

> **Solución**
>
> ```javascript
> for (const n of [7, 10]) {
>   if (n % 2 === 0) {
>     console.log(`${n} es par`);
>   } else {
>     console.log(`${n} es impar`);
>   }
> }
> ```
>
> ```sortie
> 7 es impar
> 10 es par
> ```

### Ejercicio 3: FizzBuzz

Nivel: ★☆☆

Muestra los números del 1 al 15. Pero en lugar de los múltiplos de 3, muestra `Fizz`;
en lugar de los múltiplos de 5, `Buzz`; y de los múltiplos de ambos, `FizzBuzz`.
Muestra todo en una línea, separado por espacios.

> **Pista:** comprueba primero el caso «múltiplo de 15», si no, nunca se alcanzará
> (mira [condiciones y bucles](conditions-boucles.md)).

> **Solución**
>
> ```javascript
> const resultados = [];
> for (let i = 1; i <= 15; i++) {
>   if (i % 15 === 0) {
>     resultados.push("FizzBuzz");
>   } else if (i % 3 === 0) {
>     resultados.push("Fizz");
>   } else if (i % 5 === 0) {
>     resultados.push("Buzz");
>   } else {
>     resultados.push(i);
>   }
> }
> console.log(resultados.join(" "));
> ```
>
> ```sortie
> 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz
> ```

### Ejercicio 4: Convertir temperaturas

Nivel: ★☆☆

Escribe una función flecha `aFahrenheit(celsius)` que devuelva
`celsius * 9 / 5 + 32`. Muestra la conversión de `0`, `37` y `100` grados.

> **Pista:** una función flecha se escribe `const f = (x) => …;` (mira
> [las funciones](fonctions.md)).

> **Solución**
>
> ```javascript
> const aFahrenheit = (celsius) => celsius * 9 / 5 + 32;
>
> for (const c of [0, 37, 100]) {
>   console.log(`${c} °C = ${aFahrenheit(c).toFixed(1)} °F`);
> }
> ```
>
> ```sortie
> 0 °C = 32.0 °F
> 37 °C = 98.6 °F
> 100 °C = 212.0 °F
> ```

## Texto y colecciones

### Ejercicio 5: Contar las vocales

Nivel: ★★☆

Escribe una función `contarVocales(texto)` que devuelva el número de vocales (`a`,
`e`, `i`, `o`, `u`, `y`) de un texto, mayúsculas incluidas. Pruébala con
`"JavaScript"`.

> **Pista:** `[...texto]` da un array de caracteres; `"aeiouy".includes(c)` dice si
> `c` es una vocal (mira [el texto](texte.md)).

> **Solución**
>
> ```javascript
> function contarVocales(texto) {
>   return [...texto.toLowerCase()].filter((c) => "aeiouy".includes(c)).length;
> }
>
> console.log(contarVocales("JavaScript"));
> console.log(contarVocales("Buenos"));
> ```
>
> ```sortie
> 3
> 3
> ```

### Ejercicio 6: El palíndromo

Nivel: ★★☆

Un palíndromo se lee igual en los dos sentidos, como `kayak` o `Ana`. Escribe
`esPalindromo(palabra)` que ignore las mayúsculas. Prueba `"kayak"`, `"Radar"` y
`"código"`.

> **Pista:** invierte la palabra con `[...palabra].reverse().join("")`.

> **Solución**
>
> ```javascript
> function esPalindromo(palabra) {
>   const p = palabra.toLowerCase();
>   return p === [...p].reverse().join("");
> }
>
> for (const palabra of ["kayak", "Radar", "código"]) {
>   console.log(`${palabra}: ${esPalindromo(palabra)}`);
> }
> ```
>
> ```sortie
> kayak: true
> Radar: true
> código: false
> ```

### Ejercicio 7: Contar las palabras

Nivel: ★★☆

En la frase `"el gato y el perro y el conejo"`, cuenta cuántas veces aparece cada
palabra. Muestra las palabras de la más frecuente a la menos frecuente.

> **Pista:** un objeto sirve de contador: `cuenta[palabra] = (cuenta[palabra] ?? 0) + 1`.
> `Object.entries` lo convierte en un array de pares, que se puede ordenar (mira
> [los objetos](objets.md)).

> **Solución**
>
> ```javascript
> const frase = "el gato y el perro y el conejo";
> const cuenta = {};
> for (const palabra of frase.split(" ")) {
>   cuenta[palabra] = (cuenta[palabra] ?? 0) + 1;
> }
>
> const ordenadas = Object.entries(cuenta).toSorted((a, b) => b[1] - a[1]);
> for (const [palabra, numero] of ordenadas) {
>   console.log(`${palabra}: ${numero}`);
> }
> ```
>
> ```sortie
> el: 3
> y: 2
> gato: 1
> perro: 1
> conejo: 1
> ```
>
> En caso de empate, `toSorted` mantiene el orden original: `gato`, `perro` y luego
> `conejo`.

### Ejercicio 8: La cesta

Nivel: ★★☆

Esta es una cesta de la compra:

```javascript norun
const cesta = [
  { nombre: "Libro", precio: 12, cantidad: 2 },
  { nombre: "Bolígrafo", precio: 1.5, cantidad: 4 },
  { nombre: "Bolsa", precio: 30, cantidad: 1 },
];
```

Con `filter`, `map` y `reduce`, muestra los nombres de los artículos de menos de 20 €
y luego el total de la cesta.

> **Pista:** el total de una línea es `precio * cantidad`; `reduce` los suma empezando
> desde `0` (mira [las colecciones](collections.md)).

> **Solución**
>
> ```javascript
> const cesta = [
>   { nombre: "Libro", precio: 12, cantidad: 2 },
>   { nombre: "Bolígrafo", precio: 1.5, cantidad: 4 },
>   { nombre: "Bolsa", precio: 30, cantidad: 1 },
> ];
>
> const baratos = cesta.filter((a) => a.precio < 20).map((a) => a.nombre);
> console.log(baratos.join(", "));
>
> const total = cesta.reduce((suma, a) => suma + a.precio * a.cantidad, 0);
> console.log(`Total: ${total.toFixed(2)} €`);
> ```
>
> ```sortie
> Libro, Bolígrafo
> Total: 60.00 €
> ```

## Objetos, clases y funciones

### Ejercicio 9: Un contador privado

Nivel: ★★☆

Escribe una función `crearContador()` que devuelva un objeto con dos métodos:
`incrementar()`, que suma 1 y devuelve el nuevo valor, y `valor()`. La variable de la
cuenta no debe ser accesible desde fuera. Dos contadores deben ser independientes.

> **Pista:** es una [closure](../debuter/glossaire.md#closure): los métodos recuerdan
> una variable `let` declarada en `crearContador` (mira [las closures](closures.md)).

> **Solución**
>
> ```javascript
> function crearContador() {
>   let cuenta = 0;
>   return {
>     incrementar() {
>       cuenta += 1;
>       return cuenta;
>     },
>     valor() {
>       return cuenta;
>     },
>   };
> }
>
> const a = crearContador();
> const b = crearContador();
> a.incrementar();
> a.incrementar();
> b.incrementar();
> console.log(a.valor(), b.valor(), a.cuenta);
> ```
>
> ```sortie
> 2 1 undefined
> ```

### Ejercicio 10: La cuenta bancaria

Nivel: ★★★

Escribe una clase `CuentaBancaria` con un titular y un saldo **privado** (`#saldo`).
Tiene tres métodos: `depositar(importe)`, `retirar(importe)` y un getter `saldo`.
`retirar` lanza un `RangeError` si el saldo no alcanza; `depositar` lanza uno si el
importe no es positivo.

> **Pista:** un campo que empieza por `#` solo se puede leer dentro de la clase;
> `get saldo()` crea un getter de solo lectura (mira [la herencia](heritage.md) y
> [los errores](erreurs.md)).

> **Solución**
>
> ```javascript
> class CuentaBancaria {
>   #saldo = 0;
>
>   constructor(titular) {
>     this.titular = titular;
>   }
>
>   get saldo() {
>     return this.#saldo;
>   }
>
>   depositar(importe) {
>     if (importe <= 0) throw new RangeError("importe no válido");
>     this.#saldo += importe;
>   }
>
>   retirar(importe) {
>     if (importe > this.#saldo) throw new RangeError("saldo insuficiente");
>     this.#saldo -= importe;
>   }
> }
>
> const cuenta = new CuentaBancaria("Alice");
> cuenta.depositar(100);
> cuenta.retirar(30);
> console.log(`${cuenta.titular}: ${cuenta.saldo} €`);
>
> try {
>   cuenta.retirar(500);
> } catch (error) {
>   console.log(`Rechazado: ${error.message}`);
> }
> console.log(`Saldo sin cambios: ${cuenta.saldo} €`);
> ```
>
> ```sortie
> Alice: 70 €
> Rechazado: saldo insuficiente
> Saldo sin cambios: 70 €
> ```

## Para ir más lejos

### Ejercicio 11: Un generador de Fibonacci

Nivel: ★★★

La sucesión de Fibonacci empieza por `0, 1`, y luego cada número es la suma de los dos
anteriores: `0, 1, 1, 2, 3, 5, 8…`. Escribe un generador `fibonacci()` sin fin, y una
función `tomar(iterable, n)` que devuelva los `n` primeros valores en un array.
Muestra los 10 primeros números.

> **Pista:** un generador se escribe `function*` y produce sus valores con `yield`
> (mira [los iteradores](iterateurs.md)). En `tomar`, detén el bucle `for…of` con
> `break`.

> **Solución**
>
> ```javascript
> function* fibonacci() {
>   let [a, b] = [0, 1];
>   while (true) {
>     yield a;
>     [a, b] = [b, a + b];
>   }
> }
>
> function tomar(iterable, n) {
>   const resultado = [];
>   if (n <= 0) return resultado;
>   for (const valor of iterable) {
>     resultado.push(valor);
>     if (resultado.length === n) break;
>   }
>   return resultado;
> }
>
> console.log(tomar(fibonacci(), 10).join(", "));
> ```
>
> ```sortie
> 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
> ```

### Ejercicio 12: Cargar en paralelo

Nivel: ★★★

Una función simula la carga de una nota desde un servidor lento:

```javascript norun
function cargarNota(alumno) {
  const notas = { Alice: 15, Bob: 9, Chloé: 12 };
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      if (alumno in notas) resolve(notas[alumno]);
      else reject(new Error(`alumno desconocido: ${alumno}`));
    }, 50);
  });
}
```

1. Carga las notas de Alice, Bob y Chloé **a la vez** con `Promise.all`, y muestra su
   media.
2. Después carga la nota de `"Zoé"` capturando el error con `try … catch`.

> **Pista:** `await Promise.all(alumnos.map(cargarNota))` espera todas las promesas a
> la vez (mira [lo asíncrono](asynchrone.md)).

> **Solución**
>
> ```javascript
> function cargarNota(alumno) {
>   const notas = { Alice: 15, Bob: 9, Chloé: 12 };
>   return new Promise((resolve, reject) => {
>     setTimeout(() => {
>       if (alumno in notas) resolve(notas[alumno]);
>       else reject(new Error(`alumno desconocido: ${alumno}`));
>     }, 50);
>   });
> }
>
> const alumnos = ["Alice", "Bob", "Chloé"];
> const notas = await Promise.all(alumnos.map(cargarNota));
> const media = notas.reduce((s, n) => s + n, 0) / notas.length;
> console.log(`Notas: ${notas.join(", ")}`);
> console.log(`Media: ${media}`);
>
> try {
>   await cargarNota("Zoé");
> } catch (error) {
>   console.log(`Error: ${error.message}`);
> }
> ```
>
> ```sortie
> Notas: 15, 9, 12
> Media: 12
> Error: alumno desconocido: Zoé
> ```
>
> Las tres cargas se hacen a la vez: todo tarda unos 50 ms, y no 150 ms.
> `Promise.all` mantiene el orden del array de partida.

## En resumen

- Empieza por los ejercicios ★☆☆ y luego sube de dificultad.
- Inténtalo siempre solo antes de abrir la solución.
- Los ejercicios 1 a 4 repasan lo básico: constantes, condiciones, bucles, funciones.
- Los ejercicios 5 a 8 trabajan el texto, los arrays, los objetos y `map`, `filter`,
  `reduce`.
- Los ejercicios 9 a 12 lo combinan todo: closures, clases, errores, generadores y
  promesas.

Siguiente: [ejercicios resueltos: segunda serie](exercices-2.md)
