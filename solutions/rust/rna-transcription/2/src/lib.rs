#[derive(Debug, PartialEq, Eq)]
enum DnaNucleo {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
}

impl DnaNucleo {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Self::Adenine),
            'C' => Some(Self::Cytosine),
            'G' => Some(Self::Guanine),
            'T' => Some(Self::Thymine),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RnaNucleo {
    Adenine,
    Cytosine,
    Guanine,
    Uracil,
}

impl RnaNucleo {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Self::Adenine),
            'C' => Some(Self::Cytosine),
            'G' => Some(Self::Guanine),
            'U' => Some(Self::Uracil),
            _ => None,
        }
    }
}

impl From<DnaNucleo> for RnaNucleo {
    fn from(value: DnaNucleo) -> Self {
        match value {
            DnaNucleo::Adenine => RnaNucleo::Uracil,
            DnaNucleo::Cytosine => RnaNucleo::Guanine,
            DnaNucleo::Guanine => RnaNucleo::Cytosine,
            DnaNucleo::Thymine => RnaNucleo::Adenine,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    seq: Vec<DnaNucleo>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    seq: Vec<RnaNucleo>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .enumerate()
            .map(|(i, c)| DnaNucleo::from_char(c).ok_or(i))
            .collect::<Result<Vec<_>, _>>()
            .map(|seq| Dna { seq })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            seq: self.seq.into_iter().map(Into::into).collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .enumerate()
            .map(|(i, c)| RnaNucleo::from_char(c).ok_or(i))
            .collect::<Result<Vec<_>, _>>()
            .map(|seq| Rna { seq })
    }
}
