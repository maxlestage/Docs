# Aprender bien a programar

A programar se aprende programando, un poco cada día: esta página te da un método, ideas de proyectos, consejos para leer la documentación, hacer preguntas y usar la IA, y un plan de cuatro semanas.

> **Objetivos:**
>
> - seguir un método sencillo: leer, teclear, modificar, romper;
> - organizar tu práctica en sesiones cortas y regulares;
> - encontrar la respuesta en la documentación oficial;
> - hacer una buena pregunta y usar un asistente de IA sin dejar de aprender;
> - seguir un plan de cuatro semanas en el lenguaje que elijas.

## Programar se aprende como un instrumento

Nadie aprende a tocar el piano viendo vídeos. Hay que poner los dedos en
el teclado, equivocarse, volver a empezar.

Con la programación pasa lo mismo: leer este sitio no basta. Son tus dedos
y tus errores los que te enseñan. La buena noticia: veinte minutos al día
bastan para avanzar rápido.

## El método en cuatro gestos

Para cada ejemplo de este sitio, haz estos cuatro gestos, en orden.

### 1. Leer

Lee el código **antes** de ejecutarlo y adivina lo que va a mostrar.
Escribe tu respuesta en un papel. Toma este ejemplo en Ruby:

```ruby
1.upto(3) do |n|
  puts "3 x #{n} = #{3 * n}"
end
```

¿Qué muestra? Piénsalo y después mira:

```sortie
3 x 1 = 3
3 x 2 = 6
3 x 3 = 9
```

Si te has equivocado, mejor: acabas de aprender algo.

### 2. Teclear el código tú mismo

No copies y pegues. **Teclea** el código, letra a letra, en tu
[editor](editeur.md). Es más lento, pero tus dedos memorizan la sintaxis:
dónde van los paréntesis, las comillas, el `end`.

### 3. Modificar

Cambia una sola cosa y adivina el resultado antes de volver a ejecutar.
Por ejemplo, en Rust, cuenta hacia atrás con `.rev()`:

```rust
for n in (1..=3).rev() {
    println!("3 x {n} = {}", 3 * n);
}
```

```sortie
3 x 3 = 9
3 x 2 = 6
3 x 1 = 3
```

Ideas de modificaciones: cambiar un número, añadir una línea, usar otra
tabla de multiplicar, llegar hasta 10.

### 4. Romper

Quita a propósito un trozo y lee el mensaje de error. En Rythme, quitemos
el último paréntesis:

```rythme norun
each(1..3, fn(n) = print("3 x {n} = {3 * n}")
```

```text
error[E001]: se esperaba `,` o `)`, pero se encontró el final del código
 --> main.ry:2:1
  |
2 | 
  | ^ se esperaba `,` o `)` aquí
  |
1 | each(1..3, fn(n) = print("3 x {n} = {3 * n}")
  |     - abierto aquí
  |
  = más sobre este error: `rythme explain E001`
```

El día que cometas este error sin querer, reconocerás el mensaje
enseguida. Esta es la versión arreglada:

```rythme
each(1..3, fn(n) = print("3 x {n} = {3 * n}"))
```

```sortie
3 x 1 = 3
3 x 2 = 6
3 x 3 = 9
```

> **Recuerda:** leer, teclear, modificar, romper. Los errores no son un
> fracaso: son tus mejores lecciones. Consulta también
> [Encontrar y corregir tus errores](deboguer.md).

## Practicar un poco cada día

El cerebro recuerda mejor lo que repasa varias veces, con pausas entre los
repasos. Es la **práctica espaciada**.

- **Veinte o treinta minutos al día** valen más que cinco horas el
  domingo.
- **Vuelve a un capítulo** dos días después, y luego una semana después:
  rehaz sus ejemplos de memoria, sin mirar.
- **Empieza cada sesión** reescribiendo de memoria un pequeño programa del
  día anterior.
- **Para** cuando estés cansado: el cerebro sigue ordenando durante el
  descanso y el sueño.

> **Consejo:** las secciones «Ponte a prueba» de cada capítulo están
> hechas para repasar. Responde sin mirar y después comprueba.

## Pequeños proyectos, por nivel

