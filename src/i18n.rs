//! Les trois langues du site : le français (aux adresses d'origine), l'anglais
//! (`/en/…`) et l'espagnol (`/es/…`). Les textes de l'interface sont écrits en français
//! dans le code et traduits ici par `tr` ; les pages traduites sont dans
//! `content/en/` et `content/es/`.

use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Lang {
    Fr,
    En,
    Es,
}

impl Lang {
    pub const ALL: [Lang; 3] = [Lang::Fr, Lang::En, Lang::Es];

    /// `fr`, `en`, `es`.
    pub fn code(self) -> &'static str {
        match self {
            Lang::Fr => "fr",
            Lang::En => "en",
            Lang::Es => "es",
        }
    }

    /// Le début des adresses : `""`, `/en` ou `/es`.
    pub fn prefix(self) -> &'static str {
        match self {
            Lang::Fr => "",
            Lang::En => "/en",
            Lang::Es => "/es",
        }
    }

    pub fn locale(self) -> &'static str {
        match self {
            Lang::Fr => "fr_FR",
            Lang::En => "en_US",
            Lang::Es => "es_ES",
        }
    }

    /// Le nom de la langue, dans cette langue.
    pub fn name(self) -> &'static str {
        match self {
            Lang::Fr => "Français",
            Lang::En => "English",
            Lang::Es => "Español",
        }
    }

    /// La langue d'un code (`en`), sauf le français, qui n'a pas de préfixe.
    pub fn from_prefix(code: &str) -> Option<Lang> {
        match code {
            "en" => Some(Lang::En),
            "es" => Some(Lang::Es),
            _ => None,
        }
    }

    fn at(self) -> usize {
        self as usize
    }

    /// Choisit parmi trois textes : (français, anglais, espagnol).
    pub fn pick<T>(self, fr: T, en: T, es: T) -> T {
        match self {
            Lang::Fr => fr,
            Lang::En => en,
            Lang::Es => es,
        }
    }
}

/// Le texte français `fr`, dans la langue `lang` (le français s'il n'est pas traduit :
/// un test vérifie que chaque texte de l'interface l'est).
pub fn tr(lang: Lang, fr: &'static str) -> &'static str {
    if lang == Lang::Fr {
        return fr;
    }
    TABLE.get(fr).map_or(fr, |t| t[lang.at() - 1])
}

static TABLE: LazyLock<HashMap<&'static str, [&'static str; 2]>> =
    LazyLock::new(|| TEXTS.iter().map(|[fr, en, es]| (*fr, [*en, *es])).collect());

/// Le nombre de chapitres : « 12 chapitres ».
pub fn chapters(lang: Lang, n: usize) -> String {
    let word = match (lang, n > 1) {
        (Lang::Fr, true) => "chapitres",
        (Lang::Fr, false) => "chapitre",
        (Lang::En, true) => "chapters",
        (Lang::En, false) => "chapter",
        (Lang::Es, true) => "capítulos",
        (Lang::Es, false) => "capítulo",
    };
    format!("{n} {word}")
}

/// « Chapitre 3 sur 12 ».
pub fn chapter_of(lang: Lang, n: usize, total: usize) -> String {
    match lang {
        Lang::Fr => format!("Chapitre {n} sur {total}"),
        Lang::En => format!("Chapter {n} of {total}"),
        Lang::Es => format!("Capítulo {n} de {total}"),
    }
}

/// « 5 min de lecture ».
pub fn reading(lang: Lang, minutes: usize) -> String {
    match lang {
        Lang::Fr => format!("{minutes} min de lecture"),
        Lang::En => format!("{minutes} min read"),
        Lang::Es => format!("{minutes} min de lectura"),
    }
}

/// « Chapitres de Rust ».
pub fn chapters_of(lang: Lang, name: &str) -> String {
    match lang {
        Lang::Fr => format!("Chapitres de {name}"),
        Lang::En => format!("{name} chapters"),
        Lang::Es => format!("Capítulos de {name}"),
    }
}

/// « Une fonction en Rust ».
pub fn idea_in(lang: Lang, idea: &str, name: &str) -> String {
    match lang {
        Lang::Fr | Lang::Es => format!("{idea} en {name}"),
        Lang::En => format!("{idea} in {name}"),
    }
}

/// « Le parcours Rust → ».
pub fn track_link(lang: Lang, name: &str) -> String {
    match lang {
        Lang::Fr => format!("Le parcours {name} →"),
        Lang::En => format!("The {name} track →"),
        Lang::Es => format!("El curso de {name} →"),
    }
}

