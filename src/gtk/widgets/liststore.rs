// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use gtk::ffi;

pub struct ListStore {
    pointer: *mut ffi::C_GtkListStore
}

impl ListStore {
    pub fn new(column_types: Vec<u64>) -> Option<ListStore> {
        let tmp_pointer = unsafe { ffi::gtk_list_store_newv(column_types.len().to_i32().unwrap(), column_types.as_slice()) };
        check_pointer!(tmp_pointer, ListStore)
    }

    pub fn set_column_types(&self, column_types: Vec<u64>) {
        unsafe { ffi::gtk_list_store_set_column_types(self.pointer, column_types.len().to_i32().unwrap(), column_types.as_slice()) }
    }

    // TODO: set_value

    // TODO: set

    // TODO: set_values

    // TODO: remove

    // TODO: insert

    // TODO: insert_before

    // TODO: insert_after

    // TODO: insert_with_values

    // TODO: prepend

    // TODO: append

    // TODO: clear

    // TODO: iter_is_valid

    // TODO: reorder

    // TODO: swap

    // TODO: move_before

    // TODO: move_after

    #[doc(hidden)]
    pub fn get_pointer(&self) -> *mut ffi::C_GtkListStore {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_liststore: *mut ffi::C_GtkListStore) -> ListStore {
        ListStore {
            pointer: c_liststore
        }
    }
}
