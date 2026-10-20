# Herencia y prototipos

Esta página muestra cómo una clase puede reutilizar otra clase con `extends` y `super`, qué es un prototipo y cuándo preferir la composición.

> **Objetivos:**
>
> - sabrás crear una subclase con `extends`;
> - sabrás llamar al padre con `super`;
> - sabrás redefinir un método y aprovechar el polimorfismo;
> - entenderás, de forma sencilla, qué es un prototipo;
> - sabrás elegir entre herencia, composición y mixin.

## Crear una subclase con extends

La [herencia](../debuter/glossaire.md#herencia) permite crear una clase a partir de otra.
La nueva clase, la **subclase** (o clase hija), recibe todo lo que tiene la clase
**padre**: sus propiedades y sus métodos.

Piensa en una familia de vehículos: un coche *es un* vehículo, una bici *es un*
vehículo. Todos ruedan; cada uno añade sus particularidades.

```javascript
class Animal {
  constructor(nombre) {
    this.nombre = nombre;
  }

  comer() {
    console.log(`${this.nombre} come.`);
  }
}

class Perro extends Animal {
  ladrar() {
    console.log(`${this.nombre} ladra: ¡Guau!`);
  }
}

const rex = new Perro("Rex");
rex.comer();
rex.ladrar();
console.log(rex instanceof Perro, rex instanceof Animal);
```

```sortie
Rex come.
Rex ladra: ¡Guau!
true true
```

`Perro` no ha escrito ni `constructor` ni `comer`: los hereda de `Animal`. Un perro
también es un animal, así que `instanceof Animal` responde `true`.

## El constructor y super

Si la subclase necesita su propio constructor, primero debe llamar al del padre con
`super(...)`. El padre prepara la base del objeto; después, la hija la completa.

```javascript
class Animal {
  constructor(nombre) {
    this.nombre = nombre;
  }
}

class Perro extends Animal {
  constructor(nombre, raza) {
    super(nombre);
    this.raza = raza;
  }
}

const rex = new Perro("Rex", "labrador");
console.log(rex);
```

```sortie
Perro { nombre: 'Rex', raza: 'labrador' }
```

> **Recuerda:** en el constructor de una subclase, llama a `super(...)` **antes** de
> usar `this`.

## Redefinir un método

Una subclase puede **redefinir** un método del padre: escribe un método con el mismo
nombre, que sustituye al del padre para sus objetos.

```javascript
class Animal {
  constructor(nombre) {
    this.nombre = nombre;
  }

  hablar() {
    return "...";
  }
}

class Gato extends Animal {
  hablar() {
    return "Miau";
  }
}

console.log(new Animal("Bicho").hablar());
console.log(new Gato("Misi").hablar());
```

```sortie
...
Miau
```

### Completar el método del padre con super

A veces no quieres sustituir el método del padre, sino **enriquecerlo**.
`super.metodo()` llama a la versión del padre.

```javascript
class Empleado {
  constructor(nombre, salario) {
    this.nombre = nombre;
    this.salario = salario;
  }

  describir() {
    return `${this.nombre}, salario ${this.salario} €`;
  }
}

class Jefe extends Empleado {
  constructor(nombre, salario, equipo) {
    super(nombre, salario);
    this.equipo = equipo;
  }

  describir() {
    return `${super.describir()}, jefe de ${this.equipo.length} personas`;
  }
}

console.log(new Empleado("Léa", 2000).describir());
console.log(new Jefe("Sam", 3000, ["Léa", "Tom"]).describir());
```

```sortie
Léa, salario 2000 €
Sam, salario 3000 €, jefe de 2 personas
```

## El polimorfismo

«Polimorfismo» quiere decir «muchas formas». Objetos de clases distintas responden a la
**misma llamada** de método, cada uno a su manera. El código que los usa no necesita
saber quién es quién.

Aquí tienes una lista de figuras. Cada una sabe calcular su área; el bucle simplemente
llama a `area()`.

```javascript
class Figura {
  constructor(nombre) {
    this.nombre = nombre;
  }

  area() {
    throw new Error("area() debe redefinirse");
  }

  toString() {
    return `${this.nombre}: ${this.area().toFixed(2)}`;
  }
}

class Rectangulo extends Figura {
  constructor(ancho, alto) {
    super("Rectángulo");
    this.ancho = ancho;
    this.alto = alto;
  }

  area() {
    return this.ancho * this.alto;
  }
}

class Cuadrado extends Rectangulo {
  constructor(lado) {
    super(lado, lado);
    this.nombre = "Cuadrado";
  }
}

class Circulo extends Figura {
  constructor(radio) {
    super("Círculo");
    this.radio = radio;
  }

  area() {
    return Math.PI * this.radio ** 2;
  }
}

const figuras = [new Rectangulo(2, 3), new Cuadrado(4), new Circulo(1)];
for (const figura of figuras) {
  console.log(String(figura));
}

const total = figuras.reduce((suma, f) => suma + f.area(), 0);
console.log(`Área total: ${total.toFixed(2)}`);
```

```sortie
Rectángulo: 6.00
Cuadrado: 16.00
Círculo: 3.14
Área total: 25.14
```

Tres ideas en este ejemplo:

- `Figura.area` lanza un error: es una forma de decir «cada subclase **debe** escribir
  su propia versión»;
- `toString` se escribe una sola vez, en `Figura`, y llama a `this.area()`: se elige la
  versión correcta según el objeto;
- `Cuadrado` hereda de `Rectangulo`, que hereda de `Figura`: la herencia puede tener
  varios pisos.

Para añadir un triángulo, basta con escribir una clase `Triangulo`: el bucle no cambia.

## Los prototipos, de forma sencilla

Por debajo, JavaScript no tiene realmente «clases» como otros lenguajes. Tiene
**prototipos**. La palabra `class` es una forma más agradable de escribir este
mecanismo.

Cada objeto tiene un enlace oculto hacia otro objeto: su **prototipo**. Cuando pides una
propiedad, JavaScript la busca primero en el objeto. Si no la encuentra, la busca en el
prototipo, luego en el prototipo del prototipo, etc. Es la **cadena de prototipos**.

Piensa en un niño que no sabe responder: le pregunta a su padre, que le pregunta al
abuelo, y así sucesivamente.

`Object.getPrototypeOf(objeto)` muestra este enlace.

```javascript
class Animal {
  constructor(nombre) {
    this.nombre = nombre;
  }

  hablar() {
    return "...";
  }
}

class Perro extends Animal {}

const rex = new Perro("Rex");

console.log(Object.getPrototypeOf(rex) === Perro.prototype);
console.log(Object.getPrototypeOf(Perro.prototype) === Animal.prototype);
console.log(Object.hasOwn(rex, "nombre"));
console.log(Object.hasOwn(rex, "hablar"));
console.log(Object.getOwnPropertyNames(Animal.prototype));
```

```sortie
true
true
true
false
[ 'constructor', 'hablar' ]
```

¿Qué pasa cuando escribes `rex.hablar()`?

```text
rex                 → nombre: "Rex"          no tiene hablar
  ↓ prototipo
Perro.prototype     → (nada más)             no tiene hablar
  ↓ prototipo
Animal.prototype    → hablar()               ¡encontrado!
  ↓ prototipo
Object.prototype    → toString(), hasOwnProperty()…
  ↓ prototipo
null                → fin de la cadena
```

- Los **datos** (`nombre`) se guardan en cada objeto: `Object.hasOwn(rex, "nombre")`
  responde `true`.
- Los **métodos** se guardan una sola vez, en el prototipo de la clase: todos los
  objetos los comparten.

Por eso también todo objeto tiene `toString`: la cadena siempre termina en
`Object.prototype`.

> **Nota:** casi nunca necesitas manipular los prototipos a mano. Escribe clases;
> recuerda solo que los métodos se comparten a través de la cadena de prototipos.

## Preferir la composición

La herencia expresa una relación «**es un**»: un perro *es un* animal. Pero a menudo, la
relación es «**tiene un**»: un coche *tiene un* motor. ¡Un coche no es un motor! En ese
caso, guardas el objeto en una propiedad: es la **composición**.

```javascript
class Motor {
  constructor(caballos) {
    this.caballos = caballos;
  }

  arrancar() {
    return `Brum (${this.caballos} CV)`;
  }
}

class Coche {
  constructor(marca, motor) {
    this.marca = marca;
    this.motor = motor;
  }

  arrancar() {
    console.log(`${this.marca}: ${this.motor.arrancar()}`);
  }
}

const pequeno = new Coche("Utilitario", new Motor(70));
const grande = new Coche("Camión", new Motor(400));
pequeno.arrancar();
grande.arrancar();
```

```sortie
Utilitario: Brum (70 CV)
Camión: Brum (400 CV)
```

La composición es más flexible: cambias de motor sin tocar `Coche`. Las cadenas de
herencia largas (`A` → `B` → `C` → `D`) se vuelven difíciles de entender enseguida.

> **Consejo:** si dudas, pregúntate «¿X **es un** Y?». Si la frase suena mal, elige la
> composición.

## Los mixins con Object.assign

Una clase solo tiene **un** padre. ¿Cómo dar la misma capacidad (por ejemplo «saber
nadar») a clases que no tienen relación entre sí? Con un
[mixin](../debuter/glossaire.md#mixin): un simple objeto que contiene métodos, que se
copia en el prototipo de una clase.

`Object.assign(destino, origen)` copia las propiedades de `origen` en `destino`.

```javascript
const Nadador = {
  nadar() {
    return `${this.nombre} nada.`;
  },
};

const Volador = {
  volar() {
    return `${this.nombre} vuela.`;
  },
};

class Pato {
  constructor(nombre) {
    this.nombre = nombre;
  }
}

class Pez {
  constructor(nombre) {
    this.nombre = nombre;
  }
}

Object.assign(Pato.prototype, Nadador, Volador);
Object.assign(Pez.prototype, Nadador);

const donald = new Pato("Donald");
const nemo = new Pez("Nemo");
console.log(donald.nadar());
console.log(donald.volar());
console.log(nemo.nadar());
console.log(typeof nemo.volar);
```

```sortie
Donald nada.
Donald vuela.
Nemo nada.
undefined
```

El pato sabe nadar y volar; el pez solo sabe nadar. Los métodos del mixin usan
`this.nombre`: funcionan con cualquier objeto que tenga un `nombre`.

## Errores frecuentes

### Usar this antes de super

```javascript norun
class Animal {
  constructor(nombre) { this.nombre = nombre; }
}
class Perro extends Animal {
  constructor(nombre, raza) {
    this.raza = raza;
    super(nombre);
  }
}
new Perro("Rex", "labrador");
```

```text
file:///home/vous/projet/main.mjs:6
    this.raza = raza;
    ^

ReferenceError: Must call super constructor in derived class before accessing 'this' or returning from derived constructor
    at new Perro (file:///home/vous/projet/main.mjs:6:5)
    at file:///home/vous/projet/main.mjs:10:1
```

El mensaje dice: «llama al constructor padre antes de usar `this`». Pon `super(nombre);`
en la **primera** línea del constructor. El mismo mensaje aparece si olvidas `super` por
completo.

### Olvidar super. al completar un método

```javascript norun
class Animal {
  describir() { return "un animal"; }
}
class Perro extends Animal {
  describir() { return this.describir() + " que ladra"; }
}
console.log(new Perro().describir());
```

```text
file:///home/vous/projet/main.mjs:5
  describir() { return this.describir() + " que ladra"; }
                            ^

RangeError: Maximum call stack size exceeded
    at Perro.describir (file:///home/vous/projet/main.mjs:5:29)
    at Perro.describir (file:///home/vous/projet/main.mjs:5:29)
    at Perro.describir (file:///home/vous/projet/main.mjs:5:29)
```

`this.describir()` llama al método del **perro**, es decir, a sí mismo, hasta el
infinito. La pila de llamadas se desborda. Para llamar a la versión del padre, escribe
`super.describir()`.

### Escribir mal el nombre de un método redefinido

```javascript norun
class Figura { area() { throw new Error("area() debe redefinirse"); } }
class Cuadrado extends Figura { constructor(l){super(); this.l=l;} aera(){ return this.l**2; } }
console.log(new Cuadrado(2).area());
```

```text
file:///home/vous/projet/main.mjs:1
class Figura { area() { throw new Error("area() debe redefinirse"); } }
                              ^

Error: area() debe redefinirse
    at Cuadrado.area (file:///home/vous/projet/main.mjs:1:31)
    at file:///home/vous/projet/main.mjs:3:29
```

`aera` no es `area`: el método no está redefinido, así que se ejecuta el del padre.
Comprueba que el nombre es **exactamente** el mismo que en la clase padre.

## Ponte a prueba

**Pregunta 1:** ¿qué muestra este código?

```javascript norun
class A { nombre() { return "A"; } }
class B extends A { nombre() { return "B" + super.nombre(); } }
class C extends B {}
console.log(new C().nombre());
```

> **Respuesta**
>
> `BA`. `C` no tiene método `nombre`: JavaScript lo encuentra en `B`, que añade `"B"`
> delante del resultado de la versión de `A`.

**Pregunta 2:** una `Casa` tiene `Habitacion`es. ¿Hay que escribir `class Casa extends
Habitacion`?

> **Respuesta**
>
> No. Una casa **tiene** habitaciones, no **es** una habitación. Usa la composición:
> `this.habitaciones = [new Habitacion(...), ...]`.

**Pregunta 3:** ¿dónde se guarda el método `hablar` de un objeto `new Gato()`: en el
propio objeto o en otro sitio?

> **Respuesta**
>
> En `Gato.prototype` (o más arriba en la cadena). El objeto solo contiene sus datos;
> `Object.hasOwn(objeto, "hablar")` responde `false`. Todas las instancias comparten el
> mismo método.

## En resumen

- `class B extends A`: `B` hereda las propiedades y los métodos de `A`.
- En el constructor hijo, `super(...)` va antes de cualquier `this`.
- Un método con el mismo nombre redefine el del padre; `super.metodo()` llama al padre.
- El polimorfismo: objetos distintos responden a la misma llamada, cada uno a su manera.
- Los métodos viven en los prototipos, enlazados en cadena hasta `Object.prototype`.
- «Es un» → herencia; «tiene un» → composición; capacidad compartida → mixin.

Siguiente: [alcance y closures](closures.md). Compara con
[la herencia en Ruby](../ruby/heritage.md).
