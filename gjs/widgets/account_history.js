'use-strict';

const Gio = imports.gi.Gio;
const Gtk   = imports.gi.Gtk;
const GObj  = imports.gi.GObject;

const { NirahSocket } = imports.utils.socket;

var AccountHistory = class accountHistory {
  constructor() {
      let self = this;
      this._component = new Gtk.ListBox({ margin: 7, expand: true });
  }

  widget() {
    return this._component;
  }

  loadAccount(account) {
    let self = this;
    let client = new NirahSocket();
    client.connect();
    let req = { method: 'GetAccount', account: account };
    req = { method: 'AccountTransactions', account: account };
    client.send_then_expect(req, 'AccountTransactions', function (item) {
      log(JSON.stringify(item.transactions));
      let row = new Gtk.ListBoxRow();
      let vbox = new Gtk.VBox();
      let top_row = new Gtk.Box();
      let middle_row = new Gtk.Box();
      let bottom_row = new Gtk.Box();
      top_row.add(new Gtk.Label({ label: JSON.stringify(item) }));

      vbox.add(top_row);
      vbox.add(middle_row);
      vbox.add(bottom_row);
      row.add(vbox);
      self._component.add(row);
    })
  }
};
