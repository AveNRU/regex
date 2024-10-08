/*!
Это дополнение предоставляет надежный оценщик регулярных выражений.

Это дополнение определяет два основных вида:

* [`Ast`](ast::Ast) абстрактные правила написания регулярного выражения. Абстрактные правила написания 
 соответствует структурированному представлению конкретного правил написания  регулярного выражения,
  где определенные правила написания — это сама строка образца (например, `foo(bar)+`). 
   При наличии некоторого абстрактного правил написания  его можно преобразовать обратно в исходный 
   определенные правила написания (по разделу некоторых подробностей, таких как пробелы). 
   В первом приближении абстрактные правила написания сложен и труден для анализа.
* [`Hir`](hir::Hir) является промежуточным представлением высокого уровня 
(сокращенно «HIR» или «IR высокого уровня») регулярного выражения. 
Оно соответствует промежуточному состоянию регулярного выражения, 
которое находится между абстрактными правилами написания  и собранными 
кодами операций низкого уровня, которые в конечном итоге отвечают за 
выполнение поиска регулярного выражения. При наличии некоторого IR 
высокого уровня невозможно создать исходные определенные правила написания 
(хотя возможно создать сопоставимые определенные правила написания , но он, 
скорее всего, вряд ли будет напоминать исходный образец). 
В первом приближении IR высокого уровня прост и легко поддается анализу.

Эти два вида поставляются с процедурами преобразования:

* [`ast::parse::Parser`] преобразует определенные правила написания (`&str`) в
[`Ast`](ast::Ast).
* A [`hir::translate::Translator`] преобразует [`Ast`](ast::Ast) в
[`Hir`](hir::Hir).

Для удобства, вышеприведенные две процедуры преобразования объединены в одну 
через вид верхнего уровня [`Parser`] type. Этот `Parser` сначала преобразует ваш образец в `Ast`, 
а затем преобразует `Ast` в `Hir`. Это также представлено как
[`parse`] свободная функция верхнего уровня.


# Пример

В этом примере показано, как преобразовать строку образца в ее HIR:

```
use regex_syntax::{hir::Hir, parse};

let hir = parse("a|b")?;
assert_eq!(hir, Hir::alternation(vec![
    Hir::literal("a".as_bytes()),
    Hir::literal("b".as_bytes()),
]));
# Ok::<(), Box<dyn std::error::Error>>(())
```


# Поддерживаются определенные правила написания 

Определенные правила написания представлен как часть публичного API
[`regex` crate](https://docs.rs/regex/%2A/regex/#syntax).


# Безопасность на входе

Ключевой особенностью этой библиотеки является то, что ее можно безопасно использовать 
с конечным пользователем, который обращается к вводу. Это играет важную роль во 
внутреннем исполнении. В частности:

1. Обработчики предоставляют `nest_limit` возможность, которая позволяет вызывающим отслеживать, 
насколько глубоко вложенным может быть регулярное выражение. Это позволяет проводить 
анализ случаев с использованием рекурсии `Ast` или `Hir` без беспокойства о переполнении обоймы.
2. Поскольку полагаться на определенный размер обоймы ненадежно, это дополнение делает 
все возможное, чтобы обеспечивать, что все взаимодействия как с , `Ast` так и с
   `Hir` не используют рекурсию. А именно, они используют постоянное пространство 
   обоймы и пространство кучи, сопоставимое размеру исходной строки образца 
   (в байтах). Это включает в себя соответствующие уничтожители вида. 
   (Единственным исключением является буквальное извлечение, но это в конечном 
   итоге будет исправлено.)


# Сообщение об ошибке

Использование `Display` для всех видов `Error` , представленных в этой библиотеке, 
обеспечивает удобные для восприятия человеком ошибки, которые можно 
отображать конечным пользователям в моноширинном шрифте.

# Буквальное извлечение

Это дополнение обеспечивает ограниченную поддержку для [буквального извлечения из `Hir` значений](hir::literal). 
Имейте в виду, что буквальное извлечение использует рекурсию, и, 
следовательно, размер обоймы сопоставим размеру `Hir`.

Целью извлечения знаков является ускорение поиска. То есть, если вы знаете, 
что регулярное выражение должно соответствовать префиксному или суффиксному знаку, 
то часто быстрее выполнить поиск образцов этого знака, а затем подтвердить или 
опровергнуть совпадение, используя полный рычаг регулярных выражений. 
Эти оптимизации выполняются самостоятельно в `regex` дополнении.


# Характеристики дополнения

Важной возможностью, предоставляемой этим дополнением, является поддержка Unicode. 
Сюда входят такие вещи, как сворачивание строчных и заглавных букв, логические свойства, общие 
разделы, скрипты и поддержка Unicode для классов Perl `\w`, `\s` и `\d`.
Однако недостатком этой поддержки является то, что она требует объединения 
нескольких таблиц данных Unicode, которые имеют существенный размер.

Достаточное количество исходов использования не требует полной поддержки Unicode. 
По этой причине это дополнение предоставляет ряд функций для управления доступностью данных Unicode.

Если регулярное выражение пытается использовать функцию Unicode, которая недоступна из-за того, 
что соответствующая функция дополнения была отключена, то перевод этого регулярного выражения 
в `Hir` вернет ошибку. (Все еще возможно построить `Ast` для такого регулярного выражения, 
поскольку данные Unicode не используются до перевода в `Hir`.) Другими словами, включение 
или отключение любой из функций ниже может только добавить или вычесть из общего набора 
допустимых регулярных выражений. Включение или отключение функции никогда не изменит 
смысл соответствия регулярного выражения.

Доступны следующие функции:

* **std** -
  включает поддержку встроенной библиотеки. Эта функция включена по умолчанию. 
  Если отключено, используются только `core` и `alloc`.  В противном случае включение `std` обычно 
  просто включает использование сущности `std::error::Error` для различных видов ошибок .
* **unicode** -
 Включает все функции Unicode. Эта функция включена по умолчанию и всегда 
 будет охватывать все функции Unicode, даже если в будущем будут добавлены новые.
* **unicode-age** -
  Предоставьте данные для 
  [свойства Unicode `Age`](https://www.unicode.org/reports/tr44/tr44-24.html#Character_Age).
  Это позволяет использовать классы, например, `\p{Age:6.0}`
  для ссылки на все знаки, впервые представленные в Unicode 6.0
* **unicode-bool** -
  предоставляет данные для многочисленных разумных свойств Unicode. 
  Полный список здесь не включен, но содержит такие свойства, как `Alphabetic`, `Emoji`,
  `Lowercase`, `Math`, `Uppercase` и `White_Space`.
* **unicode-case** -
  предоставление данных для сопоставления без учета строчных и заглавных букв с использованием 
  [перечня «простых свободных сопоставлений» Unicode](https://www.unicode.org/reports/tr18/#Simple_Loose_Matches).
* **unicode-gencat** -
  Предоставьте данные для 
  [общих разделов Unicode](https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values).
  Это включает, но не ограничивается, `Decimal_Number`, `Letter`,
  `Math_Symbol`, `Number` and `Punctuation`.
* **unicode-perl** -
  Предоставьте данные для поддержки классов знаков Perl, поддерживающих Unicode, 
  соответствующих `\w`, `\s` и `\d`.  Это также необходимо для использования определений границ
  слов, поддерживающих Unicode. Обратите внимание, что если эта функция отключена, классы знаков 
  `\s` и `\d` по-прежнему доступны, если включены функции `unicode-bool` и `unicode-gencat` соответственно.
* **unicode-script** -
  Предоставьте данные для
  [скриптов Unicode и расширений скриптов](https://www.unicode.org/reports/tr24/).
  Это включает, но не ограничивается, `Arabic`, `Cyrillic`, `Hebrew`,
  `Latin` и `Thai`.
* **unicode-segment** -
  Предоставьте данные, необходимые для предоставления свойств, используемых для использования
  [алгоритмов сегментации писания Unicode](https://www.unicode.org/reports/tr29/).
  Это позволяет использовать такие классы, как `\p{gcb=Extend}`, `\p{wb=Katakana}` и
  `\p{sb=ATerm}`.
* **произвольный** -
  Включение этой функции вводит общедоступную зависимость от
  [`arbitrary`](https://crates.io/crates/arbitrary)
  дополнения. А именно, она использует `Arbitrary` черту из этого дополнения для
  [`Ast`](crate::ast::Ast) вида. Эта функция отключена по умолчанию.
*/

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![warn(missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(any(test, feature = "std"))]
extern crate std;

extern crate alloc;

pub use crate::{
    error::Error,
    parser::{parse, Parser, ParserBuilder},
    unicode::UnicodeWordError,
};

use alloc::string::String;

pub mod ast;
mod debug;
mod either;
mod error;
pub mod hir;
mod parser;
mod rank;
mod unicode;
mod unicode_tables;
pub mod utf8;

/// Экранирует все метазнаки регулярных выражений в `text`.
///
/// The string returned may be safely used as a literal in a regular
/// expression.
pub fn escape(text: &str) -> String {
    let mut quoted = String::new();
    escape_into(text, &mut quoted);
    quoted
}

/// Экранирует все метазнаки `text` и записывает итог в `buf`.
///
/// This will append escape characters into the given buffer. The characters
/// that are appended are safe to use as a literal in a regular expression.
pub fn escape_into(text: &str, buf: &mut String) {
    buf.reserve(text.len());
    for c in text.chars() {
        if is_meta_character(c) {
            buf.push('\\');
        }
        buf.push(c);
    }
}

/// Возвращает true, если указанный знак имеет значение в регулярном выражении.
///
/// Generally speaking, these are the only characters which _must_ be escaped
/// in order to match their literal meaning. For example, to match a literal
/// `|`, one could write `\|`. Sometimes escaping isn't always necessary. For
/// example, `-` is treated as a meta character because of its significance
/// for writing ranges inside of character classes, but the regex `-` will
/// match a literal `-` because `-` has no special meaning outside of character
/// classes.
///
/// In order to determine whether a character may be escaped at all, the
/// [`is_escapeable_character`] routine should be used. The difference between
/// `is_meta_character` and `is_escapeable_character` is that the latter will
/// return true for some characters that are _not_ meta characters. For
/// example, `%` and `\%` both match a literal `%` in all contexts. In other
/// words, `is_escapeable_character` includes "superfluous" escapes.
///
/// Note that the set of characters for which this function returns `true` or
/// `false` is fixed and won't change in a semver compatible release. (In this
/// case, "semver compatible release" actually refers to the `regex` crate
/// itself, since reducing or expanding the set of meta characters would be a
/// breaking change for not just `regex-syntax` but also `regex` itself.)
///
/// # Example
///
/// ```
/// use regex_syntax::is_meta_character;
///
/// assert!(is_meta_character('?'));
/// assert!(is_meta_character('-'));
/// assert!(is_meta_character('&'));
/// assert!(is_meta_character('#'));
///
/// assert!(!is_meta_character('%'));
/// assert!(!is_meta_character('/'));
/// assert!(!is_meta_character('!'));
/// assert!(!is_meta_character('"'));
/// assert!(!is_meta_character('e'));
/// ```
pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{'
        | '}' | '^' | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}

/// Возвращает значение true, если заданный знак можно экранировать в регулярном выражении.
///
/// This returns true in all cases that `is_meta_character` returns true, but
/// also returns true in some cases where `is_meta_character` returns false.
/// For example, `%` is not a meta character, but it is escapeable. That is,
/// `%` and `\%` both match a literal `%` in all contexts.
///
/// The purpose of this routine is to provide knowledge about what characters
/// may be escaped. Namely, most regex engines permit "superfluous" escapes
/// where characters without any special significance may be escaped even
/// though there is no actual _need_ to do so.
///
/// This will return false for some characters. For example, `e` is not
/// escapeable. Therefore, `\e` will either result in a parse error (which is
/// true today), or it could backwards compatibly evolve into a new construct
/// with its own meaning. Indeed, that is the purpose of banning _some_
/// superfluous escapes: it provides a way to evolve the syntax in a compatible
/// manner.
///
/// # Example
///
/// ```
/// use regex_syntax::is_escapeable_character;
///
/// assert!(is_escapeable_character('?'));
/// assert!(is_escapeable_character('-'));
/// assert!(is_escapeable_character('&'));
/// assert!(is_escapeable_character('#'));
/// assert!(is_escapeable_character('%'));
/// assert!(is_escapeable_character('/'));
/// assert!(is_escapeable_character('!'));
/// assert!(is_escapeable_character('"'));
///
/// assert!(!is_escapeable_character('e'));
/// ```
pub fn is_escapeable_character(c: char) -> bool {
    // Certainly escapeable if it's a meta character.
    if is_meta_character(c) {
        return true;
    }
    // Any character that isn't ASCII is definitely not escapeable. There's
    // no real need to allow things like \☃ right?
    if !c.is_ascii() {
        return false;
    }
    // Otherwise, we basically say that everything is escapeable unless it's a
    // letter or digit. Things like \3 are either octal (when enabled) or an
    // error, and we should keep it that way. Otherwise, letters are reserved
    // for adding new syntax in a backwards compatible way.
    match c {
        '0'..='9' | 'A'..='Z' | 'a'..='z' => false,
        // While not currently supported, we keep these as not escapeable to
        // give us some flexibility with respect to supporting the \< and
        // \> word boundary assertions in the future. By rejecting them as
        // escapeable, \< and \> will result in a parse error. Thus, we can
        // turn them into something else in the future without it being a
        // backwards incompatible change.
        //
        // OK, now we support \< and \>, and we need to retain them as *not*
        // escapeable here since the escape sequence is significant.
        '<' | '>' => false,
        _ => true,
    }
}

/// Возвращает значение true тогда и только тогда, когда заданный знак является знаком слова Unicode.
/// 
///
/// A Unicode word character is defined by
/// [UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties).
/// In particular, a character
/// is considered a word character if it is in either of the `Alphabetic` or
/// `Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`
/// or `Connector_Punctuation` general categories.
///
/// # Panics
///
/// If the `unicode-perl` feature is not enabled, then this function
/// panics. For this reason, it is recommended that callers use
/// [`try_is_word_character`] instead.
pub fn is_word_character(c: char) -> bool {
    try_is_word_character(c).expect("unicode-perl feature must be enabled")
}

/// Возвращает значение true тогда и только тогда, когда заданный знак является знаком слова Unicode.
/// 
///
/// A Unicode word character is defined by
/// [UTS#18 Annex C](https://unicode.org/reports/tr18/#Compatibility_Properties).
/// In particular, a character
/// is considered a word character if it is in either of the `Alphabetic` or
/// `Join_Control` properties, or is in one of the `Decimal_Number`, `Mark`
/// or `Connector_Punctuation` general categories.
///
/// # Errors
///
/// If the `unicode-perl` feature is not enabled, then this function always
/// returns an error.
pub fn try_is_word_character(
    c: char,
) -> core::result::Result<bool, UnicodeWordError> {
    unicode::is_word_character(c)
}

/// Возвращает значение true тогда и только тогда, когда заданный знак является знаком слова ASCII.
///
/// An ASCII word character is defined by the following character class:
/// `[_0-9a-zA-Z]`.
pub fn is_word_byte(c: u8) -> bool {
    match c {
        b'_' | b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn escape_meta() {
        assert_eq!(
            escape(r"\.+*?()|[]{}^$#&-~"),
            r"\\\.\+\*\?\(\)\|\[\]\{\}\^\$\#\&\-\~".to_string()
        );
    }

    #[test]
    fn word_byte() {
        assert!(is_word_byte(b'a'));
        assert!(!is_word_byte(b'-'));
    }

    #[test]
    #[cfg(feature = "unicode-perl")]
    fn word_char() {
        assert!(is_word_character('a'), "ASCII");
        assert!(is_word_character('à'), "Latin-1");
        assert!(is_word_character('β'), "Greek");
        assert!(is_word_character('\u{11011}'), "Brahmi (Unicode 6.0)");
        assert!(is_word_character('\u{11611}'), "Modi (Unicode 7.0)");
        assert!(is_word_character('\u{11711}'), "Ahom (Unicode 8.0)");
        assert!(is_word_character('\u{17828}'), "Tangut (Unicode 9.0)");
        assert!(is_word_character('\u{1B1B1}'), "Nushu (Unicode 10.0)");
        assert!(is_word_character('\u{16E40}'), "Medefaidrin (Unicode 11.0)");
        assert!(!is_word_character('-'));
        assert!(!is_word_character('☃'));
    }

    #[test]
    #[should_panic]
    #[cfg(not(feature = "unicode-perl"))]
    fn word_char_disabled_panic() {
        assert!(is_word_character('a'));
    }

    #[test]
    #[cfg(not(feature = "unicode-perl"))]
    fn word_char_disabled_error() {
        assert!(try_is_word_character('a').is_err());
    }
}
