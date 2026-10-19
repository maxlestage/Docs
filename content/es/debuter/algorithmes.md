# Pensar como un programador

Antes de escribir código, se piensa en el método: esta página muestra cómo dividir un problema en pasos pequeños, escribirlo en español y luego traducirlo a Python, JavaScript, Ruby, Rust y Rythme.

> **Objetivos:**
>
> - saber qué es un algoritmo;
> - dividir un problema en pasos pequeños;
> - escribir un método en español, en pseudocódigo;
> - usar variables, condiciones y repeticiones;
> - traducir un pseudocódigo a varios lenguajes;
> - comprobar un algoritmo a mano, con una tabla de valores.

## Qué es un algoritmo

Un **algoritmo** es una secuencia de pasos precisos que resuelve un problema.
Una receta de cocina, el camino para ir a casa de un amigo, las instrucciones de un
mueble: todo eso son algoritmos.

Un buen algoritmo tiene tres cualidades:

- cada paso es **claro**: solo hay una forma de entenderlo;
- los pasos están en el **orden correcto**;
- **termina** y da un resultado.

Un programa es un algoritmo escrito en un lenguaje de programación. La
parte difícil suele ser el algoritmo, no el lenguaje.

> **Recuerda:** primero el método, después el código.

## Dividir un problema en pasos pequeños

Un problema grande da miedo. Dividido en trozos pequeños, se vuelve sencillo.

Tomemos «preparar un té». Es demasiado vago para un ordenador.
Lo dividimos:

```text
1. Llenar el hervidor de agua.
2. Encender el hervidor.
3. Esperar a que el agua hierva.
4. Poner una bolsita de té en una taza.
5. Verter el agua en la taza.
6. Esperar 3 minutos.
7. Quitar la bolsita.
```

Ahora cada paso es lo bastante pequeño como para hacerlo sin pensar. Si
un paso sigue siendo confuso, se divide otra vez.

> **Consejo:** pregúntate en cada línea: «¿sabría hacerlo un niño de 8
> años?». Si la respuesta es no, divide.

## Escribir primero en español: el pseudocódigo

El **pseudocódigo** es un algoritmo escrito en español sencillo, con la forma
de un programa. No tiene reglas estrictas: sirve para pensar, no para
ejecutarse.

```text
preguntar la edad de la persona
si la edad es 18 o más
    mostrar «Puedes votar.»
si no
    mostrar «Todavía no.»
```

El pseudocódigo tiene dos ventajas:

- piensas en el problema **sin pelearte** con la sintaxis;
- después se traduce a **cualquier lenguaje**.

## Las variables: cajas con una etiqueta

Un algoritmo tiene que recordar cosas: una edad, un total, un nombre. Se
guardan en **variables**.

Imagina una caja con una etiqueta. La etiqueta es el **nombre** de la
variable; lo que hay dentro es su **valor**.

```text
poner 0 en la caja «total»
poner 5 en la caja «precio»
poner total + precio en la caja «total»   (total vale ahora 5)
```

En Ruby, se escribe con `=`:

```ruby
total = 0
precio = 5
total = total + precio
puts total
```

```sortie
5
```

> **Atención:** en programación, `=` no significa «es igual a».
> Significa «guarda en la caja». Para comparar, se escribe `==`.

Elige nombres que digan lo que contiene la caja: `precio_total`
mejor que `x`.

## Las condiciones en la vida diaria

Tomas decisiones todo el día: «**si** llueve, cojo un
paraguas; **si no**, cojo unas gafas de sol». Eso es una
**condición**.

En pseudocódigo:

```text
si llueve
    coger un paraguas
si no
    coger unas gafas de sol
```

En Ruby:

```ruby
llueve = true
if llueve
  puts "Cojo un paraguas."
else
  puts "Cojo unas gafas de sol."
end
```

```sortie
Cojo un paraguas.
```

Una condición es siempre una pregunta que se responde con **sí** o
**no**: `true` o `false`.

## La repetición en la vida diaria

Muchas tareas se repiten: «por cada plato sucio, lavarlo»,
«mientras la masa no esté lisa, remover». Eso es una **repetición**, o
**bucle**.

Hay dos tipos de repeticiones:

- **por cada** elemento de una lista: sabes cuántas veces repites;
- **mientras** algo sea verdad: te detienes cuando cambia.

```text
por cada plato de la pila
    lavar el plato
    ponerlo en el escurridor
```

