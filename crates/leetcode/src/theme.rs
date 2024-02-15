use core::Extension;

use crate::Generator;

#[derive(Debug, Clone)]
pub struct Theme {
    prefered_color_scheme: &'static str,
    colors: &'static [Variable],
    style: &'static str,
}

impl Extension<Generator> for Theme {
    fn extend(&self, _: &mut Generator, _: &mut Vec<core::item::Item>, style: &mut Vec<String>) {
        let vars = self.vars();
        let mut theme = vars.into_iter().fold(
            format!(
                "@media (prefers-color-scheme: {}) {{:root{{",
                self.prefered_color_scheme
            ),
            |acc, var| format!("{}{}", acc, var,),
        );
        theme.push('}');
        theme.push_str(DEFAULT_STR);
        theme.push_str(self.style);
        theme.push('}');
        style.push(minimize(&theme).to_string());
    }
}

impl Theme {
    const fn new(
        prefered_color_scheme: &'static str,
        colors: &'static [Variable],
        style: &'static str,
    ) -> Theme {
        Theme {
            prefered_color_scheme,
            colors,
            style,
        }
    }

    fn vars(&self) -> Vec<String> {
        let mut vars: Vec<String> = self
            .colors
            .iter()
            .take(self.colors.len() - 2)
            .map(|c| format!("{}:{};", c.name, c.value))
            .collect::<Vec<String>>();
        vars.push(format!(
            "{}:{}",
            self.colors.last().unwrap().name,
            self.colors.last().unwrap().value
        ));

        vars
    }
}

#[derive(Debug)]
struct Variable {
    name: &'static str,
    value: &'static str,
}

impl Variable {
    const fn new(name: &'static str, value: &'static str) -> Variable {
        Variable { name, value }
    }
}

macro_rules! variables {
    ($(($name:literal, $value:literal)),*) => {
        &[$(Variable::new($name, $value)),*]
    };
}

fn minimize(css: &str) -> String {
    let css = css.replace(' ', "");
    css.replace('\n', "")
}

pub const LIGHT: Theme = Theme::new(
    "light",
    variables!(
        ("--bg-0", "#fff"),
        ("--bg-1", "#e5e5e5"),
        ("--bg-2", "#e5e5e5"),
        ("--bg-3", "#e5e5e5"),
        ("--text-0", "#000"),
        ("--text-1", "#808080"),
        ("--text-2", "#808080"),
        ("--text-3", "#808080")
    ),
    "",
);

pub const DARK: Theme = Theme::new(
    "dark",
    variables!(
        ("--bg-0", "#101010"),
        ("--bg-1", "#404040"),
        ("--bg-2", "#404040"),
        ("--bg-3", "#404040"),
        ("--text-0", "#f0f0f0"),
        ("--text-1", "#dcdcdc"),
        ("--text-2", "#dcdcdc"),
        ("--text-3", "#dcdcdc"),
        ("--color-0", "#ffa116"),
        ("--color-1", "#5cb85c"),
        ("--color-2", "#f0ad4e"),
        ("--color-3", "#d9534f")
    ),
    "#L { fill: #fff }",
);

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
