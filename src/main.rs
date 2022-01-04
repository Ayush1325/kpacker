use ki18n::klocalizedcontext::KLocalizedContext;
use log::info;
use qmetaobject::prelude::*;
use qmetaobject::QUrl;

qrc!(root_qml,
    "" {
        "qml/main.qml" as "main.qml",
    }
);

fn main() {
    qmetaobject::log::init_qt_to_rust();
    env_logger::init();
    info!("Log⋅Test");

    let mut engine = QmlEngine::new();
    KLocalizedContext::init_from_engine(&engine);

    root_qml();

    engine.load_url(QUrl::from(QString::from("qrc:///main.qml")));
    engine.exec();
}
