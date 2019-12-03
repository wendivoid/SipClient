#!/usr/bin/gjs

const Gio   = imports.gi.Gio;
const GLib  = imports.gi.GLib;
const Gtk   = imports.gi.Gtk;
const Lang  = imports.lang;

// Get application folder and add it into the imports path
function getAppFileInfo() {
    let stack = (new Error()).stack,
        stackLine = stack.split('\n')[1],
        coincidence, path, file;

    if (!stackLine) throw new Error('Could not find current file (1)');

    coincidence = new RegExp('@(.+):\\d+').exec(stackLine);
    if (!coincidence) throw new Error('Could not find current file (2)');

    path = coincidence[1];
    file = Gio.File.new_for_path(path);
    return [file.get_path(), file.get_parent().get_path(), file.get_basename()];
}
const path = getAppFileInfo()[1];
imports.searchPath.push(path);
const { PreferencesWindow } = imports.preferences;
const { HistoryPage } = imports.pages.history;
const { ContactsPage } = imports.pages.contacts;

const App = function () {
    this.title = 'Nirah';
    GLib.set_prgname(this.title);
};

App.prototype.run = function (ARGV) {

    this.application = new Gtk.Application();
    this.application.connect('activate', () => { this.onActivate(); });
    this.application.connect('startup', () => { this.onStartup(); });
    this.application.run([]);
};

App.prototype.onActivate = function () {

    this.window.show_all();
};

App.prototype.onStartup = function() {

    this.buildUI();
};

App.prototype.buildUI = function() {

    this.window = new Gtk.ApplicationWindow({ application: this.application,
                                              default_height: 450,
                                              default_width: 300,
                                              window_position: Gtk.WindowPosition.CENTER });

    this.window.set_titlebar(this.getHeader());

    this._stack = new Gtk.Stack();
    this._transactions = new HistoryPage();
    this._contacts = new ContactsPage();
    this._stack.add_titled(this._transactions.widget(), "history", "History");
    this._stack.add_titled(this._contacts.widget(), "contacts", "Contacts");
    this.window.add(this._stack);
};

App.prototype.getHeader = function () {

    let headerBar, headerStart, imageNew, buttonNew, popMenu, imageMenu, buttonMenu;

    headerBar = new Gtk.HeaderBar();
    headerBar.set_title(this.title);
    headerBar.set_show_close_button(true);

    headerStart = new Gtk.Grid({ column_spacing: headerBar.spacing });

    headerBar.pack_start(headerStart);

    popMenu = new Gtk.Popover();
    imageMenu = new Gtk.Image ({ icon_name: 'open-menu-symbolic', icon_size: Gtk.IconSize.SMALL_TOOLBAR });
    buttonMenu = new Gtk.MenuButton({ image: imageMenu });
    buttonMenu.set_popover(popMenu);
    popMenu.set_size_request(-1, -1);
    buttonMenu.set_menu_model(this.getMenu());

    headerBar.pack_start(buttonMenu);

    return headerBar;
};

App.prototype.getMenu = function () { /* GMenu popover */

    let menu, section, submenu, self;

    self = this;
    menu = new Gio.Menu();

    section = new Gio.Menu();
    section.append("Contacts", 'app.toContactsPage');
    section.append("Preferences", 'app.toggleMenu');
    menu.append_section(null, section);

    let actionToggleMenu = new Gio.SimpleAction ({ name: 'toggleMenu' });
        actionToggleMenu.connect('activate', () => {
                new PreferencesWindow();
            });
        this.application.add_action(actionToggleMenu);
    let actionToContactsPage = new Gio.SimpleAction ({ name: 'toContactsPage' });
    actionToContactsPage.connect('activate', () => {
          self._stack.set_visible_child_name("contacts");
    });
    this.application.add_action(actionToContactsPage);
    return menu;
};

App.prototype.printText = function (text) {

    print(text);
};

//Run the application
let app = new App();
app.run(ARGV);
