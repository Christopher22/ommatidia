# coding: utf-8

from fastapi.testclient import TestClient
from pupil_detector.detector import Config


def test_config_get(client: TestClient):
    """
    Test case for config_get:
    Define the configurations for the pupil detection algorithm as an JSON Schema.
    """

    response = client.request(
        "GET",
        "/config/",
        headers={},
    )

    assert response.status_code == 200
    assert response.json() == Config.schema()


def test_root_get(client: TestClient):
    """Test case for root_get

    Get the meta data of the detector
    """

    response = client.request("GET", "/")
    assert response.status_code == 200
