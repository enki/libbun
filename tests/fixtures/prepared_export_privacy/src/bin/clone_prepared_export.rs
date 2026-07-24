use libbun::PreparedExport;

fn clone_prepared(value: &PreparedExport) -> PreparedExport {
    value.clone()
}

fn main() {
    let _ = clone_prepared;
}
