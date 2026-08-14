use std::collections::HashMap;

fn validate_nucleotide(nucleotide: char) -> Result<(), char> {
    match nucleotide {
        'G' | 'A' | 'T' | 'C' => Ok(()),
        x => Err(x),
    }
}

fn validate_dna(dna: &str) -> Result<(), char> {
    for nucleotide in dna.chars() {
        validate_nucleotide(nucleotide)?;
    }

    Ok(())
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    validate_nucleotide(nucleotide)?;
    validate_dna(dna)?;
    Ok(dna.chars().filter(|c| *c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    validate_dna(dna)?;

    Ok(dna.chars().fold(
        HashMap::from([('G', 0), ('A', 0), ('T', 0), ('C', 0)]),
        |mut counts, nucleotide| {
            counts
                .entry(nucleotide)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            counts
        },
    ))
}
