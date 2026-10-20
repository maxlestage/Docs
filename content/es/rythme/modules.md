# Módulos: un programa en varios archivos

Cuando un programa crece, se reparte en varios archivos: cada archivo es un **módulo**, que los demás usan con `use`.

> **Objetivos:**
>
> - usar las funciones de otro archivo con `use nombre`;
> - guardar archivos en carpetas, con `use carpeta/nombre`;
> - dar un nombre más corto a un módulo con `use x as y`;
> - mantener una función privada en su archivo, con un nombre que empieza por `_`.

## Por qué varios archivos

Un programa de 30 líneas cabe muy bien en un solo archivo. Con 300 líneas,
te pierdes: buscas una función desplazando la pantalla, y mezclas cosas que
no tienen nada que ver.

Entonces ordenas el código por **tema**: los cálculos de geometría en un
archivo, lo que se muestra en otro, el programa principal en un tercero.
Cada archivo es corto, y su nombre dice lo que contiene.

En Rythme, cada archivo `.ry` puede servir de **módulo**: otro archivo lo
usa con la palabra clave `use`.

> **Nota:** los ejemplos de esta página necesitan varios archivos, así que
> no se ejecutan en el sitio. Sus salidas se copiaron de ejecuciones reales:
> crea los archivos en tu ordenador para probarlos.

## Un primer módulo con use

Crea una carpeta con dos archivos:

```text
mi_carpeta/
├── geometria.ry
└── main.ry
```

El archivo `geometria.ry` contiene dos funciones. Nada nuevo aquí: es un
archivo Rythme normal.

```rythme norun
## El área de un rectángulo.
fn area_rectangulo(ancho, alto) = ancho * alto

## El área de un cuadrado.
fn area_cuadrado(lado) = area_rectangulo(lado, lado)
```

El archivo `main.ry` las usa:

```rythme norun
use geometria

print(geometria.area_rectangulo(2, 3))
print(geometria.area_cuadrado(4))
```

En la terminal, desde `mi_carpeta`:

```sh
rythme run main.ry
```

```text
6
16
```

Dos cosas que recordar:

- `use geometria` carga el archivo `geometria.ry`. Escribes el nombre **sin
  comillas y sin `.ry`**. El archivo se busca en la carpeta del archivo que
  contiene el `use`.
- Después, escribes `geometria.` delante de cada nombre del módulo:
  `geometria.area_cuadrado(4)`. Al leer el código, sabes enseguida de dónde
  viene cada función.

> **Recuerda:** `use` se escribe al principio del archivo, fuera de
> cualquier función.

## Cada módulo tiene sus propios nombres

Como cada nombre va precedido de su módulo, dos archivos pueden tener una
función con el mismo nombre sin molestarse. `geometria.area` y
`circulo.area` serían dos funciones distintas, y tu programa puede definir
también su propia función `area`.

Por eso mismo `geometria` solo no es un valor: es solamente el nombre del
archivo, al que siempre le sigue un punto y un nombre.

## Un módulo en una carpeta con use carpeta/nombre

Cuando los módulos son muchos, se guardan en carpetas. Añadamos una carpeta
`formas` con un archivo `circulo.ry`:

```text
mi_carpeta/
├── formas/
│   └── circulo.ry
├── geometria.ry
└── main.ry
```

Este es `formas/circulo.ry`:

```rythme norun
## El área de un círculo, redondeada a 2 decimales.
fn area(radio) = round(pi * radio ** 2, 2)

## El perímetro de un círculo, redondeado a 2 decimales.
fn perimetro(radio) = round(2 * pi * radio, 2)
```

En `main.ry`, escribes la ruta con una barra `/`, siempre sin `.ry`:

```rythme norun
use geometria
use formas/circulo

print(geometria.area_cuadrado(3))
print(circulo.area(1))
print(circulo.perimetro(1))
```

```text
9
3.14
6.28
```

El módulo se llama como la **última parte** de la ruta: `circulo`, y no
`formas/circulo`.

