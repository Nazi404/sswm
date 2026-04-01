use x11rb::{connect,connection::Connection};
use x11rb::protocol::{xproto::*,Event};

fn main() {
    let (conn,screen_num) = connect(None).unwrap();
    println!("Connected to X server {}",screen_num);

    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;
    println!("Root window id is {}",root);

    conn.change_window_attributes(
        root,
        &ChangeWindowAttributesAux::new()
        .event_mask(
            EventMask::SUBSTRUCTURE_REDIRECT |
            EventMask::SUBSTRUCTURE_NOTIFY |
            EventMask::KEY_PRESS
        )
    ).unwrap();
    conn.flush().unwrap();

    println!("Done");

    loop {
        let event = conn.wait_for_event().unwrap();
        println!("Event {:?}",event);

        match event {
            Event::MapRequest(ev) => {
                println!("New window id is {}",ev.window);
                conn.map_window(ev.window).unwrap();
                conn.flush().unwrap();
            }
            _ => {}
        }
    }
}
