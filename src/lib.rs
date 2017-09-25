////////////////////////////////////////////////////////
// THIS IS AN EXAMPLE OF C COMPATIBLE RAUI PLUGIN     //
// READY TO USE AS SHARED LIBRARY WITH EXTERNAL APPS. //
////////////////////////////////////////////////////////

extern crate raui;
mod server_singleton;

use std::os::raw::c_char;
use std::ffi::CStr;
use raui::data::rect::*;
use raui::data::vec2::*;
use raui::data::color::*;
use raui::data::key_modifiers::*;
use raui::data::behaviour_flags;
use raui::component::*;
use raui::components;
use raui::renderables;
use ::server_singleton::*;

#[no_mangle]
pub extern fn raui_server_startup() -> bool {
    // HERE YOU PUT YOUR UI INITIALIZATION (HERE YOU CREATE UI TREE).
    if let Ok(_) = server().perform(&mut |server| {
        server.set_root_with_action::<components::container::Container, _>(
            &mut |server, node, component| {
                component.set_area(
                    &Rect::from(
                        &Vec2::zero(),
                        &Vec2::zero()
                    )
                );
                component.set_coords(components::container::CS_PARENT);
                component.set_color(&Color::red());

                server.add_child_with_action::<components::image::Image, _>(
                    node,
                    &mut |_, _, component| {
                        component.set_area(
                            &Rect::from(
                                &Vec2::from(100.0, 50.0),
                                &Vec2::from(300.0, 150.0)
                            )
                        );
                        component.set_coords(components::container::CS_LOCAL);
                        component.set_image_source("logo".to_string());
                        component.set_behaviour(behaviour_flags::BF_READING_MOUSE);
                        component.base.on_click_event = Some(|| {
                            println!("Click!");
                        });
                        component.base.on_focus_event = Some(|| {
                            println!("Got focus!");
                        });
                        component.base.on_blur_event = Some(|| {
                            println!("Lost focus!");
                        });
                    }
                );

                server.add_child_with_action::<components::text::Text, _>(
                    node,
                    &mut |_, _, component| {
                        component.set_area(
                            &Rect::from(
                                &Vec2::zero(),
                                &Vec2::zero()
                            )
                        );
                        component.set_coords(components::container::CS_PARENT);
                        component.set_font_source("verdana".to_string());
                        component.set_value("Hello World!".to_string());
                        component.set_size(32.0);
                        component.set_halign(renderables::text::HA_CENTER);
                        component.set_valign(renderables::text::VA_MIDDLE);
                    }
                );
            }
        );

        Ok(())
    }) {
        return true;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_set_client_rect(rect: &Rect) -> bool {
    if let Ok(_) = server().perform::<()>(&mut |s| Ok(s.set_client_rect(rect))) {
        return true;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_get_client_rect() -> Rect {
    if let Ok(r) = server().perform::<Rect>(&mut |s| Ok(*s.get_client_rect())) {
        return r;
    }

    Rect::zero()
}

#[no_mangle]
pub extern fn raui_server_bind_signal_callback(callback: extern fn(*const c_char)) -> bool {
    if let Ok(_) = server().perform::<()>(&mut |s| Ok(s.on_signal_event = Some(callback))) {
        return true;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_unbind_signal_callback() -> bool {
    if let Ok(_) = server().perform::<()>(&mut |s| Ok(s.on_signal_event = None)) {
        return true;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_bind_render_callback(callback: extern fn(*const u8, u32)) -> bool {
    if let Ok(_) = server().perform::<()>(&mut |s| Ok(s.on_render_event = Some(callback))) {
        return true;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_unbind_render_callback() -> bool {
    if let Ok(_) = server().perform::<()>(&mut |s| Ok(s.on_render_event = None)) {
        return true;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_process(forced: bool) -> bool {
    if let Ok(r) = server().perform(&mut |s| Ok(s.process(forced))) {
        return r;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_signal(id: *const c_char) -> bool {
    if id == (0 as *const c_char) {
        return false;
    }

    if let Ok(v) = unsafe { CStr::from_ptr(id) }.to_str() {
        if let Ok(_) = server().perform(&mut |s| Ok(s.signal(v))) {
            return true;
        }
    }

    false
}

#[no_mangle]
pub extern fn raui_server_trigger_mouse_down(screen_pos: &Vec2) -> bool {
    if let Ok(r) = server().perform(&mut |s| Ok(s.trigger_mouse_down(screen_pos))) {
        return r;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_trigger_mouse_up(screen_pos: &Vec2) -> bool {
    if let Ok(r) = server().perform(&mut |s| Ok(s.trigger_mouse_up(screen_pos))) {
        return r;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_trigger_mouse_move(screen_pos: &Vec2) -> bool {
    if let Ok(r) = server().perform(&mut |s| Ok(s.trigger_mouse_move(screen_pos))) {
        return r;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_trigger_mouse_click(screen_pos: &Vec2) -> bool {
    if let Ok(r) = server().perform(&mut |s| Ok(s.trigger_mouse_click(screen_pos))) {
        return r;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_trigger_key_pressed(code: u8, modifiers: u8) -> bool {
    if let Ok(r) = server().perform(&mut |s| {
        Ok(s.trigger_key_pressed(code, &KeyModifiers::from_val(modifiers)))
    }) {
        return r;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_trigger_key_released(code: u8, modifiers: u8) -> bool {
    if let Ok(r) = server().perform(&mut |s| {
        Ok(s.trigger_key_released(code, &KeyModifiers::from_val(modifiers)))
    }) {
        return r;
    }

    false
}

#[no_mangle]
pub extern fn raui_server_trigger_key_tap(code: u8, modifiers: u8) -> bool {
    if let Ok(r) = server().perform(&mut |s| {
        Ok(s.trigger_key_tap(code, &KeyModifiers::from_val(modifiers)))
    }) {
        return r;
    }

    false
}

#[cfg(test)]
mod tests {

    use std::ffi::CString;
    use super::*;

    static mut DONE: bool = false;

    extern fn callback_signal(_: *const c_char) {
        unsafe { DONE = true }
    }

    extern fn callback_render(_: *const u8, _: u32) {
        unsafe { DONE = true }
    }

    fn reset_state() {
        unsafe { DONE = false }
    }

    #[test]
    fn main() {

        {
            assert_eq!(
                true,
                raui_server_startup()
            );
        }

        {
            assert_eq!(
                true,
                raui_server_set_client_rect(
                    &Rect::from(
                        &Vec2::zero(),
                        &Vec2::from(1024.0, 768.0)
                    )
                )
            );
        }

        {
            assert_eq!(
                Rect::from(
                    &Vec2::zero(),
                    &Vec2::from(1024.0, 768.0)
                ),
                raui_server_get_client_rect()
            );
        }

        {
            assert_eq!(
                true,
                raui_server_bind_render_callback(callback_render)
            );
            assert_eq!(
                true,
                raui_server_process(false)
            );
            assert_eq!(
                false,
                raui_server_process(false)
            );
            assert_eq!(
                true,
                raui_server_unbind_render_callback()
            );
            assert_eq!(true, unsafe { DONE });
            reset_state();
            assert_eq!(false, unsafe { DONE });
        }

        {
            assert_eq!(
                true,
                raui_server_bind_signal_callback(callback_signal)
            );
            assert_eq!(
                true,
                raui_server_signal(CString::new("ping").unwrap().as_ptr())
            );
            assert_eq!(
                true,
                raui_server_unbind_signal_callback()
            );
            assert_eq!(true, unsafe { DONE });
            reset_state();
            assert_eq!(false, unsafe { DONE });
        }

    }

}
