use libbun::SelectedProviderPackage;

fn clone_selected(value: &SelectedProviderPackage) -> SelectedProviderPackage {
    value.clone()
}

fn main() {
    let _ = clone_selected;
}