> **Consejo:** para usar un archivo de una carpeta vecina, `..` sube una
> carpeta: `use ../herramientas/calculos`.

## Un nombre más corto con use x as y

`as` le da al módulo el nombre que tú elijas:

```rythme norun
use formas/circulo as disco

fn area(lado) = lado * lado

print(disco.area(2))
print(area(2))
```

```text
12.57
4
```

Aquí, `disco.area` es el área del círculo, y `area` la función del propio
archivo. `as` sirve sobre todo en dos casos:

- el nombre del módulo es largo, y prefieres un nombre corto;
- dos archivos tienen el mismo nombre, como `formas/circulo.ry` y
  `dibujos/circulo.ry`: dos módulos no pueden llamarse los dos `circulo`,
  así que renombras uno de ellos (`use dibujos/circulo as dibujo`).

## Valores y tipos en un módulo

Un módulo no da solo funciones: también da sus **valores** y sus **tipos**
(ver [tus propios tipos](types.md)). Este es `semaforo.ry`:

```rythme norun
## Los tres colores de un semáforo.
type Color = Rojo | Ambar | Verde

## El color que viene después.
fn siguiente(Rojo) = Verde
fn siguiente(Verde) = Ambar
fn siguiente(Ambar) = Rojo
```

En `main.ry`, los colores también se escriben con el nombre del módulo,
incluso en un `match`:

```rythme norun
use semaforo

fn consejo(color) = match color
  semaforo.Rojo -> "hay que parar"
  semaforo.Ambar -> "hay que frenar"
  semaforo.Verde -> "se puede pasar"
end

print(semaforo.siguiente(semaforo.Rojo))
print(consejo(semaforo.Verde))
```

```text
Verde
se puede pasar
```

## Mantener un nombre privado con _

Algunas funciones solo sirven dentro de su archivo: son **herramientas
internas**. Para ocultarlas a los demás archivos, dales un nombre que
empiece por `_`. Este es `puntos.ry`:

```rythme norun
## El punto de partida, en el centro.
origen = {x: 0, y: 0}

## La distancia entre dos puntos {x, y}.
fn distancia(a, b) = sqrt(_cuadrado(a.x - b.x) + _cuadrado(a.y - b.y))

fn _cuadrado(x) = x * x
```

`distancia` usa `_cuadrado` sin problema: están en el mismo archivo. Pero
`main.ry` solo ve `origen` y `distancia`:

```rythme norun
use puntos

print(puntos.origen)
print(puntos.distancia(puntos.origen, {x: 3, y: 4}))
```

```text
{x: 0, y: 0}
5.0
```

Un nombre privado te deja libre para cambiarlo más adelante, sin romper los
demás archivos. Escribir `puntos._cuadrado(3)` en `main.ry` es un error (ver
más abajo).

## Lo que hace use, paso a paso

Cuando ejecutas `rythme run main.ry`:

1. Rythme lee `main.ry`, encuentra sus `use`, y carga cada módulo, y luego
   los módulos de esos módulos.
2. **Comprueba** todos los archivos antes de ejecutar nada: un error de
   tecleo en un módulo se encuentra enseguida.
3. Cada módulo se ejecuta **una sola vez**, antes del archivo que lo usa,
   aunque varios archivos lo usen.

`rythme check main.ry` hace la misma comprobación sin ejecutar nada.

> **Consejo:** un módulo sirve para **definir** cosas. Evita poner `print`
> en su nivel superior: se mostrarían en cuanto otro archivo lo use.

Dos módulos no pueden usarse el uno al otro (`a.ry` usa `b.ry`, que usa
`a.ry`): pon lo que necesitan los dos en un tercer archivo.

## Errores frecuentes

### Olvidar el nombre del módulo

Con `use geometria`, escribes `area_cuadrado(4)` en lugar de
`geometria.area_cuadrado(4)`:

```rythme norun
use geometria

print(area_cuadrado(4))
```

