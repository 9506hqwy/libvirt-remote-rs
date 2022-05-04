use super::error::Error;
use fluent::{FluentArgs, FluentBundle, FluentResource};
use fluent_langneg::{convert_vec_str_to_langids_lossy, negotiate_languages, NegotiationStrategy};
use once_cell::sync::Lazy;
use std::collections::HashMap;
#[cfg(target_family = "unix")]
use std::env;
use unic_langid::{langid, langids, LanguageIdentifier};
#[cfg(target_family = "windows")]
use windows::{
    core::PWSTR,
    Win32::Globalization::{GetUserPreferredUILanguages, MUI_LANGUAGE_NAME},
};

static RESOURCES: Lazy<HashMap<LanguageIdentifier, &str>> = Lazy::new(|| {
    let mut r = HashMap::new();
    r.insert(langid!("en-us"), include_str!("en-us.txt"));
    r.insert(langid!("ja-jp"), include_str!("ja-jp.txt"));
    r
});

pub struct Locale {
    bundle: FluentBundle<FluentResource>,
}

impl Locale {
    pub fn format_message(&self, id: &str, args: Vec<(&str, &str)>) -> String {
        let mut msg_args = FluentArgs::new();
        for arg in args {
            msg_args.set(arg.0, arg.1);
        }

        if let Some(msg) = self.bundle.get_message(id) {
            if let Some(pattern) = msg.value() {
                let mut error = vec![];
                return self
                    .bundle
                    .format_pattern(pattern, Some(&msg_args), &mut error)
                    .into_owned();
            }
        }

        id.to_string()
    }

    pub fn get_message(&self, id: &str) -> String {
        if let Some(msg) = self.bundle.get_message(id) {
            if let Some(pattern) = msg.value() {
                let mut error = vec![];
                return self
                    .bundle
                    .format_pattern(pattern, None, &mut error)
                    .into_owned();
            }
        }

        id.to_string()
    }
}

pub fn setup() -> Result<Locale, Error> {
    let locale = current_locale()?;
    let bundle = current_bundle(locale)?;
    Ok(Locale { bundle })
}

fn current_bundle(id: LanguageIdentifier) -> Result<FluentBundle<FluentResource>, Error> {
    let mut bundle = FluentBundle::new(vec![id.clone()]);

    let resource = RESOURCES.get(&id).unwrap();
    let res = FluentResource::try_new(resource.to_string()).map_err(|_| Error::Locale)?;
    bundle.add_resource(res).map_err(|_| Error::Locale)?;

    // disable Unicode Directionality Isolation Marks.
    bundle.set_use_isolating(false);

    Ok(bundle)
}

fn current_locale() -> Result<LanguageIdentifier, Error> {
    let request_locales = get_request_locales()?;
    let available_locales = get_available_locales()?;
    let default_locale = langid!("en-us");
    let supported = negotiate_languages(
        &request_locales,
        &available_locales,
        Some(&default_locale),
        NegotiationStrategy::Lookup,
    );
    Ok(supported.first().cloned().unwrap().clone())
}

fn get_available_locales() -> Result<Vec<LanguageIdentifier>, Error> {
    Ok(RESOURCES.keys().cloned().collect())
}

#[cfg(target_family = "windows")]
fn get_request_locales() -> Result<Vec<LanguageIdentifier>, Error> {
    // https://docs.microsoft.com/ja-jp/windows/win32/intl/user-interface-language-management

    let mut count: u32 = 0;
    let langs = PWSTR::default();
    let mut langs_len: u32 = 0;
    unsafe {
        GetUserPreferredUILanguages(MUI_LANGUAGE_NAME, &mut count, langs, &mut langs_len)
            .ok()
            .map_err(|_| Error::Locale)
    }?;

    if count == 0 {
        return Ok(langids!("en-us"));
    }

    let mut buffer = vec![0u16; langs_len as usize];
    let langs = PWSTR(buffer.as_mut_ptr());
    unsafe {
        GetUserPreferredUILanguages(MUI_LANGUAGE_NAME, &mut count, langs, &mut langs_len)
            .ok()
            .map_err(|_| Error::Locale)
    }?;

    let results: Vec<String> = buffer
        .split(|&v| v == 0)
        .take(count as usize)
        .map(String::from_utf16_lossy)
        .collect();
    Ok(convert_vec_str_to_langids_lossy(results.as_slice()))
}

#[cfg(target_family = "unix")]
fn get_request_locales() -> Result<Vec<LanguageIdentifier>, Error> {
    if let Ok(lang) = env::var("LANG") {
        let lang = lang.split('.').next().unwrap();
        return Ok(convert_vec_str_to_langids_lossy(&[lang]));
    }

    Ok(langids!("en-us"))
}
