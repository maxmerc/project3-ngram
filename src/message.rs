/// A request from the client to the server
#[derive(Debug, PartialEq)]
pub enum Request {
    /// Add the document `doc` to the archive
    Publish { doc: String },
    /// Search for the word `word` in the archive
    Search { word: String },
    /// Retrieve the document with the index `id` from the archive
    Retrieve { id: usize },
}
impl Request {
    // TODO:
    // Convert the request `self` into a byte vector. See the assignment handout for suggestions on
    // how to represent the request as a series of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        match self {
            Request::Publish { doc } => {
                bytes.push(0); // Use 0 as a marker for Publish
                let doc_bytes = doc.as_bytes();
                bytes.extend((doc_bytes.len() as u32).to_be_bytes());
                bytes.extend(doc_bytes);
            }
            Request::Search { word } => {
                bytes.push(1); // Use 1 as a marker for Search
                let word_bytes = word.as_bytes();
                bytes.extend((word_bytes.len() as u32).to_be_bytes());
                bytes.extend(word_bytes);
            }
            Request::Retrieve { id } => {
                bytes.push(2); // Use 2 as a marker for Retrieve
                bytes.extend((*id as u64).to_be_bytes());
            }
        }
        bytes
    }
    // TODO:
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let mut request_type = [0; 1];
        reader.read_exact(&mut request_type).ok()?;

        match request_type[0] {
            0 => {
                // Publish
                let mut length_bytes = [0; 4];
                reader.read_exact(&mut length_bytes).ok()?;
                let length = u32::from_be_bytes(length_bytes) as usize;

                let mut doc_bytes = vec![0; length];
                reader.read_exact(&mut doc_bytes).ok()?;
                let doc = String::from_utf8(doc_bytes).ok()?;
                Some(Request::Publish { doc })
            }
            1 => {
                // Search
                let mut length_bytes = [0; 4];
                reader.read_exact(&mut length_bytes).ok()?;
                let length = u32::from_be_bytes(length_bytes) as usize;

                let mut word_bytes = vec![0; length];
                reader.read_exact(&mut word_bytes).ok()?;
                let word = String::from_utf8(word_bytes).ok()?;
                Some(Request::Search { word })
            }
            2 => {
                // Retrieve
                let mut id_bytes = [0; 8];
                reader.read_exact(&mut id_bytes).ok()?;
                let id = usize::from_be_bytes(id_bytes);
                Some(Request::Retrieve { id })
            }
            _ => None,
        }
    }
}

/// A response from the server to the client
#[derive(Debug, PartialEq)]
pub enum Response {
    /// The document was successfully added to the archive with the given index
    PublishSuccess(usize),
    /// The search for the word was successful, and the indices of the documents containing the
    /// word are returned
    SearchSuccess(Vec<usize>),
    /// The retrieval of the document was successful, and the document is returned
    RetrieveSuccess(String),
    /// The request failed
    Failure,
}
impl Response {
    // TODO:
    // Convert the request `self` into a byte vector. See the assignment handout for suggestions on
    // how to represent the request as a series of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        match self {
            Response::PublishSuccess(id) => {
                bytes.push(0); // Use 0 as a marker for PublishSuccess
                bytes.extend((*id as u64).to_be_bytes());
            }
            Response::SearchSuccess(ids) => {
                bytes.push(1); // Use 1 as a marker for SearchSuccess
                bytes.extend((ids.len() as u32).to_be_bytes());
                for id in ids {
                    bytes.extend((*id as u64).to_be_bytes());
                }
            }
            Response::RetrieveSuccess(doc) => {
                bytes.push(2); // Use 2 as a marker for RetrieveSuccess
                let doc_bytes = doc.as_bytes();
                bytes.extend((doc_bytes.len() as u32).to_be_bytes());
                bytes.extend(doc_bytes);
            }
            Response::Failure => {
                bytes.push(3); // Use 3 as a marker for Failure
            }
        }

        bytes
    }
    // TODO:
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let mut response_type = [0; 1];
        reader.read_exact(&mut response_type).ok()?;

        match response_type[0] {
            0 => {
                // PublishSuccess
                let mut id_bytes = [0; 8];
                reader.read_exact(&mut id_bytes).ok()?;
                let id = usize::from_be_bytes(id_bytes);
                Some(Response::PublishSuccess(id))
            }
            1 => {
                // SearchSuccess
                let mut length_bytes = [0; 4];
                reader.read_exact(&mut length_bytes).ok()?;
                let length = u32::from_be_bytes(length_bytes) as usize;

                let mut ids = Vec::with_capacity(length);
                for _ in 0..length {
                    let mut id_bytes = [0; 8];
                    reader.read_exact(&mut id_bytes).ok()?;
                    ids.push(usize::from_be_bytes(id_bytes));
                }
                Some(Response::SearchSuccess(ids))
            }
            2 => {
                // RetrieveSuccess
                let mut length_bytes = [0; 4];
                reader.read_exact(&mut length_bytes).ok()?;
                let length = u32::from_be_bytes(length_bytes) as usize;

                let mut doc_bytes = vec![0; length];
                reader.read_exact(&mut doc_bytes).ok()?;
                let doc = String::from_utf8(doc_bytes).ok()?;
                Some(Response::RetrieveSuccess(doc))
            }
            3 => {
                // Failure
                Some(Response::Failure)
            }
            _ => None,
        }
    }
}

