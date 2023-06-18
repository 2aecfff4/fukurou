use log::info;
use qmetaobject::{prelude::*, QObject};

///
#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct AppState {
    base: qt_base_class!(trait QObject),
    hello: qt_method!(fn(&self)),
}

impl AppState {
    fn hello(&self) {
        info!("AppState::hello");
    }
}
