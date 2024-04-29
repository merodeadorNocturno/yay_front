class EnvVars {
  #server_protocol = Symbol("protocol");
  #server_address = Symbol("address");
  #server_port = Symbol("port");
  constructor() {}

  set protocol(prot) {
    this[this.#server_protocol] = prot;
  }

  get protocol() {
    return this[this.#server_protocol];
  }

  set address(addr) {
    this[this.#server_address] = addr;
  }

  get address() {
    return this[this.#server_address];
  }

  set port(port) {
    this[this.#server_port] = port;
  }

  get port() {
    return this[this.#server_port];
  }
}