/// La suite du bandeau « Jamais programmé ? ».
pub fn debuter_pages(lang: Lang, n: usize) -> String {
    match lang {
        Lang::Fr => format!(
            " {n} pages pour comprendre ce qu'est un programme, utiliser le terminal, un éditeur et Git, corriger ses erreurs et choisir son langage."
        ),
        Lang::En => format!(
            " {n} pages to understand what a program is, use the terminal, an editor and Git, fix your mistakes and choose your language."
        ),
        Lang::Es => format!(
            " {n} páginas para entender qué es un programa, usar la terminal, un editor y Git, corregir tus errores y elegir tu lenguaje."
        ),
    }
}

/// Le bandeau d'une page pas encore traduite.
pub fn untranslated(lang: Lang) -> &'static str {
    lang.pick(
        "",
        "This chapter is not translated into English yet, so here it is in French. The code works the same, and the other chapters may already be in English.",
        "Este capítulo todavía no está traducido al español, así que aquí está en francés. El código funciona igual, y otros capítulos quizás ya estén en español.",
    )
}

/// Ce qu'un parcours dit de lui-même, traduit : (nom, titre, phrase, description,
/// nom du site officiel, adresse du site officiel).
pub struct Meta {
    pub name: &'static str,
    pub title: &'static str,
    pub tagline: &'static str,
    pub description: &'static str,
    pub site: (&'static str, &'static str),
}

