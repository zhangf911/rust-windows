#[feature(globs)];

extern mod win32;

use std::ptr;
use std::cell::Cell;

use win32::window::*;
use win32::ll::*;

struct MainFrame {
    win: Window,
    title: ~str,
    edit: Cell<Window>,
}

impl OnCreate for MainFrame {
    fn on_create(&self, _cs: &CREATESTRUCT) -> bool {
        let params = WindowParams {
            window_name: ~"Hello World",
            style: WS_CHILD | WS_VISIBLE | WS_BORDER | WS_VSCROLL |
                ES_AUTOVSCROLL | ES_MULTILINE | ES_NOHIDESEL,
            x: 0,
            y: 0,
            width: 300,
            height: 300,
            parent: self.win,
            menu: ptr::mut_null(),
            ex_style: 0,
        };
        let edit = Window::new(Instance::main_instance(), "EDIT", &params);
        match edit {
            None => false,
            Some(e) => {
                self.edit.put_back(e);
                true
            }
        }
    }
}

impl OnDestroy for MainFrame {}

impl WndProc for MainFrame {
    fn wnd<'a>(&'a self) -> &'a Window {
        &self.win
    }

    fn wnd_mut<'a>(&'a mut self) -> &'a mut Window {
        &mut self.win
    }

    fn wnd_proc(&self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
        if msg == 0x0001 { // WM_CREATE
            let cs = unsafe {
                let pcs = std::cast::transmute::<LPARAM, *CREATESTRUCT>(l);
                &(*pcs)
            };
            let ret = self.on_create(cs);
            return if ret { 0 as LRESULT } else { -1 as LRESULT };
        }
        if msg == 0x0002 { // WM_DESTROY
            self.on_destroy();
            return 0 as LRESULT;
        }
        if msg == 0x000F { // WM_PAINT
            let (dc, ps) = (*self).begin_paint();
            self.on_paint(dc);
            (*self).end_paint(&ps);
            return 0 as LRESULT;
        }
        win32::def_window_proc(self.wnd().wnd, msg, w, l)
    }
}

impl OnPaint for MainFrame {
    fn on_paint(&self, dc: HDC) {
        (*self).text_out(dc, 0, 0, self.title);
    }
}

impl MainFrame {
    fn new(instance: Instance, title: ~str) -> Option<Window> {
        let wnd_class = WndClass {
            classname: ~"MainFrame",
            style: 0x0001 | 0x0002, // CS_HREDRAW | CS_VREDRAW
            icon: ptr::mut_null(),
            icon_small: ptr::mut_null(),
            cursor: Cursor::load_resource(32514), // hourglass
            background: (5 + 1) as HBRUSH,
            menu_name: None,
            cls_extra: 0,
            wnd_extra: 0,
        };
        let res = wnd_class.register(instance);
        if !res {
            return None;
        }

        let proc = ~MainFrame {
            win: Window::null(),
            title: title.clone(),
            edit: Cell::new_empty(),
        };
        set_proc(proc as ~WndProc);

        let win_params = WindowParams {
            window_name: title,
            style: WS_OVERLAPPEDWINDOW,
            x: 0,
            y: 0,
            width: 400,
            height: 400,
            parent: Window::null(),
            menu: ptr::mut_null(),
            ex_style: 0,
        };

        Window::new(instance, wnd_class.classname, &win_params)
    }
}

fn main() {
    init_window_map();

    let instance = Instance::main_instance();
    let main = MainFrame::new(instance, ~"Hello");
    let main = main.unwrap();

    main.show(1);
    main.update();

    let exit_code = win32::main_window_loop();
    std::os::set_exit_status(exit_code as int);
}
