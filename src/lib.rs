extern crate strsim;
use strsim::*;

fn equalish(a: f64, b: f64) -> bool {
    let eps = 0.0001;
    let d = a - b;
    if d.abs() < eps {
        true
    } else {
        false
    }
}
/// Given a &str and a slice of &str, find the closest to the former within the latter
/// list, ignoring case. Returns an Ok wrapped &str if succcessful, or an Err wrapped
/// static &str if unsuccessful.
#[allow(dead_code)]
fn closest(this: &str, thats: &[&str])->Result<String, &'static str> {
    // conver this to lowercase because we don't want to consider case when
    // calculating the distance
    let this_lower = this.to_lowercase();
    // convert the str slice to a vector of lowercase equivalents
    let thats_lower: Vec<String> = thats.iter().map(|x| x.to_lowercase()).collect();
    // initialize a vector to store the weights in.
    let mut weights = Vec::new();
    // initialize a cnt var to use in case we find an exact match
    let mut cnt:usize = 0;
    // iterate over lowercase word list, storing the resemblance factor (0->1 for least to most)
    // between each word in the list and the lower case version of the first argument.
    // Along the way, if we find an exact match, then return it.
    for word in &thats_lower {
        let answer = jaro_winkler(&this_lower, word);
        if equalish(answer,1.0) { return Ok(String::from(thats[cnt]));}
        weights.push(answer);
        cnt += 1;
    }
    // create a variable to store the index of the closest match
    let mut key:usize = 0;
    // calculate the highest resemblance, storing the index in key
    let greatest = weights.iter().enumerate().fold( 0.0, |acc, x| if *x.1 as f64 > acc {key=x.0; *x.1}else{ acc } );
    // throw an error if we were unable to find any match at all. Otherwise return
    // the match corresponding to the original list (before converting to lower case)
    if greatest == 0.0 {
        Err("Did not find any matches")
    } else {
        Ok(String::from(thats[key]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s<I>(value: I) -> String where I: Into<String> {
        value.into()
    }

    #[test]
    fn closest_test() {
        let a = "fred";
        let b = ["frood","fr", "Fred"];
        let answer = closest(a, &b);
        assert_eq!( Ok(s("Fred")), answer);
    }

    #[test]
    fn closest_strings() {
        let a = s("fred");
        let b:Vec<String> = ["frood","fr", "Fred"].iter().map(|x| x.to_string()).collect();
        let v2: Vec<&str> = b.iter().map(|s| &**s).collect();
        let answer = closest(&a, &v2);
        assert_eq!( Ok(s("Fred")), answer);
    }

    #[test]
    fn closest_test2() {
        let a = "";
        let b = ["frood","fr", "Fred"];
        let answer = closest(a, &b);
        assert_eq!( Err("Did not find any matches"), answer);
    }

    #[test]
    fn closest_test3() {
        let a = "freddy";
        let b = ["frood","fr", "Fred"];
        let answer = closest(a, &b);
        assert_eq!( Ok(s("Fred")), answer);
    }
}