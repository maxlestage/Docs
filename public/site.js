// Docs : le thème, la recherche, les boutons « Copier », le sommaire qui suit la
// lecture, la barre de progression et le menu des chapitres sur téléphone.
// Sans ce script, le site reste lisible : ces boutons sont simplement cachés.
(function () {
  "use strict";

  var root = document.documentElement;

  function ready(fn) {
    if (document.readyState !== "loading") fn();
    else document.addEventListener("DOMContentLoaded", fn);
  }

  // ----- Thème ----------------------------------------------------------------
  function isDark() {
    var t = root.getAttribute("data-theme");
    if (t) return t === "dark";
    return window.matchMedia("(prefers-color-scheme: dark)").matches;
  }

  function setupTheme() {
    var btn = document.getElementById("theme-toggle");
    if (!btn) return;
    btn.hidden = false;
    btn.addEventListener("click", function () {
      var next = isDark() ? "light" : "dark";
      root.setAttribute("data-theme", next);
      try { localStorage.setItem("theme", next); } catch (e) {}
    });
  }

  // ----- Copier le code -------------------------------------------------------
  function setupCopy() {
    document.addEventListener("click", function (e) {
      var btn = e.target.closest(".code__copy");
      if (!btn) return;
      var code = btn.closest(".code").querySelector("pre code");
      var text = code ? code.textContent : "";
      var done = function () {
        btn.textContent = "Copié ✓";
        btn.classList.add("is-done");
        setTimeout(function () { btn.textContent = "Copier"; btn.classList.remove("is-done"); }, 1600);
      };
      if (navigator.clipboard && navigator.clipboard.writeText) {
        navigator.clipboard.writeText(text).then(done, function () {});
      }
    });
  }

  // ----- Sommaire et progression ----------------------------------------------
  function setupReading() {
    var links = document.querySelectorAll("[data-toc]");
    if (links.length && "IntersectionObserver" in window) {
      var byId = {};
      links.forEach(function (a) { byId[a.getAttribute("data-toc")] = a; });
      var visible = {};
      var observer = new IntersectionObserver(function (entries) {
        entries.forEach(function (en) { visible[en.target.id] = en.isIntersecting; });
        var heads = document.querySelectorAll(".prose h2[id], .prose h3[id]");
        var current = null;
        for (var i = 0; i < heads.length; i++) {
          if (visible[heads[i].id]) { current = heads[i].id; break; }
          if (heads[i].getBoundingClientRect().top < 120) current = heads[i].id;
        }
        links.forEach(function (a) { a.classList.toggle("is-active", a.getAttribute("data-toc") === current); });
      }, { rootMargin: "-60px 0px -60% 0px" });
      document.querySelectorAll(".prose h2[id], .prose h3[id]").forEach(function (h) { observer.observe(h); });
    }
    var bar = document.querySelector(".page-doc .progress");
    if (bar) {
      var ticking = false;
      var update = function () {
        ticking = false;
        var max = document.documentElement.scrollHeight - window.innerHeight;
        var read = max > 0 ? Math.min(1, Math.max(0, window.scrollY / max)) : 0;
        bar.style.setProperty("--read", read.toFixed(4));
      };
      window.addEventListener("scroll", function () {
        if (!ticking) { ticking = true; requestAnimationFrame(update); }
      }, { passive: true });
      update();
    }
  }

  // ----- Menu des chapitres (téléphone) ----------------------------------------
  function setupMenu() {
    var btn = document.querySelector(".menu-btn");
    var site = document.querySelector(".site");
    if (!btn || !site) return;
    var set = function (open) {
      site.classList.toggle("menu-open", open);
      btn.setAttribute("aria-expanded", open ? "true" : "false");
      if (open) {
        var current = document.querySelector(".sidebar .is-current");
        if (current) current.focus();
      }
    };
    btn.addEventListener("click", function () { set(!site.classList.contains("menu-open")); });
    document.addEventListener("click", function (e) {
      if (e.target.closest("[data-close-menu]")) set(false);
    });
    document.addEventListener("keydown", function (e) {
      if (e.key === "Escape" && site.classList.contains("menu-open")) { set(false); btn.focus(); }
    });
  }

  // ----- Recherche ------------------------------------------------------------
  function norm(s) {
    return s.toLowerCase().normalize("NFD").replace(/[̀-ͯ]/g, "").replace(/œ/g, "oe");
  }

  function setupSearch() {
    var dialog = document.getElementById("search");
    if (!dialog || typeof dialog.showModal !== "function") return;
    var input = dialog.querySelector(".search__input");
    var list = dialog.querySelector(".search__results");
    var chips = dialog.querySelectorAll("[data-filter]");
    var index = null;
    var loading = null;
    var filter = "";
    var selected = 0;
    var results = [];

    document.querySelectorAll("[data-search]").forEach(function (b) {
      b.hidden = false;
      b.addEventListener("click", open);
    });

    function load() {
      if (index || loading) return loading;
      loading = fetch(dialog.getAttribute("data-index"))
        .then(function (r) { return r.json(); })
        .then(function (data) {
          index = data.map(function (e) {
            e.nt = norm(e.t);
            e.nd = norm(e.d || "");
            e.nx = e.x ? norm(e.x) : "";
            return e;
          });
          run();
        })
        .catch(function () {
          list.innerHTML = "";
          var li = document.createElement("li");
          li.className = "search__empty";
          li.textContent = "La recherche n'a pas pu se charger.";
          list.appendChild(li);
        });
      return loading;
    }

    function open() {
      if (dialog.open) return;
      dialog.showModal();
      input.select();
      load();
      run();
    }

    function score(e, words) {
      var s = 0;
      for (var i = 0; i < words.length; i++) {
        var w = words[i];
        var t = e.nt.indexOf(w);
        if (t === 0) s += 12;
        else if (t > 0) s += 8;
        else if (e.nd.indexOf(w) >= 0) s += 3;
        else if (e.nx.indexOf(w) >= 0) s += 1;
        else return 0;
      }
      if (e.k === "page") s += 2;
      return s;
    }

    function mark(parent, text, words) {
      var n = norm(text);
      var ranges = [];
      words.forEach(function (w) {
        var at = n.indexOf(w);
        if (at >= 0 && w) ranges.push([at, at + w.length]);
      });
      ranges.sort(function (a, b) { return a[0] - b[0]; });
      var pos = 0;
      ranges.forEach(function (r) {
        if (r[0] < pos) return;
        parent.appendChild(document.createTextNode(text.slice(pos, r[0])));
        var m = document.createElement("mark");
        m.textContent = text.slice(r[0], r[1]);
        parent.appendChild(m);
        pos = r[1];
      });
      parent.appendChild(document.createTextNode(text.slice(pos)));
    }

    function run() {
      var q = norm(input.value.trim());
      var words = q.split(/\s+/).filter(Boolean);
      list.innerHTML = "";
      if (!index) return;
      var pool = index.filter(function (e) { return !filter || e.p === filter || e.p === "Docs"; });
      if (!words.length) {
        results = pool.filter(function (e) { return e.k === "page"; }).slice(0, 40);
      } else {
        results = pool
          .map(function (e) { return { e: e, s: score(e, words) }; })
          .filter(function (r) { return r.s > 0; })
          .sort(function (a, b) { return b.s - a.s; })
          .slice(0, 40)
          .map(function (r) { return r.e; });
      }
      if (!results.length) {
        var li = document.createElement("li");
        li.className = "search__empty";
        li.textContent = "Rien trouvé pour « " + input.value.trim() + " ». Essayez un mot plus court.";
        list.appendChild(li);
        return;
      }
      results.forEach(function (e, i) {
        var li = document.createElement("li");
        li.setAttribute("role", "option");
        li.id = "result-" + i;
        var a = document.createElement("a");
        a.href = e.u;
        var cls = { Ruby: "t-ruby", Rust: "t-rust", Rythme: "t-rythme", "Débuter": "t-debuter" }[e.p] || "";
        if (cls) li.className = cls;
        var kind = document.createElement("span");
        kind.className = "search__kind";
        kind.setAttribute("aria-hidden", "true");
        kind.textContent = e.k === "page" ? ({ Ruby: "Rb", Rust: "Rs", Rythme: "Ry", "Débuter": "✦" }[e.p] || "⇄") : "#";
        var title = document.createElement("span");
        title.className = "search__title";
        mark(title, e.t, words);
        var meta = document.createElement("span");
        meta.className = "search__meta";
        meta.textContent = e.k === "page" ? e.p + " · " + (e.d || "") : e.p + " › " + e.d;
        a.appendChild(kind);
        a.appendChild(title);
        a.appendChild(meta);
        li.appendChild(a);
        list.appendChild(li);
      });
      select(0);
    }

    function select(i) {
      var items = list.querySelectorAll("[role=option]");
      if (!items.length) return;
      selected = (i + items.length) % items.length;
      items.forEach(function (li, k) { li.setAttribute("aria-selected", k === selected ? "true" : "false"); });
      items[selected].scrollIntoView({ block: "nearest" });
      input.setAttribute("aria-activedescendant", items[selected].id);
    }

    input.addEventListener("input", run);
    input.addEventListener("keydown", function (e) {
      if (e.key === "ArrowDown") { e.preventDefault(); select(selected + 1); }
      else if (e.key === "ArrowUp") { e.preventDefault(); select(selected - 1); }
      else if (e.key === "Enter") {
        var r = results[selected];
        if (r) { e.preventDefault(); location.href = r.u; dialog.close(); }
      }
    });
    chips.forEach(function (c) {
      c.addEventListener("click", function () {
        filter = c.getAttribute("data-filter");
        chips.forEach(function (o) { o.classList.toggle("is-on", o === c); });
        run();
        input.focus();
      });
    });
    dialog.querySelector(".search__close").addEventListener("click", function () { dialog.close(); });
    dialog.addEventListener("click", function (e) { if (e.target === dialog) dialog.close(); });
    list.addEventListener("click", function () { dialog.close(); });

    document.addEventListener("keydown", function (e) {
      var tag = (e.target.tagName || "").toLowerCase();
      var typing = tag === "input" || tag === "textarea" || e.target.isContentEditable;
      if ((e.key === "k" || e.key === "K") && (e.metaKey || e.ctrlKey)) { e.preventDefault(); open(); }
      else if (e.key === "/" && !typing && !dialog.open) { e.preventDefault(); open(); }
    });
  }

  // ----- Chapitres lus ---------------------------------------------------------
  // Retenus dans ce navigateur seulement (localStorage), jamais envoyés.
  function setupDone() {
    var KEY = "docs-lus";
    var read = function () {
      try { return JSON.parse(localStorage.getItem(KEY) || "[]"); } catch (e) { return []; }
    };
    var save = function (list) {
      try { localStorage.setItem(KEY, JSON.stringify(list)); } catch (e) {}
    };
    var paint = function () {
      var done = read();
      document.querySelectorAll("[data-key]").forEach(function (a) {
        a.classList.toggle("is-done", done.indexOf(a.getAttribute("data-key")) >= 0);
      });
      document.querySelectorAll("[data-done-track]").forEach(function (el) {
        var prefix = el.getAttribute("data-done-track") + "/";
        var n = done.filter(function (k) { return k.indexOf(prefix) === 0; }).length;
        el.textContent = n ? n + "/" + el.getAttribute("data-done-total") + " lus" : "";
      });
      document.querySelectorAll("[data-done]").forEach(function (b) {
        var on = done.indexOf(b.getAttribute("data-done")) >= 0;
        b.setAttribute("aria-pressed", on ? "true" : "false");
        b.querySelector(".done__text").textContent = on ? "Chapitre lu" : "Marquer ce chapitre comme lu";
      });
    };
    document.querySelectorAll("[data-done]").forEach(function (b) {
      b.hidden = false;
      b.addEventListener("click", function () {
        var key = b.getAttribute("data-done");
        var done = read();
        var at = done.indexOf(key);
        if (at >= 0) done.splice(at, 1); else done.push(key);
        save(done);
        paint();
      });
    });
    paint();
  }

  ready(function () {
    setupDone();
    setupTheme();
    setupCopy();
    setupReading();
    setupMenu();
    setupSearch();
  });
})();
