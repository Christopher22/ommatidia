import importlib.resources

from . import Test, Container, res


class TestCreate(Test):
    def __init__(self):
        super().__init__("Create detector")

    def run(self, container: Container):
        response = container.request("/detections/", method="POST", body={})
        assert response.status == 200
        assert isinstance(response.json, int)


class TestGet(Test):
    def __init__(self):
        super().__init__("Get")

    def run(self, container: Container):
        response = container.request("/detections/", method="POST", body={})
        assert response.status == 200

        detector_id = response.json
        response = container.request(f"/detections/{detector_id}/")
        assert response.status == 200
        assert isinstance(response.json, object)


class TestInvalidGet(Test):
    def __init__(self):
        super().__init__("Get invalid")

    def run(self, container: Container):
        response = container.request("/detections/666")
        assert response.status == 404


class TestDetect(Test):
    def __init__(self):
        super().__init__("Detect")

    def run(self, container: Container):
        # Create detector
        creation_response = container.request(
            "/detections/",
            method="POST",
        )
        assert creation_response.status == 200

        # Send invalid data to the detector
        sample = importlib.resources.read_binary(res, "example.png")
        created_id = int(creation_response.json)
        response = container.request(
            f"/detections/{created_id}/",
            method="POST",
            body=sample,
            content_type="image/png",
        )

        assert response.status == 200
        response = response.json
        assert isinstance(response["x"], int)
        assert isinstance(response["y"], int)


class TestDetectInvalid(Test):
    def __init__(self):
        super().__init__("Detect invalid")

    def run(self, container: Container):
        # Create detector
        creation_response = container.request(
            "/detections/",
            method="POST",
        )
        assert creation_response.status == 200

        # Send invalid data to the detector
        created_id = int(creation_response.json)
        response = container.request(
            f"/detections/{created_id}/",
            method="POST",
            body=b"ThisIsNotAnImage",
            content_type="image/png",
        )

        assert response.status == 400


class TestDelete(Test):
    def __init__(self):
        super().__init__("Delete")

    def run(self, container: Container):
        # Create detector
        creation_response = container.request("/detections/", method="POST", body={})
        assert creation_response.status == 200
        created_id = int(creation_response.json)

        # Check it was created successfully
        response = container.request(f"/detections/{created_id}/")
        assert response.status == 200

        # Check deleting works
        response = container.request(f"/detections/{created_id}/", method="DELETE")
        assert response.status == 200

        # Check it does not exists any longer
        response = container.request(f"/detections/{created_id}/")
        assert response.status == 404


class TestQuery(Test):
    def __init__(self):
        super().__init__("Query")

    def run(self, container: Container):
        # Create detector 1
        creation_response = container.request("/detections/", method="POST")
        assert creation_response.status == 200
        created_id_1 = int(creation_response.json())

        # Create detector 2
        creation_response = container.request(
            "/detections/",
            method="POST",
        )
        assert creation_response.status == 200
        created_id_2 = int(creation_response.json())

        # Create response
        creation_response = container.request(
            "/detections/",
        )
        assert creation_response.status == 200

        # The old IDs may remain in the list. Check only for the two most recent ones
        assert creation_response.json()[-2:] == [created_id_1, created_id_2]
