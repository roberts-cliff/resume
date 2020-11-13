#[macro_use]
extern crate schemars_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

pub mod data_structure {
    // serde defaults are kinda strange to deal with, these functions work but are uglyish to me
    fn default_tenure_start() -> String {
        "".to_string()
    }

    fn default_tenure_end() -> String {
        "present".to_string()
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
    pub struct Tenure {
        #[serde(default = "default_tenure_start")]
        pub start: String,
        #[serde(default = "default_tenure_end")]
        pub end: String,
    }

    impl Default for Tenure {
        fn default() -> Self {
            Tenure { start: "now".to_string(), end: "never".to_string() }
        }
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
    pub struct Project {
        pub name: String,
        #[serde(default)]
        pub period: Tenure,
        pub description: String,
        #[serde(default)]
        pub bullets: Vec<String>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
    pub struct Job {
        pub company: String,
        pub tenure: Tenure,
        pub roles: Vec<Project>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
    pub struct Person {
        pub name: String,
        pub contact: String,
        pub purpose: String,
        pub skills: Vec<String>,
        pub jobs: Vec<Job>,
        pub education: Vec<String>
    }
}



