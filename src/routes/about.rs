use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about() -> Html {
    let about_content = include_str!("about.html");
    let about_content_in_js = AttrValue::from(about_content);
    html!(
        <div class="content">
            { Html::from_html_unchecked(about_content_in_js) }
        </div>
    )
}
