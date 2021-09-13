use yew::prelude::*;

use cobul::*;

use crate::Page;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onpage: Callback<Page>,
    pub onexternal: Callback<String>,
}

#[function_component(MainFooter)]
pub fn footer(props: &Props) -> Html {
    let ongithub = props.onexternal.reform(|_| "https://github.com/E3-NICU/leche.github.io".to_owned());
    let oninfo = props.onpage.reform(|_| Page::Info);

    html! {
        <Footer extra="column is-narrow pb-3">
            <Columns>
                <Column extra="pr-6">
                    <Title> {"Over het project"}</Title>
                    <Block>
                        <p style="hyphens:auto"> {"Dit is een pilootproject dat streeft naar standardisatie \
                        van het opwarmproces van babymelk in de microgolf."} </p>
                    </Block>
                    <Buttons>
                        <Button onclick={ongithub}> <Icon icon="fab fa-github"/> <span> {"Github"} </span> </Button>
                    </Buttons>
                </Column>
                <Column extra="pr-6">
                    <Title> {"Ontwikkelaar"}</Title>
                    <Block>
                        <p style="hyphens:auto">
                            {"Deze applicatie is ontwikkelt door Thomas Dooms, een masterstudent informatica aan UAntwerpen."}
                            {"Indien u vragen heeft over de website, kan u terecht bij:"}
                        </p>
                    </Block>
                    <Block>
                        <IconText>
                            <Icon icon="fas fa-envelope"/>
                            {"charlie.beirnaert@uantwerpen.be"}
                        </IconText>
                        <IconText>
                            <Icon icon="fas fa-envelope"/>
                            {"ludo.mahieu@uza.be"}
                        </IconText>
                    </Block>
                </Column>
                <Column extra="pr-6">
                    <Title> {"Info"}</Title>
                    <Block>
                         <p style="hyphens:auto"> {"Meer info over dit project en uitleg bij de bij het \
                         opwarmproces en functionaliteit van deze hulpsite kan u hier vinden."} </p>
                    </Block>
                    <Block>
                        <Button onclick={oninfo}> <Icon icon="fas fa-question"/> <span> {"info"} </span> </Button>
                    </Block>
                </Column>
            </Columns>
        </Footer>
    }
}