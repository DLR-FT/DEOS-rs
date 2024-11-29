#!/usr/bin/env bash

if [ $(sysctl --values net.ipv4.ip_unprivileged_port_start) -gt 20 ]
then
  echo "run the following"
  echo "sudo sysctl --write net.ipv4.ip_unprivileged_port_start=0"
  exit 1
fi

if [[ -z "${DDCIFLEX_LICENSE_FILE}" ]]
then
  echo "you must set DDCIFLEX_LICENSE_FILE"
  echo "Expected syntax: port@hostname.tld"
  exit 1
fi


if [[ -z "${CONTAINER_HOME}" ]]
then
  echo "you may set CONTAINER_HOME to control which host dir becomes the home dir for the container"
  echo "Defaults to current working director"
  echo "Beware when setting CONTAINER_HOME=\$HOME, that might break the Rust installation"
  echo "CONTAINER_HOME=${CONTAINER_HOME:=$PWD}"
fi


SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
TAG="ddc-i-rust"

# build the container image
podman build --tag "$TAG" "$SCRIPT_DIR" < "$SCRIPT_DIR/Dockerfile"

# start a container instance & get its name
export CONTAINER_ID=$(podman run --detach --rm --env DISPLAY --env DDCIFLEX_LICENSE_FILE="$DDCIFLEX_LICENSE_FILE" --net host --volume ~/.Xauthority:/root/.Xauthority:Z --volume "$CONTAINER_HOME:/work" --volume /dev:/dev "$TAG":latest /OpenArbor/OpenArbor)
export CONTAINER_NAME=$(podman ps --format "{{.Names}}" --latest)

# to get access:
echo -e "\n\nRun the following to get access to the container:\n\033[1mpodman exec -it $CONTAINER_NAME bash -l\033[0m"

# to get edit files using the host environment:
echo -e "\n\nRun the following to navigate the filessystem of the container:\n\033[1mpodman unshare \$SHELL\033[0m"
echo -e "From the 'podman unshare' shell, find the containers rootfs under:\n\033[1m$(podman unshare podman mount "$CONTAINER_NAME")\033[0m"
