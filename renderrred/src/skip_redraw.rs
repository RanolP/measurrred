pub enum SkipRedraw {
    Never,
    VariablesUnchanged(Vec<String>),
    Always,
}

impl SkipRedraw {
    pub fn combine_with(self, other: SkipRedraw) -> SkipRedraw {
        match (self, other) {
            (SkipRedraw::Never, _) | (_, SkipRedraw::Never) => SkipRedraw::Never,
            (SkipRedraw::VariablesUnchanged(mut a), SkipRedraw::VariablesUnchanged(mut b)) => {
                a.append(&mut b);
                SkipRedraw::VariablesUnchanged(a)
            }
            (SkipRedraw::Always, other) | (other, SkipRedraw::Always) => other,
        }
    }
}
