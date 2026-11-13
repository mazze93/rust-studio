// Copy buttons, local progress, chapter search and scroll-spy. Pages that
// lack a feature's elements simply skip it. Nothing here makes a request.

document.querySelectorAll<HTMLButtonElement>(".copy").forEach((button) =>
  button.addEventListener("click", async () => {
    const code = button.closest(".codebox")?.querySelector("code");
    if (!code) return;
    try {
      await navigator.clipboard.writeText(code.textContent ?? "");
      button.textContent = "Copied";
    } catch {
      const range = document.createRange();
      range.selectNodeContents(code);
      const selection = getSelection();
      selection?.removeAllRanges();
      selection?.addRange(range);
      button.textContent = "Selected — use ⌘C / Ctrl+C";
    }
    setTimeout(() => (button.textContent = "Copy"), 2500);
  }),
);

const checks = [...document.querySelectorAll<HTMLInputElement>("[data-progress]")];
const progressStatus = document.querySelector("#progress-status");
const showProgress = () => {
  if (progressStatus) {
    const done = checks.filter((c) => c.checked).length;
    progressStatus.textContent = `${done} of ${checks.length} sessions completed.`;
  }
};
checks.forEach((check) => {
  const key = `rust-studio-${check.dataset.progress}`;
  try {
    check.checked = localStorage.getItem(key) === "yes";
  } catch {
    /* storage unavailable: progress simply is not remembered */
  }
  check.addEventListener("change", () => {
    try {
      localStorage.setItem(key, check.checked ? "yes" : "no");
    } catch {
      /* as above */
    }
    showProgress();
  });
});
showProgress();

const search = document.querySelector<HTMLInputElement>("#search");
const links = [...document.querySelectorAll<HTMLAnchorElement>("nav .navlinks a")];
search?.addEventListener("input", () => {
  const query = search.value.trim().toLowerCase();
  let hits = 0;
  links.forEach((link) => {
    const section = document.querySelector(link.hash);
    const hit = !query || (section?.textContent ?? "").toLowerCase().includes(query);
    link.classList.toggle("hidden", !hit);
    if (hit) hits++;
  });
  const status = document.querySelector("#search-status");
  if (status) status.textContent = query ? `${hits} matching chapters` : "";
});

if ("IntersectionObserver" in window && links.length) {
  const observer = new IntersectionObserver(
    (entries) =>
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          links.forEach((a) => a.classList.toggle("active", a.hash === `#${entry.target.id}`));
        }
      }),
    { rootMargin: "-10% 0px -70% 0px" },
  );
  document.querySelectorAll(".chapter").forEach((el) => observer.observe(el));
}
