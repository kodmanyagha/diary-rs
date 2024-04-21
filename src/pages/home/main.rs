use yew::prelude::*;

use crate::{helpers::format_time, pages::timer::main::TimerState};

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <div class={classes!("flex", "flex-col", "items-center", "justify-center", "h-screen")}>
        {"This is main page"}
        </div>
    }
}
