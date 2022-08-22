from ..docker import Container


class Test:
    def __init__(self, name: str):
        self.name = name

    def run(self, _detector: Container):
        raise NotImplementedError()
