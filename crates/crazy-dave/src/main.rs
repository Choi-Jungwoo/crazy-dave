use dll_syringe::{process::OwnedProcess, Syringe};

fn main() {
    let target_process = OwnedProcess::find_first_by_name("PlantsVsZombies.exe").unwrap();
    let syringe = Syringe::for_process(target_process);
    let _injected_payload = syringe.inject("crazy_dave_dynlib.dll").unwrap();
}
