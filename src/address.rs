pub trait NimiqAddressString {
    fn to_relaxed_form(&self) -> Self;
    fn is_valid(&self) -> bool;
}

const NIMIQ_ALPHABET: &'static str = "0123456789ABCDEFGHJKLMNPQRSTUVXY";

impl NimiqAddressString for String {
    fn to_relaxed_form(&self) -> Self {
        self.chars()
            .map(|c| match c {
                // Non-existent characters
                'O' => '0',
                'I' => '1',
                'Z' => '2',
                // Character equivalence
                'E' => '3',
                'A' => '4',
                'S' => '5',
                'G' => '6',
                'T' => '7',
                'L' => '1',
                _ => c
            }).collect()
    }

    fn is_valid(&self) -> bool {
        self.chars().all(|c| NIMIQ_ALPHABET.contains(c))
    }
}
