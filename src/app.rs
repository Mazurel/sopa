use crate::tags::{SelectableTag, TagPreferenceSelection, TagSelectionType, Tags};
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
struct Location {
    name: String,
    tags: Tags,
}

#[derive(Clone)]
struct Locations {
    locations: Vec<Location>,
}

impl From<Vec<Location>> for Locations {
    fn from(value: Vec<Location>) -> Self {
        Locations { locations: value }
    }
}

impl Locations {
    pub fn build_tags(&self) -> Tags {
        let mut tags = Tags::new();
        for loc in self.locations.iter() {
            for tag in loc.tags.get_all_tags() {
                tags.define_tag(tag.name.clone());
            }
        }
        tags
    }

    pub fn all_locations<'a>(&'a self) -> &'a Vec<Location> {
        &self.locations
    }
}

fn preference_selection(tags: Tags) -> Html {
    html! {
        <div class="container box is-max-desktop mt-2">
            <TagPreferenceSelection {tags}/>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq, Eq)]
struct LocationViewProps {
    location: Location,
}

#[function_component(LocationView)]
fn location_view(props: &LocationViewProps) -> Html {
    let location = &props.location;
    html!(
        <div class="component is-max-tablet p-5">
            <div class="card">
                <div class="card-header">
                    <div class="card-header-title has-background-info has-text-dark is-size-4 is-capitalized has-text-centered">
                        {location.name.clone()}
                    </div>
                </div>
                <div class="card-content">
                    <div class="component">
                        {t!("location-tags-info").to_string()}
                    </div>
                    <div class="component">
                        {location.tags
                            .get_all_tags()
                            .iter()
                            .map(|t| html!(<SelectableTag
                                tag={(*t).clone()}
                                selection_type={TagSelectionType::Acceptable}
                                interactive={false}
                            />))
                            .collect::<Vec<_>>()
                        }
                    </div>
                </div>
            </div>
        </div>
    )
}

pub fn locations_view(locations: &Locations) -> Html {
    html!(
        <div class="container box is-max-desktop mt-2">
        {
            locations.all_locations()
                .iter()
                .map(|l| html!(<LocationView location={l.clone()}/>))
                .collect::<Vec<_>>()
        }
        </div>
    )
}

#[function_component(LocationFinder)]
pub fn location_finder() -> Html {
    let sample_locations = Locations::from(vec![
        Location {
            name: "OiK Gdańsk".to_string(),
            tags: Tags::new_tags(["gender:male", "gender:female"]),
        },
        Location {
            name: "OiK Gdańsk - Hostel".to_string(),
            tags: Tags::new_tags(["gender:male", "gender:female", "type:hostel"]),
        },
    ]);

    html! {
        <div>
            <div class="container">
                {preference_selection(sample_locations.build_tags())}
            </div>
            <div class="container">
                {locations_view(&sample_locations)}
            </div>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <div class="is-primary">
                <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                    <div class="navbar-brand">
                        <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                            <span aria-hidden="false"></span>
                            <span aria-hidden="false"></span>
                            <span aria-hidden="false"></span>
                            <span aria-hidden="false"></span>
                        </a>
                    </div>
                    <div class="navbar-menu">
                        <div class="navbar-start">
                            <div class="navbar-item">
                                {"SHOB"}
                            </div>
                        </div>
                        <div class="navbar-end">
                            <a class="navbar-item">
                                {t!("navbar:location-finder")}
                            </a>
                            <a class="navbar-item">
                                {t!("navbar:about")}
                            </a>
                        </div>
                    </div>
                </nav>
            </div>
            <div class="container">
                <LocationFinder/>
            </div>
        </div>
    }
}
