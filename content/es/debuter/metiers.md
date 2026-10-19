# Los oficios del desarrollo

Saber programar abre la puerta a muchos oficios distintos: esta página describe los principales con palabras sencillas, con los lenguajes de este sitio que les corresponden y una idea de primer proyecto para probar.

> **Objetivos:**
>
> - descubrir diez oficios del desarrollo y lo que se hace en ellos durante un día;
> - saber qué lenguajes de este sitio llevan a cada oficio;
> - tener una idea de primer proyecto para cada uno;
> - saber cómo empezar: pequeños proyectos, GitHub, lectura de código;
> - conocer los caminos de formación y las cualidades que cuentan.

## Un solo saber, muchos oficios

Piensa en la cocina. Un pastelero, un chef de restaurante y un panadero
saben todos cocinar. Pero su día a día es muy diferente.

En programación pasa lo mismo. Las bases son las mismas en todas partes:
[variables](variables.md), [condiciones](logique.md),
[bucles](boucles.md), [funciones](fonctions.md). Lo que cambia es
**lo que construyes** y **para quién**.

Para cada oficio, encontrarás:

- **el día**: lo que haces, con palabras sencillas;
- **los lenguajes** de este sitio que le corresponden;
- **un primer proyecto** para probar en casa.

> **Recuerda:** no tienes que elegir tu oficio ahora.
> Prueba varios proyectos pequeños: verás lo que te gusta.

## Desarrollador front-end

**El front-end** es la parte de un sitio que se **ve**: las páginas,
los botones, los menús. Es el escaparate de la tienda.

- **El día**: convertir una maqueta (un dibujo de la página) en una página
  de verdad. Hacer que un botón se pueda pulsar. Comprobar que el sitio
  funciona en un teléfono. Corregir un menú que se muestra mal.
