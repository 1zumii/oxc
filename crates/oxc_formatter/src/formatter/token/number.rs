// use biome_rowan::{Language, SyntaxToken};
// use biome_string_case::StrLikeExtension;
use std::{borrow::Cow, num::NonZeroUsize};

use super::super::SyntaxToken;

#[derive(Debug, Default, Clone, Copy)]
pub struct NumberFormatOptions {
    /// Controls how numbers with trailing decimal zeroes are formatted.
    ///
    /// Prettier behaves differently when printing numbers like `x.00000` in different languages:
    /// - In JavaScript: `x.00000` is printed as `x.0`
    /// - In CSS: `x.00000` is printed as `x`
    keep_one_trailing_decimal_zero: bool,
}

impl NumberFormatOptions {
    pub fn keep_one_trailing_decimal_zero(self) -> Self {
        Self { keep_one_trailing_decimal_zero: true }
    }
}

pub fn format_number_token(
    token: &SyntaxToken,
    options: NumberFormatOptions,
) -> CleanedNumberLiteralText
where
{
    CleanedNumberLiteralText { token, options }
}

pub struct CleanedNumberLiteralText<'token> {
    token: &'token SyntaxToken,
    options: NumberFormatOptions,
}

// impl<L, C> Format<C> for CleanedNumberLiteralText<'_, L>
// where
// L: Language + 'static,
// C: CstFormatContext<Language = L>,
// {
// fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
// format_replaced(
// self.token,
// &syntax_token_cow_slice(
// format_trimmed_number(self.token.text_trimmed(), self.options),
// self.token,
// self.token.text_trimmed_range().start(),
// ),
// )
// .fmt(f)
// }
// }

enum FormatNumberLiteralState {
    IntegerPart,
    DecimalPart(FormatNumberLiteralDecimalPart),
    Exponent(FormatNumberLiteralExponent),
}

