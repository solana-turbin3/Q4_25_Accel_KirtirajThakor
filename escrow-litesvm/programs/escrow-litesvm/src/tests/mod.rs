#[cfg(test)]
mod escrow_tests {
    use {
        anchor_lang::{
            prelude::msg,
            solana_program::program_pack::Pack,
            AccountDeserialize, InstructionData, ToAccountMetas,
        },
        anchor_spl::{
            associated_token::{self, spl_associated_token_account},
            token::spl_token,
        },
        litesvm::LiteSVM,
        litesvm_token::{spl_token::ID as TOKEN_PROGRAM_ID, CreateAssociatedTokenAccount, CreateMint, MintTo},
        solana_keypair::Keypair,
        solana_message::Message,
        solana_pubkey::Pubkey,
        solana_signer::Signer,
        solana_transaction::Transaction,
        solana_native_token::LAMPORTS_PER_SOL,
        std::path::PathBuf,
    };

    static PROGRAM_ID: Pubkey = crate::ID;

    fn init_env() -> (
        LiteSVM,
        Keypair,
        Keypair, 
        Pubkey, 
        Pubkey, 
        Pubkey, 
    ) {
        let mut svm = LiteSVM::new();
        let maker = Keypair::new();
        let taker = Keypair::new();

        svm.airdrop(&maker.pubkey(), 5 * LAMPORTS_PER_SOL).unwrap();
        svm.airdrop(&taker.pubkey(), 5 * LAMPORTS_PER_SOL).unwrap();

        let mint_a = CreateMint::new(&mut svm, &maker)
            .authority(&maker.pubkey())
            .decimals(6)
            .send()
            .unwrap();

        let mint_b = CreateMint::new(&mut svm, &taker)
            .authority(&taker.pubkey())
            .decimals(6)
            .send()
            .unwrap();

        let escrow = Pubkey::find_program_address(
            &[b"escrow", maker.pubkey().as_ref(), &123u64.to_le_bytes()],
            &PROGRAM_ID,
        )
        .0;

        
        let so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/deploy/anchor_escrow.so");
        let binary = std::fs::read(so_path).expect("Unable to load program");
        svm.add_program(PROGRAM_ID, &binary);

        (svm, maker, taker, mint_a, mint_b, escrow)
    }


    fn create_ata(program: &mut LiteSVM, owner: &Keypair, mint: &Pubkey) -> Pubkey {
        CreateAssociatedTokenAccount::new(program, owner, mint)
            .owner(&owner.pubkey())
            .send()
            .unwrap()
    }

    #[test]
    fn make_should_create_escrow_and_vault() {
        let (mut svm, maker, _, mint_a, mint_b, escrow) = init_env();

        let vault = associated_token::get_associated_token_address(&escrow, &mint_a);
        let token_program = TOKEN_PROGRAM_ID;
        let system_program = anchor_lang::solana_program::system_program::ID;

        let maker_ata_a = create_ata(&mut svm, &maker, &mint_a);

        MintTo::new(&mut svm, &maker, &mint_a, &maker_ata_a, 1_000_000)
            .send()
            .unwrap();

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: PROGRAM_ID,
            accounts: crate::accounts::Make {
                maker: maker.pubkey(),
                mint_a,
                mint_b,
                maker_ata_a,
                escrow,
                vault,
                associated_token_program: spl_associated_token_account::ID,
                token_program,
                system_program,
            }
            .to_account_metas(None),
            data: crate::instruction::Make {
                deposit: 50,
                seed: 123,
                receive: 100,
            }
            .data(),
        };

        let msg = Message::new(&[ix], Some(&maker.pubkey()));
        let tx = Transaction::new(&[&maker], msg, svm.latest_blockhash());
        let res = svm.send_transaction(tx).unwrap();

        msg!("Make executed successfully");
        msg!("Compute Units Used: {}", res.compute_units_consumed);

