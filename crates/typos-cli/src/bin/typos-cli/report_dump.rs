use std::io::Write;
use std::sync::Mutex;
use typos_cli::report::{Message, Report};

pub(crate) struct DumpIgnoresReporter<'r> {
    ignores: Mutex<std::collections::HashSet<String>>,
    reporter: &'r dyn Report,
}

impl<'r> DumpIgnoresReporter<'r> {
    pub(crate) fn new(reporter: &'r dyn Report) -> Self {
        Self {
            ignores: Mutex::new(std::collections::HashSet::new()),
            reporter,
        }
    }

    pub(crate) fn dump(&self, output_path: &std::path::Path) -> std::io::Result<()> {
        let set = self.ignores.lock().unwrap();
        let mut sorted_ignores: Vec<_> = set.iter().collect();
        sorted_ignores.sort();

        // Always create the file, even if empty, as per user expectation
        let mut file = std::fs::File::create(output_path)?;
        for typo in sorted_ignores {
            writeln!(file, "{}", typo)?;
        }
        Ok(())
    }
}

impl Report for DumpIgnoresReporter<'_> {
    fn report(&self, msg: Message<'_>) -> Result<(), std::io::Error> {
        if let Message::Typo(typo) = &msg {
            let typo_str = typo.typo.to_lowercase();
            self.ignores.lock().unwrap().insert(typo_str);
        }
        self.reporter.report(msg)
    }

    fn generate_final_result(&self) -> Result<(), std::io::Error> {
        self.reporter.generate_final_result()
    }
}
