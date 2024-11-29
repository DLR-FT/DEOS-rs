FROM ddc-i

# remove capabilities from QEMU executables
RUN find /usr/local/qemu/bin/ -type f -executable -exec setcap -r {} \;

# patch start-qemu to work with modern QEMU
COPY qemu-user-networking.patch /
RUN cp /desk/bin/startqemu /desk/bin/startqemu.old \
  && patch /desk/bin/startqemu < /qemu-user-networking.patch 

# enable the Ubuntu repositories
RUN \
  sed 's/#\(deb .*ubuntu\.com.*\)/\1/ ; s/\(.*ddci.com.*\)/#\1/' --in-place /etc/apt/sources.list \
  && apt-get update \
  && apt-get -y install gcc bindgen

# install Rust toolchain
RUN wget --https-only --quiet --output-document - https://sh.rustup.rs | sh -s -- \
  -y --default-toolchain nightly --profile default --component rust-src --no-modify-path

# make sure PATH is set up to pick up both the Rust toolchain and OpenArbors desk 
COPY --chmod=755 deos-rust-profile.sh /etc/profile.d/

# declare the working directory
WORKDIR /work
