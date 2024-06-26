from typing import Callable

from rstypes import Option
from rstypes import Result


func: Callable[[int], int] = lambda x: x + 1
a = Option(1)
d = Option((1, 2))
b = a.map(func)
c = b.unwrap()
e = a.zip(d)
f = Option(a)
g = Result.Ok(1)
h = g.is_ok()
i = Result.Err("1")
j = i.is_ok()
k = g.map(func)
l = i.map(func)
a.inspect(lambda x: print(x))
m = Result.Ok(1)
n = m.transpose()
print(b)
print(d)
print(e)
print(f)
print(g)
print(h)
print(j)
# print(k)
print(l)
print(m)
print(n)