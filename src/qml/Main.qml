import QtQuick
import QtQuick.Controls as Controls
import org.kde.kirigami as Kirigami

Kirigami.ApplicationWindow {
    Controls.Label {
        text: "Compose"
    }

    Image {
        asynchronous: true
        cache: false
        source: "image://preview/cat"
    }
}
