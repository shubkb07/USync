use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use arboard::Clipboard;
use clap::{Parser, Subcommand};
use glib::{self, ControlFlow};
use gtk4::glib::set_application_name;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, Label, ListBox, ListBoxRow, Orientation,
    ScrolledWindow, SelectionMode, SpinButton, Stack,
};

use usync_app::{AppSettings, ClipboardStore};

#[derive(Parser, Debug)]
#[command(name = "usync-app", about = "USync")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Gui,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Gui) | None => run_gui(),
    }
}

fn run_gui() -> Result<()> {
    set_application_name("USync");
    let app = Application::builder().application_id("com.usync.app").build();
    app.connect_activate(build_ui);
    app.run_with_args::<&str>(&[]);
    Ok(())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("USync")
        .icon_name("com.usync.app")
        .default_width(900)
        .default_height(600)
        .build();

    let root = GtkBox::new(Orientation::Horizontal, 0);

    // Sidebar
    let sidebar = ListBox::new();
    sidebar.set_selection_mode(SelectionMode::Single);
    sidebar.add_css_class("navigation-sidebar");
    sidebar.set_size_request(220, -1);

    let clip_row = ListBoxRow::new();
    clip_row.set_child(Some(&Label::new(Some("Clipboard"))));
    sidebar.append(&clip_row);

    let settings_row = ListBoxRow::new();
    settings_row.set_child(Some(&Label::new(Some("Clipboard Settings"))));
    sidebar.append(&settings_row);

    root.append(&sidebar);

    let stack = Stack::new();
    stack.set_hexpand(true);
    stack.set_vexpand(true);

    let store = Rc::new(RefCell::new(ClipboardStore::load()));
    let clipboard = Rc::new(RefCell::new(Clipboard::new().ok()));
    let last_seen = Rc::new(RefCell::new(String::new()));

    // Clipboard page
    let clip_page = GtkBox::new(Orientation::Vertical, 10);
    clip_page.set_margin_top(14);
    clip_page.set_margin_bottom(14);
    clip_page.set_margin_start(14);
    clip_page.set_margin_end(14);

    let title = Label::new(Some("Clipboard"));
    title.set_xalign(0.0);
    title.add_css_class("title-2");
    clip_page.append(&title);

    let subtitle = Label::new(Some("Recent clipboard entries are saved and restored between app launches."));
    subtitle.set_xalign(0.0);
    clip_page.append(&subtitle);

    let history_list = ListBox::new();
    history_list.set_selection_mode(SelectionMode::Single);
    let scroll = ScrolledWindow::new();
    scroll.set_vexpand(true);
    scroll.set_child(Some(&history_list));
    clip_page.append(&scroll);

    let row = GtkBox::new(Orientation::Horizontal, 8);
    let capture_btn = Button::with_label("Capture Now");
    let copy_selected_btn = Button::with_label("Copy Selected");
    row.append(&capture_btn);
    row.append(&copy_selected_btn);
    clip_page.append(&row);

    let clip_status = Label::new(Some("Ready"));
    clip_status.set_xalign(0.0);
    clip_page.append(&clip_status);

    stack.add_named(&clip_page, Some("clipboard"));

    // Settings page
    let settings_page = GtkBox::new(Orientation::Vertical, 10);
    settings_page.set_margin_top(14);
    settings_page.set_margin_bottom(14);
    settings_page.set_margin_start(14);
    settings_page.set_margin_end(14);

    let settings_title = Label::new(Some("Clipboard Settings"));
    settings_title.set_xalign(0.0);
    settings_title.add_css_class("title-2");
    settings_page.append(&settings_title);

    let max_items_label = Label::new(Some("Max stored clipboard items"));
    max_items_label.set_xalign(0.0);
    settings_page.append(&max_items_label);

    let max_items_spin = SpinButton::with_range(10.0, 500.0, 1.0);
    max_items_spin.set_value(store.borrow().settings().max_items as f64);
    settings_page.append(&max_items_spin);

    let poll_interval_label = Label::new(Some("Clipboard poll interval (seconds)"));
    poll_interval_label.set_xalign(0.0);
    settings_page.append(&poll_interval_label);

    let poll_interval_spin = SpinButton::with_range(1.0, 10.0, 1.0);
    poll_interval_spin.set_value(store.borrow().settings().poll_interval_seconds as f64);
    settings_page.append(&poll_interval_spin);

    let save_settings_btn = Button::with_label("Save Settings");
    settings_page.append(&save_settings_btn);

    let settings_status = Label::new(Some(""));
    settings_status.set_xalign(0.0);
    settings_page.append(&settings_status);

    stack.add_named(&settings_page, Some("settings"));

    root.append(&stack);

    let refresh_history_view = {
        let history_list = history_list.clone();
        let store = Rc::clone(&store);
        move || {
            while let Some(child) = history_list.first_child() {
                history_list.remove(&child);
            }

            for item in store.borrow().items() {
                let label = Label::new(Some(item));
                label.set_xalign(0.0);
                let row = ListBoxRow::new();
                row.set_child(Some(&label));
                history_list.append(&row);
            }
        }
    };

    refresh_history_view();

    {
        let stack = stack.clone();
        sidebar.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                match row.index() {
                    0 => stack.set_visible_child_name("clipboard"),
                    1 => stack.set_visible_child_name("settings"),
                    _ => {}
                }
            }
        });
    }
    sidebar.select_row(Some(&clip_row));

    {
        let store = Rc::clone(&store);
        let clipboard = Rc::clone(&clipboard);
        let refresh_history_view = refresh_history_view.clone();
        let clip_status = clip_status.clone();
        capture_btn.connect_clicked(move |_| {
            let text = clipboard
                .borrow_mut()
                .as_mut()
                .and_then(|c| c.get_text().ok());

            match text {
                Some(text) if !text.trim().is_empty() => {
                    let mut s = store.borrow_mut();
                    s.add_clipboard_entry(&text);
                    if let Err(e) = s.save() {
                        clip_status.set_text(&format!("Saved in memory, failed to write disk: {e}"));
                    } else {
                        clip_status.set_text("Captured and saved");
                    }
                    drop(s);
                    refresh_history_view();
                }
                _ => clip_status.set_text("Clipboard does not contain text"),
            }
        });
    }

    {
        let store = Rc::clone(&store);
        let clipboard = Rc::clone(&clipboard);
        let history_list = history_list.clone();
        let clip_status = clip_status.clone();
        copy_selected_btn.connect_clicked(move |_| {
            if let Some(row) = history_list.selected_row() {
                let idx = row.index() as usize;
                if let Some(item) = store.borrow().items().get(idx) {
                    if let Some(clip) = clipboard.borrow_mut().as_mut() {
                        if clip.set_text(item.clone()).is_ok() {
                            clip_status.set_text("Copied selected item");
                            return;
                        }
                    }
                }
            }
            clip_status.set_text("No item selected");
        });
    }

    {
        let store = Rc::clone(&store);
        let max_items_spin = max_items_spin.clone();
        let poll_interval_spin = poll_interval_spin.clone();
        let settings_status = settings_status.clone();
        save_settings_btn.connect_clicked(move |_| {
            let settings = AppSettings {
                max_items: max_items_spin.value() as usize,
                poll_interval_seconds: poll_interval_spin.value() as u32,
            };
            let mut s = store.borrow_mut();
            s.update_settings(settings);
            match s.save() {
                Ok(_) => settings_status.set_text("Settings saved"),
                Err(e) => settings_status.set_text(&format!("Failed to save settings: {e}")),
            }
        });
    }

    {
        let store = Rc::clone(&store);
        let clipboard = Rc::clone(&clipboard);
        let last_seen = Rc::clone(&last_seen);
        let clip_status = clip_status.clone();
        let refresh_history_view = refresh_history_view.clone();

        glib::timeout_add_seconds_local(1, move || {
            let current = clipboard
                .borrow_mut()
                .as_mut()
                .and_then(|c| c.get_text().ok())
                .unwrap_or_default();

            if !current.trim().is_empty() && *last_seen.borrow() != current {
                *last_seen.borrow_mut() = current.clone();
                let mut s = store.borrow_mut();
                s.add_clipboard_entry(&current);
                let _ = s.save();
                drop(s);
                refresh_history_view();
                clip_status.set_text("Clipboard updated");
            }

            ControlFlow::Continue
        });
    }

    window.set_child(Some(&root));
    window.present();
}
