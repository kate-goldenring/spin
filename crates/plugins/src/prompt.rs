/// Prompts user as to whether they trust the source of the plugin and
/// want to proceed with installation
use anyhow::Result;
use std::io;
use url::Url;

pub(crate) struct Prompter {
    plugin_name: String,
    plugin_license: String,
    plugin_repo_url: Url,
    source_url: String,
}

impl Prompter {
    pub fn new(
        plugin_name: &str,
        plugin_license: &str,
        plugins_repo_url: Url,
        source_url: &str,
    ) -> Result<Self> {
        Ok(Self {
            plugin_name: plugin_name.to_string(),
            plugin_license: plugin_license.to_string(),
            plugin_repo_url: plugins_repo_url.join(&format!("{}.json", plugin_name))?,
            source_url: source_url.to_string(),
        })
    }
    fn print_prompt(&self) {
        println!(
            "Installing plugin {} with license {} from {}\n",
            self.plugin_name, self.plugin_license, self.source_url
        );
        println!(
            "For more information, reference the plugin metadata at {}\n",
            self.plugin_repo_url
        );
        println!("Are you sure you want to proceed? ('yes'/'no') (default: no)");
    }

    fn are_you_sure(&self) -> Result<bool> {
        let mut resp = String::new();
        io::stdin().read_line(&mut resp)?;
        Ok(self.parse_response(&mut resp))
    }
    fn parse_response(&self, resp: &mut str) -> bool {
        resp.trim().to_lowercase().eq("yes")
        // TODO: consider checking for invalid response
    }

    // Returns whether or not the user would like to proceed with the installation
    pub(crate) fn run(&self) -> Result<bool> {
        self.print_prompt();
        self.are_you_sure()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_response() {
        let p = Prompter::new(
            "best-plugin",
            "MIT",
            Url::parse("www.spin.com/spin-plugins/best-plugin").unwrap(),
            "www.example.com",
        )
        .unwrap();
        let mut resp = String::from("\n\t  yes   ");
        assert!(p.parse_response(&mut resp));
    }
}
