#[cfg(test)]
mod files {
    #[test]
    fn test_create_dir_html() {
        use std::collections::HashMap;
        let mut routes = HashMap::new();
        routes.insert("/test_files/index.html".to_string(), "tests/test_files/index.html".to_string());
        let dir: String = String::from("tests/test_files/");
        assert_eq!(simple_http::create_dir_html(dir, &routes), "<!DOCTYPE html><html><body><a href='/test_files'>../</a><br><a href='/test_files/index.html'>/test_files/index.html</a><br></body><style>body {\n                    font-family: Courier new;\n                    display: inline-block;\n                    position: absolute;\n                    background-color: #0f1419;\n                    left: 40%; \n                    top: 12%;}\n                    a, a:hover, a:visited, a:active {\n                    color: #b9b9ba;\n                    text-decoration: none;\n                    }</style></body></html>");
    }
}