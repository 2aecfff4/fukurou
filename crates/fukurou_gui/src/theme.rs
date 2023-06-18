use std::cell::RefCell;

use qmetaobject::prelude::*;

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct ButtonTheme {
    base: qt_base_class!(trait QObject),
    onThemeChanged: qt_signal!(),

    radius: qt_property!(f32; NOTIFY onThemeChanged),
    foreground: qt_property!(QString; NOTIFY onThemeChanged),
    background: qt_property!(QString; NOTIFY onThemeChanged),
    hoverBackground: qt_property!(QString; NOTIFY onThemeChanged),
}

impl ButtonTheme {
    pub fn new() -> Self {
        Self {
            radius: 2.0,
            foreground: "#e0def4".into(),
            background: "#191724".into(),
            hoverBackground: "#23212e".into(),
            ..Default::default()
        }
    }
}

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct Theme {
    base: qt_base_class!(trait QObject),
    onThemeChanged: qt_signal!(),

    foreground: qt_property!(QString; NOTIFY onThemeChanged),
    background: qt_property!(QString; NOTIFY onThemeChanged),
    textColor: qt_property!(QString; NOTIFY onThemeChanged),

    statusBackgroundColor: qt_property!(QString; NOTIFY onThemeChanged),

    button: qt_property!(RefCell<ButtonTheme>; NOTIFY onThemeChanged),
}

impl Theme {
    pub fn new() -> Self {
        let mut theme = Self {
            button: RefCell::new(ButtonTheme::new()),
            ..Default::default()
        };
        theme.set_theme("dark");
        theme
    }

    pub fn set_theme(&mut self, theme: &str) {
        match theme {
            "dark" => {
                self.background = "#191724".into();
                self.foreground = "#e0def4".into();
                self.textColor = "#e0def4".into();
                self.statusBackgroundColor = "#1f1d2e".into();

                // self.background = "#242933".into();
                // self.foregroundColor = "#d8dee9".into();
                // self.textColor = "#d8dee9".into();
                // self.statusBackgroundColor = "#3b4252".into();
            }
            "light" => {
                log::info!("`light` theme is currently unimplemented.");
                self.set_theme("dark");
            }
            _ => {
                log::info!(
                    "Invalid theme name. Expected `dark` or `light` got `{}`",
                    theme
                );

                self.set_theme("dark");
            }
        }

        self.onThemeChanged();
    }
}
