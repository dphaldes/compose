use typst::layout::PagedDocument;

use crate::world::ComposeWorld;

mod world;

fn main() {
    let content = r#"
        = Typst from rust!
        #pagebreak()
        = New Page
        "#;

    let mut world = ComposeWorld::new();
    world.set_source(content.into());

    let document: PagedDocument = typst::compile(&world)
        .output
        .expect("Error Compiling typst");

    for (index, page) in document.pages.iter().enumerate() {
        let img = typst_render::render(&page, 1.0);
        img.save_png(format!("image{}.png", index))
            .expect("error writing image.");
    }
}
