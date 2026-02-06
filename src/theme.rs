//! Theme definitions and configuration.
//!
//! This module contains the [`ThemeName`] enum representing all available themes,
//! and the [`Theme`] struct which provides a convenient wrapper for working with themes.

use crate::palette::ThemePalette;
use ratatui::style::Color;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enumeration of all available color themes.
///
/// Each variant represents a popular terminal/editor color scheme that has been
/// carefully adapted for terminal UI use. Themes are sourced from their official
/// specifications where available.
///
/// # Default Theme
///
/// The default theme is [`Dracula`](Self::Dracula), chosen for its excellent
/// readability and widespread popularity.
///
/// # Serialization
///
/// With the `serde` feature enabled (default), theme names serialize to kebab-case
/// strings for human-readable configuration files:
///
/// ```rust
/// use ratatui_themes::ThemeName;
///
/// // Serializes as: "tokyo-night"
/// let theme = ThemeName::TokyoNight;
/// ```
///
/// # Example
///
/// ```rust
/// use ratatui_themes::ThemeName;
///
/// // Get a specific theme
/// let theme = ThemeName::Nord;
/// let palette = theme.palette();
///
/// // Iterate over all themes
/// for theme in ThemeName::all() {
///     println!("{}: {:?}", theme.display_name(), theme.palette().accent);
/// }
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[non_exhaustive]
pub enum ThemeName {
    /// Dracula — dark purple aesthetic.
    ///
    /// A dark theme with a distinctive purple accent, excellent contrast,
    /// and vibrant colors. One of the most popular themes across editors.
    ///
    /// Source: <https://draculatheme.com>
    #[default]
    Dracula,

    /// One Dark Pro — Atom's iconic dark theme.
    ///
    /// The beloved theme from the Atom editor, featuring a balanced
    /// color palette with blue as the primary accent.
    ///
    /// Source: <https://github.com/Binaryify/OneDark-Pro>
    OneDarkPro,

    /// Nord — arctic, bluish color palette.
    ///
    /// An arctic, north-bluish color palette inspired by the beauty
    /// of the arctic. Clean, elegant, and easy on the eyes.
    ///
    /// Source: <https://www.nordtheme.com>
    Nord,

    /// Catppuccin Mocha — warm pastel dark theme.
    ///
    /// A soothing pastel theme with warm colors. Mocha is the darkest
    /// variant with a rich, cozy feel.
    ///
    /// Source: <https://catppuccin.com>
    CatppuccinMocha,

    /// Catppuccin Latte — warm pastel light theme.
    ///
    /// The light variant of Catppuccin with the same warm pastel
    /// aesthetic adapted for bright environments.
    ///
    /// Source: <https://catppuccin.com>
    CatppuccinLatte,

    /// Gruvbox Dark — retro groove colors.
    ///
    /// A retro groove color scheme with warm, earthy tones. Known for
    /// its excellent readability and nostalgic feel.
    ///
    /// Source: <https://github.com/morhetz/gruvbox>
    GruvboxDark,

    /// Gruvbox Light — retro groove, light variant.
    ///
    /// The light variant of Gruvbox, perfect for well-lit environments
    /// while maintaining the warm, retro aesthetic.
    ///
    /// Source: <https://github.com/morhetz/gruvbox>
    GruvboxLight,

    /// Tokyo Night — futuristic dark blue.
    ///
    /// A clean, dark theme that celebrates the lights of downtown Tokyo
    /// at night. Features a deep blue background with vibrant accents.
    ///
    /// Source: <https://github.com/enkia/tokyo-night-vscode-theme>
    TokyoNight,

    /// Solarized Dark — precision colors, dark.
    ///
    /// A precision-engineered color scheme with scientifically chosen
    /// colors for optimal readability. Dark variant.
    ///
    /// Source: <https://ethanschoonover.com/solarized>
    SolarizedDark,

    /// Solarized Light — precision colors, light.
    ///
    /// The light variant of Solarized, using the same precision-engineered
    /// colors adapted for bright environments.
    ///
    /// Source: <https://ethanschoonover.com/solarized>
    SolarizedLight,

    /// Monokai Pro — classic syntax highlighting colors.
    ///
    /// The professional evolution of the iconic Monokai color scheme.
    /// Features refined colors with excellent contrast.
    ///
    /// Source: <https://monokai.pro>
    MonokaiPro,

