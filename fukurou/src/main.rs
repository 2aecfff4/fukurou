//#![windows_subsystem = "windows"]
use qmetaobject::{prelude::*, qtcore::core_application::QCoreApplication, QQuickStyle};

fn main() {
    fukurou_gui::qt_resources::qt_resources();
    qmetaobject::log::init_qt_to_rust();
    env_logger::init();

    QQuickStyle::set_style("Material");
    QCoreApplication::set_organization_domain("com".into());
    QCoreApplication::set_organization_name("fukurou".into());
    QCoreApplication::set_application_name("fukurou".into());

    let mut engine = QmlEngine::new();
    fukurou_gui::register(&mut engine);

    engine.load_file("qrc:/resources/qml/main.qml".into());
    engine.exec();

    // cpp!(unsafe [] {
    //     auto f = QGuiApplication::font();
    //     f.setFamily("Noto Sans");
    //     QGuiApplication::setFont(f);
    // });
}
