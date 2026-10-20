# Clases y objetos

Esta página explica cómo crear tus propios tipos con `class`: sus atributos, sus métodos, cómo se muestran y sus propiedades.

> **Objetivos:**
>
> - sabrás escribir una clase con `__init__` y crear objetos;
> - sabrás qué representa `self`;
> - sabrás añadir métodos y una forma de mostrarlos con `__str__` y `__repr__`;
> - sabrás usar `@property`, `@classmethod` y `@staticmethod`;
> - sabrás que una variable contiene una **referencia** a un objeto.

## ¿Por qué clases?

Hasta ahora, has guardado tus datos en listas y diccionarios. Para una cuenta bancaria,
queda así:

```python
cuenta = {"titular": "Alice", "saldo": 100}
cuenta["saldo"] += 50
print(cuenta)
```

```sortie
{'titular': 'Alice', 'saldo': 150}
```

Funciona, pero nada impide una errata (`"sado"`), y las acciones (depositar, retirar)
están dispersas en otras partes del código.

Una [clase](../debuter/glossaire.md#clase) agrupa **los datos y las acciones** en el
mismo lugar. Es un plano de construcción: la clase `Casa` describe las casas; cada casa
construida a partir del plano es un [objeto](../debuter/glossaire.md#objeto), también
llamado **instancia**.

> **Nota:** en Python, usas objetos desde el principio: `"abc"` es un objeto de la clase
> `str`, `[1, 2]` un objeto de la clase `list`. `type("abc")` muestra `<class 'str'>`.

## Una primera clase

Se escribe `class`, un nombre en `CamelCase` (una mayúscula en cada palabra) y luego dos
puntos. Se crea un objeto **llamando** a la clase como a una función.

```python
class Perro:
    pass

rex = Perro()
toby = Perro()
print(type(rex))
print(rex is toby)
```

```sortie
<class '__main__.Perro'>
False
```

`pass` quiere decir «nada por ahora». `rex` y `toby` son dos objetos distintos,
fabricados con el mismo plano. `__main__` es el nombre del archivo principal.

## __init__ y self

El método especial `__init__` (*inicializar*) se ejecuta automáticamente al crear el
objeto. Sirve para rellenar sus **atributos**: las variables propias de cada objeto.

```python
class Perro:
    def __init__(self, nombre, edad):
        self.nombre = nombre
        self.edad = edad

rex = Perro("Rex", 3)
toby = Perro("Toby", 5)
print(rex.nombre, rex.edad)
print(toby.nombre, toby.edad)
```

```sortie
Rex 3
Toby 5
```

¿Quién es `self`? Es **el objeto que se está construyendo**. Cuando escribes
`Perro("Rex", 3)`, Python:

1. crea un objeto vacío;
2. llama a `__init__(objeto, "Rex", 3)`: `self` es ese objeto;
3. te devuelve el objeto relleno.

Así que `self.nombre = nombre` quiere decir: «guarda el valor `nombre` en el atributo
`nombre` de este objeto». Sin `self.`, el valor seguiría siendo una simple variable
local, perdida al final de `__init__`.

> **Recuerda:** `self` es siempre el **primer parámetro** de los métodos, pero nunca lo
> pasas tú: Python lo hace por ti.

## Los métodos

Un [método](../debuter/glossaire.md#metodo) es una función escrita dentro de la clase.
Recibe `self` y por eso puede leer y modificar los atributos del objeto.

```python
class Cuenta:
    def __init__(self, titular, saldo=0):
        self.titular = titular
        self.saldo = saldo

    def depositar(self, cantidad):
        self.saldo += cantidad

    def retirar(self, cantidad):
        if cantidad > self.saldo:
            print("Saldo insuficiente")
            return
        self.saldo -= cantidad

cuenta = Cuenta("Alice", 100)
cuenta.depositar(50)
cuenta.retirar(500)
cuenta.retirar(30)
print(cuenta.saldo)
```

```sortie
Saldo insuficiente
120
```

`cuenta.depositar(50)` es un atajo de `Cuenta.depositar(cuenta, 50)`: el objeto que va
antes del punto se convierte en `self`.

## Mostrar un objeto: __str__ y __repr__

Por defecto, `print` muestra un objeto de una forma poco útil, con una dirección de
memoria que cambia en cada ejecución (por ejemplo `<__main__.Punto object at 0x7f3a…>`).
Dos métodos especiales resuelven este problema:

- `__str__`: el texto para un **humano**, que usan `print` y `str()`;
- `__repr__`: el texto para un **desarrollador**, que se usa en las listas, en `repr()`
  y en la consola. Lo ideal es que se parezca al código que vuelve a crear el objeto.

```python
class Punto:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __str__(self):
        return f"({self.x}, {self.y})"

    def __repr__(self):
        return f"Punto({self.x}, {self.y})"

p = Punto(2, 5)
print(p)
print(repr(p))
print([p, Punto(0, 0)])
```

```sortie
(2, 5)
Punto(2, 5)
[Punto(2, 5), Punto(0, 0)]
```

> **Consejo:** si solo escribes uno de los dos, escribe `__repr__`: `print` también lo
> usa cuando falta `__str__`.

## Atributos de clase

Un atributo escrito **directamente en la clase** (no en `__init__`) lo comparten todos
los objetos. Sirve para una constante común o un contador.

```python
class Perro:
    especie = "Canis familiaris"
    cantidad = 0

    def __init__(self, nombre):
        self.nombre = nombre
        Perro.cantidad += 1

a = Perro("Rex")
b = Perro("Toby")
print(a.especie, b.especie)
print(Perro.cantidad)
```

```sortie
Canis familiaris Canis familiaris
2
```

> **Atención:** nunca uses una lista o un diccionario como atributo de clase si cada
> objeto debe tener el suyo: todos los objetos compartirían **la misma** lista. Créala
> en `__init__` con `self.trucos = []`.

## Las propiedades con @property

A veces, un valor se **calcula** a partir de otros atributos. `@property` convierte un
método en un atributo de solo lectura: se usa sin paréntesis.

```python
class Rectangulo:
    def __init__(self, ancho, alto):
        self.ancho = ancho
        self.alto = alto

    @property
    def area(self):
        return self.ancho * self.alto

r = Rectangulo(3, 4)
print(r.area)
r.ancho = 10
print(r.area)
```

```sortie
12
40
```

El área siempre está al día, porque se vuelve a calcular en cada lectura.

### Controlar un cambio

Con `@nombre.setter`, puedes comprobar un valor antes de aceptarlo. Por convención, el
valor real se guarda en un atributo que empieza por `_`: significa «interno, no tocar
directamente».

```python
class Termostato:
    def __init__(self, temperatura):
        self.temperatura = temperatura

    @property
    def temperatura(self):
        return self._temperatura

    @temperatura.setter
    def temperatura(self, valor):
        if valor < 5 or valor > 30:
            raise ValueError(f"temperatura fuera de rango: {valor}")
        self._temperatura = valor

t = Termostato(20)
t.temperatura = 22
print(t.temperatura)
try:
    t.temperatura = 80
except ValueError as e:
    print(e)
```

```sortie
22
temperatura fuera de rango: 80
```

`raise` y `try` se explican en [gestionar los errores](erreurs.md).

## @classmethod y @staticmethod

Otros dos decoradores cambian lo que recibe un método:

| Decorador | Primer parámetro | Uso |
|-----------|------------------|-----|
| ninguno | `self`, el objeto | método normal |
| `@classmethod` | `cls`, la clase | otra forma de **crear** un objeto |
| `@staticmethod` | nada | función auxiliar guardada en la clase |

```python
class Fecha:
    def __init__(self, dia, mes, anio):
        self.dia = dia
        self.mes = mes
        self.anio = anio

    @classmethod
    def desde_texto(cls, texto):
        dia, mes, anio = texto.split("/")
        return cls(int(dia), int(mes), int(anio))

    @staticmethod
    def es_bisiesto(anio):
        return anio % 4 == 0 and (anio % 100 != 0 or anio % 400 == 0)

    def __repr__(self):
        return f"Fecha({self.dia}, {self.mes}, {self.anio})"

f = Fecha.desde_texto("14/07/2024")
print(f)
print(Fecha.es_bisiesto(2024))
```

```sortie
Fecha(14, 7, 2024)
True
```

`desde_texto` es un «constructor alternativo»: prepara los valores y luego llama a
`cls(...)`, es decir, a `Fecha(...)`.

## Los objetos son referencias

Una variable no **contiene** el objeto: contiene una **referencia** a él, como una
etiqueta pegada encima. Dos variables pueden ser dos etiquetas sobre el mismo objeto.

```python
class Cuenta:
    def __init__(self, saldo):
        self.saldo = saldo

a = Cuenta(100)
b = a
b.saldo = 0
print(a.saldo)
print(a is b)
```

```sortie
0
True
```

`b = a` no copia la cuenta: `a` y `b` designan el **mismo** objeto. Pasa lo mismo
cuando le pasas un objeto a una función: la función puede modificarlo.

```python
class Cuenta:
    def __init__(self, saldo):
        self.saldo = saldo

def vaciar(cuenta):
    cuenta.saldo = 0

c = Cuenta(100)
vaciar(c)
print(c.saldo)
```

```sortie
0
```

Para una copia realmente independiente, usa el módulo `copy`:

```python
import copy

class Cuenta:
    def __init__(self, saldo):
        self.saldo = saldo

a = Cuenta(100)
b = copy.copy(a)
b.saldo = 0
print(a.saldo, b.saldo)
```

```sortie
100 0
```

> **Nota:** `is` responde a «¿es el mismo objeto?», mientras que `==` responde a «¿tienen
> el mismo valor?». En tus clases, `==` compara la identidad mientras no hayas escrito
> `__eq__` (ver [herencia y polimorfismo](heritage.md)).

## Errores frecuentes

### Olvidar self en un método

```python norun
class Perro:
    def ladrar():
        print("¡Guau!")

rex = Perro()
rex.ladrar()
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    rex.ladrar()
    ~~~~~~~~~~^^
TypeError: Perro.ladrar() takes 0 positional arguments but 1 was given
```

Python siempre pasa el objeto como primer argumento: «1 was given» es `rex`. Añade
`self`: `def ladrar(self):`.

### Olvidar self. delante de un atributo

```python norun
class Perro:
    def __init__(self, nombre):
        nombre = nombre

rex = Perro("Rex")
print(rex.nombre)
```

```text
Traceback (most recent call last):
  File "main.py", line 6, in <module>
    print(rex.nombre)
          ^^^^^^^^^^
AttributeError: 'Perro' object has no attribute 'nombre'
```

`nombre = nombre` no guarda nada en el objeto. Escribe `self.nombre = nombre`.

### Olvidar un argumento del constructor

```python norun
class Perro:
    def __init__(self, nombre):
        self.nombre = nombre

rex = Perro()
```

```text
Traceback (most recent call last):
  File "main.py", line 5, in <module>
    rex = Perro()
TypeError: Perro.__init__() missing 1 required positional argument: 'nombre'
```

Los parámetros de `__init__` (salvo `self`) son los que se le dan a la clase:
`Perro("Rex")`. O dale un valor por defecto: `def __init__(self, nombre="Desconocido")`.

### Modificar una propiedad sin setter

```python norun
class Circulo:
    def __init__(self, radio):
        self.radio = radio

    @property
    def diametro(self):
        return self.radio * 2

c = Circulo(3)
c.diametro = 10
```

```text
Traceback (most recent call last):
  File "main.py", line 10, in <module>
    c.diametro = 10
    ^^^^^^^^^^
AttributeError: property 'diametro' of 'Circulo' object has no setter
```

Una propiedad sin `@diametro.setter` es de solo lectura. Modifica `c.radio`, o añade un
setter que calcule `self.radio = valor / 2`.

## Ponte a prueba

**Pregunta 1:** ¿qué diferencia hay entre `self.nombre` y `Perro.cantidad` en la clase
`Perro` vista más arriba?

> **Respuesta**
>
> `self.nombre` es un atributo **de objeto**: cada perro tiene el suyo. `Perro.cantidad`
> es un atributo **de clase**: un único valor, compartido por todos los perros.

**Pregunta 2:** ¿qué muestra este código?

```python norun
class Caja:
    def __init__(self):
        self.contenido = []

a = Caja()
b = a
b.contenido.append("llave")
print(a.contenido)
```

> **Respuesta**
>
> `['llave']`. `b = a` no crea una caja nueva: `a` y `b` designan el mismo objeto.

**Pregunta 3:** escribe una clase `Cuadrado` con un atributo `lado` y una propiedad
`perimetro`.

> **Respuesta**
>
> ```python
> class Cuadrado:
>     def __init__(self, lado):
>         self.lado = lado
>
>     @property
>     def perimetro(self):
>         return 4 * self.lado
>
> print(Cuadrado(5).perimetro)
> ```
>
> ```sortie
> 20
> ```

## En resumen

- `class Nombre:` define un tipo; `Nombre(...)` crea un objeto y llama a `__init__`.
- `self` es el objeto actual; `self.x = ...` crea un atributo propio del objeto.
- `__str__` da lo que muestra `print`, `__repr__` lo que ve el desarrollador.
- `@property` calcula un atributo; `@classmethod` crea objetos de otra forma;
  `@staticmethod` guarda una función auxiliar.
- Una variable es una referencia: `b = a` no copia el objeto.

Siguiente: [herencia y polimorfismo](heritage.md). Compara con
[las clases en Ruby](../ruby/types.md).
