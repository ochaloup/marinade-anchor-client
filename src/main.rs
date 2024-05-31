use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::NullSigner},
    Client, Cluster,
};
use marinade_finance::state::State;
use std::str::FromStr;
use std::sync::Arc;
use std::{env, process::exit};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cluster_url = if args.len() > 1 {
        args[1].as_str()
    } else {
        "mainnet"
    };

    let anchor_cluster = Cluster::from_str(&cluster_url).unwrap_or_else(|_| {
        eprintln!("Erroneous cluster url: {}, cannot be parsed", cluster_url);
        exit(1)
    });
    let non_existent_fee_payer = Arc::from(NullSigner::new(&Pubkey::default()));
    let anchor_client = Client::new_with_options(
        anchor_cluster,
        non_existent_fee_payer,
        CommitmentConfig::confirmed(),
    );

    // --- Marinade go, go, go!
    let marinade_state_default = Pubkey::from_str("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC")
        .unwrap_or_else(|_| {
            eprintln!("Error parsing state address");
            exit(1)
        });
    let program = anchor_client.program(marinade_finance::id())?;
    let state: State = program.account(marinade_state_default).unwrap_or_else(|_| {
        eprintln!("Error fetching state {}", marinade_state_default);
        exit(1)
    });
    println!("state: {:?}", state);

    // --- What about some SPL stuff?
    // some MNDE  account taken address from latest blockhash, maybe it won't be existing in future
    let random_token_pubkey = Pubkey::from_str("8Qp54JB61k9J9CLr3V6cgjZxkwTLM4HbQr1D44FSxvsF")?;
    let spl_program = anchor_client.program(anchor_spl::token::spl_token::id())?;
    let token_info: anchor_spl::token::TokenAccount = spl_program.account(random_token_pubkey)?;
    println!(
        "token {random_token_pubkey} [owner: {}, amount: {}]",
        token_info.owner, token_info.amount
    );

    let init_mint = anchor_spl::token::spl_token::instruction::initialize_mint(
        &anchor_spl::token::spl_token::id(),
        &random_token_pubkey,
        &random_token_pubkey,
        None,
        0,
    )?;
    let request_builder = spl_program.request();
    let _ = request_builder.instruction(init_mint);

    Ok(())
}
