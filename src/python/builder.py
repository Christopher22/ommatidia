# This small script generates appropiate containers for different depending images

import re
import argparse
from pathlib import Path
from subprocess import run
from dataclasses import dataclass
from typing import Mapping, Optional


@dataclass(eq=True, frozen=True)
class ImageDefinition:
    """
    The definition of a (Python) image.
    """

    image_version: str
    version_py: str
    version_opencv: str
    version_numpy: str

    def __str__(self) -> str:
        return f"ommatidia-py{self.version_py}-cv{self.version_opencv}-np{self.version_numpy}:{self.image_version}"

    def build(self):
        """
        Build a new image given the parameters.
        """

        arguments = ["docker", "build", "-t", str(self)]

        for name, version in self._docker_args().items():
            arguments.append("--build-arg")
            arguments.append(f"{name}={version}")
        arguments.append(".")

        run(arguments, cwd=Path(__file__).parent, check=False)

    def _docker_args(self) -> Mapping[str, str]:
        """
        Calculate the build arguments for Docker.
        """

        return {
            "VERSION_PYTHON": self.version_py,
            "VERSION_OPENCV": self.version_opencv,
            "VERSION_NUMPY": self.version_numpy,
        }

    @classmethod
    def parse(cls, file_name: Path) -> Optional["ImageDefinition"]:
        """
        Try to parse a definition.
        """

        with file_name.open(mode="r", encoding="utf8") as file:
            regex = re.compile(
                r"FROM\s+ommatidia-py(?P<py>[0-9\.]+)-cv(?P<cv>[0-9\.]+)-np(?P<np>[0-9\.]+):(?P<version>[0-9\.]+)"
            )
            required_versions = regex.match(file.read())
            if required_versions is None:
                return None

            return ImageDefinition(
                image_version=required_versions.group("version"),
                version_py=required_versions.group("py"),
                version_opencv=required_versions.group("cv"),
                version_numpy=required_versions.group("np"),
            )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Build the appropiate container for a given Dockerfile"
    )
    parser.add_argument(
        "path", type=str, help="folder containing (possible multiple) detectors"
    )
    args = parser.parse_args()

    # Parse all the files within
    image_definitions = [
        ImageDefinition.parse(path) for path in Path(args.path).rglob("Dockerfile")
    ]
    image_definitions = set(
        image_definition
        for image_definition in image_definitions
        if image_definition is not None
    )

    # Build all the different detectors
    for definition in image_definitions:
        definition.build()
