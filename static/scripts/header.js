const delete_user_alert_event = new Event("delete-user-alert");
const local_url = () =>
  `${ev.protocol}://${ev.address}${ev.port === "80" ? "" : ":" + ev.port}/`;
const backe_url = () =>
  `${ev.be_protocol}://${ev.be_address}${ev.be_port === "80" || ev.be_port === "443" ? "" : ":" + ev.be_port}/`;

var buffer_name = document.getElementById("buffer-name");
var buffer_id = document.getElementById("buffer-id");

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