```text
error[E003]: nombre desconocido `area_cuadrado`
 --> main.ry:3:7
  |
3 | print(area_cuadrado(4))
  |       ^^^^^^^^^^^^^ no definido aquí
  |
  = ayuda: `area_cuadrado` está definido en `geometria.ry`: escribe `geometria.area_cuadrado`
  = más sobre este error: `rythme explain E003`
```

Corrección: `print(geometria.area_cuadrado(4))`. Si olvidaste el propio
`use`, el mensaje también te propone añadirlo.

### Equivocarse en el nombre del archivo

```rythme norun
use geometira

print(geometira.area_cuadrado(4))
```

```text
error[E017]: archivo `geometira.ry` no encontrado
 --> main.ry:1:5
  |
1 | use geometira
  |     ^^^^^^^^^
  |
  = note: se buscó: geometira.ry
  = ayuda: ¿quisiste decir `use geometria`?
  = ayuda: la ruta es relativa a la carpeta del archivo que contiene `use`
  = más sobre este error: `rythme explain E017`
```

Corrección: `use geometria`. Comprueba también que el archivo esté de
verdad en la misma carpeta que `main.ry` (o en la carpeta que indica la
ruta).

### Escribir .ry en el use

```rythme norun
use geometria.ry

print(geometria.area_cuadrado(4))
```

```text
error[E001]: escribe el nombre del archivo sin `.ry`
 --> main.ry:1:14
  |
1 | use geometria.ry
  |              ^
  |
  = ayuda: ejemplo: `use geometria`
  = más sobre este error: `rythme explain E001`
```

Corrección: `use geometria`, sin `.ry`.

### Usar un nombre privado

```rythme norun
use puntos

print(puntos._cuadrado(3))
```

```text
error[E003]: `_cuadrado` es privado del archivo `puntos.ry`
 --> main.ry:3:7
  |
3 | print(puntos._cuadrado(3))
  |       ^^^^^^^^^^^^^^^^ privado
  |
  = note: un nombre que empieza por `_` solo se puede usar en su propio archivo
  = ayuda: para usarlo desde otros archivos, quítale el `_` en `puntos.ry`
  = más sobre este error: `rythme explain E003`
```

Corrección: si otros archivos la necesitan de verdad, renombra la función
`cuadrado` (sin `_`) en `puntos.ry`.

## Ponte a prueba

1. El archivo `herramientas/texto.ry` contiene una función `titulo`. ¿Qué
   hay que escribir en `main.ry` para llamarla?

> **Respuesta**
>
> `use herramientas/texto` al principio del archivo, y luego
> `texto.titulo(...)`: el módulo toma el nombre de la última parte de la
> ruta.

2. ¿Por qué escribir `use calculos as c`?

> **Respuesta**
>
> Para usar el módulo con un nombre más corto: `c.promedio(...)` en lugar
> de `calculos.promedio(...)`. También es la solución cuando dos archivos
> tienen el mismo nombre.

3. En `banco.ry`, ¿se puede usar la función `_redondear` desde `main.ry`?

> **Respuesta**
>
> No: su nombre empieza por `_`, así que es privada de `banco.ry`. Las demás
> funciones de `banco.ry` pueden usarla, pero `banco._redondear(...)` en
> `main.ry` es el error E003.

## En resumen

- `use geometria` carga `geometria.ry` (sin comillas, sin `.ry`); sus
  nombres se escriben después `geometria.nombre`.
- `use formas/circulo` carga `formas/circulo.ry`, que se usa con el nombre
  `circulo`.
- `use formas/circulo as disco` elige otro nombre.
- Un módulo da sus funciones, sus valores y sus tipos; un nombre que empieza
  por `_` queda privado en su archivo.
- Rythme comprueba todos los archivos antes de ejecutar el programa.

Siguiente: [leer y escribir archivos](fichiers.md). Compara con los módulos
de [Ruby](../ruby/modules.md) y de [Rust](../rust/modules.md).
