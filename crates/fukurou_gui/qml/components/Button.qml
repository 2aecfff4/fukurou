import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC
import QtCore
import QtQuick.Controls.Material
import com.fukurou

Button {
    id: root
    property bool fadeWhenDisabled: true

    Material.foreground: Theme.button.foreground
    hoverEnabled: root.enabled
    scale: root.down ? 0.970 : 1.0

    Behavior on scale  {
        NumberAnimation {
            duration: 100
        }
    }

    Keys.onPressed: e => {
        if (e.key == Qt.Key_Enter || e.key == Qt.Key_Return) {
            root.clicked();
        }
    }

    background: Rectangle {
        anchors.fill: parent
        border.width: 0
        radius: Theme.button.radius

        color: {
            if (root.hovered || root.activeFocus) {
                //return Qt.lighter(Theme.button.hoverBackground, 1.2);
                return Theme.button.hoverBackground;
            } else {
                return Theme.button.background;
            }
        }

        opacity: {
            if (!parent.enabled && root.fadeWhenDisabled) {
                return 0.75;
            } else if (root.down) {
                return 0.75;
            } else {
                return 1.0;
            }
        }

        Behavior on opacity  {
            NumberAnimation {
                duration: 100
            }
        }
    }
}
