Synonym Search
==============

Build: `cargo b`

Para correr el programa se puede elegir entre dos modos:
* actors
* threads

Tambien se le tiene que indicar el nombre del archivo que contiene la lista de palabras a las que se quiere buscar sinonimos (una palabra por linea).

Exec: `cargo r <mode> <filename> <max_concurrency> <min_time_request_sec>`

Ver logs en tiempo real (a medida que se van escribiendo) para observar los sleeps().

En una consola levantar el programa en alguno de los modos.

```
cargo r <mode> <filename> <max_concurrency> <min_time_request_sec>
```

En otra consola ejecutar el siguiente comando: `tail -f log.txt`. Esto va a provocar que tail se quede escuchando los movimientos del archivo, por lo que comenzara a verse en tiempo real los loggings que se vayan haciendo.
