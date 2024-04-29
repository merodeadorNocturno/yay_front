document.body.addEventListener("user_delete_confirmation", (e) => {
  console.log("USER DELETE CONFIRMATION");
});

let ajaxCompleteEvent = new CustomEvent("ajaxComplete", {
  detail: {
    message: "AJAX request completed!",
  },
});

// document.body.addEventListener("user-delete-error", (e) => {
//   console.log("USER DELETE ERROR");
// });

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
  nbs.addEventListener("enterprise_table", (e) => {
    add_skeleton();
    const enterprise_element = e.target;
    const parent_element = enterprise_element.parentNode;

    remove_class(parent_element.childNodes, "is-active");

    enterprise_element.classList.add("is-active");
  });

  nbs.addEventListener("user_table", (e) => {
    add_skeleton();
    const user_element = e.target;
    const parent_element = user_element.parentNode;

    remove_class(parent_element.childNodes, "is-active");

    user_element.classList.add("is-active");
  });

  nbs.addEventListener("help_table", (e) => {
    add_skeleton();
    const help_element = e.target;
    const parent_element = help_element.parentNode;

    remove_class(parent_element.childNodes, "is-active");

    help_element.classList.add("is-active");
  });

  nbs.addEventListener("error_enterprise_table", (e) => {
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

  document.addEventListener("delete-user-alert", (e) => {
    const delete_user_alert = document.getElementById("delete-user-alert");
    delete_user_alert.classList.add("is-active");
  });

  const mc = document.querySelectorAll(".modal-close");
  const m = document.querySelectorAll(".modal");
  for (const modal_close of mc) {
    modal_close.addEventListener("click", (e) =>
      m.forEach((el) => close_modal(el)),
    );
  }
})();
