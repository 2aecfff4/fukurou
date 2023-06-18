import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import QtQuick.Controls.Material
import QtCore
import "components"
import "components" as Components
import com.fukurou

ApplicationWindow {
    id: root
    title: "fukurou"
    visible: true
    minimumWidth: 800
    minimumHeight: 600
    width: 1337
    height: 768
    Material.theme: Material.Dark
    Material.accent: Material.BlueGrey
    color: Theme.background
    font.family: "Noto Sans"

    Settings {
        id: settings
        property alias x: root.x
        property alias y: root.y
        property alias width: root.width
        property alias height: root.height
        property alias visibility: root.visibility

        // Split view
        property var splitView
    }

    AbstractListModel {
        id: test_model
        playlist_id: "1234fe1234"
    }

    Page {
        anchors.fill: parent
        ColumnLayout {
            anchors.fill: parent
            spacing: 0

            // handle: #TODO: Maybe change handle, or maybe not?

            // https://doc.qt.io/qt-6/qml-qtquick-controls2-splitview.html#serializing-splitview-s-state
            SplitView {
                id: splitView
                Layout.fillWidth: true
                Layout.fillHeight: true
                orientation: Qt.Horizontal
                spacing: 4

                // Playlist view?
                Page {
                    id: leftPage
                    implicitWidth: parent.width / 4
                    SplitView.minimumWidth: parent.width / 5

                    Rectangle {
                        anchors.fill: parent
                        color: Theme.background
                    }

                    Shortcut {
                        id: sendShortcut
                        sequences: ["Space"]
                        onActivated: {
                        }
                    }

                    Button {
                        id: sendAction
                        text: "Hello!"
                        onClicked: {
                        }
                        Layout.alignment: Qt.AlignRight | Qt.AlignBottom
                    }
                    // Text {
                    //     id: token_id
                    //     text: "N/A"
                    //     color: "white"
                    // }
                    // 
                    // Text {
                    //     text: test_model.loading ? "LOADING" : "LOADED"
                    //     color: "white"
                    // }
                    BusyIndicator {
                        id: loadingIndicator
                        anchors.centerIn: parent
                        visible: true // #TODO:
                    }
                }

                Page {
                    id: rightPage
                    SplitView.minimumWidth: parent.width / 4

                    Rectangle {
                        anchors.fill: parent
                        color: Theme.background
                    }
                }
            }

            BottomDrawer {
                height: 64 * Screen.devicePixelRatio
                Layout.preferredHeight: 64 * Screen.devicePixelRatio
                Layout.fillWidth: true
                thumbnail: "qrc:/resources/qml/icons/gigachad.png" // #TODO:
            }
        }
    }
}
