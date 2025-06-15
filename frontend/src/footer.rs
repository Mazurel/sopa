use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let footer_copyright = t!("footer-copyright");
    let footer_license_text = t!("footer-license-text");
    let footer_license_link = t!("footer-license-link");

    html! {
        <footer class="footer">
            <div class="content has-text-centered">
                <p>
                    <strong>
                        <a href="https://github.com/Mazurel/sopa" target="_blank">
                            {"SOPa"}
                        </a>
                        {footer_copyright}
                    </strong>
                </p>
                <p>
                    {footer_license_text} {" "}
                    <a href="https://www.gnu.org/licenses/gpl-2.0.html" target="_blank">
                        {footer_license_link}
                    </a>
                </p>
            </div>
        </footer>
    }
}
