from typing import Callable
from rstypes import Option, Result


func: Callable[[int], int] = lambda x: x + 1
a = Option(1)
d = Option((1, 2))
b = a.map(func)
c = b.unwrap()
e = a.zip(d)
f = Option(a)
g = Result(1, "Ok")
print(b)
print(d)
print(e)
print(f)
print(g)