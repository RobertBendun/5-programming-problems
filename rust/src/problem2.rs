struct Mix<T, It1, It2>
where
    It1: Iterator<Item = T>,
    It2: Iterator<Item = T>,
{
    fst: It1,
    snd: It2,
    yield_fst: bool,
}

impl<I1, I2, T> Iterator for Mix<T, I1, I2>
where
    I1: Iterator<Item = T>,
    I2: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result = if self.yield_fst {
            self.fst.next()
        } else {
            self.snd.next()
        };
        self.yield_fst = !self.yield_fst;
        result
    }
}

trait IntoMix<T> {
    fn mix<It>(self, mixed_with: It) -> Mix<T, Self, It>
    where
        It: Iterator<Item = T>,
        Self: Iterator<Item = T> + Sized;
}

impl<T, It1> IntoMix<T> for It1
where
    It1: Iterator<Item = T>,
{
    fn mix<It2>(self, mixed_with: It2) -> Mix<T, Self, It2>
    where
        It2: Iterator<Item = T>,
    {
        Mix {
            fst: self,
            snd: mixed_with,
            yield_fst: true,
        }
    }
}

#[test]
fn mixin() {
    assert_eq!(
        "abc".chars().mix("123".chars()).collect::<String>(),
        String::from("a1b2c3")
    );
}
