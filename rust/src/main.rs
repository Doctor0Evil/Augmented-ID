//! # Augmented-ID Core Binary
//!
//! Main entry point for the Augmented-ID command-line interface.
//! Provides tools for identity management, token generation, and
//! ledger operations.
//!
//! ## Usage
//!
//! ```bash
//! # Generate new citizen identity
//! augid_core citizen new --did bostrom18... --name "John Doe"
//!
//! # Generate BfcToken.v1
//! augid_core token generate --citizen <citizen_id> --consent CONFIRMED
//!
//! # Validate token
//! augid_core token validate --token <token_json>
//!
//! # Check ledger integrity
//! augid_core ledger verify --chain <chain_file>
//! ```

use augid_core::{
    AugmentedCitizenId, AugIdStatus, AugIdLedgerEntry, BfcTokenV1,
    ConsentState, InterfaceType, CapsOk, EcoFlags,
};
use augid_core::crypto::{KeyPair, sha256_hash, sign_data};
use augid_core::guards::{
    AugFingerprintGuard, AntiRollbackGuard, GuardContext, BiophysicalState,
};
use chrono::Utc;
use std::env;
use std::fs;
use std::io::{self, Write};
use serde_json;

/// Print usage information
fn print_usage() {
    println!("Augmented-ID Core v{}", augid_core::VERSION);
    println!("ALN Schema Version: {}", augid_core::ALN_SCHEMA_VERSION);
    println!();
    println!("Usage: augid_core <command> [options]");
    println!();
    println!("Commands:");
    println!("  citizen new     Create new citizen identity");
    println!("  token generate  Generate BfcToken.v1");
    println!("  token validate  Validate BfcToken.v1");
    println!("  ledger verify   Verify ledger chain integrity");
    println!("  guard check     Run guard validation");
    println!("  help            Show this help message");
    println!();
    println!("Examples:");
    println!("  augid_core citizen new --did bostrom18... --region US-AZ");
    println!("  augid_core token generate --citizen citizen.json --consent CONFIRMED");
    println!("  augid_core token validate --token token.json");
    println!("  augid_core ledger verify --chain chain.json");
}

