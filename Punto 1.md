# Diagrama general del diseño (UML simplificado)

```
          +------------------+                     
          |   Coordinator    |                     
          +------------------+                     
             |        |                          
     start/params   stop                         
             |        |                          
             v        v                          
+------------------+     +------------------+     
|    DataLoader    |     |   Aggregator     |<-------------------+
+------------------+     +------------------+                    |
        |                         ^                              |
   chunks/task                   | gradients                     |
        |                         |                              |
        v                         |                              |
   +-------------------------------------------+                 |
   |                 Workers                   |                 |
   |-------------------------------------------|                 |
   |  Worker 1 | Worker 2 | ... | Worker N     |-----------------+
   +-------------------------------------------+
            ^          ^             ^
            | params   | params      | params
            +----------+-------------+
```

---

# Diseño: Regresión Lineal concurrente usando concurrencia + cálculo π (π-calculus)
 Regresión Lineal concurrente usando concurrencia + cálculo π (π-calculus)

**Propósito**: diseñar una solución para entrenar una regresión lineal (descenso de gradiente) usando ideas del paradigma de **concurrencia** y conceptos esenciales del **cálculo π (pi-calculus)**. El diseño es conceptual — no es código— y está pensado para que puedas implementarlo luego con hilos, procesos o un modelo de actores.

---

## Resumen ejecutivo (una línea)
Dividimos los datos en *trozos* que se procesan en paralelo por **Workers** que calculan gradientes parciales; un **Agregador** (o *Parameter Server*) recibe esos gradientes por canales, los combina y actualiza los parámetros `w` y `b`; luego notifica a los Workers para la siguiente iteración. Todo esto está modelado como procesos que se comunican por nombres/canales (concepto clave del π-calculus).

---

## Objetivos del diseño
- Mantener la corrección del descenso de gradiente.
- Aprovechar la concurrencia para acelerar el cómputo de gradientes.
- Usar conceptos clave de π-calculus: **procesos**, **canales (nombres)**, **envío/recepción** y **composición**.
- Evitar conceptos demasiado teóricos: usar una explicación intuitiva y mapeos a primitivas prácticas (threads, colas, canales).

---

## Componentes principales (visión de alto nivel)
1. **Data Loader**: lee el dataset y lo fragmenta en *mini-batches* o trozos.
2. **Workers** (N réplicas): reciben un trozo de datos y la versión actual de los parámetros; calculan `dw` y `db` parciales y los envían al Agregador.
3. **Agregador / Parameter Server**: recibe gradientes parciales de todos los Workers, los combina (suma/promedio) y actualiza `w` y `b` usando la tasa de aprendizaje; publica los nuevos parámetros.
4. **Coordinador de Épocas**: controla el avance por época/iteración y decide cuándo terminar.
5. **Canales de Comunicación**: colas o canales para enviar datos, gradientes y parámetros entre componentes.

---

## Conceptos de concurrencia utilizados (explicación simple)
- **Procesos/Workers concurrentes**: unidades que ejecutan en paralelo (hilos/procesos/actores).
- **Pasaje de mensajes**: los procesos no comparten memoria para parámetros; se comunican mediante mensajes (canales/colas). Evita condiciones de carrera si el acceso está bien definido.
- **Sincronización por barrera**: esperar a que todos los Workers envíen sus gradientes antes de actualizar (sincronización por iteración).
- **Modelo asíncrono (opcional)**: los Workers envían gradientes y el Agregador actualiza cuando llegan (más rápido pero puede introducir estancamiento/diferencia de parámetros — *staleness*).

---

## Conceptos de π-calculus usados (explicación no formal)
- **Nombres / canales**: en π-calculus los procesos hablan por nombres. Aquí un nombre ≈ un canal/cola (por ejemplo `chan_grad`, `chan_param`).
- **Output/Input**: `send` y `recv`. Un proceso `P` puede enviar `send(chan, msg)` y otro recibir `recv(chan, var)`.
- **Composición paralela**: `P | Q` significa `P` y `Q` corren en paralelo → equivalente a lanzar varios Workers.
- **Restricción (new n)**: crear un canal privado para una estructura particular (ej. un canal por iteración).
- **Replicación (!P)**: modelo para crear muchos Workers idénticos.

> Mapeo práctico: piensa en cada nombre como una cola de mensajes (ej. `gradients`, `params`) y en los procesos como hilos que hacen `put/get`.

---

## Flujo de datos y control (explicación paso a paso)
1. **Inicialización**: `w=0`, `b=0`. El Data Loader divide `X,y` en `K` trozos. Se crean `N` Workers.
2. **Inicio de época**: el Coordinador publica el canal de parámetros `chan_param` y envía `w,b` a cada Worker.
3. **Cómputo local** (en cada Worker): recibe su trozo y `w,b`; calcula predicciones locales `y_pred = w*x + b` y el `error = y_pred - y`. Calcula gradientes parciales:
   - `dw_local = (2/m_local) * sum(error * x)`
   - `db_local = (2/m_local) * sum(error)`
   Envía `(dw_local, db_local)` al `chan_grad`.
4. **Agregación**: el Agregador recibe los `N` mensajes de `chan_grad`, los suma/normaliza (`dw = mean(dw_local)` y `db = mean(db_local)`), actualiza `w` y `b`:
   - `w -= lr * dw`
   - `b -= lr * db`
   Luego publica los nuevos `w,b` en `chan_param` o notifica al Coordinador.
