'use-strict';

const Gio = imports.gi.Gio;
const Gtk   = imports.gi.Gtk;
const GObj  = imports.gi.GObject;

const { NirahSocket } = imports.utils.socket;

var AccountsTable = class accountsTable {
  constructor() {
      let self = this;
      this._treeview = new Gtk.TreeView({ margin: 20, expand: true });
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
      client.send_message({ method: 'AllAccounts' });
      let res = client.read_message();
      res.accounts.forEach(function (item) {
        self._store.set(self._store.append(), [0, 1, 2, 3, 4], [item.id, item.username, item.password, item.host, item.activate]);
      });
      this._treeview.set_model(this._store);
  }

  widget() {
    return this._treeview;
  }

  add_columns() {
        let self = this;

        let col = new Gtk.TreeViewColumn({ title: 'Id', expand: false });
        this._treeview.append_column(col);
        let idColRender = new Gtk.CellRendererText();
        col.pack_start(idColRender, true);
        col.set_cell_data_func(idColRender, self.renderId);

        col = new Gtk.TreeViewColumn({ title: 'Username', expand: true })
        let unameColRender = new Gtk.CellRendererText();
        col.pack_start(unameColRender, true);
        unameColRender.connect('edited', function (firstArg, old, new_text, user_data) {
          let client = new NirahSocket();
          let req = client.editAccountUsernameRequest(parseInt(old), new_text);
          client.send_then(req, function () { self.updateUser(old, new_text); });
        });
        col.set_cell_data_func(unameColRender, self.renderUser);
        this._treeview.append_column(col);

        col = new Gtk.TreeViewColumn({ title: 'Password', expand: true })
        let pwordColRender = new Gtk.CellRendererText();
        col.pack_start(pwordColRender, true);
        pwordColRender.connect('edited', function (firstArg, old, password, user_data) {
          let client = new NirahSocket();
          let req = client.editAccountPasswordRequest(parseInt(old), password);
          client.send_then(req, function () { self.updatePass(old, password);});
        });
        col.set_cell_data_func(pwordColRender, self.renderPass);
        this._treeview.append_column(col);

        col = new Gtk.TreeViewColumn({ title: 'Host', expand: true })
        let hostColRender = new Gtk.CellRendererText();
        col.pack_start(hostColRender, true);
        hostColRender.connect('edited', function (firstArg, old, new_text, user_data) {
          let client = new NirahSocket();
          let req = client.editAccountHostnameRequest(parseInt(old), new_text);
          client.send_then(req, function () { self.updateHost(old, new_text) });
        });
        col.set_cell_data_func(hostColRender, self.renderHost);
        this._treeview.append_column(col);

        col = new Gtk.TreeViewColumn({ title: 'Activate', expand: false })
        let activateColRender = new Gtk.CellRendererToggle({ radio: true });
        col.pack_start(activateColRender, true);
        activateColRender.connect('toggled', function (renderer, path) {
          let iter = self._store.get_iter (Gtk.TreePath.new_from_string(path))[1];
          let value = self._store.get_value(iter, 4);
          let client = new NirahSocket();
          client.send_message({ method: 'EditAccountActivation', account: parseInt(path)});
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

AccountsTable.prototype.updateUser = function (user, label) {
  let iter = this._store.get_iter (Gtk.TreePath.new_from_string(user))[1];
  this._store.set_value(iter, 1, label);
};

AccountsTable.prototype.updatePass = function (pass, label) {
  let iter = this._store.get_iter (Gtk.TreePath.new_from_string(pass))[1];
  this._store.set_value(iter, 2, label);
};

AccountsTable.prototype.updateHost = function (host, label) {
  let iter = this._store.get_iter (Gtk.TreePath.new_from_string(host))[1];
  this._store.set_value(iter, 3, label);
};

AccountsTable.prototype.renderUser = function (col, cell, model, iter) {
  cell.editable = true;
  cell.text = model.get_value(iter, 1);
};

AccountsTable.prototype.renderPass = function (col, cell, model, iter) {
  cell.editable = true;
  cell.text = model.get_value(iter, 2);
};

AccountsTable.prototype.renderHost = function (col, cell, model, iter) {
  cell.editable = true;
  cell.text = model.get_value(iter, 3);
};

AccountsTable.prototype.renderId = function (col, cell, model, iter) {
  cell.editable = false;
  cell.text = model.get_value(iter, 0).toString();
}
