from typing import Any, Type
from inspect import getframeinfo, stack
from ..docker import Container


class Test:
    """
    The abstract superclass for all tests.
    """

    def __init__(self, name: str):
        self.name = name

    def run(self, _detector: Container):
        """
        The actual test code overwritten by the test.
        """
        raise NotImplementedError()

    def assert_equal(self, value1: Any, value2: Any, msg: str = "") -> None:
        """
        Ensure two values are the same.
        """
        if value1 != value2:
            caller = getframeinfo(stack()[1][0])
            msg = "" if len(msg) == 0 else f": {msg}"
            raise AssertionError(
                f"{caller.filename}:{caller.lineno} {value1} != {value2}{msg}"
            )

    def assert_smaller(self, value: Any, maximum_value: Type, msg: str = "") -> None:
        """
        Ensure a value is the instance of a specific class
        """
        if value >= maximum_value:
            caller = getframeinfo(stack()[1][0])
            msg = "" if len(msg) == 0 else f": {msg}"
            raise AssertionError(
                f"{caller.filename}:{caller.lineno} '{value}' is not smaller than {maximum_value}{msg}"
            )

    def assert_isinstance(self, value: Any, expected_type: Type, msg: str = "") -> None:
        """
        Ensure a value is the instance of a specific class
        """
        if not isinstance(value, expected_type):
            caller = getframeinfo(stack()[1][0])
            msg = "" if len(msg) == 0 else f": {msg}"
            raise AssertionError(
                f"{caller.filename}:{caller.lineno} '{value}' is not of type {expected_type}{msg}"
            )

    def assert_in(self, value: Any, collection: Any, msg: str = "") -> None:
        """
        Assert a value is included within a collection.
        """
        is_included = True
        try:
            if value not in collection:
                is_included = False
        except TypeError:
            # Ensure correct error handly if type does not support proper "in" checks
            is_included = False

        if not is_included:
            caller = getframeinfo(stack()[1][0])
            msg = "" if len(msg) == 0 else f": {msg}"
            raise AssertionError(
                f"{caller.filename}:{caller.lineno} '{value}' is not in {collection}{msg}"
            )
