pub mod app_state;
pub mod helpers;
pub mod model;
pub mod qt_resources;
pub mod theme;

use crate::{app_state::AppState, theme::Theme};
use qmetaobject::{QObjectBox, QmlEngine};

pub fn register(engine: &mut QmlEngine) {
    qmetaobject::qml_register_singleton_instance(
        cstr::cstr!("com.fukurou"),
        1,
        0,
        cstr::cstr!("Theme"),
        Theme::new(),
    );
    qmetaobject::qml_register_type::<crate::model::playlist_model::AbstractListModel>(
        cstr::cstr!("com.fukurou"),
        1,
        0,
        cstr::cstr!("AbstractListModel"),
    );
    // engine.load_file("qrc:/resources/qml/main.qml".into());

    // let app_state = QObjectBox::new(AppState::default());
    // engine.set_object_property("AppState".into(), app_state.pinned());
}
