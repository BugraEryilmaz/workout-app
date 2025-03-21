use leptos::prelude::*;

use crate::utils::models::Workout;
use crate::utils::video_metadata::get_thumbnail;

stylance::import_style!(
    #[allow(dead_code)]
    card_style,
    "../create_workout/card.css"
);

stylance::import_style!(
    #[allow(dead_code)]
    recents_style,
    "recents.css"
);

#[component]
pub fn RecentsCard(workout: Workout) -> impl IntoView {
    view! {
            <div class=stylance::classes!(card_style::card_thumbnail, recents_style::recents_card)>
                <img class=card_style::card_img src={ get_thumbnail(&workout.link).unwrap_or_else(|_| "https://miro.medium.com/v2/resize:fit:532/1*69aTahESxdQG3uHV8Y6Row.png".to_string()) } alt="Card image cap"/>
                <p class=card_style::card_duration>{ workout.duration/60 }:{ format!("{:02}", workout.duration%60) }</p>
            </div>
    }
}
