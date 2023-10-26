fn main() {

}
trait Text {
    fn value(&self) -> String;
    fn clone_box(&self) -> Box<dyn Text>;
}
    
impl Clone for Box<dyn Text> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
struct PlainText { chars: String }

#[derive(Clone)]
struct RepeatedText {chars: Box<dyn Text>, num: usize}

#[derive(Clone)]
struct JoinedText {tvec: Vec<Box<dyn Text>>, sep: Box<dyn Text>}

impl From<&str> for PlainText {
    fn from(text: &str) -> PlainText {
        PlainText{ chars: text.to_string() }
    }
}

impl Text for PlainText {
    fn value(&self) -> String { self.chars.clone() }
    fn clone_box(&self) -> Box<dyn Text> { Box::new(self.clone()) }
}

impl Text for RepeatedText {
    fn value(&self) -> String { self.chars.clone().value().repeat(self.num) }
    fn clone_box(&self) -> Box<dyn Text> { Box::new(self.clone()) }
}

impl Text for JoinedText {
    fn value(&self) -> String {
        let joined_text: Vec<String> = self.tvec.iter().map(|t| t.value()).collect();
        joined_text.join(&self.sep.value())
    }
    fn clone_box(&self) -> Box<dyn Text> { Box::new(self.clone()) }
} 

impl RepeatedText {
    fn with_parts(chars: &dyn Text, num: usize) -> RepeatedText {
        RepeatedText {
            chars: chars.clone_box(),
            num,
            
        }
    }
}

impl JoinedText {
    fn with_parts(tvec: &Vec<Box<dyn Text>>, sep: &dyn Text) -> JoinedText {
        JoinedText { tvec: tvec.clone(), sep: sep.clone_box() }
    }
}

impl AsRef<dyn Text> for PlainText {
    fn as_ref(&self) -> &(dyn Text + 'static) { self }
}

#[test]
fn test_text_composition() {
    let t1 = PlainText::from("x|x");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    let mut tvec: Vec<Box<dyn Text>> = Vec::new();
    tvec.push(t1.clone_box());
    tvec.push(t2.clone_box());
    tvec.push(t3.clone_box());
    tvec.push(t4.clone_box());
    let t5 = PlainText::from("--");
    let t6 = JoinedText::with_parts(&tvec, &t5);
    let ptn = ["x|x","[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}