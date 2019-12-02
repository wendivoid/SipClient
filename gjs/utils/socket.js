'use-strict';

const Gio = imports.gi.Gio;

var NirahSocket = class nirahSocket {
  constructor() {
    this._addr = new Gio.UnixSocketAddress({ path: "/tmp/nirah/nirah-bytebuddha.socket"});
    this._sock = new Gio.SocketClient();
    this._sock.set_family(1);
  }

  connect() {
    try {
      this._conn = this._sock.connect(this._addr, null);
      return true;
    } catch(_err) {
      return false;
    }
  }

  send_message(msg) {
    let data;
    try {
      data = JSON.stringify(msg);
      this._conn.get_output_stream()
          .write_all(data+'\n', null);
    } catch(err) {
      log("Failed to send rpc message");
    }

  }

  read_message() {
    let output_reader = Gio.DataInputStream.new(this._conn.get_input_stream());
    let [output, count] = output_reader.read_line(null);
    return JSON.parse(imports.byteArray.toString(output));
  }
};
