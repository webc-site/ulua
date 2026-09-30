/**
 * ScrollSpy utility using IntersectionObserver and scroll listeners
 * Tracks active section by scroll position and viewport intersection
 */
const scrollSpyInit = (section_id_li, onActiveChange, header_height = 65) => {
  let active_id = section_id_li[0] ?? "",
    is_programmatic_scroll = false,
    scroll_timer = null;

  const sectionUpdate = () => {
    if (is_programmatic_scroll) return;

    const scroll_y = window.scrollY,
      window_height = window.innerHeight,
      doc_height = document.documentElement.scrollHeight,
      is_bottom = scroll_y + window_height >= doc_height - 40;

    if (is_bottom) {
      const last_id = section_id_li[section_id_li.length - 1];
      if (last_id && last_id !== active_id) {
        active_id = last_id;
        onActiveChange(active_id);
      }
      return;
    }

    let target_id =
      section_id_li.findLast((id) => {
        const el = document.getElementById(id);
        if (!el) return false;
        const rect = el.getBoundingClientRect();
        return rect.top <= header_height + 40 && rect.bottom > header_height;
      }) ?? null;

    if (!target_id) {
      target_id =
        section_id_li.find((id) => {
          const el = document.getElementById(id);
          if (!el) return false;
          const rect = el.getBoundingClientRect();
          return rect.top > header_height && rect.top <= window_height * 0.5;
        }) ?? null;
    }

    if (!target_id && section_id_li.length && scroll_y < 200) {
      target_id = section_id_li[0];
    }

    if (target_id && target_id !== active_id) {
      active_id = target_id;
      onActiveChange(active_id);
    }
  };

  const observer = new IntersectionObserver(sectionUpdate, {
    rootMargin: `-${header_height}px 0px -40% 0px`,
    threshold: [0, 0.25, 0.5, 0.75, 1],
  });

  section_id_li.forEach((id) => {
    const el = document.getElementById(id);
    if (el) observer.observe(el);
  });

  window.addEventListener("scroll", sectionUpdate, { passive: true });
  window.addEventListener("resize", sectionUpdate, { passive: true });
  sectionUpdate();

  const destroy = () => {
      observer.disconnect();
      window.removeEventListener("scroll", sectionUpdate);
      window.removeEventListener("resize", sectionUpdate);
      if (scroll_timer) clearTimeout(scroll_timer);
    },
    programmaticSet = () => {
      is_programmatic_scroll = true;
      if (scroll_timer) clearTimeout(scroll_timer);
      scroll_timer = setTimeout(() => {
        is_programmatic_scroll = false;
        sectionUpdate();
      }, 700);
    };

  return {
    destroy,
    programmaticSet,
    setProgrammatic: programmaticSet,
  };
};

export default scrollSpyInit;
export { scrollSpyInit };
