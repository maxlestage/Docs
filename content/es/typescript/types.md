# Las clases

Esta página explica cómo crear tus propios tipos de objetos con `class`, proteger sus datos y relacionarlos con interfaces.

> **Objetivos:**
>
> - sabrás escribir una clase con campos tipados y un constructor;
> - sabrás acortar un constructor con las propiedades de parámetro;
> - sabrás proteger datos con `private`, `protected`, `readonly` y `#`;
> - sabrás que una clase cumple una interfaz con `implements`;
> - sabrás usar `abstract`, `static` y los accesores `get` / `set`.

## Una primera clase

Una [clase](../debuter/glossaire.md#clase) es un molde de objetos. El molde describe la
forma; cada objeto fabricado con ese molde es una **instancia**. Es como un cortador de
galletas: una sola herramienta, tantas galletas como quieras.

```typescript
class Gato {
  nombre: string;
  edad: number;

  constructor(nombre: string, edad: number) {
    this.nombre = nombre;
    this.edad = edad;
  }

  presentarse(): string {
    return `Soy ${this.nombre}, tengo ${this.edad} años.`;
  }
}

const michi = new Gato("Michi", 3);
const tigre = new Gato("Tigre", 5);
console.log(michi.presentarse());
console.log(tigre.presentarse());
```

```sortie
Soy Michi, tengo 3 años.
Soy Tigre, tengo 5 años.
```

Vamos por partes:

- `nombre: string;` declara un **campo**: un dato que tiene cada gato, con su tipo;
- `constructor` es la función a la que llama `new`. Rellena los campos;
- `this` designa el objeto que se está construyendo (o aquel sobre el que se llama al
  método);
- `presentarse()` es un [método](../debuter/glossaire.md#metodo): una función unida
  al objeto.

Una clase también es un tipo: puedes escribir `const g: Gato = ...`.

### Un valor inicial

Un campo puede recibir su valor directamente, sin pasar por el constructor.
TypeScript adivina entonces su tipo.

```typescript
class Contador {
  valor = 0;

  incrementar() {
    this.valor++;
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

> **Recuerda:** en modo estricto, cada campo debe recibir un valor, ya sea en su
> declaración o en el constructor. Si no, TypeScript lo rechaza (mira los
> [errores frecuentes](#errores-frecuentes)).

## Las propiedades de parámetro

Escribir cada campo tres veces (declaración, parámetro, `this.x = x`) es pesado.
TypeScript propone un atajo: pon `public` (o `private`, `readonly`…) delante de un
parámetro del constructor. El campo se declara y se rellena automáticamente.

```typescript
class Punto {
  constructor(public x: number, public y: number) {}

  distanciaAlOrigen(): number {
    return Math.sqrt(this.x ** 2 + this.y ** 2);
  }
}

const p = new Punto(3, 4);
console.log(p.x, p.y);
console.log(p.distanciaAlOrigen());
```

```sortie
3 4
5
```

El cuerpo del constructor está vacío `{}`: todo el trabajo lo hacen las palabras
delante de los parámetros.

## Proteger los datos

Por defecto, todo es **público**: cualquier código puede leer y modificar los campos.
Algunos campos no deberían tocarse desde fuera. A una cuenta bancaria no se le debe
cambiar el saldo de cualquier manera.

| Palabra | ¿Quién puede acceder? |
|-----|----------------------|
| `public` | todo el mundo (es el valor por defecto) |
| `private` | solo el código de la clase |
| `protected` | la clase y sus clases hijas |
| `readonly` | todo el mundo puede leer, nadie modifica después del constructor |

```typescript
class Cuenta {
  private saldo = 0;
  readonly titular: string;

  constructor(titular: string) {
    this.titular = titular;
  }

  depositar(cantidad: number) {
    if (cantidad <= 0) {
      console.log("Cantidad rechazada");
      return;
    }
    this.saldo += cantidad;
  }

  mostrar() {
    console.log(`${this.titular}: ${this.saldo} €`);
  }
}

const cuenta = new Cuenta("Alice");
cuenta.depositar(50);
cuenta.depositar(-10);
cuenta.mostrar();
```

```sortie
Cantidad rechazada
Alice: 50 €
```

La única forma de cambiar el saldo es `depositar`, que comprueba la cantidad. Escribir
`cuenta.saldo = 1000` desde fuera sería rechazado por TypeScript.

### Los campos realmente privados con #

`private` es una protección solo de TypeScript: una vez compilado el código a
JavaScript, desaparece. JavaScript tiene su propia protección: un nombre que empieza
por `#`. Esta sigue activa durante la ejecución.

```typescript
class CajaFuerte {
  #codigo: string;

  constructor(codigo: string) {
    this.#codigo = codigo;
  }

  abrir(intento: string): string {
    return intento === this.#codigo ? "¡Abierta!" : "Código incorrecto";
  }
}

const caja = new CajaFuerte("1234");
console.log(caja.abrir("0000"));
console.log(caja.abrir("1234"));
```

```sortie
Código incorrecto
¡Abierta!
```

> **Nota:** para empezar, las dos valen lo mismo. `private` es más habitual en el
> código TypeScript; `#` es útil cuando la protección también debe mantenerse durante
> la ejecución.

## Implementar una interfaz

Una interfaz describe una forma (mira [los objetos](objets.md)). Con `implements`, una
clase **promete** respetar esa forma. Si olvida un campo o un método, TypeScript lo
señala.

```typescript
interface Animal {
  nombre: string;
  hacerRuido(): string;
}

class Perro implements Animal {
  constructor(public nombre: string) {}
  hacerRuido() {
    return "Guau";
  }
}

class Vaca implements Animal {
  constructor(public nombre: string) {}
  hacerRuido() {
    return "Muu";
  }
}

const granja: Animal[] = [new Perro("Rex"), new Vaca("Margarita")];
for (const a of granja) {
  console.log(`${a.nombre} hace ${a.hacerRuido()}`);
}
```

```sortie
Rex hace Guau
Margarita hace Muu
```

El array tiene el tipo `Animal[]`: acepta cualquier objeto que tenga la forma correcta.

## Heredar de una clase

`extends` crea una clase **hija** a partir de una clase **madre**: recibe todos sus
campos y métodos (mira la [herencia](../debuter/glossaire.md#herencia)). En el
constructor, `super(...)` llama primero al constructor de la madre.

```typescript
class Vehiculo {
  protected velocidad = 0;

  constructor(public nombre: string) {}

  acelerar(delta: number) {
    this.velocidad += delta;
  }
}

class Bicicleta extends Vehiculo {
  constructor() {
    super("Bicicleta");
  }

  describir(): string {
    return `${this.nombre} a ${this.velocidad} km/h`;
  }
}

const v = new Bicicleta();
v.acelerar(15);
console.log(v.describir());
```

```sortie
Bicicleta a 15 km/h
```

`velocidad` es `protected`: `Bicicleta`, la clase hija, puede leerla; el código de
fuera no.

## Las clases abstractas

Una clase **abstracta** es un molde incompleto: no se puede crear un objeto con ella.
Sirve de base común. Declara métodos `abstract` sin cuerpo, que cada clase hija debe
escribir.

```typescript
abstract class Forma {
  abstract area(): number;

  describir(): string {
    return `Área: ${this.area().toFixed(2)}`;
  }
}

class Cuadrado extends Forma {
  constructor(private lado: number) {
    super();
  }
  area() {
    return this.lado ** 2;
  }
}

class Circulo extends Forma {
  constructor(private radio: number) {
    super();
  }
  area() {
    return Math.PI * this.radio ** 2;
  }
}

const formas: Forma[] = [new Cuadrado(2), new Circulo(1)];
for (const f of formas) {
  console.log(f.describir());
}
```

```sortie
Área: 4.00
Área: 3.14
```

`describir` se escribe una sola vez, en `Forma`. Llama a `area()`, que es distinta
para cada forma.

> **Consejo:** ¿interfaz o clase abstracta? Una interfaz no contiene ningún código: es
> un contrato. Una clase abstracta puede compartir código (aquí, `describir`). En caso
> de duda, empieza por una interfaz.

## Los miembros static

Un miembro `static` pertenece a la **clase** misma, no a cada objeto. Se usa con el
nombre de la clase: `Temperatura.desdeFahrenheit(...)`. Es práctico para constantes o
funciones de fabricación.

```typescript
class Temperatura {
  static readonly CERO_ABSOLUTO = -273.15;
  static creadas = 0;

  constructor(public celsius: number) {
    Temperatura.creadas++;
  }

  static desdeFahrenheit(f: number): Temperatura {
    return new Temperatura(((f - 32) * 5) / 9);
  }
}

const t = Temperatura.desdeFahrenheit(212);
console.log(t.celsius);
console.log(Temperatura.CERO_ABSOLUTO);
console.log(`Objetos creados: ${Temperatura.creadas}`);
```

```sortie
100
-273.15
Objetos creados: 1
```

## Los accesores get y set

Un **accesor** parece un campo, pero es un método disfrazado. `get` calcula un valor
al leerlo; `set` controla una escritura. Se usan sin paréntesis.

```typescript
class Rectangulo {
  constructor(private _ancho: number, private alto: number) {}

  get area(): number {
    return this._ancho * this.alto;
  }

  get ancho(): number {
    return this._ancho;
  }

  set ancho(valor: number) {
    if (valor <= 0) {
      console.log("Ancho rechazado");
      return;
    }
    this._ancho = valor;
  }
}

const r = new Rectangulo(2, 3);
console.log(r.area);
r.ancho = 10;
r.ancho = -1;
console.log(r.area);
```

```sortie
6
Ancho rechazado
30
```

`r.area` siempre está al día: se vuelve a calcular en cada lectura. Un `get` sin `set`
es de solo lectura.

## Errores frecuentes

### Olvidar inicializar un campo

```typescript norun
class Gato {
  nombre: string;
}
```

```text
main.ts(2,3): error TS2564: Property 'nombre' has no initializer and is not definitely assigned in the constructor.
```

El campo `nombre` nunca tiene valor. Dale un valor inicial (`nombre = ""`), o
rellénalo en el constructor: `constructor(public nombre: string) {}`.

### Olvidar this

```typescript norun
class Contador {
  valor = 0;
  incrementar() {
    valor++;
  }
}
```

```text
main.ts(4,5): error TS2663: Cannot find name 'valor'. Did you mean the instance member 'this.valor'?
```

En un método, un campo se escribe siempre con `this.` delante: `this.valor++`. El
propio mensaje te lo sugiere.

### Tocar un campo privado desde fuera

```typescript norun
class Cuenta {
  private saldo = 0;
}
const c = new Cuenta();
console.log(c.saldo);
```

```text
main.ts(5,15): error TS2341: Property 'saldo' is private and only accessible within class 'Cuenta'.
```

Es a propósito: el campo está protegido. Añade un método público para leerlo, por
ejemplo `get cantidad() { return this.saldo; }`.

### Olvidar un método de la interfaz

```typescript norun
interface Animal {
  nombre: string;
  hacerRuido(): string;
}
class Perro implements Animal {
  constructor(public nombre: string) {}
}
```

```text
main.ts(5,7): error TS2420: Class 'Perro' incorrectly implements interface 'Animal'.
  Property 'hacerRuido' is missing in type 'Perro' but required in type 'Animal'.
```

La promesa `implements Animal` no se cumple. Añade el método que falta:
`hacerRuido() { return "Guau"; }`.

## Ponte a prueba

**Pregunta 1:** ¿qué hace este constructor?

```typescript norun
class Libro {
  constructor(public titulo: string, private paginas: number) {}
}
```

> **Respuesta**
>
> Declara dos campos y los rellena: `titulo`, público, y `paginas`, privado. Es la
> versión corta de dos declaraciones de campos y de dos líneas
> `this.titulo = titulo;` y `this.paginas = paginas;`.

**Pregunta 2:** ¿se puede escribir `new Forma()` si `Forma` es una clase abstracta?

> **Respuesta**
>
> No. TypeScript responde «Cannot create an instance of an abstract class». Hay que
> crear una clase hija que escriba los métodos abstractos, y luego hacer `new` con
> esa clase hija.

**Pregunta 3:** escribe una clase `Persona` con un campo `readonly nombre` y un
accesor `get inicial` que devuelva la primera letra del nombre.

> **Respuesta**
>
> ```typescript
> class Persona {
>   constructor(readonly nombre: string) {}
>
>   get inicial(): string {
>     return this.nombre[0] ?? "?";
>   }
> }
>
> const p = new Persona("Zoé");
> console.log(p.inicial);
> ```
>
> ```sortie
> Z
> ```

## En resumen

- Una clase es un molde de objetos; `new` fabrica una instancia, `constructor` la
  rellena.
- `constructor(public x: number)` declara y rellena un campo de una vez.
- `private`, `protected` y `readonly` protegen los campos; `#` también protege
  durante la ejecución.
- `implements` comprueba que una clase respeta una interfaz; `extends` hereda de una
  clase.
- Una clase `abstract` sirve de base y no se instancia; `static` pertenece a la
  clase; `get` y `set` disfrazan métodos de campos.

Siguiente: [los genéricos](generiques.md)
