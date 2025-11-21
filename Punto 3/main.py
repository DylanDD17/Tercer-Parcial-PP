import time

def linear_regression(xs, ys):
    n = len(xs)

    sum_x = sum(xs)
    sum_y = sum(ys)
    sum_xy = sum(x*y for x,y in zip(xs, ys))
    sum_x2 = sum(x*x for x in xs)

    m = (n*sum_xy - sum_x*sum_y) / (n*sum_x2 - sum_x**2)
    b = (sum_y - m*sum_x) / n

    return m, b

# Datos grandes
n = 2_000_000
xs = [float(i) for i in range(n)]
ys = [3.2 * x + 5.0 for x in xs]

start = time.time()
m, b = linear_regression(xs, ys)
duration = time.time() - start

print("m =", m)
print("b =", b)
print("Tiempo de ejecución en Python:", duration, "segundos")
