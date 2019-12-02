'use-strict';

const Gio = imports.gi.Gio;
const Gtk   = imports.gi.Gtk;
const GObj  = imports.gi.GObject;

const { NirahSocket } = imports.utils.socket;

var AboutWidget = class aboutWidget {
  constructor() {
      let self = this;
      this._component = new Gtk.VBox({ margin: 20, expand: true });
      let client = new NirahSocket();
      this._store = new Gtk.ListStore();
      this._store.set_column_types([
        GObj.TYPE_STRING, GObj.TYPE_STRING,
        GObj.TYPE_STRING
      ]);
      this._treeview = new Gtk.TreeView();
      this._roleCol = new Gtk.TreeViewColumn({ expand: true, title: 'Role' });
      this._roleColRender = new Gtk.CellRendererText();
      this._roleCol.set_cell_data_func(this._roleColRender, self.renderRole);
      this._roleCol.pack_start(this._roleColRender, true);
      this._nameCol = new Gtk.TreeViewColumn({ expand: true, title: 'Name' });
      this._nameColRender = new Gtk.CellRendererText();
      this._nameCol.set_cell_data_func(this._nameColRender, self.renderName);
      this._nameCol.pack_start(this._nameColRender, true);
      this._verCol = new Gtk.TreeViewColumn({ expand: true, title: 'Version' });
      this._verColRender = new Gtk.CellRendererText();
      this._verCol.set_cell_data_func(this._verColRender, self.renderVersion);
      this._verCol.pack_start(this._verColRender, true);
      this._treeview.set_model(this._store);
      this._treeview.append_column(this._roleCol);
      this._treeview.append_column(this._nameCol);
      this._treeview.append_column(this._verCol);
      this._component.add(this._treeview);
      if(client.connect()) {
        let self = this;
        let req = { 'method': 'AboutNirah' };
        client.send_then_expect(req, 'AboutNirah', function(res) {
          self.addItems(res, self)
        });
      } else {
        print("Failed to connect to nirah socket");
      }
  }

  widget() {
    return this._component;
  }

  addItems(result, widget) {
    widget._store.set(widget._store.append(), [0, 1, 2], ['Accounts', result.accounts[0], result.accounts[1]]);
    widget._store.set(widget._store.append(), [0, 1, 2], ['Config', result.config[0], result.config[1]]);
    widget._store.set(widget._store.append(), [0, 1, 2], ['Contacts', result.contacts[0], result.contacts[1]]);
    widget._store.set(widget._store.append(), [0, 1, 2], ['Database', result.database[0], result.database[1]]);
    widget._store.set(widget._store.append(), [0, 1, 2], ['Notifier', result.notifier[0], result.notifier[1]]);
    widget._store.set(widget._store.append(), [0, 1, 2], ['Rpc', result.rpc[0], result.rpc[1]]);
    widget._store.set(widget._store.append(), [0, 1, 2], ['Rpc', result.rpc_handler[0], result.rpc_handler[1]]);
    result.sessions.forEach(function (item) {
      widget._store.set(widget._store.append(), [0, 1, 2], ['Session', item[0], item[1]]);
    });
  }
};

AboutWidget.prototype.renderRole = function (col, cell, model, iter) {
  cell.editable = false;
  cell.text = model.get_value(iter, 0);
};

AboutWidget.prototype.renderName = function (col, cell, model, iter) {
  cell.editable = false;
  cell.text = model.get_value(iter, 1);
};

AboutWidget.prototype.renderVersion = function (col, cell, model, iter) {
  cell.editable = false;
  cell.text = model.get_value(iter, 2);
};
