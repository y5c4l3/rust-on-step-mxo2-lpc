use rust_hdl::prelude::*;
use rust_hdl_bsp_step_mxo2_lpc::pins;

#[derive(LogicBlock)]
pub struct Switch {
    leds: Signal<Out, Bits<8>>,
    switch: Signal<In, Bits<4>>,
}

impl Logic for Switch {
    #[hdl_gen]
    fn update(&mut self) {
        self.leds.next = !bit_cast::<8, 4>(self.switch.val());
    }
}

impl Default for Switch {
    fn default() -> Self {
        Switch {
            leds: pins::leds(),
            switch: pins::dip_switch(),
        }
    }
}
