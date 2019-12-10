'use-strict';

const Gio = imports.gi.Gio;
const Gtk   = imports.gi.Gtk;
const GObj  = imports.gi.GObject;

const { NirahSocket } = imports.utils.socket;
const { TransactionWidget } = imports.widgets.transaction;

var AccountHistory = class accountHistory {
  constructor() {
      let self = this;
      this.client = new NirahSocket();
      this._component = new Gtk.ListBox({ margin: 7, expand: true });
      this._component.show_all();
  }

  widget() {
    return this._component;
  }

  loadAccount(account) {
    let self = this;
    let req = { method: 'GetAccount', id: account };
    this.client.send_then_expect(req, 'Account', function (item) {
        self.account = item;
    });
    req = { method: 'AccountTransactions', account: account };
    this.client.send_then_expect(req, 'AccountTransactions', function (item) {
      item.transactions.forEach(function (transaction) {
        let req = { method: 'GetContact', id: transaction[1].contact };
        self.client.send_then_expect(req, 'Contact', function (item) {
          self.contact = item.contact;
        });
        self._component.add(new TransactionWidget(self.account, self.contact, transaction).widget());
        self._component.show_all();
      });
    });
  }
};
