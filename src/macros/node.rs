#[macro_export]
macro_rules! get_node_by_abs_path {
    ($base:expr, $path:expr) => {{
        $base
            .get_tree()
            .unwrap()
            .get_root()
            .unwrap()
            .get_node_as($path)
    }};
}
