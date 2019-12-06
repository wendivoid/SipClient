'use-strict';

const Gio = imports.gi.Gio;
const Gtk   = imports.gi.Gtk;
const GObj  = imports.gi.GObject;

const { NirahSocket } = imports.utils.socket;
const { ContactWidget } = imports.widgets.contact;

var ContactsPage = class contactsPage {
  constructor() {
      let self = this;
      this._component = new Gtk.Box({ orientation: Gtk.Orientation.VERTICAL, expand: true });
      this.scrollWindow = new Gtk.ScrolledWindow({ expand: true, margin: 7 });
      this.listbox = new Gtk.ListBox();
      this.scrollWindow.add(this.listbox);
      this._component.add(this.scrollWindow);
      this.client = new NirahSocket();
      let req = { method: 'AllContacts' };
      this.client.send_then_expect(req, 'AllContacts', function (item) {
        self.addItems(item.contacts);
      });
  }

  widget() {
    return this._component;
  }

  addItems(contacts) {
    let self = this;
    contacts.forEach(function (contact) {
        self.listbox.add(new ContactWidget(contact).widget());
    });
  }
};
