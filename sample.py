from typing import Callable
from rstypes import Option

func: Callable[[int], int] = lambda x: x + 1
a = Option(1)
d = Option(2)
b = a.map(func)
c = b.unwrap()
e = a.zip(d)
print(d.unwrap())
print(e.unwrap())