En Ruby:

```ruby
platos = ["azul", "blanco", "verde"]
platos.each do |plato|
  puts "Lavo el plato #{plato}."
end
```

```sortie
Lavo el plato azul.
Lavo el plato blanco.
Lavo el plato verde.
```

> **Recuerda:** con variables, condiciones y repeticiones,
> se pueden escribir casi todos los algoritmos.

## Un ejemplo completo: encontrar el número más grande

Aquí tienes una lista de números: `4, 17, 9, 12`. ¿Cómo encontrar el
más grande?

Una persona mira y responde «17». Pero si la lista contiene un millón de
números, hace falta un **método**.

### Pensar como una persona

Imagina unas cartas que se dan la vuelta una a una. Recuerdas el número
más grande visto hasta ahora. Con cada carta nueva, comparas: si es
más grande, se convierte en tu nuevo «mayor».

### El pseudocódigo

```text
poner el primer número en la caja «mayor»
por cada número de la lista
    si número es más grande que mayor
        poner número en la caja «mayor»
mostrar mayor
```

Hay una variable, una repetición y una condición. Nada más.

### En Python

```python
numeros = [4, 17, 9, 12]
mayor = numeros[0]
for numero in numeros:
    if numero > mayor:
        mayor = numero
print(mayor)
```

```sortie
17
```

Cada línea del pseudocódigo se ha convertido en una línea de Python. El desplazamiento
de las líneas (la sangría) muestra lo que está **dentro** del bucle y **dentro**
de la condición.

### En JavaScript

```javascript
const numeros = [4, 17, 9, 12];
let mayor = numeros[0];
for (const numero of numeros) {
  if (numero > mayor) {
    mayor = numero;
  }
}
console.log(mayor);
```

```sortie
17
```

`let` anuncia una caja cuyo contenido va a cambiar; `const`, una caja
que siempre guarda el mismo valor. Las llaves `{ }` sustituyen
la sangría de Python.

### En Ruby

```ruby
numeros = [4, 17, 9, 12]
mayor = numeros[0]
numeros.each do |numero|
  if numero > mayor
    mayor = numero
  end
end
puts mayor
```

```sortie
17
```

La misma forma, con `each` y `end`.

### En Rust

```rust
let numeros = [4, 17, 9, 12];
let mut mayor = numeros[0];
for numero in numeros {
    if numero > mayor {
        mayor = numero;
    }
}
println!("{mayor}");
```

```sortie
17
```

La forma es la misma. Rust pide `mut`: la caja `mayor` va a
cambiar de contenido, y Rust quiere que lo digas.

### En Rythme

En Rythme, un valor nunca cambia: no se llena la caja
varias veces. Lo que se hace es describir **un paso** («entre el mayor
actual y un número, quedarse con el más grande») y luego aplicarlo a toda la
lista con `fold`:

```rythme
fn quedarse_con_el_mayor(mayor, numero) = if numero > mayor then numero else mayor

numeros = [4, 17, 9, 12]
print(fold(numeros, numeros[0], quedarse_con_el_mayor))
```

```sortie
17
```

`fold` empieza en `numeros[0]` y luego pasa cada número a
`quedarse_con_el_mayor`, junto con el resultado del paso anterior. Es el
mismo algoritmo, dicho de otra manera. Consulta [pensar en funcional](../rythme/fonctionnel.md).

### El atajo

Encontrar el más grande es tan habitual que todos los lenguajes ya lo hacen:

```python
print(max([4, 17, 9, 12]))
```

```sortie
17
```

```javascript
console.log(Math.max(...[4, 17, 9, 12]));
```

```sortie
17
```

```ruby
puts [4, 17, 9, 12].max
```

```sortie
17
```

```rust
let numeros = [4, 17, 9, 12];
println!("{:?}", numeros.iter().max());
```

```sortie
Some(17)
```

```rythme
print(max([4, 17, 9, 12]))
```

```sortie
17
```

Rust responde `Some(17)`: para una lista vacía, respondería `None`, porque
no hay número más grande. Los demás lenguajes reaccionan cada uno a
su manera ante una lista vacía: Python lanza un error, JavaScript responde
`-Infinity`. Usa los atajos en tus programas
de verdad; escribe el algoritmo tú mismo para aprender.

## Probar a mano con una tabla de valores

