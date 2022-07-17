use blockies::Ethereum;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub address: String,
}

#[function_component(Blockie)]
pub fn blockie(props: &Props) -> Html {
    let mut png = Vec::new();

    Ethereum::default()
        .create_icon(&mut png, props.address.as_bytes())
        .expect("blockie could not be encoded as base64");

    let base64 = format!("data:image/png;base64,{}", base64::encode(&png));

    html! {
        <div class="blockie">
            <img src={base64} class="mask mask-squircle" alt={props.address.clone()} />
        </div>
    }
}
