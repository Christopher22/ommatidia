from . import Test, Container
from .res import Sample
import math

class TestCreate(Test):
    def __init__(self):
        super().__init__("Create detector")

    def run(self, container: Container):
        response = container.request("/detections/", method="POST", body={})
        self.assert_equal(response.status, 200, response.body)
        self.assert_isinstance(response.json, int)


class TestGet(Test):
    def __init__(self):
        super().__init__("Get")

    def run(self, container: Container):
        response = container.request("/detections/", method="POST", body={})
        self.assert_equal(response.status, 200, response.body)

        detector_id = response.json
        response = container.request(f"/detections/{detector_id}/")
        self.assert_equal(response.status, 200)
        self.assert_isinstance(response.json, object)


class TestInvalidGet(Test):
    def __init__(self):
        super().__init__("Get invalid")

    def run(self, container: Container):
        response = container.request("/detections/666")
        self.assert_equal(response.status, 404)


class TestDetect(Test):
    def __init__(self):
        super().__init__("Detect")

    def run(self, container: Container):
        # Create detector
        creation_response = container.request("/detections/", method="POST", body={})
        self.assert_equal(creation_response.status, 200, creation_response.body)

        # Send invalid data to the detector
        sample = next(Sample.find_all())

        created_id = int(creation_response.json)
        response = container.request(
            f"/detections/{created_id}/",
            method="POST",
            body=sample.load(),
            content_type="image/png",
        )

        self.assert_equal(response.status, 200, response.body)
        response = response.json
        self.assert_isinstance(response["x"], (float, int))
        self.assert_isinstance(response["y"], (float, int))

        # Test the newly enforces information regarding the given sample
        self.assert_in("sample", response)
        self.assert_in("width", response["sample"])
        self.assert_in("height", response["sample"])
        self.assert_equal(response["sample"]["width"], sample.width)
        self.assert_equal(response["sample"]["height"], sample.height)

class TestCorrectDetection(Test):
    def __init__(self):
        super().__init__("TestCorrectDetection")

    def run(self, container: Container):
        MAXIMAL_PIXEL_DIFFERENCE = 20

        # Create detector
        creation_response = container.request("/detections/", method="POST", body={})
        self.assert_equal(creation_response.status, 200, creation_response.body)

        for sample in Sample.find_all():
            created_id = int(creation_response.json)
            response = container.request(
                f"/detections/{created_id}/",
                method="POST",
                body=sample.load(),
                content_type="image/png",
            )

            self.assert_equal(response.status, 200)
            response = response.json
            
            # The detector indicate it has no idea. Skip the image
            if "confidence" in response and (response["confidence"] is None or response["confidence"] <= 0.0):
                continue
        
            self.assert_smaller(
                ((sample.x - response["x"]) ** 2 + (sample.y - response["y"]) ** 2) ** 0.5, 
                MAXIMAL_PIXEL_DIFFERENCE, 
                f"X: {response} [{sample}]"
            )
            
            # Only if their is associated ellipse data
            if "major" in response:
                 # Ensure a proper angle
                self.assert_equal(response["angle"] >= 0.0 and response["angle"] <= math.pi * 2, True, msg="Angle invalid")

                # We need a smarter evaluation function here ...
                # self.assert_smaller(abs(sample.major - response["major"]), MAXIMAL_PIXEL_DIFFERENCE, f"Major axis: {response} [{sample}]")
                # self.assert_smaller(abs(sample.minor - response["minor"]), MAXIMAL_PIXEL_DIFFERENCE, f"Minor axis: {response} [{sample}]")
                # self.assert_smaller(abs(sample.angle - response["angle"]), 0.2, f"Angle: {response} [{sample}]")

class TestDetectInvalid(Test):
    def __init__(self):
        super().__init__("Detect invalid")

    def run(self, container: Container):
        # Create detector
        creation_response = container.request("/detections/", method="POST", body={})
        self.assert_equal(creation_response.status, 200, creation_response.body)

        # Send invalid data to the detector
        created_id = int(creation_response.json)
        response = container.request(
            f"/detections/{created_id}/",
            method="POST",
            body=b"ThisIsNotAnImage",
            content_type="image/png",
        )

        self.assert_equal(response.status, 400)


class TestDelete(Test):
    def __init__(self):
        super().__init__("Delete")

    def run(self, container: Container):
        # Create detector
        creation_response = container.request("/detections/", method="POST", body={})
        self.assert_equal(creation_response.status, 200, creation_response.body)
        created_id = int(creation_response.json)

        # Check it was created successfully
        response = container.request(f"/detections/{created_id}/")
        self.assert_equal(response.status, 200)

        # Check deleting works
        response = container.request(f"/detections/{created_id}/", method="DELETE")
        self.assert_equal(response.status, 200)

        # Check it does not exists any longer
        response = container.request(f"/detections/{created_id}/")
        self.assert_equal(response.status, 404)


class TestQuery(Test):
    def __init__(self):
        super().__init__("Query")

    def run(self, container: Container):
        # Create detector 1
        creation_response = container.request("/detections/", method="POST", body={})
        self.assert_equal(creation_response.status, 200, creation_response.body)
        created_id_1 = int(creation_response.json)

        # Create detector 2
        creation_response = container.request("/detections/", method="POST", body={})
        self.assert_equal(creation_response.status, 200)
        created_id_2 = int(creation_response.json)

        # Create response
        creation_response = container.request(
            "/detections/",
        )
        self.assert_equal(creation_response.status, 200)

        # The old IDs may remain in the list. Check only for the two most recent ones
        response = creation_response.json
        self.assert_in(value=created_id_1, collection=response)
        self.assert_in(value=created_id_2, collection=response)