Antes de fiarte de un algoritmo, síguelo **a mano**, con
papel y lápiz. Anota el valor de cada variable en cada vuelta.

Para la lista `4, 17, 9, 12`:

| Vuelta | numero | ¿numero > mayor?      | mayor después    |
|--------|--------|-----------------------|------------------|
| inicio | –      | –                     | 4                |
| 1      | 4      | no (4 > 4 es falso)   | 4                |
| 2      | 17     | sí                    | 17               |
| 3      | 9      | no                    | 17               |
| 4      | 12     | no                    | 17               |

Al final, `mayor` vale 17: es correcto.

Prueba después casos **difíciles**:

- el mayor es el **primero**: `20, 3, 5`;
- el mayor es el **último**: `3, 5, 20`;
- todos los números son **negativos**: `-8, -2, -5`;
- la lista tiene **un solo** número: `7`;
- la lista está **vacía**.

El último caso da problemas: no hay «primer número». Un buen
programador piensa en estos casos antes de que el programa falle.

> **Consejo:** en Rythme, `rythme steps archivo.ry` muestra el cálculo paso
> a paso, como tu tabla.

## Errores frecuentes

### Empezar desde cero en lugar del primer número

Es tentador: se pone `0` en `mayor` al principio. Pero con números
negativos, ninguno es mayor que 0.

```ruby norun
numeros = [-8, -2, -5]
mayor = 0
numeros.each do |numero|
  mayor = numero if numero > mayor
end
puts mayor
```

```text
0
```

No hay mensaje de error: es un **error de lógica**. El
programa responde 0, que ni siquiera está en la lista. Empieza por el primer
número: `mayor = numeros[0]`.

### Olvidar el caso de la lista vacía

Con una lista vacía, `numeros[0]` no existe. Rust detiene el programa:

```rust norun
fn main() {
    let numeros: Vec<i32> = vec![];
    let mayor = numeros[0];
    println!("{mayor}");
}
```

```text
thread 'main' (5140) panicked at main.rs:3:24:
index out of bounds: the len is 0 but the index is 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Comprueba primero que la lista no esté vacía, o usa el atajo
`numeros.iter().max()`, que responde `None` para una lista vacía.

### Un bucle «mientras» que nunca termina

Si nada cambia dentro del bucle, la condición sigue siendo verdadera para siempre.

```ruby norun
i = 0
while i < 3
  puts i
end
```

El programa muestra `0` sin fin. Detenlo con **Ctrl + C**:

```text
bucle.rb:3:in `write': Interrupt
	from bucle.rb:3:in `puts'
	from bucle.rb:3:in `puts'
	from bucle.rb:3:in `<main>'
```

Faltaba `i = i + 1` dentro del bucle. En tu tabla de valores, habrías
visto que `i` nunca cambia.

## Ponte a prueba

**1.** ¿Qué es el pseudocódigo y para qué sirve?

> **Respuesta**
>
> Es un algoritmo escrito en español sencillo, con la forma de un
> programa. Sirve para pensar en el método sin preocuparse por la
> sintaxis; después se traduce al lenguaje que elijas.

**2.** Escribe el pseudocódigo para encontrar el número **más pequeño** de una
lista.

> **Respuesta**
>
> ```text
> poner el primer número en la caja «menor»
> por cada número de la lista
>     si número es más pequeño que menor
>         poner número en la caja «menor»
> mostrar menor
> ```
>
> Solo cambia la comparación: «más pequeño» en lugar de «más grande».

**3.** Con la lista `5, 2, 8`, ¿qué valores toma `mayor` en
nuestro algoritmo?

> **Respuesta**
>
> 5 al principio, 5 después de la primera vuelta (5 no es mayor que 5), 5
> después de la segunda (2 es menor), y luego 8 después de la tercera. El
> resultado es 8.

## En resumen

- Un algoritmo es una secuencia de pasos claros, en el orden correcto, que termina.
- Divide un problema grande en pasos pequeños y luego escríbelos en pseudocódigo.
- Una variable es una caja con una etiqueta; `=` guarda un valor dentro.
- Las condiciones eligen, las repeticiones vuelven a empezar.
- El mismo algoritmo se escribe en Python, JavaScript, Ruby, Rust o Rythme: solo cambian las palabras.
- Prueba a mano con una tabla de valores, sin olvidar los casos difíciles.

Siguiente: [la terminal en 10 minutos](terminal.md).