    /// Rosé Pine — all natural pine, faux fur, and soho vibes.
    ///
    /// A low-key colorscheme with subtle, muted colors and a
    /// warm, natural feel.
    ///
    /// Source: <https://rosepinetheme.com>
    RosePine,

    /// Kanagawa — dark theme inspired by Katsushika Hokusai.
    ///
    /// Inspired by the famous "The Great Wave off Kanagawa" painting.
    /// Features a Japanese aesthetic with deep, rich colors.
    ///
    /// Source: <https://github.com/rebelot/kanagawa.nvim>
    Kanagawa,

    /// Everforest — comfortable green forest theme.
    ///
    /// A green-based color scheme designed for a comfortable,
    /// relaxed coding experience with soft, natural colors.
    ///
    /// Source: <https://github.com/sainnhe/everforest>
    Everforest,

    /// Cyberpunk — neon-soaked futuristic theme.
    ///
    /// A high-contrast theme with neon colors inspired by the
    /// cyberpunk genre. Bold, vibrant, and futuristic.
    Cyberpunk,
}

impl ThemeName {
    /// Returns a slice containing all available theme names.
    ///
    /// Useful for building theme selection UIs or iterating over all themes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::ThemeName;
    ///
    /// // Build a theme selection menu
    /// for theme in ThemeName::all() {
    ///     println!("- {}", theme.display_name());
    /// }
    ///
    /// // Get the total number of themes
    /// assert_eq!(ThemeName::all().len(), 15);
    /// ```
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Dracula,
            Self::OneDarkPro,
            Self::Nord,
            Self::CatppuccinMocha,
            Self::CatppuccinLatte,
            Self::GruvboxDark,
            Self::GruvboxLight,
            Self::TokyoNight,
            Self::SolarizedDark,
            Self::SolarizedLight,
            Self::MonokaiPro,
            Self::RosePine,
            Self::Kanagawa,
            Self::Everforest,
            Self::Cyberpunk,
        ]
    }

    /// Returns the human-readable display name for the theme.
    ///
    /// This is useful for UI display where you want properly formatted
    /// theme names (e.g., "Tokyo Night" instead of `TokyoNight`).
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::ThemeName;
    ///
    /// assert_eq!(ThemeName::TokyoNight.display_name(), "Tokyo Night");
    /// assert_eq!(ThemeName::CatppuccinMocha.display_name(), "Catppuccin Mocha");
    /// ```
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Dracula => "Dracula",
            Self::OneDarkPro => "One Dark Pro",
            Self::Nord => "Nord",
            Self::CatppuccinMocha => "Catppuccin Mocha",
            Self::CatppuccinLatte => "Catppuccin Latte",
            Self::GruvboxDark => "Gruvbox Dark",
            Self::GruvboxLight => "Gruvbox Light",
            Self::TokyoNight => "Tokyo Night",
            Self::SolarizedDark => "Solarized Dark",
            Self::SolarizedLight => "Solarized Light",
            Self::MonokaiPro => "Monokai Pro",
            Self::RosePine => "Rosé Pine",
            Self::Kanagawa => "Kanagawa",
            Self::Everforest => "Everforest",
            Self::Cyberpunk => "Cyberpunk",
        }
    }

    /// Returns the kebab-case slug for the theme (used in config files).
    ///
    /// This matches the serde serialization format and is suitable for
    /// storing in configuration files.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::ThemeName;
    ///
    /// assert_eq!(ThemeName::TokyoNight.slug(), "tokyo-night");
    /// assert_eq!(ThemeName::CatppuccinMocha.slug(), "catppuccin-mocha");
    /// assert_eq!(ThemeName::Dracula.slug(), "dracula");
    /// ```
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Dracula => "dracula",
            Self::OneDarkPro => "one-dark-pro",
            Self::Nord => "nord",
            Self::CatppuccinMocha => "catppuccin-mocha",
            Self::CatppuccinLatte => "catppuccin-latte",
            Self::GruvboxDark => "gruvbox-dark",
            Self::GruvboxLight => "gruvbox-light",
            Self::TokyoNight => "tokyo-night",
            Self::SolarizedDark => "solarized-dark",
            Self::SolarizedLight => "solarized-light",
            Self::MonokaiPro => "monokai-pro",
            Self::RosePine => "rose-pine",
            Self::Kanagawa => "kanagawa",
            Self::Everforest => "everforest",
            Self::Cyberpunk => "cyberpunk",
        }
    }

    /// Returns the next theme in the list, wrapping around at the end.
    ///
    /// Useful for implementing theme cycling with a "next theme" button.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::ThemeName;
    ///
    /// let theme = ThemeName::Dracula;
    /// assert_eq!(theme.next(), ThemeName::OneDarkPro);
    ///
    /// // Wraps around at the end
    /// let last = ThemeName::Cyberpunk;
    /// assert_eq!(last.next(), ThemeName::Dracula);
    /// ```
    #[must_use]
    pub fn next(self) -> Self {
        let themes = Self::all();
        let current = themes.iter().position(|&t| t == self).unwrap_or(0);
        themes[(current + 1) % themes.len()]
    }

    /// Returns the previous theme in the list, wrapping around at the beginning.
    ///
    /// Useful for implementing theme cycling with a "previous theme" button.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::ThemeName;
    ///
    /// let theme = ThemeName::OneDarkPro;
    /// assert_eq!(theme.prev(), ThemeName::Dracula);
    ///
    /// // Wraps around at the beginning
    /// let first = ThemeName::Dracula;
    /// assert_eq!(first.prev(), ThemeName::Cyberpunk);
    /// ```
    #[must_use]
    pub fn prev(self) -> Self {
        let themes = Self::all();
        let current = themes.iter().position(|&t| t == self).unwrap_or(0);
        themes[(current + themes.len() - 1) % themes.len()]
    }

    /// Returns the color palette for this theme.
    ///
    /// The palette contains all the semantic colors you need to style your UI.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::ThemeName;
    /// use ratatui::style::Style;
    ///
    /// let palette = ThemeName::Nord.palette();
    ///
    /// // Use palette colors in your styles
    /// let style = Style::default()
    ///     .fg(palette.fg)
    ///     .bg(palette.bg);
    /// ```
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub const fn palette(self) -> ThemePalette {
        match self {
            // Dracula: https://draculatheme.com/contribute
            Self::Dracula => ThemePalette {
                accent: Color::Rgb(189, 147, 249),    // Purple
                secondary: Color::Rgb(255, 121, 198), // Pink
                bg: Color::Rgb(40, 42, 54),           // Background
                fg: Color::Rgb(248, 248, 242),        // Foreground
                muted: Color::Rgb(98, 114, 164),      // Comment
                selection: Color::Rgb(68, 71, 90),    // Selection
                error: Color::Rgb(255, 85, 85),       // Red
                warning: Color::Rgb(255, 184, 108),   // Orange
                success: Color::Rgb(80, 250, 123),    // Green
                info: Color::Rgb(139, 233, 253),      // Cyan
            },

            // One Dark Pro: https://github.com/Binaryify/OneDark-Pro
            Self::OneDarkPro => ThemePalette {
                accent: Color::Rgb(97, 175, 239),     // Blue
                secondary: Color::Rgb(198, 120, 221), // Magenta
                bg: Color::Rgb(40, 44, 52),           // Background
                fg: Color::Rgb(171, 178, 191),        // Foreground
                muted: Color::Rgb(92, 99, 112),       // Comment
                selection: Color::Rgb(62, 68, 81),    // Selection
                error: Color::Rgb(224, 108, 117),     // Red
                warning: Color::Rgb(229, 192, 123),   // Yellow
                success: Color::Rgb(152, 195, 121),   // Green
                info: Color::Rgb(86, 182, 194),       // Cyan
            },

            // Nord: https://www.nordtheme.com
            Self::Nord => ThemePalette {
                accent: Color::Rgb(136, 192, 208),    // Frost blue
                secondary: Color::Rgb(129, 161, 193), // Frost darker
                bg: Color::Rgb(46, 52, 64),           // Polar Night
                fg: Color::Rgb(236, 239, 244),        // Snow Storm
                muted: Color::Rgb(76, 86, 106),       // Polar Night lighter
                selection: Color::Rgb(67, 76, 94),    // Selection
                error: Color::Rgb(191, 97, 106),      // Aurora red
                warning: Color::Rgb(235, 203, 139),   // Aurora yellow
                success: Color::Rgb(163, 190, 140),   // Aurora green
                info: Color::Rgb(94, 129, 172),       // Frost
            },

            // Catppuccin Mocha: https://catppuccin.com
            Self::CatppuccinMocha => ThemePalette {
                accent: Color::Rgb(137, 180, 250),    // Blue
                secondary: Color::Rgb(245, 194, 231), // Pink
                bg: Color::Rgb(30, 30, 46),           // Base
                fg: Color::Rgb(205, 214, 244),        // Text
                muted: Color::Rgb(108, 112, 134),     // Overlay0
                selection: Color::Rgb(49, 50, 68),    // Surface0
                error: Color::Rgb(243, 139, 168),     // Red
                warning: Color::Rgb(249, 226, 175),   // Yellow
                success: Color::Rgb(166, 227, 161),   // Green
                info: Color::Rgb(148, 226, 213),      // Teal
            },

            // Catppuccin Latte (light theme)
            Self::CatppuccinLatte => ThemePalette {
                accent: Color::Rgb(30, 102, 245),     // Blue
                secondary: Color::Rgb(234, 118, 203), // Pink
                bg: Color::Rgb(239, 241, 245),        // Base
                fg: Color::Rgb(76, 79, 105),          // Text
                muted: Color::Rgb(140, 143, 161),     // Overlay0
                selection: Color::Rgb(204, 208, 218), // Surface0
                error: Color::Rgb(210, 15, 57),       // Red
                warning: Color::Rgb(223, 142, 29),    // Yellow
                success: Color::Rgb(64, 160, 43),     // Green
                info: Color::Rgb(23, 146, 153),       // Teal
            },

            // Gruvbox Dark: https://github.com/morhetz/gruvbox
            Self::GruvboxDark => ThemePalette {
                accent: Color::Rgb(250, 189, 47),     // Yellow
                secondary: Color::Rgb(211, 134, 155), // Purple
                bg: Color::Rgb(40, 40, 40),           // bg0
                fg: Color::Rgb(235, 219, 178),        // fg
                muted: Color::Rgb(146, 131, 116),     // gray
                selection: Color::Rgb(80, 73, 69),    // bg2
                error: Color::Rgb(251, 73, 52),       // red
                warning: Color::Rgb(254, 128, 25),    // orange
                success: Color::Rgb(184, 187, 38),    // green
                info: Color::Rgb(131, 165, 152),      // aqua
            },

            // Gruvbox Light
            Self::GruvboxLight => ThemePalette {
                accent: Color::Rgb(181, 118, 20),     // Yellow
                secondary: Color::Rgb(143, 63, 113),  // Purple
                bg: Color::Rgb(251, 241, 199),        // bg0
                fg: Color::Rgb(60, 56, 54),           // fg
                muted: Color::Rgb(146, 131, 116),     // gray
                selection: Color::Rgb(213, 196, 161), // bg2
                error: Color::Rgb(157, 0, 6),         // red
                warning: Color::Rgb(175, 58, 3),      // orange
                success: Color::Rgb(121, 116, 14),    // green
                info: Color::Rgb(66, 123, 88),        // aqua
            },

            // Tokyo Night: https://github.com/enkia/tokyo-night-vscode-theme
            Self::TokyoNight => ThemePalette {
                accent: Color::Rgb(122, 162, 247),    // Blue
                secondary: Color::Rgb(187, 154, 247), // Magenta
                bg: Color::Rgb(26, 27, 38),           // Background
                fg: Color::Rgb(192, 202, 245),        // Foreground
                muted: Color::Rgb(86, 95, 137),       // Comment
                selection: Color::Rgb(41, 46, 66),    // Selection
                error: Color::Rgb(247, 118, 142),     // Red
                warning: Color::Rgb(224, 175, 104),   // Yellow
                success: Color::Rgb(158, 206, 106),   // Green
                info: Color::Rgb(125, 207, 255),      // Cyan
            },

            // Solarized Dark: https://ethanschoonover.com/solarized/
            Self::SolarizedDark => ThemePalette {
                accent: Color::Rgb(38, 139, 210),     // Blue
                secondary: Color::Rgb(108, 113, 196), // Violet
                bg: Color::Rgb(0, 43, 54),            // base03
                fg: Color::Rgb(131, 148, 150),        // base0
                muted: Color::Rgb(88, 110, 117),      // base01
                selection: Color::Rgb(7, 54, 66),     // base02
                error: Color::Rgb(220, 50, 47),       // red
                warning: Color::Rgb(181, 137, 0),     // yellow
                success: Color::Rgb(133, 153, 0),     // green
                info: Color::Rgb(42, 161, 152),       // cyan
            },

            // Solarized Light
            Self::SolarizedLight => ThemePalette {
                accent: Color::Rgb(38, 139, 210),     // Blue
                secondary: Color::Rgb(108, 113, 196), // Violet
                bg: Color::Rgb(253, 246, 227),        // base3
                fg: Color::Rgb(101, 123, 131),        // base00
                muted: Color::Rgb(147, 161, 161),     // base1
                selection: Color::Rgb(238, 232, 213), // base2
                error: Color::Rgb(220, 50, 47),       // red
                warning: Color::Rgb(181, 137, 0),     // yellow
                success: Color::Rgb(133, 153, 0),     // green
                info: Color::Rgb(42, 161, 152),       // cyan
            },

            // Monokai Pro: https://monokai.pro
            Self::MonokaiPro => ThemePalette {
                accent: Color::Rgb(255, 216, 102),    // Yellow
                secondary: Color::Rgb(171, 157, 242), // Purple
                bg: Color::Rgb(45, 42, 46),           // Background
                fg: Color::Rgb(252, 252, 250),        // Foreground
                muted: Color::Rgb(114, 113, 105),     // Comment
                selection: Color::Rgb(81, 80, 79),    // Selection
                error: Color::Rgb(255, 97, 136),      // Red
                warning: Color::Rgb(252, 152, 103),   // Orange
                success: Color::Rgb(169, 220, 118),   // Green
                info: Color::Rgb(120, 220, 232),      // Cyan
            },

            // Rosé Pine: https://rosepinetheme.com
            Self::RosePine => ThemePalette {
                accent: Color::Rgb(235, 188, 186),    // Rose
                secondary: Color::Rgb(196, 167, 231), // Iris
                bg: Color::Rgb(25, 23, 36),           // Base
                fg: Color::Rgb(224, 222, 244),        // Text
                muted: Color::Rgb(110, 106, 134),     // Muted
                selection: Color::Rgb(38, 35, 58),    // Overlay
                error: Color::Rgb(235, 111, 146),     // Love
                warning: Color::Rgb(246, 193, 119),   // Gold
                success: Color::Rgb(156, 207, 216),   // Foam
                info: Color::Rgb(49, 116, 143),       // Pine
            },

            // Kanagawa: https://github.com/rebelot/kanagawa.nvim
            Self::Kanagawa => ThemePalette {
                accent: Color::Rgb(127, 180, 202),    // Crystal blue
                secondary: Color::Rgb(149, 127, 184), // Oniviolet
                bg: Color::Rgb(31, 31, 40),           // Sumi ink
                fg: Color::Rgb(220, 215, 186),        // Fuji white
                muted: Color::Rgb(84, 84, 109),       // Katana gray
                selection: Color::Rgb(54, 54, 70),    // Wave blue
                error: Color::Rgb(195, 64, 67),       // Samurai red
                warning: Color::Rgb(255, 169, 107),   // Ronin yellow
                success: Color::Rgb(118, 148, 106),   // Spring green
                info: Color::Rgb(126, 156, 216),      // Spring blue
            },

            // Everforest: https://github.com/sainnhe/everforest
            Self::Everforest => ThemePalette {
                accent: Color::Rgb(131, 193, 120),    // Green
                secondary: Color::Rgb(214, 153, 182), // Purple
                bg: Color::Rgb(47, 53, 55),           // bg0
                fg: Color::Rgb(211, 198, 170),        // fg
                muted: Color::Rgb(133, 146, 137),     // gray
                selection: Color::Rgb(68, 78, 79),    // bg2
                error: Color::Rgb(230, 126, 128),     // red
                warning: Color::Rgb(219, 188, 127),   // yellow
                success: Color::Rgb(167, 192, 128),   // green
                info: Color::Rgb(124, 195, 191),      // aqua
            },

            // Cyberpunk: custom neon theme
            Self::Cyberpunk => ThemePalette {
                accent: Color::Rgb(0, 255, 255),    // Neon cyan
                secondary: Color::Rgb(255, 0, 255), // Neon magenta
                bg: Color::Rgb(13, 2, 33),          // Dark purple
                fg: Color::Rgb(240, 240, 240),      // Bright white
                muted: Color::Rgb(100, 100, 140),   // Muted purple
                selection: Color::Rgb(40, 20, 80),  // Purple selection
                error: Color::Rgb(255, 0, 60),      // Neon red
                warning: Color::Rgb(255, 230, 0),   // Neon yellow
                success: Color::Rgb(0, 255, 100),   // Neon green
                info: Color::Rgb(0, 180, 255),      // Neon blue
            },
        }
    }
}

