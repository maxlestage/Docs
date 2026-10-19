# Contribuir a un proyecto de código abierto

Un proyecto de código abierto (*open source*) es un programa cuyo código está abierto a todo el mundo: esta página explica qué significa y te guía, paso a paso, hasta tu primera contribución en GitHub.

> **Objetivos:**
>
> - entender qué es el código abierto y qué dice una licencia;
> - saber cómo ayudar a un proyecto, incluso como principiante;
> - leer un proyecto: README, CONTRIBUTING, issues;
> - hacer una contribución completa: fork, clone, rama, commit, push, pull request;
> - responder con educación a los comentarios de revisión.

## ¿Qué es el código abierto?

Imagina una receta de cocina expuesta en el escaparate de una panadería.
Todo el mundo puede leerla, prepararla en casa e incluso proponerle una
mejora al panadero.

Un programa de **código abierto** (en inglés, *open source*) funciona
igual:

- su código es **público**: todo el mundo puede leerlo;
- todo el mundo puede **usarlo** y **modificarlo**;
- todo el mundo puede **proponer cambios** al proyecto.

Ya usas algunos sin saberlo. Python, Node.js, Go, Ruby, Rust, Git,
Firefox y VLC son de código abierto.

> **Recuerda:** código abierto quiere decir «el código está abierto».
> Cualquiera puede leerlo, usarlo y proponer mejoras.

## Las licencias, en palabras sencillas

Una **licencia** es un texto corto, normalmente en un archivo `LICENSE`.
Dice lo que tienes derecho a hacer con el código. Hay dos licencias que
aparecen muy a menudo:

| Licencia | En una frase | Lo que pide |
|---|---|---|
| **MIT** | «Haz lo que quieras con este código.» | Conservar el nombre del autor y el texto de la licencia. |
| **GPL** | «Puedes hacer de todo, pero comparte tú también.» | Si distribuyes una versión modificada, su código debe seguir abierto, bajo GPL. |

Una imagen para la GPL: te prestan un libro de recetas, con la condición
de que tus nuevas recetas se presten de la misma manera.

Existen otras (Apache, BSD…): no hace falta aprenderlas de memoria.

> **Atención:** un código **sin licencia** no es libre, aunque sea público
> en GitHub. Por defecto, el autor conserva todos los derechos.

## ¿Por qué contribuir, incluso como principiante?

Quizá pienses: «No soy lo bastante bueno». No es verdad. Un proyecto
necesita muchas cosas que no son código complicado:

- **corregir una errata** en la documentación;
- **hacer más clara una explicación**, justamente porque no la entendiste
  a la primera;
