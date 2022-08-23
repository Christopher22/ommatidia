from typing import Any, Type
from inspect import getframeinfo, stack
from ..docker import Container


class Test:
    def __init__(self, name: str):
        self.name = name

    def run(self, _detector: Container):
        raise NotImplementedError()

    def assert_equal(self, value1: Any, value2: Any, msg: str = ""):
        if value1 != value2:
            caller = getframeinfo(stack()[1][0])
            msg = "" if len(msg) == 0 else f": {msg}"
            raise AssertionError(
                f"{caller.filename}:{caller.lineno} {value1} != {value2}{msg}"
            )

    def assert_isinstance(self, value: Any, expected_type: Type, msg: str = ""):
        if not isinstance(value, expected_type):
            caller = getframeinfo(stack()[1][0])
            msg = "" if len(msg) == 0 else f": {msg}"
            raise AssertionError(
                f"{caller.filename}:{caller.lineno} '{value}' is not of type {expected_type}{msg}"
            )

    def assert_in(self, value: Any, collection: Any, msg: str = ""):
        if value not in collection:
            caller = getframeinfo(stack()[1][0])
            msg = "" if len(msg) == 0 else f": {msg}"
            raise AssertionError(
                f"{caller.filename}:{caller.lineno} '{value}' is not in {collection}{msg}"
            )
