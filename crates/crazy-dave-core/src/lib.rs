use native_windows_derive as nwd;
use native_windows_gui as nwg;
use nwd::NwgUi;
use nwg::NativeUi;

const BASE_PTR: *const usize = 0x006A9EC0 as _;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "疯狂戴夫", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "9999 阳光")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0)]
    #[nwg_events( OnButtonClick: [BasicApp::change_sun] )]
    hello_button: nwg::Button,
}

impl BasicApp {
    fn change_sun(&self) {
        unsafe {
            let sun_ptr: *mut i32 = (*((*BASE_PTR + 0x00000768) as *mut usize) + 0x00005560) as _;
            *sun_ptr = 9999;
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

pub fn start_window() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_should_work() {
        start_window();
    }
}
