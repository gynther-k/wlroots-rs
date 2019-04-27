use wayland_sys::server::wl_display as wl_server_display;
use wlroots_sys::{wl_display, wlr_server_decoration_manager, wlr_server_decoration_manager_create,
                  wlr_server_decoration_manager_destroy,
                  wlr_server_decoration_manager_set_default_mode};
pub use wlroots_sys::protocols::server_decoration;

#[derive(Debug)]
pub struct ServerDecorationManager {
    manager: *mut wlr_server_decoration_manager
}

impl ServerDecorationManager {
    pub(crate) unsafe fn new(display: *mut wl_server_display) -> Option<Self> {
        let manager_raw = wlr_server_decoration_manager_create(display as *mut wl_display);

        if !manager_raw.is_null() {
            Some(ServerDecorationManager { manager: manager_raw })
        } else {
            None
        }
    }
}

impl Drop for ServerDecorationManager {
    fn drop(&mut self) {
        unsafe { wlr_server_decoration_manager_destroy(self.manager) }
    }
}