- **Los lenguajes**: [JavaScript](../javascript/introduction.md), después
  [TypeScript](../typescript/introduction.md), con
  [HTML](glossaire.md#html) y CSS (ver [la web](web.md)).
- **Un primer proyecto**: una página «lista de la compra» donde se añaden y
  se marcan artículos.

## Desarrollador back-end

**El back-end** es la parte **oculta**: el [servidor](glossaire.md#servidor)
que guarda los datos y responde a las peticiones. Es la cocina del
restaurante, que el cliente no ve.

- **El día**: escribir el código que guarda un pedido. Crear una
  [API](glossaire.md#api) (una ventanilla donde otros programas hacen sus
  preguntas). Organizar las [bases de datos](bases-de-donnees.md).
- **Los lenguajes**: [Python](../python/introduction.md),
  [Go](../go/introduction.md), [Ruby](../ruby/introduction.md),
  JavaScript con Node.js, o [Rust](../rust/introduction.md).
- **Un primer proyecto**: un pequeño servidor que devuelve una cita al
  azar cuando se visita una dirección.

## Desarrollador full-stack

**Full-stack** quiere decir «toda la pila»: el front-end **y** el
back-end. Es la persona que lleva a la vez el escaparate y la cocina.

- **El día**: añadir una funcionalidad de principio a fin. Por ejemplo,
  el botón «Me gusta»: el botón en la página, y el registro del
  «me gusta» en el servidor.
- **Los lenguajes**: JavaScript y TypeScript en los dos lados, o
  JavaScript delante y Python, Ruby o Go detrás.
- **Un primer proyecto**: un libro de visitas. Los visitantes escriben un
  mensaje en una página; el servidor lo guarda; la página muestra todos
  los mensajes.

## Desarrollador móvil

Crea las **aplicaciones** de tu teléfono.

- **El día**: construir una pantalla de la aplicación. Probar en
  varios teléfonos. Publicar una actualización en las tiendas de
  aplicaciones.
- **Los lenguajes**: los lenguajes de este sitio sirven sobre todo para
  aprender las bases. El oficio usa a menudo Kotlin (Android) o Swift
  (iPhone). Con JavaScript o TypeScript, herramientas como React Native
  crean aplicaciones para los dos.
- **Un primer proyecto**: un temporizador para cocer la pasta.

## Datos e inteligencia artificial

Se trabaja con **datos**: tablas de números, textos,
imágenes. Se les hacen preguntas. A veces, se enseña a un
programa a reconocer cosas (eso es la IA).

- **El día**: limpiar un archivo de datos lleno de huecos. Hacer un
  gráfico. Responder a una pregunta como «¿Qué producto se vende
  más los sábados?».
- **Los lenguajes**: [Python](../python/introduction.md), con mucha
  diferencia, más el lenguaje SQL de las [bases de datos](bases-de-donnees.md).
- **Un primer proyecto**: leer un archivo con tus gastos y calcular el
  total por categoría.

## DevOps y nube

**DevOps** mezcla «desarrollo» y «operaciones». Estas personas se
aseguran de que los programas **funcionen bien**, día y noche, en
servidores alquilados en línea (la **nube**, *cloud* en inglés). Son los
guardianes de la sala de máquinas.

- **El día**: automatizar la publicación de un sitio. Vigilar los
  servidores. Reaccionar cuando un servicio se cae.
- **Los lenguajes**: [Go](../go/introduction.md) (muchas herramientas de
  la nube están escritas en Go), [Python](../python/introduction.md), y la
  [terminal](terminal.md) todos los días.
- **Un primer proyecto**: un script que comprueba cada minuto que un
  sitio responde, y muestra una alerta si no.

## Desarrollador de sistemas embebidos

El código **embebido** funciona dentro de objetos: una lavadora, un
coche, un termostato, un reloj. La máquina es pequeña, con poca
[memoria](glossaire.md#memoria-ram).

- **El día**: hacer parpadear una luz en el momento justo. Leer un
  sensor de temperatura. Buscar por qué el aparato se reinicia
  solo.
- **Los lenguajes**: [Rust](../rust/introduction.md) (cada vez más),
  y C, muy usado en este oficio.
- **Un primer proyecto**: con una pequeña placa electrónica barata,
  encender una luz cuando oscurece.

## Desarrollador de videojuegos

Escribe las **reglas** del juego: cómo salta el personaje, qué
pasa cuando toca a un enemigo, cómo sube la puntuación.

- **El día**: ajustar la altura de un salto. Corregir un personaje que
  atraviesa una pared. Hacer que el juego sea más fluido.
- **Los lenguajes**: los grandes estudios usan a menudo C++ o C#. Para
  empezar, bastan [JavaScript](../javascript/introduction.md) (juegos en
  el navegador), [Python](../python/introduction.md) o
  [Rust](../rust/introduction.md).
- **Un primer proyecto**: «Adivina el número» en la terminal, y después
  un pequeño juego de la serpiente.

## Seguridad informática

Se **protegen** los programas y los datos contra los ataques. Es el
cerrajero de la informática: busca las puertas mal cerradas antes que
los ladrones.

- **El día**: revisar código para encontrar fallos. Probar un sitio
  como lo haría un atacante, con permiso. Explicar al equipo
  cómo corregirlo.
- **Los lenguajes**: [Python](../python/introduction.md) para las
  pequeñas herramientas, y saber **leer** muchos lenguajes. Lee la página
  [seguridad](securite.md).
- **Un primer proyecto**: un programa que dice si una contraseña es
  débil o sólida.

## QA y pruebas

**QA** quiere decir *quality assurance*, «aseguramiento de la calidad».
Se comprueba que el software **hace lo que promete**, antes que los
usuarios.

- **El día**: escribir [pruebas](glossaire.md#test) automáticas.
  Probar el software de todas las maneras, incluso las raras. Escribir
  informes de [errores](glossaire.md#bug) claros.
- **Los lenguajes**: [Python](../python/introduction.md),
  [JavaScript](../javascript/introduction.md) o
  [TypeScript](../typescript/introduction.md), según el proyecto probado.
- **Un primer proyecto**: escribir cinco pruebas para una función que
  calcula una media. ¿Qué pasa con una lista vacía?

## La tabla resumen

| Oficio | Lo que se construye | Lenguajes de este sitio para empezar |
|---|---|---|
| Front-end | Las páginas que se ven | JavaScript, TypeScript |
| Back-end | Los servidores y los datos | Python, Go, Ruby, Rust, JavaScript |
| Full-stack | Las dos cosas | JavaScript, TypeScript + un lenguaje de servidor |
| Móvil | Las aplicaciones del teléfono | JavaScript, TypeScript (después Kotlin o Swift) |
| Datos / IA | Respuestas a partir de datos | Python |
| DevOps / nube | Programas que funcionan bien | Go, Python |
| Embebidos | El código de los objetos | Rust (y C) |
| Videojuegos | Las reglas del juego | JavaScript, Python, Rust |
| Seguridad | La protección | Python |
| QA / pruebas | La verificación | Python, JavaScript, TypeScript |

¿Y [Rythme](../rythme/introduction.md)? Es un lenguaje para
**aprender**. Prepara para todos estos oficios, pero ninguno lo usa.

> **Pruébalo:** elige dos filas de la tabla que te atraigan.
> Haz los dos primeros proyectos esta semana, en una hora cada uno.
> ¿Cuál te ha dado ganas de seguir?

## Cómo empezar

### Pequeños proyectos, no uno grande

Un **portafolio** es un escaparate de tus proyectos. Cinco pequeños
proyectos terminados valen más que un gran proyecto abandonado.

```text
Semana 1   → un conversor de temperaturas
Semana 2   → una lista de tareas en la terminal
Semana 3   → «Adivina el número»
Semana 4   → una página web con un botón
Semana 5   → rehacer uno de los proyectos, mejor
```

Ver también [pequeños proyectos, por nivel](apprendre.md#pequenos-proyectos-por-nivel).

### Ponerlo todo en GitHub

Pon cada proyecto en GitHub (ver [Git](git.md)), con un `README.md`
que diga en tres líneas:

- lo que hace el proyecto;
- cómo ejecutarlo;
- lo que has aprendido.

Quien contrata mira a menudo estas páginas.

### Leer el código de los demás

Se aprende a escribir leyendo. Abre un pequeño proyecto en GitHub y lee
un archivo. Después intenta [contribuir](open-source.md) a él.

> **Recuerda:** unos proyectos pequeños, terminados y visibles en GitHub
> muestran lo que sabes hacer mejor que un largo discurso.

## Los caminos de formación

No hay **un** buen camino. Hay tres grandes, y mucha
gente los mezcla.

| Camino | Cómo | Puntos fuertes | Puntos a vigilar |
|---|---|---|---|
| **Estudios** (universidad, formación profesional, escuela de ingeniería) | Varios años de estudios | Bases sólidas, título, prácticas | Largo; a menudo pide empezar joven |
| **Reconversión** (bootcamp) | Unos meses, a menudo para adultos | Rápido, guiado, en grupo | Muy intenso; revisa el programa y las opiniones de antiguos alumnos |
| **Autodidacta** (por tu cuenta) | Libros, sitios como este, proyectos | Gratis o casi, a tu ritmo | Hace falta constancia; nadie te guía |

Seamos sinceros: ninguna formación **garantiza** un empleo. Desconfía
de las que lo prometen. Lo que cuenta es lo que sabes
**hacer**, y lo que puedes **mostrar**.

## Las cualidades que cuentan de verdad

Programar también es trabajar con personas. Estas cualidades cuentan
tanto como el código:

- **Hacer preguntas.** Decir «no lo entiendo» es una fortaleza.
  Ver [hacer una buena pregunta](apprendre.md#hacer-una-buena-pregunta).
- **Escribir con claridad.** Un mensaje de commit, una issue, una
  documentación: se escribe mucho.
- **Mantener la calma ante un error.** Leer el mensaje, buscar, probar. Ver
  [leer un mensaje de error](lire-une-erreur.md) y [estoy atascado](bloque.md).
- **Ser curioso.** Las herramientas cambian; se aprende toda la vida.
- **Aceptar los comentarios.** Una revisión de código ayuda, no juzga.

## Errores frecuentes

Aquí no hay errores de código, sino **ideas falsas** que desaniman
a muchos principiantes.

### «Hay que ser bueno en matemáticas»

Falso para la mayoría de los oficios. El front-end, el back-end, el móvil
o las pruebas piden sobre todo **lógica**: dividir un problema en
pequeños pasos. Las sumas y los porcentajes suelen bastar.

Las matemáticas cuentan más en algunos campos: la IA, los juegos en 3D,
algunas herramientas de seguridad. Y también se aprenden, poco a poco.

### «Hay que saberlo todo antes de presentarse»

Nadie lo sabe todo, ni siquiera tras veinte años de oficio. Una oferta de
empleo suele enumerar muchas cosas: es una lista de deseos, no de
obligaciones.

Corrección: si tienes las bases y algunos proyectos que mostrar,
preséntate. Aprenderás el resto trabajando.

### «Hay que haber empezado de niño»

Muchos desarrolladores empezaron a los 25, 35 o 45 años, después de otro
oficio. Ese otro oficio es incluso una ventaja: ya conoces un
campo (la salud, el comercio, la enseñanza…).

### «Hay que conocer el mejor lenguaje»

No hay un mejor lenguaje, solo lenguajes adaptados a un
trabajo. Ver [qué lenguaje elegir](choisir.md). Después del primero, los
siguientes se aprenden mucho más rápido.

## Ponte a prueba

**1.** ¿Cuál es la diferencia entre el front-end y el back-end?

> **Respuesta**
>
> El front-end es la parte que se ve (las páginas, los botones). El
> back-end es la parte oculta (el servidor, los datos). El full-stack
> hace las dos cosas.

**2.** Te gustan los números y los gráficos. ¿Qué oficio y qué
lenguaje de este sitio deberías probar primero?

> **Respuesta**
>
> Los datos, con Python. Primer proyecto: leer un archivo de gastos y
> calcular el total por categoría.

**3.** No eres bueno en matemáticas y solo tienes tres pequeños
proyectos. ¿Debes esperar antes de buscar unas prácticas?

> **Respuesta**
>
> No. La mayoría de los oficios piden sobre todo lógica, y nadie
> lo sabe todo. Pon tus proyectos en GitHub con un README claro, y
> preséntate.

## En resumen

- Las bases de la programación son las mismas para todos los oficios; lo que cambia es lo que se construye.
- Front-end (lo que se ve), back-end (lo que está oculto), full-stack (los dos), pero también móvil, datos, DevOps, embebidos, juegos, seguridad y pruebas.
- Para empezar: pequeños proyectos terminados, en GitHub, con un README.
- Estudios, reconversión o autodidacta: ningún camino garantiza un empleo; cuenta lo que sabes mostrar.
- Hacer preguntas y escribir con claridad cuentan tanto como el código.

Siguiente: [el glosario](glossaire.md), para tenerlo siempre a mano.
