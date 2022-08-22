from . import Test, Container


class TestMetadata(Test):
    def __init__(self) -> None:
        super().__init__("Metadata")

    def run(self, detector: Container):
        response = detector.request("/")
        response_json = response.json
        assert response.status == 200
        assert "name" in response_json
        assert "additional_information" in response_json
        assert "authors" in response_json
        assert "license" in response_json
        assert "prediction" in response_json


class TestConfig(Test):
    def __init__(self):
        super().__init__("Config")

    def run(self, detector: Container):
        response = detector.request("/")
        assert isinstance(response.json, object)
