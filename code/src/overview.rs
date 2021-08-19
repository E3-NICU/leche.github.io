use yew::prelude::*;
use yew::utils::NeqAssign;
use pbs::*;

use strum_macros::EnumIter;
use derive_more::Display;

const SECOND_EST: f64 = 2.1892;
const VOLUME_EST: f64 = -0.4125;
const INTERCEPT: f64 = 12.2044;
const TARGET_TEMP: f64 = 32.0;

pub struct Overview {
    link: ComponentLink<Self>,

    fridge: Fridge,
    duration: Duration,

    volume: f64,
    temp: f64,
    time: f64,
}

pub enum Msg {
    Fridge(Fridge),
    Duration(Duration),

    Volume(f64),
    Temp(f64),
    Time(f64),
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
        Self { link, fridge: Fridge::Fridge, duration: Duration::Immediately, volume: 40.0, temp: 9.0, time: 3.0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Volume(volume) => self.volume.neq_assign(volume),
            Msg::Temp(temp) => self.temp.neq_assign(temp),
            Msg::Time(time) => self.time.neq_assign(time),
            Msg::Fridge(fridge) => {
                self.temp = match fridge {
                    Fridge::Fridge => 9.0,
                    Fridge::Synthetic => 7.0,
                    Fridge::Room => 24.0,
                    Fridge::Measured => 9.0
                };
                self.fridge.neq_assign(fridge)
            }
            Msg::Duration(duration) => {
                self.time = 3.0;
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

        let seconds = (TARGET_TEMP - self.temp - INTERCEPT - VOLUME_EST * self.volume) / SECOND_EST;
        let rounded = seconds.round();

        let expected = INTERCEPT + SECOND_EST * rounded + VOLUME_EST * self.volume + self.temp;

        let fridge_slider = match self.fridge {
            Fridge::Measured => html! {
                <cbs::Slider onchange={temp_change} range={0.0..25.0} value={self.temp} steps={100} label={"milk temperature"} postfix={"°C"}/>
            },
            _ => html! {}
        };

        let time_slider = match self.duration {
            Duration::Immediately => html! {},
            Duration::Later => html! {
                <cbs::Slider onchange={time_change} range={0.0..10.0} value={self.time} steps={100} label={"time"} postfix={"min"}/>
            }
        };

        html! {
            <>
            <Columns>
                <Column>
                    <Box>
                        <Content>
                        <b class="is-size-3"> {"Time"} </b>
                        <p class="is-size-3"> {rounded.to_string()} <span class="has-text-grey"> {" seconds"} </span> </p>
                        </Content>
                    </Box>
                </Column>
                <Column>
                    <Box>
                        <Content>
                        <b class="is-size-3"> {"Power"} </b>
                        <p class="is-size-3">{"360"} <span class="has-text-grey"> {" Watt"} </span> </p>
                        </Content>
                    </Box>
                </Column>
                <Column>
                    <Box>
                        <b class="is-size-3"> {"Estimate"} </b>
                        <p class="is-size-3">
                            { format!("{:.1}", expected - 2.0) }
                            <span class="has-text-grey"> {"°C"} </span>
                            {" - "}
                            { format!("{:.1}", expected + 2.0) }
                            <span class="has-text-grey"> {"°C"} </span>
                        </p>
                    </Box>
                </Column>
            </Columns>
            <cbs::Slider onchange={volume_change} range={5.0..70.0} value={self.volume} steps={100} label={"Volume"} postfix={"ml"}/>

            <div class="field p-4">
                <label class="label"> {"Temperature"} </label>
                <div class="control">
                    <cbs::KvButtons<Fridge> onclick={fridge_change} value={self.fridge} alignment={Alignment::Centered} color={Color::Link}/>
                    {fridge_slider}
                </div>
            </div>

            <div class="field p-4">
                <label class="label"> {"Time of use"} </label>
                <div class="control">
                    <cbs::KvButtons<Duration> onclick={duration_change} value={self.duration} alignment={Alignment::Centered} color={Color::Link}/>
                    {time_slider}
                </div>
            </div>
            </>
        }
    }
}