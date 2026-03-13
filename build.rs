use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("org.kde.compose").qml_file("src/qml/Main.qml"))
        .file("src/bridge.rs")
        .build();
}
