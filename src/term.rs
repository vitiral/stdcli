/* Copyright (C) 2017  Garrett Berg <@vitiral, vitiral@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the Lesser GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the Lesser GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */

//! Exports a unified terminal interface.
//!
//! There are several small crates in rust which are intended to be
//! used together to create better terminal interfaces. This module
//! combines their types and exports them together.
//!
//! See the documents for each of the respective crates.

pub use console;
pub use dialoguer;
pub use indicatif;
pub use tabwriter;

// Structs / Enums
pub use console::{
    // structs
    AnsiCodeIterator,   // An iterator over ansi codes in a string.
    Emoji,              // "Intelligent" emoji formatter.
    Style,              // A stored style that can be applied.
    StyledObject,       // A formatting wrapper that can be styled for a terminal.
    Term,               // Abstraction around a terminal.

    // enums
    Alignment,          // Defines the alignment for padding operations.
    Attribute,          // A terminal style attribute.
    Color,              // A terminal color.
    Key,                // Key mapping
};

pub use dialoguer::{
    Checkboxes,         // Renders a multi select checkbox menu.
    Confirmation,       // Renders a simple confirmation prompt.
    Editor,             // Launches the default editor edit a string.
    Input,              // Renders a simple input prompt.
    PasswordInput,      // Renders a password input prompt.
    Select,             // Renders a selection menu.
};

pub use indicatif::{
    FormattedDuration,  // Wraps an std duration for human basic formatting.
    HumanBytes,         // Formats bytes for human readability
    HumanDuration,      // Wraps an std duration for human readable formatting.
    MultiProgress,      // Manages multiple progress bars from different threads.
    ProgressBar,        // A progress bar or spinner.
    ProgressBarIter,
    ProgressDrawTarget, // Target for draw operations
    ProgressStyle,      // Controls the rendering style of progress bars.
};

use tabwriter::TabWriter;

// Functions
pub use console::{
    colors_enabled,     // Returns true if colors should be enabled.
    measure_text_width, // Measure the width of a string in terminal characters.
    pad_str,            // Pads a string to fill a certain number of characters.
    set_colors_enabled, // Forces colorization on or off.
    strip_ansi_codes,   // Helper function to strip ansi codes.
    style,              // Wraps an object for formatting for styling.
    truncate_str,       // Truncates a string to a certain number of characters.
    user_attended,      // A fast way to check if the application has a user attended.
};
