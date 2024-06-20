use scraper::{Html, Selector};

const LOW_TTR: f64 = 0.8;
const LOW_LOC_PROB: f64 = 0.6; 

/// This type defines a Localization, as we do not use extra structs for this 
/// we use a custom type name to make this more readable. 
/// The first part of this type is the found localization, the second part is
/// the probability it's the right localization. It may be weird at first glance
/// to have this probability data but when textual language detection algorithm
/// will be implemented it will be a key information when language is not 
/// explicitly set.
/// WARN: IDK IF IT'S A GOOD IDEA TO DO THIS
pub type Localization = (String, f64);

/// To determine a website's localization, we use the following things:
/// 1. `html[lang]`
/// 2. `meta[http-equiv="content-language"]`
///
/// INFO: The following techniques should be implemented ASAP:
/// TODO: `hreflang` for external pages
pub fn get_localization(page: Html) -> Localization {
    let html_selector = Selector::parse("html").unwrap();
    let meta_selector = Selector::parse("meta[http-equiv=\"content_language\"]")
        .unwrap();

    let html = page.select(&html_selector).collect::<Vec<_>>();
    let meta = page.select(&meta_selector).collect::<Vec<_>>();

    if let Some(lang) = html.get(0).unwrap().attr("lang") {
        return (lang.into(), 1.0);
    }
    if let Some(lang) = meta.get(0).unwrap().attr("lang") {
        return (lang.into(), 1.0);
    }

    ("en-US".into(), 0.0)
}

/// Automatically determine if the localization found should be used or the 
/// default localization should be used. It works by determining the TTR we
/// already use for indexation. If the probability AND the TTR are low, the
/// default localization will be used.
pub fn auto_choose_localization(loc: Localization, ttr: f64) -> Localization {
    println!("{:?}", loc);
    if loc.1 <= LOW_LOC_PROB && ttr <= LOW_TTR {
        return ("en-US".into(), 0.0)
    }

    loc
}