/// La présentation du parcours `slug` en anglais ou en espagnol (`None` : en français,
/// celle du catalogue).
pub fn meta(lang: Lang, slug: &str) -> Option<Meta> {
    let m = |name, title, tagline, description, site| {
        Some(Meta {
            name,
            title,
            tagline,
            description,
            site,
        })
    };
    match (lang, slug) {
        (Lang::Fr, _) => None,
        (Lang::En, "debuter") => m(
            "Start here",
            "Getting started with programming",
            "Never programmed? Everything starts here.",
            "For people who have never programmed: what a program is, thinking in algorithms, the terminal, a code editor, fixing your mistakes, Git, which language to choose, and a glossary.",
            ("The languages side by side", "/comparer/"),
        ),
        (Lang::Es, "debuter") => m(
            "Empezar",
            "Empezar a programar",
            "¿Nunca has programado? Todo empieza aquí.",
            "Para quienes nunca han programado: qué es un programa, pensar un algoritmo, la terminal, un editor de código, corregir tus errores, Git, qué lenguaje elegir y un glosario.",
            ("Los lenguajes lado a lado", "/comparer/"),
        ),
        (Lang::En, "python") => m(
            "Python",
            "Learn Python",
            "The most used language for learning, and much more.",
            "Learn Python step by step: variables, conditions, functions, text, lists and dictionaries, comprehensions, classes, type hints, generators, decorators, errors, modules, files, JSON, regex, dates, asyncio, the web, tests, guided projects and solved exercises.",
            ("python.org", "https://www.python.org/"),
        ),
        (Lang::Es, "python") => m(
            "Python",
            "Aprender Python",
            "El lenguaje más usado para aprender, y mucho más.",
            "Aprende Python paso a paso: variables, condiciones, funciones, texto, listas y diccionarios, comprensiones, clases, anotaciones de tipo, generadores, decoradores, errores, módulos, archivos, JSON, regex, fechas, asyncio, web, tests, proyectos guiados y ejercicios resueltos.",
            ("python.org", "https://www.python.org/es/"),
        ),
        (Lang::En, "javascript") => m(
            "JavaScript",
            "Learn JavaScript",
            "The language of the web, in the browser and on the server.",
            "Learn JavaScript step by step: variables, conditions, functions, text, arrays, objects, classes, closures, iterators, errors, modules and npm, promises and async/await, the DOM, files with Node, JSON, regex, dates, the web, tests, guided projects and solved exercises.",
            (
                "developer.mozilla.org",
                "https://developer.mozilla.org/en-US/docs/Web/JavaScript",
            ),
        ),
        (Lang::Es, "javascript") => m(
            "JavaScript",
            "Aprender JavaScript",
            "El lenguaje de la web, en el navegador y en el servidor.",
            "Aprende JavaScript paso a paso: variables, condiciones, funciones, texto, arrays, objetos, clases, closures, iteradores, errores, módulos y npm, promesas y async/await, el DOM, archivos con Node, JSON, regex, fechas, web, tests, proyectos guiados y ejercicios resueltos.",
            (
                "developer.mozilla.org",
                "https://developer.mozilla.org/es/docs/Web/JavaScript",
            ),
        ),
        (Lang::En, "typescript") => m(
            "TypeScript",
            "Learn TypeScript",
            "JavaScript with types: errors found before running.",
            "Learn TypeScript step by step: basic types, functions, arrays and tuples, interfaces, unions and narrowing, classes, generics, literal types, advanced types, errors, modules, async, tsconfig, migrating from JavaScript, the web, tests, guided projects and solved exercises.",
            ("typescriptlang.org", "https://www.typescriptlang.org/"),
        ),
        (Lang::Es, "typescript") => m(
            "TypeScript",
            "Aprender TypeScript",
            "JavaScript con tipos: los errores aparecen antes de ejecutar.",
            "Aprende TypeScript paso a paso: tipos básicos, funciones, arrays y tuplas, interfaces, uniones y estrechamiento, clases, genéricos, tipos literales, tipos avanzados, errores, módulos, async, tsconfig, migrar desde JavaScript, web, tests, proyectos guiados y ejercicios resueltos.",
            ("typescriptlang.org", "https://www.typescriptlang.org/es/"),
        ),
        (Lang::En, "go") => m(
            "Go",
            "Learn Go",
            "Simple, fast, made for servers and tools.",
            "Learn Go step by step: variables, conditions, functions, text, slices and maps, structs and methods, interfaces, pointers, generics, errors, packages, goroutines and channels, files, JSON, regex, dates, the web with net/http, tests, guided projects and solved exercises.",
            ("go.dev", "https://go.dev/"),
        ),
        (Lang::Es, "go") => m(
            "Go",
            "Aprender Go",
            "Simple, rápido, hecho para servidores y herramientas.",
            "Aprende Go paso a paso: variables, condiciones, funciones, texto, slices y maps, estructuras y métodos, interfaces, punteros, genéricos, errores, paquetes, goroutines y canales, archivos, JSON, regex, fechas, web con net/http, tests, proyectos guiados y ejercicios resueltos.",
            ("go.dev", "https://go.dev/"),
        ),
        (Lang::En, "ruby") => m(
            "Ruby",
            "Learn Ruby",
            "The language designed for developer happiness.",
            "Learn Ruby step by step: variables, conditions, methods, text, arrays and hashes, blocks, classes, modules, errors, files, tests, gems, guided projects and solved exercises.",
            ("ruby-lang.org", "https://www.ruby-lang.org/en/"),
        ),
        (Lang::Es, "ruby") => m(
            "Ruby",
            "Aprender Ruby",
            "El lenguaje pensado para la felicidad de quien programa.",
            "Aprende Ruby paso a paso: variables, condiciones, métodos, texto, arrays y hashes, bloques, clases, módulos, errores, archivos, tests, gems, proyectos guiados y ejercicios resueltos.",
            ("ruby-lang.org", "https://www.ruby-lang.org/es/"),
        ),
        (Lang::En, "rust") => m(
            "Rust",
            "Learn Rust",
            "Fast, reliable, and a compiler that helps you.",
            "Learn Rust step by step: variables, functions, text, collections, ownership and borrowing, structs, traits, errors, iterators, modules, files, Box and Rc, threads, tests, Cargo, guided projects and solved exercises.",
            ("rust-lang.org", "https://www.rust-lang.org/"),
        ),
        (Lang::Es, "rust") => m(
            "Rust",
            "Aprender Rust",
            "Rápido, fiable, y un compilador que te ayuda.",
            "Aprende Rust paso a paso: variables, funciones, texto, colecciones, propiedad y préstamo, estructuras, traits, errores, iteradores, módulos, archivos, Box y Rc, hilos, tests, Cargo, proyectos guiados y ejercicios resueltos.",
            ("rust-lang.org", "https://www.rust-lang.org/es"),
        ),
        (Lang::En, "rythme") => m(
            "Rythme",
            "Learn Rythme",
            "Simpler than Python, as pleasant as Ruby, safe like Rust.",
            "Learn Rythme step by step: values, functions, text, lists and records, functional style, types and match, errors, modules, files, tests, guided projects and solved exercises.",
            ("Rythme on GitHub", "https://github.com/maxlestage/Rythme"),
        ),
        (Lang::Es, "rythme") => m(
            "Rythme",
            "Aprender Rythme",
            "Más simple que Python, tan agradable como Ruby, seguro como Rust.",
            "Aprende Rythme paso a paso: valores, funciones, texto, listas y registros, estilo funcional, tipos y match, errores, módulos, archivos, tests, proyectos guiados y ejercicios resueltos.",
            ("Rythme en GitHub", "https://github.com/maxlestage/Rythme"),
        ),
        _ => None,
    }
}