struct FormatNumberLiteralDecimalPart {
    dot_index: usize,
    last_non_zero_index: Option<NonZeroUsize>,
}
struct FormatNumberLiteralExponent {
    e_index: usize,
    is_negative: bool,
    first_digit_index: Option<NonZeroUsize>,
    first_non_zero_index: Option<NonZeroUsize>,
}
// Regex-free version of https://github.com/prettier/prettier/blob/ca246afacee8e6d5db508dae01730c9523bbff1d/src/common/util.js#L341-L356
fn format_trimmed_number(text: &str, options: NumberFormatOptions) -> Cow<str> {
    todo!()
    // use FormatNumberLiteralState::*;

    // let text = text.to_ascii_lowercase_cow();
    // let mut copied_or_ignored_chars = 0usize;
    // let mut iter = text.bytes().enumerate();
    // let mut curr = iter.next();
    // let mut state = IntegerPart;

    // // Will be filled only if and when the first place that needs reformatting is detected.
    // let mut cleaned_text = String::new();

    // // Look at only the start of the text, ignore any sign, and make sure numbers always start with a digit. Add 0 if missing.
    // if let Some((_, b'+' | b'-')) = curr {
    // curr = iter.next();
    // }
    // if let Some((curr_index, b'.')) = curr {
    // cleaned_text.push_str(&text[copied_or_ignored_chars..curr_index]);
    // copied_or_ignored_chars = curr_index;
    // cleaned_text.push('0');
    // }

    // // Loop over the rest of the text, applying the remaining rules.
    // loop {
    // // We use a None pseudo-char at the end of the string to simplify the match cases that follow
    // let curr_or_none_terminator_char = match curr {
    // Some((curr_index, curr_char)) => (curr_index, Some(curr_char)),
    // None => (text.len(), None),
    // };
    // // Look for termination of the decimal part or exponent and see if we need to print it differently.
    // match (&state, curr_or_none_terminator_char) {
    // (
    // DecimalPart(FormatNumberLiteralDecimalPart {
    // dot_index,
    // last_non_zero_index: None,
    // }),
    // (curr_index, Some(b'e') | None),
    // ) => {
    // // The decimal part equals zero, ignore it completely. However when the `keep_one_trailing_decimal_zero` option is enabled, print `.0` unless there was *only* a trailing dot.
    // if curr_index > dot_index + 1 && options.keep_one_trailing_decimal_zero {
    // cleaned_text.push_str(&text[copied_or_ignored_chars..=*dot_index]);
    // cleaned_text.push('0');
    // } else {
    // cleaned_text.push_str(&text[copied_or_ignored_chars..*dot_index]);
    // }
    // copied_or_ignored_chars = curr_index;
    // }
    // (
    // DecimalPart(FormatNumberLiteralDecimalPart {
    // last_non_zero_index: Some(last_non_zero_index),
    // ..
    // }),
    // (curr_index, Some(b'e') | None),
    // ) if last_non_zero_index.get() < curr_index - 1 => {
    // // The decimal part ends with at least one zero, ignore them but copy the part from the dot until the last non-zero.
    // cleaned_text.push_str(&text[copied_or_ignored_chars..=last_non_zero_index.get()]);
    // copied_or_ignored_chars = curr_index;
    // }
    // (
    // Exponent(FormatNumberLiteralExponent {
    // e_index, first_non_zero_index: None, ..
    // }),
    // (curr_index, None),
    // ) => {
    // // The exponent equals zero, ignore it completely.
    // cleaned_text.push_str(&text[copied_or_ignored_chars..*e_index]);
    // copied_or_ignored_chars = curr_index;
    // }
    // (
    // Exponent(FormatNumberLiteralExponent {
    // e_index,
    // is_negative,
    // first_digit_index: Some(first_digit_index),
    // first_non_zero_index: Some(first_non_zero_index),
    // }),
    // (curr_index, None),
    // ) if (first_digit_index.get() > e_index + 1 && !is_negative)
    // || (first_non_zero_index.get() > first_digit_index.get()) =>
    // {
    // // The exponent begins with a plus or at least one zero, ignore them but copy the part from the first non-zero until the end.
    // cleaned_text.push_str(&text[copied_or_ignored_chars..=*e_index]);
    // if *is_negative {
    // cleaned_text.push('-');
    // }
    // cleaned_text.push_str(&text[first_non_zero_index.get()..curr_index]);
    // copied_or_ignored_chars = curr_index;
    // }
    // _ => {}
    // }

    // // Update state after the current char
    // match (&state, curr) {
    // // Cases entering or remaining in decimal part
    // (_, Some((curr_index, b'.'))) => {
    // state = DecimalPart(FormatNumberLiteralDecimalPart {
    // dot_index: curr_index,
    // last_non_zero_index: None,
    // });
    // }
    // (DecimalPart(decimal_part), Some((curr_index, b'1'..=b'9'))) => {
    // state = DecimalPart(FormatNumberLiteralDecimalPart {
    // last_non_zero_index: Some(unsafe {
    // // We've already entered InDecimalPart, so curr_index must be >0
    // NonZeroUsize::new_unchecked(curr_index)
    // }),
    // ..*decimal_part
    // });
    // }
    // // Cases entering or remaining in exponent
    // (_, Some((curr_index, b'e'))) => {
    // state = Exponent(FormatNumberLiteralExponent {
    // e_index: curr_index,
    // is_negative: false,
    // first_digit_index: None,
    // first_non_zero_index: None,
    // });
    // }
    // (Exponent(exponent), Some((_, b'-'))) => {
    // state = Exponent(FormatNumberLiteralExponent { is_negative: true, ..*exponent });
    // }
    // (
    // Exponent(exponent @ FormatNumberLiteralExponent { first_digit_index: None, .. }),
    // Some((curr_index, curr_char @ b'0'..=b'9')),
    // ) => {
    // state = Exponent(FormatNumberLiteralExponent {
    // first_digit_index: Some(unsafe {
    // // We've already entered InExponent, so curr_index must be >0
    // NonZeroUsize::new_unchecked(curr_index)
    // }),
    // first_non_zero_index: if curr_char != b'0' {
    // Some(unsafe {
    // // We've already entered InExponent, so curr_index must be >0
    // NonZeroUsize::new_unchecked(curr_index)
    // })
    // } else {
    // None
    // },
    // ..*exponent
    // });
    // }
    // (
    // Exponent(exponent @ FormatNumberLiteralExponent { first_non_zero_index: None, .. }),
    // Some((curr_index, b'1'..=b'9')),
    // ) => {
    // state = Exponent(FormatNumberLiteralExponent {
    // first_non_zero_index: Some(unsafe { NonZeroUsize::new_unchecked(curr_index) }),
    // ..*exponent
    // });
    // }
    // _ => {}
    // }

    // // Repeat or exit
    // match curr {
    // None | Some((_, b'x') [> hex bailout <]) => break,
    // Some(_) => curr = iter.next(),
    // }
    // }

    // if cleaned_text.is_empty() {
    // text
    // } else {
    // // Append any unconsidered text
    // cleaned_text.push_str(&text[copied_or_ignored_chars..]);
    // Cow::Owned(cleaned_text)
    // }
}
