use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};
use typst::layout::PagedDocument;

use crate::{bridge::PreviewProvider, world::ComposeWorld};

mod bridge;
mod world;

extern crate cxx_qt_lib; // remove later

fn main() {
    cxx_qt::init_crate!(cxx_qt_lib);
    cxx_qt::init_qml_module!("org.kde.compose"); // remove later

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    QGuiApplication::set_desktop_file_name(&QString::from("org.kde.kontrast"));

    // let content = r#"
    //     = Typst from rust!
    //     #pagebreak()
    //     = New Page
    //     "#;
    //
    // let mut world = ComposeWorld::new();
    // world.set_source(content.into());
    //
    // let document: PagedDocument = typst::compile(&world)
    //     .output
    //     .expect("Error Compiling typst");

    // for (index, page) in document.pages.iter().enumerate() {
    //     let img = typst_render::render(&page, 1.0);
    //     img.save_png(format!("image{}.png", index))
    //         .expect("error writing image.");
    // }

    if let Some(mut engine) = engine.as_mut() {
        // KLocalizedContext::initialize_engine(engine.as_mut().upcast_pin());
        unsafe {
            engine.as_mut().add_image_provider(
                &QString::from("preview"),
                PreviewProvider::new().pin_mut().cast_to_base(),
            );
        }
        engine.load(&QUrl::from("qrc:/qt/qml/org/kde/compose/src/qml/Main.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