5. **Repetir**: repetir por cada época hasta `epochs` o hasta convergencia.
6. **Terminación**: Coordinador cierra canales y notifica a Workers para que terminen.

---

## Modelado ligero en π-calculus (intuitivo, no formal)
- `Loader = !readChunk(c) . send(chan_task, c)`  — lee y envía chunks.
- `Worker_i = recv(chan_param, (w,b)) . recv(chan_task, chunk) . computeGrad(chunk,w,b, g) . send(chan_grad, g) . Worker_i` (replicado por iteración)
- `Aggregator = recv(chan_grad, g1) . recv(chan_grad, g2) . ... . update(w,b) . send(chan_param, (w,b)) . Aggregator`
- Sistema completo: `Loader | Worker_1 | ... | Worker_N | Aggregator | Coordinator`

---

## Diagramas UML (texto que puede transformarse a un diagrama visual)

### Diagrama de Componentes (alto nivel)
```
[DataLoader] --> (TaskQueue)
(Workers) --|> [Worker1]
          --|> [Worker2]
(TaskQueue) --> (Worker)
(Worker) --> (GradQueue)
(Aggregator) --> (ParamStore)
(Coordinator) --> (Aggregator)
(Coordinator) --> (Workers)
```
**Leyenda**: `-->` flujo de mensajes/colas. `TaskQueue` = canal de tareas; `GradQueue` = canal de gradientes; `ParamStore` = canal/almacén de parámetros.

### Diagrama de Secuencia (una iteración simplificada)
```
Coordinator -> DataLoader: requestChunks()
DataLoader -> TaskQueue: push(chunk_i)
Coordinator -> Worker_i: send(params)
Worker_i -> TaskQueue: pop() -> chunk
Worker_i -> Aggregator: send(dw_i, db_i)
Aggregator -> Aggregator: sum/avg
Aggregator -> Coordinator: updated_params
Coordinator -> Worker*: broadcast(updated_params)
```

---

## Estrategias de sincronización (ventajas/desventajas simples)
1. **Sincronización por iteración (síncrona)**
   - *Cómo*: esperar a N gradientes antes de actualizar.
   - *Ventaja*: comportamiento similar al descenso de gradiente por batch completo (estable y determinista).
   - *Desventaja*: lento si un Worker es lento (efecto *straggler*).
2. **Asíncrono (hogwild / parameter server)**
   - *Cómo*: actualizar parámetros cuando llegue cualquier gradiente.
   - *Ventaja*: mayor throughput, menor espera.
   - *Desventaja*: las actualizaciones usan parámetros con distinto estatus (staleness). Puede funcionar bien en problemas convexo simples.
3. **Barrera parcial (bounded staleness)**
   - *Cómo*: permite cierta diferencia (p.e. esperar K < N gradientes), balance entre velocidad y estabilidad.

---

## Manejo de fallos y terminación
- Si un Worker falla, el Coordinador puede reasignar su chunk a otro Worker.
- Si el Agregador falla, usar replicación o guardar checkpoints periódicos de `w,b`.
- Señal de terminación: enviar un mensaje `STOP` por `chan_param`.

---

## Medidas de rendimiento y complejidad
- **Coste computacional** por época: O(m) cálculo total (m = tamaño total dataset). Paralelizando en N Workers, idealmente tiempo ≈ O(m/N) + overhead de comunicación.
- **Overhead de comunicación**: mensajes por iteración = N (un gradient por Worker) + N (recepción parámetros) -> importante diseñar tamaño de mensajes pequeño.

---

## Mapping práctico (cómo implementarlo sin entrar en cosas avanzadas)
- **Canales**: colas `Queue` de Python (`multiprocessing.Queue`) o canales en Go.
- **Workers**: `Thread` o `Process` que ejecutan un bucle: recibir parámetros -> procesar su trozo -> enviar gradiente.
- **Agregador**: hilo que hace `get()` N veces desde la cola de gradientes, actualiza, y `put()` en cola de parámetros.
- **Coordinador**: puede ser el proceso principal que lanza Workers, inicializa parámetros y controla las épocas.

---

## Ejemplo mapeado al código original (guía rápida)
- La línea `y_pred = w * X + b` y cálculo de `dw, db` se mueve dentro del `Worker` pero usando solo su `X_chunk` e `y_chunk`.
- En lugar de `for epoch in range(epochs):` con bucle único, cada época corresponde a: broadcast parámetros -> Workers calculan -> agregador actualiza -> siguiente época.

---

## Recomendaciones finales (prácticas)
- Empieza con **sincronización síncrona** y pocos Workers (p.e. 2–4) para validar lógica.
- Mide tiempo por época y tamaño de mensajes; si la comunicación domina, considera aumentar el tamaño de chunk (menos mensajes).
- Si enseñas esto o lo implementas, representarlo con las expresiones simples de π-calculus que incluí arriba ayuda a razonar sobre el flujo de mensajes.

---

Si quieres, convierto este diseño en un diagrama UML gráfico (PNG/SVG) o en pseudocódigo para cada componente (DataLoader, Worker, Aggregator, Coordinator). Indícame si prefieres que use hilos (threading), procesos (multiprocessing) o canales estilo Go para el ejemplo de implementación.

