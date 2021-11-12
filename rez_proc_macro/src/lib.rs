use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Špatný" | "Špatná" | "Špatné" => "Err",
        "VPořádku" => "Ok",
        "Řetězec" => "String",
        "Slovník" => "HashMap",
        "Výchozí" => "Default",
        "Chyba" => "Error",
        "Možná" => "Option",
        "Nějaka" | "Nějaký" | "Nějaké" => "Some",
        "Nic" => "None",
        "Výsledek" => "Result",
        "Já" => "Self",
        "napiš_řádek" => "println",
        "vyskoč" => "break",
        "pokračuj" => "continue",
        "asynchronní" => "async",
        "vyčkej" => "await",
        "smyčka" => "loop",
        "přesuň" => "move",
        "bedna" | "bedně" => "crate",
        "nedosažitelný_kód" => "unreachable_code",
        "jako" => "as",
        "konstanta" => "const",
        "vlastnost" => "trait",
        "nebezpečný" | "nebezpečná" | "nebezpečné" => "unsafe",
        "v" => "in",
        "z" => "from",
        "dynamický" | "dynamická" | "dynamické" => "dyn",
        "rozbal" => "unwrap",
        "očekávej" => "expect",
        "výchozí" => "default",
        "jako_odkaz" => "as_ref",
        "vstup_výstup" => "io",
        "vnější" => "extern",
        "nepravda" => "false",
        "funkce" => "fn",
        "nadřazený" | "nadřazená" | "nadřazené" => "super",
        "vlož" => "insert",
        "dostaň" => "get",
        "povol" => "allow",
        "panika" | "ups" => "panic",
        "modul" => "mod",
        "měnitelný" | "měnitelná" | "měnitelné" => "mut",
        "nový" | "nová" | "nové" => "new",
        "kde" => "where",
        "pro" => "for",
        "dostaň_nebo_vlož_s" => "get_or_insert_with",
        "hlavní" => "main",
        "veřejný" | "veřejná" | "veřejné" => "pub",
        "vrať" => "return",
        "implementuj" => "impl",
        "odkaz" => "ref",
        "odpovídá" => "match",
        "jestli" => "if",
        "jinak" => "else",
        "já" => "self",
        "ať" | "je" => "let",
        "statický" | "statická" | "statické" => "static",
        "struktura" => "struct",
        "čekej" => "expect",
        "zatímco" => "while",
        "používej" => "use",
        "konvertuj" => "into",
        "pravda" => "true",
        "výčet" => "enum",
        "standardní_knihovna" => "std",
        "kolekce" => "collections",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rez(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