- **traducir** una página al español;
- **informar de un [bug](glossaire.md#bug)** como es debido, en una issue;
- **añadir un ejemplo** o un [test](glossaire.md#test).

Y tú también ganas: lees código real, aprendes a trabajar en equipo con
[Git](glossaire.md#git) y recibes consejos de gente con experiencia.

> **Recuerda:** una errata corregida en una documentación es una
> contribución de verdad. Todo el mundo empieza por ahí.

## Leer un proyecto antes de tocarlo

Un proyecto en GitHub es un [repositorio](glossaire.md#repositorio). Antes de
cambiar nada, lee tres cosas.

### El README

El archivo `README.md` se muestra en la página principal del repositorio.
Dice para qué sirve el proyecto y cómo instalarlo y ejecutarlo.

### El CONTRIBUTING

El archivo `CONTRIBUTING.md` es el **reglamento interno** del proyecto.
Dice cómo proponer un cambio: nombre de la rama, mensaje del commit, tests
que ejecutar. Léelo **entero**: cada proyecto tiene sus costumbres.

### Las issues

Una **issue** (en español, «tema» o «incidencia») es una página de
discusión en GitHub: un bug del que alguien informa, una idea, una
pregunta. Están en la pestaña **Issues** del repositorio.

Los proyectos acogedores ponen una **etiqueta** (un *label*) en las
tareas fáciles:

```text
good first issue      ← una buena primera tarea
help wanted           ← el proyecto busca ayuda
documentation         ← la tarea trata de la documentación
```

Antes de empezar, escribe debajo de la issue «¡Hola! Me gustaría
encargarme, ¿sigue disponible?». Así, dos personas no hacen el mismo
trabajo.

> **Consejo:** en GitHub, busca `label:"good first issue"` en la barra de
> búsqueda de las issues para filtrar las tareas fáciles.

## El camino de una contribución

Este es todo el viaje, antes de hacerlo paso a paso.

```text
  EL PROYECTO (github.com/proyecto/herramienta)
      │                         ▲
      │ 1. fork                 │ 6. pull request
      ▼                         │
  TU COPIA (github.com/alice/herramienta)
      │                         ▲
      │ 2. clone                │ 5. push
      ▼                         │
  TU ORDENADOR: 3. rama → 4. commit
```

| Paso | Palabra | En claro |
|---|---|---|
| 1 | fork | Hacer **tu copia** del proyecto, en GitHub. |
| 2 | clone | Descargar tu copia en tu ordenador. |
| 3 | rama | Abrir un **borrador aparte** para tu cambio. |
| 4 | commit | Guardar tu cambio (ver [Git](git.md)). |
| 5 | push | Enviar el borrador a tu copia de GitHub. |
| 6 | pull request | **Pedir** al proyecto que acepte tu cambio. |

En los ejemplos, te llamas `alice` y el proyecto se llama
`proyecto/herramienta`.

## Paso 1: hacer un fork

No tienes permiso para escribir directamente en el repositorio del
proyecto. Así que haces una copia tuya: un **fork**.

1. Abre la página del proyecto en GitHub.
2. Haz clic en el botón **Fork**, arriba a la derecha.
3. Haz clic en **Create fork**.

Ahora tienes `github.com/alice/herramienta`. Esta copia te pertenece.

## Paso 2: clonar tu fork

Descarga **tu** copia (no la del proyecto) en tu ordenador, en la
[terminal](terminal.md):

```sh norun
git clone https://github.com/alice/herramienta.git
cd herramienta
```

```text
Cloning into 'herramienta'...
```

Para Git, esta copia en línea se llama ahora `origin`.

## Paso 3: crear una rama

Una **rama** es un borrador aparte. La rama `main` se queda limpia; tu
trabajo va en una rama nueva.

Dale un nombre que diga lo que haces:

```sh norun
git checkout -b corrige-errata-readme
```

```text
Switched to a new branch 'corrige-errata-readme'
```

> **Recuerda:** una contribución = una rama. No trabajes nunca
> directamente en `main`.

## Paso 4: modificar y hacer un commit

Abre el archivo en tu [editor](editeur.md), haz **un solo** cambio
pequeño y guarda. Después:

```sh norun
git status
git add README.md
git commit -m "Corrige una errata en el README"
```

```text
[corrige-errata-readme 3f2a1b7] Corrige una errata en el README
 1 file changed, 1 insertion(+), 1 deletion(-)
```

Si el proyecto tiene tests, ejecútalos ahora. El CONTRIBUTING dice cómo.

## Paso 5: subir la rama

Envía tu rama a **tu** fork, `origin`:

```sh norun
git push -u origin corrige-errata-readme
```

`-u` hace falta la primera vez: le dice a Git adónde enviar esta rama.
Después, basta con `git push`.

GitHub responde con un enlace ya preparado:

```text
remote:
remote: Create a pull request for 'corrige-errata-readme' on GitHub by visiting:
remote:      https://github.com/alice/herramienta/pull/new/corrige-errata-readme
remote:
To https://github.com/alice/herramienta.git
 * [new branch]      corrige-errata-readme -> corrige-errata-readme
branch 'corrige-errata-readme' set up to track 'origin/corrige-errata-readme'.
```

## Paso 6: abrir la pull request

Una **pull request** (**PR**) es una petición: «¿Quieres añadir mi cambio?»

1. Abre el enlace que muestra `git push`, o la página de tu fork.
2. Haz clic en **Compare & pull request**.
3. Escribe un título claro y una descripción corta.
4. Haz clic en **Create pull request**.

Una buena descripción responde a tres preguntas:

```text
Título: Corrige una errata en el README

Qué: «instalado» estaba escrito «istalado» en la sección Instalación.
Por qué: una errata dificulta la lectura.
Enlace: corrige la issue #42.
```

> **Consejo:** escribir `corrige la issue #42` (o en inglés `Fixes #42`)
> enlaza la PR con la issue. GitHub cerrará la issue cuando la PR sea
> aceptada.

## Responder a la revisión

Alguien del proyecto va a **revisar** tu PR: es la *review*. Puede
aceptarla (se dice **hacer merge**, «fusionar»), hacer una pregunta o
pedir cambios.

Una petición de cambios **no** es un rechazo. Es un paso normal, incluso
para los expertos.

Para corregir, quédate en la misma rama, modifica y después:

```sh norun
git add README.md
git commit -m "Aplica el comentario de la revisión"
git push
```

La PR se actualiza sola. Después responde al comentario:

```text
¡Gracias por la revisión! Está corregido en el último commit.
```

> **Recuerda:** un comentario trata del código, no de ti. Corrige, da las
> gracias, sube.

## Ser educado y paciente

Muchos proyectos los mantienen voluntarios, en su tiempo libre.

- Di **hola** y **gracias**.
- Ten **paciencia**: una respuesta puede tardar varios días, o semanas.
- Tras una semana sin noticias, basta con un mensaje corto y amable:
  «Hola, ¿has tenido tiempo de mirarlo? ¡Gracias!»
- Si no entiendes un comentario, **pregunta**. Es normal.
- Respeta el **código de conducta** (el archivo `CODE_OF_CONDUCT.md`).

## Ideas para una primera contribución

Ordenadas de la más fácil a la menos fácil:

| Idea | Dónde buscar |
|---|---|
| Corregir una errata | README, documentación |
| Hacer más clara una frase | Una página que te costó entender |
| Traducir una página al español | Una carpeta `docs/es` o `translations` |
| Informar de un bug con los pasos para reproducirlo | Pestaña Issues, botón **New issue** |
| Añadir un ejemplo a la documentación | Una función sin ejemplo |
| Resolver una issue `good first issue` | Pestaña Issues, filtro por etiqueta |

> **Pruébalo:** elige una herramienta que uses. Abre su repositorio, lee el
> README y busca una issue `good first issue`. No hagas nada más hoy: solo
> leer.

## Errores frecuentes

### Trabajar directamente en main

Haces dos cambios en `main`, sin rama. Acaban mezclados en la misma PR, y
quien revisa no puede aceptar uno sin el otro.

Corrección: antes de **cada** contribución, crea una rama desde `main`:
`git checkout -b anade-traduccion`.

### Un git push rechazado

El repositorio en línea tiene commits que no tienes en tu ordenador (por
ejemplo, modificaste un archivo directamente en GitHub).

```sh norun
git push origin main
```

```text
To https://github.com/alice/herramienta.git
 ! [rejected]        main -> main (fetch first)
error: failed to push some refs to 'https://github.com/alice/herramienta.git'
hint: Updates were rejected because the remote contains work that you do not
hint: have locally. This is usually caused by another repository pushing to
hint: the same ref. If you want to integrate the remote changes, use
hint: 'git pull' before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
```

Primero recupera los commits en línea con `git pull` y después vuelve a
hacer `git push`.

### Una PR enorme, sin haber leído el CONTRIBUTING

Cambias 40 archivos, el estilo del código y tres funciones de golpe.
Nadie se atreve a revisarla: la PR se queda abierta durante meses, o la
cierran.

Corrección:

- lee el `CONTRIBUTING.md` antes de empezar;
- **un** cambio por PR, lo más pequeño posible;
- para una idea grande, abre primero una issue para hablarlo.

## Ponte a prueba

**1.** ¿Cuál es la diferencia entre un fork y un clone?

> **Respuesta**
>
> Un fork es tu copia del proyecto **en GitHub**. Un clone es una copia
> **en tu ordenador**. Primero se hace el fork y después se clona ese
> fork.

**2.** Alguien que revisa tu PR te pide un cambio. ¿Tienes que abrir una
PR nueva?

> **Respuesta**
>
> No. Modifica en la misma rama, haz un commit y `git push`: la PR se
> actualiza sola. Después da las gracias y responde al comentario.

**3.** Estás empezando y no sabes qué hacer. Cita dos contribuciones
posibles.

> **Respuesta**
>
> Corregir una errata en la documentación, traducir una página, informar
> de un bug con los pasos para reproducirlo, o coger una issue marcada
> `good first issue`.

## En resumen

- Código abierto: el código es público, cualquiera puede leerlo, usarlo y proponer cambios.
- La licencia dice lo que está permitido: MIT deja hacerlo todo, GPL pide compartir a tu vez.
- Lee el README, el CONTRIBUTING y las issues `good first issue` antes de empezar.
- El camino: fork, clone, rama, commit, push, pull request.
- Un comentario de revisión es normal: corrige en la misma rama, sube y da las gracias.
- PR pequeñas, una rama por contribución y mucha educación.

Siguiente: [Los oficios del desarrollo](metiers.md)
