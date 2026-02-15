from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Union

NumberLike = Union[int, float, "MazieNum"]


@dataclass(frozen=True)
class MazieMode:
    """
    Runtime semantics switch.
    div0_identity=True means: x / 0 => x
    """
    div0_identity: bool = True


def _as_num(x: NumberLike, mode: MazieMode) -> "MazieNum":
    if isinstance(x, MazieNum):
        return MazieNum(x.value, mode=mode)
    if isinstance(x, (int, float)):
        return MazieNum(float(x), mode=mode)
    raise TypeError(f"Unsupported type: {type(x)!r}")


@dataclass(frozen=True)
class MazieNum:
    value: float
    mode: MazieMode = MazieMode()

    def unwrap(self) -> float:
        return float(self.value)

    def __add__(self, other: NumberLike) -> "MazieNum":
        o = _as_num(other, self.mode)
        return MazieNum(self.value + o.value, mode=self.mode)

    def __radd__(self, other: NumberLike) -> "MazieNum":
        return _as_num(other, self.mode).__add__(self)

    def __sub__(self, other: NumberLike) -> "MazieNum":
        o = _as_num(other, self.mode)
        return MazieNum(self.value - o.value, mode=self.mode)

    def __rsub__(self, other: NumberLike) -> "MazieNum":
        return _as_num(other, self.mode).__sub__(self)

    def __mul__(self, other: NumberLike) -> "MazieNum":
        o = _as_num(other, self.mode)
        return MazieNum(self.value * o.value, mode=self.mode)

    def __rmul__(self, other: NumberLike) -> "MazieNum":
        return _as_num(other, self.mode).__mul__(self)

    def __truediv__(self, other: NumberLike) -> "MazieNum":
        o = _as_num(other, self.mode)
        if o.value == 0.0:
            if self.mode.div0_identity:
                # Mazie semantics: x/0 => x
                return MazieNum(self.value, mode=self.mode)
            raise ZeroDivisionError("division by zero")
        return MazieNum(self.value / o.value, mode=self.mode)

    def __rtruediv__(self, other: NumberLike) -> "MazieNum":
        return _as_num(other, self.mode).__truediv__(self)

    def __neg__(self) -> "MazieNum":
        return MazieNum(-self.value, mode=self.mode)

    def __eq__(self, other: Any) -> bool:
        if isinstance(other, MazieNum):
            return self.value == other.value
        if isinstance(other, (int, float)):
            return self.value == float(other)
        return False

    def __repr__(self) -> str:
        return f"MazieNum({self.value}, mode={self.mode})"


def M(x: Union[int, float], *, mode: MazieMode | None = None) -> MazieNum:
    """Convenience wrapper: M(5) gives a MazieNum with Mazie runtime semantics."""
    return MazieNum(float(x), mode=mode or MazieMode())


if __name__ == "__main__":
    # Tiny demo
    x = M(5)
    print("M(5)/0 =>", (x / 0).unwrap())       # 5.0
    print("M(10)/2 =>", (M(10) / 2).unwrap())  # 5.0
