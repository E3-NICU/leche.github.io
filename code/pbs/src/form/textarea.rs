use yew::prelude::*;
use yew::utils::NeqAssign;
use yew::web_sys::HtmlInputElement;

use crate::{Color, Size};
use crate::classify;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TextAreaProps {
    /// The `name` attribute for this form element.
    #[prop_or_default]
    pub name: Option<String>,

    /// The controlled value of this form element.
    #[prop_or_default]
    pub value: Option<String>,

    /// The callback to be used for propagating changes to this element's value.
    pub oninput: Callback<String>,

    #[prop_or_default]
    pub extra: String,

    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,

    /// The number of rows to which this component will be locked.
    #[prop_or_default]
    pub rows: Option<u32>,

    /// The size of this component.
    #[prop_or_default]
    pub size: Size,

    /// The size of this component.
    #[prop_or_default]
    pub color: Option<Color>,

    /// Fix the size of this component.
    #[prop_or_default]
    pub fixed: bool,

    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,

    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,

    /// Make this component read-only.
    #[prop_or_default]
    pub readonly: bool,

    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
}

/// A multiline textarea component.
///
/// [https://bulma.io/documentation/form/textarea/](https://bulma.io/documentation/form/textarea/)
pub struct TextArea {
    props: TextAreaProps,
    link: ComponentLink<Self>,
}

impl Component for TextArea {
    type Message = ();
    type Properties = TextAreaProps;

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
        let TextAreaProps { loading, r#static, .. } = self.props;

        let classes = classes!(
            "textarea",
            &self.props.extra,
            self.props.color.as_ref().map(ToString::to_string),
            self.props.size.to_string(),
            classify!(loading, r#static),
            self.props.fixed.then(|| "has-fixed-size")
        );

        let oninput = self.props.oninput.reform(|e: InputEvent| e.target_unchecked_into::<HtmlInputElement>().value());

        html! {
            <textarea
                name={self.props.name.clone()}
                value={self.props.value.clone()}
                oninput={oninput}
                class={classes}
                rows={self.props.rows.as_ref().map(ToString::to_string)}
                placeholder={self.props.placeholder.clone()}
                disabled={self.props.disabled}
                readonly={self.props.readonly}
                />
        }
    }
}
