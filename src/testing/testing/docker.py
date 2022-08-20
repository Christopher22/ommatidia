from pathlib import Path
import string
from typing import Optional
import subprocess
from contextlib import closing
import socket
from urllib.error import URLError
from urllib.request import urlopen


class InvalidContainerException(Exception):
    """
    The containers with detectors must spawn a HTTP server at port 8080.
    Failure from this behavior result in this error.
    """


class Container:
    """
    A container with included pupil detector.
    """

    def __init__(self, name_and_tag: str):
        self.name_and_tag = name_and_tag
        self.port = Container._find_free_port()
        self._process = None

    def __enter__(self) -> "Container":
        self._process = subprocess.Popen(
            (
                "docker",
                "run",
                "-p",
                f"127.0.0.1:{self.port}:8080/tcp",
                self.name_and_tag,
            )
        )
        return self

    def __exit__(self, _type, _value, _tb):
        if self._process is not None and self._process.returncode is not None:
            self._process.kill()

    def is_ready(self, wait: int) -> bool:
        """
        Check for the detector if it is ready. This will yield InvalidDetectorException on errors.
        """

        if self._process is None:
            raise ValueError("The process is not started")

        # Wait some time to allow start of the process
        try:
            self._process.wait(wait)
            # The detector should not exit by itself!
            raise InvalidContainerException("Unable to start the detector")
        except subprocess.TimeoutExpired:
            pass

        try:
            with urlopen(self.entry_point):
                return True
        except URLError as error:
            # We are unable to connect
            if isinstance(error.reason, socket.timeout):
                return False
            raise InvalidContainerException(
                f"The HTTP response of the detector appears corrupt: {error}"
            ) from error

    @property
    def entry_point(self) -> str:
        """
        Calculate the entry point for this image.
        """
        return f"http://127.0.0.1:{self.port}/"

    @staticmethod
    def _find_free_port() -> int:
        """
        Identify a free port on the system.
        Adopted from https://stackoverflow.com/questions/1365265/on-localhost-how-do-i-pick-a-free-port-number
        """
        with closing(socket.socket(socket.AF_INET, socket.SOCK_STREAM)) as sock:
            sock.bind(("", 0))
            sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            return int(sock.getsockname()[1])


class Image:
    """
    An Docker image of a pupil detector.
    """

    def __init__(self, path: Path, name_and_tag: Optional[str] = None):
        if not path.is_dir() or not (path / "Dockerfile").is_file():
            raise ValueError("The given path does not point to a detector")
        self.path = path
        self.name_and_tag = (
            name_and_tag
            if name_and_tag is not None
            else Image._create_name_and_tag(path)
        )

    def __enter__(self) -> "Image":
        subprocess.run(
            ("docker", "build", "-t", self.name_and_tag, "."),
            cwd=self.path,
            check=True,
            capture_output=False,
        )
        return self

    def __exit__(self, _type, _value, _tb):
        pass

    def spawn(self) -> Container:
        """
        Prepare a container from this image.
        """
        return Container(self.name_and_tag)

    @staticmethod
    def _create_name_and_tag(path: Path) -> str:
        valid_chars = string.ascii_uppercase + string.ascii_lowercase
        return f"{''.join(value.lower() for value in path.name if value in valid_chars)}:0.1"
