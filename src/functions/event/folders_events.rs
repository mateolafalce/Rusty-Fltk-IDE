use fltk::prelude::*;
#[path="../options_windows.rs"]
mod options_windows;
#[path="../root/set_folders_roots.rs"]
mod set_folders_roots;
#[path="../folders_functions/render_folder.rs"]
mod render_folder;

pub fn folders_events(
    folders: &mut fltk::tree::Tree,
    event: fltk::enums::Event,
    app: fltk::app::App,
    mut text_buffer: fltk::text::TextBuffer,
    mut text_editor: fltk::text::TextEditor
) -> bool {
    //let mut item: fltk::tree::TreeItem = fltk::tree::TreeItem::new(&folders, "Show in scrren the file");
    match event {
        fltk::enums::Event::Push => {
            // Handle right-click event
            if fltk::app::event_mouse_button() == fltk::app::MouseButton::Right {
                options_windows::options_windows(app.clone(),folders,text_buffer.clone());
            } else if fltk::app::event_mouse_button() == fltk::app::MouseButton::Left {
                match folders.get_selected_items() {
                    None => (),
                    Some(vals) => {
                        let path = vals.iter().map(|i| folders.item_pathname(i).unwrap() + "\n").collect::<String>();
                        //println!("{}")
                        //get path then draw
                        text_buffer.set_text("Set content");
                        text_editor.set_buffer(Some(text_buffer.clone()));
                    }
                }
            };
            true
        },
        fltk::enums::Event::DndEnter => true, // Handle drag and drop enter event
        fltk::enums::Event::DndDrag => true, // Handle drag and drop drag event
        fltk::enums::Event::DndRelease => true, // Handle drag and drop release event
        fltk::enums::Event::Paste => {
            // Handle paste event
            set_folders_roots::set_folders_roots(fltk::app::event_text()).unwrap();
            render_folder::render_folder(app.clone(),folders.clone(),text_buffer.clone());
            true
        }
        _ => false,
    }
}
