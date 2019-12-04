'use-strict';

const Gio = imports.gi.Gio;
const Gtk   = imports.gi.Gtk;
const GObj  = imports.gi.GObject;

const { NirahSocket } = imports.utils.socket;

var TransactionWidget = class transactionWidget {
  constructor(account, contact, transaction) {
      this.account = account;
      this.contact = contact;
      this.transaction = transaction;
      this._component = new Gtk.VBox({ expand: true });
      this._vbox = new Gtk.VBox();
      this._top_row = new Gtk.Box();
      this._middle_row = new Gtk.Box();
      this._bottom_row = new Gtk.Box();
      this._top_row.add(this.getContactName());
      this._middle_row.add(new Gtk.HSeparator({ margin: 7, expand: true }));
      this._bottom_row.add(this.getLastMessageWidget());
      this._vbox.add(this._top_row);
      this._vbox.add(this._middle_row);
      this._vbox.add(this._bottom_row);
      this._component.add(this._vbox);
  }

  widget() {
    return this._component;
  }

  getContactName() {
    if(this.contact.display_name) {
      return new Gtk.Label({ label: this.contact.display_name });
    } else if (this.contact.uri.auth.username) {
      return new Gtk.Label({ label: this.contact.uri.auth.username });
    } else {
      return new Gtk.Label({ label: this.contact.id.toString() });
    }
  }

  getLastMessageWidget() {
    if(this.transaction[1].data.TextMessage.message) {
      let box = new Gtk.Box();
      let spacer = new Gtk.Box({ expand: true });
      let label = new Gtk.Label({ label: 'Message: ' });
      let message = new Gtk.Label({ label: this.transaction[1].data.TextMessage.message });
      box.pack_start(label, false, false, 0);
      box.pack_start(spacer, true, true, 0);
      box.pack_end(message, false, false, 0);
      return box;
    }
  }

};