Un proyecto tuyo te motiva mucho más que un ejercicio. Elige uno pequeño,
termínalo y pasa al siguiente.

| Nivel         | Ideas de proyectos                                                      |
|---------------|-------------------------------------------------------------------------|
| Principiante absoluto | una tabla de multiplicar; un conversor de euros ↔ dólares; un contador de vocales en una frase |
| Principiante  | un juego de «adivina el número»; una lista de la compra; un cálculo de la media de unas notas |
| Intermedio    | una agenda de contactos guardada en un archivo; un generador de contraseñas; un contador de palabras de un libro |
| Avanzado      | un pequeño juego en el terminal (ahorcado, tres en raya); una herramienta de línea de comandos; un pequeño sitio web o una API |

Las páginas de proyectos de cada lenguaje te guían paso a paso:
[Python](../python/projets.md), [JavaScript](../javascript/projets.md),
[TypeScript](../typescript/projets.md), [Go](../go/projets.md),
[Ruby](../ruby/projets.md), [Rust](../rust/projets.md) y
[Rythme](../rythme/projets.md).

## Leer la documentación

La documentación oficial es la fuente más fiable. Al principio impone un
poco; acostúmbrate a consultarla pronto.

- **Python**: [docs.python.org/es](https://docs.python.org/es/3/) existe
  en español. El comando `python3 -m pydoc str` la muestra en el terminal.
- **JavaScript**: [MDN](https://developer.mozilla.org/es/) describe todo
  el lenguaje, en español, con ejemplos para probar.
- **TypeScript**: [typescriptlang.org/docs](https://www.typescriptlang.org/docs/)
  contiene el manual oficial y un *playground* en el navegador.
- **Go**: [go.dev/doc](https://go.dev/doc/) y
  [pkg.go.dev/std](https://pkg.go.dev/std) describen el lenguaje y su
  biblioteca estándar. `go doc strings.ToUpper` la muestra en el terminal.
- **Ruby**: [ruby-doc.org](https://ruby-doc.org/) describe cada clase.
  Busca por ejemplo `String` para ver todos los métodos de los textos.
- **Rust**: [doc.rust-lang.org/std](https://doc.rust-lang.org/std/)
  describe la biblioteca estándar. El comando `rustup doc --std` la abre
  incluso sin Internet.
- **Rythme**: el comando `rythme help` muestra los temas, y `rythme help`
  seguido de un nombre explica una función.

```sh
rythme help reverse
```

```text
reverse(x)
  La lista o el texto en orden inverso.
  Ejemplo:
    reverse([1, 2, 3])         # [3, 2, 1]
```

El propio código también puede informarte. En Ruby, `methods` muestra lo
que sabe hacer un valor:

```ruby
p "hola".methods.grep(/case/).sort
puts 5.respond_to?(:even?)
```

```sortie
[:casecmp, :casecmp?, :downcase, :downcase!, :swapcase, :swapcase!, :upcase, :upcase!]
true
```

> **Consejo:** en una página de documentación, lee primero el **ejemplo**.
> A menudo dice más que la descripción.

## Hacer una buena pregunta

¿Llevas veinte minutos atascado? Pide ayuda. Una buena pregunta recibe
una respuesta rápida. Contiene:

1. **lo que quieres hacer**: «quiero mostrar los números pares de una
   lista»;
2. **el código**, lo más corto posible, que muestra el problema (copiado
   como texto, no como foto);
3. **el mensaje de error completo**, copiado tal cual;
4. **lo que ya has probado**.

A menudo, al escribir la pregunta, encuentras la respuesta tú mismo. Es el
método del [patito de goma](deboguer.md).

## Las comunidades en español

No estás solo. Hay personas que ayudan gratis, en español:

- **Stack Overflow en español** (es.stackoverflow.com): preguntas y
  respuestas sobre todos los lenguajes;
- **freeCodeCamp en español** (freecodecamp.org/espanol): artículos y
  cursos gratuitos para principiantes;
- **las comunidades de Python** (es.python.org, python.org.ar…):
  asociaciones con foros, listas de correo y encuentros;
- **los encuentros locales** (*meetups*): grupos de Ruby, Rust o
  JavaScript en muchas ciudades, a menudo anunciados en meetup.com;
- **Rythme**: las preguntas y los bugs se plantean en la página de GitHub
  del proyecto.

Muchos recursos siguen estando en inglés. Un traductor en línea te ayuda a
leerlos: no dudes en usarlo.

## Usar un asistente de IA con sensatez

Un asistente de IA (un chatbot que escribe código) puede ser un profesor
excelente, o impedirte aprender. Todo depende de cómo lo uses.

**Buenas formas:**

- pedirle que te **explique** un mensaje de error o un trozo de código;
- pedirle una **pista**, no la solución: «dame una pista, sin el código»;
- pedirle que **revise** tu código cuando ya funcione, para aprender a
  escribirlo mejor;
- pedirle que te **haga preguntas** para comprobar que lo has entendido.

**Malas formas:**

- copiar y pegar su respuesta sin entenderla;
- hacer que resuelva los ejercicios: eres tú quien debe progresar, no él.

> **Atención:** una IA a veces se equivoca con mucha seguridad. Puede
> inventarse un método que no existe. Prueba siempre su código y
> compruébalo en la documentación.

La regla de oro: **si no puedes explicar cada línea, no la uses.**

## Un plan de cuatro semanas

Elige **un** lenguaje (consulta [¿Qué lenguaje elegir?](choisir.md)) y
sigue su plan. Cuenta con cuatro o cinco sesiones por semana. Al final de
cada semana, haz uno de los [pequeños proyectos](#pequenos-proyectos-por-nivel).

### En Python

| Semana | Capítulos                                                                                                   | Al final, sabes…                        |
|--------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1      | [Introducción](../python/introduction.md), [Primeros pasos](../python/premiers-pas.md), [Números](../python/nombres.md), [Condiciones y bucles](../python/conditions-boucles.md) | calcular, elegir, repetir |
| 2      | [Funciones](../python/fonctions.md), [Texto](../python/texte.md), [Colecciones](../python/collections.md), [Diccionarios](../python/dictionnaires.md) | guardar datos y transformarlos |
| 3      | [Comprensiones](../python/comprehensions.md), [Errores](../python/erreurs.md), [Archivos](../python/fichiers.md), [Ejercicios](../python/exercices.md) | leer un archivo y gestionar los errores |
| 4      | [Clases y tipos](../python/types.md), [Módulos](../python/modules.md), [Pruebas](../python/tests.md), [Proyecto guiado](../python/projet.md) | terminar un pequeño programa de verdad |

### En JavaScript

| Semana | Capítulos                                                                                                   | Al final, sabes…                        |
|--------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1      | [Introducción](../javascript/introduction.md), [Primeros pasos](../javascript/premiers-pas.md), [Números](../javascript/nombres.md), [Condiciones y bucles](../javascript/conditions-boucles.md) | calcular, elegir, repetir |
| 2      | [Funciones](../javascript/fonctions.md), [Texto](../javascript/texte.md), [Arrays](../javascript/collections.md), [Objetos](../javascript/objets.md) | guardar datos y transformarlos |
| 3      | [Errores](../javascript/erreurs.md), [Asincronía](../javascript/asynchrone.md), [El DOM](../javascript/dom.md), [Ejercicios](../javascript/exercices.md) | esperar una respuesta y animar una página |
| 4      | [Módulos](../javascript/modules.md), [Archivos](../javascript/fichiers.md), [Pruebas](../javascript/tests.md), [Proyecto guiado](../javascript/projet.md) | terminar un pequeño programa de verdad |

### En TypeScript

Empieza TypeScript después de las dos primeras semanas de JavaScript.

| Semana | Capítulos                                                                                                   | Al final, sabes…                        |
|--------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1      | [Introducción](../typescript/introduction.md), [Primeros pasos](../typescript/premiers-pas.md), [Condiciones y bucles](../typescript/conditions-boucles.md), [Funciones](../typescript/fonctions.md) | tipar variables y funciones |
| 2      | [Texto](../typescript/texte.md), [Colecciones](../typescript/collections.md), [Objetos](../typescript/objets.md), [Uniones](../typescript/unions.md) | describir la forma de tus datos |
| 3      | [Tipos](../typescript/types.md), [Genéricos](../typescript/generiques.md), [Errores](../typescript/erreurs.md), [Asincronía](../typescript/asynchrone.md) | escribir código reutilizable y seguro |
| 4      | [Módulos](../typescript/modules.md), [Configuración](../typescript/configuration.md), [Pruebas](../typescript/tests.md), [Proyecto guiado](../typescript/projet.md) | terminar un pequeño programa de verdad |

### En Go

| Semana | Capítulos                                                                                                   | Al final, sabes…                        |
|--------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1      | [Introducción](../go/introduction.md), [Primeros pasos](../go/premiers-pas.md), [Números](../go/nombres.md), [Condiciones y bucles](../go/conditions-boucles.md) | compilar, calcular, elegir, repetir |
| 2      | [Funciones](../go/fonctions.md), [Texto](../go/texte.md), [Slices](../go/collections.md), [Maps](../go/maps.md) | guardar datos y transformarlos |
| 3      | [Tipos](../go/types.md), [Interfaces](../go/interfaces.md), [Punteros](../go/pointeurs.md), [Errores](../go/erreurs.md) | modelar datos y gestionar los errores |
| 4      | [Módulos](../go/modules.md), [Archivos](../go/fichiers.md), [Pruebas](../go/tests.md), [Proyecto guiado](../go/projet.md) | terminar un pequeño programa de verdad |

### En Ruby

| Semana | Capítulos                                                                                                   | Al final, sabes…                        |
|--------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1      | [Introducción](../ruby/introduction.md), [Primeros pasos](../ruby/premiers-pas.md), [Números](../ruby/nombres.md), [Condiciones y bucles](../ruby/conditions-boucles.md) | calcular, elegir, repetir               |
| 2      | [Funciones](../ruby/fonctions.md), [Texto](../ruby/texte.md), [Colecciones](../ruby/collections.md), [Rangos](../ruby/intervalles.md) | guardar datos y transformarlos          |
| 3      | [Bloques](../ruby/blocs.md), [Errores](../ruby/erreurs.md), [Archivos](../ruby/fichiers.md), [Ejercicios](../ruby/exercices.md) | leer un archivo y gestionar los errores |
| 4      | [Tipos y clases](../ruby/types.md), [Pruebas](../ruby/tests.md), [Proyecto guiado](../ruby/projet.md)       | terminar un pequeño programa de verdad  |

### En Rust

| Semana | Capítulos                                                                                                   | Al final, sabes…                        |
|--------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1      | [Introducción](../rust/introduction.md), [Primeros pasos](../rust/premiers-pas.md), [Números](../rust/nombres.md), [Condiciones y bucles](../rust/conditions-boucles.md) | compilar, calcular, elegir, repetir     |
| 2      | [Funciones](../rust/fonctions.md), [Texto](../rust/texte.md), [Colecciones](../rust/collections.md), [Propiedad](../rust/possession.md) | entender quién es dueño de un valor     |
| 3      | [Tipos](../rust/types.md), [Pattern matching](../rust/pattern-matching.md), [Errores](../rust/erreurs.md), [Iteradores](../rust/iterateurs.md) | modelar datos y gestionar los errores |
| 4      | [Módulos](../rust/modules.md), [Archivos](../rust/fichiers.md), [Pruebas](../rust/tests.md), [Proyecto guiado](../rust/projet.md) | terminar un pequeño programa de verdad  |

### En Rythme

| Semana | Capítulos                                                                                                   | Al final, sabes…                        |
|--------|-------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| 1      | [Introducción](../rythme/introduction.md), [Primeros pasos](../rythme/premiers-pas.md), [Números](../rythme/nombres.md), [Condiciones y bucles](../rythme/conditions-boucles.md) | calcular, elegir, repetir               |
| 2      | [Funciones](../rythme/fonctions.md), [Texto](../rythme/texte.md), [Colecciones](../rythme/collections.md), [El estilo funcional](../rythme/fonctionnel.md) | transformar listas sin bucles           |
| 3      | [Recursión](../rythme/recursion.md), [Pattern matching](../rythme/pattern-matching.md), [Errores](../rythme/erreurs.md), [Archivos](../rythme/fichiers.md) | dividir un problema y gestionar los errores |
| 4      | [Tipos](../rythme/types.md), [Pruebas](../rythme/tests.md), [Ejercicios](../rythme/exercices.md), [Proyecto guiado](../rythme/projet.md) | terminar un pequeño programa de verdad  |

> **Nota:** tardar dos semanas en lugar de una no es un problema. El
> ritmo regular importa más que la velocidad.

## Errores frecuentes

### Copiar código con comillas tipográficas

Un código copiado desde un procesador de textos o una página web a veces
contiene comillas curvas `“ ”` en lugar de las comillas rectas `"`:

```ruby norun
puts “Hola”
```

```text
main.rb:1:in `<main>': undefined local variable or method `“Hola”' for main (NameError)
```

En Rythme, el mensaje te dice directamente qué hacer:

```rythme norun
print(“Hola”)
```

```text
error[E018]: carácter inesperado `“`
 --> main.ry:1:7
  |
1 | print(“Hola”)
  |       ^
  |
  = ayuda: usa comillas dobles rectas `"` para los textos
  = más sobre este error: `rythme explain E018`
```

Teclea el código tú mismo: es la mejor protección.

### Fiarse de un método inventado

Un asistente de IA, o un sitio poco serio, te propone un método que
parece lógico, pero que no existe:

```ruby norun
puts "hola mundo".capitalize_words
```

```text
main.rb:1:in `<main>': undefined method `capitalize_words' for an instance of String (NoMethodError)

puts "hola mundo".capitalize_words
                 ^^^^^^^^^^^^^^^^^
Did you mean?  capitalize
```

Compruébalo en la documentación. Aquí, se pone mayúscula a cada palabra
con `"hola mundo".split.map(&:capitalize).join(" ")`.

### Mezclar los lenguajes

Después de leer Ruby o Python, es fácil olvidar el `!` de `println!` en
Rust:

```rust norun
fn main() {
    println("Hola");
}
```

```text
error[E0423]: expected function, found macro `println`
 --> main.rs:2:5
  |
2 |     println("Hola");
  |     ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
2 |     println!("Hola");
  |            +
```

Aprende un lenguaje cada vez y ten su chuleta a mano:
[Python](../python/aide-memoire.md), [JavaScript](../javascript/aide-memoire.md),
[TypeScript](../typescript/aide-memoire.md), [Go](../go/aide-memoire.md),
[Ruby](../ruby/aide-memoire.md), [Rust](../rust/aide-memoire.md),
[Rythme](../rythme/aide-memoire.md).

## Ponte a prueba

**1.** ¿Cuáles son los cuatro gestos del método, en orden?

> **Respuesta**
>
> Leer (y adivinar el resultado), teclear el código uno mismo,
> modificarlo y romperlo a propósito para leer el error.

**2.** ¿Es mejor programar cuatro horas el sábado o treinta minutos cada
día?

> **Respuesta**
>
> Treinta minutos cada día: la práctica espaciada ayuda al cerebro a
> recordar, y los repasos regulares fijan lo que has aprendido.

**3.** Un asistente de IA te da un código que funciona, pero no entiendes
dos líneas. ¿Qué haces?

> **Respuesta**
>
> Le pides que te las explique, lo compruebas en la documentación y
> después reescribes el código tú mismo. Ninguna línea que no sepas
> explicar.

## En resumen

- Se aprende practicando: leer, teclear, modificar, romper.
- Un poco cada día, con repasos espaciados, es mejor que una sesión larga.
- Los proyectos pequeños terminados motivan más que los grandes proyectos abandonados.
- La documentación oficial (docs.python.org, MDN, go.dev, ruby-doc.org, doc.rust-lang.org/std, `rythme help`…) es tu mejor amiga.
- Una buena pregunta contiene el objetivo, el código corto, el error completo y lo que has probado.
- La IA ayuda a entender; no debe hacer el trabajo por ti.

Siguiente: [¿qué lenguaje elegir?](choisir.md).
