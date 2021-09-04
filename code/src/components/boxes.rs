use yew::prelude::*;

use pbs::prelude::*;

use crate::models::ModelResult;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub data: ModelResult,
}

#[function_component(Boxes)]
pub fn boxes(props: &Props) -> Html {
    let Props { data: ModelResult { seconds, expected, watt } } = props.clone();

    html! {
        <Columns>
            <Column>
                <Box>
                    <Content>
                        <b class="is-size-3"> {"Tijd"} </b>
                        <p class="is-size-3"> {seconds} <span class="has-text-grey"> {" seconden"} </span> </p>
                    </Content>
                </Box>
            </Column>
            <Column>
                <Box>
                    <Content>
                        <b class="is-size-3"> {"Wattage"} </b>
                        <p class="is-size-3">{watt} <span class="has-text-grey"> {" Watt"} </span> </p>
                    </Content>
                </Box>
            </Column>
            <Column>
                <Box>
                    <b class="is-size-3"> {"Schatting"} </b>
                    <p class="is-size-3">
                        { format!("{:.1}", expected.start) }
                        <span class="has-text-grey"> {"°C"} </span>
                        {" - "}
                        { format!("{:.1}", expected.end) }
                        <span class="has-text-grey"> {"°C"} </span>
                    </p>
                </Box>
            </Column>
        </Columns>
    }
}