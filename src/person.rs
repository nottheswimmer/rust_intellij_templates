use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;

/// A Person is represented here
///
/// #[derive(Debug)] is for allowing the struct to be debug printed
#[derive(Debug, Clone, PartialEq)]
pub struct Person<'a> {
    /// A person instance must have a ethnicity
    ethnicity: &'a str,
    /// A person instance must have a name
    name: &'a str,
    /// A person instance must have a favorite_name
    favorite_name: String,
    /// A person instance must have a first_initial
    first_initial: char,
    /// A person instance must have a middle_initial
    middle_initial: char,
    /// A person instance must have a last_initial
    last_initial: char,
    /// A person instance must have a age
    age: i32,
    /// A person instance must have a iq
    iq: i32,
    /// A person instance must have a gpa
    gpa: f32,
    /// A person instance must have a ic_cool
    ic_cool: bool,
    /// A person instance must have a lottery_numbers
    lottery_numbers: Vec<i32>,
    /// A person instance must have a last_words
    last_words: Vec<&'a str>,
    /// A person instance must have a contact_list
    contact_list: Vec<String>,
    /// A person instance must have a letter_grades
    letter_grades: Vec<char>,
    /// A person instance must have a favorite_numbers
    favorite_numbers: HashSet<i32>,
    /// A person instance must have a favorite_colors
    favorite_colors: HashSet<&'a str>,
    /// A person instance must have a favorite_letters
    favorite_letters: HashSet<char>,
    /// A person instance must have a favorite_words
    favorite_words: HashSet<String>,
    /// A person instance must have a top_songs_by_album
    top_songs_by_album: HashMap<String,String>,
}

impl<'a> Person<'a> {
    /// Returns a person instance
    ///
    /// # Arguments
    ///
    /// * `ethnicity` - A &'a str that holds the ethnicity of the person instance
    /// * `name` - A &'a str that holds the name of the person instance
    /// * `favorite_name` - A String that holds the favorite_name of the person instance
    /// * `first_initial` - A char that holds the first_initial of the person instance
    /// * `middle_initial` - A char that holds the middle_initial of the person instance
    /// * `last_initial` - A char that holds the last_initial of the person instance
    /// * `age` - A i32 that holds the age of the person instance
    /// * `iq` - A i32 that holds the iq of the person instance
    /// * `gpa` - A f32 that holds the gpa of the person instance
    /// * `ic_cool` - A bool that holds the ic_cool of the person instance
    /// * `lottery_numbers` - A Vec<i32> that holds the lottery_numbers of the person instance
    /// * `last_words` - A Vec<&'a str> that holds the last_words of the person instance
    /// * `contact_list` - A Vec<String> that holds the contact_list of the person instance
    /// * `letter_grades` - A Vec<char> that holds the letter_grades of the person instance
    /// * `favorite_numbers` - A HashSet<i32> that holds the favorite_numbers of the person instance
    /// * `favorite_colors` - A HashSet<&'a str> that holds the favorite_colors of the person instance
    /// * `favorite_letters` - A HashSet<char> that holds the favorite_letters of the person instance
    /// * `favorite_words` - A HashSet<String> that holds the favorite_words of the person instance
    /// * `top_songs_by_album` - A HashMap<String,String> that holds the top_songs_by_album of the person instance
    ///
    /// # Example
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to Rustdoc, it will even test it for you!
    /// use doc::Person
    /// let person = Person::new("abc", "abc", String::from("abc"), 'a', 'a', 'a', 1, 1, 1.0, false, vec![1, 2, 3], vec!["a", "b", "c" ], vec![String::from("a"), String::from("b"), String::from("c")], vec!['a', 'b', 'c'], HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashMap::new());
    /// println!("{}", person);
    /// ```
    pub fn new(ethnicity: &'a str, name: &'a str, favorite_name: String, first_initial: char, middle_initial: char, last_initial: char, age: i32, iq: i32, gpa: f32, ic_cool: bool, lottery_numbers: Vec<i32>, last_words: Vec<&'a str>, contact_list: Vec<String>, letter_grades: Vec<char>, favorite_numbers: HashSet<i32>, favorite_colors: HashSet<&'a str>, favorite_letters: HashSet<char>, favorite_words: HashSet<String>, top_songs_by_album: HashMap<String,String>) -> Person<'a> {
    Person { ethnicity, name, favorite_name, first_initial, middle_initial, last_initial, age, iq, gpa, ic_cool, lottery_numbers, last_words, contact_list, letter_grades, favorite_numbers, favorite_colors, favorite_letters, favorite_words, top_songs_by_album }
    }

    // Getters
    pub fn ethnicity(&self) -> &'a str { &self.ethnicity }
    pub fn name(&self) -> &'a str { &self.name }
    pub fn favorite_name(&self) -> &String { &self.favorite_name }
    pub fn first_initial(&self) -> &char { &self.first_initial }
    pub fn middle_initial(&self) -> &char { &self.middle_initial }
    pub fn last_initial(&self) -> &char { &self.last_initial }
    pub fn age(&self) -> &i32 { &self.age }
    pub fn iq(&self) -> &i32 { &self.iq }
    pub fn gpa(&self) -> &f32 { &self.gpa }
    pub fn ic_cool(&self) -> &bool { &self.ic_cool }
    pub fn lottery_numbers(&self) -> &Vec<i32> { &self.lottery_numbers }
    pub fn last_words(&self) -> &Vec<&'a str> { &self.last_words }
    pub fn contact_list(&self) -> &Vec<String> { &self.contact_list }
    pub fn letter_grades(&self) -> &Vec<char> { &self.letter_grades }
    pub fn favorite_numbers(&self) -> &HashSet<i32> { &self.favorite_numbers }
    pub fn favorite_colors(&self) -> &HashSet<&'a str> { &self.favorite_colors }
    pub fn favorite_letters(&self) -> &HashSet<char> { &self.favorite_letters }
    pub fn favorite_words(&self) -> &HashSet<String> { &self.favorite_words }
    pub fn top_songs_by_album(&self) -> &HashMap<String,String> { &self.top_songs_by_album }
}

impl<'a> fmt::Display for Person<'a> {
    /// A string representation of the person instance for user-facing output
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person instance")
    }
}

// Issue: The above doc-tests won't work if the target is a binary.
// Here are some physical tests as an alternative that will run for a binary
// with `cargo test`.
//
// https://github.com/rust-lang/rust/issues/50784
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create() {
        let person = Person::new("abc", "abc", String::from("abc"), 'a', 'a', 'a', 1, 1, 1.0, false, vec![1, 2, 3], vec!["a", "b", "c" ], vec![String::from("a"), String::from("b"), String::from("c")], vec!['a', 'b', 'c'], HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(), HashMap::new());
        println!("{}", person);
                println!("{:?}", person.ethnicity());
                println!("{:?}", person.name());
                println!("{:?}", person.favorite_name());
                println!("{:?}", person.first_initial());
                println!("{:?}", person.middle_initial());
                println!("{:?}", person.last_initial());
                println!("{:?}", person.age());
                println!("{:?}", person.iq());
                println!("{:?}", person.gpa());
                println!("{:?}", person.ic_cool());
                println!("{:?}", person.lottery_numbers());
                println!("{:?}", person.last_words());
                println!("{:?}", person.contact_list());
                println!("{:?}", person.letter_grades());
                println!("{:?}", person.favorite_numbers());
                println!("{:?}", person.favorite_colors());
                println!("{:?}", person.favorite_letters());
                println!("{:?}", person.favorite_words());
                println!("{:?}", person.top_songs_by_album());
            }
}
