use libbun::{BunModuleSpec, ProviderContractIdentity, ProviderDomainClass, SelectedProviderPackage};

fn main() {
    let _ = SelectedProviderPackage {
        brand: 7,
        contract: ProviderContractIdentity {
            package: "forbidden".to_owned(),
            capability: "forbidden".to_owned(),
            contract_fingerprint: "forbidden".to_owned(),
        },
        domain: ProviderDomainClass::JavaScriptExternalTransport,
        module: BunModuleSpec::Source {
            module_id: "forbidden".to_owned(),
            source: "forbidden".to_owned(),
        },
        export: "forbidden".to_owned(),
    };
}
