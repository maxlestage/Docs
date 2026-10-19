# ¿Qué lenguaje elegir?

Python, JavaScript, TypeScript, Go, Ruby, Rust y Rythme tienen cada uno sus puntos fuertes: esta página los compara de forma sencilla y te propone un recorrido según lo que quieras hacer.

> **Objetivos:**
>
> - conocer los puntos fuertes y los límites de los siete lenguajes de este sitio;
> - comparar los lenguajes en una tabla y con un mismo programa pequeño;
> - elegir un recorrido según lo que quieras hacer;
> - saber por qué página empezar.

## No hay malas elecciones

Tu primer lenguaje no es un compromiso para toda la vida. Las ideas básicas
son las mismas en todas partes: variables, condiciones, bucles, funciones,
listas.

Cuando conoces un lenguaje, el segundo se aprende mucho más rápido. Lo
importante es **empezar**.

> **Recuerda:** aprender un lenguaje es, sobre todo, aprender a programar.
> Ese conocimiento te acompaña a todas partes.

## Python: el más usado para empezar

Python es un lenguaje muy legible: pocos símbolos, y la indentación (el
desplazamiento de las líneas) forma parte de la sintaxis.

- **Fácil de leer**: un programa en Python se parece al pseudocódigo.
- **Está en todas partes**: scripts, automatización, ciencia de datos,
  inteligencia artificial, sitios web.
