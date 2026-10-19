# ¿Qué es programar?

Programar es escribir instrucciones que un ordenador sigue al pie de la letra: esta página explica qué es un programa, un lenguaje, un bug, y cómo se aprende.

> **Objetivos:**
>
> - saber qué es un programa y un lenguaje de programación;
> - reconocer el código fuente y sus extensiones de archivo;
> - leer un mismo programa pequeño en varios lenguajes;
> - distinguir un lenguaje interpretado de un lenguaje compilado;
> - saber qué es un bug y cómo se aprende a programar.

## Un programa es una lista de instrucciones

Un ordenador no adivina nada. Hace exactamente lo que le dices, en
orden, muy rápido.

Un **programa** es una lista de instrucciones escritas para él. Por ejemplo:

```text
1. Preguntar el nombre de la persona.
2. Mostrar «Hola» seguido de ese nombre.
3. Detenerse.
```

**Programar** es escribir esas instrucciones. La persona que programa
se llama desarrollador o desarrolladora.

## Como una receta de cocina

Una receta se parece mucho a un programa:

- una lista de **ingredientes**: son los **datos** (números,
  textos);
- unos **pasos** en un orden preciso: son las **instrucciones**;
- unas **decisiones**: «si la masa está demasiado espesa, añade leche»;
- unas **repeticiones**: «remueve hasta que quede lisa».

La gran diferencia: un cocinero entiende «una pizca de sal».
El ordenador, no. Hay que decirlo todo, sin ambigüedad.

> **Recuerda:** un programa es una receta muy precisa, escrita para
> alguien que obedece sin pensar.

## El código fuente

Las instrucciones se escriben en un **archivo de texto**, como una nota. Ese
texto se llama **código fuente**, o simplemente **código**.

El nombre del archivo termina con una **extensión** que indica en qué lenguaje
está escrito: `.py` para Python, `.js` para JavaScript, `.ts` para
TypeScript, `.go` para Go, `.rb` para Ruby, `.rs` para Rust, `.ry` para
Rythme.

Para escribir ese texto se usa un **editor de código**: consulta
[Instalar un editor de código](editeur.md).

## Los lenguajes de programación

El ordenador no entiende el español. Entiende un **lenguaje de
programación**: un vocabulario reducido, con reglas estrictas. Esas reglas
se llaman **sintaxis**.

Existen cientos de lenguajes. Esta documentación presenta siete:
Python, JavaScript, TypeScript, Go, Ruby, Rust y Rythme. A menudo dicen
lo mismo con palabras distintas. Para saber por cuál empezar,
consulta [¿Qué lenguaje elegir?](choisir.md).

## Un mismo programa en tres lenguajes

Este pequeño programa guarda un nombre en una **variable**, saluda y luego
muestra el resultado de un cálculo.

En Ruby:

```ruby
nombre = "Ada"
puts "¡Hola, #{nombre}!"
puts 2 + 3
```

```sortie
¡Hola, Ada!
5
```

En Rust:

```rust
let nombre = "Ada";
println!("¡Hola, {nombre}!");
println!("{}", 2 + 3);
```

```sortie
¡Hola, Ada!
5
```

En Rythme:

```rythme
nombre = "Ada"
print("¡Hola, {nombre}!")
print(2 + 3)
```

```sortie
¡Hola, Ada!
5
```

### Y en Python o en JavaScript

En Python:

```python
nombre = "Ada"
print(f"¡Hola, {nombre}!")
print(2 + 3)
```

```sortie
¡Hola, Ada!
5
```

En JavaScript:

```javascript
const nombre = "Ada";
console.log(`¡Hola, ${nombre}!`);
console.log(2 + 3);
```

```sortie
¡Hola, Ada!
5
```

Todos hacen lo mismo. Solo cambian las palabras: `puts`, `println!`,
`print` o `console.log` significan todos «muestra».

## Interpretado o compilado

El ordenador tiene que transformar tu texto en acciones. Hay dos maneras de
hacerlo.

- **Interpretado**: un programa, el **intérprete**, lee tu código y
  lo ejecuta enseguida, línea tras línea. Es el caso de **Python**,
  **JavaScript**, **Ruby** y **Rythme**.
- **Compilado**: un programa, el **compilador**, traduce primero todo tu
  código a lenguaje máquina. Solo después se ejecuta el resultado. Es el
  caso de **Go** y **Rust**.

TypeScript es un caso aparte: primero se comprueba y se traduce a
JavaScript, que después se interpreta.

