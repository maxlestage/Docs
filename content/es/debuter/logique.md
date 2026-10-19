# Verdadero, falso y las condiciones

Un programa toma decisiones haciéndose preguntas que se responden con verdadero o falso: esta página muestra cómo comparar, combinar preguntas con y, o, no, y luego elegir un camino con `if` y `else`.

> **Objetivos:**
>
> - saber qué es un booleano: verdadero o falso;
> - comparar dos valores con `==`, `!=`, `<`, `>=`…;
> - combinar preguntas con y, o, no;
> - elegir entre dos caminos con `if` y `else`;
> - elegir entre varios caminos con `elif`;
> - evitar los errores clásicos: `=` en lugar de `==`, el caso «igual» olvidado.

## Verdadero o falso: los booleanos

Algunas preguntas solo se responden con **sí** o con **no**:
«¿Llueve?», «¿Tengo más de 18 años?».

En programación, sí se dice **verdadero** y no se dice **falso**. Estos dos
valores se llaman **booleanos** (ver [booleano](glossaire.md#booleano)).

```python
llueve = True
tengo_hambre = False
print(llueve)
print(tengo_hambre)
```

```sortie
True
False
```

En Python, se escribe `True` y `False`, con mayúscula. En JavaScript y en
Ruby, se escribe `true` y `false`, en minúsculas.

Un booleano se guarda en una variable, como un número. Ver
[las variables](variables.md).

> **Recuerda:** un booleano solo tiene dos valores posibles: verdadero o falso.

## Comparar dos valores

Casi siempre, no escribes `True` a mano. Lo obtienes **comparando** dos
valores. La respuesta es un booleano.

```python
edad = 20
print(edad > 18)
print(edad < 18)
print(edad == 20)
```

```sortie
True
False
True
```

Estas son las seis comparaciones. Se escriben igual en Python, JavaScript y
Ruby:

| Se escribe | Se lee | Ejemplo verdadero |
|---|---|---|
| `==` | es igual a | `3 == 3` |
| `!=` | es distinto de | `3 != 4` |
| `<` | es menor que | `2 < 5` |
| `>` | es mayor que | `5 > 2` |
| `<=` | es menor **o igual** | `5 <= 5` |
| `>=` | es mayor **o igual** | `5 >= 5` |

También se comparan textos: `"sol" == "Sol"` vale `False`. Para el
ordenador, `s` y `S` son dos letras distintas.

> **Atención:** `=` guarda un valor en una variable. `==` hace una
> pregunta: «¿es igual?». Son dos cosas muy distintas.

Prueba: cambia `edad = 20` por `edad = 18`. La primera línea muestra
`False`: 18 no es mayor que 18.

## Combinar preguntas: y, o, no

A veces, una sola pregunta no basta. «Salgo **si** hace buen tiempo **y**
he terminado mi trabajo.»

Hay tres palabras para combinar booleanos. Python las escribe en inglés;
JavaScript y Ruby usan símbolos:

| En español | Python | JavaScript, Ruby |
|---|---|---|
| y | `and` | `&&` |
| o | `or` | `\|\|` |
| no | `not` | `!` |

### «y»: los dos deben ser verdaderos

«Hace buen tiempo **y** he terminado» solo es verdadero si **los dos** son
verdaderos.

| ¿Hace buen tiempo? | ¿He terminado? | ¿Salgo? |
|---|---|---|
| sí | sí | **sí** |
| sí | no | no |
| no | sí | no |
| no | no | no |

```python
buen_tiempo = True
trabajo_terminado = False
print(buen_tiempo and trabajo_terminado)
```

```sortie
False
```

### «o»: basta con uno

«Tengo café **o** té» es verdadero en cuanto **al menos uno** de los dos es
verdadero.

| ¿Café? | ¿Té? | ¿Tengo una bebida? |
|---|---|---|
| sí | sí | **sí** |
| sí | no | **sí** |
| no | sí | **sí** |
| no | no | no |

```python
cafe = False
te = True
print(cafe or te)
```

```sortie
True
```

### «no»: lo contrario

`not` le da la vuelta a la respuesta: verdadero se vuelve falso, falso se
vuelve verdadero.

```python
llueve = False
print(not llueve)
```

```sortie
True
```

El mismo cálculo en JavaScript (Ruby escribe exactamente lo mismo, con
`puts` en lugar de `console.log`):

```javascript
const buenTiempo = true;
const trabajoTerminado = false;
console.log(buenTiempo && trabajoTerminado, buenTiempo || trabajoTerminado, !buenTiempo);
```

```sortie
false true false
```

> **Recuerda:** «y» pide que todo sea verdadero; «o» se conforma con un
> solo verdadero; «no» invierte.

## Elegir un camino: if y else

Imagina una carretera que se divide en dos. Una señal hace una pregunta.
Según la respuesta, vas a la izquierda o a la derecha. Luego las dos
carreteras se vuelven a juntar.

```text
              salida
                |
        +---------------+
        | edad >= 18 ?  |
        +---------------+
           sí /     \ no
             /       \
   «Puedes         «Todavía no.»
     votar.»
             \       /
              \     /
                |
            lo que sigue
```

Eso es exactamente lo que hacen `if` («si») y `else` («si no»):

```python
edad = 20
if edad >= 18:
    print("Puedes votar.")
else:
    print("Todavía no.")
print("Adiós.")
```

```sortie
Puedes votar.
Adiós.
```

Léelo en voz alta: «**si** la edad es 18 o más, muestra…; **si no**,
muestra…». Solo se ejecuta una de las dos ramas. La última línea, en
cambio, se ejecuta siempre: es «lo que sigue».

En Python, los dos puntos `:` y el **desplazamiento** de las líneas (se
llama **sangría**, ver [sangría](glossaire.md#indentacion)) muestran lo que
está **dentro** del `if`.

Prueba: pon `edad = 15`. El programa muestra «Todavía no.» y luego
«Adiós.».

El `else` es opcional: si no hay nada que hacer cuando la respuesta es no,
no se escribe.

### En JavaScript y en Ruby

JavaScript pone la pregunta entre paréntesis y las ramas entre llaves
`{ }`:

```javascript
const edad = 20;
if (edad >= 18) {
  console.log("Puedes votar.");
} else {
  console.log("Todavía no.");
}
```

```sortie
Puedes votar.
```

Ruby cierra la condición con `end`:

```ruby
edad = 20
if edad >= 18
  puts "Puedes votar."
else
  puts "Todavía no."
end
```

```sortie
Puedes votar.
```

> **Recuerda:** `if` hace una pregunta; según la respuesta, solo se ejecuta
> una rama.

## Más de dos caminos: elif

Una nota de 0 a 20 da una calificación. Hay **cuatro** caminos, no dos.

Una primera idea: poner un `if` dentro del `else`, luego otro, etc. Se
habla de condiciones **anidadas**:

```python
nota = 13
if nota >= 16:
    print("Sobresaliente")
else:
    if nota >= 12:
        print("Notable")
    else:
        if nota >= 10:
            print("Aprobado")
        else:
            print("Suspenso")
```

```sortie
Notable
```

Funciona, pero el código se va cada vez más a la derecha. Cuesta leerlo.
Python tiene una palabra para «si no, si»: `elif`.

```python
nota = 13
if nota >= 16:
    print("Sobresaliente")
elif nota >= 12:
    print("Notable")
elif nota >= 10:
    print("Aprobado")
else:
    print("Suspenso")
```

```sortie
Notable
```

El ordenador hace las preguntas **en orden**, y se detiene en la
**primera** respuesta «sí»:

| Paso | Pregunta | Respuesta para 13 | ¿Qué pasa? |
|---|---|---|---|
| 1 | `nota >= 16` | no | pregunta siguiente |
| 2 | `nota >= 12` | sí | muestra «Notable», y se acabó |
| 3 | `nota >= 10` | no se hace | – |

13 también es mayor que 10, pero esa pregunta nunca se hace. Así que el
orden de las preguntas importa: pon la más exigente primero.

En JavaScript, se escribe `else if`: `} else if (nota >= 12) {`. En Ruby,
se escribe `elsif` (sin «e»):

```ruby
nota = 13
if nota >= 16
  puts "Sobresaliente"
elsif nota >= 12
  puts "Notable"
else
  puts "Suspenso"
end
```

```sortie
Notable
```

Prueba: pon `nota = 16`, y luego `nota = 9`. Sigue las preguntas una a una
antes de ejecutar el programa.

> **Recuerda:** con `elif`, gana la primera condición verdadera; las
> siguientes se ignoran.

## Errores frecuentes

### Escribir `=` en lugar de `==`

`=` guarda un valor; no hace ninguna pregunta. Python lo rechaza:

```python norun
x = 5
if x = 5:
    print("cinco")
```

```text
  File "cinco.py", line 2
    if x = 5:
       ^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

«Maybe you meant '=='?»: «¿querías decir `==`?». JavaScript, en cambio, no
dice **nada**, y eso es peor:

```javascript
let x = 3;
if (x = 0) {
  console.log("cero");
} else {
  console.log("no es cero");
}
console.log(x);
```

```sortie
no es cero
0
```

`x = 0` ha guardado 0 en `x`: el valor de `x` se ha perdido. La
corrección: `if (x == 0)`, o mejor en JavaScript, `if (x === 0)`.

### Comprobar un intervalo con `0 < x < 10` en JavaScript

En matemáticas, `0 < x < 10` quiere decir «x está entre 0 y 10».
JavaScript lo lee de otra manera:

```javascript
const x = 50;
if (0 < x < 10) {
  console.log("entre 0 y 10");
}
```

```sortie
entre 0 y 10
```

¡50 no está entre 0 y 10! JavaScript calcula primero `0 < 50`, que vale
`true`. Luego calcula `true < 10`. Convierte `true` en `1`, y `1 < 10` es
verdadero.

La corrección: dos preguntas unidas por «y».

```javascript
const x = 50;
if (0 < x && x < 10) {
  console.log("entre 0 y 10");
} else {
  console.log("fuera");
}
```

```sortie
fuera
```

En Python, `0 < x < 10` funciona como en matemáticas. En Ruby, provoca un
error (`undefined method '<' for true`): escribe `0 < x && x < 10`.

### Olvidar el caso «igual»

Una tienda ofrece el envío gratis **a partir de** 50 €.

```python
carrito = 50
if carrito > 50:
    print("Envío gratis")
else:
    print("Envío: 5 €")
```

```sortie
Envío: 5 €
```

No hay ningún mensaje de error: es un **error de lógica**. Con 50 € justos,
el cliente debería tener el envío gratis. «A partir de 50» incluye 50: hacía
falta `>=`. Con `if carrito >= 50:`, el programa muestra «Envío gratis».

> **Consejo:** prueba siempre el valor **exacto** del límite (aquí 50), más
> uno justo por debajo y uno justo por encima.

## Ponte a prueba

**1.** ¿Cuánto vale `(3 > 2) and (1 == 2)`?

> **Respuesta**
>
> `False`. `3 > 2` es verdadero, pero `1 == 2` es falso. Con «y», los dos
> tienen que ser verdaderos.

**2.** Con `nota = 10`, ¿qué muestra el programa de calificaciones con
`elif`?

> **Respuesta**
>
> «Aprobado». `10 >= 16`: no. `10 >= 12`: no. `10 >= 10`: sí, porque `>=`
> incluye la igualdad.

**3.** ¿Cuál es la diferencia entre `=` y `==`?

> **Respuesta**
>
> `=` guarda un valor en una variable (`edad = 20`). `==` compara dos
> valores y responde verdadero o falso (`edad == 20`).

## En resumen

- Un booleano vale verdadero o falso: `True`/`False` en Python, `true`/`false` en JavaScript y Ruby.
- Una comparación (`==`, `!=`, `<`, `<=`, `>`, `>=`) responde con un booleano.
- «y» quiere todo verdadero, «o» un solo verdadero, «no» invierte.
- `if` / `else` elige una rama; `elif` encadena las preguntas, y gana la primera verdadera.
- Cuidado con `=` en lugar de `==`, y con el caso «igual»: ¿`>` o `>=`?

Siguiente: [los bucles](boucles.md)