- **Una comunidad enorme**: miles de [paquetes](glossaire.md#paquete) que
  se instalan con [pip](glossaire.md#pip), y respuestas en línea a casi
  todas las preguntas.

Su límite: es más lento que Go o Rust, y no se ejecuta directamente en un
navegador.

## JavaScript: el lenguaje de la web

JavaScript es el único lenguaje que entienden **todos los navegadores**.
Hace que las páginas web sean interactivas: un botón que reacciona, un
menú que se abre.

- **Nada que instalar para empezar**: tu navegador ya sabe ejecutarlo.
- **La web de principio a fin**: en la página (el
  [DOM](glossaire.md#dom)) y en el servidor, con Node.js.
- **Una comunidad inmensa**: el registro [npm](glossaire.md#npm) es el
  mayor catálogo de paquetes del mundo.

Su límite: deja pasar muchos errores. `"2" + 1` da `"21"`, sin avisar.
Algunas reglas antiguas sorprenden a los principiantes.

## TypeScript: JavaScript con tipos

TypeScript es JavaScript más unos **[tipos](glossaire.md#tipo)** escritos
en el código. Una herramienta, `tsc`, comprueba esos tipos antes de
ejecutar el programa y después lo convierte en JavaScript.

- **Menos sorpresas**: muchos errores se encuentran antes de la ejecución.
- **Un editor más listo**: completa el código y señala los fallos mientras
  escribes.
- **Muy demandado**: la mayoría de los grandes proyectos web lo usan.

Su límite: primero hay que conocer las bases de JavaScript. Apréndelo en
segundo lugar.

## Go: sencillo, rápido, para servidores

Go se creó en Google para escribir servidores y herramientas. Es
**pequeño** a propósito: pocas palabras clave y una sola forma de dar
formato al código.

- **Rápido**: es compilado, y un programa en Go arranca en un instante.
- **Trabajo en paralelo fácil**: las
  [goroutines](glossaire.md#goroutine) y los [canales](glossaire.md#canal)
  hacen varias cosas a la vez.
- **Un solo archivo que entregar**: el compilador produce un programa
  completo, sin nada más que instalar.

Su límite: es un poco **verboso**. A menudo se escriben las mismas líneas,
por ejemplo para comprobar cada error.

## Ruby: para crear rápido

Ruby es un lenguaje agradable de leer, casi como el inglés.

- **Fácil de escribir**: pocos símbolos, muchas cosas ya preparadas.
- **Los sitios web**: el framework Ruby on Rails sirve para construir
  muchos sitios conocidos.
- **Una comunidad acogedora**: miles de bibliotecas (las
  [gems](glossaire.md#gem)) y mucha ayuda en línea.

Su límite: es más lento que Go o Rust. En la mayoría de los programas, no
se nota.

## Rust: para ir muy rápido y entender la máquina

Rust es un lenguaje compilado, muy rápido y muy seguro.

- **Programas rápidos**: tan rápidos como los escritos en C o C++.
- **La seguridad**: el compilador rechaza muchos errores incluso antes de
  ejecutar el programa.
- **Entender el ordenador**: Rust te hace pensar en la memoria y en la
  forma en que se guardan los datos.

Su límite: es **más difícil** de aprender. El compilador es exigente, y
algunas ideas, como la [propiedad](../rust/possession.md), requieren
tiempo.

## Rythme: para aprender

Rythme es un lenguaje pequeño, pensado para principiantes.

- **Pocas reglas**: solo 15 palabras clave, y una sola forma de hacer cada
  cosa.
- **Errores que explican**: cada mensaje dice dónde está el problema y
  cómo corregirlo, en español.
- **Un estilo funcional**: los valores nunca cambian, lo que evita muchas
  sorpresas.

Su límite: es joven. Poca gente lo usa, y no se emplea en grandes
proyectos profesionales.

> **Nota:** el propio Rythme está escrito en Rust. Los lenguajes de esta
> documentación van de la mano.

## La tabla comparativa

| Lenguaje   | Facilidad           | Velocidad     | Usos principales                     | Dónde se ejecuta                  | Comunidad               |
|------------|---------------------|---------------|--------------------------------------|-----------------------------------|-------------------------|
| Python     | Muy fácil           | Media         | Scripts, datos, IA, web              | Ordenador, servidor               | Inmensa                 |
| JavaScript | Fácil               | Buena         | Páginas web, servidores, aplicaciones | Navegador, servidor (Node.js)    | Inmensa                 |
| TypeScript | Media               | Buena         | Grandes proyectos web                | Como JavaScript, tras `tsc`       | Muy grande              |
| Go         | Fácil               | Rápida        | Servidores, herramientas, nube       | Servidor, terminal                | Grande                  |
| Ruby       | Fácil               | Media         | Sitios web (Rails), scripts          | Ordenador, servidor               | Grande y acogedora      |
| Rust       | Difícil al principio | Muy rápida   | Herramientas, sistemas, juegos, web rápida | En todas partes, incluso el navegador | Grande, en crecimiento |
| Rythme     | Muy fácil           | Lenta         | Aprender, pequeños scripts           | Terminal, playground del navegador | Pequeña y naciente     |

Tres palabras para leer esta tabla:

- **Interpretados**: Python, JavaScript, Ruby y Rythme los lee y ejecuta
  directamente un [intérprete](glossaire.md#interprete).
- **Compilados**: Go y Rust los traduce primero a lenguaje máquina un
  [compilador](glossaire.md#compilador).
- **TypeScript** va aparte: se comprueba y después se traduce a
  JavaScript.

## Un ejemplo para comparar

Este es el mismo programa pequeño en los siete lenguajes: suma una lista
de números.

### En Python

```python
numeros = [4, 8, 15, 16, 23, 42]
print(sum(numeros))
```

```sortie
108
```

### En JavaScript

```javascript
const numeros = [4, 8, 15, 16, 23, 42];
const total = numeros.reduce((suma, n) => suma + n, 0);
console.log(total);
```

```sortie
108
```

JavaScript no tiene `sum`: `reduce` va sumando los números uno a uno,
empezando por `0`.

### En TypeScript

```typescript
const numeros: number[] = [4, 8, 15, 16, 23, 42];
const total: number = numeros.reduce((suma, n) => suma + n, 0);
console.log(total);
```

```sortie
108
```

Es el mismo código, con los tipos añadidos: `number[]` quiere decir «una
lista de números».

### En Go

```go
package main

import "fmt"

func main() {
	numeros := []int{4, 8, 15, 16, 23, 42}
	total := 0
	for _, n := range numeros {
		total += n
	}
	fmt.Println(total)
}
```

```sortie
108
```

Go pide un programa completo (`package main`, `func main`) y un bucle
escrito a mano: es más largo, pero todo está a la vista.

### En Ruby

```ruby
numeros = [4, 8, 15, 16, 23, 42]
puts numeros.sum
```

```sortie
108
```

### En Rust

```rust
let numeros = [4, 8, 15, 16, 23, 42];
let total: i32 = numeros.iter().sum();
println!("{total}");
```

```sortie
108
```

Rust quiere conocer el tipo del resultado: `i32`, un número entero.

### En Rythme

```rythme
numeros = [4, 8, 15, 16, 23, 42]
print(sum(numeros))
```

```sortie
108
```

Los siete hacen lo mismo. Python, Ruby y Rythme son los más cortos; Go es
el más explícito.

## Qué recorrido seguir

### «Quiero aprender a programar»

Empieza por **Python** o por **Rythme**. Python te abre enseguida las
puertas de proyectos reales; Rythme te enseña las ideas sin perderte en
los detalles, con errores en español.

### «Quiero hacer sitios web»

Empieza por **JavaScript** y luego pasa a **TypeScript**. JavaScript es
obligatorio en el navegador; TypeScript te ayuda después a escribir
proyectos más grandes sin perderte.

### «Quiero escribir servidores y herramientas»

Elige **Go**. Es sencillo, rápido y muy usado para servidores, la nube y
las herramientas de línea de comandos.

### «Quiero programas muy rápidos, o entender la máquina»

Elige **Rust**. Tómate tu tiempo: los comienzos son exigentes, pero el
compilador te enseña muchísimo.

### «Quiero escribir scripts, o usar Rails»

Elige **Ruby**. Es agradable para automatizar pequeñas tareas, y es el
camino directo hacia Ruby on Rails.

> **Consejo:** ¿todavía dudas? Elige Python, o Rythme. El
> [playground de Rythme](https://maxlestage.github.io/Rythme/) se abre en
> el navegador, sin instalar nada.

## Empezar

Cada recorrido empieza con una página de introducción, que explica cómo
instalar el lenguaje y ejecutar un primer programa:

- [Empezar con Python](../python/introduction.md)
- [Empezar con JavaScript](../javascript/introduction.md)
- [Empezar con TypeScript](../typescript/introduction.md)
- [Empezar con Go](../go/introduction.md)
- [Empezar con Ruby](../ruby/introduction.md)
- [Empezar con Rust](../rust/introduction.md)
- [Empezar con Rythme](../rythme/introduction.md)

Antes de lanzarte, asegúrate de saber usar [el terminal](terminal.md) y
[un editor de código](editeur.md).

## Errores frecuentes

Cuando pasas de un lenguaje a otro, enseguida mezclas las palabras. Cada
lenguaje lo señala a su manera.

### Escribir Ruby en Python

```python norun
puts "Hola"
```

```text
  File "main.py", line 1
    puts "Hola"
         ^^^^^^
SyntaxError: invalid syntax
```

En Python, se escribe `print("Hola")`, con paréntesis.

### Escribir Python en JavaScript

```javascript norun
print("Hola");
```

```text
file:///home/ada/main.mjs:1
print("Hola");
^

ReferenceError: print is not defined
    at file:///home/ada/main.mjs:1:1
```

En JavaScript, se escribe `console.log("Hola");`.

### Dejar una variable sin usar en Go

Go se niega a compilar un programa que crea una variable sin usarla:

```go norun
package main

import "fmt"

func main() {
	numeros := []int{4, 8, 15}
	fmt.Println("Hola")
}
```

```text
# command-line-arguments
./main.go:6:2: declared and not used: numeros
```

Usa la variable o borra la línea. Ruby, Python y JavaScript no dicen nada
en este caso.

### Escribir un bucle for en Rythme

```rythme norun
numeros = [1, 2, 3]
for n in numeros
  print(n)
end
```

```text
error[E001]: `for` no se usa en Rythme
 --> bucle.ry:2:1
  |
2 | for n in numeros
  | ^^^ viene de otro lenguaje
  |
  = ayuda: Rythme no tiene bucles. Trabaja con la lista entera en su lugar:
        1..5 |> each(fn(i) = print(i))
        [1, 2, 3] |> map(fn(x) = x * 2)
      o escribe una función recursiva. Consulta `rythme help functional`.
  = más sobre este error: `rythme explain E001`
```

En Rythme, se escribe `numeros |> each(print)`. Consulta
[condiciones y repetición](../rythme/conditions-boucles.md).

> **Consejo:** aprende un lenguaje cada vez. Cuando empieces un segundo,
> ten su chuleta abierta al lado, por ejemplo
> [la de Python](../python/aide-memoire.md).

## Ponte a prueba

**1.** Quieres crear páginas web interactivas. ¿Por qué lenguaje empiezas,
y cuál aprendes después?

> **Respuesta**
>
> Primero JavaScript, porque es el lenguaje de los navegadores. Después
> TypeScript, que añade tipos para evitar muchos errores.

**2.** De los siete lenguajes, ¿cuáles se compilan a lenguaje máquina?

> **Respuesta**
>
> Go y Rust. Python, JavaScript, Ruby y Rythme son interpretados;
> TypeScript se comprueba y después se traduce a JavaScript.

**3.** ¿Has hecho una «mala» elección si cambias de lenguaje más
adelante?

> **Respuesta**
>
> No. Las ideas básicas (variables, condiciones, bucles, funciones,
> listas) están en todas partes, y el segundo lenguaje se aprende mucho
> más rápido.

## En resumen

- El primer lenguaje no es una elección definitiva: las ideas básicas están en todas partes.
- Para aprender a programar: Python o Rythme.
- Para los sitios web: JavaScript y después TypeScript.
- Para los servidores y las herramientas: Go; para programas muy rápidos: Rust.
- Para los scripts y Ruby on Rails: Ruby.
- Cada recorrido empieza con su página de introducción.

Siguiente: [Contribuer à un projet open source](open-source.md)
