use core::pin::Pin;
use cxx::UniquePtr;
use cxx_qt::Initialize;

#[cxx_qt::bridge]
mod ffi {

    extern "C++" {
        include!("compose/PixmapImageProvider.h");
        type PixmapImageProvider;

        type QQmlImageProviderBase = cxx_qt_lib::QQmlImageProviderBase;
        type QString = cxx_qt_lib::QString;
        type QSize = cxx_qt_lib::QSize;

        include!("cxx-qt-lib/qimage.h");
        type QImage = cxx_qt_lib::QImage;
    }

    extern "RustQt" {

        #[qobject]
        #[base = PixmapImageProvider]
        type PreviewProvider = super::PreviewProviderType;

        #[cxx_virtual]
        #[cxx_name = "requestImage"]
        unsafe fn request_image(
            self: &PreviewProvider,
            id: &QString,
            size: *const QSize,
            requested_size: &QSize,
        ) -> QImage;
    }

    unsafe extern "RustQt" {

        #[inherit]
        #[cxx_name = "castToBase"]
        pub unsafe fn cast_to_base(self: Pin<&mut PreviewProvider>) -> *mut QQmlImageProviderBase;
    }

    impl cxx_qt::Constructor<()> for PreviewProvider {}

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[cxx_name = "make_unique"]
        #[doc(hidden)]
        fn new_preview_provider() -> UniquePtr<PreviewProvider>;
    }
}

#[derive(Default)]
pub struct PreviewProviderType;
pub use ffi::PreviewProvider;

impl Initialize for PreviewProvider {
    fn initialize(self: Pin<&mut Self>) {}
}

use cxx_qt_lib::QImage;
use cxx_qt_lib::QImageFormat;
use cxx_qt_lib::QSize;
use cxx_qt_lib::QString;

impl PreviewProvider {
    pub fn new() -> UniquePtr<Self> {
        ffi::new_preview_provider()
    }

    unsafe fn request_image(
        self: &Self,
        id: &QString,
        size: *const QSize,
        requested_size: &QSize,
    ) -> QImage {
        QImage::from_width_height_and_format(50, 70, QImageFormat::Format_Mono)
    }
}
