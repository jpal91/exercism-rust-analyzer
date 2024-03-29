use cargo_metadata::{diagnostic::DiagnosticLevel, Message};
use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::analyzers::AnalysisComments;

fn cd_into_repo_root(path: &Path) {
    std::env::set_current_dir(path).unwrap();
}

// Parses any messages classified as "compiler-message" from clippy output.
// Clippy also emits "build" and "artifact" messages which aren't necessary for this purpose.
fn parse_msg(msg: Message) -> Option<AnalysisComments> {
    if let Message::CompilerMessage(m) = msg {
        let diagnostic = m.message;

        // Maybe TODO: The very last message - diagnostic.code is None.
        // It appears to only be a message stating how many warnings/errors
        // there are so may be worth it to filter. Unsure if it has more use.

        let analysis_type = match diagnostic.level {
            DiagnosticLevel::Error => "essential".to_string(),
            DiagnosticLevel::Warning => "actionable".to_string(),
            _ => "informative".to_string(),
        };

        let mut params = HashMap::with_capacity(1);

        if let Some(message) = diagnostic.rendered {
            params.insert("clippy".to_string(), message);
        } else {
            params.insert("clippy".to_string(), diagnostic.message);
        };

        Some(AnalysisComments::Extended {
            comment: "rust.general.clippy".to_string(),
            params,
            analysis_type,
        })
    } else {
        None
    }
}

/// Moves into the targeted exercise's repo, runs clippy, and returns parsed results
pub fn get_clippy(path: &Path) -> Result<Vec<AnalysisComments>, std::io::Error> {
    cd_into_repo_root(path);

    let mut cmd = Command::new("cargo")
        .args(["clippy", "--message-format=json"])
        .stdout(Stdio::piped())
        .spawn()?;

    let reader = std::io::BufReader::new(cmd.stdout.take().unwrap());

    let clippy_comments = Message::parse_stream(reader)
        .filter_map(|msg| parse_msg(msg.unwrap()))
        .collect();

    cmd.wait()?;

    Ok(clippy_comments)
}
