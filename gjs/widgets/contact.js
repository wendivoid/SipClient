'use-strict';

const Gio = imports.gi.Gio;
const Gtk   = imports.gi.Gtk;
const GObj  = imports.gi.GObject;

const { NirahSocket } = imports.utils.socket;

var ContactWidget = class contactWidget {
  constructor(contact) {
      let self = this;
      this._component = new Gtk.VBox();
      this.firstrow = new Gtk.Box();
      this.middlerow = new Gtk.Box();
      this.lastrow = new Gtk.Box();
      this._component.add(this.firstrow);
      this._component.add(this.middlerow);
      this._component.add(this.lastrow);
      this.idLabel = new Gtk.Label({ label: contact.id.toString() });
      this.firstrow.add(this.idLabel);
      let display_name = contact.display_name ? contact.display_name : contact.uri.auth.username ? contact.uri.auth.username : JSON.stringify(contact.uri);
      this.nameLabel = new Gtk.Label({ label: display_name})
      this.firstrow.pack_end(this.nameLabel, false, false, 0);
  }

  widget() {
    return this._component;
  }
};
