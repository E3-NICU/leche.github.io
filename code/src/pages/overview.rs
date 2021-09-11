use yew::prelude::*;

use cobul::*;

use crate::components::{Boxes, Duration, DurationSelect, Temperature, TemperatureSelect};
use crate::constants::{VOLUME_INITIAL, VOLUME_RANGE, VOLUME_STEPS};
use crate::models::{LinearModel, Model};

#[derive(PartialEq, Clone, Copy)]
pub struct Overview {
    volume: u64,
    start: Temperature,
    wait: Duration,
}

#[derive(Debug)]
pub enum Msg {
    Volume(u64),
    Start(Temperature),
    Wait(Duration),
}

impl Component for Overview {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            volume: VOLUME_INITIAL,
            start: Temperature::Fridge,
            wait: Duration::Immediately,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        let old = *self;
        match msg {
            Msg::Volume(volume) => self.volume = volume,
            Msg::Start(start) => self.start = start,
            Msg::Wait(wait) => self.wait = wait,
        }
        self != &old
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let volume_change = ctx.link().callback(Msg::Volume);
        let temp_change = ctx.link().callback(Msg::Start);
        let time_change = ctx.link().callback(Msg::Wait);

        let model = LinearModel::default();
        let result = model.calc(self.volume, self.start.into(), self.wait.into());

        html! {
            <>
            <Boxes data={result} />

            <div class="py-5"> </div>

            <div class="field p-4">
                <label class="label"> {"Volume"} </label>
                <div class="control">
                    <Slider<u64> onchange={volume_change} range={VOLUME_RANGE} value={self.volume} steps={VOLUME_STEPS} postfix={"ml"} />
                </div>
            </div>

            <TemperatureSelect value={self.start} onchange={temp_change}/>
            <DurationSelect value={self.wait} onchange={time_change}/>
            </>
        }
    }
}
