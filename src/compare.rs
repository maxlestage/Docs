//! La page « Comparer » : les mêmes idées dans les trois langages, côte à côte. Chaque
//! ligne renvoie au chapitre qui l'explique dans chaque parcours.

/// Une idée, écrite en Ruby, en Rust et en Rythme.
pub struct Row {
    pub idea: &'static str,
    /// Le chapitre qui en parle (le même nom dans les trois parcours, sauf exception).
    pub chapter: [&'static str; 3],
    pub code: [&'static str; 3],
}

/// Ruby, Rust, Rythme : l'ordre des colonnes.
pub const LANGS: [&str; 3] = ["ruby", "rust", "rythme"];

pub const ROWS: &[Row] = &[
    Row {
        idea: "Afficher du texte",
        chapter: ["premiers-pas"; 3],
        code: ["puts \"Bonjour !\"", "println!(\"Bonjour !\");", "print(\"Bonjour !\")"],
    },
    Row {
        idea: "Un commentaire",
        chapter: ["premiers-pas"; 3],
        code: ["# une remarque", "// une remarque", "# une remarque"],
    },
    Row {
        idea: "Nommer une valeur",
        chapter: ["premiers-pas"; 3],
        code: ["age = 36", "let age = 36;", "age = 36"],
    },
    Row {
        idea: "Changer une valeur",
        chapter: ["premiers-pas"; 3],
        code: [
            "age += 1",
            "let mut age = 36;\nage += 1;",
            "# rien ne change jamais :\nplus_vieux = age + 1",
        ],
    },
    Row {
        idea: "Insérer une valeur dans un texte",
        chapter: ["premiers-pas"; 3],
        code: [
            "\"J'ai #{age} ans\"",
            "format!(\"J'ai {age} ans\")",
            "\"J'ai {age} ans\"",
        ],
    },
    Row {
        idea: "Une condition",
        chapter: ["conditions-boucles"; 3],
        code: [
            "age >= 18 ? \"majeur\" : \"mineur\"",
            "if age >= 18 { \"majeur\" } else { \"mineur\" }",
            "if age >= 18 then \"majeur\" else \"mineur\"",
        ],
    },
    Row {
        idea: "Choisir selon une valeur",
        chapter: ["conditions-boucles", "conditions-boucles", "types"],
        code: [
            "case n\nwhen 0 then \"zéro\"\nelse \"autre\"\nend",
            "match n {\n    0 => \"zéro\",\n    _ => \"autre\",\n}",
            "match n\n  0 -> \"zéro\"\n  _ -> \"autre\"\nend",
        ],
    },
    Row {
        idea: "Répéter",
        chapter: ["conditions-boucles"; 3],
        code: [
            "3.times { |i| puts i }",
            "for i in 0..3 {\n    println!(\"{i}\");\n}",
            "(0..2).each { |i| print(i) }",
        ],
    },
    Row {
        idea: "Une fonction",
        chapter: ["fonctions"; 3],
        code: [
            "def double(x)\n  x * 2\nend",
            "fn double(x: i32) -> i32 {\n    x * 2\n}",
            "fn double(x) = x * 2",
        ],
    },
    Row {
        idea: "Une fonction anonyme",
        chapter: ["blocs", "fonctions", "fonctions"],
        code: [
            "triple = ->(x) { x * 3 }",
            "let triple = |x| x * 3;",
            "triple = fn(x) = x * 3",
        ],
    },
    Row {
        idea: "Une liste",
        chapter: ["collections"; 3],
        code: [
            "nombres = [1, 2, 3]",
            "let nombres = vec![1, 2, 3];",
            "nombres = [1, 2, 3]",
        ],
    },
    Row {
        idea: "Transformer chaque élément",
        chapter: ["blocs", "collections", "fonctionnel"],
        code: [
            "nombres.map { |x| x * 2 }",
            "nombres.iter().map(|x| x * 2).collect::<Vec<_>>()",
            "nombres.map { |x| x * 2 }",
        ],
    },
    Row {
        idea: "Filtrer puis additionner",
        chapter: ["blocs", "collections", "fonctionnel"],
        code: [
            "(1..10).select(&:even?).sum",
            "(1..=10).filter(|x| x % 2 == 0).sum::<i32>()",
            "1..10 |> filter(is_even) |> sum",
        ],
    },
    Row {
        idea: "Un dictionnaire",
        chapter: ["collections"; 3],
        code: [
            "ages = { \"Ada\" => 36 }",
            "let ages = HashMap::from([(\"Ada\", 36)]);",
            "ages = dict([[\"Ada\", 36]])",
        ],
    },
    Row {
        idea: "Regrouper des données",
        chapter: ["types", "types", "collections"],
        code: [
            "Point = Struct.new(:x, :y)\np = Point.new(1, 2)",
            "struct Point { x: i32, y: i32 }\nlet p = Point { x: 1, y: 2 };",
            "p = {x: 1, y: 2}",
        ],
    },
    Row {
        idea: "Votre propre type",
        chapter: ["types"; 3],
        code: [
            "class Cercle\n  def initialize(r) = @r = r\n  def aire = 3.14 * @r ** 2\nend",
            "enum Forme {\n    Cercle(f64),\n    Carre(f64),\n}",
            "type Forme = Cercle(r) | Carre(c)",
        ],
    },
    Row {
        idea: "Une valeur qui peut manquer",
        chapter: ["erreurs"; 3],
        code: ["nil", "Option<T> : Some(x) ou None", "Some(x) ou None"],
    },
    Row {
        idea: "Signaler une erreur",
        chapter: ["erreurs"; 3],
        code: ["raise \"oups\"", "return Err(\"oups\".to_string());", "Err(\"oups\")"],
    },
    Row {
        idea: "Transmettre une erreur",
        chapter: ["erreurs"; 3],
        code: [
            "begin\n  risque\nrescue => e\n  puts e.message\nend",
            "let n: i32 = texte.parse()?;",
            "n = parse_int(texte)?",
        ],
    },
    Row {
        idea: "Un test",
        chapter: ["projet"; 3],
        code: [
            "assert_equal 4, 2 + 2",
            "#[test]\nfn addition() {\n    assert_eq!(2 + 2, 4);\n}",
            "test \"addition\" = assert_eq(2 + 2, 4)",
        ],
    },
    Row {
        idea: "Lancer un programme",
        chapter: ["introduction"; 3],
        code: ["ruby main.rb", "cargo run", "rythme main.ry"],
    },
];

/// Les cartes « Quel langage choisir ? » : (parcours, pour qui, pourquoi).
pub const CHOOSE: [(&str, &str, &str); 3] = [
    (
        "ruby",
        "Pour aller vite",
        "Des scripts, des sites web avec Rails, des outils : Ruby se lit presque comme de l'anglais et laisse vous concentrer sur l'idée.",
    ),
    (
        "rust",
        "Pour aller loin",
        "Des programmes très rapides et fiables : serveurs, outils en ligne de commande, WebAssembly. Le compilateur attrape les bugs avant vous.",
    ),
    (
        "rythme",
        "Pour apprendre",
        "Le plus simple des trois : pas de boucles, pas de null, des valeurs qui ne changent jamais, et des messages d'erreur qui expliquent quoi faire.",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog;

    #[test]
    fn every_row_links_to_a_chapter() {
        for row in ROWS {
            for (lang, chapter) in LANGS.iter().zip(row.chapter) {
                assert!(
                    catalog::find(lang, chapter).is_some(),
                    "{lang}/{chapter} ({})",
                    row.idea
                );
            }
        }
    }
}
