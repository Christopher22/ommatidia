from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Optional
import re
import math

@dataclass
class Sample:
    path: Path
    width: int
    height: int
    x: float
    y: float
    major: float
    minor: float
    angle: float

    @staticmethod
    def find_all(path: Optional[Path] = None) -> Iterable["Sample"]:
        if path is None:
            path = Path(__file__).parent

        pattern = re.compile(r"[A-Za-z0-9]+_(?P<width>[0-9]+)_(?P<height>[0-9]+)_(?P<x>[0-9\.]+)_(?P<y>[0-9\.]+)_(?P<major>[0-9\.]+)_(?P<minor>[0-9\.]+)_(?P<angle>[0-9\.\-]+)\.png")
        for file in path.glob("*.png"):
            match = pattern.match(file.name)
            if match is not None:
                yield Sample(
                    path=file,
                    width=int(match.group("width")),
                    height=int(match.group("height")),
                    x=float(match.group("x")),
                    y=float(match.group("y")),
                    major=float(match.group("major")),
                    minor=float(match.group("minor")),
                    # We define angles to be between 0 and 2 * PI
                    angle=float(match.group("angle")) + math.pi
                )

    def load(self) -> bytes:
        with self.path.open("rb") as file:
            return file.read()