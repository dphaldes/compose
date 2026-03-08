use typst::{
    Library, LibraryExt, World,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_kit::fonts::{FontSlot, Fonts};

pub struct ComposeWorld {
    main_file: FileId,
    library: LazyHash<Library>,
    font_book: LazyHash<FontBook>,
    fonts: Vec<FontSlot>,
}

impl ComposeWorld {
    pub fn new() -> Self {
        let main_file = FileId::new(None, VirtualPath::new("main.typ"));
        let library = LazyHash::new(Library::builder().build());
        let loaded = Fonts::searcher().include_system_fonts(true).search();
        let font_book = LazyHash::new(loaded.book);
        let fonts = loaded.fonts;

        Self {
            main_file,
            library,
            font_book,
            fonts,
        }
    }
}

impl World for ComposeWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.font_book
    }

    fn main(&self) -> FileId {
        self.main_file
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        Err(FileError::Other(None))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::Other(None))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index)?.get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        Some(Datetime::from_ymd(1970, 1, 1).unwrap()) // FIXME
    }
}
