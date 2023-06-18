use std::collections::HashMap;

use qmetaobject::{prelude::*, QAbstractListModel, SimpleListItem};

use crate::helpers;

// pub struct ListModel<T: SimpleListItem + 'static> {}

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct AbstractListModel {
    base: qt_base_class!(trait QAbstractListModel),
    // Status
    loading: qt_property!(bool; READ loading WRITE set_loading NOTIFY loading_changed),
    loading_changed: qt_signal!(),

    // Input
    playlist_id: qt_property!(QString; WRITE set_playlist_id NOTIFY playlist_id_changed),
    playlist_id_changed: qt_signal!(),

    // Output
    playlist_name: qt_property!(QString; NOTIFY playlist_name_changed),
    playlist_name_changed: qt_signal!(),

    thumbnail_url: qt_property!(QString; NOTIFY thumbnail_url_changed),
    thumbnail_url_changed: qt_signal!(),
}

impl AbstractListModel {
    pub fn loading(&self) -> bool {
        self.loading
    }

    pub fn set_loading(&mut self, loading: bool) {
        if self.loading == loading {
            return;
        }

        self.loading = loading;
        self.loading_changed();
    }

    fn set_playlist_id(&mut self, playlist_id: QString) {
        self.playlist_id = playlist_id;
        self.set_loading(true);

        let callback = helpers::queued_callback_mut(self, |this, _: ()| {
            this.begin_reset_model();
            // set vec
            this.end_reset_model();
            this.set_loading(false);

            //
            this.playlist_name_changed();
            this.thumbnail_url_changed();
        });

        // #TODO: Fetch playlist
        callback(());
    }
}

impl QAbstractListModel for AbstractListModel {
    fn row_count(&self) -> i32 {
        todo!()
    }

    fn data(&self, index: QModelIndex, _role: i32) -> QVariant {
        let idx = index.row();
        // if idx >= 0 && (idx as usize) < self.values.len() {
        //     self.values[idx as usize].get(role - USER_ROLE).clone()
        // } else {
        //     QVariant::default()
        // }
        todo!()
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        todo!()
    }
}
