from typing import Callable, TypeVar, Generic
from option import Option


T = TypeVar('T')
U = TypeVar('U')
E = TypeVar('E')


class Result:
    class Ok(Generic[T]):
        def __init__(self, value: T) -> None: ...
        def is_ok(self) -> bool: ...
        def is_ok_and(self, f: Callable[[T], bool]) -> bool: ...
        def is_err(self) -> bool: ...
        def is_err_and(self, f: Callable[[T], bool]) -> bool: ...
        def ok(self) -> Option[T]: ...
        def err(self) -> Option[E]: ...
        def map(self, f: Callable[[T], U]) -> Result.Ok[U]: ...
        def map_or(self, default: U, f: Callable[[T], U]) -> U: ...
        def map_or_else(self, default: Callable[[], U], f: Callable[[T], U]) -> U: ...
        def inspect(self, f: Callable[[T],]) -> Result: ...
        def inspect_err(self, f: Callable[[T],]) -> Result: ...
        def expect(self, msg: str) -> T: ...
        def unwrap(self) -> T: ...
        def expect_err(self, msg: str) -> T: ...
        def unwrap_err(self) -> T: ...
        def and_then(self, op: Callable[[T], Result]) -> Result: ...
        def or_else(self, op: Callable[[T], Result]) -> Result: ...
        def unwrap_or(self, default: T) -> T: ...
        def unwrap_or_else(self, op: Callable[[E], T]) -> T: ...
        def transpose(self) -> Option[Result.Ok[T]]: ...
        def flatten(self) -> Result.Ok[T]: ...

    class Err(Generic[T]):
        def __init__(self, value: T) -> None: ...
        def is_ok(self) -> bool: ...
        def is_ok_and(self, f: Callable[[T], bool]) -> bool: ...
        def is_err(self) -> bool: ...
        def is_err_and(self, f: Callable[[T], bool]) -> bool: ...
        def ok(self) -> Option[T]: ...
        def err(self) -> Option[E]: ...
        def map(self, f: Callable[[T], U]) -> Result.Err[U]: ...
        def map_or(self, default: U, f: Callable[[T], U]) -> U: ...
        def map_or_else(self, default: Callable[[], U], f: Callable[[T], U]) -> U: ...
        def map_err(self, op: Callable[[T], U]) -> Result: ...
        def inspect(self, f: Callable[[T],]) -> Result: ...
        def inspect_err(self, f: Callable[[T],]) -> Result: ...
        def expect(self, msg: str) -> T: ...
        def unwrap(self) -> T: ...
        def expect_err(self, msg: str) -> T: ...
        def unwrap_err(self) -> T: ...
        def and_then(self, op: Callable[[T], Result]) -> Result: ...
        def or_else(self, op: Callable[[T], Result]) -> Result: ...
        def unwrap_or(self, default: T) -> T: ...
        def unwrap_or_else(self, op: Callable[[E], T]) -> T: ...
        def transpose(self) -> Option[Result.Err[T]]: ...
        def flatten(self) -> Result.Err[T]: ...