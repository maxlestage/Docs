//! Les couleurs du code, calculées sur le serveur : aucune bibliothèque à charger dans
//! le navigateur. Un petit découpeur par langage (Ruby, Rust, Rythme, shell, TOML) :
//! mots-clés, textes et leurs insertions, commentaires, nombres, types, appels,
//! symboles, macros et attributs. Chaque morceau devient un `<span class="…">`.

/// Ce qu'il faut savoir d'un langage pour le colorer.
struct Syntax {
    keywords: &'static [&'static str],
    /// `true`, `nil`, `None`… : des valeurs écrites comme des mots.
    constants: &'static [&'static str],
    /// Le début d'un commentaire de fin de ligne.
    line_comment: &'static str,
    /// `/* … */`.
    block_comments: bool,
    /// `'texte'` est un texte (Ruby, shell) ; en Rust, un caractère ou une durée de vie.
    single_quotes: bool,
    /// `:symbole` (Ruby, Rythme).
    symbols: bool,
    /// `nom!(…)` (Rust).
    macros: bool,
    /// `#[…]` (Rust).
    attributes: bool,
    /// `@variable` (Ruby).
    instance_vars: bool,
    /// Ce qui insère une valeur dans un texte : `#{…}` (Ruby), `{…}` (Rust, Rythme).
    interpolation: Interpolation,
    /// Le premier mot d'une ligne est une commande (shell).
    commands: bool,
    /// `` `texte` `` : un gabarit (JavaScript, TypeScript : `${…}`) ou un texte brut (Go).
    backticks: bool,
    /// `"""texte"""` sur plusieurs lignes, et `@décorateur` (Python).
    python: bool,
}

#[derive(PartialEq)]
enum Interpolation {
    None,
    Hash,
    Braces,
    /// `${…}`, dans les gabarits `` `…` `` seulement.
    Dollar,
}

const RUBY: Syntax = Syntax {
    keywords: &[
        "alias",
        "and",
        "begin",
        "break",
        "case",
        "class",
        "def",
        "defined?",
        "do",
        "else",
        "elsif",
        "end",
        "ensure",
        "for",
        "if",
        "in",
        "module",
        "next",
        "not",
        "or",
        "redo",
        "rescue",
        "retry",
        "return",
        "self",
        "super",
        "then",
        "undef",
        "unless",
        "until",
        "when",
        "while",
        "yield",
        "raise",
        "require",
        "require_relative",
        "include",
        "extend",
        "attr_accessor",
        "attr_reader",
        "attr_writer",
        "private",
        "protected",
        "public",
        "lambda",
        "proc",
        "loop",
    ],
    constants: &["true", "false", "nil", "__method__"],
    line_comment: "#",
    block_comments: false,
    single_quotes: true,
    symbols: true,
    macros: false,
    attributes: false,
    instance_vars: true,
    interpolation: Interpolation::Hash,
    commands: false,
    backticks: false,
    python: false,
};

const RUST: Syntax = Syntax {
    keywords: &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "fn", "for",
        "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
        "static", "struct", "super", "trait", "type", "unsafe", "use", "where", "while",
    ],
    constants: &["true", "false"],
    line_comment: "//",
    block_comments: true,
    single_quotes: false,
    symbols: false,
    macros: true,
    attributes: true,
    instance_vars: false,
    interpolation: Interpolation::Braces,
    commands: false,
    backticks: false,
    python: false,
};

const RYTHME: Syntax = Syntax {
    keywords: &[
        "and", "do", "else", "end", "fn", "if", "match", "not", "or", "test", "then", "type", "use", "unless", "as",
    ],
    constants: &["true", "false"],
    line_comment: "#",
    block_comments: false,
    single_quotes: false,
    symbols: true,
    macros: false,
    attributes: false,
    instance_vars: false,
    interpolation: Interpolation::Braces,
    commands: false,
    backticks: false,
    python: false,
};

const SHELL: Syntax = Syntax {
    keywords: &["if", "then", "fi", "for", "do", "done", "export", "cd", "sudo"],
    constants: &[],
    line_comment: "#",
    block_comments: false,
    single_quotes: true,
    symbols: false,
    macros: false,
    attributes: false,
    instance_vars: false,
    interpolation: Interpolation::None,
    commands: true,
    backticks: false,
    python: false,
};

const TOML: Syntax = Syntax {
    keywords: &[],
    constants: &["true", "false"],
    line_comment: "#",
    block_comments: false,
    single_quotes: true,
    symbols: false,
    macros: false,
    attributes: false,
    instance_vars: false,
    interpolation: Interpolation::None,
    commands: false,
    backticks: false,
    python: false,
};

