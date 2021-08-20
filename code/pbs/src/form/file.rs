use gloo::file::File as SysFile;
use yew::prelude::*;
use yew::web_sys::HtmlInputElement;

use crate::{Alignment, classify, Color, Size};

#[derive(Properties, Clone)]
pub struct FileProps {
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub fullwidth: bool,

    #[prop_or_default]
    pub filename: Option<String>,

    #[prop_or_default]
    pub accept: Option<String>,

    #[prop_or_default]
    pub boxed: bool,

    #[prop_or_default]
    pub alignment: Alignment,

    #[prop_or_default]
    pub extra: String,

    // We don't use gloo_file filelist because it
    // doesn't allow to move from it for some reason.
    pub onupload: Callback<Vec<SysFile>>,
}

pub struct File {
    link: ComponentLink<Self>,
    props: FileProps,
}

impl Component for File {
    type Message = ();
    type Properties = FileProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let maybe_file = || match &self.props.filename {
            None => html! {},
            Some(file) => html! {<span class="file-name"> {file} </span>},
        };

        let FileProps { boxed, fullwidth, .. } = self.props;
        let maybe_name = self.props.filename.as_ref().map(|_| "has-name");
        let accept = self.props.accept.clone();

        let classes = classes!(
            "file",
            classify!(fullwidth, boxed),
            maybe_name,
            self.props.alignment.to_string(),
            &self.props.extra
        );

        let onchange = self.props.onupload.reform(|e: Event| {
            let files = e.target_unchecked_into::<HtmlInputElement>().files().unwrap();
            (0..files.length()).filter_map(|i| files.get(i)).map(SysFile::from).collect()
        });

        html! {
            <div class={classes}>
                <label class="file-label">
                <input class="file-input" type="file" accept={accept} onchange={onchange} />
                <span class="file-cta">
                    <span class="file-icon">
                    <i class="fas fa-upload"></i>
                    </span>
                    <span class="file-label">
                        {"Choose a file..."}
                    </span>
                </span>
                    { maybe_file() }
                </label>
            </div>
        }
    }
}
