use rust_hdl::prelude::*;
use rust_hdl_bsp_step_mxo2_lpc::pins;
use rust_hdl_bsp_step_mxo2_lpc::pins::CLOCK_SPEED_12MHZ;
use std::time::Duration;

#[derive(LogicBlock)]
pub struct Blinky {
    pulser: Pulser,
    clock: Signal<In, Clock>,
    leds: Signal<Out, Bits<8>>,
}

impl Logic for Blinky {
    #[hdl_gen]
    fn update(&mut self) {
        self.pulser.enable.next = true;
        self.pulser.clock.next = self.clock.val();
        self.leds.next = 0x00.into();
        if self.pulser.pulse.val() {
            self.leds.next = 0xAA.into();
        }
    }
}

impl Default for Blinky {
    fn default() -> Self {
        let pulser = Pulser::new(CLOCK_SPEED_12MHZ.into(), 1.0, Duration::from_millis(250));
        Blinky {
            pulser,
            clock: pins::clock(),
            leds: pins::leds(),
        }
    }
}
