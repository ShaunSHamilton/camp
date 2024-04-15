use proptest::{
    collection::vec, option::of, prelude::*, strategy::ValueTree, test_runner::TestRunner,
};
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};

#[derive(Arbitrary, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[proptest(strategy = "of(\"[a-zA-Z0-9 ]{20,100}\")")]
    pub about: Option<String>,
    pub accepted_privacy_terms: bool,
    #[proptest(strategy = "vec(any::<CompletedChallenge>(), 0..1000)")]
    pub completed_challenges: Vec<CompletedChallenge>,
    // TODO: Ensure uniqueness
    #[proptest(regex = "[a-zA-Z0-9]{2,20}@[a-z]{2,10}\\.(com|org|edu)")]
    pub email: String,
    #[proptest(strategy = "of(\"https://github\\\\.com/[a-zA-Z\\\\-]{1,20}\")")]
    pub github_profile: Option<String>,
    pub is_apis_microservices_cert: bool,
    pub is_back_end_cert: bool,
    pub is_banned: bool,
    pub is_cheater: bool,
    pub is_college_algebra_py_cert_v8: bool,
    pub is_data_analysis_py_cert_v7: bool,
    pub is_data_vis_cert: bool,
    pub is_donating: bool,
    pub is_front_end_cert: bool,
    pub is_front_end_libs_cert: bool,
    pub is_full_stack_cert: bool,
    pub is_foundational_c_sharp_cert_v8: bool,
    pub is_honest: bool,
    pub is_infosec_cert_v7: bool,
    pub is_infosec_qa_cert: bool,
    pub is_js_algo_data_struct_cert: bool,
    pub is_js_algo_data_struct_cert_v8: bool,
    pub is_machine_learning_py_cert_v7: bool,
    pub is_qa_cert_v7: bool,
    pub is_relational_database_cert_v8: bool,
    pub is_resp_web_design_cert: bool,
    pub is_sci_comp_py_cert_v7: bool,
    pub is_2018_data_vis_cert: bool,
    pub is_2018_full_stack_cert: bool,
    pub keyboard_shortcuts: Option<bool>,
    #[proptest(strategy = "of(\"[a-zA-Z0-9]{20,100}\")")]
    pub linkedin: Option<String>,
    #[proptest(strategy = "of(\"[a-zA-Z ]{1,20}\")")]
    pub name: Option<String>,
    pub needs_moderation: Option<bool>,
    #[proptest(strategy = "of(vec(any::<Timestamp>(), 0..10))")]
    pub partially_completed_challenges: Option<Vec<Timestamp>>,
    #[proptest(strategy = "of(\"https://[a-zA-Z]{1,20}\")")]
    pub picture: Option<String>,
    #[serde(rename = "profileUI")]
    pub profile_ui: ProfileSettings,
    #[proptest(strategy = "vec(any::<Timestamp>(), 0..1000)")]
    pub progress_timestamps: Vec<Timestamp>,
    pub rand: f64,
    #[proptest(strategy = "of(vec(any::<SavedChallenge>(), 0..10))")]
    pub saved_challenges: Option<Vec<SavedChallenge>>,
    pub send_quincy_email: bool,
    pub sound: Option<bool>,
    #[proptest(strategy = "of(\"https://twitter\\\\.com/[a-zA-Z]{1,20}\")")]
    pub twitter: Option<String>,
    // TODO: Ensure uniqueness
    #[proptest(regex = "[a-z][a-z0-9_]{1,25}")]
    pub username: String,
    #[proptest(strategy = "of(\"[a-zA-Z]{1,20}\")")]
    pub username_display: Option<String>,
    #[proptest(strategy = "of(\"https://[a-zA-Z]{1,20}\\\\.com\")")]
    pub website: Option<String>,
    #[proptest(strategy = "of(vec(\"20[12][0-9]\", 8))")]
    pub years_top_contributor: Option<Vec<String>>,
}

#[derive(Arbitrary, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    #[proptest(filter = "|x| *x > 0")]
    completed_date: isize,
    #[proptest(regex = "[a-zA-Z0-9]{24}")]
    id: String,
}

#[derive(Arbitrary, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSettings {
    is_locked: bool,
    show_about: bool,
    show_certs: bool,
    show_donation: bool,
    show_heat_map: bool,
    show_location: bool,
    show_name: bool,
    show_points: bool,
    show_portfolio: bool,
    show_time_line: bool,
}

#[derive(Arbitrary, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletedChallenge {
    pub challenge_type: u8,
    #[proptest(filter = "|x| *x > 0")]
    pub completed_date: isize,
    #[proptest(strategy = "vec(any::<File>(), 0..10)")]
    pub files: Vec<File>,
    #[proptest(strategy = "of(\"https://github\\\\.com/[a-zA-Z\\\\-]{1,20}\")")]
    pub github_link: Option<String>,
    #[proptest(regex = "[a-zA-Z0-9]{24}")]
    pub id: String,
    pub is_manually_approved: bool,
    #[proptest(regex = "[a-zA-Z0-9 ]{50,300}")]
    pub solution: String,
}
#[derive(Arbitrary, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedChallenge {
    pub challenge_type: u8,
    #[proptest(strategy = "vec(any::<File>(), 0..10)")]
    pub files: Vec<File>,
    #[proptest(regex = "[a-zA-Z0-9]{24}")]
    pub id: String,
    #[proptest(filter = "|x| *x > 0")]
    pub last_saved_date: isize,
}

#[derive(Arbitrary, Debug, Serialize, Deserialize)]
pub struct File {
    // #[proptest(regex = "[a-zA-Z0-9]{20,100}")]
    pub contents: String,
    #[proptest(regex = "\\\\.[a-z]{2,4}")]
    pub ext: String,
    #[proptest(regex = "[a-z]{2,4}")]
    pub key: String,
    #[proptest(regex = "[a-z]{2,4}")]
    pub name: String,
    #[proptest(regex = "[a-z]{2,4}")]
    pub path: String,
}

// Generate a random user
pub fn generate_user() -> User {
    let mut runner = TestRunner::default();
    // TODO: smart generation for ids
    let user = User::arbitrary().new_tree(&mut runner).unwrap().current();
    user
}
