pub use core::theme::{DARK, FERRARI, LIGHT};
use core::{minimize_css, theme::Theme, Extension};

use crate::Generator;

impl Extension<Generator> for Theme {
    async fn extend(
        &self,
        _: &mut Generator,
        _: &mut Vec<core::item::Item>,
        style: &mut Vec<String>,
    ) {
        let vars = self.format_vars();
        let (start, end) =
            if self.prefered_color_scheme == "dark" || self.prefered_color_scheme == "light" {
                (
                    format!(
                        "@media (prefers-color-scheme: {}) {{:root{{",
                        self.prefered_color_scheme
                    ),
                    '}',
                )
            } else {
                (":root{".into(), ' ')
            };
        let mut theme = vars
            .into_iter()
            .fold(start, |acc, var| format!("{}{}", acc, var,));
        theme.push('}');
        theme.push_str(DEFAULT_STR);
        theme.push_str(self.style);
        theme.push(end);
        style.push(minimize_css(&theme).to_string());
    }
}

const DEFAULT_STR: &str = "#background {
      fill: var(--bg-0)
    }
    #total-solved-bg {
      stroke: var(--bg-1)
    }
    #easy-solved-bg {
      stroke: var(--bg-1)
    }
    #medium-solved-bg {
      stroke: var(--bg-1)
    }
    #hard-solved-bg {
      stroke: var(--bg-1)
    }
    #username {
      fill: var(--text-0)
    }
    #username-text {
      fill: var(--text-0)
    }
    #total-solved-text {
      fill: var(--text-0)
    }
    #easy-solved-type {
      fill: var(--text-0)
    }
    #medium-solved-type {
      fill: var(--text-0)
    }
    #hard-solved-type {
      fill: var(--text-0)
    }
    #ranking {
      fill: var(--text-1)
    }
    #easy-solved-count {
      fill: var(--text-1)
    }
    #medium-solved-count {
      fill: var(--text-1)
    }
    #hard-solved-count {
      fill: var(--text-1)
    }
    #total-solved-ring {
      stroke: var(--color-0)
    }
    #easy-solved-progress {
      stroke: var(--color-1)
    }
    #medium-solved-progress {
      stroke: var(--color-2)
    }
    #hard-solved-progress {
      stroke: var(--color-3)
    }
";
