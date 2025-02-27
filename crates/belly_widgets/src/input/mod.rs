pub mod button;
pub mod slider;
pub mod text;

use bevy::prelude::Plugin;

pub mod prelude {
    pub use super::button::prelude::*;
    pub use super::slider::prelude::*;
    pub use super::text::prelude::*;
}

pub struct InputPlugins;
impl Plugin for InputPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(button::ButtonPlugin);
        app.add_plugin(slider::SliderPlugin);
        app.add_plugin(text::TextInputPlugin);
    }
}
