#[cfg(test)]
mod tests {
    #[test]
    fn hello_world() {
        let result: regex_types::ReString<&str> =
            regex_types::ReString::new("Hello, World!", "World").unwrap();
        assert_eq!(result.result, "World");
    }

    #[test]
    fn iterator_test() {
        let s = "Good, good! Let's the hate flow through you. GOOD!";

        for (i, result) in regex_types::ReString::<String>::new(s, "[Gg]")
            .unwrap()
            .into_iter()
            .enumerate()
        {
            match i {
                0 => assert_eq!(result, "ood, good! Let's the hate flow through you. GOOD!"),
                1 => assert_eq!(result, "ood! Let's the hate flow through you. GOOD!"),
                2 => assert_eq!(result, "h you. GOOD!"),
                3 => assert_eq!(result, "OOD!"),
                _ => unreachable!(),
            };
        }
    }
}
