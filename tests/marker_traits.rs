// DO NOT EDIT. Automatically generated by 'scripts/public-traits-tests.py'
// on 2021-07-03 16:14:18.759956.

#[test]
fn marker_traits() {
    use std::panic::{RefUnwindSafe, UnwindSafe};
    fn assert_marker_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe>() {}

    assert_marker_traits::<regex::RegexSetBuilder>();
    assert_marker_traits::<regex::RegexBuilder>();
    assert_marker_traits::<regex::RegexSet>();
    assert_marker_traits::<regex::SetMatches>();
    assert_marker_traits::<regex::SetMatchesIntoIter>();
    assert_marker_traits::<regex::SetMatchesIter>();
    assert_marker_traits::<regex::CaptureLocations>();
    // assert_marker_traits::<regex::CaptureMatches>();
    assert_marker_traits::<regex::CaptureNames>();
    assert_marker_traits::<regex::Captures>();
    assert_marker_traits::<regex::Match>();
    // assert_marker_traits::<regex::Matches>();
    assert_marker_traits::<regex::NoExpand>();
    assert_marker_traits::<regex::Regex>();
    // assert_marker_traits::<regex::ReplacerRef<&str>>();
    // assert_marker_traits::<regex::Split>();
    // assert_marker_traits::<regex::SplitN>();
    assert_marker_traits::<regex::SubCaptureMatches>();

    assert_marker_traits::<regex::bytes::RegexBuilder>();
    assert_marker_traits::<regex::bytes::RegexSetBuilder>();
    assert_marker_traits::<regex::bytes::Match>();
    assert_marker_traits::<regex::bytes::Regex>();
    // assert_marker_traits::<regex::bytes::Matches>();
    // assert_marker_traits::<regex::bytes::CaptureMatches>();
    // assert_marker_traits::<regex::bytes::Split>();
    // assert_marker_traits::<regex::bytes::SplitN>();
    assert_marker_traits::<regex::bytes::CaptureNames>();
    assert_marker_traits::<regex::bytes::CaptureLocations>();
    assert_marker_traits::<regex::bytes::Captures>();
    assert_marker_traits::<regex::bytes::SubCaptureMatches>();
    // assert_marker_traits::<regex::bytes::ReplacerRef<&str>>();
    assert_marker_traits::<regex::bytes::NoExpand>();
    assert_marker_traits::<regex::bytes::RegexSet>();
    assert_marker_traits::<regex::bytes::SetMatches>();
    assert_marker_traits::<regex::bytes::SetMatchesIntoIter>();
    assert_marker_traits::<regex::bytes::SetMatchesIter>();
}
