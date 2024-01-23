# rust-on-step-mxo2-lpc

`rust-hdl` and `yosys` examples on STEP-MXO2-LPC.

Based on support package [rust-hdl-bsp-step-mxo2-lpc](https://github.com/y5c4l3/rust-hdl-bsp-step-mxo2-lpc).

The support package requires these open FPGA synthesis and P&R tools:
  * [`yosys`](https://github.com/YosysHQ/yosys)
  * [`nextpnr-machxo2`](https://github.com/YosysHQ/nextpnr)
  * [JEDEC bitstream patched `ecppack`](https://github.com/cr1901/prjtrellis/tree/jed)

This project includes 4 examples:
  * `blinky`: official example from `rust-hdl`, blinking 8 onboard LEDs
  * `breathing`: PWMing onboard LED
  * `segment`: showing symbols at onboard 7-segment display
  * `switch`: controlling onboard LEDs via DIP switch

## Start

* If you have had those tools installed, simply run

  ```bash
  cargo run
  ```
  It will generate `blinky.jed` and `segment.jed` bitstreams.

* For convenience, I also packed a Docker image [step-mxo2-lpc-toolchains](https://hub.docker.com/r/y5c4l3/step-mxo2-lpc-toolchains) for all of the tools above. This project also includes a script `run_docker.sh`, which will run the binary in that container, and bring back all artifacts.

  ```bash
  cargo build
  # if you want to build it in Docker
  ./run_docker.sh
  ```

## Programming

STEP-MXO2-LPC only supports programming via an emulated flash drive.

```bash
cp firmware/blinky/top.jed /media/STEPLink
```
