# TP1: Internet of Coffee

### Correr el programa

```
cargo run <archivo>
```

donde `<archivo>` es la ruta de un archivo de texto que contiene los pedidos a realizar, con el formato que se especifica en la sección [Pedidos](#pedidos).

Por ejemplo:

```
cargo run pedidos.txt
```

### Generar documentación

```
cargo doc --document-private-items --open
```

### Pedidos

Los pedidos a realizar se representan con un id y las cantidades de agua, café y espuma a utilizar.
Se simula su llegada mediante la lectura de un archivo de texto, donde cada línea representa un pedido siguiendo el siguiente formato:

```
<id>,<agua>,<cafe>,<espuma>
```

donde `<agua>`, `<cafe>` y `<espuma>` son un número natural entre MIN_CANTIDAD y MAX_CANTIDAD incluídos, e `<id>` es un número natural que lo representa.

Por ejemplo:
```
1,8,5,2
```

Se simula la llegada de clientes mediante un sleep entre cada uno de estos pedidos.

En caso de error al procesar un pedido del archivo, se imprimirá una advertencia y se continuará intentando leer pedidos del archivo.

### Cafetera

La cafetera consta de un contenedor de café molido, un contenedor de granos de café, un contenedor de espuma y un contenedor de leche, además de N dispensadores. Los dispensadores de café molido y espuma están inicialmente vacíos, mientras que los contenedores de granos y leche inicialmente contienen la totalidad de su capacidad.

Al llegar un nuevo pedido, este esperará por un dispensador libre y en cuanto lo consiga, este se marcará como en uso para que no pueda ser utilizado por más de un pedido a la vez y se comenzará a preparar el pedido en un nuevo hilo. Una vez en el dispensador, se servirán las cantidades de agua, café y espuma correspondientes, en ese orden. Esto se simulará mediante sleeps durante una cantidad de tiempo relativa a la cantidad de producto. Solo un dispensador a la vez podrá servir café, y solo un dispensador a la vez podrá servir espuma.

Además, en dos hilos separados se realizarán reposiciones de café molido y espuma respectivamente. Esto ocurrirá cuando la cantidad restante en el contenedor sea menor a la cantidad máxima de producto de un pedido y el dispensador correspondiente se encuentre disponible. Este proceso se simulará mediante un sleep de un tiempo constante, y durante este período no se podrá utilizar el dispensador correspondiente. Una unidad de granos de café generará una unidad de café molido, y una unidad de leche generará una unidad de espuma.

Al finalizar ese proceso, en caso de que la cantidad en los contenedores de granos de café o leche sea menor a la capacidad de los contenedores de café molido o espuma respectivamente, se alertará por consola de la cantidad restante y se repondrán a su capacidad completa. Este proceso es instantáneo.

La cantidad de producto restante para realizar la reposición fue elegida como la mínima posible que asegura que se podrán seguir procesando pedidos.

### Estadísticas

Periódicamente se imprimirán las estadísticas de la cafetera con el tag [INFO], incluyendo la cantidad actual de cada uno de los contenedores, la cantidad total utilizada de cada uno de los productos y la cantidad de pedidos finalizados.

También si imprimirá con el tag [INFO] cada pedido que se completa.
