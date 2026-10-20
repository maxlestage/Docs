# Herencia y polimorfismo

Esta página muestra cómo una clase puede heredar de otra, cómo objetos distintos pueden responder al mismo mensaje y cuándo preferir la composición a la herencia.

> **Objetivos:**
>
> - sabrás crear una subclase con `<` y llamar al padre con `super`;
> - sabrás redefinir un método y leer la cadena `ancestors`;
> - sabrás tratar una lista de objetos distintos de la misma manera (polimorfismo);
> - sabrás la diferencia entre `private` y `protected`;
> - sabrás cuándo usar la composición en lugar de la herencia.

## Crear una subclase

El capítulo [clases y objetos](types.md#la-herencia) presentó la herencia. Recordatorio: una
[clase](../debuter/glossaire.md#classe) puede **heredar** de otra con `<`. Recibe todos
sus métodos, como un hijo hereda los rasgos de sus padres.

```ruby
class Vehiculo
  def initialize(marca)
    @marca = marca
  end

  def arrancar
    "#{@marca} arranca."
  end
end

class Coche < Vehiculo
end

puts Coche.new("Renault").arrancar
puts Coche.superclass
```

```sortie
Renault arranca.
Vehiculo
```

`Coche` no ha escrito nada, pero ya sabe `arrancar`. `superclass` da el padre directo.

## Redefinir un método

La subclase puede escribir su propia versión de un método del padre. Se dice que lo
**redefine**. Es su versión la que se usa.

```ruby
class Vehiculo
  def ruido
    "..."
  end
end

class Moto < Vehiculo
  def ruido
    "¡Brum!"
  end
end

class Bici < Vehiculo
  def ruido
    "¡Rin rin!"
  end
end

puts Vehiculo.new.ruido
puts Moto.new.ruido
puts Bici.new.ruido
```

```sortie
...
¡Brum!
¡Rin rin!
```

## Completar con super

`super` llama al método del mismo nombre en el padre. Así puedes **añadir** algo en
lugar de reescribirlo todo.

```ruby
class Empleado
  def initialize(nombre, salario)
    @nombre = nombre
    @salario = salario
  end

  def ficha
    "#{@nombre}: #{@salario} €"
  end
end

class Jefe < Empleado
  def initialize(nombre, salario, equipo)
    super(nombre, salario)
    @equipo = equipo
  end

  def ficha
    super + " (equipo de #{@equipo.size})"
  end
end

puts Empleado.new("Lea", 2500).ficha
puts Jefe.new("Hugo", 4000, ["Lea", "Tom"]).ficha
```

```sortie
Lea: 2500 €
Hugo: 4000 € (equipo de 2)
```

> **Atención:** `super` solo, sin paréntesis, pasa **los mismos argumentos** que ha
> recibido. `super()` con paréntesis vacíos no pasa ninguno.

## La cadena de ancestros

Cuando llamas a un método, Ruby lo busca en la clase del objeto, luego en su padre,
luego en el padre del padre… `ancestors` muestra este orden de búsqueda.

```ruby
class Animal
end

class Perro < Animal
end

p Perro.ancestors
puts Perro.new.is_a?(Animal)
puts Perro.new.instance_of?(Animal)
```

```sortie
[Perro, Animal, Object, Kernel, BasicObject]
true
false
```

Toda clase hereda al final de `Object`. `Kernel` es un
[módulo](../debuter/glossaire.md#module) incluido en `Object`: es él quien proporciona
`puts`. `is_a?` mira toda la cadena, `instance_of?` solo la clase exacta.

## El polimorfismo

«Polimorfismo» quiere decir «varias formas». Objetos de clases distintas responden al
**mismo mensaje**, cada uno a su manera. El código que los usa no necesita saber quién
es quién.

```ruby
class Forma
  def descripcion
    "#{self.class.name} de área #{area.round(2)}"
  end
end

class Rectangulo < Forma
  def initialize(ancho, alto)
    @ancho = ancho
    @alto = alto
  end

  def area = @ancho * @alto
end

class Circulo < Forma
  def initialize(radio)
    @radio = radio
  end

  def area = Math::PI * @radio**2
end

formas = [Rectangulo.new(3, 4), Circulo.new(1), Rectangulo.new(2, 2)]
formas.each { |f| puts f.descripcion }
puts "Total: #{formas.sum(&:area).round(2)}"
```

```sortie
Rectangulo de área 12
Circulo de área 3.14
Rectangulo de área 4
Total: 19.14
```

El bucle no contiene ningún `if`. Para añadir un triángulo, basta con escribir una
clase nueva: el bucle no cambia.

## Los métodos abstractos

`Forma`, más arriba, usa `area`, pero no la define. Si una subclase se olvida de
escribirla, el error es confuso. Un método **abstracto** deja clara la regla: existe en
el padre solo para lanzar `NotImplementedError`.

```ruby
class Forma
  def area
    raise NotImplementedError, "#{self.class} debe definir area"
  end
end

class Triangulo < Forma
end

begin
  Triangulo.new.area
rescue NotImplementedError => e
  puts "Olvido: #{e.message}"
end
```

```sortie
Olvido: Triangulo debe definir area
```

## El duck typing

En Ruby, la herencia ni siquiera es necesaria para el polimorfismo. Lo que importa es
que el objeto **sepa responder** al mensaje. «Si camina como un pato y grazna como un
pato, entonces es un pato»: a esto se le llama *duck typing* («tipado de pato»).

```ruby
class Pato
  def hablar = "¡Cuac!"
end

class Robot
  def hablar = "Bip."
end

[Pato.new, Robot.new, "texto"].each do |cosa|
  if cosa.respond_to?(:hablar)
    puts cosa.hablar
  else
    puts "#{cosa.inspect} no sabe hablar"
  end
end
```

```sortie
¡Cuac!
Bip.
"texto" no sabe hablar
```

`Pato` y `Robot` no tienen ningún padre común (aparte de `Object`). `respond_to?`
pregunta si el objeto conoce un método.

## private y protected

- Un método `private` solo se puede llamar **desde dentro** del objeto, sobre sí mismo.
- Un método `protected` también se puede llamar sobre **otro objeto de la misma
  familia** de clases. Es útil para comparar dos objetos sin enseñarlo todo al
  exterior.

```ruby
class Cuenta
  def initialize(saldo)
    @saldo = saldo
  end

  def mas_rica_que?(otra)
    saldo > otra.saldo
  end

  protected

  def saldo
    @saldo
  end
end

a = Cuenta.new(500)
b = Cuenta.new(200)
puts a.mas_rica_que?(b)

begin
  a.saldo
rescue NoMethodError => e
  puts "Prohibido: #{e.message}"
end
```

```sortie
true
Prohibido: protected method `saldo' called for an instance of Cuenta
```

Con `private`, `otra.saldo` habría sido rechazado, incluso dentro de la clase.

## Preferir la composición

La herencia dice «**es un**»: un `Perro` es un `Animal`. Si la frase suena mal, no uses
la herencia. Un `Coche` no es un `Motor`: **tiene un** motor. Entonces se guarda el
objeto en una variable: es la **composición**.

```ruby
class Motor
  def initialize(potencia)
    @potencia = potencia
  end

  def arrancar = "Motor de #{@potencia} CV encendido"
end

class Coche
  def initialize(motor)
    @motor = motor
  end

  def arrancar = "#{@motor.arrancar}, el coche avanza."
end

puts Coche.new(Motor.new(90)).arrancar
puts Coche.new(Motor.new(150)).arrancar
```

```sortie
Motor de 90 CV encendido, el coche avanza.
Motor de 150 CV encendido, el coche avanza.
```

Puedes cambiar de motor sin cambiar de clase. Para compartir métodos entre clases sin
parentesco, piensa también en los [módulos](modules.md).

> **Recuerda:** usa la herencia para una verdadera relación «es un», en uno o dos
> niveles. Para lo demás, la composición y los módulos son más flexibles.

## Errores frecuentes

### Olvidar llamar a super en initialize

```ruby norun
class Animal
  def initialize(nombre)
    @nombre = nombre
  end

  def nombre = @nombre
end

class Gato < Animal
  def initialize(nombre, color)
    @color = color
  end
end

p Gato.new("Félix", "pelirrojo").nombre
```

```text
nil
```

No hay error, pero `@nombre` nunca se ha rellenado: no se ha llamado al `initialize` del
padre. Añade `super(nombre)` al principio.

### Escribir super sin paréntesis con otros argumentos

```ruby norun
class Animal
  def initialize(nombre)
    @nombre = nombre
  end
end

class Gato < Animal
  def initialize(nombre, color)
    super
    @color = color
  end
end

Gato.new("Félix", "pelirrojo")
```

```text
main.rb:2:in `initialize': wrong number of arguments (given 2, expected 1) (ArgumentError)
	from main.rb:9:in `initialize'
	from main.rb:14:in `new'
	from main.rb:14:in `<main>'
```

`super` solo pasa **todos** los argumentos recibidos (aquí 2). Escribe `super(nombre)`.

### Llamar a un método privado desde fuera

```ruby norun
class Caja
  private

  def codigo = 1234
end

Caja.new.codigo
```

```text
main.rb:7:in `<main>': private method `codigo' called for an instance of Caja (NoMethodError)

Caja.new.codigo
        ^^^^^^^
```

Un método `private` sirve para el funcionamiento interno. Si el exterior lo necesita,
ponlo antes de `private`.

## Ponte a prueba

**Pregunta 1:** ¿qué devuelve `Integer.ancestors.include?(Comparable)`?

> **Respuesta**
>
> `true`: `Integer` incluye el módulo `Comparable`, que forma parte de sus ancestros.
>
> ```ruby
> puts Integer.ancestors.include?(Comparable)
> ```
>
> ```sortie
> true
> ```

**Pregunta 2:** en un método redefinido `hablar`, ¿cómo reutilizar el texto del padre y
añadirle «!»?

> **Respuesta**
>
> Con `super + "!"`: `super` llama a `hablar` del padre y devuelve su resultado.

**Pregunta 3:** una clase `Biblioteca` contiene libros. ¿Debe heredar de `Array`?

> **Respuesta**
>
> No: una biblioteca **tiene** libros, no es un array. Guarda un array en una variable
> `@libros` (composición).

## En resumen

- `class Hijo < Padre` hereda todos los métodos del padre.
- Una subclase puede redefinir un método; `super` llama a la versión del padre.
- `ancestors` muestra el orden en que Ruby busca un método.
- Polimorfismo: objetos distintos responden al mismo mensaje; el duck typing solo mira
  `respond_to?`.
- `private`: solo uno mismo; `protected`: también los objetos de la misma familia.
- «Es un» → herencia; «tiene un» → composición.

Siguiente: [módulos y mixins](modules.md)
