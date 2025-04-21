/*
Copyright (C) 2025 Mateusz Mazur (Mazurel) <mateusz.mazur@e.email>

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, see
<https://www.gnu.org/licenses/>.
*/

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
