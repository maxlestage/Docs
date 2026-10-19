# Glosario

Este glosario explica de forma sencilla las palabras de la programación que encontrarás en esta documentación, ordenadas alfabéticamente.

## A

### Algoritmo

Una serie de pasos precisos para resolver un problema, como una receta.
Un programa es un algoritmo escrito en un lenguaje de programación.
Consulta [Pensar como un programador](algorithmes.md) y
[pseudocódigo](#pseudocodigo).

### Ámbito

La zona del código donde existe una variable. Una variable creada dentro
de una función no existe fuera de ella. En inglés: *scope*.

### Any

En TypeScript, el tipo «cualquier cosa»: **apaga** la comprobación de
tipos. Un valor `any` lo acepta todo, incluso los errores. Es mejor usar
[unknown](#unknown), que obliga a comprobar antes de usarlo.

### API

Una lista de servicios que un programa ofrece a otros programas. En la
web, son direcciones que responden con datos, a menudo en [JSON](#json),
en lugar de con una página. El nombre viene del inglés *Application
Programming Interface*. Consulta
[Cómo funciona la web](web.md#las-api-y-json).

### Archivo fuente

El archivo de texto que contiene tu código, por ejemplo `hola.rb`,
`main.rs` o `hola.ry`.

### Argumento

El valor que le das a una [función](#funcion) cuando la llamas. En
`doble(21)`, el argumento es `21`.

### ASCII

Un código antiguo que da un número a las letras inglesas, a las cifras y a
los signos de puntuación: `A` vale 65, `a` vale 97. [Unicode](#unicode) lo
retoma y lo amplía a todas las escrituras.

```ruby
puts "A".ord
puts 97.chr
```

```sortie
65
a
```

### Async

Código **asíncrono**: mientras una tarea espera (una respuesta de la red,
un archivo), el programa se ocupa de otra cosa en lugar de quedarse
bloqueado. En Python y en JavaScript, se escribe con `async` y `await`; en
Rust, con `async` y `.await`. Consulta también [promesa](#promesa) y
[lo asíncrono en Rust](../rust/async.md).

## B

### Biblioteca

Código ya escrito por otras personas, que puedes reutilizar en tu
programa. Te evita reinventarlo todo. Consulta también
[paquete](#paquete).

### Binario

La forma de contar del ordenador, con solo dos cifras: 0 y 1. `101` en
binario vale 5. Consulta [Cómo funciona un ordenador](ordinateur.md).

```rust
println!("{:b}", 5);
```

```sortie
101
```

### Bit

El trozo de información más pequeño: un `0` o un `1`. Ocho bits forman un
[byte](#byte).

### Bloque

Un grupo de instrucciones que van juntas. En Ruby y en Rythme, un bloque
se escribe a menudo entre `do` y `end`; en Rust, entre llaves `{ }`.

```ruby
3.times do
  puts "Hola"
end
```

```sortie
Hola
Hola
Hola
```

### Booleano

Un valor que solo puede ser **verdadero** o **falso**: `true` o `false`.
Es el resultado de una comparación como `10 > 3`.

### Bucle

Una instrucción que repite código varias veces, por ejemplo para cada
elemento de una lista. Consulta también [iteración](#iteracion).

```ruby
[1, 2, 3].each { |n| puts n * 10 }
```

```sortie
10
20
30
```

### Bug

Un error en un programa: se cuelga, o no hace lo que querías. La palabra
significa «insecto» en inglés.

### Byte

Un grupo de ocho [bits](#bit). Puede valer de 0 a 255. El tamaño de los
archivos se mide en bytes: KB (mil), MB (millón), GB (mil millones). En
español también se dice **octeto**.

## C

### Cadena de caracteres

Un texto, escrito entre comillas: `"Hola"`. A menudo se dice **cadena**,
o *string* en inglés.

### Canal

En Go, un tubo por el que las [goroutines](#goroutine) se envían valores.
Se envía con `canal <- valor` y se recibe con `<-canal`.

```go
package main

import "fmt"

func main() {
	canal := make(chan string)
	go func() { canal <- "Hola desde una goroutine" }()
	fmt.Println(<-canal)
}
```

```sortie
Hola desde una goroutine
```

### Cargo

La herramienta de Rust que crea un proyecto (`cargo new`), lo compila y lo
ejecuta (`cargo run`), y descarga sus [crates](#crate).

### Clase

Un modelo para fabricar [objetos](#objeto) del mismo tipo. La clase
`Perro` describe qué es un perro; cada perro creado es un objeto. Ruby usa
mucho las clases; Rythme no las tiene.

### Cliente

El programa que **pide** algo a un [servidor](#servidor). Tu navegador es
un cliente: pide páginas web.

### Closure

Una función sin nombre que recuerda las variables que la rodean, allí
donde se creó. En español a veces se dice **clausura**.

```ruby
extra = 5
sumar_extra = ->(x) { x + extra }
puts sumar_extra.call(10)
```

```sortie
15
```

### Comentario

Un texto dentro del código que el ordenador ignora. Sirve para explicar el
código a las personas. En Ruby y en Rythme, empieza por `#`; en Rust, por
`//`.

```ruby
# Esto es un comentario: no se muestra nada.
puts "Solo cuenta esta línea"
```

```sortie
Solo cuenta esta línea
```

### Commit

Una versión guardada de tu proyecto con [Git](#git), con un mensaje que
dice qué ha cambiado. Consulta [Guardar tu trabajo con Git](git.md).

### Compilación

El trabajo del [compilador](#compilador): traducir todo el código a
lenguaje máquina antes de ejecutarlo. En Rust, `cargo build` compila;
`cargo run` compila y después ejecuta.

### Compilador

Un programa que traduce todo tu código a lenguaje máquina **antes** de
ejecutarlo. Go y Rust usan un compilador. Consulta también
[intérprete](#interprete).

### Comprensión

En Python, una forma corta de construir una [lista](#lista) a partir de
otra, en una sola línea: «el cuadrado de cada `n` de 1 a 4».

```python
cuadrados = [n * n for n in range(1, 5)]
print(cuadrados)
```

```sortie
[1, 4, 9, 16]
```

### Concurrencia

Hacer avanzar varias tareas a la vez, por ejemplo con varios
[hilos](#hilo). Consulta [la concurrencia en Rust](../rust/concurrence.md)
y [en Ruby](../ruby/concurrence.md).

### Condición

Una prueba que decide qué código ejecutar: «**si** llueve, coge un
paraguas; **si no**, unas gafas de sol». Se escribe con `if`.

```ruby
tiempo = "lluvia"
if tiempo == "lluvia"
  puts "Paraguas"
else
  puts "Gafas de sol"
end
```

```sortie
Paraguas
```

### Conjunto

Una colección **sin duplicados**, en la que el orden de llegada no
importa. Un elemento está o no está. En inglés: *set*.

```rythme
print(set([3, 1, 3, 2]))
```

```sortie
set(1, 2, 3)
```

### Constante

Un valor con nombre que no debe cambiar nunca, como `PI = 3.14159`. Por
costumbre, su nombre suele escribirse en mayúsculas.

### Crate

Un [paquete](#paquete) de Rust: una biblioteca o un programa. Se
encuentran en el sitio `crates.io`, y [Cargo](#cargo) los descarga.

## D

### Decorador

En Python, una función que **envuelve** otra función para añadirle algo,
sin tocar su código. Se coloca encima con `@`.

```python
def anuncia(funcion):
    def envoltorio():
        print("Inicio")
        funcion()
    return envoltorio

@anuncia
def hola():
    print("Hola")

hola()
```

```sortie
Inicio
Hola
```

### Depuración

Buscar y corregir un [bug](#bug). Se leen los mensajes de error, se
muestran valores, se prueba poco a poco. En inglés: *debugging*. Consulta
[Encontrar y corregir tus errores](deboguer.md).

### Diccionario

Una colección que asocia un **valor** a una **clave**, igual que un
diccionario de verdad asocia una definición a una palabra. En Python se
dice *dict*; en JavaScript, *objeto* o *Map*; en Go, *map*; en Ruby,
*hash*; en Rust, *HashMap*.

```ruby
edades = { "Ada" => 36, "Alan" => 41 }
puts edades["Ada"]
```

```sortie
36
```

### DOM

La página web tal como el navegador la guarda en memoria: un árbol de
elementos (títulos, párrafos, botones) que JavaScript puede leer y
modificar. El nombre viene del inglés *Document Object Model*.

```javascript norun
document.querySelector("h1").textContent = "¡Hola!";
```

## E

### Editor de código

El programa en el que se escribe código, como VS Code. Consulta
[Instalar un editor de código](editeur.md).

### Entero

Un número sin decimales: `0`, `42`, `-7`. En inglés: *integer*. Consulta
también [flotante](#flotante).

### Entorno virtual

En Python, una carpeta que contiene los [paquetes](#paquete) de **un
solo** proyecto, separados de los demás. Se crea con
`python3 -m venv .venv` y se activa antes de usar [pip](#pip).

### Enumeración

Un [tipo](#tipo) que solo puede tomar un valor de una lista fija: un
semáforo está en `Rojo`, `Ambar` o `Verde`. En Rust, se escribe con
`enum`; en Rythme, con `type`.

### Enumerador

En Ruby, un valor que da los elementos de una colección **uno a uno**,
cuando se le piden, con `next`. Es el primo en Ruby del
[iterador](#iterador). Consulta
[los enumeradores](../ruby/enumerateurs.md).

```ruby
letras = ["a", "b", "c"].each
puts letras.next
puts letras.next
```

```sortie
a
b
```

### Error

Un mensaje que dice que algo ha salido mal. A menudo indica el archivo, la
línea y la causa. Léelo: es tu mejor pista.

### Estrechamiento

En TypeScript, cuando una comprobación **precisa** el tipo de un valor:
después de `typeof x === "string"`, TypeScript sabe que `x` es un texto.
En inglés se dice *narrowing*.

```typescript
function longitud(x: string | number): number {
  if (typeof x === "string") {
    return x.length;
  }
  return x;
}

console.log(longitud("Ada"), longitud(7));
```

```sortie
3 7
```

### Estructura

Un [tipo](#tipo) que agrupa varios valores con nombre, sus **campos**. En
Rust, se escribe con `struct`; en Ruby, `Struct` crea una rápidamente.

```rust
struct Persona {
    nombre: String,
    edad: u32,
}

let ada = Persona { nombre: String::from("Ada"), edad: 36 };
println!("{} tiene {} años", ada.nombre, ada.edad);
```

```sortie
Ada tiene 36 años
```

### Excepción

En Ruby, un error que interrumpe el programa, salvo si se captura con
`rescue`. Rust y Rythme no las tienen: usan [Result](#result).

```ruby
begin
  Integer("abc")
rescue ArgumentError
  puts "No es un número."
end
```

```sortie
No es un número.
```

### Expresión

Un trozo de código que da un [valor](#valor). `2 + 3` es una expresión
que vale `5`; `"Ada"` es una expresión que vale `"Ada"`.

### Expresión regular

Un patrón que describe la forma de un texto, para buscarlo o reemplazarlo. Casi siempre se dice [regex](#regex).

## F

### Flotante

Un número con decimales: `3.14`, `0.5`. En programación, los decimales se
separan con un **punto**. En inglés: *float*.

### Función

Un trozo de código que tiene un nombre y que se puede reutilizar. Recibe
[argumentos](#argumento) y devuelve un resultado.

```rythme
fn doble(x) = x * 2
print(doble(21))
```

```sortie
42
```

## G

### Gem

Un [paquete](#paquete) de Ruby. Se instala con `gem install nombre`, o se
añade al archivo `Gemfile` de un proyecto.

### Generador

Una función que da sus valores **uno a uno**, a medida que se le piden, en
lugar de calcularlo todo de golpe. En Python y en JavaScript, usa
`yield`.

```python
def cuenta_atras(n):
    while n > 0:
        yield n
        n -= 1

for n in cuenta_atras(3):
    print(n)
```

```sortie
3
2
1
```

### Genérico

Código escrito una sola vez para varios [tipos](#tipo). En Rust, `Vec<T>`
es una lista de elementos de cualquier tipo `T`: `Vec<i32>`,
`Vec<String>`... TypeScript (`Array<T>`) y Go (`[T any]`) tienen la misma
idea.

### Git

Un programa que guarda el historial de tu código: cada versión es un
[commit](#commit), guardado en un [repositorio](#repositorio). Consulta
[Guardar tu trabajo con Git](git.md).

### Goroutine

En Go, una función que se ejecuta **al mismo tiempo** que el resto del
programa. Se lanza escribiendo `go` delante de la llamada. A menudo se
comunican mediante [canales](#canal). Consulta también
[concurrencia](#concurrencia).

## H

### Herencia

En Ruby, una [clase](#clase) puede tomar todo lo que sabe hacer otra clase
y después añadir sus propios métodos. Se dice que **hereda** de ella, con
`<`.

```ruby
class Animal
  def respirar = "Respiro."
end

class Gato < Animal
end

puts Gato.new.respirar
```

```sortie
Respiro.
```

### Hilo

Un hilo de ejecución (*thread* en inglés): una parte del programa que se
ejecuta al mismo tiempo que las demás. Varios hilos permiten hacer varias
cosas a la vez. Consulta [la concurrencia en Rust](../rust/concurrence.md).

### HTML

El lenguaje que describe la **estructura** de una página web: títulos,
párrafos, enlaces, imágenes. Se escribe con etiquetas como `<h1>` y
`</h1>`. Consulta [Cómo funciona la web](web.md#html-css-y-javascript).

### HTTP

El idioma que hablan el [cliente](#cliente) y el [servidor](#servidor) en
la web: una **petición** (`GET /pagina`) y después una **respuesta** con
un código, como `200` (todo va bien) o `404` (página no encontrada).
`HTTPS` es su versión cifrada.

## I

### Indentación

El desplazamiento de una línea hacia la derecha, con espacios. Muestra lo
que está **dentro** de un bucle o de una función. En Python, la
indentación forma parte de la sintaxis: una línea mal desplazada es un
error.

### Índice

La posición de un elemento en una [lista](#lista). Se cuenta desde
**cero**: el primer elemento tiene el índice `0`.

```ruby
frutas = ["manzana", "pera", "kiwi"]
puts frutas[0]
puts frutas[2]
```

```sortie
manzana
kiwi
```

### Inmutable

Que no puede cambiar. Un valor inmutable sigue siendo el mismo de
principio a fin. En Rythme, todos los valores son inmutables; en Rust, las
variables lo son por defecto. Consulta también [mutable](#mutable).

### Interfaz

Una lista de lo que un valor debe **saber hacer** o **contener**, sin
decir cómo. En Go, un tipo que tiene los métodos adecuados cumple la
interfaz por sí solo; en TypeScript, una interfaz describe la forma de un
objeto.

```typescript
interface Animal {
  nombre: string;
  sonido(): string;
}

const gato: Animal = { nombre: "Félix", sonido: () => "Miau" };
console.log(`${gato.nombre} dice ${gato.sonido()}`);
```

```sortie
Félix dice Miau
```

### Intérprete

Un programa que lee tu código y lo ejecuta enseguida, línea tras línea.
Python, JavaScript, Ruby y Rythme usan un intérprete. Consulta también
[compilador](#compilador).

### Iteración

Una pasada por un [bucle](#bucle). Iterar sobre una lista es tratar sus
elementos uno a uno.

### Iterador

Un valor que da los elementos de una colección uno a uno. En Rust,
`.iter()` crea un iterador, sobre el que se encadenan `map`, `filter` o
`sum`.

```rust
let total: i32 = [1, 2, 3].iter().map(|x| x * 10).sum();
println!("{total}");
```

```sortie
60
```

## J

### JSON

Un formato de texto muy extendido para intercambiar datos entre
programas: `{"nombre": "Ada", "edad": 36}`. Las claves siempre van entre
comillas dobles.

```rythme
print(to_json({nombre: "Ada", edad: 36}))
```

```sortie
{"nombre": "Ada", "edad": 36}
```

## L

### Lambda

Una [función](#funcion) sin nombre, guardada en una variable. En Ruby, se
escribe con `->`; a menudo también es una [closure](#closure). Consulta
[procs y lambdas](../ruby/procs-lambdas.md).

```ruby
doble = ->(x) { x * 2 }
puts doble.call(21)
```

```sortie
42
```

### Lista

Una serie ordenada de valores: `[1, 2, 3]`. En Ruby se dice *array*; en
Rust, *Vec*. Cada elemento tiene un [índice](#indice).

## M

### Macro

En Rust, una palabra que termina en `!`, como `println!` o `vec!`.
Escribe código por ti, antes de la compilación.

### Match

Una instrucción que compara un valor con varias formas posibles y elige
el código de la primera que encaja. Rust y Rythme la usan mucho; Ruby
tiene `case`.

```rythme
nota = 15
print(match nota
  0..9 -> "insuficiente"
  _ -> "bien"
end)
```

```sortie
bien
```

### Memoria RAM

La memoria rápida donde un programa guarda sus datos **mientras se
ejecuta**. Se borra al apagar el ordenador, a diferencia del disco. En
inglés: *RAM*. Consulta [Cómo funciona un ordenador](ordinateur.md).

### Metaprogramación

Código que escribe o modifica código mientras el programa se ejecuta, por
ejemplo para crear métodos automáticamente. Ruby lo permite mucho.
Consulta [la metaprogramación en Ruby](../ruby/metaprogrammation.md).

### Método

Una función unida a un valor. Se llama con un punto: `"hola".upcase`
llama al método `upcase` del texto.

```ruby
puts "hola".upcase
```

```sortie
HOLA
```

### Mixin

En Ruby, un [módulo](#modulo) cuyos métodos se añaden a una
[clase](#clase) con `include`. Así, varias clases pueden compartir los
mismos métodos.

### Módulo

Una forma de agrupar bajo un mismo nombre código que va junto, a menudo en
un archivo aparte. Ayuda a organizar un programa grande.

### Mutable

Que puede cambiar. Una variable mutable puede recibir un valor nuevo. En
Rust, se indica con `mut`.

```rust
let mut contador = 1;
contador = contador + 1;
println!("{contador}");
```

```sortie
2
```

## N

### Node.js

Un programa que ejecuta JavaScript **fuera del navegador**: en el
terminal o en un servidor. Se usa con `node main.mjs`.

### npm

El gestor de [paquetes](#paquete) de JavaScript, que viene con
[Node.js](#node-js). `npm install nombre` descarga un paquete; la lista de
paquetes de un proyecto está en el archivo `package.json`.

### Null

Un valor especial que quiere decir «nada» o «ausente». Python lo llama
`None`; JavaScript tiene `null` y `undefined`; Go y Ruby lo llaman `nil`.
Rust y Rythme no tienen `null`: usan `None`, que obliga a prever el caso
en que falta el valor.

## O

### Objeto

Un valor que agrupa datos y [métodos](#metodo). En Ruby, todo es un
objeto: incluso el número `5`.

### Operador

Un símbolo que hace una operación: `+`, `-`, `*`, `/` para calcular, `==`
para comparar, `&&` para «y».

### Option

Un valor que puede estar presente, `Some(valor)`, o ausente, `None`. Rust
y Rythme lo usan en lugar de [null](#null): el programa debe prever los
dos casos.

```rythme
print(first([5, 8]), first([]))
```

```sortie
Some(5) None
```

## P

### Pánico

En Rust, la parada brusca de un programa ante un error imposible de
superar, como leer el elemento 10 de una lista de 3 elementos. El mensaje
empieza por `panicked at`.

### Paquete

Una [biblioteca](#biblioteca) lista para instalar. En Python, se instala
con [pip](#pip); en JavaScript, con [npm](#npm); en Go, con `go get`. En
Ruby se dice *gem*; en Rust, *crate*.

### Parámetro

El nombre que recibe el [argumento](#argumento) en la definición de una
función. En `fn doble(x) = x * 2`, el parámetro es `x`.

### Pila y montón

Los dos lugares donde un programa guarda sus datos en memoria. La **pila**
(*stack*) es rápida y ordenada, como una pila de platos: ahí van los
valores pequeños de tamaño conocido. El **montón** (*heap*) es más grande
y más libre: ahí van los datos cuyo tamaño cambia, como una lista que
crece. Rust te hace pensar en esta diferencia.

### pip

El gestor de [paquetes](#paquete) de Python. `pip install nombre`
descarga un paquete desde PyPI, el catálogo oficial. Úsalo dentro de un
[entorno virtual](#entorno-virtual).

### Préstamo

En Rust, usar un valor sin quitárselo a su propietario, con `&`. También
se dice **referencia**. Consulta [propiedad](#propiedad).

### Procesador

La pieza del ordenador que ejecuta las instrucciones, miles de millones
por segundo. Solo entiende el lenguaje máquina. En inglés: *CPU*.

### Proceso

Un [programa](#programa) en ejecución. El mismo programa lanzado dos
veces da dos procesos, cada uno con su memoria.

### Programa

Una lista de instrucciones que ejecuta un ordenador. Consulta
[¿Qué es programar?](programmer.md).

### Promesa

En JavaScript, un valor que **llegará más tarde** (*Promise* en inglés),
por ejemplo la respuesta de un servidor. `await` espera a que esté lista.
Consulta también [async](#async).

```javascript
const promesa = new Promise((resolver) => {
  setTimeout(() => resolver("listo"), 10);
});
console.log(await promesa);
```

```sortie
listo
```

### Propiedad

La regla de Rust que dice que cada valor tiene **un solo propietario**.
Cuando el propietario desaparece, el valor se libera. En inglés:
*ownership*. Consulta [la propiedad en Rust](../rust/possession.md).

### Prototipo

En JavaScript, el objeto modelo del que otro objeto **hereda** sus
métodos. Cuando falta una propiedad, JavaScript la busca en el prototipo.
Las clases de JavaScript se basan en este mecanismo. Consulta también
[herencia](#herencia).

### Pseudocódigo

Un algoritmo escrito en español sencillo, con la forma de un programa,
antes de escribirlo en un lenguaje real. Consulta
[Pensar como un programador](algorithmes.md).

```text
para cada número de la lista
    si el número es par
        mostrarlo
```

### Puntero

Un valor que contiene la **dirección** de otro valor en memoria, como un
cartel que indica dónde está algo. En Rust, `&x` y `Box<T>` son tipos de
punteros.

## R

### Rango

Una serie de números definida por su inicio y su fin: `1..10`. En Rust,
`1..10` se detiene en 9 y `1..=10` llega hasta 10.

### Recursión

Cuando una función se llama a sí misma, con un problema más pequeño, hasta
llegar a un caso sencillo. Aquí se suman los números desde `n` hasta 0:

```rythme
fn total(0) = 0
fn total(n) = n + total(n - 1)
print(total(4))
```

```sortie
10
```

### Regex

Abreviatura de **expresión regular** (*regular expression*): un pequeño
lenguaje para describir la forma de un texto, para buscarlo, dividirlo o
reemplazarlo. `[0-9]+` quiere decir «una o varias cifras». Consulta
[las regex en Ruby](../ruby/regex.md) y [en Rythme](../rythme/regex.md).

```rythme
print(find_all("Tel.: 06 12, fijo: 01 45", "[0-9]+"))
```

```sortie
["06", "12", "01", "45"]
```

### Registro

En Rythme, un valor que agrupa campos con nombre:
`{nombre: "Ada", edad: 36}`. Un campo se lee con un punto.

```rythme
persona = {nombre: "Ada", edad: 36}
print(persona.nombre)
```

```sortie
Ada
```

### REPL

Un modo en el que escribes una línea de código y ves el resultado
enseguida. El nombre viene del inglés *Read, Eval, Print, Loop* (leer,
evaluar, mostrar, repetir). Ejemplos: `irb` para Ruby, `rythme` para
Rythme.

### Repositorio

Una carpeta seguida por [Git](#git), con todo su historial. En inglés:
*repository*, o *repo*. Un repositorio también puede estar en línea, en
GitHub.

### Result

El resultado de una operación que puede fallar: `Ok(valor)` en caso de
éxito, `Err(error)` en caso de fallo. Rust y Rythme lo usan en lugar de
las [excepciones](#excepcion).

```rythme
print(parse_int("42"), parse_int("abc") |> is_err)
```

```sortie
Ok(42) true
```

### Ruta

La dirección de un archivo o de una carpeta: `/home/alicia/code/hola.rb`.
Una ruta **absoluta** parte de la raíz del disco; una ruta **relativa**
parte de la carpeta en la que estás, y `..` designa la carpeta padre.

## S

### Serialización

Convertir un valor del programa en texto (o en bytes), para guardarlo en
un archivo o enviarlo por la red. Lo contrario, volver a leer ese texto,
se llama **deserialización**. [JSON](#json) es un formato de
serialización.

### Servidor

Un programa (y, por extensión, el ordenador que lo ejecuta) que espera
peticiones y las **responde**. Un servidor web envía páginas a los
[clientes](#cliente).

### Símbolo

En Ruby y en Rythme, un nombre fijo que empieza por dos puntos: `:ok`,
`:rojo`. Sirve de etiqueta, a menudo como clave de un
[diccionario](#diccionario).

### Sintaxis

Las reglas de escritura de un lenguaje: dónde poner los paréntesis, las
comillas, las palabras clave. Un fallo de sintaxis impide que el programa
arranque.

### Sistema operativo

El programa que arranca primero y gestiona todo el ordenador: reparte el
procesador y la memoria, guarda los archivos y lanza los programas.
Ejemplos: Windows, macOS, Linux, Android.

### Slice

En Go, una lista que puede crecer. `append` le añade elementos.

```go
package main

import "fmt"

func main() {
	notas := []int{12, 15}
	notas = append(notas, 9)
	fmt.Println(notas, len(notas))
}
```

```sortie
[12 15 9] 3
```

## T

### Terminal

Una ventana en la que se escriben comandos con el teclado. Consulta
[El terminal en 10 minutos](terminal.md).

### Test

Un pequeño trozo de código que comprueba que otro código funciona bien
(en español también se dice **prueba**). Si cambias tu programa, los
tests te avisan cuando rompes algo.

### Tiempo de vida

En Rust, el periodo durante el que una [referencia](#prestamo) sigue
siendo válida. El compilador comprueba que una referencia nunca vive más
que el valor al que apunta. En inglés: *lifetime*. Consulta
[los tiempos de vida en Rust](../rust/durees-de-vie.md).

### Tipado

La forma en que un lenguaje comprueba los [tipos](#tipo). **Estático**:
antes de ejecutar el programa (Go, Rust, TypeScript). **Dinámico**:
mientras se ejecuta (Python, JavaScript, Ruby, Rythme).

### Tipo

La clase de un valor: un [entero](#entero), un texto, un
[booleano](#booleano), una [lista](#lista)... El tipo dice lo que se puede
hacer con el valor.

```ruby
puts 42.class
puts "Ada".class
```

```sortie
Integer
String
```

### Tipo literal

En TypeScript, un tipo que solo acepta **un único valor** concreto, como
`"rojo"`. Con una [unión](#union-de-tipos), se enumeran los valores
permitidos.

```typescript
type Semaforo = "rojo" | "ambar" | "verde";
const luz: Semaforo = "verde";
console.log(luz);
```

```sortie
verde
```

### Trait

En Rust, una lista de métodos que un [tipo](#tipo) promete saber hacer.
El trait `Display`, por ejemplo, dice que un valor sabe mostrarse con
`{}`.

### Tupla

Un pequeño grupo de valores, de tipos quizá diferentes, sin nombres de
campos. En Rust, sus elementos se leen por su posición: `.0`, `.1`.

```rust
let punto = (3, 7);
println!("x = {}, y = {}", punto.0, punto.1);
```

```sortie
x = 3, y = 7
```

## U

### Unicode

El código que da un número a **cada carácter** de todas las escrituras del
mundo, emojis incluidos: `é` vale 233, `€` vale 8364. Casi siempre se
guarda en **UTF-8**.

```rythme
print(char_code("é"), char(8364))
```

```sortie
233 €
```

### Unión de tipos

En TypeScript, un tipo que acepta **varios** tipos, separados por `|`:
`string | number` es «un texto o un número». Antes de usarlo, se hace un
[estrechamiento](#estrechamiento).

### Unknown

En TypeScript, el tipo de un valor **desconocido**: TypeScript no deja
usarlo hasta que se comprueba qué es. Es la versión segura de
[any](#any).

```typescript
const valor: unknown = JSON.parse("42");
if (typeof valor === "number") {
  console.log(valor + 1);
}
```

```sortie
43
```

### URL

La dirección de un recurso en la web:
`https://www.ejemplo.es/recetas?raciones=4`. Indica el protocolo, la
máquina, la ruta y a veces unos parámetros. Consulta
[Cómo funciona la web](web.md#la-direccion-la-url).

## V

### Valor

Un dato: un número, un texto, una lista, `true`... Es lo que maneja un
programa.

### Variable

Un nombre que designa un [valor](#valor), como una etiqueta pegada en una
caja. Sirve para encontrar el valor más tarde.

```ruby
nombre = "Ada"
puts nombre
```

```sortie
Ada
```

## En resumen

- Un programa maneja valores: enteros, flotantes, textos, booleanos, listas.
- Los valores se guardan en variables, y el código se reutiliza con funciones.
- Las condiciones eligen, los bucles repiten.
- Un intérprete ejecuta enseguida, un compilador traduce primero.
- Un bug se corrige leyendo el mensaje de error: es la depuración.