/// Les cartes « Lequel choisir ? » : (titre, texte) d'un langage.
pub fn choose(lang: Lang, slug: &str) -> Option<(&'static str, &'static str)> {
    Some(match (lang, slug) {
        (Lang::En, "python") => (
            "To get started",
            "The most used for learning: a clear syntax, and everywhere else too: data, scripts, artificial intelligence, websites.",
        ),
        (Lang::Es, "python") => (
            "Para empezar",
            "El más usado para aprender: una sintaxis clara, y también en todas partes: datos, scripts, inteligencia artificial, sitios web.",
        ),
        (Lang::En, "javascript") => (
            "For the web",
            "The language of browsers: bring a page to life, and with Node.js, write the server too. A must for making websites.",
        ),
        (Lang::Es, "javascript") => (
            "Para la web",
            "El lenguaje de los navegadores: dar vida a una página y, con Node.js, escribir también el servidor. Imprescindible para hacer sitios.",
        ),
        (Lang::En, "typescript") => (
            "For the web, safely",
            "JavaScript with types: the editor and the compiler find mistakes before you do. The choice of large web projects.",
        ),
        (Lang::Es, "typescript") => (
            "Para la web, con seguridad",
            "JavaScript con tipos: el editor y el compilador encuentran los errores antes que tú. La elección de los grandes proyectos web.",
        ),
        (Lang::En, "go") => (
            "For servers",
            "Simple, fast and made for servers and tools: few rules, lightning-fast compilation, built-in concurrency.",
        ),
        (Lang::Es, "go") => (
            "Para servidores",
            "Simple, rápido y hecho para servidores y herramientas: pocas reglas, compilación instantánea, concurrencia integrada.",
        ),
        (Lang::En, "ruby") => (
            "To go fast",
            "Scripts, websites with Rails, tools: Ruby reads almost like English and lets you focus on the idea.",
        ),
        (Lang::Es, "ruby") => (
            "Para ir rápido",
            "Scripts, sitios web con Rails, herramientas: Ruby se lee casi como inglés y te deja concentrarte en la idea.",
        ),
        (Lang::En, "rust") => (
            "To go far",
            "Very fast and reliable programs: servers, command-line tools, WebAssembly. The compiler catches bugs before you do.",
        ),
        (Lang::Es, "rust") => (
            "Para llegar lejos",
            "Programas muy rápidos y fiables: servidores, herramientas de línea de comandos, WebAssembly. El compilador atrapa los bugs antes que tú.",
        ),
        (Lang::En, "rythme") => (
            "To understand",
            "The simplest: no loops, no null, values that never change, and error messages that explain what to do.",
        ),
        (Lang::Es, "rythme") => (
            "Para entender",
            "El más simple: sin bucles, sin null, valores que nunca cambian y mensajes de error que explican qué hacer.",
        ),
        _ => return None,
    })
}

/// Le premier programme de chaque langage, sur l'accueil : (nom du fichier, code).
pub fn hello(lang: Lang, slug: &str) -> (&'static str, &'static str) {
    match (lang, slug) {
        (Lang::Fr, "python") => (
            "bonjour",
            "for nom in [\"Ada\", \"Grace\"]:\n    print(f\"Bonjour, {nom} !\")",
        ),
        (Lang::En, "python") => (
            "hello",
            "for name in [\"Ada\", \"Grace\"]:\n    print(f\"Hello, {name}!\")",
        ),
        (Lang::Es, "python") => (
            "hola",
            "for nombre in [\"Ada\", \"Grace\"]:\n    print(f\"¡Hola, {nombre}!\")",
        ),
        (Lang::Fr, "javascript") => (
            "bonjour",
            "for (const nom of [\"Ada\", \"Grace\"]) {\n  console.log(`Bonjour, ${nom} !`);\n}",
        ),
        (Lang::En, "javascript") => (
            "hello",
            "for (const name of [\"Ada\", \"Grace\"]) {\n  console.log(`Hello, ${name}!`);\n}",
        ),
        (Lang::Es, "javascript") => (
            "hola",
            "for (const nombre of [\"Ada\", \"Grace\"]) {\n  console.log(`¡Hola, ${nombre}!`);\n}",
        ),
        (Lang::Fr, _) => (
            "bonjour",
            "fn main() {\n    for nom in [\"Ada\", \"Grace\"] {\n        println!(\"Bonjour, {nom} !\");\n    }\n}",
        ),
        (Lang::En, _) => (
            "hello",
            "fn main() {\n    for name in [\"Ada\", \"Grace\"] {\n        println!(\"Hello, {name}!\");\n    }\n}",
        ),
        (Lang::Es, _) => (
            "hola",
            "fn main() {\n    for nombre in [\"Ada\", \"Grace\"] {\n        println!(\"¡Hola, {nombre}!\");\n    }\n}",
        ),
    }
}

