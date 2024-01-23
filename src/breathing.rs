use rust_hdl::prelude::*;
use rust_hdl_bsp_step_mxo2_lpc::pins;

#[derive(Copy, Clone, PartialEq, Debug, LogicState)]
enum State {
    StateIncrement,
    StateDecrement,
} 

#[derive(LogicBlock)]
pub struct Breathing {
    pwm: PulseWidthModulator<8>,
    min_value: Constant<Bits<8>>,
    max_value: Constant<Bits<8>>,
    value: DFF<Bits<8>>,
    state: DFF<State>,
    threshold: Constant<Bits<26>>,
    counter: DFF<Bits<26>>,
    clock: Signal<In, Clock>,
    leds: Signal<Out, Bits<8>>,
}

impl Logic for Breathing {
    #[hdl_gen]
    fn update(&mut self) {
        self.pwm.clock.next = self.clock.val();
        self.pwm.enable.next = true.into();
        self.pwm.threshold.next = self.value.q.val();

        self.leds.next = !bit_cast::<8, 1>(self.pwm.active.val().into());

        dff_setup!(self, clock, counter, value, state);

        if self.counter.q.val() >= self.threshold.val() {
            if self.state.q.val() == State::StateIncrement {
                self.value.d.next = self.value.q.val() + 1;
            } else {
                self.value.d.next = self.value.q.val() - 1;
            }
            self.counter.d.next = 0.into();
        } else {
            self.counter.d.next = self.counter.q.val() + 1;
        }

        if self.value.q.val() >= self.max_value.val() {
            self.state.d.next = State::StateDecrement;
        }

        if self.value.q.val() <= self.min_value.val() {
            self.state.d.next = State::StateIncrement;
        }
    }
}

impl Default for Breathing {
    fn default() -> Self {
        Breathing {
            pwm: PulseWidthModulator::default(),
            min_value: Constant::new(5.into()),
            max_value: Constant::new(250.into()),
            value: DFF::default(),
            state: DFF::default(),
            threshold: Constant::new(72_000.into()),
            counter: DFF::default(),
            clock: pins::clock(),
            leds: pins::leds(),
        }
    }
}
