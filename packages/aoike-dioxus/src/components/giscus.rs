use dioxus::{
    core::{Element, IntoAttributeValue},
    prelude::*,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GiscusOptions {
    repo: String,
    repo_id: String,
    category: Option<String>,
    category_id: String,
    mapping: Mapping,
    strict: bool,
    reactions_enabled: bool,
    emit_metadata: bool,
    input_position: InputPosition,
    theme: String,
    lang: String,
    lazy: bool,
}


#[derive(Debug, Clone, PartialEq)]
pub enum InputPosition {
    Bottom,
    Top,
}

impl IntoAttributeValue for InputPosition {
    fn into_value(self) -> dioxus_core::AttributeValue {
        match self {
            InputPosition::Bottom => "bottom".to_string(),
            InputPosition::Top => "top".to_string(),
        }
        .into_value()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mapping {
    Pathname,
    Url,
    Title,
    OgTitle,
    Specific(String),
    Number(usize),
}

impl GiscusOptions {
    pub fn new(repo: String, repo_id: String, category_id: String) -> Self {
        Self {
            repo,
            repo_id,
            category: None,
            category_id,
            mapping: Mapping::Pathname,
            strict: false,
            reactions_enabled: false,
            emit_metadata: false,
            input_position: InputPosition::Bottom,
            theme: "preferred_color_scheme".to_string(),
            lang: "zh-CN".to_string(),
            lazy: false,
        }
    }
    pub fn with_category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }
    pub fn with_reactions_enabled(mut self, reactions_enabled: bool) -> Self {
        self.reactions_enabled = reactions_enabled;
        self
    }
    pub fn with_emit_metadata(mut self, emit_metadata: bool) -> Self {
        self.emit_metadata = emit_metadata;
        self
    }
    pub fn with_lazy(mut self, lazy: bool) -> Self {
        self.lazy = lazy;
        self
    }
    pub fn with_input_position(mut self, input_position: InputPosition) -> Self {
        self.input_position = input_position;
        self
    }
}

#[component]
pub fn Giscus(options: GiscusOptions) -> Element {
    let mapping = match &options.mapping {
        Mapping::Pathname => "pathname",
        Mapping::Url => "url",
        Mapping::Title => "title",
        Mapping::OgTitle => "og:title",
        Mapping::Specific(_) => "specific",
        Mapping::Number(_) => "number",
    };
    let term = match &options.mapping {
        Mapping::Specific(s) => Some(s.clone()),
        Mapping::Number(n) => Some(n.to_string()),
        _ => None,
    };
    rsx! {
        script {
            src: "https://giscus.app/client.js",
            "data-repo": options.repo,
            "data-repo-id": options.repo_id,
            "data-category": options.category,
            "data-category-id": options.category_id,
            "data-mapping": mapping,
            "data-term": term,
            "data-strict": if options.strict {
                "1"
            } else {
                "0"
            },
            "data-reactions-enabled": if options.reactions_enabled {
                "1"
            } else {
                "0"
            },
            "data-emit-metadata": if options.emit_metadata {
                "1"
            } else {
                "0"
            },
            "data-input-position": options.input_position,
            "data-theme": options.theme,
            "data-lang": options.lang,
            "data-loading": if options.lazy {
                Some("lazy")
            } else {
                None
            },
            "crossorigin": "anonymous"
        }
    }
}
