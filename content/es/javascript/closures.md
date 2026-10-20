# Ámbito y closures

Esta página explica dónde es visible una variable, cómo una función recuerda sus variables (una closure) y qué vale `this` según la forma de escribir una función.

> **Objetivos:**
>
> - sabrás dónde es visible una variable declarada con `let` o `const`;
> - entenderás que una función recuerda las variables que la rodean;
> - sabrás crear funciones con un estado privado;
> - sabrás evitar la trampa de `var` en un bucle;
> - sabrás qué vale `this` en una función flecha y con `bind`.

## El ámbito de una variable

El [ámbito](../debuter/glossaire.md#ambito) de una variable es la zona del programa donde
existe. Con `let` y `const`, esa zona es el **bloque**: todo lo que está entre las llaves
`{ }` donde se declara la variable.

Piensa en las habitaciones de una casa: lo que se guarda en la cocina no se ve desde el
salón. Pero desde la cocina sí se ve lo que hay en el pasillo común.

```javascript
const ciudad = "Lyon";

if (true) {
  const barrio = "Croix-Rousse";
  console.log(ciudad, barrio);
}

console.log(ciudad);
console.log(typeof barrio);
```

```sortie
Lyon Croix-Rousse
Lyon
undefined
```

- `ciudad` se declara arriba del todo: es visible en todas partes.
- `barrio` se declara en el bloque del `if`: desaparece en la llave de cierre.

> **Nota:** `typeof` es la única forma de tocar una variable inexistente sin error:
> responde `"undefined"`. Escribir directamente `barrio` lanzaría un `ReferenceError`.

### Los bloques se anidan

Un bloque interior ve las variables de los bloques que lo rodean. Al revés, no.
Una función también tiene su propio ámbito: sus parámetros y sus variables se quedan
dentro.

```javascript
const tasa = 0.2;

function precioConIva(precioSinIva) {
  const iva = precioSinIva * tasa;
  return precioSinIva + iva;
}

console.log(precioConIva(100));
console.log(typeof iva);
```

```sortie
120
undefined
```

`precioConIva` lee `tasa`, declarada fuera. Pero `iva` solo existe durante la llamada.

### El ocultamiento

Una variable interior puede llevar el mismo nombre que una variable exterior. La
**oculta**: dentro del bloque, es la que se ve.

```javascript
const nombre = "exterior";

{
  const nombre = "interior";
  console.log(nombre);
}

console.log(nombre);
```

```sortie
interior
exterior
```

> **Consejo:** evita reutilizar los mismos nombres: el código es más fácil de leer.

## Una función recuerda sus variables

Esta es la idea central de este capítulo. Una función creada **dentro** de otra función
conserva el acceso a las variables que la rodeaban, **incluso después** de que la función
exterior haya terminado.

Esta pareja «función + variables que recuerda» se llama
[closure](../debuter/glossaire.md#closure) («clausura» en español). Imagina una mochila:
al salir de casa, la función se lleva las variables que necesita.

```javascript
function crearSaludo(formula) {
  return function (nombre) {
    return `¡${formula}, ${nombre}!`;
  };
}

const hola = crearSaludo("Hola");
const buenas = crearSaludo("Buenas");

console.log(hola("Ada"));
console.log(buenas("Linus"));
```

```sortie
¡Hola, Ada!
¡Buenas, Linus!
```

Cuando llamas a `hola("Ada")`, `crearSaludo` terminó hace mucho. Sin embargo, la función
recuerda que `formula` vale `"Hola"`. Cada llamada a `crearSaludo` crea una mochila
nueva: `hola` y `buenas` tienen cada una la suya.

## Una fábrica de contadores

La mochila no contiene una copia congelada: la función puede **modificar** las variables
que recuerda. Así se puede crear un contador.

```javascript
function crearContador() {
  let cuenta = 0;
  return () => {
    cuenta += 1;
    return cuenta;
  };
}

const contadorA = crearContador();
const contadorB = crearContador();

console.log(contadorA());
console.log(contadorA());
console.log(contadorA());
console.log(contadorB());
```

```sortie
1
2
3
1
```

`contadorA` y `contadorB` tienen cada uno su propia variable `cuenta`. Llamar a uno no
cambia el otro.

## Un estado privado

Nadie, aparte de la función, puede tocar `cuenta`: no hay ninguna forma de escribir
`contadorA.cuenta`. La closure da un **estado privado**, como los campos `#` de una
[clase](types.md#los-campos-privados).

Puedes devolver varias funciones en un objeto; comparten la misma mochila.

```javascript
function crearHucha() {
  let saldo = 0;

  return {
    ingresar(cantidad) {
      if (cantidad > 0) {
        saldo += cantidad;
      }
    },
    retirar(cantidad) {
      if (cantidad > saldo) {
        console.log("No hay suficiente dinero.");
        return;
      }
      saldo -= cantidad;
    },
    verSaldo() {
      return saldo;
    },
  };
}

const hucha = crearHucha();
hucha.ingresar(20);
hucha.retirar(50);
hucha.retirar(5);
console.log(hucha.verSaldo());
console.log(hucha.saldo);
```

```sortie
No hay suficiente dinero.
15
undefined
```

`hucha.saldo` vale `undefined`: el saldo está bien escondido. Para cambiarlo, hay que
pasar por `ingresar` y `retirar`, que comprueban las cantidades.

### Recordar para ir más rápido

Una closure también puede guardar una **caché**: la función memoriza los resultados ya
calculados.

```javascript
function crearCuadrado() {
  const conocidos = new Map();
  return (n) => {
    if (conocidos.has(n)) {
      console.log(`(${n} ya conocido)`);
      return conocidos.get(n);
    }
    const resultado = n * n;
    conocidos.set(n, resultado);
    return resultado;
  };
}

const cuadrado = crearCuadrado();
console.log(cuadrado(9));
console.log(cuadrado(9));
```

```sortie
81
(9 ya conocido)
81
```

## La trampa de var en un bucle

Antes de `let` y `const`, las variables se declaraban con `var`. Lo verás en código
antiguo. `var` **no** tiene ámbito de bloque: la variable existe en toda la función. En
un bucle, esto da una sola variable compartida por todas las vueltas.

```javascript
const funcionesVar = [];
for (var i = 0; i < 3; i++) {
  funcionesVar.push(() => i);
}
console.log(funcionesVar.map((f) => f()));

const funcionesLet = [];
for (let j = 0; j < 3; j++) {
  funcionesLet.push(() => j);
}
console.log(funcionesLet.map((f) => f()));
```

```sortie
[ 3, 3, 3 ]
[ 0, 1, 2 ]
```

Con `var`, las tres funciones recuerdan **la misma** variable `i`. Cuando las llamas, el
bucle ha terminado e `i` vale `3`. Con `let`, cada vuelta del bucle crea una variable `j`
**nueva**: cada función guarda la suya.

> **Recuerda:** no uses nunca `var`. Escribe `const` por defecto, y `let` cuando el valor
> tenga que cambiar.

## this en las funciones

En un método, `this` designa el objeto sobre el que se llama. Pero cuidado: en una
función escrita con `function`, `this` depende de **la forma en que se llama**, no del
lugar donde está escrita.

Una función flecha `=>` no tiene `this` propio: usa el del lugar donde está escrita, como
cualquier otra variable de su mochila.

```javascript
class Temporizador {
  constructor(nombre) {
    this.nombre = nombre;
  }

  conFlecha() {
    [1, 2].forEach((n) => {
      console.log(`${this.nombre}: ${n}`);
    });
  }

  conFunction() {
    [1, 2].forEach(function (n) {
      console.log(`${this === undefined ? "this perdido" : this.nombre}: ${n}`);
    });
  }
}

const t = new Temporizador("Tic");
t.conFlecha();
t.conFunction();
```

```sortie
Tic: 1
Tic: 2
this perdido: 1
this perdido: 2
```

En `conFunction`, la función la llama `forEach`, no el objeto: su `this` es `undefined`.

> **Consejo:** para una función que se pasa a otra (`forEach`, `map`, `setTimeout`…),
> usa una función flecha. Para un método de objeto o de clase, usa la forma normal
> `metodo() { }`.

## Fijar this con bind

El mismo problema aparece cuando **separas** un método de su objeto, por ejemplo para
guardarlo en una variable. `bind(objeto)` crea una función nueva cuyo `this` queda fijado
de una vez por todas.

```javascript
const gato = {
  nombre: "Michi",
  presentar() {
    return `Soy ${this.nombre}`;
  },
};

console.log(gato.presentar());

const separada = gato.presentar;
console.log(separada === gato.presentar);

const atada = gato.presentar.bind(gato);
console.log(atada());
```

```sortie
Soy Michi
true
Soy Michi
```

`separada()` daría un error: llamada sola, sin `gato.` delante, ya no tiene `this`.
`atada`, en cambio, recuerda a `gato`.

## Errores frecuentes

### Usar una variable fuera de su bloque

```javascript norun
if (true) {
  let mensaje = "hola";
}
console.log(mensaje);
```

```text
file:///home/vous/projet/main.mjs:4
console.log(mensaje);
            ^

ReferenceError: mensaje is not defined
    at file:///home/vous/projet/main.mjs:4:13
```

`mensaje` solo existe en el bloque del `if`. Declárala **antes** del bloque
(`let mensaje;`) y dale su valor dentro.

### Usar una variable antes de declararla

```javascript norun
console.log(total);
let total = 10;
```

```text
file:///home/vous/projet/main.mjs:1
console.log(total);
            ^

ReferenceError: Cannot access 'total' before initialization
```

El mensaje dice: «no se puede acceder a `total` antes de su inicialización». Declara
siempre una variable antes de usarla.

### Perder this al separar un método

```javascript norun
const gato = { nombre: "Michi", presentar() { return "Soy " + this.nombre; } };
const f = gato.presentar;
console.log(f());
```

```text
file:///home/vous/projet/main.mjs:1
const gato = { nombre: "Michi", presentar() { return "Soy " + this.nombre; } };
                                                                   ^

TypeError: Cannot read properties of undefined (reading 'nombre')
    at presentar (file:///home/vous/projet/main.mjs:1:68)
    at file:///home/vous/projet/main.mjs:3:13
```

`f()` se llama sin ningún objeto delante: `this` vale `undefined`. Llama a
`gato.presentar()`, o fija `this` con `gato.presentar.bind(gato)`. El mismo mensaje
aparece con una `function` pasada a `forEach`: usa una función flecha.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```javascript norun
function crear() {
  let n = 10;
  return () => (n -= 1);
}
const a = crear();
const b = crear();
a();
a();
console.log(a(), b());
```

> **Respuesta**
>
> `7 9`. `a` y `b` tienen cada uno su propio `n`. Tras tres llamadas, el de `a` vale `7`;
> el de `b` se ha reducido una sola vez: `9`.

**Pregunta 2:** ¿por qué un bucle `for (var i ...)` que crea funciones da a menudo el
mismo valor en todas partes?

> **Respuesta**
>
> Porque `var` crea **una sola** variable para todo el bucle. Todas las funciones la
> recuerdan y la leen cuando el bucle ha terminado. Con `let`, cada vuelta tiene su
> propia variable.

**Pregunta 3:** escribe una función `crearAcumulador()` que devuelva una función: cada
llamada `sumar(x)` suma `x` a un total escondido y devuelve el nuevo total.

> **Respuesta**
>
> ```javascript
> function crearAcumulador() {
>   let total = 0;
>   return (x) => {
>     total += x;
>     return total;
>   };
> }
>
> const sumar = crearAcumulador();
> sumar(5);
> sumar(10);
> console.log(sumar(1));
> ```
>
> ```sortie
> 16
> ```

## En resumen

- Una variable `let` o `const` solo existe en su bloque `{ }`; un bloque interior ve las
  variables exteriores.
- Una closure es una función que recuerda las variables que la rodeaban.
- Una fábrica (`crearContador`) da a cada función su propio estado privado.
- `var` no tiene ámbito de bloque: no lo uses.
- Una función flecha conserva el `this` del lugar donde está escrita; `bind` fija el
  `this` de una función.

Siguiente: [iteradores y generadores](iterateurs.md). Compara con
[las closures en Rust](../rust/fonctions.md).
