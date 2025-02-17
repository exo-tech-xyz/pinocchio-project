use litesvm::LiteSVM;

/// Get LiteSvm with myproject loaded.
pub fn lite_svm_with_programs() -> LiteSVM {
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/myproject.so");
    svm.add_program(myproject_client::programs::MYPROJECT_ID, bytes);
    svm
}