const PYTHON: Syntax = Syntax {
    keywords: &[
        "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except",
        "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "match", "case", "nonlocal", "not",
        "or", "pass", "raise", "return", "try", "while", "with", "yield", "self",
    ],
    constants: &["True", "False", "None"],
    line_comment: "#",
    block_comments: false,
    single_quotes: true,
    symbols: false,
    macros: false,
    attributes: false,
    instance_vars: false,
    interpolation: Interpolation::Braces,
    commands: false,
    backticks: false,
    python: true,
};

const GO: Syntax = Syntax {
    keywords: &[
        "break",
        "case",
        "chan",
        "const",
        "continue",
        "default",
        "defer",
        "else",
        "fallthrough",
        "for",
        "func",
        "go",
        "goto",
        "if",
        "import",
        "interface",
        "map",
        "package",
        "range",
        "return",
        "select",
        "struct",
        "switch",
        "type",
        "var",
    ],
    constants: &["true", "false", "nil", "iota"],
    line_comment: "//",
    block_comments: true,
    single_quotes: true,
    symbols: false,
    macros: false,
    attributes: false,
    instance_vars: false,
    interpolation: Interpolation::None,
    commands: false,
    backticks: true,
    python: false,
};

const JAVASCRIPT: Syntax = Syntax {
    keywords: &[
        "async",
        "await",
        "break",
        "case",
        "catch",
        "class",
        "const",
        "continue",
        "default",
        "delete",
        "do",
        "else",
        "export",
        "extends",
        "finally",
        "for",
        "from",
        "function",
        "if",
        "import",
        "in",
        "instanceof",
        "let",
        "new",
        "of",
        "return",
        "static",
        "super",
        "switch",
        "this",
        "throw",
        "try",
        "typeof",
        "var",
        "void",
        "while",
        "yield",
        "get",
        "set",
    ],
    constants: &["true", "false", "null", "undefined", "NaN", "Infinity"],
    line_comment: "//",
    block_comments: true,
    single_quotes: true,
    symbols: false,
    macros: false,
    attributes: false,
    instance_vars: false,
    interpolation: Interpolation::Dollar,
    commands: false,
    backticks: true,
    python: false,
};

const TYPESCRIPT: Syntax = Syntax {
    keywords: &[
        "abstract",
        "as",
        "async",
        "await",
        "break",
        "case",
        "catch",
        "class",
        "const",
        "continue",
        "declare",
        "default",
        "delete",
        "do",
        "else",
        "enum",
        "export",
        "extends",
        "finally",
        "for",
        "from",
        "function",
        "if",
        "implements",
        "import",
        "in",
        "infer",
        "instanceof",
        "interface",
        "is",
        "keyof",
        "let",
        "namespace",
        "new",
        "of",
        "private",
        "protected",
        "public",
        "readonly",
        "return",
        "satisfies",
        "static",
        "super",
        "switch",
        "this",
        "throw",
        "try",
        "type",
        "typeof",
        "var",
        "void",
        "while",
        "yield",
        "get",
        "set",
    ],
    constants: &[
        "true",
        "false",
        "null",
        "undefined",
        "NaN",
        "Infinity",
        "number",
        "string",
        "boolean",
        "any",
        "unknown",
        "never",
        "object",
        "bigint",
        "symbol",
    ],
    line_comment: "//",
    block_comments: true,
    single_quotes: true,
    symbols: false,
    macros: false,
    attributes: false,
    instance_vars: false,
    interpolation: Interpolation::Dollar,
    commands: false,
    backticks: true,
    python: false,
};

/// Le nom affiché d'un langage de bloc de code (`rust` → `Rust`), et s'il est coloré.
pub fn label(lang: &str) -> &'static str {
    match lang {
        "ruby" | "rb" => "Ruby",
        "rust" | "rs" => "Rust",
        "rythme" | "ry" => "Rythme",
        "python" | "py" => "Python",
        "go" | "golang" => "Go",
        "javascript" | "js" => "JavaScript",
        "typescript" | "ts" => "TypeScript",
        "html" => "HTML",
        "css" => "CSS",
        "sh" | "bash" | "shell" | "console" => "Terminal",
        "powershell" | "ps1" => "PowerShell",
        "toml" => "TOML",
        "json" => "JSON",
        "sortie" => "Sortie",
        _ => "Texte",
    }
}

