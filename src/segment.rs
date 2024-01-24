use rust_hdl::prelude::*;
use rust_hdl_bsp_step_mxo2_lpc::pins::{self, SegmentDisplay, CLOCK_SPEED_12MHZ};

const SYMBOL_Y: u8 = 0b01101110;
const SYMBOL_5: u8 = 0b01101101;
const SYMBOL_C: u8 = 0b00111001;
const SYMBOL_4: u8 = 0b01100110;
const SYMBOL_L: u8 = 0b00111000;
const SYMBOL_3: u8 = 0b01001111;

#[derive(Copy, Clone, PartialEq, Debug, LogicState)]
enum State {
    StateY5,
    StateC4,
    StateL3,
    StateY,
    State5,
    StateC,
    State4,
    StateL,
    State3,
}

#[derive(LogicBlock)]
pub struct Segment {
    clock: Signal<In, Clock>,
    counter: DFF<Bits<24>>,
    threshold: Constant<Bits<24>>,
    state: DFF<State>,
    symbols: [Constant<Bits<8>>; 6],
    displays: [SegmentDisplay; 2],
}

impl Logic for Segment {
    #[hdl_gen]
    fn update(&mut self) {
        self.displays[0].dimmed.next = false.into();
        self.displays[1].dimmed.next = false.into();
        dff_setup!(self, clock, counter, state);
        if self.counter.q.val() == self.threshold.val() {
            self.counter.d.next = bits::<24>(0);
            match self.state.q.val() {
                State::StateY5 => {
                    self.state.d.next = State::StateC4;
                }
                State::StateC4 => {
                    self.state.d.next = State::StateL3;
                }
                State::StateL3 => {
                    self.state.d.next = State::StateY;
                }
                State::StateY => {
                    self.state.d.next = State::State5;
                }
                State::State5 => {
                    self.state.d.next = State::StateC;
                }
                State::StateC => {
                    self.state.d.next = State::State4;
                }
                State::State4 => {
                    self.state.d.next = State::StateL;
                }
                State::StateL => {
                    self.state.d.next = State::State3;
                }
                State::State3 => {
                    self.state.d.next = State::StateY5;
                }
                _ => {
                    self.state.d.next = State::StateY5;
                }
            }
        } else {
            self.counter.d.next = self.counter.q.val() + 1;
        }

        match self.state.q.val() {
            State::StateY5 => {
                self.displays[0].segments.next = self.symbols[0].val();
                self.displays[1].segments.next = self.symbols[1].val();
            }
            State::StateC4 => {
                self.displays[0].segments.next = self.symbols[2].val();
                self.displays[1].segments.next = self.symbols[3].val();
            }
            State::StateL3 => {
                self.displays[0].segments.next = self.symbols[4].val();
                self.displays[1].segments.next = self.symbols[5].val();
            }
            State::StateY => {
                self.displays[0].segments.next = self.symbols[0].val();
                self.displays[1].segments.next = self.symbols[0].val();
            }
            State::State5 => {
                self.displays[0].segments.next = self.symbols[1].val();
                self.displays[1].segments.next = self.symbols[1].val();
            }
            State::StateC => {
                self.displays[0].segments.next = self.symbols[2].val();
                self.displays[1].segments.next = self.symbols[2].val();
            }
            State::State4 => {
                self.displays[0].segments.next = self.symbols[3].val();
                self.displays[1].segments.next = self.symbols[3].val();
            }
            State::StateL => {
                self.displays[0].segments.next = self.symbols[4].val();
                self.displays[1].segments.next = self.symbols[4].val();
            }
            State::State3 => {
                self.displays[0].segments.next = self.symbols[5].val();
                self.displays[1].segments.next = self.symbols[5].val();
            }
            _ => {
                self.displays[0].segments.next = self.symbols[0].val();
                self.displays[1].segments.next = self.symbols[1].val();
            }
        }
    }
}

impl Default for Segment {
    fn default() -> Self {
        Segment {
            clock: pins::clock(),
            counter: DFF::default(),
            threshold: Constant::new(CLOCK_SPEED_12MHZ.into()),
            state: DFF::default(),
            symbols: [SYMBOL_Y, SYMBOL_5, SYMBOL_C, SYMBOL_4, SYMBOL_L, SYMBOL_3]
                .map(|s| Constant::new((s as u64).into())),
            displays: pins::segment_displays(),
        }
    }
}
