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

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# start a container instance & get the its name
export CONTAINER_ID=$(podman run --detach --rm --env DISPLAY --env DDCIFLEX_LICENSE_FILE="$DDCIFLEX_LICENSE_FILE" --net host --volume ~/.Xauthority:/root/.Xauthority:Z --volume $HOME:/root --volume /dev:/dev ddc-i:latest /OpenArbor/OpenArbor)
export CONTAINER_NAME=$(podman ps --format "{{.Names}}" --latest)


# remove capabilities from QEMU
podman exec -it $CONTAINER_ID sh -c 'find /usr/local/qemu/bin/ -type f -executable -exec setcap -r {} \;'

# copy the existing startqemu script
podman exec --interactive $CONTAINER_ID cp /desk/bin/startqemu /desk/bin/startqemu.old

# patch the startqemu script
cat "$SCRIPT_DIR/qemu-user-networking.patch" | podman exec --interactive $CONTAINER_ID patch /desk/bin/startqemu

# enable ubuntu repositories
podman exec --interactive $CONTAINER_ID sed 's/#\(deb .*ubuntu\.com.*\)/\1/ ; s/\(.*ddci.com.*\)/#\1/' --in-place /etc/apt/sources.list

# install rustup pre-requisites
podman exec --interactive $CONTAINER_ID apt-get update 
podman exec --interactive $CONTAINER_ID apt-get -y install gcc bindgen

# install rustup
podman exec --interactive $CONTAINER_ID bash -c "wget --https-only --quiet --output-document - https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly --profile default --no-modify-path"

# to get access:
echo "podman exec -it $CONTAINER_NAME bash --rcfile <(echo '. ~/.cargo/env ; export PATH=/desk/bin:$PATH')"
