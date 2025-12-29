(() => {
  const STORAGE_KEY = "sidebarCollapsed";

  function applySidebarState(collapsed) {
    document.documentElement.classList.toggle("sidebar-collapsed", collapsed);

    const btn = document.querySelector(".sidebar-toggle");
    if (!btn) return;

    btn.setAttribute("aria-expanded", String(!collapsed));
    btn.setAttribute("aria-label", collapsed ? "Show sidebar" : "Hide sidebar");

    const icon = btn.querySelector("i");
    if (icon) {
      icon.classList.toggle("fa-angle-right", collapsed);
      icon.classList.toggle("fa-angle-left", !collapsed);
    }
  }

  document.addEventListener("DOMContentLoaded", () => {
    const btn = document.querySelector(".sidebar-toggle");
    if (!btn) return;

    // State is applied as early as possible (inline script in <head>),
    // so we only sync the button UI here (no layout changes / animations).
    applySidebarState(document.documentElement.classList.contains("sidebar-collapsed"));

    // Re-enable transitions after first paint.
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        document.documentElement.classList.remove("sidebar-preload");
      });
    });

    btn.addEventListener("click", () => {
      const collapsed = !document.documentElement.classList.contains("sidebar-collapsed");
      applySidebarState(collapsed);
      localStorage.setItem(STORAGE_KEY, collapsed ? "1" : "0");
    });
  });
})();

