pub fn tokenize(expr: &str) -> Vec<String> {
    // 这里用了偷懒的方法来分词
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}
