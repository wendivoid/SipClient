'use-strict';

const Gio = imports.gi.Gio;
const Gtk   = imports.gi.Gtk;
const GObj  = imports.gi.GObject;

const { NirahSocket } = imports.utils.socket;

var AccountsTable = class accountsTable {
  constructor() {
      let self = this;
      this._treeview = new Gtk.TreeView();
      this._treeview.set_vexpand(true);
      this._treeview.set_hexpand(true);
      this._store = new Gtk.ListStore();
      this._store.set_column_types([
        GObj.TYPE_INT,
        GObj.TYPE_STRING,
        GObj.TYPE_STRING,
        GObj.TYPE_STRING,
        GObj.TYPE_BOOLEAN
      ]);
      this.add_columns();
      let client = new NirahSocket();
      if(client.connect()) {
        client.send_message({ method: 'AllAccounts' });
        let res = client.read_message();
        res.accounts.forEach(function (item) {
          self._store.set(self._store.append(), [0, 1, 2, 3, 4], [item.id, item.username, item.password, item.host, item.activate]);
        });
      } else {
        log("Failed to connect to nirah socket");
      }
      this._treeview.set_model(this._store);
  }

  widget() {
    return this._treeview;
  }

  add_columns() {

        let col, self;

        self = this;

        col = new Gtk.TreeViewColumn({ title: 'Id' });
        this._treeview.append_column(col);
        let idColRender = new Gtk.CellRendererText();
        col.pack_start(idColRender, true);
        col.set_expand(false);
        col.set_cell_data_func(idColRender, (col, cell, model, iter) => {
          cell.editable = false;
          cell.text = model.get_value(iter, 0).toString();
        });

        col = new Gtk.TreeViewColumn({ title: 'Username' })
        let unameColRender = new Gtk.CellRendererText();
        col.pack_start(unameColRender, true);
        col.set_expand(true);
        unameColRender.connect('edited', function (firstArg, old, new_text, user_data) {
          let client = new NirahSocket();
          client.connect();
          client.send_message({ method: 'EditAccountUsername', account: 0, username: new_text});
          let res = client.read_message();
          if(res.response == 'Ok') {
              let iter = self._store.get_iter (Gtk.TreePath.new_from_string(old))[1];
              self._store.set_value(iter, 1, new_text);
          } else {
            log("Failed to Edit account username");
          }
        });
        col.set_cell_data_func(unameColRender, (col, cell, model, iter) => {
          cell.editable = true;
          cell.text = model.get_value(iter, 1);
        });
        this._treeview.append_column(col);

        col = new Gtk.TreeViewColumn({ title: 'Password' })
        let pwordColRender = new Gtk.CellRendererText();
        col.pack_start(pwordColRender, true);
        col.set_expand(true);
        pwordColRender.connect('edited', function (firstArg, old, new_text, user_data) {
          let client = new NirahSocket();
          client.connect();
          client.send_message({ method: 'EditAccountPassword', account: 0, password: new_text});
          let res = client.read_message();
          if(res.response == 'Ok') {
              let iter = self._store.get_iter (Gtk.TreePath.new_from_string(old))[1];
              self._store.set_value(iter, 2, new_text);
          } else {
            log("Failed to Edit account password");
          }
        });
        col.set_cell_data_func(pwordColRender, (col, cell, model, iter) => {
          cell.editable = true;
          cell.text = model.get_value(iter, 2);
        });
        this._treeview.append_column(col);

        col = new Gtk.TreeViewColumn({ title: 'Host' })
        let hostColRender = new Gtk.CellRendererText();
        col.pack_start(hostColRender, true);
        col.set_expand(true);
        hostColRender.connect('edited', function (firstArg, old, new_text, user_data) {
          let client = new NirahSocket();
          client.connect();
          client.send_message({ method: 'EditAccountHost', account: 0, host: new_text});
          let res = client.read_message();
          if(res.response == 'Ok') {
              let iter = self._store.get_iter (Gtk.TreePath.new_from_string(old))[1];
              self._store.set_value(iter, 3, new_text);
          } else {
            log("Failed to Edit account host");
          }
        });
        col.set_cell_data_func(hostColRender, (col, cell, model, iter) => {
          cell.editable = true;
          cell.text = model.get_value(iter, 3);
        });
        this._treeview.append_column(col);

        col = new Gtk.TreeViewColumn({ title: 'Activate' })
        let activateColRender = new Gtk.CellRendererToggle();
        col.pack_start(activateColRender, true);
        col.set_expand(false);
        activateColRender.set_radio(true);
        activateColRender.connect('toggled', function (renderer, path) {
          let iter = self._store.get_iter (Gtk.TreePath.new_from_string(path))[1];
          let value = self._store.get_value(iter, 4);
          let client = new NirahSocket();
          client.connect();
          client.send_message({ method: 'EditAccountActivation', account: 0});
          let res = client.read_message();
          if(res.response == 'Ok') {
              self._store.set_value(iter, 4, !value);
          } else {
            log("Failed to Edit account host");
          }
        });
        col.set_cell_data_func(activateColRender, (col, cell, model, iter) => {
          cell.editable = true;
          cell.active = model.get_value(iter, 4);
        });
        this._treeview.append_column(col);
  }
};
