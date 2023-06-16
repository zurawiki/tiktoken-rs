#![feature(test)]
extern crate test;

use tiktoken_rs::cl100k_base;

static SIZE_FACTOR: usize = 64;
static CONTENT: &str = r#"
# Modus cognitius profanam ne duae virtutis mundi

## Ut vita

Lorem markdownum litora, care ponto nomina, et ut aspicit gelidas sui et
purpureo genuit. Tamen colla venientis [delphina](http://nil-sol.com/ecquis)
Tusci et temptata citaeque curam isto ubi vult vulnere reppulit.

- :one: Seque vidit flendoque de quodam
- :two: Dabit minimos deiecto caputque noctis pluma
- :three: Leti coniunx est Helicen
- :four: Illius pulvereumque Icare inpositos
- :five: Vivunt pereo pluvio tot ramos Olenios gelidis
- :six: Quater teretes natura inde

### A subsection

Protinus dicunt, breve per, et vivacis genus Orphei munere. Me terram [dimittere
casside](http://corpus.org/) pervenit saxo primoque frequentat genuum sorori
praeferre causas Libys. Illud in serpit adsuetam utrimque nunc haberent,
**terrae si** veni! Hectoreis potes sumite [Mavortis retusa](http://tua.org/)
granum captantur potuisse Minervae, frugum.

> Clivo sub inprovisoque nostrum minus fama est, discordia patrem petebat precatur
absumitur, poena per sit. Foramina *tamen cupidine* memor supplex tollentes
dictum unam orbem, Anubis caecae. Viderat formosior tegebat satis, Aethiopasque
sit submisso coniuge tristis ubi! :exclamation:

## Praeceps Corinthus totidem quem crus vultum cape

```rs
#[derive(Debug)]
pub struct Site {
    /// The base path of the gutenberg site
    pub base_path: PathBuf,
    /// The parsed config for the site
    pub config: Config,
    pub pages: HashMap<PathBuf, Page>,
    pub sections: HashMap<PathBuf, Section>,
    pub tera: Tera,
    live_reload: bool,
    output_path: PathBuf,
    static_path: PathBuf,
    pub tags: Option<Taxonomy>,
    pub categories: Option<Taxonomy>,
    /// A map of all .md files (section and pages) and their permalink
    /// We need that if there are relative links in the content that need to be resolved
    pub permalinks: HashMap<String, String>,
}
```

## More stuff
And a shortcode:

{{ youtube(id="my_youtube_id") }}

### Another subsection
Gotta make the toc do a little bit of work

# A big title :fire:

- hello
- world
- !

```py
if __name__ == "__main__":
    gen_site("basic-blog", [""], 250, paginate=True)
```
"#;

#[bench]
fn bench_cl100k_encode(b: &mut test::Bencher) {
    let content = CONTENT.repeat(SIZE_FACTOR);
    let bpe = cl100k_base().unwrap();
    b.iter(|| {
        let _tokens = bpe.encode_with_special_tokens(&content);
    });
}

#[bench]
fn bench_cl100k_decode(b: &mut test::Bencher) {
    let content = CONTENT.repeat(SIZE_FACTOR);
    let bpe = cl100k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(&content);

    b.iter(|| {
        bpe.decode(tokens.clone()).unwrap();
    });
}

#[bench]
fn bench_cl100k_encode_10x(b: &mut test::Bencher) {
    let content = CONTENT.repeat(SIZE_FACTOR * 10);
    let bpe = cl100k_base().unwrap();
    b.iter(|| {
        let _tokens = bpe.encode_with_special_tokens(&content);
    });
}

#[bench]
fn bench_cl100k_decode_10x(b: &mut test::Bencher) {
    let content = CONTENT.repeat(SIZE_FACTOR * 10);
    let bpe = cl100k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(&content);

    b.iter(|| {
        bpe.decode(tokens.clone()).unwrap();
    });
}
