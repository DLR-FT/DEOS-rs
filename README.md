# Setting up the QEMU Platform Project

1. `File` -> `New` -> `DDC-I Deos Platform Projects`
2. `Select a Deos reference platform` -> `qemu-arm`
3. Set the IP Addresses:
   - In the workspace view to the left:
   - In your Platform Project
   - Under `Complete Integration`
   - Double click `Kernel Files`
   - Right click on `lwip.config`
   - `Copy for Editing...`
   - In the file that now opened:
     - change the IP address from `192.168.19.17` to `10.0.2.15`
     - change the netmask from `255.255.255.254` to `255.255.255.0`
       (This is the default guest IP & netmask used by QEMU's SLiRP)
4. Add the QEMU remote target
   - In the `Target Manager` at the bottom left, click `New Remote Target` (icon is a T-Junction with a plus)
   - `Hostname` is `localhost`
   - `Platform Project` as created before
   - `OK`
5. Launch QEMU and connect
   - In the `Target Manager` at the bottom left, click `Launch Emulator` (icon is a green circle with a white triangle)
   - Right click `Status Monitor` -> `enable`
   - Right click `Load List Manager` -> `enable`
   - **Note** Ping does not work, QEMU SLiRP does not implement ICMP

# Launch a new project

1. `File` -> `New` -> `DDC-I Deos Example Projects`
2. Tick `Kernel` -> [ ] `hello-world`
3. Select `Target` to be `ARM`
4. Add the example project to the platform project
   - In the workspace view, drag the example project over the platform project, drop it there
   - Click `OK`

# Compiling & running example Rust App

1. Launch the IDE
   - `./scripts/run-ddc-i-deos.sh`
2. Compile the Rust code
   - Use the `podman exec` command printed by the script from step 1 to get a shell
   - Navigate to the example demo
   - Run `cargo build`
3. Copy the generated executable over the example executable from the example project
   - The Rust binary is `target/armv7-unknown-deos-eabihf/debug/deos-api.exe` (which actually is an ELF)
   - The final hello-world executable is `hello-world/output/arm-eabi/diagnostic/hello-world.exe`
   - `cp examples/armv7l-hello-world/target/armv7-unknown-deos-eabihf/debug/deos-api.exe OpenArbor/workspace/hello-world/output/arm-eabi/diagnostic/hello-world.exe`
4. Upload the new, rusty executable to the DEOS inside QEMU
   - In the `Target Manager` at the bottom left, click `Update Target Load` (icon is a green arrow above an envelope)
   - There should be a `Will Be Update` element in the `Synchronize Files` tree-view
   - Click `OK`
5. Finished! On the QEMU window you should now see a different, more rusty number sequence counting the OS ticks.
