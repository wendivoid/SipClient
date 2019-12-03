'use-strict';

const Gio = imports.gi.Gio;
const Gtk   = imports.gi.Gtk;
const GObj  = imports.gi.GObject;

const { NirahSocket } = imports.utils.socket;

var ConfigTable = class configTable {
  constructor() {
      let self = this;
      this._component = new Gtk.VBox({ margin: 20, expand: true });
      let client = new NirahSocket();
      this._store = new Gtk.ListStore();
      this._store.set_column_types([
        GObj.TYPE_STRING, GObj.TYPE_STRING,
        GObj.TYPE_STRING, GObj.TYPE_STRING,
        GObj.TYPE_STRING
      ]);
      this._treeview = new Gtk.TreeView();
      this._keyCol = new Gtk.TreeViewColumn({ expand: false, title: 'Key' });
      this._keyColRender = new Gtk.CellRendererText();
      this._keyCol.set_cell_data_func(this._keyColRender, self.renderKey);
      this._keyCol.pack_start(this._keyColRender, true);
      this._valueCol = new Gtk.TreeViewColumn({ expand: true, title: 'Value' });
      this._valueColRender = new Gtk.CellRendererText();
      this._valueColRender.connect('edited', function (firstArg, old, value, user_data) {
        let client = new NirahSocket();
        client.connect();
        let iter = self._store.get_iter (Gtk.TreePath.new_from_string(old))[1];
        let key = self._store.get_value(iter, 0);
        let ty = self._store.get_value(iter, 3);
        let req = { method: 'SetConfig', key: key, value: { ty, value }};
        client.send_then(req, function () { self.updateValue(old, value); });
      });
      this._valueCol.set_cell_data_func(this._valueColRender, self.renderValue);
      this._valueCol.pack_start(this._valueColRender, true);
      this._defaultCol = new Gtk.TreeViewColumn({ expand: false, title: 'Default' });
      this._defaultCol.set_max_width(300);
      this._defaultColRender = new Gtk.CellRendererText();
      this._defaultCol.set_cell_data_func(this._defaultColRender, self.renderDefault);
      this._defaultCol.pack_start(this._defaultColRender, true);
      this._treeview.set_model(this._store);
      this._treeview.append_column(this._keyCol);
      this._treeview.append_column(this._valueCol);
      this._treeview.append_column(this._defaultCol);
      this._component.add(this._treeview);
      if(client.connect()) {
        let self = this;
        let req = { 'method': 'AllVariables' };
        client.send_then_expect(req, 'AllConfigVariables', function(res) {
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
    let self = this;
    result.vars.forEach(function (item) {
      let key = item.key ? item.key : '';
      let value = item.value ? item.value.value : '';
      let default_value = item.default.value ? item.default.value : '';
      let ty = item.default.ty ? item.default.ty : '';
      let description = item.description ? item.description : '';
      let iter = widget._store.append();
      widget._store.set(iter, [0, 1, 2, 3, 4], [key, value, default_value, ty, description]);
      let tooltip = new  Gtk.Tooltip();
      tooltip.set_text("This is the tooltip");
      let path = self._store.get_path(iter);
      widget._treeview.set_tooltip_column(4);
    });
  }
};

ConfigTable.prototype.renderKey = function (col, cell, model, iter) {
  cell.editable = false;
  cell.text = model.get_value(iter, 0);
};

ConfigTable.prototype.renderValue = function (col, cell, model, iter) {
  cell.editable = true;
  cell.text = model.get_value(iter, 1);
};

ConfigTable.prototype.renderDefault = function (col, cell, model, iter) {
  cell.editable = false;
  cell.text = model.get_value(iter, 2);
};

ConfigTable.prototype.updateValue = function (user, label) {
  let iter = this._store.get_iter (Gtk.TreePath.new_from_string(user))[1];
  this._store.set_value(iter, 1, label);
};