        let vault_acc = svm.get_account(&vault).unwrap();
        let vault_data = spl_token::state::Account::unpack(&vault_acc.data).unwrap();
        assert_eq!(vault_data.amount, 50);
        assert_eq!(vault_data.owner, escrow);
        assert_eq!(vault_data.mint, mint_a);
    }

    #[test]
    fn take_should_swap_and_close_escrow() {
        let (mut svm, maker, taker, mint_a, mint_b, escrow) = init_env();

        let vault = associated_token::get_associated_token_address(&escrow, &mint_a);
        let token_program = TOKEN_PROGRAM_ID;
        let system_program = anchor_lang::solana_program::system_program::ID;

        let maker_ata_a = create_ata(&mut svm, &maker, &mint_a);
        let maker_ata_b = create_ata(&mut svm, &maker, &mint_b);
        let taker_ata_a = create_ata(&mut svm, &taker, &mint_a);
        let taker_ata_b = create_ata(&mut svm, &taker, &mint_b);

        MintTo::new(&mut svm, &maker, &mint_a, &maker_ata_a, 500_000).send().unwrap();
        MintTo::new(&mut svm, &taker, &mint_b, &taker_ata_b, 500_000).send().unwrap();

        let make_ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: PROGRAM_ID,
            accounts: crate::accounts::Make {
                maker: maker.pubkey(),
                mint_a,
                mint_b,
                maker_ata_a,
                escrow,
                vault,
                associated_token_program: spl_associated_token_account::ID,
                token_program,
                system_program,
            }
            .to_account_metas(None),
            data: crate::instruction::Make {
                deposit: 20,
                seed: 123,
                receive: 20,
            }
            .data(),
        };
        let make_msg = Message::new(&[make_ix], Some(&maker.pubkey()));
        svm.send_transaction(Transaction::new(&[&maker], make_msg, svm.latest_blockhash()))
            .unwrap();

        let take_ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: PROGRAM_ID,
            accounts: crate::accounts::Take {
                maker: maker.pubkey(),
                taker: taker.pubkey(),
                mint_a,
                mint_b,
                maker_ata_b,
                taker_ata_a,
                taker_ata_b,
                escrow,
                vault,
                associated_token_program: spl_associated_token_account::ID,
                token_program,
                system_program,
            }
            .to_account_metas(None),
            data: crate::instruction::Take.data(),
        };
        let msg = Message::new(&[take_ix], Some(&taker.pubkey()));
        let res = svm.send_transaction(Transaction::new(&[&taker], msg, svm.latest_blockhash())).unwrap();

        msg!("Take executed successfully (CUs: {})", res.compute_units_consumed);

        let escrow_acc = svm.get_account(&escrow).unwrap();
        assert_eq!(escrow_acc.lamports, 0);
    }

    #[test]
    fn refund_should_return_tokens_to_maker() {
        let (mut svm, maker, _, mint_a, mint_b, escrow) = init_env();

        let vault = associated_token::get_associated_token_address(&escrow, &mint_a);
        let maker_ata_a = create_ata(&mut svm, &maker, &mint_a);
        let token_program = TOKEN_PROGRAM_ID;
        let system_program = anchor_lang::solana_program::system_program::ID;

        MintTo::new(&mut svm, &maker, &mint_a, &maker_ata_a, 200_000).send().unwrap();

        let make_ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: PROGRAM_ID,
            accounts: crate::accounts::Make {
                maker: maker.pubkey(),
                mint_a,
                mint_b,
                maker_ata_a,
                escrow,
                vault,
                associated_token_program: spl_associated_token_account::ID,
                token_program,
                system_program,
            }
            .to_account_metas(None),
            data: crate::instruction::Make {
                deposit: 40,
                seed: 123,
                receive: 40,
            }
            .data(),
        };
        let make_msg = Message::new(&[make_ix], Some(&maker.pubkey()));
        svm.send_transaction(Transaction::new(&[&maker], make_msg, svm.latest_blockhash()))
            .unwrap();

        let refund_ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: PROGRAM_ID,
            accounts: crate::accounts::Refund {
                maker: maker.pubkey(),
                mint_a,
                maker_ata_a,
                escrow,
                vault,
                token_program,
                system_program,
            }
            .to_account_metas(None),
            data: crate::instruction::Refund.data(),
        };

        let msg = Message::new(&[refund_ix], Some(&maker.pubkey()));
        let res = svm.send_transaction(Transaction::new(&[&maker], msg, svm.latest_blockhash())).unwrap();

        msg!("Refund completed successfully (CUs: {})", res.compute_units_consumed);
    }
}
