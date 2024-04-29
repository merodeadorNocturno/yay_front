var State = {
  MAIN_SCREEN: 0,
  OPEN_MODAL: 1,
  SEND_ERASE_COMMAND: 2,
  CLOSE_MODAL: 3,
  DISPLAY_SERVER_ERROR_MESSAGE: 4,
  SUCCESSFUL_OPERATION: 5,
};

var Input = {
  PRESS_TRASH: 0,
  PRESS_DELETE_USER_BUTTON: 1,
  PRESS_CANCEL: 2,
  PRESS_CLOSE_BUTTON: 3,
  RECEIVE_ERROR_FROM_SERVER: 4,
  RECEIVE_CONFIRMATION_FROM_SERVER: 5,
};

// modal id
// delete-user-confirmation-modal
// tache button:
// delete-user-modal-close-button

var all_trash_cans = document.getElementsByClassName("press-trash");
var cancel_delete_user_button = document.getElementById(
  "cancel-delete-user-button",
);
var buffer_name = document.getElementById("buffer-name");
var buffer_id = document.getElementById("buffer-id");

var confirm_delete_user_message = document.getElementById(
  "confirm-delete-user-message",
);
var confirmation_modal = document.getElementById(
  "delete-user-confirmation-modal",
);
var delete_user_modal_close_button = document.getElementById(
  "delete-user-modal-close-button",
);
var user_table = document.getElementById("user-table");
var delete_user_button_modal = document.getElementById(
  "delete-user-button-modal",
);

function keep_state() {
  let previousState = null;
  return (state) => {
    if (state !== undefined) {
      previousState = state;
    }
    return previousState;
  };
}

var ks = keep_state();

function clear_buffer() {
  buffer_id.textContent = "";
  buffer_name.textContent = "";
}

if (all_trash_cans.length) {
  for (const trash of all_trash_cans)
    trash.addEventListener("click", (e) => {
      this_trash = e.currentTarget;
      buffer_id.textContent = this_trash.getAttribute("data-id");
      buffer_name.textContent = this_trash.getAttribute("data-name");
      ks(State.MAIN_SCREEN);
      machine(Input.PRESS_TRASH);
    });
}

cancel_delete_user_button.addEventListener("click", () => {
  clear_buffer();
  ks(State.OPEN_MODAL);
  machine(Input.PRESS_CANCEL);
});

delete_user_modal_close_button.addEventListener("click", () => {
  clear_buffer();
  ks(State.OPEN_MODAL);
  machine(Input.PRESS_CLOSE_BUTTON);
});

delete_user_button_modal.addEventListener("click", () => {
  ks(State.OPEN_MODAL);
  machine(Input.PRESS_DELETE_USER_BUTTON);
});

function open_modal() {
  confirm_delete_user_message.textContent = `¿Desea eliminar al usuario ${buffer_name.innerText}
    permanentemente? Esta acción es definitiva.`;

  confirmation_modal.classList.add("is-active");
}

async function send_erase_command() {
  let deleted_user = "";
  for await (const html_line of iterate_over_stream_response(
    `${backe_url()}/user/delete/${buffer_id.textContent}`,
  )) {
    deleted_user = `${deleted_user}${html_line}`;
  }
}

function close_modal() {
  ks(State.MAIN_SCREEN);
  confirmation_modal.classList.remove("is-active");
  htmx.process("user-table");
}

function display_server_error_message() {}

async function machine(input) {
  switch (ks()) {
    case State.MAIN_SCREEN:
      if (input === Input.PRESS_TRASH) {
        open_modal();
      }
      break;
    case State.OPEN_MODAL:
      if (input === Input.PRESS_DELETE_USER_BUTTON) {
        // const req_url = ;
        await send_erase_command();
      }

      if (input === Input.PRESS_CLOSE_BUTTON) {
        close_modal();
      }

      if (input === Input.PRESS_CANCEL) {
        close_modal();
      }
      break;
    case State.SEND_ERASE_COMMAND:
      if (input === Input.RECEIVE_ERROR_FROM_SERVER) {
        ks(State.DISPLAY_SERVER_ERROR_MESSAGE);
      }
      if (input === Input.RECEIVE_CONFIRMATION_FROM_SERVER) {
        ks(State.SUCCESSFUL_OPERATION);
        close_modal();
      }
      break;
  }
}
