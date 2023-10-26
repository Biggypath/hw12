fn main() {

}
#[derive(Clone)]
enum Text {
    Plain(String),
    Repeated(String, usize),
    Joined(Vec<Box<Text>>, Box<Text>)
}

impl Text {
    fn value(&self) -> String {
        match self {
            Text::Plain(t) => t.clone(),
            Text::Repeated(t, num) => t.clone().repeat(*num),
            Text::Joined(tvec, sep) => {
                let joined_text: Vec<String> = tvec.iter().map(|t| t.value()).collect();
                joined_text.join(&sep.value())
            }
        }
    }
}

impl From<&Text> for String {
    fn from(t: &Text) -> String {
        match t {
            Text::Plain(s) => s.clone(),
            Text::Repeated(s, num) => s.clone().repeat(*num),
            Text::Joined(_, _) => t.value()
        }
    }
}

impl From<&Text> for Box<Text> {
    fn from(t: &Text) -> Box<Text> {
        match t {
            Text::Plain(s) => Box::new(Text::Plain(s.clone())),
            Text::Repeated(s, num) => Box::new(Text::Repeated(s.clone(), *num)),
            Text::Joined(tvec, sep) => Box::new(Text::Joined(
                tvec.iter().map(|t| t.as_ref().into()).collect(),
                sep.as_ref().into()
            ))
        }
    }
}

impl AsRef<Text> for Text {
    fn as_ref(&self) -> &Text { &self }
}

#[test]
fn test_text_composition() {
    let t1 = Text::Plain("x|x".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    let mut tvec: Vec<Box<Text>> = Vec::new();
    tvec.push(t1.into());
    tvec.push(t2.into());
    tvec.push(t3.into());
    tvec.push(t4.into());
    let t5 = Text::Plain("--".into());
    let t6 = Text::Joined(tvec, t5.into());
    let ptn = ["x|x","[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}