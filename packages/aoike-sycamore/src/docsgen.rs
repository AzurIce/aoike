pub fn index() -> &'static aoike::PostData {
    static INDEX: std::sync::LazyLock<aoike::PostData> = std::sync::LazyLock::new(|| {
        aoike::PostData {
            title: "index".to_string(),
            slug: "index".to_string(),
            summary_html: "<p>Hello！这里是 Azur冰弦（AzurIce），一个热爱音乐、动漫、代码和游戏的二次元🥰</p>\n<p>是的，没错，我又又又换了博客框架，这次是基于 <code>build.rs</code> 和 ~<a href=\"https:>DioxusLabs/dioxus</a>~（现在改成 <a href=\"https:>sycamore-rs/sycamore</a> 了） 手搓的（Rust 赛高🥰）。\n本来想用 <a href=\"https:>getzola/zola</a> 来着，但是我发现它的模板系统比较麻烦，而且一堆的配置也对写东西侵入性比较强，所以就自己写了一个。</p>\n<p>因...</p>"
                .to_string(),
            content_html: "<p>Hello！这里是 Azur冰弦（AzurIce），一个热爱音乐、动漫、代码和游戏的二次元🥰</p>\n<p>是的，没错，我又又又换了博客框架，这次是基于 <code>build.rs</code> 和 ~<a href=\"https://github.com/DioxusLabs/dioxus\">DioxusLabs/dioxus</a>~（现在改成 <a href=\"https://github.com/sycamore-rs/sycamore\">sycamore-rs/sycamore</a> 了） 手搓的（Rust 赛高🥰）。\n本来想用 <a href=\"https://github.com/getzola/zola\">getzola/zola</a> 来着，但是我发现它的模板系统比较麻烦，而且一堆的配置也对写东西侵入性比较强，所以就自己写了一个。</p>\n<p>因为是拿来放一堆文章笔记的，所以叫它「池」，又因为喜欢蓝色，所以叫它「青池」，这里是仓库 <a href=\"https://github.com/AzurIce/aoike\">AzurIce/aoike</a>，不过目前还非常的 experimental。</p>\n<p>细数一路来用的框架 Wordpress -&gt; Hexo -&gt; Hugo -&gt; Typecho -&gt; Mkdocs -&gt; 手搓 Mkdocs -&gt; zola -&gt; 手搓 zola -&gt; \\手搓 aoike/ 折腾过来，看来生命在于折腾。</p>\n<hr />\n<p>如果发现问题欢迎来给我提 Issue 和 PR！</p>\n<p>当然，本站也接入了 Giscus 评论系统，有什么想法都可以在页面最下面灌水🥳</p>\n<p>如果想认识我/扩列的话欢迎联系我w~（超级社恐阴暗逼）。</p>\n<p>TODO: 把上面的提到的几个工具/技术加上链接\n~~现在不加是因为我是懒逼~~</p>\n"
                .to_string(),
            created: aoike::time::UtcDateTime::from_unix_timestamp(1759656018i64)
                .unwrap(),
            updated: aoike::time::UtcDateTime::from_unix_timestamp(1759656018i64)
                .unwrap(),
        }
    });
    &INDEX
}
pub fn posts() -> &'static [aoike::PostData] {
    static POSTS: std::sync::LazyLock<Vec<aoike::PostData>> = std::sync::LazyLock::new(||
    {
        let mut posts = vec![
            aoike::PostData { title : "test copy".to_string(), slug : "test-copy"
            .to_string(), summary_html :
            "\n<p>LOREM IPSUM DOLOR SIT AMET, CONSECTETUR ADIPISCING ELIT. SED DO EIUSMOD TEMPOR INCIDIDUNT UT LABORE ET DOLORE MAGNA ALIQUA. UT ENIM AD MINIM VENIAM, QUIS NOSTRUD EXERCITATION ULLAMCO LABORIS NISI UT ALIQUIP EX EA COMMODO CONSEQUAT. DUIS...</p>"
            .to_string(), content_html :
            "<h1>test copy</h1>\n<p>LOREM IPSUM DOLOR SIT AMET, CONSECTETUR ADIPISCING ELIT. SED DO EIUSMOD TEMPOR INCIDIDUNT UT LABORE ET DOLORE MAGNA ALIQUA. UT ENIM AD MINIM VENIAM, QUIS NOSTRUD EXERCITATION ULLAMCO LABORIS NISI UT ALIQUIP EX EA COMMODO CONSEQUAT. DUIS AUTE IRURE DOLOR IN REPREHENDERIT IN VOLUPTATE VELIT ESSE CILLUM DOLORE EU FUGIAT NULLA PARIATUR. EXCEPTEUR SINT OCCAECAT CUPIDATAT NON PROIDENT, SUNT IN CULPA QUI OFFICIA DESERUNT MOLLIT ANIM ID EST LABORUM.</p>\n<p>NULLA FACILISI. MAECENAS FAUCIBUS MOLLIS INTERDUM. VESTIBULUM ID LIGULA PORTA FELIS EUISMOD SEMPER. DONEC SED ODIO DUI. CRAS JUSTO ODIO, DAPIBUS AC FACILISIS IN, EGESTAS EGET QUAM. VESTIBULUM ID LIGULA PORTA FELIS EUISMOD SEMPER. PRAESENT COMMODO CURSUS MAGNA, VEL SCELERISQUE NISL CONSECTETUR ET.</p>\n"
            .to_string(), created :
            aoike::time::UtcDateTime::from_unix_timestamp(1759656018i64).unwrap(),
            updated : aoike::time::UtcDateTime::from_unix_timestamp(1759656018i64)
            .unwrap(), }, aoike::PostData { title : "test".to_string(), slug : "test"
            .to_string(), summary_html :
            "\n<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis...</p>"
            .to_string(), content_html :
            "<h1>test</h1>\n<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.</p>\n<p>Nulla facilisi. Maecenas faucibus mollis interdum. Vestibulum id ligula porta felis euismod semper. Donec sed odio dui. Cras justo odio, dapibus ac facilisis in, egestas eget quam. Vestibulum id ligula porta felis euismod semper. Praesent commodo cursus magna, vel scelerisque nisl consectetur et.</p>\n"
            .to_string(), created :
            aoike::time::UtcDateTime::from_unix_timestamp(1759656018i64).unwrap(),
            updated : aoike::time::UtcDateTime::from_unix_timestamp(1759656018i64)
            .unwrap(), }, aoike::PostData { title : "test".to_string(), slug : "ce-shi"
            .to_string(), summary_html :
            "\n<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis...</p>"
            .to_string(), content_html :
            "<h1>test</h1>\n<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.</p>\n<p>Nulla facilisi. Maecenas faucibus mollis interdum. Vestibulum id ligula porta felis euismod semper. Donec sed odio dui. Cras justo odio, dapibus ac facilisis in, egestas eget quam. Vestibulum id ligula porta felis euismod semper. Praesent commodo cursus magna, vel scelerisque nisl consectetur et.</p>\n"
            .to_string(), created :
            aoike::time::UtcDateTime::from_unix_timestamp(1759656018i64).unwrap(),
            updated : aoike::time::UtcDateTime::from_unix_timestamp(1759656018i64)
            .unwrap(), }
        ];
        posts.sort_by(|a, b| b.created.cmp(&a.created));
        posts
    });
    &POSTS
}
