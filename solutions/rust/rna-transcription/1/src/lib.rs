#[derive(Clone, Debug, PartialEq, Eq)]
enum DnaNucleo {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
}

impl TryFrom<char> for DnaNucleo {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(DnaNucleo::Adenine),
            'C' => Ok(DnaNucleo::Cytosine),
            'G' => Ok(DnaNucleo::Guanine),
            'T' => Ok(DnaNucleo::Thymine),
            c => Err(format!(
                "Invalid char {c} found. Only accepted values are 'A', 'C', 'G', 'T'."
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RnaNucleo {
    Adenine,
    Cytosine,
    Guanine,
    Uracil,
}

impl TryFrom<char> for RnaNucleo {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(RnaNucleo::Adenine),
            'C' => Ok(RnaNucleo::Cytosine),
            'G' => Ok(RnaNucleo::Guanine),
            'U' => Ok(RnaNucleo::Uracil),
            c => Err(format!(
                "Invalid char {c} found. Only accepted values are 'A', 'C', 'G', 'U'."
            )),
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
        Ok(Dna {
            seq: dna
                .chars()
                .enumerate()
                .map(|(i, c)| DnaNucleo::try_from(c).map_err(|_| i))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            seq: self.seq.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        Ok(Rna {
            seq: rna
                .chars()
                .enumerate()
                .map(|(i, c)| RnaNucleo::try_from(c).map_err(|_| i))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