Piensa en un libro extranjero. El intérprete te lo lee en voz alta,
traduciendo sobre la marcha. El traductor, en cambio, escribe primero todo el
libro en español: tarda más al principio, pero luego la lectura es
muy rápida.

> **Nota:** un compilador comprueba muchas cosas antes de ejecutar
> nada. Encuentra los errores antes, pero también es más
> exigente.

## ¿Qué es un bug?

Un **bug** es un error en un programa. El programa no funciona, o
no hace lo que querías.

Hay dos grandes tipos de bugs:

- el ordenador **rechaza** tu código: un error de sintaxis, como un
  paréntesis olvidado. Te muestra un mensaje de error;
- el código **funciona, pero da un resultado incorrecto**: escribiste
  `a - b` en lugar de `a + b`. El ordenador no puede adivinarlo.

Buscar y corregir un bug se llama **depuración**.

> **Recuerda:** todo el mundo comete bugs, todo el tiempo, incluso los
> expertos. No es un fracaso: es el trabajo normal.

## Cómo se aprende a programar

No se aprende a programar solo leyendo. Se aprende probando.

1. **Lee** un ejemplo corto.
2. **Escríbelo** tú mismo, sin copiar y pegar.
3. **Ejecútalo** y mira lo que pasa.
4. **Cambia** algo: un número, una palabra. Vuelve a ejecutarlo.
5. **Equivócate**, y lee el mensaje de error.

Un mensaje de error no es un reproche. Es una pista. A menudo dice
**dónde** está el problema (el archivo y la línea) y **por qué**.

> **Consejo:** lee siempre la primera línea de un mensaje de error
> entera, despacio. La respuesta suele estar ahí.

## Y después

Antes de escribir código, hay que aprender a dividir un problema en
pasos: de eso trata [pensar como un programador](algorithmes.md).
Para ejecutar un programa, luego tendrás que abrir una terminal: consulta
[la terminal en 10 minutos](terminal.md). Las palabras nuevas se explican
en el [glosario](glossaire.md).

## Errores frecuentes

### Escribir las instrucciones en español

El ordenador solo conoce las palabras de su lenguaje. `mostrar` no existe
en Ruby:

```ruby norun
mostrar "Hola"
```

```text
fr.rb:1:in `<main>': undefined method `mostrar' for main (NoMethodError)

mostrar "Hola"
^^^^^^^
```

En Ruby, «mostrar» se dice `puts`: `puts "Hola"`.

### Olvidar las comillas alrededor de un texto

Sin comillas, el lenguaje cree que `Hola` es un nombre que debería
conocer:

```ruby norun
puts Hola
```

```text
q.rb:1:in `<main>': uninitialized constant Hola (NameError)

puts Hola
     ^^^^
```

Un texto siempre se escribe entre comillas: `puts "Hola"`.

### Cambiar una mayúscula

Para el ordenador, `nombre` y `Nombre` son dos nombres distintos:

```ruby norun
nombre = "Ada"
puts "Hola, " + Nombre
```

```text
pm.rb:2:in `<main>': uninitialized constant Nombre (NameError)

puts "Hola, " + Nombre
                ^^^^^^
```

Escribe el nombre exactamente como lo creaste: `nombre`.

## Ponte a prueba

**1.** ¿Qué tienen en común un programa y una receta de
cocina?

> **Respuesta**
>
> Los dos son listas de instrucciones en un orden preciso, con
> datos (los ingredientes), decisiones y repeticiones. Pero el
> programa tiene que ser mucho más preciso: el ordenador no adivina nada.

**2.** ¿Cuál es la diferencia entre un lenguaje interpretado y un lenguaje
compilado? Da un ejemplo de cada uno.

> **Respuesta**
>
> Un intérprete lee y ejecuta el código enseguida (Python, Ruby,
> Rythme). Un compilador traduce primero todo el código y luego se ejecuta el
> resultado (Go, Rust).

**3.** Tu programa se ejecuta pero muestra un resultado incorrecto. ¿Es un
bug?

> **Respuesta**
>
> Sí. Un bug no es solo un programa que falla: también es un
> programa que no hace lo que querías.

## En resumen

- Un programa es una lista de instrucciones precisas, como una receta.
- El código fuente es un archivo de texto escrito en un lenguaje de programación.
- Python, JavaScript, Ruby y Rythme son interpretados; Go y Rust se compilan antes de ejecutarse.
- Un bug es un error en el programa, y todo el mundo los comete.
- Se aprende probando, equivocándose y leyendo los mensajes de error.

Siguiente: [Cómo funciona un ordenador](ordinateur.md)
