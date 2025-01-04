use yew::prelude::*;

use crate::navigation::NavigationBar;

#[function_component(App)]
pub fn app() -> Html {
    let view_content = use_state_eq(|| {
        html!(
            <div>
                {"⚒️ Loading page ⚒️"}
            </div>
        )
    });
    let view_content_clone = view_content.clone();
    let on_view_content_update =
        use_callback(move |html: Html, _| view_content_clone.set(html), ());

    html! {
        <div>
            <div class="is-primary">
                <NavigationBar {on_view_content_update}/>
            </div>
            <div class="container">
                {(*view_content).clone()}
            </div>
        </div>
    }
}
