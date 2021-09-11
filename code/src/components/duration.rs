use derive_more::Display;
use strum::EnumIter;
use yew::prelude::*;

use cobul::*;
use cobul::props::{Alignment, Color};

use crate::constants::{IMMEDIATELY_DEFAULT, LATER_INITIAL, WAIT_RANGE, WAIT_STEPS};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Later(u64);

impl Default for Later {
    fn default() -> Self { Later(LATER_INITIAL) }
}

#[derive(Clone, Debug, PartialEq, Copy, EnumIter, Display)]
pub enum Duration {
    #[display(fmt = "Direct")]
    Immediately,
    #[display(fmt = "Later")]
    Later(Later),
}

impl From<Duration> for u64 {
    fn from(duration: Duration) -> Self {
        match duration {
            Duration::Immediately => IMMEDIATELY_DEFAULT,
            Duration::Later(x) => x.0
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: Duration,
    pub onchange: Callback<Duration>,
}

#[function_component(DurationSelect)]
pub fn duration_select(props: &Props) -> Html {
    let onslider = props.onchange.reform(|x| Duration::Later(Later(x)));

    let slider = match props.value {
        Duration::Later(Later(value)) => html! {
            <Field extra="p-4">
                <Control>
                    <Slider<u64> onchange={onslider} range={WAIT_RANGE} value={value} steps={WAIT_STEPS} postfix={" min"} />
                </Control>
            </Field>
        },
        _ => html! {},
    };

    html! {
        <>
        <Field extra="p-4">
            <Label> {"Tijd van gebruik"} </Label>
            <Control>
                <EnumButtons<Duration> onclick={props.onchange.clone()} value={props.value} alignment={Alignment::Centered} color={Color::Link}/>
            </Control>
        </Field>
        {slider}
        </>
    }
}