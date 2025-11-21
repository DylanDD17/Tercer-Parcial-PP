# Diseño con el Paradigma de Programación Orientada a Aspectos (AOP) para Regresión Lineal

Este documento presenta el diseño detallado de una solución de **regresión lineal** utilizando el **paradigma de Aspectos**, centrándose solo en el diseño y utilizando conceptos básicos del paradigma.

---

## 1. Objetivo del diseño
Diseñar una arquitectura donde el algoritmo de regresión lineal mantenga su lógica central limpia y directa, mientras que las tareas transversales (logging, métricas, validación, monitoreo, etc.) se implementan mediante aspectos, sin contaminar el código principal.

---

## 2. Componentes principales del diseño

### ### 2.1 Módulo `CoreRegression`
Encargado exclusivamente de la **lógica del entrenamiento**:
- Inicializar parámetros `w` y `b`.
- Calcular predicciones.
- Calcular gradientes.
- Actualizar parámetros.
- Ejecutar el ciclo de entrenamiento.

**Propósito:** Mantener la lógica de regresión simple y sin código repetido o transversal.

---

### 2.2 Aspecto `LoggingAspect`
Aspecto encargado de registrar eventos importantes:
- Inicio y fin de cada época.
- Valores de `w`, `b` y del error.
- Notificar si se alcanza convergencia.

Se aplica en puntos del programa como:
- Antes y después del método `train()`.
- Antes del cálculo de gradientes.

---

### 2.3 Aspecto `ValidationAspect`
Valida entradas del usuario o del sistema:
- Verificar que `X` y `y` tienen el mismo tamaño.
- Verificar que los valores son numéricos.
- Validar que el learning rate > 0.

Este aspecto se activa en los puntos previos a:
- Inicialización de datos.
- Inicio del entrenamiento.

---

### 2.4 Aspecto `PerformanceAspect`
Mide rendimiento:
- Tiempo por época.
- Costo acumulado.
- Número de iteraciones.

Se aplica alrededor de:
- La ejecución de cada época.

---

### 2.5 Aspecto `MonitoringAspect`
Monitorea la progresión del modelo:
- Detecta si el error aumenta entre épocas.
- Detecta si el modelo ya convergió.

Actúa:
- Después del cálculo de MSE.
- Antes de actualizar parámetros.

---

## 3. Flujo general del diseño

1. **ValidationAspect** revisa los datos antes de iniciar.
2. El método `train()` de `CoreRegression` arranca.
3. **LoggingAspect** registra el inicio.
4. Por cada época:
   - **PerformanceAspect** inicia el cronómetro.
   - `CoreRegression` calcula predicciones.
   - Se calculan gradientes.
   - **LoggingAspect** registra gradientes.
   - **MonitoringAspect** revisa si la pérdida subió o converge.
   - Se actualizan los parámetros.
   - **PerformanceAspect** registra tiempo total.
   - **LoggingAspect** documenta el cierre de la época.
5. Cuando termina:
   - **LoggingAspect** registra parámetros finales.
   - **MonitoringAspect** guarda si el modelo convergió.
   - El sistema finaliza.

---

## 4. Ventajas del diseño
- El módulo de regresión queda **simple y limpio**.
- Los aspectos permiten:
  - Añadir nueva funcionalidad sin modificar el core.
  - Activar o desactivar funcionalidades transversales.
  - Mejor lectura del código.
  - Separar responsabilidades.

---

## 5. Posibles extensiones
- Aspecto `VisualizationAspect` para graficar la curva de error.
- Aspecto `DebugAspect` para revisar valores internos específicos.
- Aspecto `SecurityAspect` si se integran datos externos.

---

Si desea, puedo generar también:
- Diagrama UML de este diseño.
- Diagrama en PlantUML.
- Diagrama de secuencia.

Indique qué desea añadir.


## Diagrama UML (PlantUML)
```plantuml
@startuml

package Core {
  class CoreRegression {
    - w : float
    - b : float
    - learning_rate : float
    - epochs : int
    + train(X, y)
    + predict(x)
    + compute_gradients(X, y)
  }
}

package Aspects {
  aspect LoggingAspect {
    + beforeTrain()
    + afterTrain()
    + beforeEpoch()
    + afterEpoch()
  }

  aspect ValidationAspect {
    + validateData(X, y)
    + validateHyperparams(lr, epochs)
  }

  aspect PerformanceAspect {
    + startTimer()
    + endTimer()
  }

  aspect MonitoringAspect {
    + checkConvergence(error)
    + detectErrorIncrease(prev, current)
  }
}

CoreRegression <.. LoggingAspect : advises
CoreRegression <.. ValidationAspect : validates
CoreRegression <.. PerformanceAspect : measures
CoreRegression <.. MonitoringAspect : monitors

@enduml
```

