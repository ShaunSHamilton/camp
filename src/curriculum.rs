use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Serialize, Deserialize)]
pub struct Curriculum {
    pub certifications: Certifications,
    #[serde(flatten)]
    pub superblocks: HashMap<String, Superblock>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Certifications {
    pub blocks: HashMap<String, CertBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertBlock {
    pub challenges: Vec<CertChallenge>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CertChallenge {
    pub id: String,
    pub title: String,
    pub tests: Vec<CertTest>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CertTest {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Superblock {
    pub blocks: HashMap<String, Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub meta: Meta,
    pub challenges: Vec<Challenge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    #[serde(rename = "dashedName")]
    pub dashed_name: String,
    #[serde(rename = "challengeOrder")]
    pub challenge_order: Vec<ChallengeOrder>,
    pub order: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeOrder {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Challenge {
    pub id: String,
    pub title: String,
    #[serde(rename = "dashedName")]
    pub dashed_name: String,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta.name)
    }
}

impl Display for Challenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl Display for CertChallenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl Display for CertTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}
