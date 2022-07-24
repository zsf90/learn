use gloo::console::log;
use yew::prelude::*;

use crate::components::atoms::main_title::{Color, MainTitle};
use crate::components::molecules::custom_form::CustomForm;

#[function_component(App)]
pub fn app() -> Html {
    let main_title_load = Callback::from(|message: String| log!(message));
    html! {
        <div>
            <MainTitle title={"Hello,World!"} color={Color::Ok} on_load={main_title_load}/>
            <CustomForm />
        </div>
    }
}