impl std::fmt::Display for ThemeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl std::str::FromStr for ThemeName {
    type Err = String;

    /// Parse a theme name from a string.
    ///
    /// Accepts kebab-case (as used in serde/config files), PascalCase,
    /// or lowercase names.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::ThemeName;
    ///
    /// // Kebab-case (config file format)
    /// assert_eq!("tokyo-night".parse::<ThemeName>().unwrap(), ThemeName::TokyoNight);
    ///
    /// // Display name style
    /// assert_eq!("Tokyo Night".parse::<ThemeName>().unwrap(), ThemeName::TokyoNight);
    ///
    /// // Lowercase
    /// assert_eq!("dracula".parse::<ThemeName>().unwrap(), ThemeName::Dracula);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Normalize: lowercase and remove spaces/hyphens/underscores
        let normalized: String = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        match normalized.as_str() {
            "dracula" => Ok(Self::Dracula),
            "onedarkpro" | "onedark" => Ok(Self::OneDarkPro),
            "nord" => Ok(Self::Nord),
            "catppuccinmocha" | "mocha" => Ok(Self::CatppuccinMocha),
            "catppuccinlatte" | "latte" => Ok(Self::CatppuccinLatte),
            "gruvboxdark" | "gruvbox" => Ok(Self::GruvboxDark),
            "gruvboxlight" => Ok(Self::GruvboxLight),
            "tokyonight" | "tokyo" => Ok(Self::TokyoNight),
            "solarizeddark" | "solarized" => Ok(Self::SolarizedDark),
            "solarizedlight" => Ok(Self::SolarizedLight),
            "monokaipro" | "monokai" => Ok(Self::MonokaiPro),
            "rosepine" | "rose" => Ok(Self::RosePine),
            "kanagawa" => Ok(Self::Kanagawa),
            "everforest" => Ok(Self::Everforest),
            "cyberpunk" => Ok(Self::Cyberpunk),
            _ => Err(format!("Unknown theme: {s}")),
        }
    }
}

