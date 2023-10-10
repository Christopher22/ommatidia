import json
import socket
import string
import subprocess
import tempfile
from contextlib import closing
from dataclasses import dataclass
from http.client import RemoteDisconnected
from pathlib import Path
from typing import Any, Optional, Union
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen


class InvalidContainerException(Exception):
    """
    The containers with detectors must spawn a HTTP server at port 8080.
    Failure from this behavior result in this error.
    """


@dataclass
class Response:
    """
    The response created from a request.
    """

    status: int
    body: str

    @property
    def json(self) -> Any:
        """
        Format the body as JSON and return it.
        """
        return json.loads(self.body)


class Container:
    """
    A container with included pupil detector.
    """

    def __init__(self, name_and_tag: str, show_output: bool = False, remove_container: bool = True):
        self.name_and_tag = name_and_tag
        self.port = Container._find_free_port()
        self.entry_point = f"http://127.0.0.1:{self.port}"
        self.remove_container = remove_container
        self._process = None
        self._is_ready = False
        self._output = (
            subprocess.DEVNULL
            if not show_output
            else tempfile.TemporaryFile(mode="w+t")
        )

    def __enter__(self) -> "Container":
        self._process = subprocess.Popen(
            (
                "docker",
                "run",
                "-p",
                f"127.0.0.1:{self.port}:8080/tcp",
                self.name_and_tag,
            ),
            stdout=self._output,
            stderr=subprocess.STDOUT,
        )
        return self

    def __exit__(self, _type, _value, _tb):
        # Close the buffer if requested
        if not isinstance(self._output, int):
            self._output.close()

        if self._process is not None and self._process.returncode is None:
            # Kill the process
            Container._kill(self.container_id, remove=self.remove_container)
            self._process.wait(5)

    def is_ready(self, wait: int) -> bool:
        """
        Check for the detector if it is ready. This will yield InvalidDetectorException on errors.
        """

        if self._process is None:
            raise ValueError("The process is not started")
        if self._is_ready:
            return True

        # Wait some time to allow start of the process
        try:
            self._process.wait(wait)
            # The detector should not exit by itself!
            raise InvalidContainerException("Unable to start the detector")
        except subprocess.TimeoutExpired:
            pass

        try:
            with urlopen(f"{self.entry_point}/"):
                self._is_ready = True
                return True
        except URLError as error:
            # We are unable to connect
            if isinstance(error.reason, socket.timeout):
                return False
            raise InvalidContainerException(
                f"The HTTP response of the detector appears corrupt: {error}"
            ) from error
        except RemoteDisconnected:
            return False
        except ConnectionResetError:
            return False
        
    def request(
        self,
        relative_url: str,
        method: str = "GET",
        body: Union[bytes, Any, None] = None,
        content_type: Optional[str] = None,
    ) -> Response:
        """
        Send a HTTP request to the detector.
        """

        if not self._is_ready and not self.is_ready(2):
            raise ValueError("The container is not ready")

        request = Request(url=f"{self.entry_point}{relative_url}", method=method)
        if body is not None:
            # Check if it is already serialized
            if isinstance(body, bytes):
                request.data = body
            else:
                request.data = json.dumps(body).encode("utf-8")

            request.add_header(
                "Content-Type",
                "application/json" if content_type is None else content_type,
            )

        try:
            with urlopen(request) as response:
                return Response(200, response.read().decode("utf-8"))
        except HTTPError as error:
            return Response(error.code, error.read().decode("utf-8"))

    @property
    def output(self) -> Optional[str]:
        """
        Access the output of the detector written to the command line.
        """
        if not isinstance(self._output, int):
            self._output.seek(0)
            value = self._output.read()
            self._output.seek(0, 2)
            return value

        return None

    @property
    def container_id(self) -> str:
        """
        Query the ID of the container. This operation is rather expensive, consider catching it.
        """
        with subprocess.Popen(
            (
                "docker",
                "ps",
                "--format",
                '"{{.ID}}"',
                "--filter",
                f"publish={self.port}",
            ),
            stdout=subprocess.PIPE,
        ) as proc:
            try:
                stdout, _ = proc.communicate(timeout=5)
                output = stdout.decode("ascii", errors="ignore").strip().strip('"')
                if len(output) != 12:
                    raise InvalidContainerException(
                        f"Unable to identify the running container. Id is malformed: '{output}'."
                    )
                return output
            except subprocess.TimeoutExpired as ex:
                proc.kill()
                raise InvalidContainerException("Querying ID failed") from ex

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

    @staticmethod
    def _kill(name: str, remove: bool = True):
        with subprocess.Popen(("docker", "kill", name), stdout=subprocess.PIPE) as proc:
            try:
                exit_code = proc.wait(5)
                if exit_code != 0:
                    raise InvalidContainerException(
                        f"Unable to kill container, exit code {exit_code}"
                    )
            except subprocess.TimeoutExpired as ex:
                proc.kill()
                raise InvalidContainerException(
                    "Unable to kill running container"
                ) from ex

        if remove:
            with subprocess.Popen(
                ("docker", "rm", name), stdout=subprocess.PIPE
            ) as proc:
                exit_code = proc.wait(5)
                if exit_code != 0:
                    raise InvalidContainerException(
                        f"Unable to remove container, exit code {exit_code}"
                    )


class Image:
    """
    An Docker image of a pupil detector.
    """

    def __init__(
        self,
        path: Path,
        name_and_tag: Optional[str] = None,
        ignore_cache: bool = False,
    ):
        if not path.is_dir() or not (path / "Dockerfile").is_file():
            raise ValueError("The given path does not point to a detector")
        self.path = path
        self.name_and_tag = (
            name_and_tag
            if name_and_tag is not None
            else Image._create_name_and_tag(path)
        )
        self._ignore_cache = ignore_cache
        self._output = tempfile.TemporaryFile(mode="w+t")

    def __enter__(self) -> "Image":
        try:
            # Allow ignoring caches
            commands = ["docker", "build", "-t", self.name_and_tag]
            if self._ignore_cache:
                commands.append("--no-cache")
            subprocess.run(
                commands + ["."],
                cwd=self.path,
                check=True,
                stdout=self._output,
                stderr=subprocess.STDOUT,
            )
        except subprocess.CalledProcessError as ex:
            self._output.seek(0)
            error_log = self._output.read()
            raise InvalidContainerException(
                f"The building of the image failed:\n{error_log}"
            ) from ex
        return self

    def __exit__(self, _type, _value, _tb):
        self._output.close()

    def spawn(self, show_output: bool = False) -> Container:
        """
        Prepare a container from this image.
        """
        return Container(self.name_and_tag, show_output=show_output)

    @staticmethod
    def _create_name_and_tag(path: Path) -> str:
        valid_chars = string.ascii_uppercase + string.ascii_lowercase
        return f"{''.join(value.lower() for value in path.name if value in valid_chars)}:0.1"
