use crate::{
    consts::{
        SUPPORTED_ALPHA2_LIST, SUPPORTED_CONTINENT_LIST, SUPPORTED_COUNTRIES_COUNT,
        SUPPORTED_REGION_LIST, SUPPORTED_SUBREGION_LIST, SUPPORTED_WORLD_REGION_LIST,
    },
    Continent, Country, Region, SubRegion, WorldRegion,
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct ArrayIterator {
    offset: usize,
    length: usize,
    reusable: bool,
}

impl ArrayIterator {
    pub fn new(length: usize) -> Self {
        Self {
            offset: 0,
            reusable: false,
            length,
        }
    }

    pub fn new_reusable(length: usize) -> Self {
        let mut new = Self::new(length);
        new.reusable = true;
        new
    }
}

impl ArrayIterator {
    fn next<I: Clone>(&mut self, array_ref: &[I]) -> Option<I> {
        if self.offset < self.length {
            self.offset += 1;
            Some(array_ref[self.offset - 1].clone())
        } else {
            if self.reusable {
                self.offset = 0;
            }
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length - self.offset, Some(self.length - self.offset))
    }

    fn count(self) -> usize {
        self.length
    }

    fn last<I: Clone>(self, array_ref: &[I]) -> Option<I> {
        Some(array_ref[self.length - 1].clone())
    }

    fn nth<I: Clone>(&mut self, n: usize, array_ref: &[I]) -> Option<I> {
        let offset = n + self.offset;
        if offset < self.length {
            self.offset = offset;
            Some(array_ref[offset].clone())
        } else {
            None
        }
    }
}

macro_rules! impl_iterator {
    ($struct_name:ident, $item:ident, $array_ref:expr, $length:expr, $mapper:expr) => {
        impl $struct_name {
            /// Initializes a new iterator.
            pub fn new() -> Self {
                Self {
                    iterator: ArrayIterator::new($length),
                }
            }

            /// Initializes a new iterator that starts iterating from first item after yielding [`None`](None).
            pub fn new_reusable() -> Self {
                Self {
                    iterator: ArrayIterator::new_reusable($length),
                }
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Iterator for $struct_name {
            type Item = $item;

            fn next(&mut self) -> Option<Self::Item> {
                self.iterator.next($array_ref).map($mapper)
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.iterator.size_hint()
            }

            fn count(self) -> usize {
                self.iterator.count()
            }

            fn last(self) -> Option<Self::Item> {
                self.iterator.last($array_ref).map($mapper)
            }

            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                self.iterator.nth(n, $array_ref).map($mapper)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// An iterator over all of your included [`Countries`](Country).
///
/// # Example
/// ```
/// use keshvar::CountryIterator;
///
/// let mut supported_country_names = Vec::new();
/// for country in CountryIterator::new() {
///     let name = country.iso_short_name().to_string();
///     supported_country_names.push(name)
/// }
///
/// let another_supported_country_names: Vec<String> = CountryIterator::new()
///     .map(|country| country.iso_short_name().to_string())
///     .collect();
///
/// assert_eq!(supported_country_names, another_supported_country_names);
/// ```
pub struct CountryIterator {
    iterator: ArrayIterator,
}
impl_iterator!(
    CountryIterator,
    Country,
    SUPPORTED_ALPHA2_LIST,
    *SUPPORTED_COUNTRIES_COUNT,
    |alpha2| alpha2.to_country()
);

#[derive(Debug, Clone, Copy, PartialEq)]
/// An iterator over all of your supported [`Continent`](Continent)s.
///
/// # Example
/// ```
/// use keshvar::{ContinentIterator, Country};
///
/// let mut country_list = Vec::new();
/// for continent in ContinentIterator::new() {
///     for alpha2_str in continent.alpha2_list() {
///         if let Ok(country) = Country::try_from(*alpha2_str) {
///             country_list.push(country)
///         }
///     }
/// }
///
/// let another_country_list: Vec<Country> = ContinentIterator::new()
///     .map(|continent| {
///         continent
///             .alpha2_list()
///             .iter()
///             .filter_map(|alpha2_str| Country::try_from(*alpha2_str).ok())
///             .collect::<Vec<_>>()
///     })
///     .flatten()
///     .collect();
///
/// assert_eq!(country_list, another_country_list);
/// ```
pub struct ContinentIterator {
    iterator: ArrayIterator,
}
impl_iterator!(
    ContinentIterator,
    Continent,
    SUPPORTED_CONTINENT_LIST,
    SUPPORTED_CONTINENT_LIST.len(),
    |x| x
);

#[derive(Debug, Clone, Copy, PartialEq)]
/// An iterator over all of your supported [`Region`](Region)s.
///
/// # Example
/// ```
/// use keshvar::{RegionIterator, Country};
///
/// let mut country_list_with_en_spoken_lang = Vec::new();
/// for region in RegionIterator::new() {
///     for alpha2_str in region.alpha2_list() {
///         if let Ok(country) = Country::try_from(*alpha2_str) {
///             if country.spoken_language_list().contains(&"en") {
///                 country_list_with_en_spoken_lang.push(country)
///             }
///         }
///     }
/// }
///
/// let another_country_list_with_en_spoken_lang: Vec<Country> = RegionIterator::new()
///     .map(|region| {
///         region
///             .alpha2_list()
///             .iter()
///             .filter_map(|alpha2_str| Country::try_from(*alpha2_str).ok())
///             .filter(|country| country.spoken_language_list().contains(&"en"))
///             .collect::<Vec<_>>()
///     })
///     .flatten()
///     .collect();
///
/// assert_eq!(country_list_with_en_spoken_lang, another_country_list_with_en_spoken_lang);
/// ```
pub struct RegionIterator {
    iterator: ArrayIterator,
}
impl_iterator!(
    RegionIterator,
    Region,
    SUPPORTED_REGION_LIST,
    SUPPORTED_REGION_LIST.len(),
    |x| x
);

#[derive(Debug, Clone, Copy, PartialEq)]
/// An iterator over all of your supported [`SubRegion`](SubRegion)s.
///
/// # Example
/// ```
/// use keshvar::{SubRegionIterator, Country, WeekDay};
///
/// let mut country_list = Vec::new();
/// for subregion in SubRegionIterator::new() {
///     for alpha2_str in subregion.alpha2_list() {
///         if let Ok(country) = Country::try_from(*alpha2_str) {
///             if country.start_of_week() == WeekDay::Sunday {
///                 country_list.push(country)
///             }
///         }
///     }
/// }
///
/// let another_country_list: Vec<Country> = SubRegionIterator::new()
///     .map(|subregion| {
///         subregion
///             .alpha2_list()
///             .iter()
///             .filter_map(|alpha2_str| Country::try_from(*alpha2_str).ok())
///             .filter(|country| country.start_of_week() == WeekDay::Sunday)
///             .collect::<Vec<_>>()
///     })
///     .flatten()
///     .collect();
///
/// assert_eq!(country_list, another_country_list);
/// ```
pub struct SubRegionIterator {
    iterator: ArrayIterator,
}
impl_iterator!(
    SubRegionIterator,
    SubRegion,
    SUPPORTED_SUBREGION_LIST,
    SUPPORTED_SUBREGION_LIST.len(),
    |x| x
);

#[derive(Debug, Clone, Copy, PartialEq)]
/// An iterator over all of your supported [`WorldRegion`](WorldRegion)s.
///
/// # Example
/// ```
/// use keshvar::{WorldRegionIterator, Country, CurrencyCode};
///
/// let mut country_list = Vec::new();
/// for world_region in WorldRegionIterator::new() {
///     for alpha2_str in world_region.alpha2_list() {
///         if let Ok(country) = Country::try_from(*alpha2_str) {
///             if country.currency_code() == CurrencyCode::EUR {
///                 country_list.push(country)
///             }
///         }
///     }
/// }
///
/// let another_country_list: Vec<Country> = WorldRegionIterator::new()
///     .map(|world_region| {
///         world_region
///             .alpha2_list()
///             .iter()
///             .filter_map(|alpha2_str| Country::try_from(*alpha2_str).ok())
///             .filter(|country| country.currency_code() == CurrencyCode::EUR)
///             .collect::<Vec<_>>()
///     })
///     .flatten()
///     .collect();
///
/// assert_eq!(country_list, another_country_list);
/// ```
pub struct WorldRegionIterator {
    iterator: ArrayIterator,
}
impl_iterator!(
    WorldRegionIterator,
    WorldRegion,
    SUPPORTED_WORLD_REGION_LIST,
    SUPPORTED_WORLD_REGION_LIST.len(),
    |x| x
);