/// A theme configuration wrapper providing convenient access to theme colors.
///
/// This struct wraps a [`ThemeName`] and provides methods for accessing
/// the theme's color palette and metadata. It's useful when you want to
/// store a theme reference that can be easily modified.
///
/// # Example
///
/// ```rust
/// use ratatui_themes::Theme;
/// use ratatui_themes::ThemeName;
///
/// // Create a theme
/// let mut theme = Theme::new(ThemeName::Nord);
///
/// // Access the palette
/// let accent = theme.palette().accent;
///
/// // Cycle to the next theme
/// theme.next();
/// assert_eq!(theme.name, ThemeName::CatppuccinMocha);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Theme {
    /// The selected theme name.
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: ThemeName,
}

impl Theme {
    /// Create a new theme with the given name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::{Theme, ThemeName};
    ///
    /// let theme = Theme::new(ThemeName::Dracula);
    /// ```
    #[must_use]
    pub const fn new(name: ThemeName) -> Self {
        Self { name }
    }

    /// Returns the color palette for the current theme.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::{Theme, ThemeName};
    /// use ratatui::style::Style;
    ///
    /// let theme = Theme::new(ThemeName::TokyoNight);
    /// let palette = theme.palette();
    ///
    /// let style = Style::default()
    ///     .fg(palette.fg)
    ///     .bg(palette.bg);
    /// ```
    #[must_use]
    pub const fn palette(&self) -> ThemePalette {
        self.name.palette()
    }

