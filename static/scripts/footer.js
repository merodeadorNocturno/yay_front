document.body.addEventListener("user_delete_confirmation", (e) => {
  console.log("USER DELETE CONFIRMATION");
});

let ajaxCompleteEvent = new CustomEvent("ajaxComplete", {
  detail: {
    message: "AJAX request completed!",
  },
});

document.addEventListener("ajaxComplete", function (e) {
  console.log(e.detail.message); // Logs "AJAX request completed!"
  ks(State.SEND_ERASE_COMMAND);
  machine(Input.RECEIVE_CONFIRMATION_FROM_SERVER);
});

document.body.addEventListener("page_error", (e) => {
  error_modal.classList.add("is-active");
  const error_message = document.querySelector("#error_message");
  error_message.innerHTML = e.detail.value;
});

(() => {
  // functions
  const remove_class = (element = [], dom_class = "") => {
    for (const child_node of element) {
      if (child_node.nodeType === Node.ELEMENT_NODE) {
        child_node.classList.remove(dom_class);
      }
    }
  };

  const add_skeleton = () => {
    const section = document.getElementById("dynamic-content");
    section.innerHTML = '<div class="block is-skeleton"></div>';
  };

  const nbs = document.getElementById("navbar-start-id");

  const close_modal = (element) => element.classList.remove("is-active");

  // Listeners
  nbs.addEventListener("activate_navbar_element", (e) => {
    add_skeleton();
    const enterprise_element = e.target;
    const parent_element = enterprise_element.parentNode;

    remove_class(parent_element.childNodes, "is-active");

    enterprise_element.classList.add("is-active");
  });

  nbs.addEventListener("error_enterprise_table", (_e) => {
    const error_modal = document.getElementById("error_enterprise_table_modal");
    error_modal.classList.add("is-active");
  });

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      (document.querySelectorAll(".modal") || []).forEach(($modal) =>
        close_modal($modal),
      );
    }
  });

  document.addEventListener("delete-user-alert", (_e) => {
    const delete_user_alert = document.getElementById("delete-user-alert");
    delete_user_alert.classList.add("is-active");
  });

  const mc = document.querySelectorAll(".modal-close");
  const m = document.querySelectorAll(".modal");
  for (const modal_close of mc) {
    modal_close.addEventListener("click", (_e) =>
      m.forEach((el) => close_modal(el)),
    );
  }
})();
