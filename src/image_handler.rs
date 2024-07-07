use indexmap::IndexMap;
use pad::PadStr;
use regex::Regex;

pub struct ImageHandler {
    image: String,

    primary_theme_color: String,
    secondary_theme_color: String,

    color_regexp: Regex,
}

impl ImageHandler {
    pub fn new(ascii_image: String) -> Self {
        Self {
            image: ascii_image,

            primary_theme_color: "\x1b[32m".to_string(),
            secondary_theme_color: "\x1b[33m".to_string(),

            color_regexp: Regex::new(r"\x1b\[[0-9;]*m").unwrap(),
        }
    }

    pub fn interpolate_image(&self, text: String) -> Vec<String> {
        let mut image_lines: Vec<String> =
            self.image.lines().map(|line| line.to_string()).collect();
        let mut info_lines: Vec<&str> = text.lines().collect();
        let mut max_image_width = 0;

        image_lines.iter().for_each(|line| {
            let line_stripped = self
                .color_regexp
                .replace_all(&line.clone(), "")
                .trim()
                .to_string();

            if line_stripped.len() > max_image_width {
                max_image_width = line_stripped.len()
            }
        });

        image_lines = image_lines
            .iter()
            .map(|line| {
                let line_stripped = self.color_regexp.replace_all(&line.clone(), "").to_string();
                let line_diff = line.len() - line_stripped.len();

                if line_stripped.len() < max_image_width {
                    return line.pad(
                        max_image_width + line_diff,
                        ' ',
                        pad::Alignment::Left,
                        false,
                    );
                }

                line.to_string()
            })
            .collect();

        match image_lines.len() {
            i if i < info_lines.len() => {
                let line_diff = info_lines.len() - image_lines.len();
                let mut image_width = 0;

                if let Some(last_line) = image_lines.last() {
                    image_width = last_line.len();
                }

                for _ in 0..(line_diff) {
                    image_lines.push(" ".repeat(image_width));
                }
            }
            i if i > info_lines.len() => {
                let line_diff = image_lines.len() - info_lines.len();

                info_lines.resize(info_lines.len() + line_diff, "");
            }
            _ => {}
        };

        image_lines
            .iter()
            .zip(info_lines.iter())
            .map(|(a, b)| format!("{}   {}", a, b))
            .collect::<Vec<String>>()
    }

    pub fn get_primary_theme_color(&mut self) -> &str {
        if self.primary_theme_color == "\x1b[32m" {
            self.extract_theme_colors();
        }

        &self.primary_theme_color
    }

    pub fn get_secondary_theme_color(&mut self) -> &str {
        if self.secondary_theme_color == "\x1b[33m" {
            self.extract_theme_colors();
        }

        &self.secondary_theme_color
    }

    fn extract_theme_colors(&mut self) {
        let mut occourances = IndexMap::new();

        self.color_regexp.find_iter(&self.image).for_each(|m| {
            let c = m.as_str();

            if occourances.contains_key(c) {
                occourances.insert(c, occourances.get(c).unwrap() + 1);
            } else {
                occourances.insert(c, 1);
            }
        });

        occourances.sort_by(|_, a, _, b| a.cmp(b));

        if let Some(first) = occourances.first() {
            if *first.0 != "\x1b[0m" {
                self.primary_theme_color = first.0.to_string();
            }

            if let Some(second) = occourances.get_index(1) {
                if *second.0 != "\x1b[0m" {
                    self.secondary_theme_color = second.0.to_string();
                } else {
                    self.secondary_theme_color = first.0.to_string();
                }
            }
        }
    }
}
