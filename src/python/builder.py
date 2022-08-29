# This small script generates appropiate containers for different depending images

import re
import argparse
from pathlib import Path
from subprocess import run

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Build the appropiate container for a given Dockerfile"
    )
    parser.add_argument("file", type=str, help="The path to the Dockerfile")
    args = parser.parse_args()

    # Parse the arguments from the header of the file
    versions = {}
    with open(args.file, mode="r", encoding="utf8") as file:
        regex = re.compile(
            r"FROM\s+ommatidia-py(?P<py>[0-9\.]+)-cv(?P<cv>[0-9\.]+)-np(?P<np>[0-9\.]+):(?P<version>[0-9\.]+)"
        )
        required_versions = regex.match(file.read())
        version = required_versions.group("version")
        versions["VERSION_PYTHON"] = required_versions.group("py")
        versions["VERSION_OPENCV"] = required_versions.group("cv")
        versions["VERSION_NUMPY"] = required_versions.group("np")

    # Create the actual arguments
    arguments = [
        "docker",
        "build",
        "-t",
        f"ommatidia-py{versions['VERSION_PYTHON']}-cv{versions['VERSION_OPENCV']}-np{versions['VERSION_NUMPY']}:{version}",
    ]
    for name, version in versions.items():
        arguments.append("--build-arg")
        arguments.append(f"{name}={version}")
    arguments.append(".")

    run(arguments, cwd=Path(__file__).parent, check=False)
