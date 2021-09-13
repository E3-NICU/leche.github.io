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
                        van het opwarmproces van babymelk in de microgolf. Extra uitleg bij de site vindt u hier."} </p>
                    </Block>
                    <Buttons>
                        <Button onclick={oninfo}> <Icon icon="fas fa-question"/> <span> {"info"} </span> </Button>
                        <Button onclick={ongithub}> <Icon icon="fab fa-github"/> <span> {"Github"} </span> </Button>
                    </Buttons>
                </Column>
                <Column extra="pr-6">
                    <Title> {"Ontwikkelaar"}</Title>
                    <Block>
                        <p style="hyphens:auto">
                            {"Deze applicatie is ontwikkelt door Thomas Dooms, een masterstudent informatica aan UAntwerpen."}
                        </p>
                    </Block>
                </Column>
                <Column extra="pr-6">
                    <Title> {"Contact"}</Title>
                    <Block>
                        <IconText>
                            <Icon icon="fas fa-envelope"/>
                            {"charlie.beirnaert@uantwerpen.be"}
                        </IconText>
                    </Block>
                    <Block>
                        <IconText>
                            <Icon icon="fas fa-envelope"/>
                            {"ludo.mahieu@uza.be"}
                        </IconText>
                    </Block>
                </Column>
            </Columns>
        </Footer>
    }
}