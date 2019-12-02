#!/usr/bin/gjs

/*
GJS example showing how to build Gtk javascript applications
using Gtk TreeView and ListStore
Run it with:
    gjs egList.js
*/

const Gio   = imports.gi.Gio;
const GLib  = imports.gi.GLib;
const GObj  = imports.gi.GObject;
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

const { NirahSocket } = imports.utils.socket;
const { AccountsTable } = imports.widgets.accounts_table;

const App = function () {
    this.title = 'Accounts Table Example';
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
                                              default_height: 300,
                                              default_width: 720,
                                              window_position: Gtk.WindowPosition.CENTER });
    this.window.add(this.getBody());
};
App.prototype.getBody = function () {

    let box, accounts_table;

    box = new Gtk.Box({ vexpand: true });
    accounts_table = new AccountsTable();
    box.add(accounts_table.widget());
    return box;
};

//Run the application
let app = new App();
app.run(ARGV);