/// Chaque texte de l'interface : (français, anglais, espagnol).
const TEXTS: &[[&str; 3]] = &[
    // En-tête, pied de page, recherche
    ["Chapitres", "Chapters", "Capítulos"],
    ["Docs, accueil", "Docs, home", "Docs, inicio"],
    ["Parcours", "Tracks", "Cursos"],
    ["Langages", "Languages", "Lenguajes"],
    ["Comparer", "Compare", "Comparar"],
    ["Langue", "Language", "Idioma"],
    [
        "Rechercher dans la documentation",
        "Search the documentation",
        "Buscar en la documentación",
    ],
    ["Rechercher", "Search", "Buscar"],
    [
        "Passer en thème clair ou sombre",
        "Switch between light and dark theme",
        "Cambiar entre tema claro y oscuro",
    ],
    ["Thème clair ou sombre", "Light or dark theme", "Tema claro u oscuro"],
    [
        "Le code du site sur GitHub",
        "The site's code on GitHub",
        "El código del sitio en GitHub",
    ],
    [
        "Des documentations simples, en français, pour Python, JavaScript, TypeScript, Go, Ruby, Rust et Rythme. Chaque exemple est exécuté avant d'être publié.",
        "Simple documentation, in English, for Python, JavaScript, TypeScript, Go, Ruby, Rust and Rythme. Every example is run before it is published.",
        "Documentación sencilla, en español, para Python, JavaScript, TypeScript, Go, Ruby, Rust y Rythme. Cada ejemplo se ejecuta antes de publicarse.",
    ],
    ["Plan du site", "Site map", "Mapa del sitio"],
    ["Le code du site", "The site's code", "El código del sitio"],
    ["Fait avec Paquets 📦", "Made with Paquets 📦", "Hecho con Paquets 📦"],
    [
        "Chercher un sujet : boucle, closure, match…",
        "Search a topic: loop, closure, match…",
        "Busca un tema: bucle, closure, match…",
    ],
    ["Chercher", "Search", "Buscar"],
    ["Fermer", "Close", "Cerrar"],
    ["Échap", "Esc", "Esc"],
    ["Tout", "All", "Todo"],
    ["Résultats", "Results", "Resultados"],
    [" pour choisir, ", " to choose, ", " para elegir, "],
    ["Entrée", "Enter", "Intro"],
    [" pour ouvrir", " to open", " para abrir"],
    [
        "Docs : apprendre à programmer",
        "Docs: learn to program",
        "Docs: aprender a programar",
    ],
    ["Aller au contenu", "Skip to content", "Ir al contenido"],
    // Accueil
    [
        "Apprendre à programmer, ",
        "Learn to program, ",
        "Aprende a programar, ",
    ],
    ["simplement.", "simply.", "fácilmente."],
    [
        "Des chapitres courts, un exemple pour chaque idée, et du code qui marche vraiment : chaque exemple est exécuté avant d'être publié.",
        "Short chapters, an example for every idea, and code that really works: every example is run before it is published.",
        "Capítulos cortos, un ejemplo para cada idea y código que funciona de verdad: cada ejemplo se ejecuta antes de publicarse.",
    ],
    ["Choisir un parcours", "Choose a track", "Elige un curso"],
    [
        "Comparer les langages",
        "Compare the languages",
        "Comparar los lenguajes",
    ],
    [
        "Chercher : « closure », « match », « hash »…",
        "Search: “closure”, “match”, “hash”…",
        "Busca: «closure», «match», «hash»…",
    ],
    [" chapitres", " chapters", " capítulos"],
    [" exemples vérifiés", " checked examples", " ejemplos verificados"],
    [" langages", " languages", " lenguajes"],
    [
        "Le même programme dans plusieurs langages",
        "The same program in several languages",
        "El mismo programa en varios lenguajes",
    ],
    [
        "Sept langages, un parcours chacun",
        "Seven languages, one track each",
        "Siete lenguajes, un curso para cada uno",
    ],
    [
        "Commencez par l'introduction, ou allez directement au chapitre qui vous intéresse.",
        "Start with the introduction, or go straight to the chapter you care about.",
        "Empieza por la introducción, o ve directo al capítulo que te interese.",
    ],
    ["Comment apprendre ici", "How to learn here", "Cómo aprender aquí"],
    [
        "Trois étapes, à votre rythme. Le site retient les chapitres que vous avez lus.",
        "Three steps, at your own pace. The site remembers the chapters you have read.",
        "Tres pasos, a tu ritmo. El sitio recuerda los capítulos que has leído.",
    ],
    ["Lequel choisir ?", "Which one to choose?", "¿Cuál elegir?"],
    [
        "Tous sont de bons premiers langages. Ils ne visent pas la même chose.",
        "They are all good first languages. They don't aim at the same thing.",
        "Todos son buenos primeros lenguajes. No buscan lo mismo.",
    ],
    ["Pensé pour apprendre", "Designed for learning", "Pensado para aprender"],
    ["Des exemples vérifiés", "Checked examples", "Ejemplos verificados"],
    [
        "Chaque morceau de code est lancé avec le vrai Python, Node, TypeScript, Go, Ruby, Rust ou Rythme, et sa sortie comparée à celle de la page.",
        "Every piece of code is run with the real Python, Node, TypeScript, Go, Ruby, Rust or Rythme, and its output compared with the one on the page.",
        "Cada fragmento de código se ejecuta con el verdadero Python, Node, TypeScript, Go, Ruby, Rust o Rythme, y su salida se compara con la de la página.",
    ],
    ["Une recherche instantanée", "Instant search", "Búsqueda instantánea"],
    [
        "Touche / ou Ctrl+K, n'importe où : les chapitres et leurs sections, dans tous les langages.",
        "Press / or Ctrl+K, anywhere: chapters and their sections, in every language.",
        "Tecla / o Ctrl+K, en cualquier sitio: los capítulos y sus secciones, en todos los lenguajes.",
    ],
    [
        "Les langages côte à côte",
        "The languages side by side",
        "Los lenguajes lado a lado",
    ],
    [
        "Chaque chapitre renvoie au même chapitre dans les autres langages, et une page les compare ligne à ligne.",
        "Every chapter links to the same chapter in the other languages, and one page compares them line by line.",
        "Cada capítulo enlaza con el mismo capítulo en los otros lenguajes, y una página los compara línea a línea.",
    ],
    [
        "Partout, même hors ligne",
        "Everywhere, even offline",
        "En todas partes, incluso sin conexión",
    ],
    [
        "Sur téléphone comme sur ordinateur, en clair ou en sombre ; installable, et lisible sans réseau.",
        "On phone or computer, light or dark; installable, and readable without a network.",
        "En el móvil o en el ordenador, claro u oscuro; instalable y legible sin red.",
    ],
    [
        "La même idée, plusieurs façons de l'écrire",
        "The same idea, several ways to write it",
        "La misma idea, varias formas de escribirla",
    ],
    [
        "Un aperçu de la page Comparer.",
        "A preview of the Compare page.",
        "Un adelanto de la página Comparar.",
    ],
    [
        "Voir toute la comparaison →",
        "See the full comparison →",
        "Ver toda la comparación →",
    ],
    [
        "Docs : apprendre à programmer, simplement",
        "Docs: learn to program, simply",
        "Docs: aprende a programar, fácilmente",
    ],
    [
        "Des documentations simples et en français pour apprendre Python, JavaScript, TypeScript, Go, Ruby, Rust et Rythme : des chapitres courts, des exemples vérifiés, une recherche instantanée.",
        "Simple documentation in English to learn Python, JavaScript, TypeScript, Go, Ruby, Rust and Rythme: short chapters, checked examples, instant search.",
        "Documentación sencilla en español para aprender Python, JavaScript, TypeScript, Go, Ruby, Rust y Rythme: capítulos cortos, ejemplos verificados, búsqueda instantánea.",
    ],
    ["1. Les bases", "1. The basics", "1. Lo básico"],
    [
        "Jamais programmé ? Le parcours Débuter explique ce qu'est un programme, comment penser un algorithme, le terminal, l'éditeur, le débogage et Git.",
        "Never programmed? The Start here track explains what a program is, how to think in algorithms, the terminal, the editor, debugging and Git.",
        "¿Nunca has programado? El curso Empezar explica qué es un programa, cómo pensar un algoritmo, la terminal, el editor, la depuración y Git.",
    ],
    ["Débuter →", "Start here →", "Empezar →"],
    ["2. Un langage", "2. A language", "2. Un lenguaje"],
    [
        "Choisissez Python, JavaScript, TypeScript, Go, Ruby, Rust ou Rythme, et lisez les chapitres dans l'ordre : objectifs, exemples à essayer, erreurs fréquentes, questions pour vous tester.",
        "Choose Python, JavaScript, TypeScript, Go, Ruby, Rust or Rythme, and read the chapters in order: goals, examples to try, common mistakes, questions to test yourself.",
        "Elige Python, JavaScript, TypeScript, Go, Ruby, Rust o Rythme, y lee los capítulos en orden: objetivos, ejemplos para probar, errores frecuentes, preguntas para ponerte a prueba.",
    ],
    ["Choisir un langage →", "Choose a language →", "Elegir un lenguaje →"],
    ["3. La pratique", "3. Practice", "3. La práctica"],
    [
        "Chaque parcours finit par des tests, des projets guidés pas à pas, des exercices corrigés et un aide-mémoire.",
        "Every track ends with tests, step-by-step guided projects, solved exercises and a cheat sheet.",
        "Cada curso termina con tests, proyectos guiados paso a paso, ejercicios resueltos y una chuleta.",
    ],
    [
        "Comparer les langages →",
        "Compare the languages →",
        "Comparar los lenguajes →",
    ],
    [
        "Jamais programmé ? Commencez ici.",
        "Never programmed? Start here.",
        "¿Nunca has programado? Empieza aquí.",
    ],
    ["Commencer →", "Start →", "Empezar →"],
    // Comparer
    ["Idée", "Idea", "Idea"],
    ["Le chapitre →", "The chapter →", "El capítulo →"],
    ["Langages affichés :", "Languages shown:", "Lenguajes mostrados:"],
    ["Langages affichés", "Languages shown", "Lenguajes mostrados"],
    [
        "Les mêmes idées, écrites dans les sept langages du site. Choisissez ceux que vous voulez voir côte à côte ; chaque case renvoie au chapitre qui l'explique.",
        "The same ideas, written in the site's seven languages. Choose the ones you want to see side by side; each cell links to the chapter that explains it.",
        "Las mismas ideas, escritas en los siete lenguajes del sitio. Elige los que quieras ver lado a lado; cada casilla enlaza con el capítulo que la explica.",
    ],
    [
        "Afficher, nommer, choisir, répéter, fonctions, listes, types, erreurs, tests : les mêmes idées en Python, JavaScript, TypeScript, Go, Ruby, Rust et Rythme, côte à côte.",
        "Print, name, choose, repeat, functions, lists, types, errors, tests: the same ideas in Python, JavaScript, TypeScript, Go, Ruby, Rust and Rythme, side by side.",
        "Mostrar, nombrar, elegir, repetir, funciones, listas, tipos, errores, tests: las mismas ideas en Python, JavaScript, TypeScript, Go, Ruby, Rust y Rythme, lado a lado.",
    ],
    [
        "Les mêmes idées dans les sept langages, côte à côte.",
        "The same ideas in the seven languages, side by side.",
        "Las mismas ideas en los siete lenguajes, lado a lado.",
    ],
    // Un parcours
    ["Fil d'Ariane", "Breadcrumb", "Ruta de navegación"],
    ["Accueil", "Home", "Inicio"],
    [" de lecture", " of reading", " de lectura"],
    [" exemples", " examples", " ejemplos"],
    ["Commencer le parcours →", "Start the track →", "Empezar el curso →"],
    ["Les autres parcours", "The other tracks", "Los otros cursos"],
    // Les parties des parcours
    ["Comprendre", "Understand", "Entender"],
    ["Les notions de base", "The core ideas", "Las nociones básicas"],
    ["Les outils", "The tools", "Las herramientas"],
    ["Et après", "What's next", "Y después"],
    ["Les bases", "The basics", "Lo básico"],
    ["Aller plus loin", "Going further", "Ir más allá"],
    ["Le monde réel", "The real world", "El mundo real"],
    ["Bien programmer", "Programming well", "Programar bien"],
    ["Les types en profondeur", "Types in depth", "Los tipos a fondo"],
    ["Le cœur de Rust", "The heart of Rust", "El corazón de Rust"],
    // Un chapitre
    ["Autres langages", "Other languages", "Otros lenguajes"],
    ["Les langages", "The languages", "Los lenguajes"],
    ["Sur cette page", "On this page", "En esta página"],
    ["↑ Haut de page", "↑ Back to top", "↑ Volver arriba"],
    [
        "Chapitre précédent et suivant",
        "Previous and next chapter",
        "Capítulo anterior y siguiente",
    ],
    ["← Précédent", "← Previous", "← Anterior"],
    ["Suivant →", "Next →", "Siguiente →"],
    ["Choisir un langage", "Choose a language", "Elegir un lenguaje"],
    ["Et ensuite ? →", "What next? →", "¿Y después? →"],
    [
        "Le même chapitre dans les autres langages",
        "The same chapter in the other languages",
        "El mismo capítulo en los otros lenguajes",
    ],
    [
        "Marquer ce chapitre comme lu",
        "Mark this chapter as read",
        "Marcar este capítulo como leído",
    ],
    [
        "Une erreur, une idée ? Modifier cette page sur GitHub",
        "A mistake, an idea? Edit this page on GitHub",
        "¿Un error, una idea? Edita esta página en GitHub",
    ],
    ["Pas encore traduit", "Not translated yet", "Aún sin traducir"],
    // Introuvable
    [
        "Cette page n'existe pas",
        "This page doesn't exist",
        "Esta página no existe",
    ],
    [
        "Elle a peut-être changé d'adresse. Choisissez un parcours, ou cherchez ce qu'il vous faut.",
        "It may have moved. Choose a track, or search for what you need.",
        "Quizás cambió de dirección. Elige un curso, o busca lo que necesitas.",
    ],
    ["Page introuvable", "Page not found", "Página no encontrada"],
    [
        "Cette page n'existe pas.",
        "This page doesn't exist.",
        "Esta página no existe.",
    ],
    // Les pages Markdown
    ["Sortie", "Output", "Salida"],
    ["Copier", "Copy", "Copiar"],
    ["Copier le code", "Copy the code", "Copiar el código"],
    [
        "Lien vers cette section",
        "Link to this section",
        "Enlace a esta sección",
    ],
    ["Voir la solution", "Show the solution", "Ver la solución"],
    ["Voir la réponse", "Show the answer", "Ver la respuesta"],
    // La comparaison
    ["Afficher du texte", "Print text", "Mostrar texto"],
    ["Un commentaire", "A comment", "Un comentario"],
    ["Nommer une valeur", "Name a value", "Nombrar un valor"],
    ["Changer une valeur", "Change a value", "Cambiar un valor"],
    [
        "Insérer une valeur dans un texte",
        "Put a value in a text",
        "Insertar un valor en un texto",
    ],
    ["Une condition", "A condition", "Una condición"],
    ["Choisir selon une valeur", "Choose by value", "Elegir según un valor"],
    ["Répéter", "Repeat", "Repetir"],
    ["Une fonction", "A function", "Una función"],
    ["Une fonction anonyme", "An anonymous function", "Una función anónima"],
    ["Une liste", "A list", "Una lista"],
    [
        "Transformer chaque élément",
        "Transform each element",
        "Transformar cada elemento",
    ],
    ["Filtrer puis additionner", "Filter then sum", "Filtrar y luego sumar"],
    ["Un dictionnaire", "A dictionary", "Un diccionario"],
    ["Regrouper des données", "Group data", "Agrupar datos"],
    ["Votre propre type", "Your own type", "Tu propio tipo"],
    [
        "Une valeur qui peut manquer",
        "A value that may be missing",
        "Un valor que puede faltar",
    ],
    ["Signaler une erreur", "Report an error", "Señalar un error"],
    ["Transmettre une erreur", "Pass an error on", "Propagar un error"],
    ["Arrondir un nombre", "Round a number", "Redondear un número"],
    ["Filtrer par motif", "Match a pattern", "Filtrar por patrón"],
    [
        "La longueur d'un texte",
        "The length of a text",
        "La longitud de un texto",
    ],
    ["Mettre en majuscules", "Uppercase", "Pasar a mayúsculas"],
    ["Écrire un fichier", "Write a file", "Escribir un archivo"],
    ["Lire un fichier", "Read a file", "Leer un archivo"],
    ["Lire du JSON", "Read JSON", "Leer JSON"],
    [
        "Les arguments du programme",
        "Program arguments",
        "Los argumentos del programa",
    ],
    ["Un test", "A test", "Un test"],
    ["Lancer un programme", "Run a program", "Ejecutar un programa"],
    ["Trier une liste", "Sort a list", "Ordenar una lista"],
    [
        "Parcourir avec la position",
        "Loop with the position",
        "Recorrer con la posición",
    ],
    ["Compter les mots", "Count the words", "Contar las palabras"],
    [
        "Une fonction propre à un type",
        "A method on your type",
        "Un método de tu tipo",
    ],
    [
        "Assembler une liste en texte",
        "Join a list into text",
        "Unir una lista en un texto",
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Chaque `tr(l, "…")` du code a sa traduction, et aucun texte n'est en double.
    #[test]
    fn every_text_is_translated() {
        let mut seen = std::collections::HashSet::new();
        for [fr, en, es] in TEXTS {
            assert!(seen.insert(*fr), "en double : {fr}");
            assert!(!en.is_empty() && !es.is_empty(), "{fr}");
        }
        let sources = [
            include_str!("pages.rs"),
            include_str!("markdown.rs"),
            include_str!("library.rs"),
        ];
        for source in sources {
            for (_, rest) in source.match_indices("tr(l, \"").map(|(i, _)| ((), &source[i + 7..])) {
                let text = &rest[..rest.find('"').expect("fin du texte")];
                assert!(TABLE.contains_key(text), "pas traduit : {text:?}");
            }
        }
        for row in crate::compare::ROWS {
            assert!(TABLE.contains_key(row.idea), "pas traduit : {:?}", row.idea);
        }
        for track in &crate::catalog::TRACKS {
            for (_, part) in track.parts {
                assert!(TABLE.contains_key(part), "pas traduit : {part:?}");
            }
            for lang in [Lang::En, Lang::Es] {
                assert!(meta(lang, track.slug).is_some(), "{}", track.slug);
            }
        }
        assert_eq!(tr(Lang::En, "Accueil"), "Home");
        assert_eq!(tr(Lang::Fr, "Accueil"), "Accueil");
    }
}
