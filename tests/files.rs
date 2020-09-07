#[cfg(test)]
mod tests {
    #[test]
    fn check_for_file() {
        let res = simple_http::is_file("tests/test_files/index.html");
        assert_eq!(res, true);
    }

    #[test]
    fn check_for_dir() {
        let res = simple_http::is_dir("tests/test_files/");
        assert_eq!(res, true);
    }
}

