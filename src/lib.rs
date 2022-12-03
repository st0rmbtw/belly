use bevy::prelude::*;

// plugin
use bevy_elements_core::ElementsCorePlugin;
use bevy_elements_widgets::WidgetsPlugin;

// structs
pub use bevy_elements_core::eml::EmlAsset;
pub use bevy_elements_core::eml::EmlScene;
pub use bevy_elements_core::ess::StyleSheet;

// macros
pub use bevy_elements_core::bind;
pub use bevy_elements_macro::eml;

// traits
pub use bevy_elements_core::eml::build::WidgetBuilder;
pub use bevy_elements_core::eml::content::IntoContent;
pub use bevy_elements_core::ExpandElementsExt;
pub use bevy_elements_core::WithElements;

// widgets
pub use bevy_elements_widgets::prelude::*;

pub struct ElementsPlugin;
impl Plugin for ElementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ElementsCorePlugin);
        app.add_plugin(WidgetsPlugin);
    }
}

pub mod build {
    pub use super::*;
    pub use bevy_elements_core::ElementBuilder;
    pub use bevy_elements_core::ElementContext;
    pub use bevy_elements_core::Elements;
    pub use bevy_elements_core::RegisterWidgetExtension;
}
