mod blinky;
mod breathing;
mod segment;
mod switch;

use blinky::Blinky;
use breathing::Breathing;
use segment::Segment;
use switch::Switch;

use rust_hdl_bsp_step_mxo2_lpc::synth;

fn main() {
    let uut = Blinky::default();
    synth::generate_bitstream(uut, "firmware/blinky");
    let uut = Breathing::default();
    synth::generate_bitstream(uut, "firmware/breathing");
    let uut = Segment::default();
    synth::generate_bitstream(uut, "firmware/segment");
    let uut = Switch::default();
    synth::generate_bitstream(uut, "firmware/switch");
}
