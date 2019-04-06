use wlroots::{wlroots_dehandle, compositor,
              input::keyboard,
              seat::Capability,
              xkbcommon::xkb::keysyms,
              wlr_key_state::WLR_KEY_PRESSED};

use crate::{CompositorState, Inputs};

#[wlroots_dehandle]
pub fn keyboard_added(compositor_handle: compositor::Handle,
                      keyboard_handle: keyboard::Handle)
                      -> Option<Box<keyboard::Handler>> {
    #[dehandle] let compositor = compositor_handle;
    let CompositorState { ref seat_handle,
                          inputs: Inputs { ref mut keyboards, ..  },
                          .. } = compositor.downcast();
    keyboards.insert(keyboard_handle);
    if keyboards.len() == 1 {
        #[dehandle] let seat = seat_handle;
        let mut cap = seat.capabilities();
        cap.insert(Capability::Keyboard);
        seat.set_capabilities(cap);
    }
    Some(Box::new(KeyboardHandler::default()) as _)
}

#[derive(Default)]
struct KeyboardHandler {
    shift_pressed: bool,
    ctrl_pressed: bool
}

impl keyboard::Handler for KeyboardHandler {
    #[wlroots_dehandle]
    fn on_key(&mut self,
              compositor_handle: compositor::Handle,
              _keyboard_handle: keyboard::Handle,
              key_event: &keyboard::event::Key) {
        for key in key_event.pressed_keys() {
            match key {
                keysyms::KEY_Control_L | keysyms::KEY_Control_R =>
                    self.ctrl_pressed = key_event.key_state() == WLR_KEY_PRESSED,
                keysyms::KEY_Shift_L | keysyms::KEY_Shift_R =>
                    self.shift_pressed = key_event.key_state() == WLR_KEY_PRESSED,
                keysyms::KEY_Escape => {
                    if self.shift_pressed && self.ctrl_pressed {
                        wlroots::compositor::terminate()
                    }
                },
                keysyms::KEY_XF86Switch_VT_1 ..= keysyms::KEY_XF86Switch_VT_12 => {
                    #[dehandle] let compositor = compositor_handle;
                    let backend = compositor.backend_mut();
                    if let Some(mut session) = backend.get_session() {
                        session.change_vt(key - keysyms::KEY_XF86Switch_VT_1 + 1);
                    }
                }
                _ => { /* Do nothing */ }
            }
        }
    }

    #[wlroots_dehandle]
    fn destroyed(&mut self,
                 compositor_handle: compositor::Handle,
                 keyboard_handle: keyboard::Handle) {
        #[dehandle] let compositor = compositor_handle;
        let CompositorState { ref seat_handle,
                              inputs: Inputs { ref mut keyboards, ..  },
                              .. } = compositor.downcast();
        keyboards.remove(&keyboard_handle);
        if keyboards.len() == 0 {
            #[dehandle] let seat = seat_handle;
            let mut cap = seat.capabilities();
            cap.remove(Capability::Keyboard);
            seat.set_capabilities(cap)
        }
    }
}
