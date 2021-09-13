use derive_more::Display;
use strum::EnumIter;
use yew::prelude::*;

use cobul::{EnumButtons, Slider};
use cobul::props::{Alignment, Color};

use crate::constants::{FRIDGE_DEFAULT, MEASURED_INITIAL, ROOM_DEFAULT, SYNTHETIC_DEFAULT, TEMP_RANGE, TEMP_STEPS};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Measured(u64);

impl Default for Measured {
    fn default() -> Self { Measured(MEASURED_INITIAL) }
}

#[derive(Clone, Debug, PartialEq, Copy, EnumIter, Display)]
pub enum Temperature {
    #[display(fmt = "Koelkast")]
    Fridge,
    // #[display(fmt = "Synthetisch")]
    // Synthetic,
    #[display(fmt = "Kamertemperatuur")]
    Room,
    #[display(fmt = "Gemeten")]
    Measured(Measured),
}

impl From<Temperature> for u64 {
    fn from(temperature: Temperature) -> Self {
        match temperature {
            Temperature::Fridge => FRIDGE_DEFAULT,
            // Temperature::Synthetic => SYNTHETIC_DEFAULT,
            Temperature::Room => ROOM_DEFAULT,
            Temperature::Measured(x) => x.0
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: Temperature,
    pub onchange: Callback<Temperature>,
}

#[function_component(TemperatureSelect)]
pub fn temperature_select(props: &Props) -> Html {
    let onslider = props.onchange.reform(|x| Temperature::Measured(Measured(x)));

    let slider = match props.value {
        Temperature::Measured(Measured(value)) => html! {
            <div class="field p-4">
                <div class="control">
                    <Slider<u64> onchange={onslider} range={TEMP_RANGE} value={value} steps={TEMP_STEPS} postfix={"°C"} />
                </div>
            </div>
        },
        _ => html! {},
    };

    html! {
        <>
        <div class="field p-4">
            <label class="label"> {"Temperatuur"} </label>
            <div class="control">
                <EnumButtons<Temperature> onclick={props.onchange.clone()} value={props.value} alignment={Alignment::Centered} color={Color::Link}/>
            </div>
        </div>
        {slider}
        </>
    }
}