fn syntax(lang: &str) -> Option<&'static Syntax> {
    match lang {
        "ruby" | "rb" => Some(&RUBY),
        "rust" | "rs" => Some(&RUST),
        "rythme" | "ry" => Some(&RYTHME),
        "python" | "py" => Some(&PYTHON),
        "go" | "golang" => Some(&GO),
        "javascript" | "js" => Some(&JAVASCRIPT),
        "typescript" | "ts" => Some(&TYPESCRIPT),
        "sh" | "bash" | "shell" | "console" | "powershell" | "ps1" => Some(&SHELL),
        "toml" => Some(&TOML),
        _ => None,
    }
}

/// Échappe `&`, `<`, `>` et `"`.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    out
}

fn span(out: &mut String, class: &str, text: &str) {
    out.push_str("<span class=\"");
    out.push_str(class);
    out.push_str("\">");
    out.push_str(&escape(text));
    out.push_str("</span>");
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Le code `code` du langage `lang`, en HTML coloré (échappé, sans `<pre>`).
pub fn highlight(lang: &str, code: &str) -> String {
    let Some(syntax) = syntax(lang) else {
        return escape(code);
    };
    let chars: Vec<char> = code.chars().collect();
    let text = |from: usize, to: usize| chars[from..to].iter().collect::<String>();
    let starts = |at: usize, s: &str| s.chars().enumerate().all(|(k, c)| chars.get(at + k) == Some(&c));
    let mut out = String::with_capacity(code.len() * 2);
    let mut i = 0;
    let mut line_start = true;
    while i < chars.len() {
        let c = chars[i];
        if c == '\n' {
            out.push('\n');
            line_start = true;
            i += 1;
            continue;
        }
        if c.is_whitespace() {
            out.push(c);
            i += 1;
            continue;
        }
        let was_line_start = std::mem::replace(&mut line_start, false);
        // Commentaires.
        if starts(i, syntax.line_comment) && (syntax.line_comment != "#" || !starts(i, "#{")) {
            let end = chars[i..]
                .iter()
                .position(|&c| c == '\n')
                .map_or(chars.len(), |p| i + p);
            span(&mut out, "c", &text(i, end));
            i = end;
            continue;
        }
        if syntax.block_comments && starts(i, "/*") {
            let end = (i + 2..chars.len())
                .find(|&j| starts(j, "*/"))
                .map_or(chars.len(), |j| j + 2);
            span(&mut out, "c", &text(i, end));
            i = end;
            continue;
        }
        // Attributs Rust : `#[derive(Debug)]`.
        if syntax.attributes && (starts(i, "#[") || starts(i, "#![")) {
            let end = chars[i..]
                .iter()
                .position(|&c| c == ']')
                .map_or(chars.len(), |p| i + p + 1);
            span(&mut out, "a", &text(i, end));
            i = end;
            continue;
        }
        // Python : `"""texte"""` sur plusieurs lignes, et `@décorateur`.
        if syntax.python && (starts(i, "\"\"\"") || starts(i, "\'\'\'")) {
            let close: String = chars[i..i + 3].iter().collect();
            let end = (i + 3..chars.len())
                .find(|&j| starts(j, &close))
                .map_or(chars.len(), |j| j + 3);
            span(&mut out, "s", &text(i, end));
            i = end;
            continue;
        }
        if syntax.python && c == '@' && was_line_start {
            let end = (i + 1..chars.len())
                .find(|&j| !(is_word(chars[j]) || chars[j] == '.'))
                .unwrap_or(chars.len());
            span(&mut out, "a", &text(i, end));
            i = end;
            continue;
        }
        // Textes.
        if c == '"' || (c == '\'' && syntax.single_quotes) || (c == '`' && syntax.backticks) {
            i = string(&chars, i, c, syntax, &mut out);
            continue;
        }
        // En Rust, `'a'` est un caractère, `'a` une durée de vie.
        if c == '\'' && syntax.macros {
            let close = if chars.get(i + 1) == Some(&'\\') {
                (i + 2..chars.len().min(i + 12)).find(|&j| chars[j] == '\'')
            } else {
                Some(i + 2).filter(|&j| chars.get(j) == Some(&'\''))
            };
            match close {
                Some(j) => {
                    span(&mut out, "s", &text(i, j + 1));
                    i = j + 1;
                }
                None => {
                    let end = (i + 1..chars.len())
                        .find(|&j| !is_word(chars[j]))
                        .unwrap_or(chars.len());
                    span(&mut out, "t", &text(i, end));
                    i = end;
                }
            }
            continue;
        }
        // Symboles : `:ok` (mais pas `::` ni `a: 1`).
        if syntax.symbols
            && c == ':'
            && chars.get(i + 1).is_some_and(|&n| n.is_alphabetic() || n == '_')
            && (i == 0 || !(is_word(chars[i - 1]) || chars[i - 1] == ':'))
        {
            let mut end = (i + 1..chars.len())
                .find(|&j| !is_word(chars[j]))
                .unwrap_or(chars.len());
            if matches!(chars.get(end), Some('?' | '!')) {
                end += 1;
            }
            span(&mut out, "m", &text(i, end));
            i = end;
            continue;
        }
        // Variables d'instance Ruby : `@nom`.
        if syntax.instance_vars && c == '@' {
            let end = (i + 1..chars.len())
                .find(|&j| !(is_word(chars[j]) || chars[j] == '@'))
                .unwrap_or(chars.len());
            span(&mut out, "v", &text(i, end));
            i = end;
            continue;
        }
        // Nombres : `42`, `3.14`, `1_000`, `0xff`, `2u8`.
        if c.is_ascii_digit() && (i == 0 || !is_word(chars[i - 1])) {
            let mut end = i;
            while end < chars.len()
                && (is_word(chars[end]) || (chars[end] == '.' && chars.get(end + 1).is_some_and(char::is_ascii_digit)))
            {
                end += 1;
            }
            span(&mut out, "n", &text(i, end));
            i = end;
            continue;
        }
        // Mots.
        if is_word(c) {
            let mut end = (i..chars.len()).find(|&j| !is_word(chars[j])).unwrap_or(chars.len());
            // `empty?`, `save!` (Ruby, Rythme) ; `println!` (Rust).
            let bang = matches!(chars.get(end), Some('?' | '!'))
                && !matches!(chars.get(end + 1), Some('='))
                && (syntax.symbols || (syntax.macros && chars[end] == '!'));
            if bang {
                end += 1;
            }
            let word = text(i, end);
            let next = chars[end..].iter().find(|c| **c != ' ').copied();
            let class = if syntax.commands && was_line_start {
                Some("f")
            } else if syntax.keywords.contains(&word.as_str()) {
                Some("k")
            } else if syntax.constants.contains(&word.as_str()) {
                Some("o")
            } else if syntax.macros && word.ends_with('!') {
                Some("m")
            } else if c.is_uppercase() {
                Some("t")
            } else if !c.is_ascii_digit() && (next == Some('(') || (i > 0 && chars[i - 1] == '.' && !syntax.commands)) {
                Some("f")
            } else {
                None
            };
            match class {
                Some(class) => span(&mut out, class, &word),
                None => out.push_str(&escape(&word)),
            }
            i = end;
            continue;
        }
        // Le reste : opérateurs, ponctuation.
        let op = matches!(
            c,
            '=' | '+' | '-' | '*' | '/' | '%' | '<' | '>' | '!' | '&' | '|' | '?' | '^'
        ) && !syntax.commands;
        if op {
            let end = (i..chars.len())
                .find(|&j| {
                    !matches!(
                        chars[j],
                        '=' | '+' | '-' | '*' | '/' | '%' | '<' | '>' | '!' | '&' | '|' | '?' | '^'
                    )
                })
                .unwrap_or(chars.len());
            span(&mut out, "p", &text(i, end));
            i = end;
            continue;
        }
        out.push_str(&escape(&c.to_string()));
        i += 1;
    }
    out
}

/// Un texte qui commence à `start` par `quote`, avec ses insertions (`#{…}`, `{…}`) ;
/// renvoie la position qui le suit.
fn string(chars: &[char], start: usize, quote: char, syntax: &Syntax, out: &mut String) -> usize {
    let mut i = start + 1;
    let mut piece = String::from(quote);
    out.push_str("<span class=\"s\">");
    let interpolates = match syntax.interpolation {
        Interpolation::None => false,
        Interpolation::Dollar => quote == '`',
        Interpolation::Hash | Interpolation::Braces => quote == '"' || (syntax.python && quote == '\''),
    };
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() {
            piece.push(c);
            piece.push(chars[i + 1]);
            i += 2;
            continue;
        }
        if c == quote {
            piece.push(c);
            i += 1;
            break;
        }
        let opens = match syntax.interpolation {
            Interpolation::Hash => c == '#' && chars.get(i + 1) == Some(&'{'),
            // `{{` est une accolade écrite telle quelle en Rust.
            Interpolation::Braces => {
                (c == '{' && chars.get(i + 1) != Some(&'{'))
                    || (syntax.symbols && c == '#' && chars.get(i + 1) == Some(&'{'))
            }
            Interpolation::Dollar => c == '$' && chars.get(i + 1) == Some(&'{'),
            Interpolation::None => false,
        };
        if interpolates && opens {
            let open = if c == '#' || c == '$' { i + 1 } else { i };
            let mut depth = 0;
            let mut end = None;
            for (j, &d) in chars.iter().enumerate().skip(open) {
                match d {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(j);
                            break;
                        }
                    }
                    '\n' if quote != '`' && !syntax.symbols => break,
                    _ => {}
                }
            }
            if let Some(end) = end {
                out.push_str(&escape(&piece));
                piece.clear();
                span(out, "i", &chars[i..=end].iter().collect::<String>());
                i = end + 1;
                continue;
            }
        }
        if c == '{' && chars.get(i + 1) == Some(&'{') && syntax.interpolation == Interpolation::Braces {
            piece.push_str("{{");
            i += 2;
            continue;
        }
        piece.push(c);
        i += 1;
    }
    out.push_str(&escape(&piece));
    out.push_str("</span>");
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_ruby() {
        let html = highlight(
            "ruby",
            "def hello?(name) # salut\n  puts \"Bonjour #{name}\" if @ok && :sym\nend",
        );
        assert!(html.contains(r#"<span class="k">def</span>"#));
        assert!(html.contains(r#"<span class="f">hello?</span>"#));
        assert!(html.contains(r##"<span class="c"># salut</span>"##));
        assert!(html.contains(r##"<span class="i">#{name}</span>"##));
        assert!(html.contains(r#"<span class="v">@ok</span>"#));
        assert!(html.contains(r#"<span class="m">:sym</span>"#));
    }

    #[test]
    fn colors_rust() {
        let html = highlight(
            "rust",
            "#[derive(Debug)]\nfn longest<'a>(x: &'a str) -> char { println!(\"{x} {{}}\"); 'c' } // fin",
        );
        assert!(html.contains(r#"<span class="a">#[derive(Debug)]</span>"#));
        assert!(html.contains(r#"<span class="t">&#39;a</span>"#) || html.contains(r#"<span class="t">'a</span>"#));
        assert!(html.contains(r#"<span class="m">println!</span>"#));
        assert!(html.contains(r#"<span class="i">{x}</span>"#));
        assert!(html.contains(r#"<span class="s">'c'</span>"#));
        assert!(html.contains(r#"<span class="c">// fin</span>"#));
    }

    #[test]
    fn colors_rythme_and_escapes() {
        let html = highlight("rythme", "fn aire(Cercle(r)) = r < 2 # <b>\nprint(\"{x} #{y}\")");
        assert!(html.contains(r#"<span class="k">fn</span>"#));
        assert!(html.contains(r#"<span class="t">Cercle</span>"#));
        assert!(html.contains(r#"<span class="c"># &lt;b&gt;</span>"#));
        assert!(html.contains(r##"<span class="i">#{y}</span>"##));
        assert!(!html.contains("<b>"));
        assert_eq!(highlight("text", "<a>"), "&lt;a&gt;");
    }

    #[test]
    fn colors_python_go_javascript_typescript() {
        let py = highlight(
            "python",
            "@dataclass\ndef f(x):\n    \"\"\"Doc.\"\"\"\n    return f\"{x} ok\" if x else None",
        );
        assert!(py.contains(r#"<span class="a">@dataclass</span>"#), "{py}");
        assert!(py.contains(r#"<span class="k">def</span>"#));
        assert!(
            py.contains(r#"<span class="s">&quot;&quot;&quot;Doc.&quot;&quot;&quot;</span>"#),
            "{py}"
        );
        assert!(py.contains(r#"<span class="i">{x}</span>"#));
        assert!(py.contains(r#"<span class="o">None</span>"#));
        let go = highlight("go", "func main() {\n\tfmt.Println(`brut {x}`, nil) // fin\n}");
        assert!(go.contains(r#"<span class="k">func</span>"#));
        assert!(go.contains(r#"<span class="s">`brut {x}`</span>"#), "{go}");
        assert!(go.contains(r#"<span class="o">nil</span>"#));
        let js = highlight("javascript", "const x = `a ${b} c`; // fin");
        assert!(js.contains(r#"<span class="k">const</span>"#));
        assert!(js.contains(r#"<span class="i">${b}</span>"#), "{js}");
        let ts = highlight("typescript", "interface P { x: number }");
        assert!(ts.contains(r#"<span class="k">interface</span>"#));
        assert!(ts.contains(r#"<span class="o">number</span>"#));
    }

    #[test]
    fn shell_commands() {
        let html = highlight("sh", "cargo run --release  # lancer");
        assert!(html.starts_with(r#"<span class="f">cargo</span> run"#));
    }
}
