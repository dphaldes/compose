use cxx_qt::casting::Upcast;
use std::pin::Pin;

use cxx_qt_lib::{QColor, QImage, QImageFormat, QQmlImageProviderBaseImageType, QSize, QString};

#[cxx_qt::bridge]
mod ffi {

    extern "C++" {
        #[namespace = "rust::cxxqtlib1"]
        type QQmlImageProviderBaseImageType = cxx_qt_lib::QQmlImageProviderBaseImageType;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qsize.h");
        type QSize = cxx_qt_lib::QSize;

        include!("cxx-qt-lib/qimage.h");
        type QImage = cxx_qt_lib::QImage;
    }

    extern "C++Qt" {
        include!("cxx-qt-lib/qqmlimageproviderbase.h");
        type QQmlImageProviderBase = cxx_qt_lib::QQmlImageProviderBase;

        include!(<QtQuick/QQuickImageProvider>);

        #[qobject]
        #[base = QQmlImageProviderBase]
        type QQuickImageProvider;
    }

    extern "RustQt" {

        #[qobject]
        #[base = QQuickImageProvider]
        type PreviewProvider = super::PreviewProviderType;

        #[cxx_override]
        #[cxx_name = "requestImage"]
        unsafe fn request_image(
            self: Pin<&mut PreviewProvider>,
            id: &QString,
            size: *mut QSize,
            requested_size: &QSize,
        ) -> QImage;

    }

    extern "RustQt" {}

    impl
        cxx_qt::Constructor<
            (QQmlImageProviderBaseImageType,),
            BaseArguments = (QQmlImageProviderBaseImageType,),
        > for PreviewProvider
    {
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[cxx_name = "make_unique"]
        #[doc(hidden)]
        fn new_preview_provider(
            img_type: QQmlImageProviderBaseImageType,
        ) -> UniquePtr<PreviewProvider>;
    }
}

pub struct PreviewProviderType;
pub use ffi::PreviewProvider;

cxx_qt::impl_transitive_cast!(
    PreviewProvider,
    ffi::QQuickImageProvider,
    ffi::QQmlImageProviderBase
);

impl PreviewProvider {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::new_preview_provider(QQmlImageProviderBaseImageType::Image)
    }

    pub fn request_image(
        self: Pin<&mut Self>,
        id: &QString,
        _size: *const QSize,
        requested_size: &QSize,
    ) -> QImage {
        let width = if requested_size.width() > 0 {
            requested_size.width()
        } else {
            100
        };

        let height = if requested_size.height() > 0 {
            requested_size.height()
        } else {
            100
        };

        println!("Rust: request_image size {}, {}", width, height);
        let mut qimage =
            QImage::from_width_height_and_format(width, height, QImageFormat::Format_ARGB32);
        qimage.fill(&QColor::from_rgb(255, 0, 0));
        qimage
    }
}

impl cxx_qt::Constructor<(QQmlImageProviderBaseImageType,)> for PreviewProvider {
    type NewArguments = ();
    type BaseArguments = (QQmlImageProviderBaseImageType,);
    type InitializeArguments = ();

    fn route_arguments(
        args: (QQmlImageProviderBaseImageType,),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), args, ())
    }

    fn new(_args: Self::NewArguments) -> PreviewProviderType {
        PreviewProviderType {}
    }
}