    /// Check if this is a light theme.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::{Theme, ThemeName};
    ///
    /// let theme = Theme::new(ThemeName::CatppuccinLatte);
    /// assert!(theme.is_light());
    /// ```
    #[must_use]
    pub fn is_light(&self) -> bool {
        self.palette().is_light()
    }

    /// Check if this is a dark theme.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::{Theme, ThemeName};
    ///
    /// let theme = Theme::new(ThemeName::Dracula);
    /// assert!(theme.is_dark());
    /// ```
    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.palette().is_dark()
    }

    /// Cycle to the next theme in the list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::{Theme, ThemeName};
    ///
    /// let mut theme = Theme::new(ThemeName::Dracula);
    /// theme.next();
    /// assert_eq!(theme.name, ThemeName::OneDarkPro);
    /// ```
    pub fn next(&mut self) {
        self.name = self.name.next();
    }

    /// Cycle to the previous theme in the list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_themes::{Theme, ThemeName};
    ///
    /// let mut theme = Theme::new(ThemeName::OneDarkPro);
    /// theme.prev();
    /// assert_eq!(theme.name, ThemeName::Dracula);
    /// ```
    pub fn prev(&mut self) {
        self.name = self.name.prev();
    }
}

impl From<ThemeName> for Theme {
    fn from(name: ThemeName) -> Self {
        Self::new(name)
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_themes_have_palettes() {
        for theme in ThemeName::all() {
            let palette = theme.palette();
            // Just verify it doesn't panic and colors are distinct
            assert_ne!(palette.fg, palette.bg);
        }
    }

    #[test]
    fn test_theme_cycling() {
        let mut theme = ThemeName::Dracula;
        let original = theme;

        // Cycle through all themes
        for _ in 0..ThemeName::all().len() {
            theme = theme.next();
        }

        // Should be back to original
        assert_eq!(theme, original);
    }

    #[test]
    fn test_theme_cycling_backward() {
        let mut theme = ThemeName::Dracula;
        let original = theme;

        // Cycle backward through all themes
        for _ in 0..ThemeName::all().len() {
            theme = theme.prev();
        }

        // Should be back to original
        assert_eq!(theme, original);
    }

    #[test]
    fn test_light_dark_detection() {
        // Light themes
        assert!(ThemeName::CatppuccinLatte.palette().is_light());
        assert!(ThemeName::GruvboxLight.palette().is_light());
        assert!(ThemeName::SolarizedLight.palette().is_light());

        // Dark themes
        assert!(ThemeName::Dracula.palette().is_dark());
        assert!(ThemeName::Nord.palette().is_dark());
        assert!(ThemeName::TokyoNight.palette().is_dark());
    }

    #[test]
    fn test_display_name() {
        assert_eq!(ThemeName::Dracula.display_name(), "Dracula");
        assert_eq!(ThemeName::TokyoNight.display_name(), "Tokyo Night");
        assert_eq!(
            ThemeName::CatppuccinMocha.display_name(),
            "Catppuccin Mocha"
        );
    }

    #[test]
    fn test_theme_display_trait() {
        assert_eq!(format!("{}", ThemeName::Dracula), "Dracula");
        assert_eq!(format!("{}", ThemeName::TokyoNight), "Tokyo Night");
    }

    #[test]
    fn test_theme_wrapper() {
        let mut theme = Theme::new(ThemeName::Dracula);
        assert_eq!(theme.name, ThemeName::Dracula);
        assert!(theme.is_dark());

        theme.next();
        assert_eq!(theme.name, ThemeName::OneDarkPro);

        theme.prev();
        assert_eq!(theme.name, ThemeName::Dracula);
    }

    #[test]
    fn test_theme_from_name() {
        let theme: Theme = ThemeName::Nord.into();
        assert_eq!(theme.name, ThemeName::Nord);
    }

    #[test]
    fn test_default_theme() {
        assert_eq!(ThemeName::default(), ThemeName::Dracula);
        assert_eq!(Theme::default().name, ThemeName::Dracula);
    }

    #[test]
    fn test_theme_count() {
        assert_eq!(ThemeName::all().len(), 15);
    }
}
