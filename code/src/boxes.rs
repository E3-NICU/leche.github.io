use yew::prelude::*;
use yew::utils::NeqAssign;

use pbs::*;

use crate::model::ModelResult;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub model: ModelResult,
}

#[function_component(Boxes)]
pub fn boxes(props: &Props) -> Html {
    let Props { model } = props.clone();

    html! {
        <Columns>
            <Column>
                <Box>
                    <Content>
                        <b class="is-size-3"> {"Time"} </b>
                        <p class="is-size-3"> {model.seconds} <span class="has-text-grey"> {" seconds"} </span> </p>
                    </Content>
                </Box>
            </Column>
            <Column>
                <Box>
                    <Content>
                        <b class="is-size-3"> {"Power"} </b>
                        <p class="is-size-3">{model.watt} <span class="has-text-grey"> {" Watt"} </span> </p>
                    </Content>
                </Box>
            </Column>
            <Column>
                <Box>
                    <b class="is-size-3"> {"Estimate"} </b>
                    <p class="is-size-3">
                        { format!("{:.1}", model.estimate.start) }
                        <span class="has-text-grey"> {"°C"} </span>
                        {" - "}
                        { format!("{:.1}", model.estimate.end) }
                        <span class="has-text-grey"> {"°C"} </span>
                    </p>
                </Box>
            </Column>
        </Columns>
    }
}