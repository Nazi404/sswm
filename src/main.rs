use x11rb::protocol::{xproto::*, Event};
use x11rb::{connect, connection::Connection};

fn arrange(conn: &impl Connection, screen: &Screen, windows: &Vec<Window>) {
    let n = windows.len();
    if n == 0 {
        return;
    }

    let width = screen.width_in_pixels as u32;
    let height = screen.height_in_pixels as u32;

    if n == 1 {
        conn.configure_window(
            windows[0],
            &ConfigureWindowAux::new()
                .x(0)
                .y(0)
                .width(width)
                .height(height),
        )
        .unwrap();
    } else {
        let master_w = width / 2;

        // master
        conn.configure_window(
            windows[0],
            &ConfigureWindowAux::new()
                .x(0)
                .y(0)
                .width(master_w)
                .height(height),
        )
        .unwrap();

        // stack
        let stack_h = height / (n as u32 - 1);

        for (i, win) in windows.iter().skip(1).enumerate() {
            conn.configure_window(
                *win,
                &ConfigureWindowAux::new()
                    .x(master_w as i32)
                    .y((i as u32 * stack_h) as i32)
                    .width(width - master_w)
                    .height(stack_h),
            )
            .unwrap();
        }
    }

    conn.flush().unwrap();
}

fn main() {
    let mut windows: Vec<Window> = Vec::new();
    let mut focused: Option<Window> = None;
    let (conn, screen_num) = connect(None).unwrap();
    println!("Connected to X server {}", screen_num);

    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;
    println!("Root window is 0x{:x}", root);

    conn.change_window_attributes(
        root,
        &ChangeWindowAttributesAux::new().event_mask(
            EventMask::SUBSTRUCTURE_REDIRECT
                | EventMask::SUBSTRUCTURE_NOTIFY
                | EventMask::KEY_PRESS,
        ),
    )
    .unwrap();

    conn.flush().unwrap();
    println!("Flushed");

    loop {
        let event = conn.wait_for_event().unwrap();
        println!("Event {:?}", event);

        match event {
            Event::MapRequest(ev) => {
                println!("Map request for window 0x{:x}", ev.window);
                windows.push(ev.window);
                focused = Some(ev.window);

                arrange(&conn,screen,&windows);
                conn.map_window(ev.window).unwrap();
                conn.flush().unwrap();
            }
            Event::ConfigureRequest(ev) => {
                println!("Configure request for window 0x{:x}", ev.window);

                let aux = ConfigureWindowAux::from_configure_request(&ev);
                conn.configure_window(ev.window, &aux).unwrap();
                conn.flush().unwrap();
            }
            _ => {}
        }
    }
}
