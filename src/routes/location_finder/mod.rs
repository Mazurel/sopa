mod tags;

use tags::{SelectableTag, TagPreferenceSelection, TagSelectionType, Tags};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Location {
    name: String,
    tags: Tags,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locations {
    pub locations: Vec<Location>,
}

impl From<Vec<Location>> for Locations {
    fn from(value: Vec<Location>) -> Self {
        Locations { locations: value }
    }
}

impl Into<Vec<Location>> for Locations {
    fn into(self) -> Vec<Location> {
        self.locations
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

    pub fn all_locations_in_order(&self, tags_preference: &Tags) -> Vec<Location> {
        let mut locations = self.locations.clone();
        let mut locations_overlaps: Vec<_> = self
            .locations
            .clone()
            .into_iter()
            .map(|loc| loc.tags.overlap(tags_preference))
            .collect();

        let mut locations_result = Vec::new();

        while locations.len() > 0 {
            let i = locations_overlaps
                .iter()
                .enumerate()
                .min_by(|(_, o1), (_, o2)| o2.total_cmp(o1))
                .expect("Locations should not be empty due to loop condition.")
                .0;

            locations_overlaps.remove(i);
            locations_result.push(locations.remove(i));
        }

        locations_result
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

    let locations_state = use_state(|| sample_locations.clone());

    let on_tag_preference_changed = {
        let locations_state = locations_state.clone();
        use_callback(
            move |tag_preference: Tags, _| {
                let new_locations = locations_state
                    .all_locations_in_order(&tag_preference)
                    .into();
                locations_state.set(new_locations);
            },
            (),
        )
    };

    log::info!("locs: {locations_state:?}");

    html! {
        <div class="container">
            <div class="container">
                <TagPreferenceSelection tags={sample_locations.build_tags()} {on_tag_preference_changed}/>
            </div>
            <div class="container">
                {locations_view(&locations_state)}
            </div>
        </div>
    }
}
