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

use gtk::{self, ffi};
use std::default::Default;
use std::ffi::CString;

pub struct RecentFilterInfo {
    contains: gtk::RecentFilterFlags,
    uri: String,
    display_name: String,
    mime_type: String,
    applications: Vec<String>,
    groups: Vec<String>,
    age: i32
}

impl RecentFilterInfo {
    #[doc(hidden)]
    pub fn from_c(ptr: *mut ffi::C_GtkRecentFilterInfo) -> RecentFilterInfo {
        if ptr.is_null() {
            Default::default()
        } else {
            let mut tmp_app = Vec::new();
            let mut tmp_groups = Vec::new();
            let mut count = 0;

            unsafe {
                loop {
                    let tmp = (*ptr).applications.offset(count);

                    if tmp.is_null() {
                        break;
                    }
                    count = count + 1;
                    tmp_app.push(String::from_utf8_lossy(::std::ffi::CStr::from_ptr(*tmp).to_bytes()).to_string());
                }
                count = 0;
                loop {
                    let tmp = (*ptr).groups.offset(count);

                    if tmp.is_null() {
                        break;
                    }
                    count = count + 1;
                    tmp_groups.push(String::from_utf8_lossy(::std::ffi::CStr::from_ptr(*tmp).to_bytes()).to_string());
                }
                RecentFilterInfo {
                    contains: (*ptr).contains,
                    uri: String::from_utf8_lossy(::std::ffi::CStr::from_ptr((*ptr).uri).to_bytes()).to_string(),
                    display_name: String::from_utf8_lossy(::std::ffi::CStr::from_ptr((*ptr).display_name).to_bytes()).to_string(),
                    mime_type: String::from_utf8_lossy(::std::ffi::CStr::from_ptr((*ptr).mime_type).to_bytes()).to_string(),
                    applications: tmp_app,
                    groups: tmp_groups,
                    age: (*ptr).age
                }
            }
        }
    }

    #[doc(hidden)]
    pub fn get_ffi(&self) -> ffi::C_GtkRecentFilterInfo {
        let mut t_app = Vec::with_capacity(self.applications.len());
        let mut t_groups = Vec::with_capacity(self.groups.len());

        for tmp in self.applications.iter() {
            let c = CString::new(tmp.as_bytes()).unwrap();

            t_app.push(c.as_ptr());
        }
        for tmp in self.groups.iter() {
            let c = CString::new(tmp.as_bytes()).unwrap();

            t_groups.push(c.as_ptr());
        }
        let c_uri = CString::new(self.uri.as_bytes()).unwrap();
        let c_display_name = CString::new(self.display_name.as_bytes()).unwrap();
        let c_mime_type = CString::new(self.mime_type.as_bytes()).unwrap();

        ffi::C_GtkRecentFilterInfo {
            contains: self.contains,
            uri: c_uri.as_ptr(),
            display_name: c_display_name.as_ptr(),
            mime_type: c_mime_type.as_ptr(),
            applications: t_app.as_ptr(),
            groups: t_groups.as_ptr(),
            age: self.age
        }
    }
}

impl Default for RecentFilterInfo {
    fn default() -> RecentFilterInfo {
        RecentFilterInfo {
            contains: gtk::RecentFilterFlags::URI,
            uri: String::new(),
            display_name: String::new(),
            mime_type: String::new(),
            applications: Vec::new(),
            groups: Vec::new(),
            age: 0i32
        }
    }
}
