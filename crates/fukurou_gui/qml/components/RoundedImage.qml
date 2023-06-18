import QtQuick
import QtQuick.Window
import Qt5Compat.GraphicalEffects

Item {
    id: root
    property alias radius: mask.radius
    property alias source: image.source

    Image {
        id: image
        source: source
        anchors.fill: parent
        fillMode: Image.PreserveAspectCrop
        asynchronous: true
        sourceSize.width: parent.implicitWidth * Screen.devicePixelRatio
        visible: false
    }

    Rectangle {
        id: mask
        anchors.fill: parent
        visible: false
    }

    OpacityMask {
        anchors.fill: image
        source: image
        maskSource: mask
    }
}
