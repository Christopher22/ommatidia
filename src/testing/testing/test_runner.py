from dataclasses import dataclass
from http.client import RemoteDisconnected
from typing import Optional, Iterable
from urllib.error import URLError

from .tests import Test
from .tests.default import *
from .tests.detector import *

from .docker import Container


@dataclass
class Result:
    test_name: str
    error_message: Optional[str] = None

    def __bool__(self) -> bool:
        return self.error_message is None

    def __str__(self) -> str:
        if self:
            return f"Test '{self.test_name}' OK"
        return f"Test '{self.test_name}' failed: {self.error_message}"


class TestRunner:
    def __init__(self, container: Container) -> None:
        self.tests = [test() for test in Test.__subclasses__()]  # type: ignore
        self.container = container
        self.is_successful = True

    def __iter__(self) -> Iterable[Result]:
        for test in self.tests:
            try:
                test.run(self.container)
            except AssertionError as error:
                self.is_successful = False
                yield Result(test.name, str(error))
            except URLError as error:
                self.is_successful = False
                yield Result(test.name, f"HTTP error: {error.reason}")
            except RemoteDisconnected as error:
                self.is_successful = False
                yield Result(test.name, f"HTTP error: {error}")

            yield Result(test.name)

    def __len__(self) -> int:
        return len(self.tests)

    def __bool__(self) -> bool:
        return self.is_successful
