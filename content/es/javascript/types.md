# Las clases

Esta página muestra cómo crear tus propios objetos con `class`: constructor, campos, métodos, `this`, accesores, miembros estáticos y campos privados.

> **Objetivos:**
>
> - sabrás escribir una clase y crear objetos con `new`;
> - sabrás guardar datos en un objeto con `constructor` y `this`;
> - sabrás añadir métodos, getters y setters;
> - sabrás ocultar un dato con un campo privado `#`;
> - sabrás comprobar la clase de un objeto con `instanceof`.

## ¿Por qué clases?

Ya sabes crear un objeto con llaves (mira [los objetos](objets.md)):

```javascript
const michi = { nombre: "Michi", edad: 3 };
const felix = { nombre: "Félix", edad: 5 };
console.log(michi.nombre, felix.nombre);
```

```sortie
Michi Félix
```

Es práctico para uno o dos objetos. Pero si tienes cien gatos, copias cien veces la
misma forma, y una errata (`nombr` en lugar de `nombre`) pasa desapercibida.

Una [clase](../debuter/glossaire.md#clase) es un **modelo** para fabricar objetos que
tienen todos la misma forma. Piensa en un molde de pasteles: el molde es la clase, cada
pastel es un [objeto](../debuter/glossaire.md#objeto). Todos tienen la misma forma, pero
cada uno tiene su sabor.

## Definir una clase

La palabra clave `class` define una clase. Por convención, su nombre empieza por
mayúscula. `new` fabrica un objeto nuevo a partir de la clase: una **instancia**.

```javascript
class Gato {
  maullar() {
    console.log("¡Miau!");
  }
}

const michi = new Gato();
michi.maullar();
```

```sortie
¡Miau!
```

`maullar` es un [método](../debuter/glossaire.md#metodo): una función que pertenece a
los objetos de la clase. Fíjate en cómo se escribe: sin la palabra `function`, y sin
coma entre los métodos.

## El constructor y this

`constructor` es un método especial, que `new` llama automáticamente. Prepara el
objeto, como cuando rellenas una ficha al inscribirte. Recibe los argumentos de `new`.

Dentro de un método, `this` designa **el objeto actual**: «yo mismo». `this.nombre =
nombre` quiere decir: «guarda el valor `nombre` en mi propiedad `nombre`».

```javascript
class Gato {
  constructor(nombre, edad) {
    this.nombre = nombre;
    this.edad = edad;
  }

  presentarse() {
    console.log(`Me llamo ${this.nombre} y tengo ${this.edad} años.`);
  }
}

const michi = new Gato("Michi", 3);
const felix = new Gato("Félix", 5);
michi.presentarse();
felix.presentarse();
console.log(michi);
```

```sortie
Me llamo Michi y tengo 3 años.
Me llamo Félix y tengo 5 años.
Gato { nombre: 'Michi', edad: 3 }
```

Cada objeto tiene sus propios datos: `michi.nombre` y `felix.nombre` son distintos. El
método, en cambio, es el mismo para todos; `this` le dice con qué objeto trabajar.

`console.log` muestra el nombre de la clase y luego las propiedades del objeto.

## Los campos

Un **campo** es una propiedad declarada directamente en la clase, con un valor inicial.
Es más legible cuando el valor no depende de los argumentos.

```javascript
class Contador {
  valor = 0;

  incrementar() {
    this.valor += 1;
  }
}

const c = new Contador();
c.incrementar();
c.incrementar();
console.log(c.valor);
```

```sortie
2
```

Los campos se crean **antes** de que se ejecute el constructor. Cada objeto recibe su
propio campo: dos contadores no comparten su `valor`.

## Métodos que calculan

Un método puede devolver un valor con `return` y modificar el objeto. También puede
llamar a otro método del mismo objeto con `this.`.

```javascript
class Carrito {
  articulos = [];

  anadir(nombre, precio) {
    this.articulos.push({ nombre, precio });
    return this;
  }

  total() {
    return this.articulos.reduce((suma, a) => suma + a.precio, 0);
  }

  mostrar() {
    console.log(`${this.articulos.length} artículos, total ${this.total()} €`);
  }
}

const carrito = new Carrito();
carrito.anadir("pan", 2).anadir("leche", 1.5);
carrito.mostrar();
```

```sortie
2 artículos, total 3.5 €
```

> **Consejo:** `anadir` devuelve `this`, el propio objeto. Así puedes encadenar las
> llamadas: `carrito.anadir(...).anadir(...)`.

## Getters y setters

Un **getter** (`get`) es un método que se lee como una propiedad, sin paréntesis. Sirve
para un valor calculado.

Un **setter** (`set`) se llama cuando asignas un valor con `=`. Sirve para comprobar el
valor antes de guardarlo.

```javascript
class Rectangulo {
  constructor(ancho, alto) {
    this.ancho = ancho;
    this.alto = alto;
  }

  get area() {
    return this.ancho * this.alto;
  }

  set lado(valor) {
    this.ancho = valor;
    this.alto = valor;
  }
}

const r = new Rectangulo(3, 4);
console.log(r.area);
r.lado = 5;
console.log(r.ancho, r.alto, r.area);
```

```sortie
12
5 5 25
```

`r.area` se escribe sin paréntesis, pero es código que se ejecuta en cada lectura: el
área siempre está al día.

## Los miembros estáticos

Un miembro `static` pertenece a la **propia clase**, no a sus objetos. Se llama con el
nombre de la clase: `Temperatura.desdeFahrenheit(...)`. Es útil para una constante común
o para otra forma de fabricar un objeto.

```javascript
class Temperatura {
  static CERO_ABSOLUTO = -273.15;

  constructor(celsius) {
    this.celsius = celsius;
  }

  static desdeFahrenheit(f) {
    return new Temperatura(((f - 32) * 5) / 9);
  }
}

console.log(Temperatura.CERO_ABSOLUTO);
const t = Temperatura.desdeFahrenheit(212);
console.log(t.celsius);
```

```sortie
-273.15
100
```

Ya conoces algunos métodos estáticos: `Math.max`, `Number.parseInt`, `Array.isArray`,
`Object.keys`…

## Los campos privados

Por defecto, cualquiera puede leer y modificar las propiedades de un objeto. Para una
cuenta bancaria, es un problema: cualquiera podría escribir `cuenta.saldo = 1000000`.

Un campo cuyo nombre empieza por `#` es **privado**: solo el código escrito dentro de la
clase puede acceder a él. Es como la caja fuerte de un banco: se pasa por la ventanilla
(los métodos), nunca directamente por la caja fuerte.

```javascript
class Cuenta {
  #saldo = 0;

  depositar(cantidad) {
    if (cantidad <= 0) {
      console.log("Cantidad rechazada.");
      return;
    }
    this.#saldo += cantidad;
  }

  get saldo() {
    return this.#saldo;
  }
}

const cuenta = new Cuenta();
cuenta.depositar(50);
cuenta.depositar(-10);
console.log(cuenta.saldo);
try {
  cuenta.saldo = 1000000;
} catch (error) {
  console.log(error.message);
}
```

```sortie
Cantidad rechazada.
50
Cannot set property saldo of #<Cuenta> which has only a getter
```

El getter `saldo` permite **leer** el saldo, pero no hay setter: la asignación
`cuenta.saldo = 1000000` lanza un error («no se puede modificar `saldo`, que solo tiene
un getter»). Solo `depositar` puede cambiar `#saldo`, y comprueba la cantidad. `try` y
`catch` atrapan el error; los verás en detalle en [gestionar los errores](erreurs.md).

Un método también puede ser privado: `#calcularIntereses() { ... }`. Se llama con
`this.#calcularIntereses()`, y solo desde dentro de la clase.

## Mostrar un objeto con toString

Cuando JavaScript tiene que convertir un objeto en texto (en una plantilla `${}` o con
`String(...)`), llama a su método `toString`. Por defecto, da un texto poco útil:
`[object Object]`. Escribe el tuyo.

```javascript
class Punto {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  toString() {
    return `(${this.x}, ${this.y})`;
  }
}

const p = new Punto(2, 5);
console.log(`El punto está en ${p}`);
console.log(String(p));
```

```sortie
El punto está en (2, 5)
(2, 5)
```

> **Nota:** `console.log(p)` no usa `toString`: muestra la estructura del objeto,
> `Punto { x: 2, y: 5 }`. Es práctico para depurar.

## Comprobar la clase con instanceof

`instanceof` responde a la pregunta: «¿este objeto se fabricó con esta clase?». El
resultado es un [booleano](../debuter/glossaire.md#booleano).

```javascript
class Gato {}
class Perro {}

const michi = new Gato();
console.log(michi instanceof Gato);
console.log(michi instanceof Perro);
console.log([1, 2] instanceof Array);
console.log(michi.constructor.name);
console.log(typeof Gato);
```

```sortie
true
false
true
Gato
function
```

`constructor.name` da el nombre de la clase de un objeto. Y `typeof Gato` revela un
secreto: en JavaScript, una clase es una especie de función, la que fabrica los objetos.
El capítulo sobre [la herencia](heritage.md) explica lo que se esconde detrás.

## Errores frecuentes

### Olvidar new

```javascript norun
class Gato {
  constructor(nombre) { this.nombre = nombre; }
}
const g = Gato("Michi");
```

```text
file:///home/vous/projet/main.mjs:4
const g = Gato("Michi");
          ^

TypeError: Class constructor Gato cannot be invoked without 'new'
    at file:///home/vous/projet/main.mjs:4:11
```

Una clase no se llama como una función. Escribe `new Gato("Michi")`.

### Olvidar this en un método

```javascript norun
class Gato {
  constructor(nombre) { this.nombre = nombre; }
  presentarse() { console.log("Soy " + nombre); }
}
new Gato("Michi").presentarse();
```

```text
file:///home/vous/projet/main.mjs:3
  presentarse() { console.log("Soy " + nombre); }
                                       ^

ReferenceError: nombre is not defined
    at Gato.presentarse (file:///home/vous/projet/main.mjs:3:40)
    at file:///home/vous/projet/main.mjs:5:19
```

En `presentarse`, `nombre` a secas es una variable que no existe. La propiedad del
objeto se escribe `this.nombre`.

### Leer un campo privado desde fuera

```javascript norun
class Cuenta {
  #saldo = 0;
}
const c = new Cuenta();
console.log(c.#saldo);
```

```text
file:///home/vous/projet/main.mjs:5
console.log(c.#saldo);
             ^

SyntaxError: Private field '#saldo' must be declared in an enclosing class
```

El programa ni siquiera arranca: es un error de
[sintaxis](../debuter/glossaire.md#sintaxis). Añade un método o un getter en la clase
(`get saldo() { return this.#saldo; }`) y luego lee `c.saldo`.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```javascript norun
class Caja {
  contenido = [];
}
const a = new Caja();
const b = new Caja();
a.contenido.push("manzana");
console.log(b.contenido.length);
```

> **Respuesta**
>
> `0`. Cada objeto recibe su propio campo `contenido`: llenar el de `a` no cambia el
> de `b`.

**Pregunta 2:** ¿cuál es la diferencia entre `static` y un campo normal?

> **Respuesta**
>
> Un campo normal existe en **cada objeto** (`michi.nombre`). Un miembro `static`
> existe **una sola vez**, en la clase (`Temperatura.CERO_ABSOLUTO`).

**Pregunta 3:** escribe una clase `Persona` con un `nombre` y un `apellido`, y un getter
`nombreCompleto` que devuelva los dos separados por un espacio.

> **Respuesta**
>
> ```javascript
> class Persona {
>   constructor(nombre, apellido) {
>     this.nombre = nombre;
>     this.apellido = apellido;
>   }
>
>   get nombreCompleto() {
>     return `${this.nombre} ${this.apellido}`;
>   }
> }
>
> console.log(new Persona("Ada", "Lovelace").nombreCompleto);
> ```
>
> ```sortie
> Ada Lovelace
> ```

## En resumen

- Una clase es un modelo; `new` fabrica un objeto y llama a `constructor`.
- `this` designa el objeto actual; sus propiedades se escriben `this.nombre`.
- `get` y `set` dan propiedades calculadas o comprobadas.
- `static` pertenece a la clase; `#campo` es privado, invisible fuera de la clase.
- `toString` elige el texto de un objeto; `instanceof` comprueba su clase.

Siguiente: [herencia y prototipos](heritage.md). Compara con
[las clases en Ruby](../ruby/types.md).
