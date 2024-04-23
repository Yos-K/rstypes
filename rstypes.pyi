from typing import Callable, TypeVar, Generic


T = TypeVar('T')
U = TypeVar('U')


class Option(Generic[T]):
    def __init__(obj: T) -> None: ...
    def is_some(self) -> bool: ...
    def is_some_and(self, f: Callable[[T], bool]) -> bool: ...
    def is_none(self) -> bool: ...
    def expect(self, msg: str) -> T: ...
    def unwrap(self) -> T: ...
    def unwrap_or(self, default: T) -> T: ...
    def unwrap_or_else(self, f: Callable[[], T]) -> T: ...
    def map(self, f: Callable[[T], U]) -> Option[U]: ...
    def inspect(self, f: Callable[[T],]) -> Option: ...
    def map_or(self, default: U, f: Callable[[T], U]) -> U: ...
    def map_or_else(self, default: Callable[[], U], f: Callable[[T], U]) -> U: ...