#[cxx_qt::bridge]
mod qobject {
    extern "C++" {
        include!(<QtQuick/QQuickImageProvider>);
        type QQuickImageProvider;

        include!("cxx-qt-lib/qqmlimageproviderbase.h");

        #[namespace = "rust::cxxqtlib1"]
        type QQmlImageProviderBaseImageType = cxx_qt_lib::QQmlImageProviderBaseImageType;

    }

    extern "RustQt" {
        #[qobject]
        #[base = QQuickImageProvider]
        type PreviewProvider = super::PreviewProviderType;
    }

    impl cxx_qt::Constructor<(QQmlImageProviderBaseImageType,)> for PreviewProvider {}
}

#[derive(Default)]
struct PreviewProviderType;
use ffi::PreviewProvider;

use cxx_qt_lib::QQmlImageProviderBaseImageType;

impl cxx_qt::Constructor<(QQmlImageProviderBaseImageType,)> for PreviewProvider {
    type NewArguments = ();
    type BaseArguments = (QQmlImageProviderBaseImageType,);
    type InitializeArguments;

    fn route_arguments(
        args: (QQmlImageProviderBaseImageType,),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), args, ())
    }

    fn new(_: ()) -> PreviewProviderType {
        PreviewProviderType::default()
    }
}
