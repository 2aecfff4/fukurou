import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import QtQuick.Controls.Material
import QtCore
import Qt5Compat.GraphicalEffects
import "components"
import com.fukurou

Item {
    id: root
    required property string thumbnail
    signal onTrackIconClicked

    Rectangle {
        anchors.fill: parent
        color: Theme.statusBackgroundColor
    }

    RowLayout {
        anchors {
            fill: parent
            leftMargin: 20
            rightMargin: 20
        }

        spacing: 15
        Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter

        // Track image
        Item {
            property double imageSize: root.height - (10 * 2)

            Layout.maximumWidth: imageSize
            Layout.preferredWidth: imageSize
            Layout.maximumHeight: imageSize
            Layout.minimumHeight: imageSize
            Layout.alignment: Qt.AlignVCenter

            RoundedImage {
                id: mainIcon
                anchors.fill: parent
                visible: !loadingIndicator.visible
                source: root.thumbnail
                radius: 6
            }

            BusyIndicator {
                id: loadingIndicator
                anchors.centerIn: parent
                visible: false // #TODO:
            }

            MouseArea {
                id: trackClick
                anchors.fill: parent
                hoverEnabled: true
                acceptedButtons: Qt.NoButton
                cursorShape: Qt.PointingHandCursor
                onClicked: root.onTrackIconClicked()
            }
        }

        // Track details
        ColumnLayout {
            property int trackDetailsWidth: 256
            Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
            Layout.maximumWidth: trackDetailsWidth
            // Layout.minimumWidth: trackDetailsWidth

            // Animating labels: https://stackoverflow.com/a/49031115
            Label {
                id: trackLabel
                Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                // Layout.fillWidth: true
                text: "Track Label"
                color: Theme.textColor
                textFormat: Text.PlainText
                wrapMode: Text.NoWrap
                horizontalAlignment: Text.AlignLeft
                maximumLineCount: 1
                clip: true
                font.pointSize: 10

                MouseArea {
                    id: mouseArea
                    hoverEnabled: true
                    anchors.fill: parent
                    acceptedButtons: Qt.NoButton
                    cursorShape: Qt.PointingHandCursor
                }
            }
            Label {
                id: trackAutor
                Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                //Layout.fillWidth: true
                text: "Track Author"
                color: Theme.textColor
                textFormat: Text.PlainText
                wrapMode: Text.NoWrap
                horizontalAlignment: Text.AlignLeft
                maximumLineCount: 1
                clip: true
                font.pointSize: 10

                MouseArea {
                    id: asdasd // #TODO:
                    hoverEnabled: true
                    anchors.fill: parent
                    acceptedButtons: Qt.NoButton
                    cursorShape: Qt.PointingHandCursor
                }
            }
        }

        // Player
        ColumnLayout {
            Layout.alignment: Qt.AlignHCenter
            spacing: 0

            // Player controls
            RowLayout {
                Layout.alignment: Qt.AlignVCenter | Qt.AlignHCenter
                property real maxButtonSize: 30.0
                property real largeButtonSize: maxButtonSize
                property real mediumButtonSize: maxButtonSize * 0.75
                property real smallButtonSize: maxButtonSize * 0.5

                RoundButton {
                    property int size: parent.smallButtonSize
                    width: size
                    height: size
                    icon.height: size
                    icon.width: size

                    display: AbstractButton.IconOnly
                    padding: 0
                    flat: true
                    icon.source: "icons/shuffle-solid.svg"
                    icon.color: "grey"
                    background: Rectangle {
                        color: "transparent"
                    }
                }
                RoundButton {
                    property int size: parent.mediumButtonSize
                    width: size
                    height: size
                    icon.height: size
                    icon.width: size

                    display: AbstractButton.IconOnly
                    padding: 0
                    flat: true
                    icon.source: "icons/backward-solid.svg"
                    icon.color: "white"
                    background: Rectangle {
                        color: "transparent"
                    }
                    scale: down ? 0.9 : 1.0

                    Behavior on scale  {
                        NumberAnimation {
                            duration: 100
                        }
                    }
                }
                RoundButton {
                    property int size: parent.largeButtonSize
                    width: size
                    height: size
                    icon.height: size
                    icon.width: size

                    display: AbstractButton.IconOnly
                    padding: 0
                    flat: true
                    icon.source: "icons/circle-play-solid.svg"
                    icon.color: "white"
                    background: Rectangle {
                        color: "transparent"
                    }
                    scale: down ? 0.9 : 1.0

                    Behavior on scale  {
                        NumberAnimation {
                            duration: 100
                        }
                    }
                }
                RoundButton {
                    property int size: parent.mediumButtonSize
                    width: size
                    height: size
                    icon.height: size
                    icon.width: size

                    display: AbstractButton.IconOnly
                    padding: 0
                    flat: true
                    icon.source: "icons/forward-solid.svg"
                    icon.color: "white"
                    background: Rectangle {
                        color: "transparent"
                    }
                    scale: down ? 0.9 : 1.0

                    Behavior on scale  {
                        NumberAnimation {
                            duration: 100
                        }
                    }
                }
                RoundButton {
                    property int size: parent.smallButtonSize
                    width: size
                    height: size
                    icon.height: size
                    icon.width: size

                    display: AbstractButton.IconOnly
                    padding: 0
                    flat: true
                    icon.source: "icons/repeat-solid.svg"
                    icon.color: "grey"
                    background: Rectangle {
                        color: "transparent"
                    }
                }
            }

            // Track progress
            RowLayout {
                Label {
                    id: currentTimeLabel
                    Layout.alignment: Qt.AlignVCenter
                    text: "0:00"
                    textFormat: Text.PlainText
                    maximumLineCount: 1
                }

                // https://doc.qt.io/qt-6/qtquickcontrols-customize.html#customizing-slider
                Slider {
                    id: control
                    Layout.fillWidth: true
                    from: 0
                    value: 0
                    to: 1.0

                    background: Rectangle {
                        x: control.leftPadding
                        y: control.topPadding + control.availableHeight / 2 - height / 2
                        implicitHeight: 2
                        width: control.availableWidth
                        height: implicitHeight
                        radius: 2
                        color: "#bdbebf"

                        Rectangle {
                            width: control.visualPosition * parent.width
                            height: parent.height
                            color: "#21be2b"
                            radius: 2
                        }
                    }

                    handle: Rectangle {
                        x: control.leftPadding + control.visualPosition * (control.availableWidth - width)
                        y: control.topPadding + control.availableHeight / 2 - height / 2
                        implicitWidth: 12
                        implicitHeight: 12
                        radius: 6
                        color: control.pressed ? "#f0f0f0" : "#f6f6f6"
                        border.color: "#bdbebf"
                    }
                }

                Label {
                    id: totalTimeLabel
                    Layout.alignment: Qt.AlignVCenter
                    text: "4:20"
                    textFormat: Text.PlainText
                    maximumLineCount: 1
                }
            }
        }
    }
}
