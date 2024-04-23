from typing import Callable
from rstypes import Option

func: Callable[[int], int] = lambda x: x + 1
a = Option(1)
b = a.map(func)
c = b.unwrap()
print(c)