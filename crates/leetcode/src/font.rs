use core::{error::Result, Extension};

macro_rules! fonts {
    {$($const:ident),*} => {
        pub use core::font::{$(
            $const
        ),*};
    };
}

fonts! {BALOO_2, FORMULA_1}

impl Extension<super::Generator> for core::font::Font {
    async fn extend(
        &self,
        _: &mut super::Generator,
        _: &mut Vec<core::item::Item>,
        style: &mut Vec<String>,
    ) -> Result<()> {
        let font = self.fetch().await?;
        style.push(format!(
            r##"@font-face{{font-family:"{}";src:url("{}") format("woff2")}}"##,
            font.name, font.base64
        ));
        let font_family = match font.name.as_str() {
            "sans" | "serif" | "monospace" => font.name.clone(),
            _ => format!(r#""{}""#, font.name),
        };
        style.push(format!("*{{font-family:{}}}", font_family));
        Ok(())
    }
}
