pub trait HashDigest {
    type Digest;
    fn digest(&self) -> Self::Digest;
    fn hexdigest(&self) -> String;
}
