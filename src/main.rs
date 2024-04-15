//! CLI that adds challenges to the freeCodeCamp database.
//!
//! Assumes usage in the root of `freeCodeCamp/freeCodeCamp`. Otherwise, the `--curriculum-path` arg must be used to specify the relative location to the `curriculum.json` file.
//!
use clap::Parser;
use inquire::{MultiSelect, Select};
use mongodb::bson::{doc, Document};
use std::path::Path;
use user::generate_user;

mod clapper;
mod curriculum;
mod db;
mod user;

use clapper::{Args, SubCommand};
use curriculum::{Block, CertChallenge, Challenge, Curriculum};
use db::get_collection;

fn main() -> mongodb::error::Result<()> {
    let args = Args::parse();

    let collection = get_collection(&args.uri)?;

    let curriculum_path = &args.curriculum_path;

    let curriculum = get_curriculum(curriculum_path);

    let username = args.username;

    let query = doc! {"username": username};
    match args.sub_commands {
        SubCommand::ClaimCerts => {
            let certifications = claim_certifications(&curriculum).unwrap();

            let cert_tests = certifications
                .iter()
                .flat_map(|cert_challenge| cert_challenge.tests.clone())
                .collect::<Vec<_>>();

            let options = None;

            // Push `{ id: challenge.id, completedDate: Date.now() }` to the `completedChallenges` array
            let update = doc! {
                "$push": {
                    "completedChallenges": {
                        "$each": cert_tests.iter().map(|cert_test| {
                            doc! {
                                "id": cert_test.id.clone(),
                                // Current date in milliseconds since Unix epoch
                                "completedDate": chrono::Utc::now().timestamp_millis()
                            }
                        }).collect::<Vec<Document>>()
                    }
                }
            };

            collection.update_one(query, update, options)?;
        }
        SubCommand::AddChallenges => {
            let challenges = add_challenges(&curriculum).unwrap();

            let options = None;

            // Push `{ id: challenge.id, completedDate: Date.now() }` to the `completedChallenges` array
            // TODO: Update completedDate, if challenge already completed.
            let update = doc! {
                "$push": {
                    "completedChallenges": {
                        "$each": challenges.iter().map(|challenge| {
                            doc! {
                                "id": challenge.id.clone(),
                                "completedDate": chrono::Utc::now().timestamp_millis()
                            }
                        }).collect::<Vec<Document>>()
                    }
                }
            };

            collection.update_one(query, update, options)?;
        }
        SubCommand::FinishFreeCodeCamp => {
            let challenges = get_challenges(&curriculum);

            let options = None;

            let update = doc! {
                "$push": {
                    "completedChallenges": {
                        "$each": challenges.iter().map(|challenge| {
                            doc! {
                                "id": challenge.id.clone(),
                                "completedDate": chrono::Utc::now().timestamp_millis()
                            }
                        }).collect::<Vec<Document>>()
                    }
                }
            };

            collection.update_one(query, update, options)?;
        }
        SubCommand::AddUsers { count } => {
            for _ in 0..count {
                let user = generate_user();

                // TODO: This cannot be the best way to convert
                let user_string = serde_json::to_string(&user).unwrap();
                let document: Document = serde_json::from_str(&user_string).unwrap();

                collection.insert_one(document, None)?;
            }
        }
    }
    Ok(())
}

fn add_challenges(curriculum: &Curriculum) -> Result<Vec<Challenge>, ()> {
    let superblocks = &curriculum.superblocks;
    let superblock_dashed_names: Vec<String> = superblocks.keys().cloned().collect();

    let superblock = Select::new("Superblock: ", superblock_dashed_names)
        .prompt()
        .expect("user to select a superblock");

    let blocks = &superblocks.get(&superblock).unwrap().blocks;
    let mut blocks_in_order = blocks.values().collect::<Vec<&Block>>();

    blocks_in_order.sort_by(|a, b| a.meta.order.cmp(&b.meta.order));

    let block = Select::new("Block: ", blocks_in_order)
        .prompt()
        .expect("user to select a block");

    let challenges = blocks
        .get(&block.meta.dashed_name)
        .unwrap()
        .challenges
        .clone();

    let selected_challenges = MultiSelect::new("Challenges: ", challenges)
        .prompt()
        .expect("user to select challenges");

    Ok(selected_challenges)
}

fn claim_certifications(curriculum: &curriculum::Curriculum) -> Result<Vec<CertChallenge>, ()> {
    let certifications = &curriculum.certifications.blocks;
    let selected_certifications: Vec<CertChallenge> = certifications
        .values()
        .flat_map(|cert_block| cert_block.challenges.clone())
        .collect();

    let certifications = MultiSelect::new("Certification: ", selected_certifications)
        .prompt()
        .expect("user to select a certification");

    Ok(certifications)
}

fn get_curriculum(curriculum_path: &Path) -> curriculum::Curriculum {
    let curriculum = std::fs::read_to_string(curriculum_path).expect("Could not read file");
    let curriculum: curriculum::Curriculum =
        serde_json::from_str(&curriculum).expect("Could not parse JSON");
    curriculum
}

fn get_challenges(curriculum: &Curriculum) -> Vec<Challenge> {
    let challenges = curriculum
        .superblocks
        .values()
        .flat_map(|superblock| {
            superblock
                .blocks
                .values()
                .flat_map(|block| block.challenges.clone())
                .collect::<Vec<Challenge>>()
        })
        .collect();
    challenges
}
