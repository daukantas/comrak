use ::{Arena, parse_document, format_document};

fn compare(input: &[u8], expected: &str) {
    let arena = Arena::new();
    let ast = parse_document(&arena, input, 0);
    let html = format_document(ast);
    assert_eq!(html, expected);
}

#[test]
fn basic() {
    compare(b"My **document**.\n\nIt's mine.\n\n> Yes.\n\n## Hi!\n\nOkay.\n",
            concat!("<p>My <strong>document</strong>.</p>\n",
                    "<p>It's mine.</p>\n",
                    "<blockquote>\n",
                    "<p>Yes.</p>\n",
                    "</blockquote>\n",
                    "<h2>Hi!</h2>\n",
                    "<p>Okay.</p>\n"));
}

#[test]
fn codefence() {
    compare(b"``` rust\nfn main();\n```\n",
            concat!("<pre><code class=\"language-rust\">fn main();\n",
                    "</code></pre>\n"));
}
