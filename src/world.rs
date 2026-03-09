use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

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
    open_source: Option<Source>, //does this need to be here ?
    library: LazyHash<Library>,
    font_book: LazyHash<FontBook>,
    fonts: Vec<FontSlot>,
    files: Arc<Mutex<HashMap<FileId, File>>>,
}

#[derive(Clone)]
struct File {
    source: Option<Source>,
    data: Bytes,
}

impl File {
    fn new(source: Option<Source>, data: Vec<u8>) -> Self {
        Self {
            source,
            data: Bytes::new(data),
        }
    }

    fn source(&mut self, id: FileId) -> FileResult<Source> {
        let source = if let Some(source) = &self.source {
            source
        } else {
            let contents = std::str::from_utf8(&self.data).map_err(|_| FileError::InvalidUtf8)?;
            let contents = contents.trim_start_matches('\u{feff}');
            let source = Source::new(id, contents.into());
            self.source.insert(source)
        };
        Ok(source.clone())
    }
}

impl ComposeWorld {
    pub fn new() -> Self {
        let library = LazyHash::new(Library::builder().build());
        let loaded = Fonts::searcher().include_system_fonts(true).search();
        let font_book = LazyHash::new(loaded.book);
        let fonts = loaded.fonts;

        Self {
            open_source: None,
            library,
            font_book,
            fonts,
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_source(&mut self, source: String) {
        self.open_source = Some(Source::detached(source));
    }

    pub fn get_file(&self, id: FileId) -> FileResult<File> {
        let mut files = self.files.lock().map_err(|_| FileError::AccessDenied)?;
        if let Some(file) = files.get(&id) {
            return Ok(file.clone());
        }

        let path = id
            .vpath()
            .resolve(&PathBuf::from("../"))
            .ok_or(FileError::AccessDenied)?;

        let content = std::fs::read(&path).map_err(|error| FileError::from_io(error, &path))?;
        Ok(files.entry(id).or_insert(File::new(None, content)).clone())
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
        if let Some(file) = &self.open_source {
            return file.id();
        }
        FileId::new_fake(VirtualPath::new("main.typ"))
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if let Some(file) = &self.open_source {
            if id == file.id() {
                return Ok(file.clone());
            }
        }
        self.get_file(id)?.source(id)
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.get_file(id).map(|file| file.data.clone())
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index)?.get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        Some(Datetime::from_ymd(1970, 1, 1).unwrap()) // FIXME
    }
}
