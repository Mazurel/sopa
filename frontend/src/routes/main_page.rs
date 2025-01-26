use yew::prelude::*;

#[function_component(MainPage)]
pub fn main_page() -> Html {
    html!(
        <div class="content is-large">
            <div>
                <h1>
                    {"Witaj w SOPa Aphia"}
                </h1>
                <p>
                {"
                    Cieszę się, że tu jesteś.
                    SOPa Aphia, to wersja demo Systemu Ośrodków Pomocy przygotowana na potrzeby Ośrodka Interwencji Kryzysowej w Gdyni.
                "}
                </p>
                <p>
                {"
                    Niezależnie z jakim wyzwaniem się mierzysz, zostałem stworzony, aby pomóc ci zrobić pierwszy krok w stronę zmiany.
                "}
                </p>
                <p>
                {"
                    Znajdź wsparcie, którego potrzebujesz i odkryj, że nie jesteś sam.
                "}
                </p>
            </div>
        </div>
    )
}
