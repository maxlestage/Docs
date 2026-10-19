// Le site dans Chromium : chaque page sans erreur JavaScript ni défilement de côté (ni du
// code ni des tableaux sur téléphone), sur téléphone et sur ordinateur, en clair et en
// sombre ; la recherche, le thème, le menu des chapitres et le bouton « Copier ».
//
//   node tests/navigateur.js http://localhost:8000/Docs
// (l'export statique, servi par n'importe quel serveur de fichiers : le serveur Paquets,
// lui, limite le nombre de requêtes par visiteur)
// (Playwright doit être installé : npm install playwright)

const { chromium } = require("playwright");

const site = (process.argv[2] || "http://localhost:3000").replace(/\/$/, "");
const origin = new URL(site).origin;
const failures = [];
const check = (ok, what) => { if (!ok) failures.push(what); };

(async () => {
  const browser = await chromium.launch(process.env.CHROMIUM ? { executablePath: process.env.CHROMIUM } : {});
  const paths = await (await fetch(site + "/sitemap.xml")).text()
    .then((xml) => [...xml.matchAll(/<loc>([^<]*)<\/loc>/g)].map((m) => new URL(m[1]).pathname));
  check(paths.length > 30, `sitemap: ${paths.length} pages`);

  for (const [width, height, scheme] of [[390, 844, "light"], [1440, 900, "dark"]]) {
    const page = await browser.newPage({ viewport: { width, height }, colorScheme: scheme });
    const errors = [];
    page.on("pageerror", (e) => errors.push(e.message));
    page.on("console", (m) => { if (m.type() === "error") errors.push(m.text()); });
    for (const path of paths) {
      const res = await page.goto(origin + path, { waitUntil: "load" });
      check(res.status() === 200, `${path}: ${res.status()}`);
      const wide = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
      check(wide <= 0, `${path} (${width}px): ${wide}px de trop en largeur`);
      // Sur téléphone, ni le code ni les tableaux ne défilent de côté non plus.
      if (width < 640) {
        const scrolls = await page.evaluate(() => [...document.querySelectorAll(".prose pre, .prose .table-wrap")]
          .filter((el) => el.scrollWidth > el.clientWidth + 1).length);
        check(scrolls === 0, `${path} (${width}px): ${scrolls} bloc(s) qui défilent de côté`);
      }
    }
    check(errors.length === 0, `${width}px: ${errors.join(" | ")}`);
    await page.close();
  }

  const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
  await page.goto(site + "/rust/possession/", { waitUntil: "networkidle" });
  // La recherche : `/`, un mot, Entrée.
  await page.waitForSelector("[data-search]:not([hidden])");
  await page.keyboard.press("/");
  await page.locator(".search__input").fill("closure");
  await page.waitForSelector(".search__results [role=option]");
  const first = await page.locator(".search__results [role=option] .search__title").first().textContent();
  check(/closure/i.test(first), `recherche « closure » : ${first}`);
  const href = await page.locator(".search__results [role=option] a").first().getAttribute("href");
  await page.keyboard.press("Enter");
  await page.waitForURL((url) => url.href.endsWith(href));
  check(page.url().endsWith(href), `Entrée ouvre le résultat : ${page.url()}`);
  // Le thème.
  await page.click("#theme-toggle");
  const theme = await page.evaluate(() => document.documentElement.getAttribute("data-theme"));
  check(theme === "dark" || theme === "light", `thème : ${theme}`);
  // Le sommaire suit la lecture.
  await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight / 2));
  await page.waitForTimeout(300);
  check(await page.locator(".toc a.is-active").count() === 1, "le sommaire montre la section lue");
  await page.close();

  // Le menu des chapitres, sur téléphone.
  const phone = await browser.newPage({ viewport: { width: 390, height: 844 } });
  await phone.goto(site + "/ruby/fonctions/");
  await phone.click(".menu-btn");
  check(await phone.locator(".site.menu-open").count() === 1, "le menu des chapitres s'ouvre");
  await phone.keyboard.press("Escape");
  check(await phone.locator(".site.menu-open").count() === 0, "Échap ferme le menu");
  await phone.close();

  // Copier un bloc de code.
  const context = await browser.newContext({ permissions: ["clipboard-read", "clipboard-write"] });
  const copy = await context.newPage();
  await copy.goto(site + "/rythme/fonctions/");
  await copy.locator(".code").first().hover();
  await copy.locator(".code__copy").first().click();
  await copy.waitForTimeout(200);
  check((await copy.locator(".code__copy").first().textContent()).includes("Copié"), "le bouton Copier répond");
  await context.close();

  // En anglais et en espagnol : l'interface, et le sélecteur de langue.
  const en = await browser.newContext({ permissions: ["clipboard-read", "clipboard-write"], viewport: { width: 1280, height: 800 } });
  const english = await en.newPage();
  await english.goto(site + "/en/rust/fonctions/");
  check(await english.locator("html[lang=en]").count() === 1, "la page anglaise est en anglais");
  await english.locator(".code").first().hover();
  await english.locator(".code__copy").first().click();
  await english.waitForTimeout(200);
  check((await english.locator(".code__copy").first().textContent()).includes("Copied"), "Copy en anglais");
  await english.locator(".bar__inner > .lang-switch .lang-switch__item[hreflang=es]").click();
  await english.waitForURL((url) => url.pathname.endsWith("/es/rust/fonctions/"));
  check(await english.locator("html[lang=es]").count() === 1, "le sélecteur mène à la page espagnole");
  await en.close();

  await browser.close();
  if (failures.length) {
    console.error(failures.map((f) => "✗ " + f).join("\n"));
    process.exit(1);
  }
  console.log(`✓ ${paths.length} pages, sur téléphone et sur ordinateur, et les interactions`);
})().catch((e) => { console.error(e); process.exit(1); });
