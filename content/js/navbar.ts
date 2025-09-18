const open_btn: HTMLAnchorElement | null = document.querySelector("#nav-dialog-open");
const close_btn: HTMLAnchorElement | null = document.querySelector("#nav-dialog-close")
const dialog: HTMLDialogElement | null = document.querySelector("#nav-dialog");

const toggled = false;

open_btn?.addEventListener("click", () => {
    dialog?.showModal();
    open_btn.style.display = 'none';
});

close_btn?.addEventListener("click", () => {
    dialog?.close();
    if (!open_btn) return;
    open_btn.style.display = '';
});
