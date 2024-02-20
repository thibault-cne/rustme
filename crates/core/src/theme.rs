#[derive(Debug, Clone)]
pub struct Theme {
    pub prefered_color_scheme: &'static str,
    pub colors: &'static [Variable],
    pub style: &'static str,
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

    pub fn set_light(&mut self) {
        self.prefered_color_scheme = "light";
    }

    pub fn set_dark(&mut self) {
        self.prefered_color_scheme = "dark";
    }

    pub fn format_vars(&self) -> Vec<String> {
        let mut vars: Vec<String> = self
            .colors
            .iter()
            .take(self.colors.len() - 1)
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
pub struct Variable {
    name: &'static str,
    value: &'static str,
}

impl Variable {
    const fn new(name: &'static str, value: &'static str) -> Variable {
        Variable { name, value }
    }
}

pub const LIGHT: Theme = Theme::new(
    "light",
    macros::variables! {
        "--bg-0": "#fff",
        "--bg-1": "#e5e5e5",
        "--bg-2": "#e5e5e5",
        "--bg-3": "#e5e5e5",
        "--text-0": "#000",
        "--text-1": "#808080",
        "--text-2": "#808080",
        "--text-3": "#808080"
    },
    "",
);

pub const DARK: Theme = Theme::new(
    "dark",
    macros::variables! {
        "--bg-0": "#101010",
        "--bg-1": "#404040",
        "--bg-2": "#404040",
        "--bg-3": "#404040",
        "--text-0": "#f0f0f0",
        "--text-1": "#dcdcdc",
        "--text-2": "#dcdcdc",
        "--text-3": "#dcdcdc",
        "--color-0": "#ffa116",
        "--color-1": "#5cb85c",
        "--color-2": "#f0ad4e",
        "--color-3": "#d9534f"
    },
    "#L { fill: #fff }",
);

pub const FERRARI: Theme = Theme::new(
    "ferrari",
    macros::variables! {
        "--bg-0": "#a6051a",
        "--bg-1": "#ed1c24",
        "--bg-2": "#ed1c24",
        "--bg-3": "#ed1c24",
        "--text-0": "#fff200",
        "--text-1": "#ffffff",
        "--text-2": "#ffffff",
        "--text-3": "#ffffff",
        "--color-0": "#fff200",
        "--color-1": "#009a4e",
        "--color-2": "#ffffff",
        "--color-3": "#111111"
    },
    "#L { fill: #ffffff }",
);

macros::impl_themes! {
    "light" => LIGHT;
    "dark" => DARK;
    "ferrari" => FERRARI;
}

mod macros {
    macro_rules! variables {
        {$($name:literal: $value:literal),*} => {
            &[$(Variable::new($name, $value)),*]
        };
    }

    macro_rules! impl_themes {
        {$($name:literal => $theme:ident);*;} => {
            impl From<&str> for Theme {
                fn from(s: &str) -> Theme {
                    match s {
                        $($name => $theme,)*
                        _ => LIGHT,
                    }
                }
            }

            impl From<String> for Theme {
                fn from(s: String) -> Theme {
                    s.as_str().into()
                }
            }
        };
}

    pub(super) use impl_themes;
    pub(super) use variables;
}
