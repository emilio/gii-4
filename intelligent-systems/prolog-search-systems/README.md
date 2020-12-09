# Sistemas de búsqueda

## Requerimientos

Para ejecutar los predicados de las prácticas automáticamente, es necesario
tener `make` instalado. Pero se pueden ejecutar los comandos del `Makefile`
a mano alternativamente.

Para la primera parte, es necesario `gprolog`, que se puede instalar desde el
gestor de paquetes, o descargar desde http://gprolog.org/#download

Para la segunda parte, es necesario una versión moderna de Rust / Cargo. Se
puede obtener en https://rustup.rs/.

## Parte 1 - Solución con backtrack (prolog)

La entrada se ha separado a un fichero extra (`contacto.pdb`) para poder
reutilizarse en la parte 2.

La parte 1 está realizada con `gprolog`, y se ubica en el fichero
`practica.pdb`.

El predicado principal de la primera parte es `principal`, que utiliza
`rutaIdeologica`. El código es relativamente sencillo y he intentado comentar
las partes más oscuras...

El predicado usando BFS (para hallar una longitud de propagación mínima) es
`principal2`. Este enunciado usa `rutaIdeologicaBFS`
/ `rutaIdeologicaBFSInternal`, que son algo más complejas pero tampoco muy
difíciles.

Básicamente, la función consume el primer elemento de la lista, si no hemos
llegado, luego buscamos todos los nodos que siguen al actual, poniéndolos en
`FOLLOWER_NODES`, los añadimos al final de la lista de nodos a visitar
(`append(REST, FOLLOWERS, NEW_QUEUE)`), y recurre otra vez con la lista nueva,
se detiene cuando encontramos el nombre a convencer en la lista.

## Parte 2 - Exploración de grafos

Esta parte se ha realizado en Rust y se encuentra en el directorio `part-2`. He
intentado que el código sea lo más claro posible.

Obtenemos el grafo via línea de comandos. El código que lee el fichero y lo
vuelve una estructura de datos adecuada está en `input.rs`. Las estructuras
principales que podemos ver ahí son:

 * `Input`: Representa todo el grafo, está compuesto de `nodes` (`NodeStore`)
   y `contacts` (`Contacts`).
 * `NodeId`: Representa una referencia a un nodo.
 * `NodeStore`: Representa los nodos del grafo. Los nodos se representan como
   índices por simplicidad, y este mapa almacena el mapa de índice a nombre
   (`node_names`) y almacena el mapa de nombre a id también (`nodes_by_name`).
 * `Contacts`: Representa los edges del grafo (las conexiones entre los nodos).
   Es efectivamente un mapa de nodo a `Contact`.
 * `Contact`: Representa un edge del grafo, y tiene los dos nodos que conecta,
   nuestro valor de `k`, y opcionalmente una distancia, que se usa como
   heurística para encontrar el camino más corto.

El código en sí para hallar la solución se encuentra en `solver.rs`. Permitimos
usar opcionalmente una heurística de distancia, o alternativamente usamos un
coste fijo (`FIXED_COST`).

El código es relativamente sencillo:

 * Mantenemos un min-heap con todos los nodos por visitar, ordenados por coste
   (`open: BinaryHeap<cmp::Reverse<State>>`).
 * Mantenemos un mapa de nodo a mejor camino hasta ahora
   (`came_from: NodeMap<CameFrom>`).

La mayor diferencia con respecto a los algoritmos discutidos en clase es que en
vez de eliminar caminos peores en `open`, simplemente dejamos que sigan en el
min-heap, y evitamos visitarlos (si llegamos a ellos) usando `came_from`:

```
if let Some(came_from) = self.came_from.get(edge.to) {
    if came_from.cost <= cost {
        continue;
    }
}
```

Se puede trazar la ejecución del algoritmo con la variable de entorno `DEBUG`.
Por ejemplo:

```
$ DEBUG=1 make run-part-2
cd part-2 && cargo run -- ../contacto.pdb vidal luis 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/a-star-or-something-similar ../contacto.pdb vidal luis 1`
Edges from pepe (NodeId(0))
  -> juan (NodeId(1)) with k=0.3, distance=None
  -> vidal (NodeId(8)) with k=1, distance=None
  -> luis (NodeId(3)) with k=0.5, distance=None
Edges from juan (NodeId(1))
  -> pepe (NodeId(0)) with k=0.3, distance=None
  -> manolo (NodeId(2)) with k=0.6, distance=None
Edges from manolo (NodeId(2))
  -> juan (NodeId(1)) with k=0.6, distance=None
  -> luis (NodeId(3)) with k=4, distance=None
  -> felipe (NodeId(7)) with k=2.5, distance=None
Edges from luis (NodeId(3))
  -> manolo (NodeId(2)) with k=4, distance=None
  -> felipe (NodeId(7)) with k=2, distance=None
  -> pepe (NodeId(0)) with k=0.5, distance=None
Edges from josemari (NodeId(4))
  -> lucia (NodeId(5)) with k=4, distance=None
  -> raul (NodeId(6)) with k=3, distance=None
Edges from lucia (NodeId(5))
  -> josemari (NodeId(4)) with k=4, distance=None
Edges from raul (NodeId(6))
  -> josemari (NodeId(4)) with k=3, distance=None
Edges from felipe (NodeId(7))
  -> luis (NodeId(3)) with k=2, distance=None
  -> manolo (NodeId(2)) with k=2.5, distance=None
Edges from vidal (NodeId(8))
  -> pepe (NodeId(0)) with k=1, distance=None
State { node: NodeId(8), cost: 0.0, k_so_far: 0.5 }
 > trying edge Contact { from: NodeId(8), to: NodeId(0), k: 1.0, distance: None }
State { node: NodeId(0), cost: 1.0, k_so_far: 0.5 }
 > trying edge Contact { from: NodeId(0), to: NodeId(1), k: 0.3, distance: None }
 > trying edge Contact { from: NodeId(0), to: NodeId(8), k: 1.0, distance: None }
 > trying edge Contact { from: NodeId(0), to: NodeId(3), k: 0.5, distance: None }
State { node: NodeId(1), cost: 2.0, k_so_far: 0.15 }
 > trying edge Contact { from: NodeId(1), to: NodeId(0), k: 0.3, distance: None }
 >> not acceptable
 > trying edge Contact { from: NodeId(1), to: NodeId(2), k: 0.6, distance: None }
 >> not acceptable
State { node: NodeId(3), cost: 2.0, k_so_far: 0.25 }
0 - vidal (cost = 0) (k = 0.5)
1 - pepe (cost = 1) (k = 0.5)
2 - luis (cost = 2) (k = 0.25)
```

## Uso de información adicional.

Se ha usado una versión mínimamente modificada del problema
(`contacto-con-distancia.pdb`), que incluye:

 * Un valor de `k` mínimamente mayor para el contacto entre `juan` y `manolo`,
   para poder encontrar varios caminos viables.

 * Una distancia como cuarto argumento que se usa como coste. Se ha dado una
   distancia grande entre `pepe` y `luis`, para demostrar cómo el algoritmo
   funciona y consigue encontrar el camino adecuado.
