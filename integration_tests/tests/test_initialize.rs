use helpers::lite_svm_with_programs;
use myproject_client::instructions::InitializeBuilder;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

mod helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn initialize_success() {
        let user_kp = Keypair::new();
        let mut svm = lite_svm_with_programs();
        svm.airdrop(&user_kp.pubkey(), 100_000_000).unwrap();

        let ix = InitializeBuilder::new().instruction();

        let transaction = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user_kp.pubkey()),
            &[&user_kp],
            svm.latest_blockhash(),
        );

        let _tx_resp = svm.send_transaction(transaction).unwrap();
    }
}