/// Create new citizen identity
fn cmd_citizen_new(args: &[String]) -> Result<(), String> {
    let mut did = String::new();
    let mut region = String::from("US-AZ");
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--did" => {
                i += 1;
                if i < args.len() {
                    did = args[i].clone();
                }
            }
            "--region" => {
                i += 1;
                if i < args.len() {
                    region = args[i].clone();
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    if did.is_empty() {
        return Err("DID required (--did bostrom18...)".to_string());
    }
    
    let citizen = AugmentedCitizenId::new(
        did,
        sha256_hash(b"legal_name_placeholder"),
        region,
        "vault:biometric_binding_placeholder".to_string(),
        AugIdStatus::Active,
    )?;
    
    let json = serde_json::to_string_pretty(&citizen)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    println!("{}", json);
    Ok(())
}

/// Generate BfcToken.v1
fn cmd_token_generate(args: &[String]) -> Result<(), String> {
    let mut citizen_file = String::new();
    let mut consent = String::from("CONFIRMED");
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--citizen" => {
                i += 1;
                if i < args.len() {
                    citizen_file = args[i].clone();
                }
            }
            "--consent" => {
                i += 1;
                if i < args.len() {
                    consent = args[i].clone();
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    if citizen_file.is_empty() {
        return Err("Citizen file required (--citizen <file>)".to_string());
    }
    
    // Load citizen from file
    let citizen_json = fs::read_to_string(&citizen_file)
        .map_err(|e| format!("Failed to read citizen file: {}", e))?;
    
    let citizen: AugmentedCitizenId = serde_json::from_str(&citizen_json)
        .map_err(|e| format!("Failed to parse citizen: {}", e))?;
    
    // Parse consent state
    let consent_state = match consent.as_str() {
        "CONFIRMED" => ConsentState::Confirmed,
        "DENY" => ConsentState::Denied,
        "SUSPENDED" => ConsentState::Suspended,
        _ => return Err("Invalid consent state (CONFIRMED, DENY, SUSPENDED)".to_string()),
    };
    
    // Generate token
    let mut token = BfcTokenV1::new(
        &citizen,
        consent_state,
        InterfaceType::MobileApp,
        CapsOk {
            spend_cap_ok: true,
            prompt_cap_ok: true,
            id_check_ok: true,
        },
        EcoFlags {
            eco_impact_score_band: "Gold".to_string(),
            eaccessibility: true,
            service_class_basic: "Enabled".to_string(),
        },
    )?;
    
    // Sign token (placeholder - would use actual key in production)
    let keypair = KeyPair::generate()
        .map_err(|e| format!("Key generation failed: {:?}", e))?;
    
    let signature = sign_data(&keypair.signing_key, b"token_data")
        .map_err(|e| format!("Signing failed: {:?}", e))?;
    
    token.sign(signature);
    
    let json = serde_json::to_string_pretty(&token)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    println!("{}", json);
    Ok(())
}

/// Validate BfcToken.v1
fn cmd_token_validate(args: &[String]) -> Result<(), String> {
    let mut token_file = String::new();
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--token" => {
                i += 1;
                if i < args.len() {
                    token_file = args[i].clone();
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    if token_file.is_empty() {
        return Err("Token file required (--token <file>)".to_string());
    }
    
    // Load token from file
    let token_json = fs::read_to_string(&token_file)
        .map_err(|e| format!("Failed to read token file: {}", e))?;
    
    let token: BfcTokenV1 = serde_json::from_str(&token_json)
        .map_err(|e| format!("Failed to parse token: {}", e))?;
    
    // Validate token
    token.validate()
        .map_err(|e| format!("Validation failed: {:?}", e))?;
    
    println!("✓ Token is valid");
    println!("  Token ID: {}", token.tokenid);
    println!("  Version: {}", token.token_version);
    println!("  Consent: {:?}", token.aiconsentstate);
    println!("  Valid Until: {}", token.valid_until);
    println!("  Neurorights Flags: {} present", token.neurorights_flags.len());
    
    Ok(())
}

/// Verify ledger chain integrity
fn cmd_ledger_verify(args: &[String]) -> Result<(), String> {
    let mut chain_file = String::new();
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--chain" => {
                i += 1;
                if i < args.len() {
                    chain_file = args[i].clone();
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    if chain_file.is_empty() {
        return Err("Chain file required (--chain <file>)".to_string());
    }
    
    // Load chain from file
    let chain_json = fs::read_to_string(&chain_file)
        .map_err(|e| format!("Failed to read chain file: {}", e))?;
    
    let entries: Vec<AugIdLedgerEntry> = serde_json::from_str(&chain_json)
        .map_err(|e| format!("Failed to parse chain: {}", e))?;
    
    // Verify chain integrity
    AntiRollbackGuard::validate_chain_integrity(&entries)
        .map_err(|e| format!("Chain validation failed: {:?}", e))?;
    
    println!("✓ Chain integrity verified");
    println!("  Total Entries: {}", entries.len());
    println!("  First Entry: {}", entries.first().map(|e| &e.entry_id).unwrap_or(&"N/A".to_string()));
    println!("  Last Entry: {}", entries.last().map(|e| &e.entry_id).unwrap_or(&"N/A".to_string()));
    
    Ok(())
}

/// Run guard validation
fn cmd_guard_check(args: &[String]) -> Result<(), String> {
    let mut citizen_file = String::new();
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--citizen" => {
                i += 1;
                if i < args.len() {
                    citizen_file = args[i].clone();
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    if citizen_file.is_empty() {
        return Err("Citizen file required (--citizen <file>)".to_string());
    }
    
    // Load citizen from file
    let citizen_json = fs::read_to_string(&citizen_file)
        .map_err(|e| format!("Failed to read citizen file: {}", e))?;
    
    let citizen: AugmentedCitizenId = serde_json::from_str(&citizen_json)
        .map_err(|e| format!("Failed to parse citizen: {}", e))?;
    
    // Run neurorights guard
    AugFingerprintGuard::validate_neurorights(&citizen.neurightsflags)
        .map_err(|e| format!("Neurorights validation failed: {:?}", e))?;
    
    // Run anti-rollback guard
    citizen.verify_antirollback()
        .map_err(|e| format!("Anti-rollback validation failed: {:?}", e))?;
    
    println!("✓ All guards passed");
    println!("  Neurorights Flags: {} present", citizen.neurightsflags.len());
    println!("  Anti-Rollback: {}", citizen.antirollback);
    println!("  Status: {:?}", citizen.status);
    
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let command = &args[1];
    let subcommand = args.get(2).map(|s| s.as_str()).unwrap_or("");
    let remaining_args = &args[3..];
    
    let result = match (command.as_str(), subcommand) {
        ("help", _) | ("--help", _) | ("-h", _) => {
            print_usage();
            Ok(())
        }
        ("citizen", "new") => cmd_citizen_new(remaining_args),
        ("token", "generate") => cmd_token_generate(remaining_args),
        ("token", "validate") => cmd_token_validate(remaining_args),
        ("ledger", "verify") => cmd_ledger_verify(remaining_args),
        ("guard", "check") => cmd_guard_check(remaining_args),
        _ => {
            eprintln!("Unknown command: {} {}", command, subcommand);
            print_usage();
            std::process::exit(1);
        }
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
