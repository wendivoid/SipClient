'use-strict';

const Gio = imports.gi.Gio;
const Gtk   = imports.gi.Gtk;
const GObj  = imports.gi.GObject;

const { NirahSocket } = imports.utils.socket;
const { AccountHistory } = imports.widgets.account_history;

var HistoryPage = class historyPage {
  constructor() {
      let self = this;
      this._component = new Gtk.Box({ orientation: Gtk.Orientation.VERTICAL, expand: true });
      this.accStore = new Gtk.ListStore();
      this.accStore.set_column_types([
        GObj.TYPE_INT, GObj.TYPE_STRING
      ]);
      this._accountsCombo = new Gtk.ComboBox({ model: this.accStore });
      this._accountsComboRenderer = new Gtk.CellRendererText();
      this._accountsCombo.pack_start(this._accountsComboRenderer, false);
      this._accountsCombo.add_attribute (this._accountsComboRenderer, "text", 1);
      this._scrolledWindow = new Gtk.ScrolledWindow({ expand: true });
      this._history = new AccountHistory();
      this._addButton = new Gtk.Button({ label: '+' });
      this._component.add(this._accountsCombo);
      this._component.add(this._scrolledWindow);
      this._scrolledWindow.add(this._history.widget());
      this._component.add(this._addButton);
      this.loadAccounts();
      this._accountsCombo.connect('changed', function () {
        let path = self._accountsCombo.get_active().toString();
        let iter = self.accStore.get_iter (Gtk.TreePath.new_from_string(path))[1];
        let accId = self.accStore.get_value(iter, 0);
        self.loadTransactions(accId);
      });
  }

  widget() {
    return this._component;
  }

  loadAccounts() {
    let client = new NirahSocket();
    client.connect();
    let self = this;
    let req = { method: 'AllAccounts' };
    client.send_then_expect(req, 'AllAccounts', function (res) {
      res.accounts.forEach(function (item) {
        self.accStore.set(self.accStore.append(), [0, 1], [item.id, item.username+' '+item.host]);
      });
    });
  }

  loadTransactions(account) {
    this._history.loadAccount(account);
  }
};
