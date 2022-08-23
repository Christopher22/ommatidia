from . import Test, Container


class TestMetadata(Test):
    def __init__(self) -> None:
        super().__init__("Metadata")

    def run(self, detector: Container):
        response = detector.request("/")
        response_json = response.json
        self.assert_equal(response.status, 200)
        self.assert_in("name", response_json)
        self.assert_in("additional_information", response_json)
        self.assert_in("authors", response_json)
        self.assert_in("license", response_json)
        self.assert_in("prediction", response_json)


class TestConfig(Test):
    def __init__(self):
        super().__init__("Config")

    def run(self, detector: Container):
        response = detector.request("/")
        self.assert_isinstance(response.json, object)
