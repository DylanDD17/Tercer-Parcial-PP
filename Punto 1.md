# Diagrama general del diseño 

<img width="1615" height="918" alt="Mermaid Chart - Create complex, visual diagrams with text -2025-11-21-125034" src="https://github.com/user-attachments/assets/5cb361ff-9762-4679-8fea-c3a2e9dbb64e" />

---

# Diseño: Regresión Lineal concurrente usando concurrencia + cálculo π 

---

## Resumen 
Dividimos los datos en *trozos* que se procesan en paralelo por **Workers** que calculan gradientes parciales; un **Agregador** (o *Parameter Server*) recibe esos gradientes por canales, los combina y actualiza los parámetros `w` y `b`; luego notifica a los Workers para la siguiente iteración. Todo esto está modelado como procesos que se comunican por nombres/canales (concepto clave del π-calculus).

---

## Componentes principales 
1. **Data Loader**: lee el dataset y lo fragmenta en *mini-batches* o trozos.
2. **Workers** (N réplicas): reciben un trozo de datos y la versión actual de los parámetros; calculan `dw` y `db` parciales y los envían al Agregador.
3. **Agregador / Parameter Server**: recibe gradientes parciales de todos los Workers, los combina (suma/promedio) y actualiza `w` y `b` usando la tasa de aprendizaje; publica los nuevos parámetros.
4. **Coordinador de Épocas**: controla el avance por época/iteración y decide cuándo terminar.
5. **Canales de Comunicación**: colas o canales para enviar datos, gradientes y parámetros entre componentes.

---

## Conceptos de concurrencia utilizados
- **Procesos/Workers concurrentes**: unidades que ejecutan en paralelo (hilos/procesos/actores).
- **Pasaje de mensajes**: los procesos no comparten memoria para parámetros; se comunican mediante mensajes (canales/colas). Evita condiciones de carrera si el acceso está bien definido.
- **Sincronización**: esperar a que todos los Workers envíen sus gradientes antes de actualizar (sincronización por iteración).

---

## Conceptos de π-calculus usados 
- **Nombres / canales**: en π-calculus los procesos hablan por nombres. Aquí un nombre ≈ un canal/cola (por ejemplo `chan_grad`, `chan_param`).
- **Output/Input**: `send` y `recv`. Un proceso `P` puede enviar `send(chan, msg)` y otro recibir `recv(chan, var)`.
- **Composición paralela**: `P | Q` significa `P` y `Q` corren en paralelo → equivalente a lanzar varios Workers.
- **Restricción (new n)**: crear un canal privado para una estructura particular (ej. un canal por iteración).
- **Replicación (!P)**: modelo para crear muchos Workers idénticos.

---
<img width="1256" height="1606" alt="image" src="https://github.com/user-attachments/assets/6a649835-eda5-4f6e-b181-e6d0e1667db8" />

## Flujo de datos y control 
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

## Estrategias de sincronización 
1. **Sincronización por iteración (síncrona)**
   - Esperar a N gradientes antes de actualizar.
2. **Asíncrono**
   - Actualizar parámetros cuando llegue cualquier gradiente.
3. **Barrera parcial**
   - Permite cierta diferencia (p.e. esperar K < N gradientes), balance entre velocidad y estabilidad.

---

## Manejo de fallos y terminación
- Si un Worker falla, el Coordinador puede reasignar su chunk a otro Worker.
- Si el Agregador falla, usar replicación o guardar checkpoints periódicos de `w,b`.
- Señal de terminación: enviar un mensaje `STOP` por `chan_param`.
