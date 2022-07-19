# coding: utf-8

from fastapi.testclient import TestClient
import cv2 as cv
import numpy as np


def test_create(client: TestClient):
    """
    Initialize a new pupil detection algorithm with specific configuration.
    It should return an ID.
    """

    response = client.request(
        "POST",
        "/detections/",
        headers={},
        json={},
    )

    assert response.status_code == 200
    assert response.json() == 0


def test_get_invalid(client: TestClient):
    """
    Query an non-existing pupil detection algorithm and its configuration.
    """

    response = client.request(
        "GET",
        "/detections/42/",
        headers={},
    )

    assert response.status_code == 404


def test_get(client: TestClient):
    """
    Query an pupil detection algorithm and its configuration.
    """

    # Create response
    creation_response = client.request(
        "POST",
        "/detections/",
    )
    assert creation_response.status_code == 200

    # Query the created response
    created_id = int(creation_response.json())
    response = client.request("GET", f"/detections/{created_id}/")
    assert response.status_code == 200
    assert response.json() == {}


def test_detect_invalid(client: TestClient):
    """
    Evaluate a invalid sample with the configured pupil detection algorihm.
    """

    # Create detector
    creation_response = client.request(
        "POST",
        "/detections/",
    )
    assert creation_response.status_code == 200

    # Send invalid data to the detector
    created_id = int(creation_response.json())
    response = client.request(
        "POST",
        f"/detections/{created_id}/",
        data=b"This is not an image",
    )

    assert response.status_code == 400


def test_detect(client: TestClient):
    """
    Evaluate a invalid sample with the configured pupil detection algorihm.
    """

    # Create detector
    creation_response = client.request(
        "POST",
        "/detections/",
    )
    assert creation_response.status_code == 200

    # Send invalid data to the detector
    sample = np.full((21, 31), fill_value=42, dtype=np.uint8)
    sample = cv.imencode(".png", sample)
    assert sample[0], "Encoding failed"

    created_id = int(creation_response.json())
    response = client.request(
        "POST",
        f"/detections/{created_id}/",
        data=sample[1].tobytes(),
    )

    assert response.status_code == 200
    response = response.json()
    assert isinstance(response["x"], int)
    assert isinstance(response["y"], int)


def test_delete(client: TestClient):
    """Test case for detections_detection_id_delete

    Delete the instance of the pupil detection algorithm and remove all associated resources.
    """

    # Create detector
    creation_response = client.request(
        "POST",
        "/detections/",
    )
    assert creation_response.status_code == 200
    created_id = int(creation_response.json())

    # Check it was created successfully
    response = client.request("GET", f"/detections/{created_id}/")
    assert response.status_code == 200

    # Check deleting works
    response = client.request("DELETE", f"/detections/{created_id}/")
    assert response.status_code == 200

    # Check it does not exists any longer
    response = client.request("GET", f"/detections/{created_id}/")
    assert response.status_code == 404


def test_query_detections(client: TestClient):
    """
    Returns a list of running pupil detection algorithms.
    """

    # Create detector 1
    creation_response = client.request(
        "POST",
        "/detections/",
    )
    assert creation_response.status_code == 200
    created_id_1 = int(creation_response.json())

    # Create detector 2
    creation_response = client.request(
        "POST",
        "/detections/",
    )
    assert creation_response.status_code == 200
    created_id_2 = int(creation_response.json())

    # Create response
    creation_response = client.request(
        "GET",
        "/detections/",
    )
    assert creation_response.status_code == 200

    # The old IDs may remain in the list. Check only for the two most recent ones
    assert creation_response.json()[-2:] == [created_id_1, created_id_2]
