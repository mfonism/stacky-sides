let clipBoardIcon = document.querySelector(".share .bi.bi-clipboard");
let checkIcon = document.querySelector(".share .bi.bi-check2");
let alertHolder = document.getElementById("alertHolder");

document.querySelector(".share").addEventListener("click", () => {
  clipBoardIcon.classList.add("d-none");
  checkIcon.classList.remove("d-none");

  setTimeout(() => {
    clipBoardIcon.classList.remove("d-none");
    checkIcon.classList.add("d-none");
    if (alertHolder.firstChild) {
      alertHolder.removeChild(alertHolder.firstChild);
    }
  }, 3000);

  navigator.permissions.query({ name: "clipboard-write" }).then((result) => {
    if (result.state !== "granted" && result.state !== "prompt") {
      return;
    }

    navigator.clipboard
      .writeText(document.getElementById("gameUrl").innerText)
      .then(() => {
        let alertComponent = document.createElement("div");
        alertComponent.classList.add("alert", "alert-info", "py-2");
        alertComponent.setAttribute("role", "alert");
        alertComponent.innerText = "Link copied to clipboard!";

        while (alertHolder.firstChild) {
          alertHolder.removeChild(alertHolder.firstChild);
        }
        alertHolder.appendChild(alertComponent);
      });
  });
});
