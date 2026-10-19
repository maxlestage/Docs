# Los símbolos y cuándo usarlos

Un símbolo como `:hecho` o `:derecha` es un pequeño nombre usado como valor: esta página muestra cómo escribirlo, compararlo, reconocerlo con `match`, y cuándo preferir un texto o un `type`.

> **Objetivos:**
>
> - sabrás escribir un símbolo y reconocer su tipo, `Symbol`;
> - sabrás comparar símbolos y reconocerlos con `match`;
> - sabrás guardar símbolos en registros y en diccionarios;
> - sabrás por qué un símbolo nunca es igual a un texto;
> - sabrás elegir entre un símbolo, un texto y un `type`.

## Un nombre usado como valor

Un [símbolo](../debuter/glossaire.md#simbolo) es un nombre precedido de dos
puntos: `:sí`, `:hecho`, `:derecha`. No es un nombre que designa un valor.
Es **el valor mismo**.

Piensa en la ficha de un guardarropa. La ficha no contiene nada: solo sirve
para ser reconocida. Un símbolo es igual: solo sirve para decir «es este».

```rythme
respuesta = :sí
print(respuesta, type_of(respuesta))
```

```sortie
:sí Symbol
```

Lee la primera línea así: «el nombre `respuesta` designa el símbolo `:sí`».

```text
 respuesta ──► [ :sí ]
 (un nombre)   (un valor: el símbolo sí)
```

`type_of` responde `Symbol`: es un tipo de valor aparte, como `Int` o
`Text`.

```rythme
print(type_of(:sí), type_of("sí"), type_of(1))
```

```sortie
Symbol Text Int
```

Prueba: cambia `:sí` por `:quizas`. Rythme lo acepta sin declarar nada: un
símbolo no necesita crearse antes de usarse.

> **Recuerda:** `:sí` es un valor ya listo. Los dos puntos van pegados al
> nombre.

## Escribir un símbolo

Un símbolo sigue las mismas reglas que un nombre: letras, cifras y `_`, y
se permiten los acentos.

```rythme
print(:hecho, :por_hacer, :año, :paso2)
```

```sortie
:hecho :por_hacer :año :paso2
```

Tres reglas que conocer:

- los dos puntos van **pegados** al nombre: `:hecho`, nunca `: hecho`;
- las mayúsculas y las minúsculas cuentan: `:Hecho` y `:hecho` son dos
  símbolos distintos;
- por costumbre, los símbolos se escriben en minúsculas, con `_` entre las
  palabras: `:en_curso`.

## Comparar símbolos

Dos símbolos son iguales cuando tienen el mismo nombre. Se comparan con
`==` y `!=`, como los números:

```rythme
respuesta = :sí
print(respuesta == :sí, respuesta == :no, respuesta != :no)
```

```sortie
true false true
```

| Comparación | Pregunta | Resultado |
|-------------|----------|-----------|
| `respuesta == :sí` | ¿es la ficha `sí`? | `true` |
| `respuesta == :no` | ¿es la ficha `no`? | `false` |
| `respuesta != :no` | ¿es otra cosa que `no`? | `true` |

Los símbolos también se ordenan por orden alfabético de sus nombres:

```rythme
print(sort([:pera, :albaricoque, :kiwi]), :albaricoque < :kiwi)
```

```sortie
[:albaricoque, :kiwi, :pera] true
```

## Un símbolo no es un texto

`:sí` y `"sí"` se parecen, pero son dos tipos de valores. Rythme nunca los
confunde: un símbolo **nunca** es igual a un texto.

```rythme
respuesta = :sí
print(respuesta == "sí")
```

```sortie
false
```

Rythme te avisa, con una advertencia:

```text
advertencia[W006]: esta comparación siempre es falsa: Symbol y Text nunca son iguales
 --> main.ry:2:17
  |
2 | print(respuesta == "sí")
  |                 ^^ Symbol == Text "sí"
  |
  = ayuda: los valores de tipos distintos nunca son iguales: Rythme nunca convierte los valores sin avisar
```

Para poner un símbolo en un texto, funcionan la inserción `{...}` y `text`.
El símbolo conserva sus dos puntos:

```rythme
print("respuesta: {:sí}", text(:sí), len(text(:sí)))
```

```sortie
respuesta: :sí :sí 3
```

`text(:sí)` es el texto `":sí"`: tres caracteres, dos puntos incluidos.

Cuando se escribe [JSON](../debuter/glossaire.md#json) con `to_json`, un
símbolo se convierte en un texto normal, sin los dos puntos:

```rythme
print(to_json({estado: :hecho, numero: 2}))
```

```sortie
{"estado": "hecho", "numero": 2}
```

> **Recuerda:** compara un símbolo con un símbolo: `estado == :hecho`,
> nunca `estado == "hecho"`.

## Reconocer un símbolo con match

Un símbolo está hecho para ser reconocido. `match` lo compara con cada
caso, de arriba abajo:

```rythme
fn icono(estado) = match estado
  :hecho -> "[x]"
  :en_curso -> "[~]"
  _ -> "[ ]"
end

print(icono(:hecho), icono(:en_curso), icono(:por_hacer))
```

```sortie
[x] [~] [ ]
```

Sigamos `icono(:en_curso)` paso a paso:

| Paso | Caso probado | ¿Coincide? | Resultado |
|------|--------------|------------|-----------|
| 1 | `:hecho` | no | |
| 2 | `:en_curso` | sí | `"[~]"` |

El caso `_` atrapa todos los demás símbolos, e incluso los demás valores.

También puedes escribir una función **por casos**, una línea por símbolo
(ver [definir por casos](fonctions.md#definir-por-casos)):

```rythme
fn traducir(:sí) = "yes"
fn traducir(:no) = "no"
fn traducir(_) = "?"

print(traducir(:sí), traducir(:no), traducir(:bah))
```

```sortie
yes no ?
```

Prueba: quita la línea `fn traducir(_) = "?"`. `traducir(:bah)` detiene
entonces el programa: ningún caso lo reconoce.

## Símbolos en los registros

Un símbolo es ideal para describir el **estado** de algo: una tarea hecha o
no, un pedido pendiente o entregado.

```rythme
tarea = {nombre: "compras", estado: :por_hacer}
tarea2 = {..tarea, estado: :hecho}
print(tarea.estado, tarea2.estado)
```

```sortie
:por_hacer :hecho
```

Fíjate en los dos `:` de `estado: :hecho`. El primero separa el nombre del
campo de su valor. El segundo empieza el símbolo.

```text
 { estado:   :hecho }
         ▲   ▲
         │   └── el comienzo del símbolo :hecho
         └────── «el campo estado vale...»
```

Con una lista de tareas, se puede filtrar según el estado:

```rythme
tareas = [
  {nombre: "compras", estado: :hecho},
  {nombre: "limpieza", estado: :por_hacer},
  {nombre: "deberes", estado: :en_curso},
  {nombre: "platos", estado: :hecho},
]
fn icono(:hecho) = "[x]"
fn icono(:en_curso) = "[~]"
fn icono(_) = "[ ]"
tareas |> each(fn(t) = print("{icono(t.estado)} {t.nombre}"))
hechas = tareas |> filter(fn(t) = t.estado == :hecho)
print("{len(hechas)} tareas hechas de {len(tareas)}")
```

```sortie
[x] compras
[ ] limpieza
[~] deberes
[x] platos
2 tareas hechas de 4
```

Prueba: pon la limpieza en `:hecho`. La última línea muestra entonces
`3 tareas hechas de 4`.

## Símbolos como claves de diccionario

Un símbolo también es una buena clave de
[diccionario](../debuter/glossaire.md#diccionario) (ver
[diccionarios y conjuntos](dictionnaires.md)):

```rythme
existencias = dict([[:manzana, 3], [:pera, 0], [:kiwi, 5]])
print(existencias[:kiwi])
print(get(existencias, :platano) |> unwrap_or(0))
print(keys(existencias))
```

```sortie
5
0
[:kiwi, :manzana, :pera]
```

El diccionario está ordenado según los nombres de los símbolos: `kiwi`,
`manzana`, `pera`.

> **Nota:** si vienes de Ruby, `{manzana: 3}` en Rythme no es un
> diccionario con la clave `:manzana`. Es un registro con un campo
> `manzana`. Para claves símbolo, escribe `dict([[:manzana, 3]])`.

## Un ejemplo: moverse por una cuadrícula

Los símbolos describen bien una **dirección**. Aquí tienes una ficha que
parte de la casilla `{x: 0, y: 0}`:

```rythme
fn mover(p, :arriba) = {..p, y: p.y - 1}
fn mover(p, :abajo) = {..p, y: p.y + 1}
fn mover(p, :izquierda) = {..p, x: p.x - 1}
fn mover(p, :derecha) = {..p, x: p.x + 1}

inicio = {x: 0, y: 0}
p1 = mover(inicio, :derecha)
p2 = mover(p1, :derecha)
p3 = mover(p2, :abajo)
print(inicio, p1, p2, p3)
```

```sortie
{x: 0, y: 0} {x: 1, y: 0} {x: 2, y: 0} {x: 2, y: 1}
```

| Paso | Dirección | x | y |
|------|-----------|---|---|
| inicio | | 0 | 0 |
| 1 | `:derecha` | 1 | 0 |
| 2 | `:derecha` | 2 | 0 |
| 3 | `:abajo` | 2 | 1 |

Como el camino es una lista de símbolos, `fold` puede dar todos los pasos
de una vez (ver [pensar en funcional](fonctionnel.md)):

```rythme
fn mover(p, :arriba) = {..p, y: p.y - 1}
fn mover(p, :abajo) = {..p, y: p.y + 1}
fn mover(p, :izquierda) = {..p, x: p.x - 1}
fn mover(p, :derecha) = {..p, x: p.x + 1}

camino = [:derecha, :derecha, :abajo]
print(fold(camino, {x: 0, y: 0}, mover))
```

```sortie
{x: 2, y: 1}
```

## Pasar de un texto a un símbolo

Lo que escribe el usuario siempre es un texto. Para convertirlo en un
símbolo, escribe una función por casos. Da una
[opción](../debuter/glossaire.md#option), porque el texto puede no
corresponder a nada:

```rythme
fn a_respuesta("sí") = Some(:sí)
fn a_respuesta("no") = Some(:no)
fn a_respuesta(_) = None

print(map(["sí", "no", "bah"], a_respuesta))
```

```sortie
[Some(:sí), Some(:no), None]
```

Una vez obtenido el símbolo, el resto del programa solo maneja `:sí` y
`:no`. Las erratas se detienen en la entrada.

## ¿Símbolo, texto o tipo?

Los tres pueden describir un estado. Así se elige:

| Tienes... | Elige | Ejemplo |
|-----------|-------|---------|
| palabras para mostrar o que vienen del usuario | un texto | `"Lea"`, `"hola"` |
| unas pocas etiquetas, para un programa pequeño | un símbolo | `:hecho`, `:derecha` |
| una lista cerrada de casos, que nunca hay que olvidar | un `type` | `type Estado = PorHacer \| Hecho` |

El símbolo es rápido de escribir. Pero Rythme no conoce la lista de tus
símbolos: una errata pasa sin ruido.

```rythme
fn icono(:hecho) = "[x]"
fn icono(:en_curso) = "[~]"
fn icono(_) = "[ ]"

tarea = {nombre: "deberes", estado: :en_curs}
print(icono(tarea.estado))
```

```sortie
[ ]
```

Falta una `o` en `:en_curs`. El programa no dice nada, y muestra un
resultado equivocado.

Con un [tipo](types.md), Rythme conoce todos los casos. Una errata es un
error, y un caso olvidado en un `match` da una advertencia:

```rythme
type Estado = PorHacer | EnCurso | Hecho

fn icono(e) = match e
  PorHacer -> "[ ]"
  EnCurso -> "[~]"
  Hecho -> "[x]"
end

print(icono(EnCurso), icono(Hecho))
```

```sortie
[~] [x]
```

Escribir `EnCurs` aquí sería el error E003, con «¿quisiste decir
`EnCurso`?».

> **Recuerda:** un símbolo para ir rápido, un `type` cuando los casos se
> conocen de antemano y no hay que olvidar ninguno.

## Errores frecuentes

### Poner un espacio después de los dos puntos

```rythme norun
estado = : hecho
```

```text
error[E001]: se esperaba una expresión, pero se encontró `:`
 --> main.ry:1:10
  |
1 | estado = : hecho
  |          ^ se esperaba una expresión aquí
  |
  = ayuda: Rythme no usa `:` para empezar un bloque; consulta `rythme help syntax`
  = más sobre este error: `rythme explain E001`
```

Pega los dos puntos al nombre: `estado = :hecho`.

### Olvidar el espacio en un registro

```rythme norun
tarea = {estado:terminado}
```

```text
error[E003]: nombre desconocido `terminado`
 --> main.ry:1:17
  |
1 | tarea = {estado:terminado}
  |                 ^^^^^^^^^ no definido
  |
  = ayuda: defínelo antes de usarlo: `terminado = ...` o `fn terminado(...) = ...`
  = más sobre este error: `rythme explain E003`
```

Justo después de un nombre de campo, `:` separa el campo de su valor. Aquí,
el valor es el **nombre** `terminado`, que no existe. Escribe
`{estado: :terminado}`, con dos `:` y un espacio entre ellos.

### Un match sin caso para todo lo demás

```rythme norun
fn icono(estado) = match estado
  :hecho -> "[x]"
  :en_curso -> "[~]"
end

print(icono(:hecho))
print(icono(:por_hacer))
```

```text
[x]
error[E006]: ningún caso de este `match` trata Symbol :por_hacer
 --> main.ry:1:26
  |
1 | fn icono(estado) = match estado
  |                          ^^^^^^ este valor
  |
  = ayuda: añade un caso para este valor, o un último caso `_ -> ...` que trate todo lo demás
  = llamadas en curso (la más reciente primero):
      en `icono` llamada en main.ry:7:7
  = más sobre este error: `rythme explain E006`
```

La primera llamada funciona, la segunda detiene el programa. Rythme no
puede conocer todos los símbolos posibles: añade un último caso
`_ -> "[ ]"`.

### Calcular con un símbolo

```rythme norun
print(:manzana + :pera)
```

```text
error[E005]: no se puede usar `+` con Symbol y Symbol
 --> main.ry:1:16
  |
1 | print(:manzana + :pera)
  |                ^ Symbol :manzana + Symbol :pera
  |
  = más sobre este error: `rythme explain E005`
```

Un símbolo no es ni un número ni un texto: se compara, se reconoce, no se
calcula con él. Para un texto, pasa por `text(:manzana)`.

## Ponte a prueba

**Pregunta 1:** ¿cuánto valen `:sí == :sí`, `:sí == :Sí` y `:sí == "sí"`?

> **Respuesta**
>
> `true`, `false` y `false`. Las mayúsculas cuentan, y un símbolo nunca es
> igual a un texto.

**Pregunta 2:** ¿qué muestra este programa?

```rythme norun
fn senal(:verde) = "pase"
fn senal(:rojo) = "alto"
fn senal(_) = "precaución"
print(senal(:rojo), senal(:naranja))
```

> **Respuesta**
>
> `alto precaución`. `:rojo` tiene su propia línea; `:naranja` no, así que
> cae en `_`.

**Pregunta 3:** el estado de un pedido puede ser «pendiente», «enviado» o
«entregado», y no hay que olvidar ningún caso. ¿Símbolo o `type`?

> **Respuesta**
>
> Un `type`: `type Pedido = Pendiente | Enviado | Entregado`. Rythme
> conoce entonces todos los casos: rechaza una errata y señala un caso
> olvidado en un `match` (W002).

## En resumen

- Un símbolo se escribe `:` pegado a un nombre: `:hecho`. Su tipo es
  `Symbol`.
- Dos símbolos son iguales cuando tienen el mismo nombre; un símbolo nunca
  es igual a un texto.
- `match` y las funciones por casos reconocen los símbolos; guarda un
  último caso `_`.
- En un registro, escribe `{estado: :hecho}`, con un espacio entre los dos
  `:`.
- Un símbolo para ir rápido; un `type` cuando los casos se conocen y no hay
  que olvidarlos.

Siguiente: [el pattern matching](pattern-matching.md)
