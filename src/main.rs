use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use arboard::Clipboard;
use clap::{Parser, Subcommand};
use glib::{self, ControlFlow};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Entry, Label, ListBox, Orientation};

use usync_app::{add_numbers, get_status_message, ClipboardHistory};

#[derive(Parser, Debug)]
#[command(name = "usync-app", about = "USync Ubuntu app with CLI and GTK GUI")]
struct Cli {
    #[arg(long, default_value = "User")]
    name: String,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Add { a: f64, b: f64 },
    Gui,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Add { a, b }) => {
            println!("{}", add_numbers(a, b));
            Ok(())
        }
        Some(Command::Gui) => run_gui(),
        None => {
            println!("{}", get_status_message(&cli.name));
            Ok(())
        }
    }
}

fn run_gui() -> Result<()> {
    let app = Application::builder()
        .application_id("com.usync.app")
        .build();

    app.connect_activate(build_ui);
    app.run();
    Ok(())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("USync Clipboard")
        .default_width(680)
        .default_height(500)
        .build();

    let root = GtkBox::new(Orientation::Vertical, 8);
    root.set_margin_top(14);
    root.set_margin_bottom(14);
    root.set_margin_start(14);
    root.set_margin_end(14);

    let status = Label::new(Some(&get_status_message("User")));
    status.set_xalign(0.0);
    root.append(&status);

    let clipboard_title = Label::new(Some("Clipboard History"));
    clipboard_title.set_xalign(0.0);
    root.append(&clipboard_title);

    let history_list = ListBox::new();
    history_list.set_vexpand(true);
    root.append(&history_list);

    let button_row = GtkBox::new(Orientation::Horizontal, 8);
    let capture_btn = Button::with_label("Capture Clipboard");
    let copy_selected_btn = Button::with_label("Copy Selected");
    button_row.append(&capture_btn);
    button_row.append(&copy_selected_btn);
    root.append(&button_row);

    let clip_status = Label::new(Some("Clipboard monitor ready"));
    clip_status.set_xalign(0.0);
    root.append(&clip_status);

    let calc_title = Label::new(Some("Quick Add"));
    calc_title.set_xalign(0.0);
    root.append(&calc_title);

    let calc_row = GtkBox::new(Orientation::Horizontal, 8);
    let entry_a = Entry::builder().placeholder_text("A").text("1").build();
    let entry_b = Entry::builder().placeholder_text("B").text("2").build();
    let calc_btn = Button::with_label("Calculate");
    calc_row.append(&entry_a);
    calc_row.append(&entry_b);
    calc_row.append(&calc_btn);
    root.append(&calc_row);

    let calc_result = Label::new(Some("Result: 3"));
    calc_result.set_xalign(0.0);
    root.append(&calc_result);

    let history = Rc::new(RefCell::new(ClipboardHistory::new(20)));
    let clipboard = Rc::new(RefCell::new(Clipboard::new().ok()));
    let last_seen = Rc::new(RefCell::new(String::new()));

    let refresh_view = {
        let history = Rc::clone(&history);
        let history_list = history_list.clone();
        move || {
            while let Some(child) = history_list.first_child() {
                history_list.remove(&child);
            }

            for item in history.borrow().items() {
                let row_label = Label::new(Some(item));
                row_label.set_xalign(0.0);
                history_list.append(&row_label);
            }
        }
    };

    {
        let history = Rc::clone(&history);
        let clipboard = Rc::clone(&clipboard);
        let clip_status = clip_status.clone();
        let refresh_view = refresh_view.clone();
        capture_btn.connect_clicked(move |_| {
            let current = clipboard
                .borrow_mut()
                .as_mut()
                .and_then(|c| c.get_text().ok());

            match current {
                Some(text) if !text.trim().is_empty() => {
                    history.borrow_mut().add(&text);
                    refresh_view();
                    clip_status.set_text("Captured current clipboard text");
                }
                _ => clip_status.set_text("Clipboard does not contain text"),
            }
        });
    }

    {
        let history = Rc::clone(&history);
        let clipboard = Rc::clone(&clipboard);
        let clip_status = clip_status.clone();
        let history_list = history_list.clone();
        copy_selected_btn.connect_clicked(move |_| {
            if let Some(row) = history_list.selected_row() {
                let idx = row.index() as usize;
                if let Some(value) = history.borrow().items().get(idx) {
                    if let Some(c) = clipboard.borrow_mut().as_mut() {
                        if c.set_text(value.clone()).is_ok() {
                            clip_status.set_text("Selected item copied to clipboard");
                            return;
                        }
                    }
                }
            }
            clip_status.set_text("No clipboard item selected");
        });
    }

    {
        let calc_result = calc_result.clone();
        let entry_a = entry_a.clone();
        let entry_b = entry_b.clone();
        calc_btn.connect_clicked(move |_| {
            let a = entry_a.text().parse::<f64>();
            let b = entry_b.text().parse::<f64>();
            match (a, b) {
                (Ok(x), Ok(y)) => calc_result.set_text(&format!("Result: {}", add_numbers(x, y))),
                _ => calc_result.set_text("Result: invalid input"),
            }
        });
    }

    {
        let history = Rc::clone(&history);
        let clipboard = Rc::clone(&clipboard);
        let last_seen = Rc::clone(&last_seen);
        let clip_status = clip_status.clone();
        let refresh_view = refresh_view.clone();
        glib::timeout_add_seconds_local(1, move || {
            let current = clipboard
                .borrow_mut()
                .as_mut()
                .and_then(|c| c.get_text().ok())
                .unwrap_or_default();

            if !current.trim().is_empty() && *last_seen.borrow() != current {
                *last_seen.borrow_mut() = current.clone();
                history.borrow_mut().add(&current);
                refresh_view();
                clip_status.set_text("Clipboard updated");
            }
            ControlFlow::Continue
        });
    }

    window.set_child(Some(&root));
    window.present();
}
