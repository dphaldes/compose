#pragma once

#include <QtQml/QQmlImageProviderBase>
#include <QtQuick/QQuickImageProvider>

class PixmapImageProvider : public QQuickImageProvider {

public:
  PixmapImageProvider() : QQuickImageProvider(QQuickImageProvider::Image) {}

  auto castToBase() -> QQmlImageProviderBase * { return this; }
};
