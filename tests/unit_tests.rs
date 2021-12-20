#[cfg(test)]
mod tests {
    #[test]
    fn string_of_ints() {
        let result : regex_types::ReString<&str> = regex_types::ReString::new("Hello, World!", "World").unwrap();
        assert_eq!(result.result, "World");
    }
}