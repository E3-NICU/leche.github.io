use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yew::prelude::*;
use yew::utils::NeqAssign;

use pbs::{Alignment, Color};

#[derive(Clone, Properties, PartialEq)]
pub struct KvButtonsProps<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub alignment: Alignment,

    pub color: Color,

    pub value: T,

    pub onclick: Callback<T>,
}

pub struct KvButtons<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    props: KvButtonsProps<T>,
    link: ComponentLink<Self>,
}

impl<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> Component for KvButtons<T> {
    type Message = ();
    type Properties = KvButtonsProps<T>;

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
        let button_map = |variant: T| {
            let selected = self.props.value == variant;
            let color = selected.then(|| self.props.color);

            let onclick = self.props.onclick.reform(move |_| variant);

            html! {
                <pbs::Button color={color} onclick={onclick} selected={selected}>
                    {variant.to_string()}
                </pbs::Button>
            }
        };

        html! {
            <pbs::Buttons addons=true alignment={self.props.alignment}>
                { for T::iter().map(button_map) }
            </pbs::Buttons>
        }
    }
}
