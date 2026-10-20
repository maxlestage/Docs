# La coincidencia de patrones

Un patrón describe la forma de un valor: esta página repasa todos los tipos de patrones de Rythme, tanto en `match` como en los parámetros de las funciones.

> **Objetivos:**
>
> - sabrás reconocer un número, un intervalo, un texto o el comienzo de un texto;
> - sabrás descomponer una lista o un registro con un patrón;
> - sabrás combinar patrones con `|`, guardas y patrones anidados;
> - sabrás leer las advertencias W002 y W004 del verificador;
> - sabrás usar patrones en los parámetros de una función.

## Recordatorio: match

[`match`](../debuter/glossaire.md#match) compara un valor con una serie de
**patrones**, en orden. El primero que encaja da el resultado. Es como un
clasificador de monedas: cada moneda cae en el primer agujero de su
tamaño.

```rythme
fn dia(n) = match n
  6 -> "sábado"
  7 -> "domingo"
  _ -> "un día laborable"
end

print(dia(6), dia(2))
```

```sortie
sábado un día laborable
```

Cada caso se escribe `patrón -> resultado`, y `end` cierra el `match`. Un
patrón no calcula nada: describe una **forma** y da nombres a los trozos.

## Los valores exactos

El patrón más sencillo es un valor escrito tal cual: un número, un texto,
`true` o `false`, `()`, un símbolo. `_` encaja con todo, sin dar nombre.

```rythme
fn respuesta(x) = match x
  0 -> "cero"
  2.5 -> "dos y medio"
  "sí" -> "de acuerdo"
  true -> "verdadero"
  () -> "nada"
  :ok -> "el símbolo ok"
  _ -> "otra cosa"
end

print([0, 0.0, 2.5, "sí", true, (), :ok, 99] |> map(respuesta))
```

```sortie
["cero", "cero", "dos y medio", "de acuerdo", "verdadero", "nada", "el símbolo ok", "otra cosa"]
```

El patrón `0` también acepta `0.0`: los números iguales valen lo mismo.

Un **nombre** también encaja con todo, pero guarda el valor:

```rythme
fn describir(x) = match x
  0 -> "nada"
  n -> "el número {n}"
end

print(describir(0), "/", describir(42))
```

```sortie
nada / el número 42
```

## Los intervalos

`a..b` encaja con un número de `a` a `b`, **ambos extremos incluidos**. Los
límites son números escritos en el código.

```rythme
fn calificacion(nota) = match nota
  16..20 -> "sobresaliente"
  12..15 -> "notable"
  10..11 -> "aprobado"
  0..9 -> "suspenso"
  _ -> "no es una nota"
end

print([17, 12, 10, 3, 25] |> map(calificacion))
```

```sortie
["sobresaliente", "notable", "aprobado", "suspenso", "no es una nota"]
```

Un intervalo también acepta Floats: `15.5` no está en ninguno de estos
intervalos, así que iría a `_`. El último caso `_` es imprescindible,
porque ningún intervalo cubre todos los números.

## El comienzo de un texto

`"Sr. " + nombre` encaja con un texto que **empieza** por `"Sr. "`, y
llama `nombre` al resto. Es muy práctico para leer comandos:

```rythme
fn comando("/ayuda") = "aquí tienes la ayuda"
fn comando("/decir " + mensaje) = "dices: {mensaje}"
fn comando("/" + _) = "comando desconocido"
fn comando(texto) = "mensaje: {texto}"

print(comando("/ayuda"))
print(comando("/decir hola"))
print(comando("/baile"))
print(comando("buenas"))
```

```sortie
aquí tienes la ayuda
dices: hola
comando desconocido
mensaje: buenas
```

Solo el **comienzo** de un texto puede servir de patrón. Para el final,
usa una guarda (ver más abajo) con `ends_with`.

## Las listas

Un patrón de lista describe su longitud y sus elementos:

| Patrón | Encaja con |
|--------|------------|
| `[]` | la lista vacía |
| `[x]` | una lista de exactamente un elemento |
| `[x, y]` | una lista de exactamente dos elementos |
| `[primero, ..resto]` | una lista de al menos un elemento; `resto` es lo que sigue |
| `[a, b, ..]` | al menos dos elementos; lo que sigue no tiene nombre |

```rythme
fn describir(lista) = match lista
  [] -> "vacía"
  [x] -> "uno solo: {x}"
  [x, y] -> "dos: {x} y {y}"
  [primero, ..resto] -> "{primero} y luego {len(resto)} más"
end

print(describir([]), "/", describir([7]), "/", describir([7, 8]), "/", describir([1, 2, 3, 4]))
```

```sortie
vacía / uno solo: 7 / dos: 7 y 8 / 1 y luego 3 más
```

`..resto` debe ir al final. Este patrón es el que hace funcionar
[la recursión](recursion.md) sobre las listas.

## Los registros

`{nombre}` encaja con un registro que tiene **al menos** un campo
`nombre`, y da ese campo con el nombre `nombre`. `{edad: 0}` exige que el
campo `edad` valga 0. El registro puede tener otros campos.

```rythme
fn ficha(p) = match p
  {nombre, edad: 0} -> "{nombre} acaba de nacer"
  {nombre, ciudad: "París"} -> "{nombre} vive en París"
  {nombre, edad} -> "{nombre} tiene {edad} años"
  {} -> "no es una persona"
end

print(ficha({nombre: "Léo", edad: 0}))
print(ficha({nombre: "Ada", edad: 36, ciudad: "París"}))
print(ficha({nombre: "Alan", edad: 41}))
print(ficha({titulo: "Libro"}))
```

```sortie
Léo acaba de nacer
Ada vive en París
Alan tiene 41 años
no es una persona
```

`{}` encaja con cualquier registro.

## Los constructores

Las variantes de un tipo (ver [tus propios tipos](types.md)), y `Some`,
`None`, `Ok`, `Err`, se reconocen por su nombre. Sus campos son, a su vez,
patrones:

```rythme
type Forma = Circulo(radio) | Rect(ancho, alto) | Punto

fn area(f) = match f
  Circulo(r) -> round(pi * r ** 2, 2)
  Rect(an, al) -> an * al
  Punto -> 0
end

print(map([Circulo(1), Rect(2, 3), Punto], area))
```

```sortie
[3.14, 6, 0]
```

## Varias posibilidades con |

`p1 | p2` encaja si uno de los dos patrones encaja:

```rythme
fn tipo_de_dia(dia) = match dia
  "sábado" | "domingo" -> "fin de semana"
  _ -> "entre semana"
end

print(tipo_de_dia("sábado"), "/", tipo_de_dia("lunes"))
```

```sortie
fin de semana / entre semana
```

Los patrones unidos por `|` no pueden dar nombres (ver los errores
frecuentes).

## Las guardas

Una **guarda** es una condición añadida después del patrón con `if`. El
caso solo se elige si el patrón encaja **y** la condición vale `true`. Los
nombres del patrón se pueden usar en la guarda.

```rythme
type Forma = Circulo(radio) | Rect(ancho, alto) | Punto

fn naturaleza(f) = match f
  Rect(an, al) if an == al -> "un cuadrado de lado {an}"
  Rect(an, al) -> "un rectángulo de {an} × {al}"
  Circulo(_) | Punto -> "círculo o punto"
end

print(naturaleza(Rect(2, 2)), "/", naturaleza(Rect(2, 5)), "/", naturaleza(Punto))
```

```sortie
un cuadrado de lado 2 / un rectángulo de 2 × 5 / círculo o punto
```

Una guarda sirve también para el final de un texto, o para comparar con
un valor que no está escrito en el código:

```rythme
fn tono(frase) = match frase
  t if ends_with(t, "?") -> "pregunta"
  t if ends_with(t, "!") -> "exclamación"
  _ -> "afirmación"
end

print(tono("¿De verdad?"), tono("¡Bravo!"), tono("Llueve."))
```

```sortie
pregunta exclamación afirmación
```

## Patrones anidados

Los patrones se encajan como cajas dentro de cajas: un registro que
contiene un registro y una lista, una lista de `Result`...

```rythme
fn resumen(pedido) = match pedido
  {cliente: {nombre}, articulos: [primero, ..]} -> "{nombre} pide primero {primero}"
  {cliente: {nombre}, articulos: []} -> "{nombre} no ha pedido nada"
  _ -> "pedido ilegible"
end

print(resumen({cliente: {nombre: "Ada", ciudad: "Londres"}, articulos: ["té", "scones"]}))
print(resumen({cliente: {nombre: "Alan"}, articulos: []}))

fn primero([Ok(v), ..]) = "primer resultado: {v}"
fn primero([Err(e), ..]) = "error: {e}"
fn primero([]) = "nada"
print(primero([Ok(3), Err("x")]), "/", primero([Err("ups")]))
```

```sortie
Ada pide primero té
Alan no ha pedido nada
primer resultado: 3 / error: ups
```

## Patrones en los parámetros

Los parámetros de una función también son patrones. Una función definida
por casos es, por tanto, un `match` escondido:

```rythme
fn distancia({x: x1, y: y1}, {x: x2, y: y2}) = sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)
print(distancia({x: 0, y: 0}, {x: 3, y: 4}))

fn cabeza([x, ..]) = Some(x)
fn cabeza([]) = None
print(cabeza([5, 6]), cabeza([]))
```

```sortie
5.0
Some(5) None
```

Por último, los patrones sirven para nombrar varios trozos de una vez:

```rythme
[a, b, ..] = [10, 20, 30]
{nombre} = {nombre: "Ada", edad: 36}
print(a, b, nombre)
```

```sortie
10 20 Ada
```

## El verificador te ayuda

Antes de ejecutar el programa, Rythme relee tus `match`. Señala dos
olvidos frecuentes.

**W002: un caso olvidado.** Aquí, el semáforo en naranja no se trata:

```rythme norun
type Semaforo = Rojo | Naranja | Verde
fn accion(luz) = match luz
  Rojo -> "alto"
  Verde -> "adelante"
end
print(accion(Verde))
```

```text
advertencia[W002]: este `match` no trata todos los casos
 --> main.ry:2:18
  |
2 | fn accion(luz) = match luz
  |                  ^^^^^^^^^
  |
  = note: sin tratar: `Naranja`
  = ayuda: añade los casos que faltan, como `Naranja -> ...`, o un último caso `_ -> ...`

adelante
```

El programa se ejecuta igualmente, pero `accion(Naranja)` lo detendría con
el error E006. Añade el caso `Naranja -> "frena"`.

**W004: un caso nunca alcanzado.** Los casos se prueban en orden: un `_`
puesto el primero se lo lleva todo.

```rythme norun
fn tamano(n) = match n
  _ -> "desconocido"
  0 -> "nulo"
end
print(tamano(0))
```

```text
advertencia[W004]: este caso nunca se puede alcanzar
 --> main.ry:3:3
  |
3 |   0 -> "nulo"
  |   ^
  |
2 |   _ -> "desconocido"
  |   - este caso ya lo trata todo
  |
  = ayuda: los casos se prueban en orden: pon `_` al final

desconocido
```

Pon los casos más precisos arriba, y `_` al final del todo.

> **Recuerda:** una advertencia no detiene el programa, pero casi siempre
> señala un problema real. Léelas.

## Errores frecuentes

### Usar dos veces el mismo nombre

```rythme norun
fn iguales([a, a]) = true
fn iguales(_) = false
print(iguales([1, 1]))
```

```text
error[E011]: `a` aparece dos veces en este patrón
 --> main.ry:1:16
  |
1 | fn iguales([a, a]) = true
  |                ^
  |
1 | fn iguales([a, a]) = true
  |             - la primera vez aquí
  |
  = ayuda: usa dos nombres distintos, y luego compáralos con `==` en una condición: `[a, b] if a == b -> ...`
  = más sobre este error: `rythme explain E011`
```

Un patrón no compara dos trozos entre sí. Escribe
`fn iguales([a, b]) if a == b = true`.

### Reconocer el final de un texto

```rythme norun
fn tono(t) = match t
  _ + "!" -> "exclamación"
  _ -> "otro"
end
print(tono("¡oh!"))
```

```text
error[E001]: solo un texto puede ir antes de `+` en un patrón
 --> main.ry:2:3
  |
2 |   _ + "!" -> "exclamación"
  |   ^^^
  |
  = ayuda: un texto que empieza por un texto se escribe `"Sr. " + nombre`: `nombre` es el resto del texto
  = ayuda: para mirar el final de un texto, usa una condición: `s if ends_with(s, "!") -> ...`
  = más sobre este error: `rythme explain E001`
```

Solo se reconoce el comienzo de un texto. Para el final: `t if ends_with(t, "!") -> ...`.

### Dar un nombre en un patrón con |

```rythme norun
fn inicio(lista) = match lista
  [x] | [x, _] -> x
  _ -> 0
end
print(inicio([4, 5]))
```

```text
error[E001]: los patrones unidos con `|` no pueden dar nombres (aquí `x`)
 --> main.ry:2:4
  |
2 |   [x] | [x, _] -> x
  |    ^
  |
  = ayuda: escribe casos separados en su lugar
  = más sobre este error: `rythme explain E001`
```

Escribe dos casos: `[x] -> x` y luego `[x, _] -> x`.

## Ponte a prueba

**Pregunta 1:** ¿qué valores aceptan los patrones `[x]`, `[x, ..]` y
`[x, ..resto]`?

> **Respuesta**
>
> `[x]`: una lista de exactamente un elemento. `[x, ..]` y `[x, ..resto]`:
> una lista de al menos un elemento; el segundo también nombra lo que sigue.

**Pregunta 2:** ¿qué da `clasificar(15)`?

```rythme norun
fn clasificar(n) = match n
  0..9 -> "pequeño"
  10..99 if n % 5 == 0 -> "redondo"
  10..99 -> "mediano"
  _ -> "grande"
end
```

> **Respuesta**
>
> `"redondo"`: 15 está en `10..99`, y la guarda `15 % 5 == 0` vale `true`.

**Pregunta 3:** ¿qué señala la advertencia W004?

> **Respuesta**
>
> Un caso que nunca puede elegirse, porque un caso puesto antes ya acepta
> los mismos valores (a menudo un `_` puesto demasiado pronto).

## En resumen

- Un patrón describe una forma: valor exacto, intervalo `a..b`, comienzo
  de texto `"Sr. " + nombre`, lista, registro, constructor.
- `[x, ..resto]` descompone una lista; `{nombre}` toma un campo; `_`
  acepta todo sin nombre.
- `|` ofrece varias posibilidades (sin nombres); `if` añade una guarda.
- Los patrones se anidan, y sirven también en los parámetros de las
  funciones y en `[a, b] = lista`.
- W002 señala un caso olvidado, W004 un caso nunca alcanzado.

Siguiente: [gestionar los errores](erreurs.md)
