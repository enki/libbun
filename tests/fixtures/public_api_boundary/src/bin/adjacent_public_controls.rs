use libbun::DriveControl;
use libbun::MechanicalTerminal;
use libbun::PreparedExport;

fn adjacent_public_controls_remain_available(
    _prepared: Option<PreparedExport>,
    _control: Option<DriveControl>,
) -> Option<MechanicalTerminal> {
    None
}

fn main() {
    let _ = adjacent_public_controls_remain_available
        as fn(Option<PreparedExport>, Option<DriveControl>) -> Option<MechanicalTerminal>;
}
