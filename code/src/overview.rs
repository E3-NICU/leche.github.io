use std::ops::Range;

use derive_more::Display;
use strum_macros::EnumIter;
use yew::prelude::*;
use yew::utils::NeqAssign;

use pbs::*;

use crate::boxes::Boxes;
use crate::model::exec_model;

// Change these values to have different min and max for the sliders
const TEMP_RANGE: Range<u64> = 0..25;
const VOLUME_RANGE: Range<u64> = 5..70;
const TIME_RANGE: Range<u64> = 0..10;

// Do not change these, these indicate the steps on their respective slider slider
const TEMP_STEPS: u64 = TEMP_RANGE.end - TEMP_RANGE.start;
const VOLUME_STEPS: u64 = VOLUME_RANGE.end - VOLUME_RANGE.start;
const TIME_STEPS: u64 = TIME_RANGE.end - TIME_RANGE.start;

const FRIDGE_DEFAULT: u64 = 9;
const SYNTHETIC_DEFAULT: u64 = 7;
const ROOM_DEFAULT: u64 = 23;

const VOLUME_INITIAL: u64 = 40;
const MEASURED_INITIAL: u64 = 9;
const LATER_INITIAL: u64 = 3;

pub struct Overview {
    link: ComponentLink<Self>,

    fridge: Fridge,
    duration: Duration,

    volume: u64,
    temp: u64,
    time: u64,
}

#[derive(Debug)]
pub enum Msg {
    Fridge(Fridge),
    Duration(Duration),

    Volume(u64),
    Temp(u64),
    Time(u64),
}

#[derive(Clone, Debug, PartialEq, Copy, EnumIter, Display)]
pub enum Fridge {
    #[display(fmt = "Fridge")]
    Fridge,
    #[display(fmt = "Synthetic")]
    Synthetic,
    #[display(fmt = "Room")]
    Room,
    #[display(fmt = "Measured")]
    Measured,
}

#[derive(Clone, Debug, PartialEq, Copy, EnumIter, Display)]
pub enum Duration {
    #[display(fmt = "Immediately")]
    Immediately,
    #[display(fmt = "Later")]
    Later,
}

impl Component for Overview {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fridge: Fridge::Fridge,
            duration: Duration::Immediately,
            volume: VOLUME_INITIAL,
            temp: MEASURED_INITIAL,
            time: LATER_INITIAL,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Volume(volume) => self.volume.neq_assign(volume),
            Msg::Temp(temp) => self.temp.neq_assign(temp),
            Msg::Time(time) => self.time.neq_assign(time),
            Msg::Fridge(fridge) => {
                self.temp = match fridge {
                    Fridge::Fridge => FRIDGE_DEFAULT,
                    Fridge::Synthetic => SYNTHETIC_DEFAULT,
                    Fridge::Room => ROOM_DEFAULT,
                    Fridge::Measured => MEASURED_INITIAL,
                };
                self.fridge.neq_assign(fridge)
            }
            Msg::Duration(duration) => {
                self.time = LATER_INITIAL;
                self.duration.neq_assign(duration)
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let volume_change = self.link.callback(Msg::Volume);
        let temp_change = self.link.callback(Msg::Temp);
        let time_change = self.link.callback(Msg::Time);
        let fridge_change = self.link.callback(Msg::Fridge);
        let duration_change = self.link.callback(Msg::Duration);

        let fridge_slider = match self.fridge {
            Fridge::Measured => html! {
                <div class="field p-4">
                    <div class="control">
                        <cbs::Slider<u64> onchange={temp_change} range={TEMP_RANGE} value={self.temp} steps={TEMP_STEPS} postfix={"°C"} />
                    </div>
                </div>
            },
            _ => html! {},
        };

        let time_slider = match self.duration {
            Duration::Immediately => html! {},
            Duration::Later => html! {
                <div class="field p-4">
                    <div class="control">
                        <cbs::Slider<u64> onchange={time_change} range={TIME_RANGE} value={self.time} steps={TIME_STEPS} postfix={"min"} />
                    </div>
                </div>
            },
        };

        html! {
            <>
            <Boxes model={exec_model(self.volume, self.temp, self.time)} />

            <div class="py-5"> </div>

            <div class="field p-4">
                <label class="label"> {"Volume"} </label>
                <div class="control">
                    <cbs::Slider<u64> onchange={volume_change} range={VOLUME_RANGE} value={self.volume} steps={VOLUME_STEPS} postfix={"ml"} />
                </div>
            </div>

            <div class="field p-4">
                <label class="label"> {"Temperature"} </label>
                <div class="control">
                    <cbs::KvButtons<Fridge> onclick={fridge_change} value={self.fridge} alignment={Alignment::Centered} color={Color::Link}/>
                </div>
            </div>
            {fridge_slider}

            <div class="field p-4">
                <label class="label"> {"Time of use"} </label>
                <div class="control">
                    <cbs::KvButtons<Duration> onclick={duration_change} value={self.duration} alignment={Alignment::Centered} color={Color::Link}/>
                </div>
            </div>
            {time_slider}
            </>
        }
    }
}
