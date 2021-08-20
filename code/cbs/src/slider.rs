use std::fmt::Display;
use std::ops::Range;
use std::rc::Rc;

use num::{FromPrimitive, ToPrimitive};
use yew::prelude::*;
use yew::utils::NeqAssign;
use yew::web_sys::HtmlInputElement;

use pbs::Color;

// maybe use num crate to define the trait bounds
#[derive(Clone, PartialEq, Properties)]
pub struct SliderProps<T: PartialEq + Clone + Display + FromPrimitive + ToPrimitive + 'static> {
    pub onchange: Callback<T>,

    pub range: Range<T>,
    pub value: T,

    pub steps: u64,

    #[prop_or_default]
    pub postfix: String,
}

pub struct Slider<T: PartialEq + Clone + Display + FromPrimitive + ToPrimitive + 'static> {
    props: SliderProps<T>,
    link: ComponentLink<Self>,
}

impl<T: PartialEq + Clone + Display + FromPrimitive + ToPrimitive + 'static> Component for Slider<T> {
    type Message = String;
    type Properties = SliderProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let SliderProps { value, range, steps, postfix, .. } = self.props.clone();
        let (start, end) = (range.start.to_f64().unwrap(), range.end.to_f64().unwrap());

        let percent = 100.0 * (value.to_f64().unwrap() - start) / (end - start);
        let style = format!("position:absolute;left:calc({}% + {}px)", percent, 12.0 - 0.23 * percent);

        let min = start.to_string();
        let max = end.to_string();
        let step = ((end - start) / (steps as f64)).to_string();

        let oninput = self.props.onchange.reform(|e: InputEvent| {
            T::from_f64(e.target_unchecked_into::<HtmlInputElement>().value_as_number()).unwrap()
        });

        html! {
            <>
            <input class="slider" min={min} max={max} step={step} type="range" value={value.to_string()} oninput={oninput} />
            <p class="slider-label-top" style={style}> {format!("{:.0} {}", value, postfix)} </p>
            <div class="is-flex is-justify-content-space-between">
                <p>{start}</p>
                <p>{end}</p>
            </div>
            </>
        }
    }
}