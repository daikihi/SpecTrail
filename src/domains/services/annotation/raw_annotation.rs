pub struct RawAnnotation {
    pub id: String,
    pub layer: String,
    pub annotation_type: String,
    pub name: String,
    pub links: Vec<String>,  // 生のID文字列のみ、例：["@st-foo", "@st-bar"]
    pub source_file: String, // このアノテーションが見つかったファイルパス
}
