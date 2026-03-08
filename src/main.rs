use typst_pdf::PdfOptions;

use crate::world::ComposeWorld;

mod world;

fn main() {
    let world = ComposeWorld::new();

    let document = typst::compile(&world)
        .output
        .expect("Error Compiling typst");

    // Output to pdf and svg
    let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
    std::fs::write("./output.pdf", pdf).expect("Error writing PDF.");
    println!("Created pdf: `./output.pdf`");
}
