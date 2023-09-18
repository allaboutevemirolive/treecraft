struct ColorTable {
    term_flg: String,
    CSS_name: String,
    font_fg: String,
    font_bg: String,
}

struct Extensions {
    ext: String,
    term_flg: String,
    CSS_name: String,
    web_fg: String,
    web_bg: String,
    web_extattr: String,
    nxt: Option<Box<Extensions>>,
}

struct LineDraw<'a> {
    name: &'a [&'a str],
    vert: &'a str,
    vert_left: &'a str,
    corner: &'a str,
    copy: &'a str,
    ctop: &'a str,
    cbot: &'a str,
    cmid: &'a str,
    cext: &'a str,
    csingle: &'a str,
}

struct MetaIds {
    name: String,
    term_flg: String,
}
