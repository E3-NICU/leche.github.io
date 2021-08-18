use yew::prelude::*;
use yew::web_sys::HtmlInputElement;
use yew::utils::NeqAssign;

use pbs::Color;
use std::ops::Range;
use std::rc::Rc;

#[derive(Clone, Properties, PartialEq)]
pub struct SliderProps {
    pub onchange: Callback<f64>,

    pub range: Range<f64>,
    pub value: f64,
    pub steps: u64,
    pub label: String,

    // TODO: make this a generic function
    // #[prop_or_default]
    pub postfix: String,
}

pub struct Slider {
    props: SliderProps,
    link: ComponentLink<Self>,
}

impl Component for Slider {
    type Message = String;
    type Properties = SliderProps;

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
        let SliderProps{value, range, steps, label, postfix, ..} = self.props.clone();

        let percent = 100.0 * (value - range.start) / (range.end - range.start);
        let style = format!("position:absolute;left:calc({}% + {}px)", percent, 12.0 - 0.23 * percent);

        let min = range.start.to_string();
        let max = range.end.to_string();
        let step = ((range.end - range.start) as f64 / steps as f64).to_string();

        let oninput = self.props.onchange.reform(|e: InputEvent| {
            e.target_unchecked_into::<HtmlInputElement>().value_as_number()
        });

        let bubble = format!("{:.0} {}", value, postfix);

        html! {
            <div class="field p-4">
                <label class="label"> {label} </label>
                <div class="control">
                    <input class="slider" min={min} max={max} step={step} type="range" value={value.to_string()} oninput={oninput} />
                    <p class="slider-label-top" style={style}> {bubble} </p>
                    <div class="is-flex is-justify-content-space-between">
                        <p>{range.start}</p>
                        <p>{range.end}</p>
                    </div>
                </div>
            </div>
        }
    }
}