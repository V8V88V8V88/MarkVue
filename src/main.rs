use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, HeaderBar, MenuButton, Orientation, Paned};
use gtk4::{glib, gio};
use pulldown_cmark::{html, Options, Parser};
use sourceview5::{prelude::*, View as SourceView, Buffer as SourceBuffer};

fn main() {
    let app = Application::builder()
        .application_id("com.example.MarkVue")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("MarkVue")
        .default_width(800)
        .default_height(600)
        .build();

    let header_bar = HeaderBar::new();
    let menu_button = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .build();
    header_bar.pack_end(&menu_button);

    let paned = Paned::new(Orientation::Horizontal);
    paned.set_wide_handle(true);

    let source_view = SourceView::new();
    let source_buffer = source_view.buffer().downcast::<SourceBuffer>().unwrap();
    source_buffer.set_language(Some(&sourceview5::LanguageManager::default().language("markdown").unwrap()));
    source_buffer.set_highlight_syntax(true);
    source_view.set_show_line_numbers(true);
    source_view.set_monospace(true);

    let markdown_view = gtk4::TextView::new();
    markdown_view.set_editable(false);
    markdown_view.set_wrap_mode(gtk4::WrapMode::Word);

    paned.set_start_child(Some(&source_view));
    paned.set_end_child(Some(&markdown_view));

    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.append(&header_bar);
    vbox.append(&paned);

    window.set_child(Some(&vbox));

    source_buffer.connect_changed(glib::clone!(@weak markdown_view => move |buffer| {
        let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
        let parser = Parser::new_ext(&text, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        markdown_view.buffer().set_text(&html_output);
    }));

    // Create menu
    let menu = gio::Menu::new();
    menu.append(Some("About"), Some("app.about"));
    menu.append(Some("Quit"), Some("app.quit"));
    menu_button.set_menu_model(Some(&menu));

    // Add actions
    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(glib::clone!(@weak window => move |_, _| {
        let about_dialog = gtk4::AboutDialog::builder()
            .transient_for(&window)
            .modal(true)
            .program_name("MarkVue")
            .version("1.0")
            .authors(&vec!["Vaibhav Pratap Singh"])
            .website("https://github.com/v8v88v8v88/MarkVue")
            .website_label("GitHub Repository")
            .build();
        about_dialog.present();
    }));
    app.add_action(&about_action);

    let quit_action = gio::SimpleAction::new("quit", None);
    quit_action.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.close();
    }));
    app.add_action(&quit_action);

    window.present();
}