mod preview;
mod world;

use std::{cell::RefCell, pin::Pin, rc::Rc};

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{
    QGuiApplication, QQmlApplicationEngine, QQmlEngine, QQmlImageProviderBase, QString, QUrl,
};
use typst::layout::PagedDocument;

use crate::{preview::PreviewProvider, world::ComposeWorld};

pub struct State {
    world: ComposeWorld,
}

impl State {
    fn new() -> Self {
        State {
            world: ComposeWorld::new(),
        }
    }
}

fn main() {
    cxx_qt::init_qml_module!("org.kde.compose"); // remove later

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    let preview = PreviewProvider::new();

    QGuiApplication::set_desktop_file_name(&QString::from("org.kde.kontrast"));

    let content = r#"
        = Typst from rust!
        #pagebreak()
        = New Page
        "#;

    let mut state = State::new();
    state.world.set_source(content.into());

    let document: PagedDocument = typst::compile(&state.world)
        .output
        .expect("Error Compiling typst");

    // for (index, page) in document.pages.iter().enumerate() {
    //     let img = typst_render::render(&page, 1.0);
    //     img.save_png(format!("image{}.png", index))
    //         .expect("error writing image.");
    // }

    if let Some(mut engine) = engine.as_mut() {
        // KLocalizedContext::initialize_engine(engine.as_mut().upcast_pin());
        let mut qmlengine: Pin<&mut QQmlEngine> = engine.as_mut().upcast_pin();

        unsafe {
            let provider: *const QQmlImageProviderBase =
                PreviewProvider::upcast_ptr(preview.into_raw());

            qmlengine
                .as_mut()
                .add_image_provider(&QString::from("preview"), provider.cast_mut());
        }

        engine.load(&QUrl::from("qrc:/qt/qml/org/kde/compose/src/qml/Main.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
