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
    The base image definition.
    """

    image_version: str
    version_opencv: str

    def build(self):
        """
        Build a new image given the parameters.
        """

        arguments = ["docker", "build", "-t", str(self)]

        for name, version in self._docker_args().items():
            arguments.append("--build-arg")
            arguments.append(f"{name}={version}")
        arguments.append(".")

        run(arguments, cwd=self._template_path(), check=False)

    def _docker_args(self) -> Mapping[str, str]:
        """
        The runtime args given to Docker.
        """
        raise NotImplementedError("Args are not specified")

    def _template_path(self) -> Path:
        """
        The path where the build should occure.
        """
        raise NotImplementedError("Template path not specified")


@dataclass(eq=True, frozen=True)
class ImageDefinitionPython(ImageDefinition):
    """
    The definition of a Python base image.
    """

    version_py: str
    version_numpy: str
    use_cuda: bool

    def __str__(self) -> str:
        if self.version_py is None:
            return f"ommatidia:{self.image_version}"
        else:
            return f"ommatidia-py{self.version_py}-cv{self.version_opencv}-np{self.version_numpy}{'-cuda' if self.use_cuda else ''}:{self.image_version}"

    def _docker_args(self) -> Mapping[str, str]:
        """
        Calculate the build arguments for Docker.
        """

        # Allow usage of default parameters
        if self.version_py is not None:
            args = {
                "VERSION_PYTHON": self.version_py,
                "VERSION_OPENCV": self.version_opencv,
                "VERSION_NUMPY": self.version_numpy,
            }
            if self.use_cuda:
                args["MICROMAMBA_TAG"] = "focal-cuda-11.3.1"
        else:
            args = {}
    
        return args

    def _template_path(self) -> Path:
        return Path(__file__).parent / "python"

    @classmethod
    def parse(cls, file_name: Path) -> Optional["ImageDefinition"]:
        """
        Try to parse a definition.
        """

        with file_name.open(mode="r", encoding="utf8") as file:
            regex = re.compile(
                r"FROM\s+ommatidia(?P<config>-py(?P<py>[0-9\.\*]+)-cv(?P<cv>[0-9\.\*]+)-np(?P<np>[0-9\.\*]+)(?P<cuda>-cuda)?)?:(?P<version>[0-9\.]+)"
            )
            image_config = regex.match(file.read())
            if image_config is None:
                return None

            return ImageDefinitionPython(
                image_version=image_config.group("version"),
                version_py=image_config.group("py"),
                version_opencv=image_config.group("cv"),
                version_numpy=image_config.group("np"),
                use_cuda=image_config.group("cuda") is not None,
            )


@dataclass(eq=True, frozen=True)
class ImageDefinitionCpp(ImageDefinition):
    """
    The definition of a C++ base image.
    """

    def __str__(self) -> str:
        return f"ommatidia_opencv{''.join(self.version_opencv.split('.'))}:{self.image_version}"

    def _docker_args(self) -> Mapping[str, str]:
        """
        Calculate the build arguments for Docker.
        """
        return {
            "OPENCV_VERSION": self.version_opencv,
        }

    def _template_path(self) -> Path:
        return Path(__file__).parent / "cpp"

    @classmethod
    def parse(cls, file_name: Path) -> Optional["ImageDefinition"]:
        """
        Try to parse a definition.
        """

        with file_name.open(mode="r", encoding="utf8") as file:
            regex = re.compile(
                r"FROM\s+ommatidia_opencv(?P<cv_major>[0-9\.])(?P<cv_minor>[0-9\.])(?P<cv_patch>[0-9\.]+):(?P<version>[0-9\.]+)"
            )
            required_versions = regex.match(file.read())
            if required_versions is None:
                return None

            return ImageDefinitionCpp(
                image_version=required_versions.group("version"),
                version_opencv=f"{required_versions.group('cv_major')}.{required_versions.group('cv_minor')}.{required_versions.group('cv_patch')}",
            )


def extract_image_definition(path: Path) -> Optional[ImageDefinition]:
    """
    Try to extract the image definition independently of its type.
    """
    definition = ImageDefinitionCpp.parse(path)
    if definition is not None:
        return definition
    return ImageDefinitionPython.parse(path)


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
        extract_image_definition(path) for path in Path(args.path).rglob("Dockerfile")
    ]
    image_definitions = set(
        image_definition
        for image_definition in image_definitions
        if image_definition is not None
    )

    # Build all the different detectors
    for definition in image_definitions:
        definition.build()
