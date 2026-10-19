# Los decoradores

Un decorador es una función que «envuelve» un método, un campo o un accesor de una clase para añadirle un comportamiento, sin tocar su código.

> **Objetivos:**
>
> - sabrás qué es un [decorador](../debuter/glossaire.md#decorateur) y leer la
>   sintaxis `@nombre`;
> - sabrás escribir un decorador de método, por ejemplo para mostrar cada llamada;
> - sabrás decorar un campo y un accesor creado con `accessor`;
> - sabrás escribir una fábrica de decoradores que recibe parámetros;
> - sabrás leer el objeto contexto que TypeScript le da al decorador.

Esta página usa los decoradores **estándar** de TypeScript 5. Tienes que conocer las
[clases](types.md) antes de leerla.

## Un decorador es papel de regalo

Imagina un regalo. No cambias el objeto que hay dentro: lo envuelves. El papel añade
algo **alrededor** (un lazo, una etiqueta), y el objeto sigue igual por dentro.

Un decorador hace lo mismo con un método:

- **recibe** el método original (el regalo);
- **devuelve** una nueva función que llama a la original (el regalo envuelto);
- después, la clase usa la versión envuelta.

```text
   sin decorador                con @envolver

   abrir()                      abrir()
     │                            │
     ▼                            ▼
   ┌────────────┐               ┌──────────────────────┐
   │"¡Un balón!"│               │ ***  ┌────────────┐  │
   └────────────┘               │      │"¡Un balón!"│  │
                                │      └────────────┘ ***
                                └──────────────────────┘
```

Se escribe `@` seguido del nombre de la función, justo encima de lo que se decora:

```typescript
function envolver(metodo: () => string, contexto: ClassMethodDecoratorContext) {
  return function (this: unknown): string {
    return "*** " + metodo.call(this) + " ***";
  };
}

class Regalo {
  constructor(public contenido: string) {}

  @envolver
  abrir(): string {
    return `¡Un ${this.contenido}!`;
  }
}

const regalo = new Regalo("balón");
console.log(regalo.abrir());
```

```sortie
*** ¡Un balón! ***
```

Leámoslo despacio:

- `envolver` es una función normal. Recibe dos cosas: el método original (`metodo`) y
  un objeto `contexto` (volveremos a él más abajo);
- devuelve una **nueva** función. Esta función llama a `metodo` y añade estrellas
  alrededor del resultado;
- `metodo.call(this)` llama al método original con el objeto correcto: `this` es el
  regalo sobre el que se llamó `abrir()`;
- `@envolver` dice: «sustituye `abrir` por lo que devuelve `envolver`».

Pruébalo: quita la línea `@envolver`. El programa muestra `¡Un balón!`, sin estrellas.
El código de `abrir` no ha cambiado: solo ha desaparecido el envoltorio.

> **Recuerda:** un decorador es una simple función. Recibe el método original y
> devuelve una función que lo sustituye.

## Cuándo se ejecuta el decorador

Hay dos momentos distintos, y eso es lo que suele confundir al principio:

1. la función `envolver` se ejecuta **una sola vez**, cuando se crea la clase;
2. la función que devuelve se ejecuta **en cada llamada** al método.

Añadamos mensajes para verlo:

```typescript
function envolver(metodo: () => string, contexto: ClassMethodDecoratorContext) {
  console.log(`Envuelvo ${String(contexto.name)}`);
  return function (this: unknown): string {
    console.log("  desenvolviendo…");
    return "*** " + metodo.call(this) + " ***";
  };
}

class Regalo {
  constructor(public contenido: string) {}

  @envolver
  abrir(): string {
    return `¡Un ${this.contenido}!`;
  }
}

console.log("Inicio del programa");
console.log(new Regalo("balón").abrir());
console.log(new Regalo("libro").abrir());
```

```sortie
Envuelvo abrir
Inicio del programa
  desenvolviendo…
*** ¡Un balón! ***
  desenvolviendo…
*** ¡Un libro! ***
```

Sigamos el programa paso a paso:

| Paso | Qué pasa | Salida |
|------|----------|--------|
| 1 | se crea la clase `Regalo`: se llama a `envolver` para `abrir` | `Envuelvo abrir` |
| 2 | la línea `console.log("Inicio…")` | `Inicio del programa` |
| 3 | `abrir()` sobre el balón: se ejecuta la función envuelta | `  desenvolviendo…` |
| 4 | llama a la original, que devuelve `"¡Un balón!"`, y luego añade las estrellas | `*** ¡Un balón! ***` |
| 5 | `abrir()` sobre el libro: lo mismo, sin volver a llamar a `envolver` | `  desenvolviendo…` y luego `*** ¡Un libro! ***` |

`Envuelvo abrir` se muestra **antes** de `Inicio del programa`: el decorador trabaja en
cuanto se lee la clase, antes incluso de que exista un solo regalo.

Pruébalo: crea tres regalos en lugar de dos. `Envuelvo abrir` sigue mostrándose una
sola vez; `desenvolviendo…` se muestra tres veces.

> **Recuerda:** el decorador envuelve una vez, al crear la clase. La función envuelta
> se ejecuta en cada llamada.

## Registrar las llamadas

Un uso clásico: mostrar cada llamada a un método, con sus argumentos y su resultado.
Es **registrar** (en inglés, *to log*). Muy útil para entender un programa sin añadir
`console.log` por todas partes.

Para que este decorador funcione con **cualquier** método, lo hacemos
[genérico](generiques.md): `This` es el tipo del objeto, `Args` la lista de parámetros,
`R` el tipo del resultado. TypeScript los deduce solo.

```typescript
function registrarLlamadas<This, Args extends unknown[], R>(
  metodo: (this: This, ...args: Args) => R,
  contexto: ClassMethodDecoratorContext<This, (this: This, ...args: Args) => R>,
) {
  const nombre = String(contexto.name);
  return function (this: This, ...args: Args): R {
    console.log(`-> ${nombre}(${args.join(", ")})`);
    const resultado = metodo.call(this, ...args);
    console.log(`<- ${nombre} devuelve ${resultado}`);
    return resultado;
  };
}

class Caja {
  total = 0;

  @registrarLlamadas
  sumar(precio: number, cantidad: number): number {
    this.total += precio * cantidad;
    return this.total;
  }
}

const caja = new Caja();
caja.sumar(3, 2);
caja.sumar(10, 1);
console.log(`Total final: ${caja.total}`);
```

```sortie
-> sumar(3, 2)
<- sumar devuelve 6
-> sumar(10, 1)
<- sumar devuelve 16
Total final: 16
```

El dibujo de una llamada:

```text
   caja.sumar(3, 2)
          │
   ┌──────▼──────────────────────────┐
   │ envoltorio (registrarLlamadas)  │  muestra "-> sumar(3, 2)"
   │   ┌───────────────────────────┐ │
   │   │ sumar original            │ │  total = 0 + 3 * 2 = 6
   │   └───────────────────────────┘ │
   │                                 │  muestra "<- sumar devuelve 6"
   └──────┬──────────────────────────┘
          ▼
          6
```

Y la evolución de `total`:

| Llamada | `precio` | `cantidad` | `total` antes | `total` después | Resultado |
|---------|----------|------------|---------------|-----------------|-----------|
| 1 | 3 | 2 | 0 | 6 | 6 |
| 2 | 10 | 1 | 6 | 16 | 16 |

Gracias a los tipos genéricos, la función envuelta conserva exactamente la forma de la
original: `caja.sumar("tres", 2)` sigue siendo un error de tipo.

Pruébalo: añade un método `vaciar(): number` que pone `total` a 0 y devuelve 0,
decóralo con `@registrarLlamadas` y llámalo. Verás `-> vaciar()` y luego
`<- vaciar devuelve 0`: el mismo decorador sirve para todos los métodos.

> **Recuerda:** un solo decorador genérico puede envolver todos los métodos. Añade un
> comportamiento común sin copiar código.

## El objeto contexto

El segundo parámetro de un decorador es el **contexto**: una ficha que describe lo que
se decora. Es como la etiqueta pegada en un paquete: dice lo que hay dentro, sin
abrirlo.

| Campo | Qué contiene |
|-------|--------------|
| `kind` | el tipo de elemento: `"method"`, `"field"`, `"accessor"`, `"getter"`, `"setter"` o `"class"` |
| `name` | el nombre del elemento (`"sumar"`) |
| `static` | `true` si el elemento es `static` |
| `private` | `true` si su nombre empieza por `#` |
| `addInitializer` | una función para ejecutar código al crear cada objeto |

Un decorador puede decorar cualquier cosa, si acepta cualquier contexto. Aquí tienes un
«espía» que solo muestra la ficha:

```typescript
function espia(_valor: unknown, contexto: DecoratorContext) {
  const detalles = "static" in contexto ? ` static=${contexto.static} private=${contexto.private}` : "";
  console.log(`${contexto.kind} ${String(contexto.name)}${detalles}`);
}

@espia
class Cuenta {
  @espia saldo = 0;
  @espia static banco = "Banco del Barrio";
  @espia #codigo = 1234;

  @espia
  depositar(monto: number) {
    this.saldo += monto;
  }
}

console.log(new Cuenta().saldo);
```

```sortie
method depositar static=false private=false
field banco static=true private=false
field saldo static=false private=false
field #codigo static=false private=true
class Cuenta
0
```

Fíjate en el orden: primero los métodos, luego los campos `static`, luego los demás
campos, y la clase al final.
`"static" in contexto` comprueba que la ficha tiene un campo `static`: una ficha de
clase no lo tiene, y TypeScript lo sabe (es un
[estrechamiento](../debuter/glossaire.md#retrecissement)).

### addInitializer: actuar sobre cada objeto

`addInitializer` registra una función que se ejecuta **al crear cada objeto**. Un
ejemplo útil: «vincular» un método a su objeto.

Primero, el problema. Cuando sacas un método de su objeto (`const pulsar =
t.sonar;`, para pasárselo a otra persona), **pierde su `this`**: llamar a `pulsar()` ya
no sabe de qué timbre se trata. Es como arrancar el botón de un timbre: ya no está
conectado a ninguna puerta.

El decorador `vincular` lo arregla: para cada nuevo objeto, guarda en el objeto una
copia del método unida para siempre a ese objeto (`bind`).

```typescript
function vincular<This extends object, Args extends unknown[], R>(
  metodo: (this: This, ...args: Args) => R,
  contexto: ClassMethodDecoratorContext<This, (this: This, ...args: Args) => R>,
) {
  contexto.addInitializer(function (this: This) {
    Reflect.set(this, contexto.name, metodo.bind(this));
  });
}

class Timbre {
  constructor(public sonido: string) {}

  @vincular
  sonar() {
    return this?.sonido ?? "(se perdió this)";
  }
}

const t = new Timbre("Din don");
const pulsar = t.sonar;
console.log(pulsar());
```

```sortie
Din don
```

`Reflect.set(this, nombre, valor)` escribe `this[nombre] = valor` sin que TypeScript se
queje del nombre desconocido.

Pruébalo: quita la línea `@vincular`. El programa muestra `(se perdió this)`.

> **Recuerda:** el contexto describe el elemento decorado (`kind`, `name`, `static`,
> `private`). `addInitializer` ejecuta código para cada objeto creado.

## Decorar un campo

Un **campo** no tiene función que envolver: tiene un **valor inicial**. Por eso, un
decorador de campo recibe `undefined` como primer parámetro, y puede devolver una
función que **transforma el valor inicial**.

Es como un sello a la entrada de una oficina: cada expediente que llega recibe el
sello, una vez, antes de archivarse.

```typescript
function enMayusculas(_valor: undefined, contexto: ClassFieldDecoratorContext<unknown, string>) {
  return function (valorInicial: string): string {
    console.log(`  ${String(contexto.name)}: "${valorInicial}" se convierte en "${valorInicial.toUpperCase()}"`);
    return valorInicial.toUpperCase();
  };
}

class Ciudad {
  @enMayusculas pais = "francia";
  @enMayusculas continente = "europa";
  nombre = "lyon";
}

console.log("Creación");
const ciudad = new Ciudad();
console.log(ciudad.nombre, ciudad.pais, ciudad.continente);
ciudad.pais = "italia";
console.log(ciudad.pais);
```

```sortie
Creación
  pais: "francia" se convierte en "FRANCIA"
  continente: "europa" se convierte en "EUROPA"
lyon FRANCIA EUROPA
italia
```

| Paso | `pais` | `continente` | `nombre` |
|------|--------|--------------|----------|
| empieza `new Ciudad()` | (vacío) | (vacío) | (vacío) |
| campo `pais`: `"francia"` pasa por el decorador | `"FRANCIA"` | (vacío) | (vacío) |
| campo `continente`: `"europa"` pasa por el decorador | `"FRANCIA"` | `"EUROPA"` | (vacío) |
| campo `nombre`: sin decorador | `"FRANCIA"` | `"EUROPA"` | `"lyon"` |
| `ciudad.pais = "italia"` | `"italia"` | `"EUROPA"` | `"lyon"` |

La última línea muestra el límite: `"italia"` **no** se pone en mayúsculas. Para vigilar
cada cambio, hace falta un accesor (sección siguiente).

> **Atención:** el decorador ve **solo el valor inicial**, el que se escribe después de
> `=`. Un campo sin valor inicial (que se rellena después en el constructor) llega con
> `undefined`, y `undefined.toUpperCase()` hace fallar el programa. Así que decora un
> campo que tenga un valor después de `=`.

> **Recuerda:** un decorador de campo solo transforma el valor inicial, una vez por
> objeto.

## Decorar un accesor con accessor

La palabra clave `accessor` delante de un campo crea automáticamente una pareja
`get` / `set` oculta (mira los [accesores](types.md)). Desde fuera, se usa como un campo
normal. Pero entonces un decorador puede **interceptar cada lectura y cada escritura**.

Es como una ventanilla delante de una caja fuerte: ya no tocas la caja directamente,
pasas por el empleado, que puede comprobar lo que depositas.

```text
   jugador.vida = 150
          │
   ┌──────▼──────────────┐
   │ ventanilla (set)    │  150 es demasiado: se guarda 100
   └──────┬──────────────┘
          ▼
   ┌─────────────────────┐
   │ caja oculta: 100    │
   └─────────────────────┘
```

El decorador recibe `objetivo`, que contiene el `get` y el `set` originales. Devuelve un
objeto con las nuevas versiones (`get`, `set`, e `init` para el valor inicial; cada una
es opcional).

```typescript
function entre0y100<This>(
  objetivo: ClassAccessorDecoratorTarget<This, number>,
  contexto: ClassAccessorDecoratorContext<This, number>,
): ClassAccessorDecoratorResult<This, number> {
  const limitar = (v: number) => Math.min(100, Math.max(0, v));
  return {
    init(inicio: number): number {
      return limitar(inicio);
    },
    set(this: This, v: number) {
      console.log(`  ${String(contexto.name)}: se pide ${v}, se guarda ${limitar(v)}`);
      objetivo.set.call(this, limitar(v));
    },
  };
}

class Jugador {
  @entre0y100 accessor vida = 150;
}

const jugador = new Jugador();
console.log(jugador.vida);
jugador.vida = 40;
jugador.vida = -10;
console.log(jugador.vida);
```

```sortie
100
  vida: se pide 40, se guarda 40
  vida: se pide -10, se guarda 0
0
```

| Paso | Se escribe | La ventanilla guarda | `vida` |
|------|------------|----------------------|--------|
| creación (`init`) | 150 | 100 | 100 |
| `jugador.vida = 40` | 40 | 40 | 40 |
| `jugador.vida = -10` | -10 | 0 | 0 |

Pruébalo: quita la palabra `accessor`. TypeScript rechaza el programa: un decorador de
accesor no se aplica a un campo simple (mira los
[errores frecuentes](#errores-frecuentes)).

> **Recuerda:** `accessor` convierte un campo en una pareja `get` / `set`. Su decorador
> ve cada escritura, no solo el valor inicial.

## Las fábricas de decoradores

`entre0y100` está fijado en 0 y 100. ¿Y si quieres de 0 a 10 para otro campo? Escribes
una **fábrica**: una función que recibe parámetros y **devuelve** un decorador.

Es como una máquina de sellos ajustable: primero eliges el texto del sello
(`"URGENTE"`, `"PAGADO"`), y luego sellas.

```text
   prefijar("[caja]")      ->  un decorador ajustado a "[caja]"
   @prefijar("[caja]")     ->  este decorador envuelve el método
```

```typescript
function prefijar(etiqueta: string) {
  return function <This, Args extends unknown[]>(
    metodo: (this: This, ...args: Args) => string,
    _contexto: ClassMethodDecoratorContext<This, (this: This, ...args: Args) => string>,
  ) {
    return function (this: This, ...args: Args): string {
      return `${etiqueta} ${metodo.call(this, ...args)}`;
    };
  };
}

class Tienda {
  @prefijar("[caja]")
  pagar(monto: number): string {
    return `${monto} € pagados`;
  }

  @prefijar("[almacén]")
  guardar(articulo: string): string {
    return `${articulo} guardado`;
  }
}

const tienda = new Tienda();
console.log(tienda.pagar(12));
console.log(tienda.guardar("jabón"));
```

```sortie
[caja] 12 € pagados
[almacén] jabón guardado
```

Fíjate en los **paréntesis**: `@prefijar("[caja]")` llama primero a `prefijar`, que
devuelve el verdadero decorador. Sin paréntesis, TypeScript creería que `prefijar`
mismo es el decorador (mira los [errores frecuentes](#errores-frecuentes)).

> **Recuerda:** una fábrica es una función que devuelve un decorador. Se escribe con
> paréntesis: `@limite(0, 100)`.

## Lo que hace el compilador

Node 22 todavía no conoce la sintaxis `@`. Con el objetivo `es2022` (el de los ejemplos
de este sitio), `tsc` **reescribe** por tanto cada decorador en JavaScript normal:
unas funciones de ayuda llamadas `__esDecorate` y `__runInitializers` llaman a tus
decoradores en el momento adecuado. Por eso todos los ejemplos de esta página funcionan
en Node 22.

No tienes nada que configurar: sin ninguna opción especial, TypeScript 5 usa estos
decoradores **estándar**.

> **Atención:** muchos artículos y bibliotecas (Angular, NestJS, TypeORM) usan los
> decoradores **antiguos**, activados con la opción `"experimentalDecorators": true`
> en `tsconfig.json`. Reciben otros parámetros (`target, key, descriptor`) y no se
> mezclan con los de esta página. Si un proyecto ya usa esta opción, sigue la
> documentación de su biblioteca.

> **Recuerda:** los decoradores estándar funcionan sin ninguna opción; `tsc` los
> traduce a funciones normales para Node.

## Errores frecuentes

### Escribir un decorador a la antigua

Un ejemplo copiado de un tutorial viejo espera tres parámetros:

```typescript norun
function registrarLlamadas(objetivo: any, clave: string, descriptor: PropertyDescriptor) {
  return descriptor;
}

class Calc {
  @registrarLlamadas
  sumar(a: number, b: number): number {
    return a + b;
  }
}
```

```text
main.ts(6,3): error TS1241: Unable to resolve signature of method decorator when called as an expression.
  The runtime will invoke the decorator with 2 arguments, but the decorator expects 3.
main.ts(6,4): error TS1270: Decorator function return type 'PropertyDescriptor' is not assignable to type 'void | ((a: number, b: number) => number)'.
```

Un decorador estándar recibe **dos** parámetros: el método y el contexto. Devuelve una
función (o nada). Toma como modelo `registrarLlamadas`, visto más arriba.

### Olvidar los paréntesis de una fábrica

```typescript norun
function limite(min: number, max: number) {
  return function (
    objetivo: ClassAccessorDecoratorTarget<unknown, number>,
    contexto: ClassAccessorDecoratorContext<unknown, number>,
  ) {
    return objetivo;
  };
}

class Jugador {
  @limite accessor vida = 100;
}
```

```text
main.ts(11,4): error TS1240: Unable to resolve signature of property decorator when called as an expression.
  Argument of type 'ClassAccessorDecoratorTarget<Jugador, number>' is not assignable to parameter of type 'number'.
main.ts(11,4): error TS1270: Decorator function return type '(objetivo: ClassAccessorDecoratorTarget<unknown, number>, contexto: ClassAccessorDecoratorContext<unknown, number>) => ClassAccessorDecoratorTarget<...>' is not assignable to type 'void | ClassAccessorDecoratorResult<Jugador, number>'.
```

`limite` espera `min` y `max`, no el objetivo. Llama a la fábrica:
`@limite(0, 100) accessor vida = 100;`.

### Olvidar la palabra accessor

```typescript norun
function limite<This>(
  objetivo: ClassAccessorDecoratorTarget<This, number>,
  contexto: ClassAccessorDecoratorContext<This, number>,
) {
  return objetivo;
}

class Jugador {
  @limite vida = 100;
}
```

```text
main.ts(9,4): error TS1240: Unable to resolve signature of property decorator when called as an expression.
  Argument of type 'undefined' is not assignable to parameter of type 'ClassAccessorDecoratorTarget<Jugador, number>'.
main.ts(9,4): error TS1270: Decorator function return type 'ClassAccessorDecoratorTarget<Jugador, number>' is not assignable to type 'void | ((this: Jugador, value: number) => number)'.
```

Sin `accessor`, `vida` es un campo simple: el decorador recibe `undefined`, no un
objetivo. Escribe `@limite accessor vida = 100;`.

## Ponte a prueba

**Pregunta 1:** ¿en qué orden se muestran `A` y `B`?

```typescript
function marcar(_m: () => void, _c: ClassMethodDecoratorContext) {
  console.log("A");
}

class Puerta {
  @marcar
  abrir() {}
}

console.log("B");
new Puerta().abrir();
```

> **Respuesta**
>
> `A` y luego `B`. El decorador se ejecuta una vez, cuando se crea la clase, antes de la
> línea `console.log("B")`. Como no devuelve nada, `abrir` sigue siendo el método
> original.
>
> ```sortie
> A
> B
> ```

**Pregunta 2:** un decorador de campo transforma `"paris"` en `"PARIS"`. ¿Cuánto vale
el campo después de `ciudad.nombre = "niza"`?

> **Respuesta**
>
> `"niza"`, en minúsculas. Un decorador de campo solo transforma el valor inicial. Para
> vigilar cada escritura, usa `accessor` y un decorador de accesor.

**Pregunta 3:** escribe una fábrica `repetir(n)` que envuelva un método que devuelve un
texto y devuelva ese texto repetido `n` veces.

> **Respuesta**
>
> ```typescript
> function repetir(n: number) {
>   return function <This>(
>     metodo: (this: This) => string,
>     _contexto: ClassMethodDecoratorContext<This, (this: This) => string>,
>   ) {
>     return function (this: This): string {
>       return metodo.call(this).repeat(n);
>     };
>   };
> }
>
> class Eco {
>   @repetir(3)
>   gritar(): string {
>     return "¡ho! ";
>   }
> }
>
> console.log(new Eco().gritar());
> ```
>
> ```sortie
> ¡ho! ¡ho! ¡ho!
> ```

## En resumen

- Un decorador es una función que se coloca con `@nombre` encima de un miembro de una
  clase: lo envuelve para añadirle un comportamiento.
- Un decorador de método recibe el método y el contexto, y devuelve la función que lo
  sustituye.
- Se ejecuta una vez al crear la clase; la función envuelta, en cada llamada.
- Un decorador de campo solo transforma el valor inicial; con `accessor`, se intercepta
  cada lectura y escritura.
- Una fábrica recibe parámetros y devuelve un decorador: `@limite(0, 100)`.
- Los decoradores estándar funcionan sin ninguna opción; `experimentalDecorators` es el
  sistema antiguo, diferente.

Siguiente: [gestionar los errores](erreurs.md).
