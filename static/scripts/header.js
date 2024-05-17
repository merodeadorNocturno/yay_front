class UrlEnvVars {
  #server_protocol = Symbol("protocol");
  #server_address = Symbol("address");
  #server_port = Symbol("port");
  #back_end_protocol = Symbol("be_protocol");
  #back_end_address = Symbol("be_address");
  #back_end_port = Symbol("be_port");
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

  set be_protocol(prot) {
    this[this.#back_end_protocol] = prot;
  }

  get be_protocol() {
    return this[this.#back_end_protocol];
  }

  set be_address(addr) {
    this[this.#back_end_address] = addr;
  }

  get be_address() {
    return this[this.#back_end_address];
  }

  set be_port(port) {
    this[this.#back_end_port] = port;
  }

  get be_port() {
    return this[this.#back_end_port];
  }
}

const ev = new UrlEnvVars();
ev.protocol = "http";
// ev.address = "crm.yayleads.mx";
ev.address = "0.0.0.0";
// ev.port = "80";
ev.port = "8081";
// ev.be_port = "80";
ev.be_port = "8080";
// ev.be_protocol = "https";
ev.be_protocol = "http";
// ev.be_address = "192.168.68.101";
ev.be_address = "api.yayleads.mx";

const delete_user_alert_event = new Event("delete-user-alert");
const local_url = () =>
  `${ev.protocol}://${ev.address}${ev.port === "80" ? "" : ":" + ev.port}/`;
const backe_url = () =>
  `${ev.be_protocol}://${ev.be_address}${ev.be_port === "80" || ev.be_port === "443" ? "" : ":" + ev.be_port}/`;

var buffer_name = document.getElementById("buffer-name");
var buffer_id = document.getElementById("buffer-id");

// htmx.logAll();

function trigger_delete($e) {
  const data = $e.getAttribute("data");
  const delete_user_button = document.getElementById("delete-user-button");

  const hx_delete = document.createAttribute("hx-delete");
  hx_delete.value = `${local_url()}users/${data}`;

  delete_user_button.setAttributeNode(hx_delete);

  htmx.process(delete_user_button);

  document.dispatchEvent(delete_user_alert_event);
}

async function remove_class_delete_user(req_url = `${local_url}htmx/user`) {
  const delete_user_modal = document.getElementById("delete-user-alert");
  const dynamic_content = document.getElementById("dynamic-content");

  dynamic_content.innerHTML =
    '<textarea class="textarea is-skeleton"></textarea>';
  let users_table = "";

  for await (const line of iterate_over_stream_response(`${req_url}`)) {
    users_table = `${users_table}${line}`;
  }

  dynamic_content.innerHTML = users_table;

  htmx.process(dynamic_content);
  delete_user_modal.classList.remove("is-active");
}

async function testHttpEvent(req_url) {
  let users_table = "";

  for await (const line of iterate_over_stream_response(
    `${req_url}user/delete/${buffer_id.textContent}`,
  )) {
    users_table = `${users_table}${line}`;
  }

  document.dispatchEvent(ajaxCompleteEvent);
}

async function* iterate_over_stream_response(file_url) {
  const user_table_content = await fetch(file_url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
  });

  const utf8Decoder = new TextDecoder("utf-8");
  const reader = user_table_content.body.getReader();
  let { value: chunk, done: readerDone } = await reader.read();
  chunk = chunk ? utf8Decoder.decode(chunk) : "";

  const newline = /\r?\n/gm;
  let startIndex = 0;

  while (true) {
    const result = newline.exec(chunk);
    if (!result) {
      if (readerDone) {
        break;
      }
      const remainder = chunk.substr(startIndex);
      ({ value: chunk, done: readerDone } = await reader.read());

      chunk = remainder + (chunk ? utf8Decoder.decode(chunk) : "");
      startIndex = newline.lastIndex = 0;
      continue;
    }
    yield chunk.substring(startIndex, result.index);
    startIndex = newline.lastIndex;
  }

  if (startIndex < chunk.length) {
    yield chunk.substr(startIndex);
  }
}
