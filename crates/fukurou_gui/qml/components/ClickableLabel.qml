import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import QtCore

Item {
    id: root
    required property string text
    property color color: Theme.textColor
    property var wrapMode: Text.Wrap
    property int maximumLineCount: 1
    property var fontWeight: Font.Bold
    property var textFormat: Text.PlainText
    property var horizontalAlignment: Text.AlignLeft
    signal onClicked

    Label {
        id: label
        text: root.text
        color: root.color
        wrapMode: root.wrapMode
        textFormat: root.PlainText
        horizontalAlignment: root.horizontalAlignment
        maximumLineCount: root.maximumLineCount
        font.weight: root.fontWeight
    }

    MouseArea {
        id: mouseArea
        hoverEnabled: true
        anchors.fill: parent
        acceptedButtons: Qt.NoButton
        cursorShape: Qt.PointingHandCursor
        onClicked: root.onClicked()
    }
}
