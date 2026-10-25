#!/bin/sh
#
# Run a command in the Linux image (see Containerfile) with the repository
# mounted at the same absolute path as on the host, so that paths in
# compile_commands.json and in the translation are identical across hosts.
#
# usage: c2rust/in-container.sh CMD [ARG...]
#
# Environment:
#   CONTAINER_IMAGE  image (default: mpdecimal-c2rust, see Containerfile;
#                    mpdecimal-amd64, see Containerfile.amd64)
#   CONTAINER_ARCH   image architecture (default: the host's; amd64 runs
#                    under Rosetta 2)
#   CONTAINER_CPUS, CONTAINER_MEMORY   resources (default: the container
#                    defaults)

set -eu

root=$(cd "$(dirname "$0")/.." && pwd)
: "${CONTAINER_IMAGE:=mpdecimal-c2rust}"

set -- --volume "$root:$root" --workdir "$root" \
    ${CONTAINER_ARCH:+--arch "$CONTAINER_ARCH"} \
    ${CONTAINER_CPUS:+--cpus "$CONTAINER_CPUS"} \
    ${CONTAINER_MEMORY:+--memory "$CONTAINER_MEMORY"} \
    "$CONTAINER_IMAGE" "$@"
[ "${CONTAINER_ARCH:-}" = amd64 ] && set -- --rosetta "$@"
exec container run --rm "$@"
